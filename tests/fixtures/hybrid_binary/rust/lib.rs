//! A task runner library with Python bindings.
//!
//! This module provides task execution and scheduling capabilities
//! exposed to Python via PyO3.

use pyo3::prelude::*;

mod internal;
mod decorators;
mod utils;
mod config;

use internal::TaskExecutor;

/// A task that can be executed by the runner.
///
/// Tasks have a name, optional description, and can be marked
/// as async or having dependencies.
#[pyclass(name = "Task")]
#[derive(Clone)]
pub struct PyTask {
    inner: internal::Task,
}

#[pymethods]
impl PyTask {
    /// Create a new task.
    ///
    /// Args:
    ///     name: The unique identifier for this task.
    ///     description: Optional human-readable description.
    ///
    /// Returns:
    ///     A new Task instance.
    #[new]
    #[pyo3(signature = (name, description=None))]
    fn new(name: &str, description: Option<&str>) -> Self {
        Self {
            inner: internal::Task::new(name, description),
        }
    }

    /// Get the task name.
    #[getter]
    fn name(&self) -> &str {
        &self.inner.name
    }

    /// Get the task description.
    #[getter]
    fn description(&self) -> Option<&str> {
        self.inner.description.as_deref()
    }

    /// Add a dependency on another task.
    ///
    /// Args:
    ///     task_name: Name of the task this depends on.
    fn depends_on(&mut self, task_name: &str) {
        self.inner.add_dependency(task_name);
    }

    /// Mark this task as async.
    fn set_async(&mut self, is_async: bool) {
        self.inner.is_async = is_async;
    }
}

/// The main task runner that executes tasks.
///
/// Handles dependency resolution, parallel execution, and
/// error reporting.
#[pyclass(name = "Runner")]
pub struct PyRunner {
    executor: TaskExecutor,
}

#[pymethods]
impl PyRunner {
    /// Create a new task runner.
    ///
    /// Args:
    ///     max_parallel: Maximum number of concurrent tasks (default: 4).
    #[new]
    #[pyo3(signature = (max_parallel=None))]
    fn new(max_parallel: Option<usize>) -> PyResult<Self> {
        let executor = TaskExecutor::new(max_parallel.unwrap_or(4))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(Self { executor })
    }

    /// Register a task with the runner.
    ///
    /// Args:
    ///     task: The task to register.
    ///     handler: Python callable to execute for this task.
    ///
    /// Raises:
    ///     ValueError: If a task with this name already exists.
    fn register(&mut self, task: &PyTask, handler: PyObject) -> PyResult<()> {
        self.executor
            .register(task.inner.clone(), handler)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }

    /// Run a task and all its dependencies.
    ///
    /// Args:
    ///     task_name: Name of the task to run.
    ///     dry_run: If True, only print what would be executed.
    ///
    /// Returns:
    ///     RunResult with execution details.
    ///
    /// Raises:
    ///     RuntimeError: If task execution fails.
    #[pyo3(signature = (task_name, dry_run=false))]
    fn run(&self, task_name: &str, dry_run: bool) -> PyResult<PyRunResult> {
        let result = self
            .executor
            .run(task_name, dry_run)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(PyRunResult { inner: result })
    }

    /// List all registered tasks.
    fn list_tasks(&self) -> Vec<String> {
        self.executor.list_tasks()
    }
}

/// Result of a task run.
#[pyclass(name = "RunResult")]
pub struct PyRunResult {
    inner: internal::RunResult,
}

#[pymethods]
impl PyRunResult {
    /// Whether all tasks succeeded.
    #[getter]
    fn success(&self) -> bool {
        self.inner.success
    }

    /// Number of tasks executed.
    #[getter]
    fn tasks_run(&self) -> usize {
        self.inner.tasks_run
    }

    /// Total execution time in seconds.
    #[getter]
    fn duration_secs(&self) -> f64 {
        self.inner.duration.as_secs_f64()
    }

    /// List of failed task names, if any.
    #[getter]
    fn failed(&self) -> Vec<String> {
        self.inner.failed.clone()
    }
}

/// Register the module with Python.
#[pymodule]
fn hybrid_binary(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyTask>()?;
    m.add_class::<PyRunner>()?;
    m.add_class::<PyRunResult>()?;
    decorators::register(m)?;
    utils::register(m)?;
    config::register(m)?;
    Ok(())
}
