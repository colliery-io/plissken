---
id: add-snapshot-tests-for-rendered
level: task
title: "Add snapshot tests for rendered output"
short_code: "PLSKN-T-0051"
created_at: 2026-01-17T19:48:46.970394+00:00
updated_at: 2026-01-18T01:25:00.345816+00:00
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

# Add snapshot tests for rendered output

## Parent Initiative

[[PLSKN-I-0011]]

## Objective

Add snapshot/golden file testing for rendered markdown output to catch regressions.

## Problem

Current testing gaps:
- `module_renderer.rs` has 1,993 lines but only 12 tests
- No snapshot testing despite complex rendering logic
- Hard to detect output format changes or regressions
- Manual verification required for rendering changes

Without snapshot tests:
- Subtle changes to output format go unnoticed
- Refactoring is risky (can't verify output unchanged)
- Badge formatting, heading levels, etc. can silently break

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [x] Add `insta` crate for snapshot testing
- [x] Create snapshots for Python module rendering
- [x] Create snapshots for Rust module rendering  
- [x] Create snapshots for each item type (class, function, struct, enum)
- [x] Snapshots include both inline and page formats
- [x] CI fails if snapshots change unexpectedly
- [x] Document how to update snapshots

## Implementation Notes

### Technical Approach

**Step 1: Add insta dependency**
```toml
[dev-dependencies]
insta = { version = "1.34", features = ["yaml"] }
```

**Step 2: Create snapshot tests**
```rust
#[cfg(test)]
mod snapshot_tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_python_module_rendering() {
        let module = PythonModule::test("mymodule")
            .with_docstring("Module docstring")
            .with_class(PythonClass::test("MyClass"));
        
        let renderer = ModuleRenderer::new(...);
        let output = renderer.render_python_module_inline(&module).unwrap();
        
        assert_snapshot!("python_module_inline", output);
    }

    #[test]
    fn test_rust_struct_rendering() {
        let s = RustStruct::test("Config")
            .with_field("name", "String")
            .with_doc("Configuration struct");
        
        let renderer = ModuleRenderer::new(...);
        let output = renderer.render_rust_struct_inline(&s).unwrap();
        
        assert_snapshot!("rust_struct_inline", output);
    }
}
```

**Step 3: Create comprehensive fixtures**
- Simple module (minimal content)
- Complex module (all item types)
- PyO3 binding module (cross-references)
- Nested structures

### Dependencies
- None - can be done independently

### Risk Considerations
- Snapshots need updating when intentional changes are made
- Large snapshots can be hard to review
- Consider using `insta`'s review workflow

## Status Updates

### Implementation Complete

**Added insta dependency:**
- `crates/plissken-core/Cargo.toml` - Added `insta = { version = "1.34", features = ["yaml"] }`

**Created 16 snapshot tests covering:**

Python module snapshots (7 tests):
- `snapshot_python_module_simple` - Minimal module with docstring
- `snapshot_python_module_with_function` - Module with typed function and docstring
- `snapshot_python_class_with_methods` - Class with init, methods, attributes, classmethod
- `snapshot_python_async_function` - Async function with full docstring
- `snapshot_python_pyo3_binding` - PyO3 binding with cross-references
- `snapshot_python_enum_class` - Python Enum class with members
- `snapshot_python_module_variables` - Module with constants/variables

Rust module snapshots (7 tests):
- `snapshot_rust_module_simple` - Minimal module with doc comment
- `snapshot_rust_struct_with_fields` - Struct with generics, derives, fields, methods
- `snapshot_rust_enum_with_variants` - Enum with documented variants
- `snapshot_rust_function_with_generics` - Generic function with Rust doc
- `snapshot_rust_async_unsafe_function` - Async unsafe function
- `snapshot_rust_pyclass_struct` - #[pyclass] with pymethods and cross-refs
- `snapshot_rust_pyfunction` - #[pyfunction] with cross-refs

Complex modules (2 tests):
- `snapshot_complex_python_module` - Full module with classes, functions, variables
- `snapshot_complex_rust_module` - Full module with structs, enums, impl blocks, functions

**Files created:**
- `crates/plissken-core/src/render/snapshots/*.snap` - 16 snapshot files

**Updating snapshots:**
Documentation added as module doc comment in the test code:
```bash
cargo insta review  # Interactive review
# or
INSTA_UPDATE=always cargo test  # Auto-accept all
```

**Verification:**
- Test count increased from 229 to 245 unit tests
- All 4 integration tests pass
- All 16 doc tests pass (6 ignored)