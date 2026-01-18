//! Lifetime parameters and complex borrowing.

use std::collections::HashMap;

/// Simple lifetime parameter.
pub fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}

/// Multiple distinct lifetimes.
pub fn multi_lifetime<'a, 'b, 'c>(
    x: &'a str,
    y: &'b str,
    z: &'c str,
) -> (&'a str, &'b str, &'c str) {
    (x, y, z)
}

/// Lifetime with generic type parameter.
pub fn find_in_slice<'a, T>(slice: &'a [T], predicate: impl Fn(&T) -> bool) -> Option<&'a T> {
    slice.iter().find(|x| predicate(x))
}

/// Lifetime bounds on generic parameters.
pub fn with_lifetime_bound<'a, T: 'a>(value: &'a T) -> &'a T {
    value
}

/// Struct with lifetime parameter.
pub struct Borrowed<'a, T> {
    /// Reference to the borrowed data.
    pub data: &'a T,
    /// Name for identification.
    pub name: &'a str,
}

impl<'a, T> Borrowed<'a, T> {
    /// Create a new borrowed wrapper.
    pub fn new(data: &'a T, name: &'a str) -> Self {
        Self { data, name }
    }

    /// Get the data with the same lifetime.
    pub fn get(&self) -> &'a T {
        self.data
    }
}

/// Higher-Ranked Trait Bounds (HRTB).
///
/// The function works with any closure that can accept
/// a reference of any lifetime.
pub fn for_all_lifetimes<F>(f: F) -> String
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    let s = String::from("hello");
    f(&s).to_string()
}

/// HRTB with multiple parameters.
pub fn complex_hrtb<F, T>(f: F, items: Vec<T>) -> Vec<String>
where
    F: for<'a, 'b> Fn(&'a T, &'b str) -> String,
    T: AsRef<str>,
{
    items
        .iter()
        .map(|item| f(item, "context"))
        .collect()
}

/// Lifetime elision edge cases.
pub struct Parser<'input> {
    input: &'input str,
    position: usize,
}

impl<'input> Parser<'input> {
    pub fn new(input: &'input str) -> Self {
        Self { input, position: 0 }
    }

    /// Returns slice with input lifetime (elided in signature).
    pub fn remaining(&self) -> &'input str {
        &self.input[self.position..]
    }

    /// Multiple references with different lifetimes.
    pub fn split_at<'a>(&'a self, pos: usize) -> (&'input str, &'input str) {
        (&self.input[..pos], &self.input[pos..])
    }
}

/// Covariant lifetime in return position.
pub fn covariant<'a, 'b: 'a>(long: &'b str, _short: &'a str) -> &'a str {
    long
}
