---
id: extract-cross-reference-path
level: task
title: "Extract cross-reference path calculation helpers"
short_code: "PLSKN-T-0046"
created_at: 2026-01-17T19:48:45.176889+00:00
updated_at: 2026-01-17T20:16:08.520961+00:00
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

# Extract cross-reference path calculation helpers

## Parent Initiative

[[PLSKN-I-0011]]

## Objective

Extract the duplicated cross-reference path calculation logic in `crossref.rs` into reusable helper functions.

## Problem

The same path calculation logic is duplicated 6 times in `crossref.rs`:

| Method | Lines | Purpose |
|--------|-------|---------|
| `python_link_for_rust_method()` | 86-98 | Rust → Python method links |
| `python_link_for_rust_function()` | 141-150 | Rust → Python function links |
| `python_link_for_rust_struct()` | 193-201 | Rust → Python struct links |
| `rust_link_for_python_function()` | 242-250 | Python → Rust function links |
| `rust_link_for_python_class()` | 288-301 | Python → Rust class links |
| `rust_link_for_python_method()` | 335-350 | Python → Rust method links |

All duplicate this pattern:
```rust
let rust_depth = 1 + rust_path.matches("::").count();
let prefix = "../".repeat(rust_depth);

let module_parts: Vec<&str> = python_module.split('.').collect();
let python_page = if module_parts.len() == 1 {
    format!("{}.md", module_parts[0])
} else {
    let last = module_parts.last().unwrap();
    let parent = module_parts[..module_parts.len()-1].join("/");
    format!("{}/{}.md", parent, last)
};
```

## Acceptance Criteria

## Acceptance Criteria

- [x] Create `compute_python_page_path(module_path: &str) -> String` helper
- [x] Create `compute_rust_page_path(module_path: &str) -> String` helper
- [x] Create `compute_relative_prefix(from_path: &str, separator: &str) -> String` helper (split into `compute_rust_relative_prefix` and `compute_python_relative_prefix`)
- [x] Replace all 6 duplicate blocks with helper calls
- [x] Add unit tests for edge cases (single segment, deeply nested)
- [x] All existing tests pass

## Implementation Notes

### Technical Approach

```rust
// Add to crossref.rs or new path_utils.rs

fn compute_python_page_path(module_path: &str) -> String {
    let parts: Vec<&str> = module_path.split('.').collect();
    if parts.len() == 1 {
        format!("{}.md", parts[0])
    } else {
        let last = parts.last().unwrap();
        let parent = parts[..parts.len()-1].join("/");
        format!("{}/{}.md", parent, last)
    }
}

fn compute_rust_page_path(module_path: &str) -> String {
    let parts: Vec<&str> = module_path.split("::").collect();
    if parts.len() == 1 {
        format!("rust/{}.md", parts[0])
    } else {
        let crate_name = parts[0];
        let subpath = parts[1..].join("/");
        format!("rust/{}/{}.md", crate_name, subpath)
    }
}

fn compute_relative_prefix(from_path: &str, separator: &str) -> String {
    let depth = 1 + from_path.matches(separator).count();
    "../".repeat(depth)
}
```

## Status Updates

### Session 1 - Completed

**Created 4 helper functions in `crossref.rs`:**
1. `compute_python_page_path(module_path)` - converts Python module path to file path
   - `mypackage` → `mypackage.md`
   - `mypackage.sub.deep` → `mypackage/sub/deep.md`

2. `compute_rust_page_path(module_path)` - converts Rust module path to file path  
   - `mycrate` → `rust/mycrate.md`
   - `mycrate::sub::deep` → `rust/mycrate/sub/deep.md`

3. `compute_rust_relative_prefix(rust_path)` - computes `../` prefix from Rust module
   - `mycrate` → `../` (depth 1 for rust/ dir)
   - `mycrate::sub` → `../../` (depth 2)

4. `compute_python_relative_prefix(python_path)` - computes `../` prefix from Python module
   - `mypackage` → `` (at root)
   - `mypackage.sub` → `../` (depth 1)

**Refactored 6 methods to use helpers:**
- `python_link_for_rust_method()` - lines 86-98 → 3 lines
- `python_link_for_rust_function()` - lines 141-150 → 3 lines  
- `python_link_for_rust_struct()` - lines 193-201 → 3 lines
- `rust_link_for_python_function()` - lines 242-250 → 3 lines
- `rust_link_for_python_class()` - lines 288-301 → 3 lines
- `rust_link_for_python_method()` - lines 335-350 → 3 lines

**Added 6 unit tests:**
- `test_compute_python_page_path_single_segment`
- `test_compute_python_page_path_nested`
- `test_compute_rust_page_path_single_segment`
- `test_compute_rust_page_path_nested`
- `test_compute_rust_relative_prefix`
- `test_compute_python_relative_prefix`

**Test Results:** 229 unit tests, 4 integration tests, 16 doc tests - all pass

**Files Modified:**
- `crates/plissken-core/src/render/module/crossref.rs`