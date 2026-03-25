# Tutorial: Document a PyO3 Hybrid Project

This tutorial walks you through documenting a project that combines Rust
and Python code via PyO3. By the end, you'll have a unified documentation
site with automatic cross-reference links between Rust implementations
and their Python bindings.

## Prerequisites

- [plissken installed](../how-to/install.md)
- Rust toolchain installed
- Python 3.8+ with pip
- Familiarity with [PyO3](https://pyo3.rs/) basics

## What You'll Build

A Rust library exposed to Python via PyO3, with documentation that:

- Shows both the Rust API and the Python API
- Links Rust structs to their Python class counterparts
- Links Python classes back to their Rust implementation
- Shows `Binding` badges on PyO3-generated items

## Step 1: Create the Project

Create the project structure:

```bash
mkdir textwrap-demo && cd textwrap-demo
```

Create `Cargo.toml`:

```toml
[package]
name = "textwrap-demo"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
pyo3 = { version = "0.22", features = ["extension-module"] }
```

Create `pyproject.toml`:

```toml
[project]
name = "textwrap-demo"
version = "0.1.0"

[tool.maturin]
python-source = "python"
module-name = "textwrap_demo._native"
```

## Step 2: Write the Rust Code

Create `src/lib.rs`:

```rust
//! # textwrap-demo
//!
//! A text wrapping library with Python bindings.
//!
//! Provides efficient text wrapping with configurable line width
//! and indentation options.

use pyo3::prelude::*;

/// Configuration for text wrapping behavior.
///
/// Controls how text is broken into lines and indented.
///
/// # Examples
///
/// ```rust
/// let config = WrapConfig::new(80);
/// assert_eq!(config.width, 80);
/// ```
#[pyclass]
#[derive(Debug, Clone)]
pub struct WrapConfig {
    /// Maximum line width in characters.
    #[pyo3(get, set)]
    pub width: usize,

    /// String to prepend to each wrapped line.
    #[pyo3(get, set)]
    pub indent: String,

    /// Whether to break words that exceed the line width.
    #[pyo3(get, set)]
    pub break_long_words: bool,
}

#[pymethods]
impl WrapConfig {
    /// Create a new wrap configuration.
    ///
    /// # Arguments
    ///
    /// * `width` - Maximum line width in characters
    #[new]
    pub fn new(width: usize) -> Self {
        Self {
            width,
            indent: String::new(),
            break_long_words: true,
        }
    }

    /// Set the indentation string.
    ///
    /// # Arguments
    ///
    /// * `indent` - String to prepend to wrapped lines
    ///
    /// # Returns
    ///
    /// The modified configuration (for chaining).
    pub fn with_indent(&mut self, indent: String) -> Self {
        self.indent = indent;
        self.clone()
    }
}

/// Wrap text to a specified width.
///
/// Breaks text into lines that fit within the configured width,
/// respecting word boundaries when possible.
///
/// # Arguments
///
/// * `text` - The text to wrap
/// * `config` - Wrapping configuration
///
/// # Returns
///
/// A vector of wrapped lines.
///
/// # Examples
///
/// ```rust
/// let config = WrapConfig::new(20);
/// let lines = wrap("Hello world, this is a test", &config);
/// assert!(lines.iter().all(|l| l.len() <= 20));
/// ```
#[pyfunction]
pub fn wrap(text: &str, config: &WrapConfig) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = config.indent.clone();

    for word in text.split_whitespace() {
        if current_line.len() + word.len() + 1 > config.width {
            if !current_line.trim().is_empty() {
                lines.push(current_line.trim_end().to_string());
            }
            current_line = config.indent.clone();
        }
        if current_line != config.indent || !config.indent.is_empty() {
            if current_line.len() > config.indent.len() {
                current_line.push(' ');
            }
        }
        current_line.push_str(word);
    }

    if !current_line.trim().is_empty() {
        lines.push(current_line.trim_end().to_string());
    }

    lines
}

