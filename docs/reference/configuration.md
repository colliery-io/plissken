# Configuration Reference

plissken is configured via `plissken.toml` in your project root.

## `[project]`

Project metadata.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | string | Inferred from Cargo.toml or pyproject.toml | Project name used in documentation titles. |
| `version_from` | `"cargo"` \| `"pyproject"` \| `"git"` | `"git"` | Where to read the project version. |

### `name`

If omitted or empty, plissken infers the name from:

1. `pyproject.toml` → `[project].name` (takes precedence)
2. `Cargo.toml` → `[package].name`

### `version_from`

| Value | Source | Requires |
|-------|--------|----------|
| `"git"` | Current git ref (branch name or tag) and commit hash | A git repository |
| `"cargo"` | `[package].version` field | `Cargo.toml` in project root |
| `"pyproject"` | `[project].version` field | `pyproject.toml` in project root |

---

## `[output]`

Output configuration.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `format` | string | `"markdown"` | Output format. Currently only `"markdown"` is supported. |
| `path` | string | `"docs/api"` | Output directory, relative to project root. |
| `template` | string | None | Theme template name. If omitted, falls back to the minimal theme with hardcoded hex colors. |
| `prefix` | string | None | Path prefix for navigation entries. Prepended to all file paths in `_nav.yml` / `SUMMARY.md`. Use when rendering into a subfolder of an existing doc site. |

### `template`

| Value | SSG | Content Directory | Navigation File |
|-------|-----|-------------------|-----------------|
| `"mkdocs-material"` | MkDocs with Material theme | Uses output path directly | `_nav.yml` |
| `"mdbook"` | mdBook | Uses output path directly | `SUMMARY.md` |

If `template` is omitted, plissken uses a minimal theme with hardcoded hex
colors instead of CSS variables. **Always set this field explicitly** — most
projects should use `"mkdocs-material"`.

Accepted aliases:

- MkDocs: `"mkdocs-material"`, `"mkdocs_material"`, `"material"`
- mdBook: `"mdbook"`, `"md-book"`, `"md_book"`

Any unrecognized template name falls back to a minimal theme with hardcoded
hex colors (no CSS variables).

### `prefix`

When rendering into a subfolder of an existing doc site, navigation file paths
need to reflect the mount point. The `prefix` field prepends a path to all
entries in `_nav.yml` (MkDocs) or `SUMMARY.md` (mdBook).

```toml
[output]
path = "docs/api"
prefix = "api"     # nav entries become api/rust/mycrate.md
```

Without prefix, nav entries are relative to the output directory:
`rust/mycrate.md`. With `prefix = "api"`, they become `api/rust/mycrate.md` —
correct for inclusion in a parent `mkdocs.yml` where `docs/` is the content
root.

The CLI `--prefix` flag overrides this config value. An empty string or
trailing slashes are normalized (treated as no prefix).

---

## `[rust]`

Rust source configuration. Omit this section entirely if you have no Rust
code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `crates` | list of strings | Inferred from Cargo.toml workspace members | Paths to Rust crates, relative to project root. |
| `entry_point` | string | Inferred from Cargo.toml `[package].name` | Main crate name for cross-referencing. |

### `crates`

Each entry is a path to a directory containing a `Cargo.toml`:

```toml
[rust]
crates = [
    ".",                        # Root crate
    "crates/core",              # Workspace member
    "crates/cli",               # Another workspace member
]
```

For a non-workspace project, use `["."]`.

plissken reads the `[package].name` from each crate's `Cargo.toml` to
determine the crate name for documentation. It then recursively finds all
`.rs` files under each crate's `src/` directory.

### `entry_point`

The crate name used as the entry point for PyO3 cross-referencing. This
tells plissken which Rust crate contains the `#[pyclass]`, `#[pyfunction]`,
and `#[pymethods]` definitions.

If omitted, plissken infers it from the root `Cargo.toml`'s `[package].name`.

---

## `[python]`

Python source configuration. Omit this section entirely if you have no
Python code.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `package` | string | Inferred from pyproject.toml `[project].name` | Python package name. |
| `source` | string | Same as `package` | Path to the Python source directory, relative to project root. |
| `auto_discover` | bool | `false` | Walk the filesystem to discover modules automatically. |
| `modules` | table | `{}` | Explicit module-to-source-type mapping. |

