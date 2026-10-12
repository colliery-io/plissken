---
title: Configuration
---

plissken is configured via `plissken.toml` in your project root.

## Full Configuration Reference

```toml
[project]
# Project name (inferred from Cargo.toml or pyproject.toml if not set)
name = "myproject"

# Version source: "cargo", "pyproject", or "git"
version_from = "cargo"

[output]
# Output format (currently only "markdown" supported)
format = "markdown"

# Output directory for generated docs
path = "docs/api"

# Template/theme: "mkdocs-material", "mdbook" or "starlight"
template = "mkdocs-material"

[rust]
# Rust crates to document (paths relative to project root)
crates = [
    ".",           # Root crate
    "crates/core", # Workspace member
]

[python]
# Python package name
package = "mypackage"

# Optional: source directory (defaults to package name)
# source = "src/mypackage"
```

## Project Section

### `name`

Project name used in documentation titles. Auto-detected from:

1. `Cargo.toml` → `[package].name`
2. `pyproject.toml` → `[project].name`

### `version_from`

Where to read the version:

- `"cargo"` - From `Cargo.toml`
- `"pyproject"` - From `pyproject.toml`
- `"git"` - From git tags (default)

## Output Section

### `path`

Directory for generated documentation. Relative to project root.

```toml
path = "docs/api"      # Recommended for MkDocs
path = "src"           # For mdBook
path = "docs/src/content/docs/reference/api"  # For Starlight
```

### `template`

Documentation template/theme:

| Value | Description |
|-------|-------------|
| `mkdocs-material` | MkDocs with Material theme (recommended) |
| `mdbook` | Rust-style mdBook |
| `starlight` | Astro Starlight, with or without starlight-aurora. See [Starlight Sites](../../how-to/starlight/) |

## Rust Section

### `crates`

List of Rust crates to document:

```toml
[rust]
crates = [
    ".",                    # Root crate
    "crates/plissken-core", # Workspace member
]
```

For workspaces, list each member you want documented.

### `entry_point`

The crate whose name the Python package replaces, for PyO3 bindings that no
`#[pymodule]` function adds (see [Hybrid Projects](../../how-to/hybrid/)).
Without it, the bindings of every crate are in the package.

```toml
[rust]
entry_point = "mycrate"
```

### `include_private`

plissken documents the public API: items that are `pub`, and the fields and
methods that are `pub`. Test code (`#[cfg(test)]` modules and items, `#[test]`
functions) is never documented. To document the other items too:

```toml
[rust]
include_private = true
```

PyO3 bindings are documented either way: they are part of the Python API.

## Python Section

### `package`

Python package name to document:

```toml
[python]
package = "mypackage"
```

### `source`

Optional source directory override:

```toml
[python]
package = "mypackage"
source = "src/mypackage"  # If not in standard location
```

`source` is the folder of the package itself (it holds `__init__.py`), or a
folder that holds the package. Modules are named from the package either way:
`mypackage.client`.

### `include_private`

Names that start with `_` (`_helper`, `_DEFAULT_TIMEOUT`) are private and not
documented; `__dunder__` methods are. To document them too:

```toml
[python]
include_private = true
```

## Validation

Check your configuration:

```bash
plissken check
```

This validates:

- Required fields are present
- Paths exist
- Rust crates compile
- Python packages are importable

## Template Overrides

Create custom templates in `.plissken/templates/`:

```
.plissken/
  templates/
    partials/
      badge.html       # Override badge appearance
      signature.html   # Override function signatures
    module.html        # Override module page layout
```

Templates use [Tera](https://keats.github.io/tera/) syntax (similar to Jinja2).