/// A Python module for text wrapping.
#[pymodule]
fn _native(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<WrapConfig>()?;
    m.add_function(pyo3::wrap_pyfunction!(wrap, m)?)?;
    Ok(())
}
```

## Step 3: Write the Python Bindings

Create the Python package:

```bash
mkdir -p python/textwrap_demo
```

Create `python/textwrap_demo/__init__.py`:

```python
"""Text wrapping with configurable options.

This package provides a Python interface to the textwrap-demo
Rust library for efficient text wrapping.

Examples:
    >>> from textwrap_demo import WrapConfig, wrap
    >>> config = WrapConfig(40)
    >>> lines = wrap("A long paragraph...", config)
"""

from ._native import WrapConfig, wrap

__all__ = ["WrapConfig", "wrap"]
```

## Step 4: Initialize plissken

```bash
plissken init
```

Review the generated `plissken.toml`:

```toml
[project]
name = "textwrap-demo"
version_from = "cargo"

[output]
format = "markdown"
path = "docs/api"
template = "mkdocs-material"

[rust]
crates = ["."]

[python]
package = "textwrap_demo"
source = "python"
```

The `source = "python"` line was inferred from `[tool.maturin].python-source`
in your pyproject.toml.

## Step 5: Add Module Source Types

Open the generated `plissken.toml` and add a `[python.modules]` section
at the end. This tells plissken which Python modules come from PyO3
bindings (as opposed to pure Python):

```toml
[python.modules]
"textwrap_demo" = "pyo3"
```

The value `"pyo3"` means the module's contents come from Rust `#[pyclass]`
and `#[pyfunction]` definitions. This enables plissken to create
cross-reference links between the Python and Rust documentation pages.

## Step 6: Generate Documentation

```bash
plissken render
```

Output:

```
docs/api/
  textwrap_demo.md
  rust/
    textwrap-demo.md
  _nav.yml
```

## Step 7: Examine the Cross-References

Open `docs/api/textwrap_demo.md`. You'll see:

- A `Binding` badge next to `WrapConfig` indicating it comes from Rust
- A link under `WrapConfig`: **Rust Implementation**: `textwrap_demo::WrapConfig`
  pointing to the Rust documentation page
- The same for the `wrap` function

Open `docs/api/rust/textwrap-demo.md`. You'll see:

- The Rust documentation for `WrapConfig` and `wrap`
- A link under `WrapConfig`: **Python API**: `textwrap_demo.WrapConfig`
  pointing back to the Python page

These bidirectional links are generated automatically from the `#[pyclass]`
and `#[pyfunction]` attributes in your Rust code.

## Step 8: Set Up MkDocs and Browse

```bash
pip install mkdocs-material
```

Create `mkdocs.yml`:

```yaml
site_name: textwrap-demo
theme:
  name: material

nav:
  - Home: index.md
  - API Reference:
    - Python:
      - textwrap_demo: api/textwrap_demo.md
    - Rust:
      - textwrap-demo: api/rust/textwrap-demo.md
```

Create `docs/index.md`:

```markdown
# textwrap-demo

Text wrapping with Python and Rust.

## Python API

- [textwrap_demo](api/textwrap_demo.md) — Python package

## Rust API

- [textwrap-demo](api/rust/textwrap-demo.md) — Rust crate
```

Serve:

```bash
mkdocs serve
```

Navigate between the Python and Rust pages using the cross-reference links
to see the bidirectional linking in action.

## What You Learned

- How to configure plissken for a PyO3/maturin hybrid project
- How to specify module source types (`pyo3` vs `python`)
- How cross-reference links are automatically generated
- How to navigate between Python and Rust documentation pages

## Next Steps

- [Reference: Configuration](../reference/configuration.md) — Full `[python.modules]` options
- [How-To: Customize Templates](../how-to/customize-templates.md) — Change badge appearance
- [Explanation: Cross-References](../explanation/cross-references.md) — Deep dive into matching
