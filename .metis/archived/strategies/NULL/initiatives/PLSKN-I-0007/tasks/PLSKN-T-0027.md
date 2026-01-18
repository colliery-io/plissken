---
id: add-python-module-auto-discovery
level: task
title: "Add Python module auto-discovery"
short_code: "PLSKN-T-0027"
created_at: 2026-01-16T17:58:34.968295+00:00
updated_at: 2026-01-17T01:20:20.926608+00:00
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

# Add Python module auto-discovery

## Parent Initiative

[[PLSKN-I-0007]] Configuration Simplification

## Objective

Eliminate the need to manually enumerate every Python module in `[python.modules]`. Add an `auto_discover = true` option that walks the filesystem to find Python files, dramatically reducing config verbosity for large packages.

## Current Problem

A 50-module Python package requires 50 lines of config:
```toml
[python.modules]
"mypackage" = "python"
"mypackage.core" = "python"
"mypackage.core.engine" = "python"
"mypackage.utils" = "python"
# ... 46 more lines
```

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [x] Add `auto_discover` boolean option to `[python]` config section
- [x] When `true`, walk filesystem from `source` path to find all `.py` files
- [x] Convert file paths to module names (e.g., `foo/bar/baz.py` → `foo.bar.baz`)
- [x] Respect `__init__.py` for package detection
- [x] Skip common non-module directories (`__pycache__`, `.venv`, `tests`, `test`, etc.)
- [x] Allow explicit `[python.modules]` to override/extend auto-discovered modules
- [x] Default to `auto_discover = false` for backwards compatibility

## Config Examples

### Auto-discovery enabled
```toml
[python]
package = "mypackage"
source = "src"
auto_discover = true  # Finds all modules automatically
```

### Hybrid (auto + explicit)
```toml
[python]
package = "mypackage"
source = "src"
auto_discover = true

[python.modules]
# Override specific modules (e.g., mark as pyo3 bindings)
"mypackage._native" = "pyo3"
```

### Explicit exclude patterns (future enhancement)
```toml
[python]
auto_discover = true
exclude = ["tests", "examples", "_internal"]
```

## Implementation Notes

### Files to Modify

1. **`crates/plissken-core/src/config.rs`** - Add field:
   ```rust
   #[derive(Deserialize)]
   pub struct PythonConfig {
       pub package: String,
       pub source: PathBuf,
       #[serde(default)]
       pub auto_discover: bool,
       pub modules: Option<HashMap<String, ModuleType>>,
   }
   ```

2. **`crates/plissken-core/src/discover.rs`** (new) - Discovery logic:
   ```rust
   pub fn discover_python_modules(
       source: &Path,
       package: &str,
   ) -> Result<Vec<DiscoveredModule>> {
       let mut modules = Vec::new();
       for entry in WalkDir::new(source) {
           let entry = entry?;
           if is_python_module(&entry) {
               let module_name = path_to_module_name(&entry.path(), source, package);
               modules.push(DiscoveredModule {
                   name: module_name,
                   path: entry.path().to_owned(),
                   module_type: detect_module_type(&entry.path())?,
               });
           }
       }
       Ok(modules)
   }
   
   fn is_python_module(entry: &DirEntry) -> bool {
       let path = entry.path();
       if path.extension() != Some("py".as_ref()) {
           return false;
       }
       // Skip __pycache__, .venv, tests, etc.
       !path.components().any(|c| {
           matches!(c.as_os_str().to_str(), 
               Some("__pycache__" | ".venv" | "venv" | "tests" | "test"))
       })
   }
   ```

3. **`crates/plissken-core/src/python_parser.rs`** - Use discovered modules

### Module Type Detection

When auto-discovering, detect if a module is PyO3 bindings by scanning for markers:
- `from <package> import` where package matches Rust crate name
- `# pyo3` comment marker
- Imports from known PyO3 stub pattern

Default to `"python"` type if no markers found.

### Dependencies

- Add `walkdir` crate dependency for filesystem traversal

## Status Updates

### 2026-01-17: Implementation Complete

Added Python module auto-discovery feature:

**New files:**
- `crates/plissken-core/src/discover.rs` - Discovery module with:
  - `discover_python_modules()` - Walks filesystem to find Python files
  - `path_to_module_name()` - Converts file paths to dotted module names
  - `detect_module_type()` - Detects PyO3 stubs via markers
  - `merge_modules()` - Merges discovered with explicit modules
  - `DiscoveredModule` struct

**Modified files:**
- `crates/plissken-core/src/config.rs` - Added `auto_discover: bool` to PythonConfig
- `crates/plissken-core/src/lib.rs` - Exported discover module
- `crates/plissken-core/Cargo.toml` - Added walkdir dependency
- `Cargo.toml` - Added walkdir to workspace
- `crates/plissken-cli/src/main.rs` - Uses discovery when auto_discover=true

**Skip directories:**
- `__pycache__`, `.venv`, `venv`, `.env`, `env`
- `.tox`, `.nox`, `.pytest_cache`, `.mypy_cache`, `.ruff_cache`
- `node_modules`, `.git`, `build`, `dist`, `*.egg-info`

**PyO3 detection markers:**
- `# pyo3` comment
- `from ._*` imports (underscore prefix convention)
- `# type: ignore[import]` (common in stubs)

**Testing:**
- Added 10 unit tests for discovery
- Verified with pure_python fixture (found 9 modules)
- All 148 tests pass