### `package`

The top-level Python package name. If inferred from pyproject.toml, dashes
are converted to underscores (`my-package` becomes `my_package`).

### `source`

The directory containing Python source files. Defaults to the package name.
Set this when your Python source is in a non-standard location:

```toml
[python]
package = "mypackage"
source = "python"          # maturin convention
# source = "src/mypackage" # src layout
```

### `auto_discover`

When `true`, plissken walks the source directory to find all `.py` files
and converts their paths to dotted module names. It skips:

- `__pycache__/`
- `.venv/`, `venv/`, `.env/`, `env/`
- `.tox/`, `.nox/`, `.pytest_cache/`, `.mypy_cache/`, `.ruff_cache/`
- `node_modules/`, `.git/`, `build/`, `dist/`
- Any directory ending in `.egg-info`

### `modules`

Explicit mapping of module names to source types:

```toml
[python.modules]
"mypackage" = "pyo3"
"mypackage.core" = "pyo3"
"mypackage.helpers" = "python"
"mypackage.utils" = "python"
```

| Source Type | Meaning |
|-------------|---------|
| `"pyo3"` | Module contents come from Rust PyO3 bindings. Cross-reference links are generated. |
| `"python"` | Pure Python source. No cross-references. |

Explicit modules override auto-discovered modules with the same name.

When a module is marked as `"pyo3"` but has no corresponding Python source
file, plissken synthesizes a Python module from the Rust PyO3 bindings. See
[How Cross-References Work](../explanation/cross-references.md) for details on
the synthesis algorithm and heuristic source type detection.

---

## `[links]`

External linking configuration.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `dependencies` | `"cargo_lock"` \| `"cargo_toml"` \| `"none"` | `"cargo_lock"` | Where to resolve dependency versions for docs.rs links. |
| `docs_rs_base` | string | `"https://docs.rs"` | Base URL for external Rust documentation links. |

### `dependencies`

Controls how plissken resolves dependency versions for generating links to
external crate documentation on docs.rs:

| Value | Source | Precision |
|-------|--------|-----------|
| `"cargo_lock"` | `Cargo.lock` | Exact resolved versions |
| `"cargo_toml"` | `Cargo.toml` dependency declarations | Version requirements (may not be exact) |
| `"none"` | Disabled | No external links generated |

---

## `[quality]`

Documentation quality settings.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `require_docstrings` | bool | `false` | Warn when public items lack documentation. |
| `min_coverage` | float | None | Minimum documentation coverage ratio (0.0 to 1.0). Measured as the fraction of public items that have doc comments or docstrings. |
| `fail_on_broken_links` | bool | `false` | Treat broken cross-reference links as errors instead of warnings. When enabled, `plissken render` exits with a non-zero code if any cross-reference cannot be resolved. |

---

## Complete Example

```toml
[project]
name = "myproject"
version_from = "cargo"

[output]
format = "markdown"
path = "docs/api"
template = "mkdocs-material"
prefix = "api"

[rust]
crates = ["crates/core", "crates/bindings"]
entry_point = "myproject_core"

[python]
package = "myproject"
source = "python"
auto_discover = false

[python.modules]
"myproject" = "pyo3"
"myproject.core" = "pyo3"
"myproject.helpers" = "python"
"myproject.utils" = "python"

[links]
dependencies = "cargo_lock"
docs_rs_base = "https://docs.rs"

[quality]
require_docstrings = true
min_coverage = 0.85
fail_on_broken_links = true
```

## Config File Location

plissken searches for `plissken.toml` starting from the given path:

1. If the path points directly to a `.toml` file, use it.
2. If the path is a directory, look for `plissken.toml` in that directory.
3. Walk up parent directories until `plissken.toml` is found or the
   filesystem root is reached.

## Inferred Defaults

When a field is omitted, plissken attempts to infer values from manifest
files. Explicit configuration always takes precedence.

| Field | Inferred From |
|-------|---------------|
| `project.name` | pyproject.toml `[project].name` (preferred) or Cargo.toml `[package].name` |
| `rust.crates` | Cargo.toml `[workspace].members` or `["."]` for single crates |
| `rust.entry_point` | Cargo.toml `[package].name` |
| `python.package` | pyproject.toml `[project].name` (dashes → underscores) |
| `python.source` | pyproject.toml `[tool.maturin].python-source` |
