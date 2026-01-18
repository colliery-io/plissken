//! Configuration value transformations.
//!
//! This module provides utilities for transforming configuration
//! values between formats and applying modifications.

mod pipeline;
mod filters;

pub use pipeline::{TransformPipeline, Transform};
pub use filters::{Filter, FilterFn, StringFilter};

use crate::config::Value;

/// Errors that can occur during transformation.
#[derive(Debug, thiserror::Error)]
pub enum TransformError {
    /// The input value had an unexpected type.
    #[error("Type mismatch: expected {expected}, got {actual}")]
    TypeMismatch {
        /// Expected type name.
        expected: &'static str,
        /// Actual type name.
        actual: &'static str,
    },

    /// A required value was missing.
    #[error("Missing value: {0}")]
    Missing(String),

    /// Custom transformation error.
    #[error("Transform failed: {0}")]
    Custom(String),
}

/// Result type for transform operations.
pub type TransformResult<T> = Result<T, TransformError>;

/// A buffer with a compile-time fixed capacity.
///
/// This demonstrates const generics for documentation purposes.
///
/// # Type Parameters
///
/// * `T` - The element type.
/// * `N` - The buffer capacity (const generic).
pub struct FixedBuffer<T, const N: usize> {
    data: [Option<T>; N],
    len: usize,
}

impl<T: Default + Copy, const N: usize> FixedBuffer<T, N> {
    /// Create a new empty buffer.
    pub fn new() -> Self
    where
        T: Copy,
    {
        Self {
            data: [None; N],
            len: 0,
        }
    }

    /// Push a value onto the buffer.
    ///
    /// # Returns
    ///
    /// Returns `Err` if the buffer is full.
    pub fn push(&mut self, value: T) -> Result<(), T> {
        if self.len >= N {
            return Err(value);
        }
        self.data[self.len] = Some(value);
        self.len += 1;
        Ok(())
    }

    /// Get the number of elements in the buffer.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Check if the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Get the buffer capacity.
    pub const fn capacity(&self) -> usize {
        N
    }
}

impl<T: Default + Copy, const N: usize> Default for FixedBuffer<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert a value using a generic converter.
///
/// # Type Parameters
///
/// * `T` - Input type.
/// * `U` - Output type.
/// * `F` - Converter function type.
pub fn convert<T, U, F>(value: T, converter: F) -> U
where
    F: FnOnce(T) -> U,
{
    converter(value)
}

/// Apply a transformation to a Value, if present.
pub fn map_value<F>(value: Option<Value>, f: F) -> Option<Value>
where
    F: FnOnce(Value) -> Value,
{
    value.map(f)
}
