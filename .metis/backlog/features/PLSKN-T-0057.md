---
id: render-markdown-formatting-in
level: task
title: "Render markdown formatting in docstrings"
short_code: "PLSKN-T-0057"
created_at: 2026-01-18T15:56:21.139245+00:00
updated_at: 2026-01-18T19:29:57.618531+00:00
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

# Render markdown formatting in docstrings

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[Parent Initiative]]

## Objective

Ensure markdown formatting in docstrings (headers, lists, code blocks, links, emphasis) is properly rendered in output documentation.

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
- **User Value**: Top-level module docs often have rich formatting that should render properly
- **Effort Estimate**: S

### Technical Debt Impact **[CONDITIONAL: Tech Debt]**
- **Current Problems**: {What's difficult/slow/buggy now}
- **Benefits of Fixing**: {What improves after refactoring}
- **Risk Assessment**: {Risks of not addressing this}

## Acceptance Criteria

## Acceptance Criteria

- [x] Markdown in docstrings renders correctly in output
- [x] Code blocks with syntax highlighting work
- [x] Links, lists, and emphasis are preserved
- [x] Top-level module/package docs look well-formed

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

### 2026-01-18: Investigation and Fix Completed

**Root Cause Identified**: Two bugs in docstring example handling:

1. **Code fence splitting**: `parse_google_examples()` in `docstring.rs` was splitting examples at empty lines even when inside code fences (```). This caused multi-line code examples with blank lines to be split into multiple malformed examples.

2. **Indentation preservation**: Examples from Rust docstrings had leading indentation (4 spaces) which was preserved but caused inconsistent formatting when the first line was trimmed but subsequent lines weren't.

**Fixes Applied**:

1. **Fixed `parse_google_examples()`** (`crates/plissken-core/src/docstring.rs:527-563`):
   - Added `in_code_block` tracking variable
   - Track code fence state by toggling on `trimmed.starts_with("```")`
   - Only split examples on empty lines when NOT inside a code block
   - This mirrors the logic already present in `parse_rust_examples()`

2. **Added `dedent_code()` function** (`crates/plissken-core/src/render/docstring_renderer.rs:182-207`):
   - Finds minimum indent of non-empty lines
   - Removes that common indent from all lines
   - Called from `render_examples()` before processing

**Files Modified**:
- `crates/plissken-core/src/docstring.rs` - Added code fence tracking to `parse_google_examples()`
- `crates/plissken-core/src/render/docstring_renderer.rs` - Added `dedent_code()` helper, updated `render_examples()`

**Tests Added**:
- `test_parse_google_examples_with_code_fence` - Verifies code fences preserve example unity
- `test_dedent_code` - Unit tests for dedentation function
- `test_render_examples_with_indented_code_fence` - Integration test for full pipeline

**Verification**: Generated docs for `separate_bindings` fixture - examples now render correctly with proper formatting and no nested/broken code fences.

All 261 tests pass.

### 2026-01-18: Ready for Review

All acceptance criteria verified:
- ✅ Markdown in docstrings renders correctly - code blocks, emphasis, headers all work
- ✅ Code blocks with syntax highlighting - `python` language tag properly applied
- ✅ Links, lists, and emphasis preserved - cross-reference links working
- ✅ Top-level module/package docs well-formed - clean structure verified

Implementation complete and tested.