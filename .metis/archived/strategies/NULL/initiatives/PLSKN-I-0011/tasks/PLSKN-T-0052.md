---
id: unify-enum-rendering-format-for
level: task
title: "Unify enum rendering format for Python and Rust"
short_code: "PLSKN-T-0052"
created_at: 2026-01-17T19:48:47.343005+00:00
updated_at: 2026-01-18T01:25:00.445197+00:00
parent: PLSKN-I-0011
blocked_by: []
archived: true

tags:
  - "#task"
  - "#phase/completed"


exit_criteria_met: false
strategy_id: NULL
initiative_id: PLSKN-I-0011
---

# Unify enum rendering format for Python and Rust

## Parent Initiative

[[PLSKN-I-0011]]

## Objective

Unify the rendering format for enums between Python and Rust for consistent output.

## Problem

Python and Rust enums are rendered differently:

**Python Enum** (`render_python_class_inline`, lines 178-186):
```markdown
#### Members

| Name | Value |
|------|-------|
| RED | 1 |
| GREEN | 2 |
```

**Rust Enum** (`render_rust_enum_inline`, lines 914-924):
```markdown
#### Variants

- **`Red`**
- **`Green`** - A green color
```

Inconsistencies:
- Section name: "Members" vs "Variants"
- Format: Table vs bullet list
- Value display: Shows values vs shows docs

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [x] Decide on unified format (table or list) - **List format**
- [x] Decide on unified section name ("Members", "Variants", or "Values") - **"Variants"**
- [x] Update Python enum rendering
- [x] Update Rust enum rendering (no changes needed, already used list)
- [x] Both use same markdown structure
- [x] All existing tests pass
- [x] Update any affected snapshots

## Implementation Notes

### Technical Approach

**Option A: Use tables for both**
```markdown
#### Variants

| Name | Value/Fields | Description |
|------|--------------|-------------|
| Red | - | A red color |
| Green | - | A green color |
| Rgb | (u8, u8, u8) | RGB color values |
```

**Option B: Use lists for both**
```markdown
#### Variants

- **`Red`** - A red color
- **`Green`** - A green color  
- **`Rgb(u8, u8, u8)`** - RGB color values
```

**Recommendation**: Use **Option B (lists)** because:
- Rust enums can have complex variants with fields
- Lists handle variable-length content better
- More readable for enums with documentation

### Changes Required

1. Update `render_python_class_inline()` to use list format for enums
2. Rename "Members" to "Variants" for consistency
3. Add variant documentation if available
4. Update tests

## Status Updates

### Implementation Complete

**Decision:** Used list format (Option B) with "Variants" section name

**Changes:**
- `crates/plissken-core/src/render/module_renderer.rs` (lines 230-245)
  - Changed section name from "Members" to "Variants"
  - Changed table format to bullet list format
  - Format: `- **\`NAME\`** = \`value\` - description`

**Before (Python enum):**
```markdown
#### Members

| Name | Value |
|------|-------|
| `PENDING` | `"pending"` |
```

**After (Python enum):**
```markdown
#### Variants

- **`PENDING`** = `"pending"`
```

**Rust enum format (unchanged):**
```markdown
#### Variants

- **`Ok`** - Operation succeeded with value
```

**Snapshots updated:**
- `python_enum_class.snap`

**Tests:**
- All 245 unit tests pass
- All 4 integration tests pass
- All 16 doc tests pass (6 ignored)