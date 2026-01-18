---
id: extract-duplicated-python
level: task
title: "Extract duplicated Python synthesis logic to helper function"
short_code: "PLSKN-T-0043"
created_at: 2026-01-17T19:48:44.330131+00:00
updated_at: 2026-01-17T20:05:30.665208+00:00
parent: PLSKN-I-0011
blocked_by: []
archived: false

tags:
  - "#task"
  - "#phase/active"


exit_criteria_met: false
strategy_id: NULL
initiative_id: PLSKN-I-0011
---

# Extract duplicated Python synthesis logic to helper function

## Parent Initiative

[[PLSKN-I-0011]]

## Objective

Extract the duplicated Python module synthesis logic that appears in two places in `main.rs` into a single helper function.

## Problem

Identical code appears at two locations:

**Location 1: `main.rs:171-182`** (in `generate()` command)
```rust
let (python_modules, cross_refs) = if python_modules.is_empty() && !rust_modules.is_empty() && config.python.is_some() {
    let module_name = config.python.as_ref().map(|p| p.package.clone()).unwrap_or_else(|| config.project.name.clone());
    let (synth_module, synth_refs) = synthesize_python_from_rust(&rust_modules, &module_name);
    (vec![synth_module], synth_refs)
} else {
    build_cross_refs(&config, &rust_modules, python_modules)
};
```

**Location 2: `main.rs:324-335`** (in `render()` command)
```rust
// IDENTICAL LOGIC
```

This violates DRY and makes it easy for the two paths to diverge.

## Acceptance Criteria

## Acceptance Criteria

- [x] Create `synthesize_python_if_needed()` helper function
- [x] Replace both duplicate blocks with calls to the helper
- [x] Helper returns `(Vec<PythonModule>, Vec<CrossRef>)`
- [x] All existing tests pass (223 unit, 4 integration, 16 doc tests)
- [x] No change in behavior (verified with both `render` and `generate` commands)

## Implementation Notes

### Technical Approach

```rust
fn synthesize_python_if_needed(
    config: &Config,
    python_modules: Vec<PythonModule>,
    rust_modules: &[RustModule],
) -> (Vec<PythonModule>, Vec<CrossRef>) {
    if python_modules.is_empty() && !rust_modules.is_empty() && config.python.is_some() {
        let module_name = config
            .python
            .as_ref()
            .map(|p| p.package.clone())
            .unwrap_or_else(|| config.project.name.clone());
        let (synth_module, synth_refs) = synthesize_python_from_rust(rust_modules, &module_name);
        (vec![synth_module], synth_refs)
    } else {
        build_cross_refs(config, rust_modules, python_modules)
    }
}
```

### Dependencies
- Should be done after or with PLSKN-T-0042 (split render function)

## Status Updates

### 2026-01-17 Session

**Completed:**

1. Created `synthesize_python_if_needed()` helper function that encapsulates the core logic:
   - Checks if Python modules are empty, Rust modules exist, and config has Python section
   - Synthesizes Python bindings from Rust if conditions met
   - Otherwise calls `build_cross_refs()` normally

2. Updated `generate()` command (line 169-170) to use the helper:
   ```rust
   // Before: 12 lines of inline logic
   // After: 1 line
   let (python_modules, cross_refs) = synthesize_python_if_needed(&config, python_modules, &rust_modules);
   ```

3. Updated `build_cross_references()` helper to use `synthesize_python_if_needed()`:
   ```rust
   let (python_modules, mut cross_refs) = synthesize_python_if_needed(config, python_modules, rust_modules);
   cross_refs.extend(initial_cross_refs);
   ```

**Test Results:**
- All 223 unit tests pass
- All 4 integration tests pass
- All 16 doc tests pass
- Both `render` and `generate` commands work correctly