---
id: improve-error-messages-with
level: task
title: "Improve error messages with recovery hints"
short_code: "PLSKN-T-0023"
created_at: 2026-01-16T17:51:44.522403+00:00
updated_at: 2026-01-16T22:14:32.332328+00:00
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

# Improve error messages with recovery hints

## Parent Initiative

[[PLSKN-I-0006]] CLI UX: Invisible by Default

## Objective

Transform cryptic error messages into helpful guidance. Every error should tell users what went wrong AND how to fix it. Follow the Rust/Cargo pattern of actionable error messages.

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [x] All user-facing errors include a "hint" with recovery action
- [x] Missing config file error suggests `plissken init`
- [x] Missing source paths error shows which paths don't exist
- [x] Parse errors include file path and line number when available
- [x] Invalid config errors show the problematic field and valid options
- [x] Error output follows consistent format across all commands

## Error Message Catalog

### Current → Improved

| Current | Improved |
|---------|----------|
| `plissken.toml not found` | `error: plissken.toml not found in current directory`<br>`hint: run 'plissken init' to create a configuration file` |
| `No modules found` | `error: no Python modules found in 'src/'`<br>`hint: check that [python.package] points to a valid Python package` |
| `Invalid config` | `error: invalid value for 'version_from': "files"`<br>`hint: valid options are: "cargo", "pyproject", "inline"` |
| `Parse error` | `error: failed to parse src/lib.rs:42`<br>`  --> unexpected token 'async'`<br>`hint: ensure the file is valid Rust syntax` |

## Implementation Notes

### Files to Modify

1. **`crates/plissken-core/src/error.rs`** (new file) - Create structured error type:
   ```rust
   pub struct PlisskenError {
       pub message: String,
       pub hint: Option<String>,
       pub context: Option<ErrorContext>,
   }
   
   pub struct ErrorContext {
       pub file: Option<PathBuf>,
       pub line: Option<usize>,
       pub field: Option<String>,
   }
   ```

2. **`crates/plissken/src/main.rs`** - Format errors consistently:
   ```rust
   fn format_error(e: &PlisskenError) -> String {
       let mut out = format!("error: {}", e.message);
       if let Some(ctx) = &e.context {
           if let Some(file) = &ctx.file {
               out.push_str(&format!("\n  --> {}", file.display()));
           }
       }
       if let Some(hint) = &e.hint {
           out.push_str(&format!("\nhint: {}", hint));
       }
       out
   }
   ```

3. **`crates/plissken-core/src/config.rs`** - Add context to config errors

### Technical Approach

1. Create `PlisskenError` type with message + hint + context
2. Audit all `anyhow!` and `bail!` calls for context
3. Add hint generation logic based on error type
4. Implement `Display` for consistent formatting
5. Update CLI to use new error formatting

### Error Categories

- **Config errors**: Missing file, invalid field, missing section
- **Parse errors**: Syntax errors, unsupported constructs
- **IO errors**: File not found, permission denied
- **Validation errors**: Empty output, no modules configured

## Status Updates

### 2026-01-16: Implementation Complete

**Created `CliError` type** in `main.rs` with:
- `message`: Main error description
- `hint`: Optional recovery action
- `context`: Optional file/location context
- Implements `Display` for consistent formatting

**Updated error sites:**

1. **Config not found**: Now shows path and suggests `plissken init`
2. **Config parsing**: Extracts field names, line numbers, suggests fixes
3. **Renderer creation**: Suggests valid template names
4. **Output directory**: Shows permission context
5. **Module rendering**: Indicates potential bug, asks for report
6. **Parse warnings**: Consistent multi-line format with file path

**Example outputs:**
```
Error: plissken.toml not found at ./plissken.toml
hint: run 'plissken init' to create a configuration file

Error: invalid configuration: missing required field 'project'
  --> plissken.toml:at line 1
hint: ensure [project] section has a 'name' field
```

**All 129 tests pass.**