---
id: cross-reference-builder
level: task
title: "Cross-Reference Builder"
short_code: "PLSKN-T-0003"
created_at: 2026-01-14T03:32:09.465460+00:00
updated_at: 2026-01-14T14:23:02.705629+00:00
parent: PLSKN-I-0001
blocked_by: []
archived: true

tags:
  - "#task"
  - "#phase/completed"


exit_criteria_met: false
strategy_id: NULL
initiative_id: PLSKN-I-0001
---

# Cross-Reference Builder

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[PLSKN-I-0001]]

## Objective

Build cross-references between Python and Rust items based on PyO3 metadata.

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

### Business Justification **[CONDITIONAL: Feature]**
- **User Value**: {Why users need this}
- **Business Value**: {Impact on metrics/revenue}
- **Effort Estimate**: {Rough size - S/M/L/XL}

### Technical Debt Impact **[CONDITIONAL: Tech Debt]**
- **Current Problems**: {What's difficult/slow/buggy now}
- **Benefits of Fixing**: {What improves after refactoring}
- **Risk Assessment**: {Risks of not addressing this}

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [x] Match #[pyclass(name="X")] to Python class X
- [x] Match #[pyfunction(name="x")] to Python function x
- [x] Populate `rust_impl` on Python items
- [x] Build `CrossRef` entries in DocModel
- [x] Handle module-level name mapping from #[pymodule]
- [x] Pass tests against `hybrid_binary` and `separate_bindings` fixtures

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
- Runs after both parsers complete
- Uses config to determine which Python modules are PyO3 bindings
- Path matching: `module.Class` ↔ `crate::module::RustClass`

### Dependencies
- PLSKN-T-0001 (Rust Parser)
- PLSKN-T-0002 (Python Parser)

### Risk Considerations
{Technical risks and mitigation strategies}

## Status Updates **[REQUIRED]**

### Session 1 - 2026-01-13
**Implementation Complete**

Implemented cross-reference builder in `crates/plissken-core/src/crossref.rs` (~410 lines).

**Two main capabilities:**

1. **`build_cross_refs()`** - Match existing Python modules to Rust implementations:
   - Indexes all `#[pyclass]` structs with their Python-visible names
   - Indexes all `#[pyfunction]` functions with their Python names
   - Indexes all methods in `#[pymethods]` impl blocks
   - Updates Python items with `rust_impl` references
   - Builds `CrossRef` entries for the DocModel
   - Respects config to distinguish PyO3 vs pure Python modules

2. **`synthesize_python_from_rust()`** - Generate Python modules from Rust:
   - Creates Python class entries from `#[pyclass]` structs
   - Creates Python function entries from `#[pyfunction]` functions
   - Synthesizes methods from `#[pymethods]` impl blocks
   - Copies docstrings from Rust to Python
   - Converts Rust types to Python type hints (best effort)
   - Useful when there are no Python stub files

**Tests Added (9 total, all passing):**
1. `test_pyclass_matching` - Match pyclass to Python class
2. `test_pyfunction_matching` - Match pyfunction to Python function
3. `test_pymethods_matching` - Match methods in impl blocks
4. `test_pure_python_unchanged` - Non-PyO3 modules unchanged
5. `test_name_fallback_to_rust_name` - Fallback when no explicit name
6. `test_synthesize_python_from_rust` - Synthesis with manual input
7. `test_rust_type_to_python` - Type conversion mapping
8. `test_synthesize_hybrid_binary_fixture` - Integration against fixture
9. `test_synthesize_separate_bindings_fixture` - Integration against fixture

**All 30 project tests passing.**