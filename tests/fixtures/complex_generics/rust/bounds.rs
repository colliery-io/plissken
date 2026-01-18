//! Complex trait bounds.

use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::ops::{Add, Mul};

/// Multiple trait bounds on single parameter.
pub fn multi_bound<T>(value: T) -> String
where
    T: Debug + Display + Clone + Default + Send + Sync + 'static,
{
    format!("{:?}", value)
}

/// Multiple generic parameters with interdependent bounds.
pub fn interdependent<T, U, V>(a: T, b: U) -> V
where
    T: Into<U> + Clone,
    U: Into<V> + From<T>,
    V: Default + From<U>,
{
    let u: U = a.into();
    u.into()
}

/// Arithmetic trait bounds.
pub fn numeric_ops<T>(a: T, b: T) -> T
where
    T: Add<Output = T> + Mul<Output = T> + Copy + Default,
{
    a + b * b
}

/// Bounds with associated type constraints.
pub fn with_associated<I, T>(iter: I) -> Vec<T>
where
    I: Iterator<Item = T>,
    T: Clone + Ord,
{
    let mut v: Vec<T> = iter.collect();
    v.sort();
    v
}

/// Negative trait bounds simulation (using marker traits).
pub trait NotCopy {}
impl<T: Copy> NotCopy for std::marker::PhantomData<T> {}

/// Supertraits - trait that requires other traits.
pub trait Serializable: Debug + Clone + Send + Sync {
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(data: &[u8]) -> Result<Self, DeserializeError>
    where
        Self: Sized;
}

/// Function using supertrait.
pub fn roundtrip<T: Serializable>(value: T) -> Result<T, DeserializeError> {
    let bytes = value.serialize();
    T::deserialize(&bytes)
}

/// Complex where clause spanning multiple lines.
pub fn complex_where<A, B, C, D, E>(
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
) -> (A, B, C, D, E)
where
    A: Clone + Debug + Default,
    B: From<A> + Into<C>,
    C: Hash + Eq + Ord,
    D: AsRef<str> + AsMut<str>,
    E: Iterator<Item = A> + ExactSizeIterator + DoubleEndedIterator,
{
    (a, b, c, d, e)
}

#[derive(Debug, thiserror::Error)]
#[error("Deserialization failed")]
pub struct DeserializeError;
