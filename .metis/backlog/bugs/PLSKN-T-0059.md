---
id: fix-remaining-pyo3-type-conversion
level: task
title: "Fix remaining PyO3 type conversion gaps"
short_code: "PLSKN-T-0059"
created_at: 2026-01-18T16:45:08.406666+00:00
updated_at: 2026-01-18T19:23:07.073466+00:00
parent: 
blocked_by: []
archived: false

tags:
  - "#task"
  - "#bug"
  - "#phase/completed"


exit_criteria_met: false
strategy_id: NULL
initiative_id: NULL
---

# Fix remaining PyO3 type conversion gaps

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Objective

Fix remaining gaps in PyO3 type conversion where Rust types are still leaking through to Python documentation instead of being converted to Python equivalents.

## Backlog Item Details

### Type
- [x] Bug - Production issue that needs fixing

### Priority
- [x] P1 - High (important for user experience)

### Impact Assessment
- **Affected Users**: All users generating docs for PyO3 bindings with complex types
- **Reproduction Steps**: 
  1. Generate docs for a PyO3 binding with tuple return types or nested generics
  2. View the generated Python API signatures
  3. Observe Rust types instead of Python types

### Examples of broken conversions

**Current output:**
```
__call__(command: str, args: Vec < String >, kwargs: Option < & Bound < '_ , pyo3 :: types :: PyDict > >) -> PyResult < (i32 , String , String) >
tag(name: str, message: Option < & str >) -> PyResult < () >
branch(name: Option < & str >, delete: Option < bool >) -> PyResult < String >
```

**Expected output:**
```
__call__(command: str, args: List[str], kwargs: Optional[dict]) -> Tuple[int, str, str]
tag(name: str, message: Optional[str]) -> None
branch(name: Optional[str], delete: Optional[bool]) -> str
```

### Missing conversions identified

**Reported issues:**
1. **Tuple types**: `(i32, String, String)` should become `Tuple[int, str, str]`
2. **Nested references in generics**: `Option < & str >` not unwrapping the `&`
3. **Deeply nested wrappers**: `Option < & Bound < '_ , pyo3 :: types :: PyDict > >` not fully unwrapping
4. **Vec with spaces**: `Vec < String >` may not be converting (spacing issue in nested context)

**Additional scenarios to verify/fix:**

5. **PyResult with collections**:
   - `PyResult<Vec<String>>` → `List[str]`
   - `PyResult<HashMap<String, i32>>` → `Dict[str, int]`
   - `PyResult<Vec<HashMap<String, PyObject>>>` → `List[Dict[str, Any]]`

6. **Nested collections**:
   - `Vec<Vec<String>>` → `List[List[str]]`
   - `HashMap<String, Vec<i32>>` → `Dict[str, List[int]]`
   - `Option<Vec<String>>` → `Optional[List[str]]`

7. **References to collections**:
   - `&Vec<String>` → `List[str]`
   - `&mut HashMap<String, i32>` → `Dict[str, int]`
   - `&[u8]` → `bytes` (slice of bytes)

8. **Bound with collection types**:
   - `Bound<'_, PyList>` → `list`
   - `&Bound<'_, PyDict>` → `dict`
   - `Option<&Bound<'_, PyList>>` → `Optional[list]`

9. **Complex real-world patterns**:
   - `PyResult<Bound<'py, PyList>>` → `list`
   - `Option<&Bound<'_, pyo3::types::PyDict>>` → `Optional[dict]`
   - `Vec<(String, PyObject)>` → `List[Tuple[str, Any]]`

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [x] Tuple types `(T1, T2, ...)` convert to `Tuple[T1, T2, ...]`
- [x] Nested references like `Option<&str>` properly unwrap the `&`
- [x] Deeply nested wrappers like `Option<&Bound<'_, PyDict>>` fully unwrap to `Optional[dict]`
- [x] PyResult with collections: `PyResult<Vec<String>>` → `List[str]`
- [x] Nested collections: `Vec<Vec<String>>` → `List[List[str]]`
- [x] References to collections properly stripped: `&Vec<String>` → `List[str]`
- [x] Byte slices: `&[u8]` → `bytes`
- [x] All existing type conversion tests still pass (258 tests)
- [x] Add comprehensive test cases for all identified scenarios (4 new test functions)



