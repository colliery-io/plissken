---
id: add-plissken-check-command-for
level: task
title: "Add plissken check command for config validation"
short_code: "PLSKN-T-0025"
created_at: 2026-01-16T17:51:44.621218+00:00
updated_at: 2026-01-17T00:59:54.514845+00:00
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

# Add plissken check command for config validation

## Parent Initiative

[[PLSKN-I-0006]] CLI UX: Invisible by Default

## Objective

Add a `plissken check` command that validates configuration without generating documentation. This enables CI integration and quick feedback during setup - users can verify their config is correct before running full generation.

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [x] `plissken check` validates plissken.toml without generating output
- [x] Validates all configured paths exist (crates, Python source, output dir parent)
- [x] Validates version_from source exists (Cargo.toml for "cargo", pyproject.toml for "pyproject")
- [x] Reports warnings for likely misconfigurations (empty sections, unused fields)
- [x] Exit code 0 on success, non-zero on validation failure
- [x] With `--verbose`, shows what was validated
- [x] Machine-readable output with `--format json` for CI integration

## Example Output

### Success (default - silent)
```
$ plissken check
$ echo $?
0
```

### Success (verbose)
```
$ plissken check --verbose
Checking plissken.toml...
  [rust] 2 crates configured
    ✓ ./Cargo.toml exists
    ✓ ./crates/core/Cargo.toml exists
  [python] package "mylib" configured
    ✓ python/mylib/__init__.py exists
  [output] docs/api (mkdocs-material)
    ✓ docs/ exists
Configuration valid.
```

### Failure
```
$ plissken check
error: validation failed

  [rust] crate path does not exist
    --> crates = [".", "./crates/missing"]
    error: ./crates/missing/Cargo.toml not found
    hint: check that the path exists or remove it from crates list

  [python] source directory does not exist  
    --> source = "src"
    error: src/ directory not found
    hint: common locations are "python/", "src/", or the package name
```

### JSON Output (for CI)
```json
{
  "valid": false,
  "errors": [
    {
      "section": "rust",
      "field": "crates",
      "message": "./crates/missing/Cargo.toml not found",
      "hint": "check that the path exists or remove it from crates list"
    }
  ],
  "warnings": []
}
```

## Implementation Notes

### Files to Create/Modify

1. **`crates/plissken/src/commands/check.rs`** (new):
   ```rust
   pub fn run_check(config_path: &Path, format: OutputFormat, verbose: u8) -> Result<()> {
       let config = Config::from_file(config_path)?;
       let results = validate_config(&config)?;
       
       match format {
           OutputFormat::Text => print_text_results(&results, verbose),
           OutputFormat::Json => print_json_results(&results),
       }
       
       if results.has_errors() {
           std::process::exit(1);
       }
       Ok(())
   }
   ```

2. **`crates/plissken-core/src/config/validate.rs`** (new):
   ```rust
   pub struct ValidationResults {
       pub errors: Vec<ValidationError>,
       pub warnings: Vec<ValidationWarning>,
   }
   
   pub fn validate_config(config: &Config) -> ValidationResults
   ```

### Validation Checks

**Errors (fail check):**
- Config file doesn't exist
- Config file has syntax errors
- Referenced crate paths don't have Cargo.toml
- Referenced Python source doesn't exist
- version_from source file doesn't exist
- No languages configured (empty rust + empty python)

**Warnings (pass but report):**
- Output directory doesn't exist (will be created)
- Python modules list is empty with auto_discover=false
- Template name not recognized (custom templates are OK)

### CLI Definition

```
plissken check [OPTIONS]

Options:
  -c, --config <PATH>    Config file path [default: plissken.toml]
  --format <FORMAT>      Output format: text, json [default: text]
```

## Dependencies

- Depends on PLSKN-T-0023 (error messages) for consistent error formatting

## Status Updates

### 2026-01-17: Implementation Complete

Implemented `plissken check` command with full validation:

**Implementation approach:**
- Added Check command to CLI with `path` and `--format` args
- Created `ValidationResult` and `ValidationIssue` structs for structured results
- Implemented validation functions:
  - `validate_version_source()` - checks Cargo.toml, pyproject.toml, or git repo exists
  - `validate_rust_config()` - validates crate paths, checks for Cargo.toml and src/
  - `validate_python_config()` - validates source directory and __init__.py presence
- Added JSON output format using serde serialization
- Returns proper exit codes (0 for valid, 1 for errors)

**Files modified:**
- `crates/plissken-cli/src/main.rs` - Added check command and validation logic
- `crates/plissken-cli/Cargo.toml` - Added serde dependency

**Testing:**
- Tested with valid configs (pure_python, pure_rust fixtures)
- Tested with invalid configs (missing paths, missing version sources)
- Verified JSON output format works correctly
- All 129 tests pass