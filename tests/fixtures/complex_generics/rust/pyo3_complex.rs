//! Complex PyO3 type signatures.

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyTuple};
use std::collections::HashMap;

/// PyO3 function with complex nested Python types.
///
/// # Arguments
///
/// * `data` - Nested structure of dicts containing lists of tuples
/// * `config` - Optional configuration dictionary
///
/// # Returns
///
/// Transformed data structure.
#[pyfunction]
#[pyo3(signature = (data, config=None))]
pub fn transform_nested(
    py: Python<'_>,
    data: HashMap<String, Vec<(i64, Option<String>)>>,
    config: Option<&Bound<'_, PyDict>>,
) -> PyResult<Vec<HashMap<String, PyObject>>> {
    let _ = (py, data, config);
    Ok(vec![])
}

/// Function with Bound references to Python types.
#[pyfunction]
pub fn process_py_types<'py>(
    dict: &Bound<'py, PyDict>,
    list: &Bound<'py, PyList>,
    tuple: &Bound<'py, PyTuple>,
) -> PyResult<Bound<'py, PyList>> {
    let _ = (dict, tuple);
    Ok(list.clone())
}

/// Generic Python wrapper with complex bounds.
#[pyclass]
pub struct GenericWrapper<T>
where
    T: Clone + Send + Sync + 'static,
{
    inner: T,
}

/// PyO3 class with lifetime in Bound.
#[pyclass]
pub struct RefHolder {
    data: String,
}

#[pymethods]
impl RefHolder {
    #[new]
    fn new(data: String) -> Self {
        Self { data }
    }

    /// Method returning borrowed data.
    fn get_slice<'py>(&self, py: Python<'py>, start: usize, end: usize) -> PyResult<Bound<'py, pyo3::types::PyString>> {
        let slice = &self.data[start..end];
        Ok(pyo3::types::PyString::new(py, slice))
    }

    /// Method with complex callback signature.
    #[pyo3(signature = (callback, **kwargs))]
    fn apply_with_kwargs(
        &self,
        py: Python<'_>,
        callback: PyObject,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<PyObject> {
        match kwargs {
            Some(kw) => callback.call(py, (), Some(kw)),
            None => callback.call0(py),
        }
    }
}

/// Async PyO3 function with complex return.
#[pyfunction]
pub fn async_transform<'py>(
    py: Python<'py>,
    items: Vec<HashMap<String, PyObject>>,
) -> PyResult<Bound<'py, pyo3::coroutine::Coroutine>> {
    pyo3::coroutine::Coroutine::new::<_, _, PyObject>(
        pyo3::intern!(py, "async_transform"),
        async move {
            let _ = items;
            Ok(Python::with_gil(|py| py.None()))
        },
    )
}

/// Class with generic pymethods impl block.
#[pyclass]
pub struct Processor {
    buffer: Vec<u8>,
}

#[pymethods]
impl Processor {
    #[new]
    fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    /// Method with impl Trait in argument.
    fn process_iter(&mut self, data: &Bound<'_, PyList>) -> PyResult<usize> {
        let mut count = 0;
        for item in data.iter() {
            let bytes: Vec<u8> = item.extract()?;
            self.buffer.extend(bytes);
            count += 1;
        }
        Ok(count)
    }

    /// Returns Python iterator over internal data.
    fn iter_chunks<'py>(
        &self,
        py: Python<'py>,
        chunk_size: usize,
    ) -> PyResult<Bound<'py, PyList>> {
        let chunks: Vec<_> = self
            .buffer
            .chunks(chunk_size)
            .map(|c| c.to_vec())
            .collect();
        PyList::new(py, chunks)
    }

    /// Static method with complex signature.
    #[staticmethod]
    #[pyo3(signature = (items, *, key=None, reverse=false))]
    fn sort_items(
        py: Python<'_>,
        items: &Bound<'_, PyList>,
        key: Option<PyObject>,
        reverse: bool,
    ) -> PyResult<Bound<'_, PyList>> {
        let _ = (key, reverse);
        Ok(items.clone())
    }
}

/// FromPyObject with complex extraction.
#[derive(Debug)]
pub struct ComplexInput {
    pub values: Vec<(String, i64)>,
    pub metadata: HashMap<String, String>,
    pub flags: Vec<bool>,
}

impl<'py> FromPyObject<'py> for ComplexInput {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let dict = ob.downcast::<PyDict>()?;

        let values: Vec<(String, i64)> = dict
            .get_item("values")?
            .ok_or_else(|| pyo3::exceptions::PyKeyError::new_err("values"))?
            .extract()?;

        let metadata: HashMap<String, String> = dict
            .get_item("metadata")?
            .map(|v| v.extract())
            .transpose()?
            .unwrap_or_default();

        let flags: Vec<bool> = dict
            .get_item("flags")?
            .map(|v| v.extract())
            .transpose()?
            .unwrap_or_default();

        Ok(ComplexInput {
            values,
            metadata,
            flags,
        })
    }
}

#[pyfunction]
pub fn process_complex_input(input: ComplexInput) -> PyResult<usize> {
    Ok(input.values.len())
}
