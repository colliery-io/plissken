//! Internal implementation details for RustScale.
//!
//! This module contains pure Rust code without PyO3 bindings.
//! It demonstrates `rustscale::internal` namespace.

pub mod parser;

/// Internal configuration for scaling operations.
pub struct ScaleConfig {
    /// Precision for floating-point comparisons.
    pub precision: f64,
    /// Maximum iterations for iterative algorithms.
    pub max_iterations: usize,
}

impl ScaleConfig {
    /// Create a new configuration with defaults.
    pub fn new() -> Self {
        Self {
            precision: 1e-10,
            max_iterations: 1000,
        }
    }

    /// Create a high-precision configuration.
    pub fn high_precision() -> Self {
        Self {
            precision: 1e-15,
            max_iterations: 10000,
        }
    }
}

impl Default for ScaleConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Error types for internal operations.
#[derive(Debug)]
pub enum InternalError {
    /// Value is out of acceptable range.
    OutOfRange { value: f64, min: f64, max: f64 },
    /// Computation failed to converge.
    ConvergenceFailure { iterations: usize },
    /// Invalid configuration provided.
    InvalidConfig(String),
}

impl std::fmt::Display for InternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OutOfRange { value, min, max } => {
                write!(f, "value {} out of range [{}, {}]", value, min, max)
            }
            Self::ConvergenceFailure { iterations } => {
                write!(f, "failed to converge after {} iterations", iterations)
            }
            Self::InvalidConfig(msg) => write!(f, "invalid config: {}", msg),
        }
    }
}

impl std::error::Error for InternalError {}

/// Clamp a value to the given range.
///
/// # Arguments
///
/// * `value` - The value to clamp.
/// * `min` - Minimum allowed value.
/// * `max` - Maximum allowed value.
///
/// # Returns
///
/// The clamped value.
pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
    value.max(min).min(max)
}
