# How To: Configure for Different Project Types

## Pure Python Project

```toml
[project]
name = "mypackage"
version_from = "pyproject"

[output]
path = "docs/api"
template = "mkdocs-material"

[python]
package = "mypackage"
```

If your Python source is not at the project root (e.g., in a `src/` layout):

```toml
[python]
package = "mypackage"
source = "src/mypackage"
```

### Auto-Discovery

To let plissken find modules by walking the filesystem instead of listing
them explicitly:

```toml
[python]
package = "mypackage"
auto_discover = true
```

### Explicit Module Listing

For fine-grained control over which modules are documented and their types:

```toml
[python.modules]
"mypackage" = "python"
"mypackage.core" = "python"
"mypackage.utils" = "python"
```

## Pure Rust Project

```toml
[project]
name = "mycrate"
version_from = "cargo"

[output]
path = "docs/api"
template = "mkdocs-material"

[rust]
crates = ["."]
```

### Workspace

For a Cargo workspace, list the crates you want documented:

```toml
[rust]
crates = [
    "crates/core",
    "crates/cli",
    "crates/utils",
]
```

plissken reads each crate's `Cargo.toml` to determine the crate name.

### Entry Point

For cross-referencing, set the main crate that other crates depend on:

```toml
[rust]
crates = ["core", "bindings"]
entry_point = "my_core_crate"
```

## Hybrid Project (PyO3/maturin)

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
source = "python"

[python.modules]
"mypackage" = "pyo3"
"mypackage.helpers" = "python"
```

The `[python.modules]` section maps each module to its source type:

- `"pyo3"` — Module contents come from Rust `#[pyclass]`/`#[pyfunction]`. Cross-reference links are generated.
- `"python"` — Pure Python module. No cross-references.

### Separate Bindings Crate

If your Rust bindings are in a separate crate from your core library:

```toml
[rust]
crates = ["core", "bindings"]
entry_point = "my_core_crate"
```

Both crates are documented, and cross-references point to the correct
Rust items.

## Version Source

The `version_from` field controls where plissken reads the project version:

| Value | Source | File |
|-------|--------|------|
| `"cargo"` | Rust package version | `Cargo.toml` |
| `"pyproject"` | Python project version | `pyproject.toml` |
| `"git"` | Current git ref/tag | `.git/` |

`"git"` is the default and requires a git repository.

## Quality Settings

Enable documentation quality checks:

```toml
[quality]
require_docstrings = true
min_coverage = 0.85
fail_on_broken_links = true
```

## External Links

Configure how dependency links are resolved:

```toml
[links]
dependencies = "cargo_lock"  # or "cargo_toml" or "none"
docs_rs_base = "https://docs.rs"
```
