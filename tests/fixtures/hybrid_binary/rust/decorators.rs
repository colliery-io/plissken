//! Python decorators for task registration.
//!
//! Provides a decorator-based API for registering tasks:
//!
//! ```python
//! from hybrid_binary import task, Runner
//!
//! runner = Runner()
//!
//! @task(runner, name="build", description="Build the project")
//! def build():
//!     print("Building...")
//! ```

use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::{PyRunner, PyTask};

/// Decorator state that captures registration info.
#[pyclass]
pub struct TaskDecorator {
    runner: Py<PyRunner>,
    name: String,
    description: Option<String>,
    depends: Vec<String>,
}

#[pymethods]
impl TaskDecorator {
    /// Called when the decorator is applied to a function.
    fn __call__(&self, py: Python<'_>, func: PyObject) -> PyResult<PyObject> {
        let mut task = PyTask::new(&self.name, self.description.as_deref());

        for dep in &self.depends {
            task.depends_on(dep);
        }

        let mut runner = self.runner.borrow_mut(py);
        runner.register(&task, func.clone())?;

        Ok(func)
    }
}

/// Create a task decorator.
///
/// This is the main entry point for the decorator API.
///
/// Args:
///     runner: The Runner instance to register with.
///     name: Unique name for this task.
///     description: Human-readable description.
///     depends: List of task names this depends on.
///
/// Returns:
///     A decorator that registers the function as a task.
///
/// Example:
///     ```python
///     @task(runner, name="test", depends=["build"])
///     def run_tests():
///         subprocess.run(["pytest"])
///     ```
#[pyfunction]
#[pyo3(signature = (runner, **kwargs))]
pub fn task(runner: Py<PyRunner>, kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<TaskDecorator> {
    let name = kwargs
        .and_then(|d| d.get_item("name").ok().flatten())
        .map(|v| v.extract::<String>())
        .transpose()?
        .unwrap_or_else(|| "unnamed".to_string());

    let description = kwargs
        .and_then(|d| d.get_item("description").ok().flatten())
        .map(|v| v.extract::<String>())
        .transpose()?;

    let depends = kwargs
        .and_then(|d| d.get_item("depends").ok().flatten())
        .map(|v| v.extract::<Vec<String>>())
        .transpose()?
        .unwrap_or_default();

    Ok(TaskDecorator {
        runner,
        name,
        description,
        depends,
    })
}

/// Register decorator functions with the module.
pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(pyo3::wrap_pyfunction!(task, m)?)?;
    m.add_class::<TaskDecorator>()?;
    Ok(())
}
