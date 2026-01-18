---
id: consolidate-navigation-generation
level: task
title: "Consolidate navigation generation into NavGenerator trait"
short_code: "PLSKN-T-0045"
created_at: 2026-01-17T19:48:44.835937+00:00
updated_at: 2026-01-17T20:15:24.562446+00:00
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

# Consolidate navigation generation into NavGenerator trait

## Parent Initiative

[[PLSKN-I-0011]]

## Objective

Consolidate the navigation generation logic that is duplicated across three files into a single `NavGenerator` trait with shared implementation.

## Problem

The same module sorting and iteration pattern is duplicated in 3 places:

1. **`mkdocs.rs:56-72`** - MkDocs navigation YAML
2. **`mdbook.rs:52-70`** - mdBook SUMMARY.md
3. **`module_renderer.rs:1446-1500`** - Legacy `generate_nav_yaml()`
4. **`module_renderer.rs:1522-1574`** - Legacy `generate_mdbook_summary()`

All four implement the same pattern:
```rust
if !python_modules.is_empty() {
    // header
    let mut sorted_modules: Vec<&PythonModule> = python_modules.iter().collect();
    sorted_modules.sort_by(|a, b| a.path.cmp(&b.path));
    for module in sorted_modules {
        let file_path = layout.python_module_page(&module.path);
        // format output
    }
}
// Identical pattern for Rust modules
```

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [x] Create shared module sorting/iteration helper function
- [x] Remove duplicate code from mkdocs.rs and mdbook.rs
- [x] Update legacy methods in module_renderer.rs to use helpers
- [x] SSG adapters use shared helper
- [x] Output remains identical for both MkDocs and mdBook
- [x] All existing tests pass

## Implementation Notes

### Technical Approach

```rust
// In a new nav_utils.rs or within ssg/traits.rs
pub fn generate_module_entries<T, F>(
    modules: &[T],
    path_fn: impl Fn(&T) -> &str,
    format_fn: F,
) -> Vec<String>
where
    F: Fn(&T, &Path) -> String,
{
    let mut sorted: Vec<&T> = modules.iter().collect();
    sorted.sort_by(|a, b| path_fn(a).cmp(path_fn(b)));
    
    sorted.iter()
        .map(|m| {
            let page = layout.module_page(path_fn(m));
            format_fn(m, &page)
        })
        .collect()
}
```

### Dependencies
- Consider doing after PLSKN-T-0049 (decouple SSG from PageLayout)

## Status Updates

### Session 1 - Completed
- Created `NavEntry` struct in `traits.rs` with `path`, `file_path`, and `depth` fields
- Created `python_nav_entries()` helper function that sorts modules and computes file paths
- Created `rust_nav_entries()` helper function with same pattern
- Updated `mkdocs.rs` to use the shared helpers (reduced ~55 lines to ~40 lines)
- Updated `mdbook.rs` to use the shared helpers
- Updated `module_renderer.rs` `generate_nav_yaml()` to use helpers
- Updated `module_renderer.rs` `generate_mdbook_summary()` to use helpers
- Fixed module visibility error by exporting helpers from `ssg/mod.rs`
- Removed unused variable warning in `render_python_module()`
- All 223 unit tests, 4 integration tests, and 16 doc tests pass

### Files Modified
- `crates/plissken-core/src/render/ssg/traits.rs` - Added NavEntry struct and helper functions
- `crates/plissken-core/src/render/ssg/mod.rs` - Exported new helpers publicly
- `crates/plissken-core/src/render/ssg/mkdocs.rs` - Updated to use helpers
- `crates/plissken-core/src/render/ssg/mdbook.rs` - Updated to use helpers
- `crates/plissken-core/src/render/module_renderer.rs` - Updated to use helpers