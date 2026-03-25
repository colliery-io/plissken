# Architecture Overview

This page explains how plissken is structured internally, the major
subsystems, and how data flows through the pipeline.

## Crate Structure

plissken is a Cargo workspace with two crates:

```
plissken/
├── crates/
│   ├── plissken-cli/     # Binary crate (the `plissken` command)
│   └── plissken-core/    # Library crate (all the logic)
```

### plissken-core

The library crate contains all parsing, modeling, cross-referencing, and
rendering logic. It has no CLI dependencies and is published to crates.io
as a reusable library.

Key modules:

| Module | Responsibility |
|--------|---------------|
| `config` | Configuration loading, validation, and defaults |
| `manifest` | Cargo.toml and pyproject.toml parsing |
| `parser` | Source code parsing (Rust via `syn`, Python via `tree-sitter`) |
| `model` | The `DocModel` data structure and all its types |
| `crossref` | Cross-reference building between Python and Rust items |
| `discover` | Python module auto-discovery via filesystem walking |
| `docstring` | Docstring parsing (Google, NumPy, Rust styles) |
| `render` | Markdown rendering, Tera templates, theme adapters, SSG adapters |
| `error` | Unified error types |

### plissken-cli

The binary crate provides the command-line interface using `clap`. It
orchestrates the pipeline by calling into plissken-core:

1. Load config
2. Parse sources
3. Build cross-references
4. Render output

All non-trivial logic lives in plissken-core. The CLI handles argument
parsing, file I/O, progress output, and error formatting.

## Pipeline

The documentation pipeline has four stages:

```
Source Files → Parse → Model → Render → Output Files
```

### Stage 1: Configuration

1. Find and load `plissken.toml`
2. Apply inferred defaults from Cargo.toml and pyproject.toml
3. Validate paths and settings
4. Resolve the output directory and template

### Stage 2: Parsing

Rust and Python sources are parsed independently:

**Rust** (via `syn`):
- Each `.rs` file in configured crate `src/` directories is parsed
- The `syn` crate provides a full AST
- plissken extracts structs, enums, functions, traits, impl blocks, consts, type aliases
- Doc comments are extracted and parsed into structured `ParsedDocstring`
- PyO3 attributes (`#[pyclass]`, `#[pyfunction]`, `#[pymethods]`) are detected

**Python** (via `tree-sitter`):
- Each `.py` file in the configured package directory is parsed
- tree-sitter provides a concrete syntax tree
- plissken extracts classes, functions, variables, decorators
- Docstrings are extracted and parsed (Google or NumPy format)
- Type annotations from function signatures are captured

Parse errors are non-fatal. Files that fail to parse are skipped with a
warning, and the rest of the project continues processing.

### Stage 3: Cross-Referencing

For hybrid projects, the cross-reference builder:

1. **Indexes Rust items**: Scans all Rust modules for `#[pyclass]`,
   `#[pyfunction]`, and `#[pymethods]` attributes. Builds a lookup map
   from Python-visible names to Rust items.

2. **Matches Python items**: For each Python module marked as `pyo3` in
   the config, matches Python classes and functions to their Rust
   counterparts using the indexed names.

3. **Synthesizes bindings**: For PyO3 modules, if no Python source file
   exists, plissken can synthesize Python module documentation directly
   from the Rust code's `#[pyclass]` and `#[pyfunction]` definitions.

4. **Produces CrossRef records**: Each match becomes a `CrossRef` with
   the Python path, Rust path, and relationship type (`Binding`, `Wraps`,
   or `Delegates`).

### Stage 4: Rendering

The rendering stage converts the `DocModel` into Markdown files:

1. **Module renderer**: Iterates over Python and Rust modules, generating
   a Markdown page for each one. Uses inline rendering (all items on one
   page per module) rather than separate files per item.

2. **Template engine**: Tera templates render badges, signatures, and code
   blocks. Theme adapters inject the correct CSS variable references for
   the target SSG.

3. **SSG adapter**: Generates navigation files and configuration specific
   to the target static site generator (MkDocs or mdBook).

4. **File output**: Writes Markdown files and SSG files to the configured
   output directory.

## API Tiers

plissken-core's public API is organized into two tiers:

### Core API (root imports)

~20 types for typical usage:

```rust
use plissken_core::{Config, RustParser, PythonParser, DocModel, Renderer};
```

### Detail API (`detail::` module)

Advanced types for fine-grained control:

```rust
use plissken_core::detail::{RustStruct, PythonFunction, Visibility};
```

This separation keeps the default import surface small while providing
full access when needed.

## Dependencies

Key external dependencies:

| Dependency | Used For |
|------------|----------|
| `syn` | Rust source code parsing (full AST) |
| `tree-sitter` + `tree-sitter-python` | Python source code parsing |
| `tera` | Template rendering (Jinja2-like) |
| `clap` | CLI argument parsing |
| `serde` + `serde_json` + `toml` | Serialization (config, JSON model) |
| `thiserror` | Error type definitions |
| `anyhow` | Error handling in the CLI |
| `walkdir` | Recursive directory traversal |

## Error Handling

plissken-core defines a unified `PlisskenError` enum covering all error
categories:

- `ConfigNotFound`, `ConfigParse`, `ConfigValidation` — config issues
- `Parse`, `FileRead` — source code problems
- `Template`, `OutputWrite` — rendering failures
- `CrossRef` — cross-reference resolution
- `Discovery` — module discovery
- `ManifestParse` — Cargo.toml/pyproject.toml issues

The CLI adds its own `CliError` for user-facing messages with recovery
hints.
