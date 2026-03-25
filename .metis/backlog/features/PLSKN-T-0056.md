---
id: show-python-types-instead-of-rust
level: task
title: "Show Python types instead of Rust PyO3 types for bindings"
short_code: "PLSKN-T-0056"
created_at: 2026-01-18T15:56:21.032796+00:00
updated_at: 2026-01-18T19:23:06.955514+00:00
parent: 
blocked_by: []
archived: false

tags:
  - "#task"
  - "#feature"
  - "#phase/completed"


exit_criteria_met: false
strategy_id: NULL
initiative_id: NULL
---

# Show Python types instead of Rust PyO3 types for bindings

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[Parent Initiative]]

## Objective

When documenting PyO3/Maturin bindings, show Python types instead of Rust PyO3 wrapper types (e.g., `str` instead of `PyString`, `list` instead of `PyList`).

## Backlog Item Details **[CONDITIONAL: Backlog Item]**

{Delete this section when task is assigned to an initiative}

### Type
- [ ] Bug - Production issue that needs fixing
- [ ] Feature - New functionality or enhancement  
- [ ] Tech Debt - Code improvement or refactoring
- [ ] Chore - Maintenance or setup work

### Priority
- [ ] P0 - Critical (blocks users/revenue)
- [ ] P1 - High (important for user experience)
- [ ] P2 - Medium (nice to have)
- [ ] P3 - Low (when time permits)

### Impact Assessment **[CONDITIONAL: Bug]**
- **Affected Users**: {Number/percentage of users affected}
- **Reproduction Steps**: 
  1. {Step 1}
  2. {Step 2}
  3. {Step 3}
- **Expected vs Actual**: {What should happen vs what happens}

### Business Justification
- **User Value**: Python users don't need to see Rust internals - they want Python types
- **Effort Estimate**: M

### Technical Debt Impact **[CONDITIONAL: Tech Debt]**
- **Current Problems**: {What's difficult/slow/buggy now}
- **Benefits of Fixing**: {What improves after refactoring}
- **Risk Assessment**: {Risks of not addressing this}

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [x] `PyResult<T>` shows as Python type (unwraps to inner type T)
- [x] `PyObject` / `Py<PyAny>` shows as `Any`
- [x] `PyString` → `str`, `PyList` → `list`, `PyDict` → `dict`, etc.
- [x] Return types in generated docs are Python-native representations

## Test Cases **[CONDITIONAL: Testing Task]**

{Delete unless this is a testing task}

### Test Case 1: {Test Case Name}
- **Test ID**: TC-001
- **Preconditions**: {What must be true before testing}
- **Steps**: 
  1. {Step 1}
  2. {Step 2}
  3. {Step 3}
- **Expected Results**: {What should happen}
- **Actual Results**: {To be filled during execution}
- **Status**: {Pass/Fail/Blocked}

### Test Case 2: {Test Case Name}
- **Test ID**: TC-002
- **Preconditions**: {What must be true before testing}
- **Steps**: 
  1. {Step 1}
  2. {Step 2}
- **Expected Results**: {What should happen}
- **Actual Results**: {To be filled during execution}
- **Status**: {Pass/Fail/Blocked}

## Documentation Sections **[CONDITIONAL: Documentation Task]**

{Delete unless this is a documentation task}

### User Guide Content
- **Feature Description**: {What this feature does and why it's useful}
- **Prerequisites**: {What users need before using this feature}
- **Step-by-Step Instructions**:
  1. {Step 1 with screenshots/examples}
  2. {Step 2 with screenshots/examples}
  3. {Step 3 with screenshots/examples}

### Troubleshooting Guide
- **Common Issue 1**: {Problem description and solution}
- **Common Issue 2**: {Problem description and solution}
- **Error Messages**: {List of error messages and what they mean}

### API Documentation **[CONDITIONAL: API Documentation]**
- **Endpoint**: {API endpoint description}
- **Parameters**: {Required and optional parameters}
- **Example Request**: {Code example}
- **Example Response**: {Expected response format}

## Implementation Notes **[CONDITIONAL: Technical Task]**

{Keep for technical tasks, delete for non-technical. Technical details, approach, or important considerations}

### Technical Approach
{How this will be implemented}

### Dependencies
{Other tasks or systems this depends on}

### Risk Considerations
{Technical risks and mitigation strategies}

## Status Updates **[REQUIRED]**

### Session 1 - 2026-01-18

**Completed:**
- Enhanced `rust_type_to_python()` function in `crossref.rs` with comprehensive PyO3 type support
- Added whitespace normalization to handle spaced types like `PyResult < String >`
- Added support for PyO3 primitive types: `PyString` → `str`, `PyList` → `list`, `PyDict` → `dict`, `PyTuple` → `tuple`, `PySet` → `set`, `PyBytes` → `bytes`, `PyBool` → `bool`, `PyInt`/`PyFloat` → `int`/`float`, `PyNone` → `None`
- Added support for `PyObject`/`PyAny` → `Any`
- Added support for `Py<T>` wrapper - unwraps to inner type
- Added support for `Bound<'_, T>` wrapper - extracts type after lifetime
- Added support for `Result<T, E>` - extracts success type
- Added support for path-qualified types like `pyo3::types::PyString`
- Added helper function `split_generic_pair()` for parsing `HashMap<K, V>` types
- Added 6 comprehensive unit tests for type conversion

**Verified output:**
- Python docs now show: `Optional[int]` instead of `Option < usize >`
- `PyResult<()>` → `None`
- `PyObject` → `Any`
- `Bound<'_, PyDict>` → `dict`
- Rust docs preserve original types (correct for Rust developers)

**Files modified:**
- `crates/plissken-core/src/crossref.rs` - Enhanced `rust_type_to_python()` function

**All 254 tests passing (including 6 new type conversion tests).**