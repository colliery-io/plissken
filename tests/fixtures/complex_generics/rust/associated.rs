//! Associated types and GATs.

use std::collections::HashMap;

/// Trait with associated type.
pub trait Container {
    /// The type of items in this container.
    type Item;

    /// Get an item by index.
    fn get(&self, index: usize) -> Option<&Self::Item>;

    /// Get the number of items.
    fn len(&self) -> usize;

    /// Check if empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Using associated types in function bounds.
pub fn first_item<C>(container: &C) -> Option<&C::Item>
where
    C: Container,
{
    container.get(0)
}

/// Complex associated type constraints.
pub fn collect_items<C, I>(container: &C) -> Vec<I>
where
    C: Container<Item = I>,
    I: Clone,
{
    let mut items = Vec::new();
    for i in 0..container.len() {
        if let Some(item) = container.get(i) {
            items.push(item.clone());
        }
    }
    items
}

/// Trait with multiple associated types.
pub trait Graph {
    type Node;
    type Edge;
    type NodeId: Copy + Eq;
    type EdgeId: Copy + Eq;

    fn nodes(&self) -> impl Iterator<Item = (Self::NodeId, &Self::Node)>;
    fn edges(&self) -> impl Iterator<Item = (Self::EdgeId, &Self::Edge)>;
    fn neighbors(&self, node: Self::NodeId) -> impl Iterator<Item = Self::NodeId>;
}

/// Generic Associated Types (GATs).
///
/// GATs allow associated types to have their own generic parameters.
pub trait LendingIterator {
    /// Item type is generic over a lifetime - this is a GAT.
    type Item<'a>
    where
        Self: 'a;

    /// Get the next item.
    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
}

/// Another GAT example - container with borrowed views.
pub trait BorrowedContainer {
    type Borrowed<'a>
    where
        Self: 'a;

    fn borrow<'a>(&'a self) -> Self::Borrowed<'a>;
}

/// Implementation of GAT trait.
pub struct WindowedSlice<T> {
    data: Vec<T>,
    window_size: usize,
    position: usize,
}

impl<T> LendingIterator for WindowedSlice<T> {
    type Item<'a> = &'a [T] where T: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {
        if self.position + self.window_size <= self.data.len() {
            let window = &self.data[self.position..self.position + self.window_size];
            self.position += 1;
            Some(window)
        } else {
            None
        }
    }
}

/// Function using GAT-bounded parameter.
pub fn consume_lending<I>(mut iter: I) -> usize
where
    I: LendingIterator,
    for<'a> I::Item<'a>: AsRef<[u8]>,
{
    let mut count = 0;
    while let Some(item) = iter.next() {
        count += item.as_ref().len();
    }
    count
}

/// Associated type with where clause.
pub trait Mappable {
    type Output<U>
    where
        U: Clone;

    fn map<U, F>(self, f: F) -> Self::Output<U>
    where
        U: Clone,
        F: FnMut(Self) -> U,
        Self: Sized;
}
