//! Configuration management for task runners.
//!
//! Provides structured configuration loading and validation
//! for task runner instances.

use pyo3::prelude::*;
use std::collections::HashMap;

/// Configuration for a task runner instance.
///
/// Holds all settings needed to configure runner behavior
/// including parallelism, timeouts, and retry policies.
#[pyclass(name = "RunnerConfig")]
#[derive(Clone)]
pub struct PyRunnerConfig {
    max_parallel: usize,
    timeout_secs: Option<f64>,
    retry_count: u32,
    env_vars: HashMap<String, String>,
}

#[pymethods]
impl PyRunnerConfig {
    /// Create a new runner configuration.
    ///
    /// Args:
    ///     max_parallel: Maximum concurrent tasks (default: 4).
    ///     timeout_secs: Global timeout in seconds (optional).
    ///     retry_count: Number of retries on failure (default: 0).
    #[new]
    #[pyo3(signature = (max_parallel=None, timeout_secs=None, retry_count=None))]
    fn new(max_parallel: Option<usize>, timeout_secs: Option<f64>, retry_count: Option<u32>) -> Self {
        Self {
            max_parallel: max_parallel.unwrap_or(4),
            timeout_secs,
            retry_count: retry_count.unwrap_or(0),
            env_vars: HashMap::new(),
        }
    }

    /// Maximum number of parallel tasks.
    #[getter]
    fn max_parallel(&self) -> usize {
        self.max_parallel
    }

    /// Set maximum parallel tasks.
    #[setter]
    fn set_max_parallel(&mut self, value: usize) {
        self.max_parallel = value;
    }

    /// Timeout in seconds, if set.
    #[getter]
    fn timeout_secs(&self) -> Option<f64> {
        self.timeout_secs
    }

    /// Number of retry attempts.
    #[getter]
    fn retry_count(&self) -> u32 {
        self.retry_count
    }

    /// Set an environment variable for task execution.
    ///
    /// Args:
    ///     key: Environment variable name.
    ///     value: Environment variable value.
    fn set_env(&mut self, key: &str, value: &str) {
        self.env_vars.insert(key.to_string(), value.to_string());
    }

    /// Get an environment variable.
    ///
    /// Args:
    ///     key: Environment variable name.
    ///
    /// Returns:
    ///     The value if set, None otherwise.
    fn get_env(&self, key: &str) -> Option<String> {
        self.env_vars.get(key).cloned()
    }

    /// Get all environment variables as a dict.
    fn env_vars(&self) -> HashMap<String, String> {
        self.env_vars.clone()
    }
}

/// Task-specific configuration overrides.
///
/// Allows per-task customization of execution behavior.
#[pyclass(name = "TaskConfig")]
#[derive(Clone)]
pub struct PyTaskConfig {
    timeout_secs: Option<f64>,
    retry_count: Option<u32>,
    run_async: bool,
}

#[pymethods]
impl PyTaskConfig {
    /// Create task-specific configuration.
    ///
    /// Args:
    ///     timeout_secs: Task timeout override.
    ///     retry_count: Task retry override.
    ///     run_async: Whether to run asynchronously.
    #[new]
    #[pyo3(signature = (timeout_secs=None, retry_count=None, run_async=false))]
    fn new(timeout_secs: Option<f64>, retry_count: Option<u32>, run_async: bool) -> Self {
        Self {
            timeout_secs,
            retry_count,
            run_async,
        }
    }

    /// Task timeout override.
    #[getter]
    fn timeout_secs(&self) -> Option<f64> {
        self.timeout_secs
    }

    /// Task retry count override.
    #[getter]
    fn retry_count(&self) -> Option<u32> {
        self.retry_count
    }

    /// Whether task runs asynchronously.
    #[getter]
    fn run_async(&self) -> bool {
        self.run_async
    }
}

/// Load configuration from a TOML file.
///
/// Args:
///     path: Path to the TOML configuration file.
///
/// Returns:
///     RunnerConfig parsed from the file.
///
/// Raises:
///     ValueError: If the file cannot be read or parsed.
#[pyfunction]
pub fn load_config(path: &str) -> PyResult<PyRunnerConfig> {
    // Simplified - in real code would parse TOML
    let _ = path;
    Ok(PyRunnerConfig::new(None, None, None))
}

/// Register the config submodule.
pub fn register(parent: &Bound<'_, PyModule>) -> PyResult<()> {
    let m = PyModule::new(parent.py(), "config")?;
    m.add_class::<PyRunnerConfig>()?;
    m.add_class::<PyTaskConfig>()?;
    m.add_function(wrap_pyfunction!(load_config, &m)?)?;
    parent.add_submodule(&m)?;
    Ok(())
}
