---
id: add-config-validate-method-with
level: task
title: "Add Config::validate() method with semantic validation"
short_code: "PLSKN-T-0026"
created_at: 2026-01-16T17:58:34.905404+00:00
updated_at: 2026-01-17T01:09:15.567531+00:00
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

# Add Config::validate() method with semantic validation

## Parent Initiative

[[PLSKN-I-0007]] Configuration Simplification

## Objective

Add a `Config::validate()` method that performs semantic validation beyond TOML parsing. Currently, configs that parse successfully may still be invalid (empty sections, missing paths, contradictory settings). This catches configuration errors early with helpful messages.

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [x] `Config::validate()` method returns `Result<ValidationResult, ConfigError>`
- [x] Validates at least one language section is non-empty (`[rust]` or `[python]`)
- [x] Validates `version_from` source file exists
- [x] Validates configured paths exist (crate dirs, Python source)
- [x] Returns warnings for likely misconfigurations (not errors)
- [x] CLI commands call `validate()` early and report issues
- [x] Unit tests cover all validation cases (9 tests added)

## Validation Rules

### Errors (fail validation)
| Rule | Message |
|------|---------|
| No language configured | "At least one of [rust] or [python] must be configured" |
| version_from="cargo" but no Cargo.toml | "version_from is 'cargo' but Cargo.toml not found" |
| version_from="pyproject" but no pyproject.toml | "version_from is 'pyproject' but pyproject.toml not found" |
| Rust crate path missing | "Crate path '{path}' does not exist" |
| Python source path missing | "Python source '{path}' does not exist" |

### Warnings (pass but report)
| Rule | Message |
|------|---------|
| Empty `[python.modules]` | "No Python modules listed; nothing will be documented" |
| Empty `[rust] crates` | "No Rust crates listed; nothing will be documented" |
| Unknown template name | "Template '{name}' not recognized; using as custom" |

## Implementation Notes

### Files to Modify

1. **`crates/plissken-core/src/config.rs`** - Add validate method:
   ```rust
   impl Config {
       pub fn validate(&self) -> Result<Vec<ConfigWarning>, ConfigError> {
           let mut warnings = Vec::new();
           
           // Must have at least one language
           if self.rust.is_none() && self.python.is_none() {
               return Err(ConfigError::NoLanguageConfigured);
           }
           
           // Validate paths exist
           if let Some(rust) = &self.rust {
               for crate_path in &rust.crates {
                   let cargo = crate_path.join("Cargo.toml");
                   if !cargo.exists() {
                       return Err(ConfigError::PathNotFound(cargo));
                   }
               }
           }
           
           // ... more validation
           Ok(warnings)
       }
   }
   ```

2. **`crates/plissken-core/src/config/error.rs`** (new) - Error types:
   ```rust
   #[derive(Debug, thiserror::Error)]
   pub enum ConfigError {
       #[error("no language configured: add [rust] or [python] section")]
       NoLanguageConfigured,
       #[error("path not found: {0}")]
       PathNotFound(PathBuf),
       // ...
   }
   
   pub struct ConfigWarning {
       pub message: String,
       pub field: String,
   }
   ```

3. **`crates/plissken/src/commands/generate.rs`** - Call validation early

### Integration with PLSKN-T-0025

This task provides the core validation logic that `plissken check` (T-0025) will use. The check command wraps this with CLI output formatting.

## Status Updates

### 2026-01-17: Implementation Complete

Added `Config::validate()` method to plissken-core with comprehensive validation:

**New types in `config.rs`:**
- `ConfigError` enum with variants: `NoLanguageConfigured`, `VersionSourceNotFound`, `RustCrateNotFound`, `PythonSourceNotFound`, `GitRepoNotFound`
- `ConfigWarning` struct with field, message, and optional hint
- `ValidationResult` struct containing valid flag and warnings vector

**Validation implemented:**
- At least one language section required ([rust] or [python])
- version_from source must exist (Cargo.toml, pyproject.toml, or git repo)
- Rust crate paths must exist
- Python source directory must exist
- Warnings for: missing Cargo.toml in crates, missing src/, missing __init__.py, empty modules list

**CLI integration:**
- Refactored `plissken check` to use `Config::validate()`
- Removed duplicate validation functions from CLI
- Core validation now reusable by library consumers

**Files modified:**
- `crates/plissken-core/src/config.rs` - Added types and validate() method
- `crates/plissken-core/src/lib.rs` - Exported new types
- `crates/plissken-core/Cargo.toml` - Added tempfile dev-dependency
- `crates/plissken-cli/src/main.rs` - Refactored check command

**Testing:**
- Added 9 unit tests for validation
- All 138 tests pass