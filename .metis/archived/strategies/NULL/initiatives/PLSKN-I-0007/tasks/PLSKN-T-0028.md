---
id: infer-config-defaults-from-cargo
level: task
title: "Infer config defaults from Cargo.toml and pyproject.toml"
short_code: "PLSKN-T-0028"
created_at: 2026-01-16T17:58:35.014579+00:00
updated_at: 2026-01-17T01:25:28.027166+00:00
parent: PLSKN-I-0007
blocked_by: []
archived: true

tags:
  - "#task"
  - "#phase/completed"


exit_criteria_met: false
strategy_id: NULL
initiative_id: PLSKN-I-0007
---

# Infer config defaults from Cargo.toml and pyproject.toml

## Parent Initiative

[[PLSKN-I-0007]] Configuration Simplification

## Objective

Reduce required configuration by inferring sensible defaults from existing manifest files. When Cargo.toml or pyproject.toml exists, extract project name, entry point, and other metadata instead of requiring explicit configuration.

## Current Problem

Users must redundantly specify information already in their manifests:
```toml
# plissken.toml
[project]
name = "my-lib"        # Already in Cargo.toml [package].name
version_from = "cargo"

[rust]
entry_point = "my_lib" # Already in Cargo.toml [lib].name
```

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [x] Parse Cargo.toml to extract: package name, lib name (entry_point), crate paths
- [x] Parse pyproject.toml to extract: project name, package directory
- [x] Use inferred values as defaults when config fields are omitted
- [x] Explicit config values always override inferred defaults
- [x] Handle workspace Cargo.toml (multiple crates)
- [x] Support both `[project]` (PEP 621) and `[tool.maturin]` sections in pyproject.toml

## Inference Rules

### From Cargo.toml
| plissken.toml field | Inferred from |
|---------------------|---------------|
| `project.name` | `[package].name` |
| `rust.entry_point` | `[lib].name` or package name with `-` → `_` |
| `rust.crates` | Current directory if Cargo.toml exists |

### From pyproject.toml
| plissken.toml field | Inferred from |
|---------------------|---------------|
| `project.name` | `[project].name` or `[tool.poetry].name` |
| `python.package` | `[project].name` or directory containing `__init__.py` |
| `python.source` | `src/` if src-layout, else `.` |

## Minimal Config Examples

### Pure Rust (with Cargo.toml)
```toml
# plissken.toml - just 4 lines!
[output]
format = "markdown"
path = "docs/api"
template = "mkdocs-material"
```
Everything else inferred from Cargo.toml.

### Pure Python (with pyproject.toml)
```toml
# plissken.toml
[output]
format = "markdown"
path = "docs/api"
template = "mkdocs-material"

[python]
auto_discover = true
```
Package name and source inferred from pyproject.toml.

## Implementation Notes

### Files to Modify

1. **`crates/plissken-core/src/manifest.rs`** (new) - Manifest parsing:
   ```rust
   pub struct CargoManifest {
       pub package_name: String,
       pub lib_name: Option<String>,
       pub workspace_members: Vec<PathBuf>,
   }
   
   pub struct PyProjectManifest {
       pub project_name: String,
       pub source_layout: SourceLayout,
   }
   
   pub enum SourceLayout {
       Src,      // src/package/
       Flat,     // package/
       Unknown,
   }
   
   pub fn parse_cargo_toml(path: &Path) -> Result<CargoManifest>;
   pub fn parse_pyproject_toml(path: &Path) -> Result<PyProjectManifest>;
   ```

2. **`crates/plissken-core/src/config.rs`** - Apply defaults:
   ```rust
   impl Config {
       pub fn with_inferred_defaults(mut self, project_root: &Path) -> Result<Self> {
           // Try to load manifests
           let cargo = project_root.join("Cargo.toml");
           let pyproject = project_root.join("pyproject.toml");
           
           if cargo.exists() {
               let manifest = parse_cargo_toml(&cargo)?;
               self.apply_cargo_defaults(&manifest);
           }
           
           if pyproject.exists() {
               let manifest = parse_pyproject_toml(&pyproject)?;
               self.apply_pyproject_defaults(&manifest);
           }
           
           Ok(self)
       }
   }
   ```

3. **`crates/plissken/src/commands/generate.rs`** - Call inference

### Cargo.toml Parsing Details

```rust
// Handle workspace vs single crate
if let Some(workspace) = &cargo_toml.workspace {
    // Multi-crate workspace
    for member in &workspace.members {
        // Expand globs, find all Cargo.toml
    }
} else {
    // Single crate
}

// Get lib name (defaults to package name with - → _)
let lib_name = cargo_toml.lib
    .as_ref()
    .and_then(|l| l.name.clone())
    .unwrap_or_else(|| package_name.replace('-', "_"));
```

### Dependencies

- Add `cargo_toml` crate for Cargo.toml parsing
- Already have `toml` for pyproject.toml

## Status Updates

### Session 2026-01-17

**Completed implementation:**

1. **Created `manifest.rs` module** (`crates/plissken-core/src/manifest.rs`):
   - `CargoManifest` struct: parses `[package]` and `[workspace]` sections
   - `PyProjectManifest` struct: parses `[project]`, `[tool.maturin]`, and `[tool.setuptools]`
   - `InferredConfig`: aggregates inference from both manifest types
   - Handles workspace Cargo.toml with members
   - Extracts `python-source` from `[tool.maturin]` for hybrid projects

2. **Added `Config::with_inferred_defaults()` method** (`crates/plissken-core/src/config.rs`):
   - Fills empty `project.name` from manifest
   - Fills empty `rust.crates` from workspace members or single crate
   - Fills empty `rust.entry_point` from package name
   - Fills empty `python.package` from project name (with dash→underscore)
   - Fills empty `python.source` from maturin config
   - Explicit config values always preserved (override inferred)

3. **Inference priority**:
   - pyproject.toml `[project].name` takes precedence for project name
   - Cargo.toml used for Rust-specific inference
   - Both can coexist for hybrid projects

**Tests added:** 17 new tests (10 in manifest.rs, 7 in config.rs)
**All 167 tests passing**

**Files modified:**
- `crates/plissken-core/src/manifest.rs` (new)
- `crates/plissken-core/src/config.rs` (added method + tests)
- `crates/plissken-core/src/lib.rs` (exports)