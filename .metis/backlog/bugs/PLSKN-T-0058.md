---
id: investigate-code-block-indentation
level: task
title: "Investigate code block indentation mangling in docstring examples"
short_code: "PLSKN-T-0058"
created_at: 2026-01-18T15:56:21.234607+00:00
updated_at: 2026-01-18T19:32:29.302962+00:00
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

# Investigate code block indentation mangling in docstring examples

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[Parent Initiative]]

## Objective

Investigate whether code block indentation is being incorrectly stripped from examples within docstrings.

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

### Impact Assessment
- **Affected Users**: All users with indented code examples in docstrings
- **Reproduction Steps**: 
  1. Create docstring with code block containing nested indentation
  2. Run plissken render
  3. Check if internal code indentation is preserved
- **Expected vs Actual**: Code indentation should be preserved; may be getting stripped

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

- [x] Investigate docstring parsing/normalization for over-aggressive dedenting
- [x] Test code blocks with various indentation patterns
- [x] Fix if issue confirmed, or document if working correctly
- [x] Add regression tests for code block indentation

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

### 2026-01-18: Fixed as part of PLSKN-T-0057

This issue was resolved while implementing PLSKN-T-0057 (Render markdown formatting in docstrings).

**Root Cause**: Examples from Rust docstrings preserved their 4-space leading indentation, but only the first line was being trimmed by `example.trim()`, leaving subsequent lines with inconsistent indentation.

**Fix Applied**: Added `dedent_code()` function in `docstring_renderer.rs` that:
- Finds the minimum indent of non-empty lines
- Removes that common leading whitespace from all lines
- Called from `render_examples()` before processing

**Tests Added**:
- `test_dedent_code` - Unit tests for the dedentation function
- `test_render_examples_with_indented_code_fence` - Integration test

**Verification**: Generated docs show properly dedented code blocks with internal indentation preserved correctly.