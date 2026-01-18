---
id: implement-plissken-init-command
level: task
title: "Implement plissken init command"
short_code: "PLSKN-T-0024"
created_at: 2026-01-16T17:51:44.571442+00:00
updated_at: 2026-01-17T00:42:23.494410+00:00
parent: PLSKN-I-0006
blocked_by: []
archived: true

tags:
  - "#task"
  - "#phase/completed"


exit_criteria_met: false
strategy_id: NULL
initiative_id: PLSKN-I-0006
---

# Implement plissken init command

## Parent Initiative

[[PLSKN-I-0006]] CLI UX: Invisible by Default

## Objective

Replace the current stub `init` command with a working implementation that auto-detects project type (Rust, Python, hybrid) and generates an appropriate `plissken.toml` configuration file.

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [x] `plissken init` creates a valid `plissken.toml` in current directory
- [x] Auto-detects Cargo.toml → enables `[rust]` section
- [x] Auto-detects pyproject.toml/setup.py → enables `[python]` section
- [x] Detects hybrid projects (both) and configures appropriately
- [x] Infers project name from Cargo.toml or pyproject.toml
- [x] Sets sensible defaults for output path (`docs/api`)
- [x] Won't overwrite existing plissken.toml without `--force`
- [x] Silent on success (respects verbosity settings)

## Generated Config Examples

### Pure Rust Project
```toml
[project]
name = "my-crate"  # from Cargo.toml
version_from = "cargo"

[output]
format = "markdown"
path = "docs/api"
template = "mkdocs-material"

[rust]
crates = ["."]
entry_point = "my_crate"
```

### Pure Python Project
```toml
[project]
name = "my-package"  # from pyproject.toml
version_from = "pyproject"

[output]
format = "markdown"
path = "docs/api"
template = "mkdocs-material"

[python]
package = "my_package"
source = "src"
```

### Hybrid Project (PyO3)
```toml
[project]
name = "my-lib"
version_from = "cargo"

[output]
format = "markdown"
path = "docs/api"
template = "mkdocs-material"

[rust]
crates = ["."]
entry_point = "my_lib"

[python]
package = "my_lib"
source = "python"
```

## Implementation Notes

### Files to Create/Modify

1. **`crates/plissken/src/commands/init.rs`** - New command implementation:
   ```rust
   pub fn run_init(force: bool, verbose: u8) -> Result<()> {
       let project = detect_project_type()?;
       let config = generate_config(&project)?;
       write_config(&config, force)?;
       Ok(())
   }
   ```

2. **`crates/plissken/src/detect.rs`** (new) - Project detection logic:
   ```rust
   pub struct DetectedProject {
       pub name: String,
       pub has_rust: bool,
       pub has_python: bool,
       pub rust_crates: Vec<PathBuf>,
       pub python_source: Option<PathBuf>,
   }
   
   pub fn detect_project_type() -> Result<DetectedProject>
   ```

### Detection Logic

1. Check for `Cargo.toml` in current dir or parent
2. Check for `pyproject.toml` or `setup.py`
3. If Cargo.toml has `[lib] crate-type = ["cdylib"]` + pyo3 dep → hybrid
4. Look for common Python source layouts: `src/`, `python/`, package dir
5. Extract names from manifest files

### CLI Flags

```
plissken init [OPTIONS]

Options:
  --force       Overwrite existing plissken.toml
  --rust-only   Only configure Rust (skip Python detection)
  --python-only Only configure Python (skip Rust detection)
```

## Status Updates

### 2026-01-16: Implementation Complete

**Implemented in `crates/plissken-cli/src/main.rs`:**

1. **Init command** with `--force` flag
2. **DetectedProject struct** tracking:
   - Project name
   - has_rust / has_python flags
   - rust_crates, rust_entry_point
   - python_package, python_source
   - is_hybrid (PyO3/maturin detection)

3. **Detection logic:**
   - Parses Cargo.toml for name and pyo3/maturin dependencies
   - Parses pyproject.toml for name and maturin build system
   - Extracts Python source from pyproject.toml:
     - `[tool.maturin] python-source` for PyO3 projects
     - `[tool.setuptools.packages.find] where` for src layout
   - Falls back to setup.py for Python detection

4. **Config generation:**
   - Generates valid TOML with all necessary sections
   - Respects project type (Rust-only, Python-only, hybrid)
   - Sets sensible defaults (mkdocs-material, docs/api)

**Example output:**
```
$ plissken -v init
Detecting project type...
Detected: Rust project 'pure_rust'
Created plissken.toml
```

**All 129 tests pass.**