---
id: docstring-parser
level: task
title: "Docstring Parser"
short_code: "PLSKN-T-0004"
created_at: 2026-01-14T03:32:09.507546+00:00
updated_at: 2026-01-14T14:23:02.738886+00:00
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

# Docstring Parser

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[PLSKN-I-0001]]

## Objective

Parse docstrings into structured `ParsedDocstring` (Google/NumPy style).

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

- [x] Extract summary (first line/paragraph)
- [x] Parse Args/Parameters section into `ParamDoc`
- [x] Parse Returns section into `ReturnDoc`
- [x] Parse Raises/Exceptions section into `RaisesDoc`
- [x] Extract Examples sections
- [x] Handle both Google and NumPy styles
- [x] Preserve original docstring text

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
- Regex or simple state machine parser
- Don't need full RST/Markdown parsing
- Focus on parameter/return documentation for tables

### Dependencies
{Other tasks or systems this depends on}

### Risk Considerations
{Technical risks and mitigation strategies}

## Status Updates **[REQUIRED]**

### Session 1 - 2026-01-13
**Implementation Complete**

Implemented docstring parser in `crates/plissken-core/src/docstring.rs` (~680 lines).

**Core Function:**
- `parse_docstring(docstring: &str) -> ParsedDocstring`

**Features:**
- **Style Detection**: Automatically detects Google, NumPy, or plain docstrings
- **Summary Extraction**: First line/paragraph before sections
- **Description Extraction**: Additional paragraphs before sections
- **Args/Parameters**: Parses `name (type): description` format
- **Returns**: Extracts type and description
- **Raises**: Parses exception types and descriptions
- **Examples**: Captures code examples
- **Google Style**: Handles `Args:`, `Returns:`, `Raises:`, `Example:` sections
- **NumPy Style**: Handles underlined section headers (`Parameters\n----------`)

**Tests Added (16 total, all passing):**
1. `test_parse_empty` - Empty docstring handling
2. `test_parse_summary_only` - Simple one-line docstring
3. `test_parse_summary_and_description` - Multi-paragraph intro
4. `test_parse_google_args` - Args section with types
5. `test_parse_google_returns` - Returns without type
6. `test_parse_google_returns_with_type` - Returns with type annotation
7. `test_parse_google_raises` - Multi-line Raises section
8. `test_parse_google_examples` - Example code blocks
9. `test_parse_google_full` - Complete Google docstring
10. `test_parse_numpy_style` - NumPy format with underlines
11. `test_parse_rust_docstring` - Rust doc comment format
12. `test_detect_google_style` - Style detection for Google
13. `test_detect_numpy_style` - Style detection for NumPy
14. `test_detect_plain_style` - Style detection for plain
15. `test_parse_scheduler_docstring` - Real fixture test
16. `test_parse_fixture_method_docstring` - Complex fixture test

**All 46 project tests passing.**

### Session 2 - 2026-01-14
**Added Rust Doc Comment Parser**

Added `parse_rust_doc()` for conventional Rust markdown-style doc comments.

**Supported Sections:**
- `# Arguments` / `# Parameters` - function parameters (list format)
- `# Returns` - return value documentation
- `# Errors` - error conditions (maps to raises with type "Error")
- `# Panics` - panic conditions (maps to raises with type "Panic")
- `# Safety` - safety requirements (appended to description)
- `# Examples` - code examples with fence block support

**Parameter Formats:**
- `* \`name\` - Description` (backtick-quoted with dash separator)
- `* name - Description` (plain with dash separator)
- `- \`name\`: Description` (backtick with colon separator)

**Additional Tests (14 new):**
1. `test_parse_rust_doc_empty`
2. `test_parse_rust_doc_summary_only`
3. `test_parse_rust_doc_summary_and_description`
4. `test_parse_rust_doc_arguments`
5. `test_parse_rust_doc_arguments_backticks`
6. `test_parse_rust_doc_returns`
7. `test_parse_rust_doc_errors`
8. `test_parse_rust_doc_errors_with_types`
9. `test_parse_rust_doc_panics`
10. `test_parse_rust_doc_examples`
11. `test_parse_rust_doc_safety`
12. `test_parse_rust_doc_full`
13. `test_parse_rust_doc_no_sections`
14. `test_parse_markdown_header`

**All 60 project tests passing.**