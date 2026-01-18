//! Utility functions and helpers for task management.
//!
//! This submodule provides common utilities used throughout
//! the task runner system.

use pyo3::prelude::*;

/// A simple logger for task execution.
///
/// Captures output from tasks and provides filtering capabilities.
#[pyclass(name = "TaskLogger")]
pub struct PyTaskLogger {
    level: LogLevel,
    buffer: Vec<String>,
}

/// Log level for filtering output.
#[pyclass(name = "LogLevel", eq, eq_int)]
#[derive(Clone, Copy, PartialEq)]
pub enum LogLevel {
    Debug = 0,
    Info = 1,
    Warning = 2,
    Error = 3,
}

#[pymethods]
impl PyTaskLogger {
    /// Create a new task logger.
    ///
    /// Args:
    ///     level: Minimum log level to capture.
    #[new]
    #[pyo3(signature = (level=None))]
    fn new(level: Option<LogLevel>) -> Self {
        Self {
            level: level.unwrap_or(LogLevel::Info),
            buffer: Vec::new(),
        }
    }

    /// Log a message at the specified level.
    ///
    /// Args:
    ///     level: The log level for this message.
    ///     message: The message to log.
    fn log(&mut self, level: LogLevel, message: &str) {
        if level as u8 >= self.level as u8 {
            self.buffer.push(format!("[{:?}] {}", level, message));
        }
    }

    /// Get all captured log messages.
    fn get_logs(&self) -> Vec<String> {
        self.buffer.clone()
    }

    /// Clear the log buffer.
    fn clear(&mut self) {
        self.buffer.clear();
    }

    /// Get the current log level.
    #[getter]
    fn level(&self) -> LogLevel {
        self.level
    }

    /// Set the log level.
    #[setter]
    fn set_level(&mut self, level: LogLevel) {
        self.level = level;
    }
}

/// Format a duration in human-readable form.
///
/// Args:
///     seconds: Duration in seconds.
///
/// Returns:
///     Formatted string like "1m 30s" or "500ms".
#[pyfunction]
pub fn format_duration(seconds: f64) -> String {
    if seconds < 1.0 {
        format!("{:.0}ms", seconds * 1000.0)
    } else if seconds < 60.0 {
        format!("{:.1}s", seconds)
    } else {
        let mins = (seconds / 60.0).floor() as u64;
        let secs = seconds % 60.0;
        format!("{}m {:.0}s", mins, secs)
    }
}

/// Register the utils submodule.
pub fn register(parent: &Bound<'_, PyModule>) -> PyResult<()> {
    let m = PyModule::new(parent.py(), "utils")?;
    m.add_class::<PyTaskLogger>()?;
    m.add_class::<LogLevel>()?;
    m.add_function(wrap_pyfunction!(format_duration, &m)?)?;
    parent.add_submodule(&m)?;
    Ok(())
}
