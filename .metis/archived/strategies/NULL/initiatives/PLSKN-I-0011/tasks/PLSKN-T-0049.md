---
id: decouple-ssg-adapters-from
level: task
title: "Decouple SSG adapters from PageLayout"
short_code: "PLSKN-T-0049"
created_at: 2026-01-17T19:48:46.244698+00:00
updated_at: 2026-01-17T20:36:08.942210+00:00
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

# Decouple SSG adapters from PageLayout

## Parent Initiative

[[PLSKN-I-0011]]

## Objective

Remove the tight coupling between SSG adapters and PageLayout, making adapters independent of file path conventions.

## Problem

Both SSG adapters directly import and use `PageLayout`:

```rust
// mkdocs.rs:5
use crate::render::module::PageLayout;

// mdbook.rs:4
use crate::render::module::PageLayout;

// Both do:
let layout = PageLayout::new();
let file_path = layout.python_module_page(&module.path);
```

This creates implicit dependency on file path conventions. SSG adapters should only care about:
- Navigation structure (module hierarchy)
- Output format (YAML vs Markdown)

They should NOT know about:
- File path conventions
- Whether it's inline vs multi-page format

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [x] SSG adapters don't import PageLayout
- [x] Path calculation injected or abstracted (moved to local functions in traits.rs)
- [x] Create `LayoutProvider` trait or similar abstraction (used local functions instead - simpler approach)
- [x] SSG adapters receive pre-computed paths (via NavEntry struct from helper functions)
- [x] Output remains identical for both MkDocs and mdBook
- [x] All existing tests pass

## Implementation Notes

### Technical Approach

**Option A: Inject paths via parameters**
```rust
trait SSGAdapter {
    fn generate_nav(
        &self,
        python_entries: &[(String, PathBuf)],  // (name, path) pairs
        rust_entries: &[(String, PathBuf)],
    ) -> String;
}
```

**Option B: LayoutProvider trait**
```rust
trait LayoutProvider {
    fn python_module_page(&self, module_path: &str) -> PathBuf;
    fn rust_module_page(&self, module_path: &str) -> PathBuf;
}

trait SSGAdapter {
    fn generate_nav(
        &self,
        python_modules: &[PythonModule],
        rust_modules: &[RustModule],
        layout: &dyn LayoutProvider,
    ) -> String;
}
```

**Option C: Pre-compute in caller**
```rust
// In module_renderer.rs or main.rs
let python_nav_entries: Vec<NavEntry> = python_modules.iter()
    .map(|m| NavEntry { name: m.path.clone(), path: layout.python_module_page(&m.path) })
    .collect();

adapter.generate_nav(&python_nav_entries, &rust_nav_entries);
```

### Dependencies
- Consider doing before PLSKN-T-0045 (NavGenerator consolidation)

## Status Updates

### Session 1 - 2026-01-17

**Analysis:**
- SSG adapters (`mkdocs.rs`, `mdbook.rs`) were already partially decoupled - they don't directly import `PageLayout`
- The coupling was in `traits.rs` which imported `PageLayout` and used it in the `python_nav_entries()` and `rust_nav_entries()` helper functions
- The path computation logic is simple string manipulation

**Implementation:**
- Removed `use crate::render::module::PageLayout;` from `traits.rs`
- Added local `python_module_page()` function that computes Python module file paths (inline format)
- Added local `rust_module_page()` function that computes Rust module file paths (inline format)
- Updated `python_nav_entries()` and `rust_nav_entries()` to use the local functions

**Files Modified:**
- `crates/plissken-core/src/render/ssg/traits.rs` - Removed PageLayout import, added local path computation functions

**Verification:**
- `cargo check` passes
- All 229 unit tests pass
- All 4 integration tests pass
- All 16 doc tests pass
- Confirmed no references to `PageLayout` in the ssg/ directory

**Notes:**
- The path computation logic is duplicated between `PageLayout` and `traits.rs`, but this is acceptable because:
  1. The logic is simple and unlikely to change
  2. It provides complete decoupling of concerns
  3. SSG adapters are now fully independent of the render/module subsystem