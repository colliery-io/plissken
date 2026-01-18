# Overview

plissken is designed to generate unified documentation for projects that combine Rust and Python code.

## How It Works

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│   Source Code   │────▶│    Parsers      │────▶│   Doc Model     │
│  .py / .rs      │     │  Python / Rust  │     │   (JSON)        │
└─────────────────┘     └─────────────────┘     └─────────────────┘
                                                        │
                                                        ▼
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│   Doc Site      │◀────│   Renderer      │◀────│  Cross-Refs     │
│  MkDocs/mdBook  │     │   + Templates   │     │  Py ↔ Rust      │
└─────────────────┘     └─────────────────┘     └─────────────────┘
```

### 1. Parsing

plissken parses your source code to extract:

- **Python**: Classes, functions, methods, docstrings, type hints
- **Rust**: Structs, enums, functions, impl blocks, doc comments

### 2. Cross-Reference Detection

For PyO3/maturin projects, plissken detects bindings:

- `#[pyclass]` structs → Python classes
- `#[pyfunction]` functions → Python functions
- `#[pymethods]` impl blocks → Python methods

### 3. Rendering

Documentation is rendered to Markdown with:

- Syntax-highlighted signatures
- Parameter tables with types
- Cross-reference links
- Badge indicators (async, unsafe, binding, etc.)

## Supported Project Types

| Type | Python | Rust | Cross-Refs |
|------|--------|------|------------|
| Pure Python | Yes | - | - |
| Pure Rust | - | Yes | - |
| Hybrid (PyO3) | Yes | Yes | Yes |

## Output Structure

```
docs/api/
├── python/
│   └── mypackage/
│       ├── index.md          # Package overview
│       ├── MyClass.md        # Class page
│       └── my_function.md    # Function page
├── rust/
│   └── mycrate/
│       ├── index.md          # Module overview
│       ├── MyStruct.md       # Struct page
│       └── my_function.md    # Function page
└── _nav.yml                  # Navigation config
```

## Key Features

### Docstring Support

Python docstrings are parsed in multiple formats:

- **Google style** (recommended)
- **NumPy style**

Rust doc comments (`///` and `//!`) are fully supported.

### Type Information

Types are extracted from:

- Python type hints (`def foo(x: int) -> str`)
- Rust function signatures
- Docstring type annotations

### Badges

Visual indicators for:

- `async` functions
- `unsafe` Rust code
- `@property`, `@classmethod`, `@staticmethod`
- Visibility (`pub`, `pub(crate)`, private)
- Source type (Python, Rust, Binding)
