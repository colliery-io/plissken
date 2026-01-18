---
id: split-render-function-into-smaller
level: task
title: "Split render() function into smaller focused functions"
short_code: "PLSKN-T-0042"
created_at: 2026-01-17T19:48:44.183451+00:00
updated_at: 2026-01-17T20:01:04.078015+00:00
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

# Split render() function into smaller focused functions

## Parent Initiative

[[PLSKN-I-0011]]

## Objective

Split the 248-line `render()` function in `main.rs:220-468` into smaller, focused functions following single responsibility principle.

## Problem

The `render()` function is too large and handles multiple concerns:
- Output directory resolution (lines 238-246)
- Python/Rust module synthesis and merging (lines 271-320)
- File rendering and writing loops (lines 369-417)
- Template-specific SSG generation (lines 421-457)

This makes the code hard to read, test, and maintain.

## Acceptance Criteria

## Acceptance Criteria

- [x] `render()` function reduced to under 50 lines (now 51 lines including signature, 44 lines body)
- [x] Extract `resolve_output_directory()` helper
- [x] Extract `merge_synthesized_modules()` helper (renamed to `merge_synthesized_python_modules()`)
- [x] Extract `write_module_files()` helper (renamed to `write_rendered_pages()`)
- [x] Extract `generate_ssg_output()` helper (renamed to `generate_ssg_files()`)
- [x] Each new function has clear single responsibility (11 functions extracted total)
- [x] All existing tests pass (223 unit, 4 integration, 16 doc tests)
- [x] No change in behavior

## Implementation Notes

### Technical Approach

Split into these functions:

```rust
fn render(...) -> Result<()> {
    let output_dir = resolve_output_directory(&config, project_root, output)?;
    let (python_modules, rust_modules) = parse_sources(&config, project_root)?;
    let (python_modules, cross_refs) = merge_synthesized_modules(&config, python_modules, rust_modules)?;
    write_module_files(&module_renderer, &python_modules, &rust_modules, &output_dir)?;
    generate_ssg_output(&module_renderer, &python_modules, &rust_modules, &output_dir, template)?;
    Ok(())
}

fn resolve_output_directory(config: &Config, project_root: &Path, output: Option<&Path>) -> Result<PathBuf>;
fn merge_synthesized_modules(config: &Config, python: Vec<PythonModule>, rust: Vec<RustModule>) -> Result<(Vec<PythonModule>, Vec<CrossRef>)>;
fn write_module_files(renderer: &ModuleRenderer, python: &[PythonModule], rust: &[RustModule], output: &Path) -> Result<()>;
fn generate_ssg_output(renderer: &ModuleRenderer, python: &[PythonModule], rust: &[RustModule], output: &Path, template: Option<&str>) -> Result<()>;
```

### Risk Considerations
- Large refactor - do in small commits
- Test after each extraction

## Status Updates

### 2026-01-17 Session

**Completed refactoring of `render()` function:**

Original: ~250 lines (220-468)
Refactored: ~51 lines (220-270)

**New helper functions extracted:**

1. `load_project_config()` - Load config and project root from path
2. `resolve_output_directory()` - Resolve output dir from config/override  
3. `log_output_settings()` - Log output directory and template
4. `parse_and_merge_modules()` - Parse sources and merge synthesized modules
5. `create_renderer()` - Create renderer with template
6. `create_output_directory()` - Create output dir with error handling
7. `resolve_content_directory()` - Determine content dir based on SSG
8. `merge_synthesized_python_modules()` - Merge Rust PyO3 synthesized modules
9. `build_cross_references()` - Build cross-references between Python/Rust
10. `write_rendered_pages()` - Write module pages to disk
11. `generate_ssg_files()` - Generate SSG-specific navigation/config

**Test Results:**
- All 223 unit tests pass
- All 4 integration tests pass
- All 16 doc tests pass
- No new compiler warnings

**Changes:**
- `render()` now has clear, readable sections with single responsibilities
- Each helper function has a focused purpose
- Total file grew by ~100 lines but maintainability greatly improved