## Implementation Notes

### Technical Approach

The type conversion logic is in `crates/plissken-core/src/crossref.rs` in the `rust_type_to_python()` and `rust_type_to_python_normalized()` functions.

**Root cause analysis:**
The current implementation normalizes whitespace then matches patterns, but:
- Inner types extracted from generics aren't being recursively converted
- Tuple syntax `(T1, T2)` isn't handled at all
- Slice syntax `&[T]` isn't handled

**Fixes needed:**
1. **Tuple parsing**: Detect `(T1, T2, ...)` pattern and convert to `Tuple[T1, T2, ...]`
   - Need to parse comma-separated types within parentheses
   - Recursively convert each element type

2. **Fix recursive unwrapping**: Ensure extracted inner types are recursively processed
   - `Option<&str>` → extract `&str` → recursively process → `str` → wrap as `Optional[str]`
   - Currently the inner type may not be getting the recursive call

3. **Slice syntax**: Add `&[T]` → `List[T]` and `&[u8]` → `bytes` special case

4. **Verify nested generics**: Test and fix `Vec<Vec<T>>`, `HashMap<K, Vec<V>>`, etc.

5. **Add helper for parsing tuple elements**: Similar to `split_generic_pair()` but for N elements

### Dependencies

- PLSKN-T-0056 (completed) - Initial PyO3 type conversion implementation

### Risk Considerations

- Need robust generic parsing that handles arbitrary nesting depth
- Edge cases with lifetimes in various positions
- Performance consideration for deeply nested types (should be fine for realistic use cases)

## Status Updates

### Session 1 - 2026-01-18

**Completed:**

1. **Added tuple type parsing**:
   - `(T1, T2, ...)` → `Tuple[T1, T2, ...]`
   - Handles nested tuples and complex element types
   - Added `split_tuple_elements()` helper function

2. **Added slice syntax support**:
   - `[T]` → `List[T]`
   - `[u8]` → `bytes` (special case)

3. **Fixed bracket handling in `split_generic_pair()`**:
   - Now tracks `(` and `)` in addition to `<` and `>`
   - Enables correct parsing of types containing tuples

4. **Added comprehensive test coverage**:
   - `test_rust_type_to_python_tuples`: Basic and nested tuple conversions
   - `test_rust_type_to_python_nested_references`: `Option<&str>` → `Optional[str]`
   - `test_rust_type_to_python_slices`: `&[u8]` → `bytes`, `&[T]` → `List[T]`
   - `test_rust_type_to_python_complex_nested`: Deep nesting like `Vec<(String, PyObject)>` → `List[Tuple[str, Any]]`

**Verified conversions:**
- `(i32, String, String)` → `Tuple[int, str, str]`
- `PyResult<(i32, String, String)>` → `Tuple[int, str, str]`
- `Option<&str>` → `Optional[str]`
- `Option<&Bound<'_, PyDict>>` → `Optional[dict]`
- `Vec<Vec<String>>` → `List[List[str]]`
- `HashMap<String, Vec<i32>>` → `Dict[str, List[int]]`
- `Vec<(String, PyObject)>` → `List[Tuple[str, Any]]`
- `&[u8]` → `bytes`
- `&[String]` → `List[str]`

**Files modified:**
- `crates/plissken-core/src/crossref.rs`
  - Added tuple handling at start of `rust_type_to_python_normalized()`
  - Added slice handling
  - Added `split_tuple_elements()` helper
  - Updated `split_generic_pair()` to track parentheses
  - Added 4 new test functions

**All 258 tests passing.**