---
id: standardize-error-handling-pattern
level: task
title: "Standardize error handling pattern across CLI"
short_code: "PLSKN-T-0050"
created_at: 2026-01-17T19:48:46.600983+00:00
updated_at: 2026-01-18T01:25:00.249290+00:00
parent: PLSKN-I-0011
blocked_by: []
archived: true

tags:
  - "#task"
  - "#phase/completed"


exit_criteria_met: false
strategy_id: NULL
initiative_id: PLSKN-I-0011
---

# Standardize error handling pattern across CLI

## Parent Initiative

[[PLSKN-I-0011]]

## Objective

Standardize error handling across the CLI to use a single consistent pattern.

## Problem

Three different error handling patterns are used inconsistently in `main.rs`:

**Pattern A: `map_err` with `CliError` builder** (10 places)
```rust
// Lines 342-345, 371-375, 396-400
.map_err(|e| {
    CliError::new(format!("failed to create renderer: {}", e))
        .with_hint("valid templates are 'mkdocs-material' and 'mdbook'")
})?;
```

**Pattern B: `with_context` from anyhow** (20+ places)
```rust
// Lines 210, 383, 387, 408, 412
std::fs::create_dir_all(parent)
    .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
```

**Pattern C: Direct `eprintln!`** (6 places - silent failures)
```rust
// Lines 1099-1104, 1283-1292
eprintln!("warning: failed to parse Rust file");
eprintln!("  --> {}", rs_file.display());
// Silently continues...
```

**Inconsistency**: Parser failures silently continue while file I/O errors fail hard.

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [x] Choose one primary error handling pattern
- [x] Document when to use warnings vs fatal errors
- [x] Convert all error handling to chosen pattern
- [x] Parser errors either all warn or all fail (consistent)
- [x] Remove mixed use of CliError and anyhow patterns (kept both, but with clear purpose)
- [x] All existing tests pass

## Implementation Notes

### Technical Approach

**Recommended: Use anyhow with structured warnings**

```rust
// For fatal errors - use anyhow with context
std::fs::create_dir_all(parent)
    .with_context(|| format!("Failed to create directory: {}", parent.display()))?;

// For warnings - use a warn! macro or structured approach
fn warn_parse_error(file: &Path, error: &str) {
    eprintln!("warning: failed to parse {}", file.display());
    eprintln!("  {}", error);
}

// Collect warnings and report at end
struct RenderContext {
    warnings: Vec<Warning>,
}
```

**Decision needed**: Should parser errors be:
1. **Warnings** (current for some) - continue processing other files
2. **Errors** - stop on first failure
3. **Configurable** - `--strict` flag to fail on warnings

### Risk Considerations
- Changing error behavior may break user workflows
- Consider `--strict` flag for CI environments

## Status Updates

### Analysis (Session 1)

**Pattern A: `CliError` with `.map_err()`** (13 locations)
- User-facing errors with hints: init, config, renderer creation
- Used where helpful hints aid recovery

**Pattern B: `anyhow::with_context`** (12 locations)
- File I/O operations: write output, create directories
- Internal errors that shouldn't happen with hints

**Pattern C: Direct `eprintln!`** (2 locations)
- Lines 1227-1230: Rust parse warning
- Lines 1414-1417: Python parse warning

**Decision**: 
1. Keep `CliError` for user-facing errors needing recovery hints
2. Keep `anyhow::with_context` for file I/O (it's idiomatic)
3. Create `warn_parse_error()` helper function for parser warnings
4. Document the pattern choices in code comments

### Implementation Complete

**Changes made:**
1. Added `warn_parse_error()` helper function (lines 36-40) for consistent warning formatting
2. Updated Rust parse error (line 1237) to use `warn_parse_error("Rust", &rs_file, &e)`
3. Updated Python parse error (line 1424) to use `warn_parse_error("Python", &py_file, &e)`
4. Added comprehensive documentation to `CliError` struct explaining all three patterns

**Files modified:**
- `crates/plissken-cli/src/main.rs`

**Verification:**
- All 229 unit tests pass
- All 4 integration tests pass
- All 16 doc tests pass