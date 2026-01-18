//! Transform pipeline for chaining transformations.

use super::{TransformError, TransformResult};
use crate::config::Value;

/// A transformation that can be applied to a value.
pub trait Transform: Send + Sync {
    /// Apply this transformation to a value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to transform.
    ///
    /// # Errors
    ///
    /// Returns a `TransformError` if the transformation fails.
    fn apply(&self, value: Value) -> TransformResult<Value>;

    /// Get a human-readable name for this transform.
    fn name(&self) -> &str;
}

/// A pipeline of transformations applied in sequence.
///
/// Each transformation receives the output of the previous one.
///
/// # Example
///
/// ```ignore
/// let pipeline = TransformPipeline::new()
///     .add(UppercaseTransform)
///     .add(TrimTransform);
/// let result = pipeline.execute(value)?;
/// ```
pub struct TransformPipeline {
    transforms: Vec<Box<dyn Transform>>,
    name: String,
}

impl TransformPipeline {
    /// Create a new empty pipeline.
    pub fn new() -> Self {
        Self {
            transforms: Vec::new(),
            name: "pipeline".to_string(),
        }
    }

    /// Create a pipeline with a custom name.
    pub fn named(name: impl Into<String>) -> Self {
        Self {
            transforms: Vec::new(),
            name: name.into(),
        }
    }

    /// Add a transform to the pipeline.
    ///
    /// Transforms are applied in the order they are added.
    pub fn add(mut self, transform: impl Transform + 'static) -> Self {
        self.transforms.push(Box::new(transform));
        self
    }

    /// Execute all transforms in the pipeline.
    ///
    /// # Arguments
    ///
    /// * `value` - The initial value to transform.
    ///
    /// # Errors
    ///
    /// Returns the first error encountered, if any.
    pub fn execute(&self, mut value: Value) -> TransformResult<Value> {
        for transform in &self.transforms {
            value = transform.apply(value)?;
        }
        Ok(value)
    }

    /// Get the number of transforms in the pipeline.
    pub fn len(&self) -> usize {
        self.transforms.len()
    }

    /// Check if the pipeline is empty.
    pub fn is_empty(&self) -> bool {
        self.transforms.is_empty()
    }
}

impl Default for TransformPipeline {
    fn default() -> Self {
        Self::new()
    }
}

/// Transform that converts string values to uppercase.
pub struct UppercaseTransform;

impl Transform for UppercaseTransform {
    fn apply(&self, value: Value) -> TransformResult<Value> {
        match value {
            Value::String(s) => Ok(Value::String(s.to_uppercase())),
            other => Err(TransformError::TypeMismatch {
                expected: "string",
                actual: type_name(&other),
            }),
        }
    }

    fn name(&self) -> &str {
        "uppercase"
    }
}

/// Transform that converts string values to lowercase.
pub struct LowercaseTransform;

impl Transform for LowercaseTransform {
    fn apply(&self, value: Value) -> TransformResult<Value> {
        match value {
            Value::String(s) => Ok(Value::String(s.to_lowercase())),
            other => Err(TransformError::TypeMismatch {
                expected: "string",
                actual: type_name(&other),
            }),
        }
    }

    fn name(&self) -> &str {
        "lowercase"
    }
}

/// Transform that trims whitespace from string values.
pub struct TrimTransform;

impl Transform for TrimTransform {
    fn apply(&self, value: Value) -> TransformResult<Value> {
        match value {
            Value::String(s) => Ok(Value::String(s.trim().to_string())),
            other => Err(TransformError::TypeMismatch {
                expected: "string",
                actual: type_name(&other),
            }),
        }
    }

    fn name(&self) -> &str {
        "trim"
    }
}

fn type_name(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "bool",
        Value::Int(_) => "int",
        Value::Float(_) => "float",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Map(_) => "map",
    }
}
