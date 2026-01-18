---
id: cross-reference-links
level: task
title: "Cross-Reference Links"
short_code: "PLSKN-T-0011"
created_at: 2026-01-14T16:56:04.135754+00:00
updated_at: 2026-01-14T20:27:22.149418+00:00
parent: PLSKN-I-0002
blocked_by: []
archived: true

tags:
  - "#task"
  - "#phase/completed"


exit_criteria_met: false
strategy_id: NULL
initiative_id: PLSKN-I-0002
---

# Cross-Reference Links

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[PLSKN-I-0002]]

## Objective

Generate working Markdown links between Python items and their Rust implementations using the CrossRef data from DocModel.

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [x] Python class with `rust_impl` links to Rust struct section
- [x] Python function with `rust_impl` links to Rust function section  
- [x] Links use relative paths that work within generated docs
- [x] Rust items link back to Python exposure where applicable
- [x] Handle items without cross-references gracefully
- [x] Test links resolve correctly in generated output

## Implementation Notes

### Technical Approach
- Created `render/crossref_renderer.rs` module with relative path computation
- Path utilities convert Rust paths (`::` separator) to file paths (`/`)
- Python paths (`.` separator) also converted to file paths
- Relative paths computed based on directory depth from source page
- Anchors generated using lowercase item names with type prefix (e.g., `#struct-config`)

### Files Created/Modified
- `crates/plissken-core/src/render/crossref_renderer.rs` (new)
- `crates/plissken-core/src/render/mod.rs` (exports added)
- `crates/plissken-core/src/lib.rs` (public API exports)

## Status Updates **[REQUIRED]**

### 2026-01-14: Completed

Created `render/crossref_renderer.rs` with complete cross-reference link generation:

**Public API:**
- `link_to_rust(rust_ref, from_python_path)` - Generate link from Python to Rust item
- `link_to_python(python_path, from_rust_path)` - Generate link from Rust to Python item
- `crossref_link(xref, from_path, from_language)` - Generate link based on CrossRef relationship
- `CrossRefLink` struct with `to_markdown()` and `to_markdown_with_badge()` methods
- `render_rust_impl_details(rust_ref, from_python_path)` - Collapsible details block with link
- `render_python_exposure_details(python_path, from_rust_path)` - Collapsible details for Rust items
- `Language` enum (Python, Rust) for determining link direction

**Path Utilities:**
- `rust_path_to_file_path()` - `crate::utils` → `crate/utils.md`
- `python_path_to_file_path()` - `mypackage.utils` → `mypackage/utils.md`
- `split_python_path()` / `split_rust_path()` - Extract module and item name
- `item_to_anchor()` - Convert item name to Markdown anchor
- `compute_relative_path()` - Calculate relative path between doc pages

**Test Coverage:**
- 20 unit tests covering all functions
- Tests for Python→Rust links, Rust→Python links, nested modules
- Tests for all CrossRefKind relationships (Binding, Wraps, Delegates)
- Tests for markdown rendering with and without badges

**Files Modified:**
- Created: `crates/plissken-core/src/render/crossref_renderer.rs` (~300 lines)
- Updated: `crates/plissken-core/src/render/mod.rs` (added exports)
- Updated: `crates/plissken-core/src/lib.rs` (added public exports)

**Test Results:** 128 unit tests + 8 doc-tests passing

### Final Verification
All acceptance criteria met:
- ✅ Python class with `rust_impl` links to Rust struct section (via `link_to_rust`, `crossref_link`)
- ✅ Python function with `rust_impl` links to Rust function section (via `link_to_rust`, `crossref_link`)
- ✅ Links use relative paths that work within generated docs (`compute_relative_path`)
- ✅ Rust items link back to Python exposure (`link_to_python`, `render_python_exposure_details`)
- ✅ Handle items without cross-references gracefully (tested in `test_handles_missing_crossref_gracefully`)
- ✅ Test links resolve correctly - 20 unit tests verify link generation