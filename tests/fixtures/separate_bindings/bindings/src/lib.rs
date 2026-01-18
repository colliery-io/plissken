//! Python bindings for the data pipeline library.
//!
//! This crate provides PyO3 bindings to expose the core pipeline
//! functionality to Python users.

use pyo3::prelude::*;
use pyo3::exceptions::{PyRuntimeError, PyValueError};
use std::collections::HashMap;

// Import from the core library
use separate_bindings_core::{
    Pipeline as CorePipeline,
    PipelineConfig as CoreConfig,
    PipelineResult as CoreResult,
    Stage as CoreStage,
    DataBatch as CoreBatch,
    Value as CoreValue,
    StageError,
};

/// A data processing pipeline.
///
/// Pipelines consist of ordered stages that process data batches
/// sequentially.
///
/// Example:
///     ```python
///     from separate_bindings import Pipeline, DataBatch
///
///     pipeline = Pipeline("etl")
///     pipeline.add_stage("transform", lambda batch: batch)
///
///     result = pipeline.run(DataBatch.from_dicts([{"a": 1}]))
///     print(f"Processed {result.rows_out} rows")
///     ```
#[pyclass(name = "Pipeline")]
pub struct PyPipeline {
    inner: CorePipeline,
    py_stages: Vec<(String, PyObject)>,
}

#[pymethods]
impl PyPipeline {
    /// Create a new pipeline.
    ///
    /// Args:
    ///     name: Identifier for this pipeline.
    #[new]
    fn new(name: &str) -> Self {
        Self {
            inner: CorePipeline::new(name),
            py_stages: vec![],
        }
    }

    /// Get the pipeline name.
    #[getter]
    fn name(&self) -> &str {
        self.inner.name()
    }

    /// Add a processing stage.
    ///
    /// Args:
    ///     name: Unique name for this stage.
    ///     processor: Callable that takes a DataBatch and returns a DataBatch.
    ///
    /// Returns:
    ///     Self for method chaining.
    fn add_stage(&mut self, name: &str, processor: PyObject) -> PyResult<()> {
        self.py_stages.push((name.to_string(), processor));
        Ok(())
    }

    /// Get the number of stages.
    fn stage_count(&self) -> usize {
        self.py_stages.len()
    }

    /// Run the pipeline.
    ///
    /// Args:
    ///     input: The input DataBatch to process.
    ///
    /// Returns:
    ///     PipelineResult with output batch and statistics.
    ///
    /// Raises:
    ///     RuntimeError: If any stage fails.
    fn run(&self, py: Python<'_>, input: &PyDataBatch) -> PyResult<PyPipelineResult> {
        let mut current = input.inner.clone();

        let start = std::time::Instant::now();
        let mut stages_run = 0;

        for (name, processor) in &self.py_stages {
            let py_batch = PyDataBatch { inner: current };
            let result = processor.call1(py, (py_batch,))?;
            current = result.extract::<PyDataBatch>(py)?.inner;
            stages_run += 1;
            let _ = name; // Would use for error context
        }

        Ok(PyPipelineResult {
            output: PyDataBatch { inner: current },
            stages_run,
            duration_secs: start.elapsed().as_secs_f64(),
        })
    }
}

/// Result of a pipeline run.
#[pyclass(name = "PipelineResult")]
pub struct PyPipelineResult {
    #[pyo3(get)]
    output: PyDataBatch,
    #[pyo3(get)]
    stages_run: usize,
    #[pyo3(get)]
    duration_secs: f64,
}

#[pymethods]
impl PyPipelineResult {
    /// Number of rows in the output.
    #[getter]
    fn rows_out(&self) -> usize {
        self.output.len()
    }
}

/// A batch of data rows.
///
/// DataBatch is the unit of data flowing through pipelines.
/// Each batch contains zero or more rows, where each row is
/// a dictionary of column names to values.
#[pyclass(name = "DataBatch")]
#[derive(Clone)]
pub struct PyDataBatch {
    inner: CoreBatch,
}

#[pymethods]
impl PyDataBatch {
    /// Create an empty batch.
    #[new]
    fn new() -> Self {
        Self {
            inner: CoreBatch::new(),
        }
    }

    /// Create a batch from a list of dictionaries.
    ///
    /// Args:
    ///     rows: List of dicts, each representing a row.
    ///
    /// Returns:
    ///     A new DataBatch containing the rows.
    ///
    /// Example:
    ///     ```python
    ///     batch = DataBatch.from_dicts([
    ///         {"name": "Alice", "age": 30},
    ///         {"name": "Bob", "age": 25},
    ///     ])
    ///     ```
    #[staticmethod]
    fn from_dicts(rows: Vec<HashMap<String, PyObject>>, py: Python<'_>) -> PyResult<Self> {
        let core_rows: Vec<_> = rows
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|(k, v)| Ok((k, py_to_value(py, &v)?)))
                    .collect::<PyResult<HashMap<_, _>>>()
            })
            .collect::<PyResult<Vec<_>>>()?;

        Ok(Self {
            inner: CoreBatch::from_rows(core_rows),
        })
    }

    /// Get the number of rows.
    fn __len__(&self) -> usize {
        self.inner.len()
    }

    /// Get the number of rows.
    fn len(&self) -> usize {
        self.inner.len()
    }

    /// Check if batch is empty.
    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Convert to a list of dictionaries.
    fn to_dicts(&self, py: Python<'_>) -> PyResult<Vec<HashMap<String, PyObject>>> {
        self.inner
            .rows()
            .iter()
            .map(|row| {
                row.iter()
                    .map(|(k, v)| Ok((k.clone(), value_to_py(py, v)?)))
                    .collect::<PyResult<HashMap<_, _>>>()
            })
            .collect()
    }
}

// Conversion helpers

fn py_to_value(py: Python<'_>, obj: &PyObject) -> PyResult<CoreValue> {
    if obj.is_none(py) {
        Ok(CoreValue::Null)
    } else if let Ok(b) = obj.extract::<bool>(py) {
        Ok(CoreValue::Bool(b))
    } else if let Ok(i) = obj.extract::<i64>(py) {
        Ok(CoreValue::Int(i))
    } else if let Ok(f) = obj.extract::<f64>(py) {
        Ok(CoreValue::Float(f))
    } else if let Ok(s) = obj.extract::<String>(py) {
        Ok(CoreValue::String(s))
    } else {
        Err(PyValueError::new_err("Unsupported value type"))
    }
}

fn value_to_py(py: Python<'_>, value: &CoreValue) -> PyResult<PyObject> {
    match value {
        CoreValue::Null => Ok(py.None()),
        CoreValue::Bool(b) => Ok(b.into_pyobject(py)?.into_any().unbind()),
        CoreValue::Int(i) => Ok(i.into_pyobject(py)?.into_any().unbind()),
        CoreValue::Float(f) => Ok(f.into_pyobject(py)?.into_any().unbind()),
        CoreValue::String(s) => Ok(s.into_pyobject(py)?.into_any().unbind()),
        _ => Err(PyValueError::new_err("Unsupported value type")),
    }
}

/// Python module definition.
#[pymodule]
fn separate_bindings(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyPipeline>()?;
    m.add_class::<PyPipelineResult>()?;
    m.add_class::<PyDataBatch>()?;
    Ok(())
}
