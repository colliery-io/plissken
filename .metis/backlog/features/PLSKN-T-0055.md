---
id: use-namespace-paths-for
level: task
title: "Use namespace paths for documentation headers"
short_code: "PLSKN-T-0055"
created_at: 2026-01-18T15:56:20.926047+00:00
updated_at: 2026-01-18T16:30:56.277561+00:00
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

# Use namespace paths for documentation headers

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[Parent Initiative]]

## Objective

Documentation headers should display the full namespace path for the code being documented, not just the item name.

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
- **User Value**: Users need full namespace context to understand where items live in the module hierarchy
- **Effort Estimate**: M

### Technical Debt Impact **[CONDITIONAL: Tech Debt]**
- **Current Problems**: {What's difficult/slow/buggy now}
- **Benefits of Fixing**: {What improves after refactoring}
- **Risk Assessment**: {Risks of not addressing this}

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [x] Python docs use dot notation (e.g., `package.submodule.ClassName`)
- [x] Rust docs use `::` delimiters (e.g., `crate::module::StructName`)
- [x] Function headers follow language conventions (verified: Python uses dots, Rust uses ::)

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

### Session 1 - 2026-01-18

**Completed:**
- Investigated current header rendering in `module_renderer.rs`
- Updated Python module headers to use full `module.path` instead of just last segment
- Updated Python class headers: `class module_path.ClassName`
- Updated Python function headers: `module_path.function_name`
- Updated Rust module headers to use full `module.path`
- Updated Rust struct headers: `struct module_path::StructName`
- Updated Rust enum headers: `enum module_path::EnumName`
- Updated Rust function headers: `fn module_path::function_name`
- Updated both inline rendering and page-per-item rendering functions
- Updated 14 snapshot tests with new namespace path format
- Updated 5 assertion-based unit tests

**Verified output:**
- Python: `class helpers.TaskBuilder`
- Rust: `struct hybrid_binary::internal::Task`

**Files modified:**
- `crates/plissken-core/src/render/module_renderer.rs`
- 14 snapshot files in `crates/plissken-core/src/render/snapshots/`

**User feedback refinement:**
User noted that type prefixes (class, struct, enum, fn) are redundant since the section headings already provide that context. Removed all type prefixes from headers.

**Final output format:**
- Python classes: `hybrid_binary.Task` (dot notation, no "class" prefix)
- Python functions: `hybrid_binary.get_version` (dot notation)
- Rust structs: `hybrid_binary::internal::Task` (double-colon notation, no "struct" prefix)
- Rust enums: `hybrid_binary::internal::ExecutorError` (no "enum" prefix)
- Rust functions: `hybrid_binary::hybrid_binary` (no "fn" prefix)

**All 249 tests passing.**