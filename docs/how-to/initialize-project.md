# How To: Initialize a Project

## Auto-Detection

Run `plissken init` in your project root:

```bash
plissken init
```

plissken inspects the current directory for:

- `Cargo.toml` — detects Rust crate or workspace
- `pyproject.toml` — detects Python package name and source directory
- Workspace members from `[workspace].members`
- Python source directory from `[tool.maturin].python-source`

The generated `plissken.toml` includes everything it found.

## Overwrite an Existing Config

If `plissken.toml` already exists, init refuses to overwrite it. Use
`--force` to replace it:

```bash
plissken init --force
```

## Manual Configuration

If auto-detection doesn't match your layout, create `plissken.toml`
manually. The minimal required configuration depends on your project type.

### Pure Python

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

### Pure Rust

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

### Hybrid (PyO3)

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
```

## After Initialization

Validate the configuration:

```bash
plissken check
```

Then generate documentation:

```bash
plissken render
```

See the [Configuration Reference](../reference/configuration.md) for all
available options.
