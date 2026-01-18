---
id: fix-ssg-output-format-to-match
level: task
title: "Fix SSG output format to match expected binding structure"
short_code: "PLSKN-T-0039"
created_at: 2026-01-17T16:54:22.330480+00:00
updated_at: 2026-01-17T17:09:47.826379+00:00
parent: 
blocked_by: []
archived: true

tags:
  - "#task"
  - "#bug"
  - "#phase/completed"


exit_criteria_met: false
strategy_id: NULL
initiative_id: NULL
---

# Fix SSG output format to match expected binding structure

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[Parent Initiative]]

## Objective **[COMPLETED]**

Fixed the SSG output format so that plissken generates single-page-per-module documentation with all content (classes, functions, methods) rendered inline on the module page.

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

## Acceptance Criteria **[REQUIRED]**

- [ ] {Specific, testable requirement 1}
- [ ] {Specific, testable requirement 2}
- [ ] {Specific, testable requirement 3}

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

## Status Updates **[COMPLETED]**

### Changes Made

1. **Modified module_renderer.rs**:
   - Rewrote `render_python_module()` to inline all content (classes, methods, functions)
   - Rewrote `render_rust_module()` to inline all content (structs, enums, methods, functions)
   - Added new inline rendering methods: `render_python_class_inline()`, `render_python_function_inline()`, `render_rust_struct_inline()`, `render_rust_enum_inline()`, `render_rust_function_inline()`
   - Fixed badge placement to not interfere with heading anchor generation

2. **Modified layout.rs**:
   - Added `python_module_page()` for single-page Python module paths
   - Added `rust_module_page()` for single-page Rust module paths

3. **Modified crossref.rs**:
   - Updated all cross-reference link generation to use correct anchors for inline format
   - Python class anchors: `#class-{classname}`
   - Python function anchors: `#{funcname}`
   - Rust struct anchors: `#class-{classname}` (uses Python class name)
   - Rust function anchors: `#fn-{funcname}` (includes `fn-` prefix)

4. **Test Results**:
   - All unit tests pass
   - MkDocs Material functional test passes
   - mdBook functional test passes
   - Cross-reference link warnings resolved