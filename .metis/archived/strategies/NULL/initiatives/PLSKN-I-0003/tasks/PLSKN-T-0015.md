---
id: add-bi-directional-cross-links-for
level: task
title: "Add Bi-directional Cross-links for Bindings"
short_code: "PLSKN-T-0015"
created_at: 2026-01-15T02:11:49.670384+00:00
updated_at: 2026-01-15T02:21:28.881793+00:00
parent: PLSKN-I-0003
blocked_by: []
archived: true

tags:
  - "#task"
  - "#phase/active"


exit_criteria_met: false
strategy_id: NULL
initiative_id: PLSKN-I-0003
---

# Add Bi-directional Cross-links for Bindings

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[PLSKN-I-0003]]

## Objective

Add bi-directional links between Python and Rust views for PyO3 bindings.

**File:** `crates/plissken-core/src/render/module_renderer.rs`

In Rust binding docs: `> **Python API**: [hybrid_binary.task](../python/hybrid_binary.md#task)`

In Python binding docs: `> **Rust Implementation**: [decorators::task](../rust/hybrid_binary/decorators.md#task)`

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

- [x] Rust binding docs include link to Python counterpart
- [x] Python binding docs include link to Rust implementation
- [x] Links resolve correctly between `python/` and `rust/` directories

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

### Session 2 - Completed
- Updated `crossref_renderer.rs` to use `python/` prefix for Python file paths
- Updated `link_to_rust` and `link_to_python` functions for correct relative paths
- Modified `ModuleRenderer` struct to store cross-refs
- Added `with_cross_refs` constructor for ModuleRenderer
- Added `python_link_for_rust_item` helper for Rust->Python links
- Added `rust_link_for_python_item` helper for Python->Rust links
- Updated `render_rust_struct` to add Python API link for pyclass structs
- Updated `render_python_class` to add Rust Implementation link for binding classes
- Updated `render_python_function` to add Rust Implementation link for binding functions
- Updated CLI to use `with_cross_refs` constructor
- Fixed integration tests for new `python/` output directory
- Fixed unit tests for updated path expectations

**Files Modified:**
- `crates/plissken-core/src/render/crossref_renderer.rs`
- `crates/plissken-core/src/render/module_renderer.rs`
- `crates/plissken-cli/src/main.rs`
- `crates/plissken-cli/tests/integration_test.rs`

**Result:** All 128 tests passing. Cross-links now appear in rendered docs:
- Rust docs: `> **Python API**: [hybrid_binary.Task](../python/hybrid_binary.md#task)`
- Python docs: `> **Rust Implementation**: [hybrid_binary::PyTask](../rust/hybrid_binary.md#pytask)`

### Session 3 - Per-File Cross-Reference Fix
Following the per-class file split (PLSKN-T-0017), updated cross-ref link helpers for new file structure:

**Changes:**
- Fixed `rust_link_for_python_method()` to generate links to per-struct pages
  - Old: `../rust/hybrid_binary.md#method`
  - New: `../../rust/hybrid_binary/PyTask.md#method`
- Fixed depth calculation for cross-refs (now +2 for per-class file depth)
- Fixed `rust_link_for_python_class()` to link to individual struct pages
- All cross-refs now use correct relative paths for new directory structure

**Verified Links:**
- Class-level: `[hybrid_binary::PyTask](../../rust/hybrid_binary/PyTask.md)`
- Method-level: `[hybrid_binary::PyTask::new](../../rust/hybrid_binary/PyTask.md#new)`
- Rust→Python: `[hybrid_binary.Task](../../python/hybrid_binary/Task.md)`

**Result:** All 129 tests passing. Bi-directional links work correctly with per-class file structure.