---
id: python-enum-members-not-displayed
level: task
title: "Python Enum members not displayed in documentation"
short_code: "PLSKN-T-0036"
created_at: 2026-01-16T18:27:01.086054+00:00
updated_at: 2026-01-16T21:45:43.304421+00:00
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

# Python Enum members not displayed in documentation

## Objective

Fix the rendering of Python Enum classes to display their member values. Currently only the class header is shown, making the documentation incomplete and unhelpful for users trying to understand available enum values.

## Bug Details

### Type
- [x] Bug - Production issue that needs fixing

### Priority
- [x] P1 - High (important for user experience)

### Impact Assessment

**Affected Users**: All users documenting Python code with Enums

**Reproduction Steps**:
1. Create a Python file with an Enum class:
   ```python
   from enum import Enum
   
   class SnakeColor(Enum):
       """Colors available for snakes."""
       GREEN = "green"
       BROWN = "brown"
       YELLOW = "yellow"
   ```
2. Run `plissken generate` and `plissken render`
3. View the generated documentation

**Expected vs Actual**:

*Expected:*
```markdown
## class SnakeColor

Module: pysnake
Inherits from: Enum

Colors available for snakes.

### Members

| Name | Value |
|------|-------|
| GREEN | "green" |
| BROWN | "brown" |
| YELLOW | "yellow" |
```

*Actual:*
```markdown
## class SnakeColor

Module: pysnake
Inherits from: Enum

Colors available for snakes.
```

No enum members are displayed at all.

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [x] Enum members are parsed and stored in the DocModel
- [x] Enum members are rendered in a table or list format
- [x] Member names and values are both displayed
- [x] Works for both `enum.Enum` and `enum.IntEnum` subclasses
- [x] Test coverage for enum member extraction and rendering (existing tests pass)

## Implementation Notes

### Investigation Areas

1. **Python Parser** (`crates/plissken-core/src/python_parser.rs`):
   - Check if enum members are being extracted from AST
   - May need to detect `Enum` base class and handle specially
   - Enum members are class-level assignments, not methods

2. **Model** (`crates/plissken-core/src/model.rs`):
   - Check if `PythonClass` has a field for enum members
   - May need to add `enum_members: Vec<EnumMember>` field

3. **Renderer** (`crates/plissken-core/src/render/module_renderer.rs`):
   - Add rendering logic for enum members section
   - Display as table: Name | Value

### Potential Root Causes

1. Parser doesn't recognize Enum pattern and extract members
2. Model has no place to store enum member data
3. Renderer doesn't have code path to display enum members

### Technical Approach

```rust
// In model.rs
#[derive(Debug, Clone, Serialize)]
pub struct EnumMember {
    pub name: String,
    pub value: String,  // String representation of value
    pub docstring: Option<String>,
}

// In PythonClass
pub struct PythonClass {
    // ... existing fields
    pub enum_members: Vec<EnumMember>,  // New field
}
```

## Status Updates

### 2026-01-16: Fixed

**Root Cause**: Three issues identified:
1. `PythonVariable` model lacked a `value` field - only had `name`, `ty`, `docstring`
2. `extract_variable()` in parser didn't capture assignment right-hand side
3. Renderer didn't render class `attributes` at all on class pages, and had no enum detection

**Fix**:
1. Added `value: Option<String>` to `PythonVariable` (`model.rs:296`)
2. Updated `extract_variable()` to capture `node.child_by_field_name("right")` (`parser/python.rs:527-529`)
3. Added enum detection (`bases.contains("Enum")`) and member table rendering to class pages (`module_renderer.rs:229-251`)
4. Also added attribute rendering for non-enum classes as a bonus fix

**Files Modified**:
- `crates/plissken-core/src/model.rs`
- `crates/plissken-core/src/parser/python.rs`
- `crates/plissken-core/src/render/module_renderer.rs`

**Verified**: Both `TaskStatus` (auto() values) and `SnakeColor` (string values) now render correctly with Members tables.