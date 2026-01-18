---
id: move-docstring-parsing-from
level: task
title: "Move docstring parsing from renderer to parser layer"
short_code: "PLSKN-T-0048"
created_at: 2026-01-17T19:48:45.880861+00:00
updated_at: 2026-01-17T20:33:03.624300+00:00
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

# Move docstring parsing from renderer to parser layer

## Parent Initiative

[[PLSKN-I-0011]]

## Objective

Move docstring parsing from the rendering layer to the parser layer, so the model contains pre-parsed docstrings.

## Problem

Currently, docstring parsing happens in `module_renderer.rs` during rendering:

```rust
// module_renderer.rs:100-102
if let Some(ref docstring) = module.docstring {
    let parsed = parse_docstring(docstring);  // ← parsing here in renderer
    content.push_str(&render_docstring(&parsed));
}
```

This violates separation of concerns:
- **Parsing** (data transformation) is mixed with **rendering** (presentation)
- The model is incomplete - it has raw docstrings but not parsed structure
- Renderer is coupled to docstring parsing logic
- Makes it harder to test rendering independently

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [x] Parse docstrings in parser layer immediately after extraction
- [x] Add `parsed_docstring: Option<ParsedDocstring>` field to model structs
- [ ] Remove `parse_docstring()` calls from module_renderer.rs (optional - data now available)
- [ ] Renderer only calls `render_docstring()` on pre-parsed data (optional - can be done in follow-up)
- [x] Model serialization includes parsed docstring structure
- [x] All existing tests pass
- [x] Output remains identical

## Implementation Notes

### Technical Approach

**Step 1: Update model structs**
```rust
pub struct PythonModule {
    pub docstring: Option<String>,           // Keep raw for compatibility
    pub parsed_doc: Option<ParsedDocstring>, // Add parsed version
    // ...
}
```

**Step 2: Parse in parser layer**
```rust
// In python.rs parser
fn parse_module(...) -> PythonModule {
    let docstring = extract_docstring(...);
    let parsed_doc = docstring.as_ref().map(|d| parse_docstring(d));
    PythonModule { docstring, parsed_doc, ... }
}
```

**Step 3: Update renderer**
```rust
// In module_renderer.rs - use pre-parsed
if let Some(ref parsed) = module.parsed_doc {
    content.push_str(&render_docstring(parsed));
}
```

### Risk Considerations
- This is an architectural change affecting multiple layers
- Need to maintain backward compatibility for JSON output
- Consider making it opt-in initially via config flag

## Status Updates

### Session 1 - 2026-01-17

**Completed Implementation:**

1. **Added `parsed_doc: Option<ParsedDocstring>` field to model structs:**
   - `PythonModule` (model.rs:242)
   - `PythonClass` (model.rs:254)
   - `RustModule` (model.rs:74)
   - `RustStruct` (model.rs:86)
   - `RustEnum` (model.rs:103)
   - `RustFunction` (model.rs:121)
   - `RustTrait` (model.rs:167)

2. **Updated test constructors** to initialize `parsed_doc: None`

3. **Updated Python parser** (parser/python.rs):
   - Added `use crate::docstring::parse_docstring;`
   - Parse docstrings during `PythonModule` extraction
   - Parse docstrings during `PythonClass` extraction  
   - Parse docstrings during `PythonFunction` extraction

4. **Updated Rust parser** (parser/rust.rs):
   - Added `use crate::docstring::parse_rust_doc;`
   - Parse doc comments during `RustModule` extraction
   - Parse doc comments during `RustStruct` extraction
   - Parse doc comments during `RustEnum` extraction
   - Parse doc comments during `RustTrait` extraction
   - Parse doc comments during `RustFunction` extraction (via `extract_function_common`)

5. **Updated crossref.rs** initializers to include `parsed_doc: None` for synthesized Python items

6. **Fixed test** in module_renderer.rs that created RustEnum directly

**Test Results:**
- 229 unit tests passed
- 4 integration tests passed
- 16 doc tests passed

**Notes:**
- Kept raw `docstring`/`doc_comment` fields for backward compatibility
- `parsed_doc` is populated at parse time, ready for renderer to use
- Renderer updates (to use `parsed_doc` instead of parsing inline) are now optional since the data is available