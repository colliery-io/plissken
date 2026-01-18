---
id: split-documentation-into-per-class
level: task
title: "Split Documentation into Per-Class Files"
short_code: "PLSKN-T-0017"
created_at: 2026-01-15T03:29:53.403251+00:00
updated_at: 2026-01-15T03:49:57.294997+00:00
parent: PLSKN-I-0003
blocked_by: []
archived: true

tags:
  - "#task"
  - "#phase/completed"


exit_criteria_met: false
strategy_id: NULL
initiative_id: PLSKN-I-0003
---

# Split Documentation into Per-Class Files

Refactor the module renderer to generate one markdown file per class/struct instead of monolithic module files.

## Parent Initiative

[[PLSKN-I-0003]]

## Objective

Split documentation output from monolithic 500+ line module files into individual per-class/struct files with module index pages. This improves navigation, discoverability, and page load times.

## Problem Statement

Current output structure:
- `python/hybrid_binary.md` - 527 lines containing 4 classes + 1 function + all methods
- `rust/rust.md` - 444 lines containing 3 structs + all methods

Issues:
- Cognitive overload from long pages
- Poor discoverability (must scroll to find classes)
- No class-level URLs (can't link directly to `Runner`)
- Mixed concerns in same visual space
- Inconsistent splitting (submodules get own files but classes don't)

## Target Structure

```
docs/
├── index.md                           # Landing with quick links
│
├── python/
│   ├── index.md                       # Python API overview (class cards)
│   └── hybrid_binary/
│       ├── index.md                   # Module overview
│       ├── Task.md                    # Task class
│       ├── Runner.md                  # Runner class  
│       ├── RunResult.md               # RunResult class
│       ├── TaskDecorator.md           # TaskDecorator class
│       └── functions.md               # Module-level functions
│
└── rust/
    ├── index.md                       # Rust API overview
    └── hybrid_binary/
        ├── index.md                   # Crate overview
        ├── Task.md                    # PyTask struct
        ├── Runner.md                  # PyRunner struct
        ├── RunResult.md               # PyRunResult struct
        ├── internal/
        │   ├── index.md               # Internal module
        │   ├── Task.md                # internal::Task
        │   └── TaskExecutor.md        # TaskExecutor
        └── decorators/
            ├── index.md               # Decorators module
            └── TaskDecorator.md       # TaskDecorator struct
```

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [x] `render_python_module` returns `Vec<RenderedPage>` (one per class + module index)
- [x] `render_rust_module` returns `Vec<RenderedPage>` (one per struct + module index)
- [x] Module index pages include class cards with 1-line descriptions
- [x] Class/struct pages are self-contained with all methods
- [x] Cross-reference links updated to use new paths (`python/hybrid_binary/Task.md`)
- [x] MkDocs nav can be auto-generated from directory structure
- [x] Existing tests updated for new output structure
- [x] Integration tests verify multi-file output

## Size Guidelines

| Content Type | Target Lines | Max Lines |
|--------------|--------------|-----------|
| Module index | 50-100 | 150 |
| Class page | 100-200 | 300 |
| Function page | 30-80 | 150 |

## Implementation Notes

### Technical Approach

1. **ModuleRenderer changes** (`module_renderer.rs`):
   - Add `render_python_class_page()` returning `RenderedPage`
   - Add `render_rust_struct_page()` returning `RenderedPage`
   - Add `render_module_index()` for overview pages with class cards
   - Change `render_python_module()` to orchestrate and return `Vec<RenderedPage>`
   - Change `render_rust_module()` to orchestrate and return `Vec<RenderedPage>`

2. **Path generation**:
   - Python: `python/{module}/{ClassName}.md`
   - Rust: `rust/{crate}/{module}/{StructName}.md`
   - Index: `python/{module}/index.md`

3. **Cross-reference updates**:
   - Update `python_link_for_rust_item()` and related helpers
   - Anchor format changes from `#class-task` to just linking to `Task.md`

4. **CLI changes** (`main.rs`):
   - Handle `Vec<RenderedPage>` from renderers
   - Write all pages to disk

### Dependencies

- Depends on PLSKN-T-0015 (cross-links) being complete for path calculation logic

### Risk Considerations

- Breaking change to output structure - existing nav configurations will need updates
- More files means more disk I/O - should still be fast for typical project sizes

## Status Updates

### 2026-01-15 - Implementation Complete

**Completed work:**

1. **ModuleRenderer refactoring** (`module_renderer.rs`):
   - Changed `render_python_module()` to return `Vec<RenderedPage>`
   - Changed `render_rust_module()` to return `Vec<RenderedPage>`
   - Added `render_python_class_page()` - renders individual class as own page
   - Added `render_python_module_index()` - renders index with class cards
   - Added `render_python_functions_page()` - renders module-level functions
   - Added `render_rust_struct_page()` - renders individual struct as own page
   - Added `render_rust_enum_page()` - renders individual enum as own page
   - Added `render_rust_module_index()` - renders index with struct/enum cards
   - Added `render_rust_functions_page()` - renders module-level functions

2. **Cross-reference link updates**:
   - Added `rust_link_for_python_class()` - links from Python class page to Rust struct page
   - Added `rust_link_for_python_function()` - links from Python functions to Rust functions
   - Added `python_link_for_rust_struct()` - links from Rust struct page to Python class page
   - Updated path depth calculations for new directory structure

3. **CLI updates** (`main.rs`):
   - Updated render loop to handle `Vec<RenderedPage>` from renderers
   - Each page in the vector is written to its own file

4. **Test updates**:
   - All 12 module_renderer tests updated for new `Vec<RenderedPage>` return type
   - Added `find_page()` helper for locating pages by path suffix
   - Tests verify index pages, class pages, struct pages, and functions pages

**Output structure now generates:**
```
python/{module}/
├── index.md         # Module overview with class cards
├── {ClassName}.md   # One per class
└── functions.md     # Module-level functions

rust/{crate}/{module}/
├── index.md         # Module overview with struct/enum cards
├── {StructName}.md  # One per struct
├── {EnumName}.md    # One per enum
└── functions.md     # Module-level functions
```

**Verified with integration test:** Generated 16 files from hybrid_binary test fixture.

### 2026-01-15 - User Review Feedback

**Issues identified during review:**

1. **Excessive "Binding" badges** - Every method showing a "Binding" badge is distracting in both navigation and docs. The binding nature is already evident from:
   - Rust code shown in Python docs
   - Bi-directional cross-reference links
   - Badge should only appear at class level (if at all)

2. **Structure improvements needed** - Researched Sphinx autodoc patterns (Requests library example):
   - Properties/Attributes section should come first
   - Methods section second  
   - Minimal visual noise - Sphinx uses only `[source]` and `¶` markers
   - Consistent method formatting: signature → description → params → returns

**Planned refinements:**
- [x] Remove "Binding" badge from individual methods (keep only at class level)
- [ ] Restructure class pages: Properties first, Methods second
- [x] Simplify method cross-refs to just show link without badge
- [ ] Consider adopting Sphinx-style parameter/return formatting

**References:**
- Sphinx autodoc: https://www.sphinx-doc.org/en/master/usage/extensions/autodoc.html
- Requests API docs: https://requests.readthedocs.io/en/latest/api/

### 2026-01-15 - Badge Refinements Implemented

**Completed:**
- Removed "Binding" badge from Python method headings (`render_python_function_with_context`)
- Removed "Binding" badge from Rust method headings (`render_rust_function_with_context`)
- Class/struct level badges retained for quick identification
- Method-level binding nature now conveyed through:
  - Bi-directional cross-reference links (e.g., "**Rust Implementation**: [...]")
  - Rust source code shown in collapsible sections

**Files modified:**
- `crates/plissken-core/src/render/module_renderer.rs` (lines 314-317, 739-747)

**Result:** All 129 tests passing. Documentation is now cleaner with less visual noise.