//! RustScale - A minimal Rust crate for testing plissken documentation.
//!
//! This crate demonstrates various Rust symbol types, PyO3 bindings,
//! and nested module structures.

use pyo3::prelude::*;

pub mod handlers;
pub mod internal;

/// Maximum scale factor allowed.
pub const MAX_SCALE: f64 = 10.0;

/// Available scaling algorithms.
#[pyclass]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ScaleMode {
    /// Linear scaling - proportional adjustment.
    Linear,
    /// Logarithmic scaling - compressed high values.
    Logarithmic,
    /// Exponential scaling - amplified high values.
    Exponential,
}

/// A scaler that transforms numeric values.
///
/// # Examples
///
/// ```
/// use rustscale::{Scaler, ScaleMode};
///
/// let scaler = Scaler::new(2.0, ScaleMode::Linear);
/// assert_eq!(scaler.scale(5.0), 10.0);
/// ```
#[pyclass]
#[derive(Clone)]
pub struct Scaler {
    /// The scaling factor to apply.
    #[pyo3(get, set)]
    pub factor: f64,
    /// The scaling algorithm to use.
    #[pyo3(get)]
    pub mode: ScaleMode,
}

#[pymethods]
impl Scaler {
    /// Create a new scaler with the given factor and mode.
    ///
    /// # Arguments
    ///
    /// * `factor` - The scaling factor to apply.
    /// * `mode` - The scaling algorithm to use.
    #[new]
    pub fn new(factor: f64, mode: ScaleMode) -> Self {
        Self { factor, mode }
    }

    /// Scale a value using the configured factor and mode.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to scale.
    ///
    /// # Returns
    ///
    /// The scaled value.
    pub fn scale(&self, value: f64) -> f64 {
        match self.mode {
            ScaleMode::Linear => value * self.factor,
            ScaleMode::Logarithmic => value.ln() * self.factor,
            ScaleMode::Exponential => value.exp() * self.factor,
        }
    }

    /// Reset the scaler to default settings.
    pub fn reset(&mut self) {
        self.factor = 1.0;
    }
}

/// Create a linear scaler with the given factor.
///
/// This is a convenience function for creating scalers with linear mode.
///
/// # Arguments
///
/// * `factor` - The scaling factor.
///
/// # Returns
///
/// A new Scaler instance configured for linear scaling.
///
/// # Examples
///
/// ```python
/// scaler = create_linear_scaler(2.5)
/// result = scaler.scale(10.0)  # Returns 25.0
/// ```
#[pyfunction]
pub fn create_linear_scaler(factor: f64) -> Scaler {
    Scaler::new(factor, ScaleMode::Linear)
}

/// Register the module with Python.
#[pymodule]
fn rustscale(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Scaler>()?;
    m.add_class::<ScaleMode>()?;
    m.add_function(wrap_pyfunction!(create_linear_scaler, m)?)?;
    m.add("MAX_SCALE", MAX_SCALE)?;

    // Register submodules
    handlers::register(m)?;

    Ok(())
}
