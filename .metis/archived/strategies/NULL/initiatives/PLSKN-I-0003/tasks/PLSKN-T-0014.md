---
id: revert-python-style-signatures-in
level: task
title: "Revert Python-style Signatures in Rust Docs"
short_code: "PLSKN-T-0014"
created_at: 2026-01-15T02:11:49.619163+00:00
updated_at: 2026-01-15T02:15:43.766049+00:00
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

# Revert Python-style Signatures in Rust Docs

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[PLSKN-I-0003]]

## Objective

Revert the earlier change that renders Python-style signatures for bindings in Rust docs. Rust docs should always show Rust signatures.

**File:** `crates/plissken-core/src/render/module_renderer.rs`

Remove the `is_binding` conditional in `render_rust_function_with_context()` that calls `rust_to_python_signature()`. Keep the `Binding` badge.

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

- [ ] Rust docs show Rust-style signatures for all functions
- [ ] `Binding` badge still appears for PyO3-exposed items
- [ ] Rust source code still shown in collapsible section

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

## Status Updates

### 2026-01-15: Completed

**Changes made to `crates/plissken-core/src/render/module_renderer.rs`:**

1. Removed `PythonSignature` helper struct (lines 17-21)
2. Modified `render_rust_function_with_context()` to always use Rust signatures instead of conditional Python-style
3. Removed unused `rust_to_python_signature()` function
4. Removed unused `rust_type_to_python()` function  
5. Updated struct field rendering to always show Rust types

**Verification:**
- Rust docs now show Rust signatures: `fn task (runner : Py < PyRunner > , kwargs : Option < & Bound < '_ , PyDict > >) -> PyResult < TaskDecorator >`
- `Binding` badge still appears for PyO3-exposed items
- Rust source still shown in collapsible `<details>` sections

All acceptance criteria met.