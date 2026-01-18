---
id: module-page-renderer
level: task
title: "Module Page Renderer"
short_code: "PLSKN-T-0009"
created_at: 2026-01-14T16:56:04.020674+00:00
updated_at: 2026-01-14T20:56:13.342956+00:00
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

# Module Page Renderer

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[PLSKN-I-0002]]

## Objective

Render Python and Rust modules from DocModel into per-module Markdown files with proper structure, signatures, and docstrings.

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [x] Render PythonModule → `{module_path}.md` with classes and functions
- [x] Render RustModule → `rust/{module_path}.md` with structs, functions, impls
- [x] Class/struct sections with name, signature, docstring
- [x] Function sections with signature in code block, docstring
- [x] Collapsible `<details>` blocks for Rust implementation on Python items
- [x] Test rendering produces expected file structure (11 unit tests)

## Implementation Notes

### Technical Approach
Created a `ModuleRenderer` struct that wraps the `Renderer` and provides specialized methods for rendering Python and Rust modules. Output is returned as `RenderedPage` structs containing the path and content.

### Key Components

**`RenderedPage`** - Output container with:
- `path: PathBuf` - Relative output path (e.g., `my_module.md` or `rust/crate/module.md`)
- `content: String` - Rendered Markdown

**`ModuleRenderer`** - Methods:
- `render_python_module(module)` - Renders Python module with classes and functions
- `render_rust_module(module)` - Renders Rust module with structs, enums, functions
- `render_python_modules(modules)` - Batch render
- `render_rust_modules(modules)` - Batch render

### Output Structure

**Python modules:** `{module.path.replace('.', '/')}.md`
- Example: `myapp.models` → `myapp/models.md`

**Rust modules:** `rust/{module.path.replace('::', '/')}.md`
- Example: `crate::utils` → `rust/crate/utils.md`

### Features

1. **Source badges** on modules (🐍 Python, 🦀 Rust, 🐍↔🦀 Binding)
2. **Visibility badges** on Rust items (pub, pub(crate), private)
3. **Method badges** (async, unsafe, const, property, staticmethod, classmethod)
4. **Collapsible Rust details** on Python bindings via `<details>` tags
5. **Field tables** for Rust structs
6. **Variant lists** for Rust enums
7. **Signature code blocks** using the existing signature renderer

## Status Updates **[REQUIRED]**

### 2026-01-14 - Implementation Complete

Created the `ModuleRenderer` for rendering Python and Rust modules to Markdown.

**Files created:**
- `crates/plissken-core/src/render/module_renderer.rs` - ModuleRenderer + RenderedPage

**Files modified:**
- `crates/plissken-core/src/render/mod.rs` - Added module_renderer export
- `crates/plissken-core/src/lib.rs` - Added ModuleRenderer, RenderedPage to public API

**Tests added (11 new):**
- `test_render_simple_python_module` - Basic Python module
- `test_render_python_module_with_class` - Class with methods and inheritance
- `test_render_python_module_pyo3_binding` - PyO3 binding with Rust details
- `test_render_simple_rust_module` - Basic Rust module with functions
- `test_render_rust_module_with_struct` - Struct with fields and impl methods
- `test_render_rust_struct_with_pyclass` - PyClass-annotated struct
- `test_render_rust_function_badges` - Async/unsafe/visibility badges
- `test_render_rust_enum` - Enum with variants
- `test_batch_render_modules` - Batch rendering multiple modules
- `test_python_async_function` - Async badge on Python functions
- `test_python_class_methods_types` - staticmethod/classmethod/property badges

**Total tests:** 95 unit tests + 4 doc-tests, all passing.