//! Event handlers for scaling operations.
//!
//! This module provides callback-based event handling for scale events.
//! Demonstrates `rustscale::handlers` namespace with PyO3 bindings.

use pyo3::prelude::*;

/// Result of a scaling operation.
#[pyclass]
#[derive(Clone)]
pub struct ScaleResult {
    /// The original input value.
    #[pyo3(get)]
    pub input: f64,
    /// The scaled output value.
    #[pyo3(get)]
    pub output: f64,
    /// Whether the operation succeeded.
    #[pyo3(get)]
    pub success: bool,
}

#[pymethods]
impl ScaleResult {
    /// Create a new scale result.
    ///
    /// # Arguments
    ///
    /// * `input` - The original value.
    /// * `output` - The scaled value.
    /// * `success` - Whether scaling succeeded.
    #[new]
    pub fn new(input: f64, output: f64, success: bool) -> Self {
        Self { input, output, success }
    }

    /// Get the scaling ratio (output / input).
    ///
    /// # Returns
    ///
    /// The ratio between output and input, or 0.0 if input is zero.
    pub fn ratio(&self) -> f64 {
        if self.input == 0.0 {
            0.0
        } else {
            self.output / self.input
        }
    }
}

/// Handler that processes scale events.
#[pyclass]
pub struct ScaleHandler {
    /// Number of events processed.
    #[pyo3(get)]
    pub count: usize,
}

#[pymethods]
impl ScaleHandler {
    /// Create a new scale handler.
    #[new]
    pub fn new() -> Self {
        Self { count: 0 }
    }

    /// Handle a scaling event.
    ///
    /// # Arguments
    ///
    /// * `result` - The scale result to process.
    pub fn handle(&mut self, result: &ScaleResult) {
        self.count += 1;
        if !result.success {
            // Log failure
        }
    }

    /// Reset the handler state.
    pub fn reset(&mut self) {
        self.count = 0;
    }
}

/// Create a successful scale result.
///
/// # Arguments
///
/// * `input` - The input value.
/// * `output` - The output value.
///
/// # Returns
///
/// A ScaleResult with success=true.
#[pyfunction]
pub fn success_result(input: f64, output: f64) -> ScaleResult {
    ScaleResult::new(input, output, true)
}

/// Register handlers module with Python.
pub fn register(parent: &Bound<'_, PyModule>) -> PyResult<()> {
    let m = PyModule::new(parent.py(), "handlers")?;
    m.add_class::<ScaleResult>()?;
    m.add_class::<ScaleHandler>()?;
    m.add_function(wrap_pyfunction!(success_result, &m)?)?;
    parent.add_submodule(&m)?;
    Ok(())
}
