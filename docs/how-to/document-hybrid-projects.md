# Hybrid Projects

Guide for documenting Rust-Python hybrid projects using PyO3 and maturin.

## Configuration

```toml
[project]
name = "myproject"
version_from = "cargo"

[output]
path = "docs/api"
template = "mkdocs-material"

[rust]
crates = ["."]

[python]
package = "mypackage"
```

## Cross-Reference Detection

plissken automatically detects PyO3 bindings:

### `#[pyclass]`

```rust
/// A data container exposed to Python.
///
/// This struct is available in Python as `mypackage.Container`.
#[pyclass]
#[derive(Debug, Clone)]
pub struct Container {
    #[pyo3(get, set)]
    pub value: i32,
}
```

In the Rust documentation, you'll see:

> **Python API**: `mypackage.Container` → links to Python doc page

### `#[pymethods]`

```rust
#[pymethods]
impl Container {
    /// Create a new container.
    #[new]
    pub fn new(value: i32) -> Self {
        Self { value }
    }

    /// Double the stored value.
    pub fn double(&mut self) {
        self.value *= 2;
    }
}
```

### `#[pyfunction]`

```rust
/// Process data and return results.
///
/// This function is exposed to Python as `mypackage.process`.
#[pyfunction]
pub fn process(data: Vec<String>) -> PyResult<Vec<String>> {
    Ok(data.into_iter().map(|s| s.to_uppercase()).collect())
}
```

## Python Side

The Python bindings appear in docs with links back to Rust:

```python
class Container:
    """A data container.

    Attributes:
        value: The stored integer value.
    """

    def __init__(self, value: int) -> None:
        """Create a new container."""
        ...

    def double(self) -> None:
        """Double the stored value."""
        ...
```

In the Python documentation, you'll see:

> **Rust Implementation**: `mycrate::Container` → links to Rust doc page

## Cross-Reference Links

### Struct ↔ Class

| Rust | Python |
|------|--------|
| `mycrate::Container` | `mypackage.Container` |

Links appear in both directions automatically.

### Method Links

Method-level cross-references include anchors:

- Rust `Container::double` → Python `Container.double`
- Python `Container.double` → Rust `Container::double`

## Best Practices

### 1. Document Both Sides

Write doc comments in Rust:

```rust
/// Process input data.
///
/// # Arguments
///
/// * `data` - Input strings to process
///
/// # Returns
///
/// Processed strings in uppercase.
#[pyfunction]
pub fn process(data: Vec<String>) -> Vec<String> {
    // ...
}
```

plissken will show this documentation on both the Rust and Python pages.

### 2. Use Type Stubs

For complex Python types, create `.pyi` stub files:

```python
# mypackage/__init__.pyi
from typing import List

class Container:
    value: int
    def __init__(self, value: int) -> None: ...
    def double(self) -> None: ...

def process(data: List[str]) -> List[str]: ...
```

### 3. Consistent Naming

PyO3 allows renaming:

```rust
#[pyclass(name = "PyContainer")]
pub struct Container { ... }

#[pyfunction(name = "py_process")]
pub fn process(...) { ... }
```

plissken uses the Python-visible names for cross-references.

## Project Structure

Typical hybrid project layout:

```
myproject/
├── Cargo.toml
├── pyproject.toml
├── plissken.toml
├── src/
│   └── lib.rs           # Rust implementation
├── mypackage/
│   ├── __init__.py      # Re-exports
│   └── py.typed         # PEP 561 marker
└── docs/
    └── api/             # Generated docs
```
