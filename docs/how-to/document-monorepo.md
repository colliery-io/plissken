# How To: Document a Monorepo

For projects with multiple Rust crates in a Cargo workspace, plissken can
document all crates in a single run and generate unified navigation.

## Workspace Configuration

List each crate you want documented in `rust.crates`:

```toml
[project]
name = "myproject"
version_from = "cargo"

[output]
path = "docs/api"
template = "mkdocs-material"

[rust]
crates = [
    "crates/core",
    "crates/cli",
    "crates/utils",
]
```

Each path should point to a directory containing a `Cargo.toml`. plissken
reads the `[package].name` from each to determine the crate name.

## Entry Point for Cross-References

If your workspace includes PyO3 bindings, set `entry_point` to the crate
that contains the `#[pymodule]`, `#[pyclass]`, and `#[pyfunction]` definitions:

```toml
[rust]
crates = [
    "crates/core",
    "crates/bindings",
]
entry_point = "myproject_bindings"
```

Only the entry point crate is scanned for cross-references. Other crates
are documented as pure Rust.

## Hybrid Monorepo (Rust + Python)

For a workspace that also has a Python package:

```toml
[project]
name = "myproject"
version_from = "cargo"

[output]
path = "docs/api"
template = "mkdocs-material"
prefix = "api"

[rust]
crates = [
    "crates/core",
    "crates/bindings",
]
entry_point = "myproject_bindings"

[python]
package = "mypackage"
source = "python/mypackage"
auto_discover = true
```

## Output Structure

Each crate gets its own section in the generated navigation. For a workspace
with `core` and `cli` crates:

```
docs/api/
  rust/
    core.md                    # Core crate root
    core/config.md             # Core submodules
    core/utils.md
    cli.md                     # CLI crate root
  _nav.yml
```

The navigation groups submodules under their parent crate as collapsible
sections:

```yaml
nav:
  - Rust:
    - core:
      - api/rust/core.md
      - config: api/rust/core/config.md
      - utils: api/rust/core/utils.md
    - cli: api/rust/cli.md
```

## Selective Documentation

You don't have to document every workspace member. Only crates listed in
`rust.crates` are included. This is useful for skipping internal or test
crates:

```toml
[rust]
# Document public crates only, skip internal test helpers
crates = [
    "crates/core",
    "crates/bindings",
]
# Omitting: crates/test-helpers, crates/bench
```

## Auto-Detection with `plissken init`

For Cargo workspaces, `plissken init` reads `[workspace].members` and
populates `rust.crates` automatically:

```bash
plissken init
```

Review the generated `plissken.toml` and remove any crates you don't want
documented.
