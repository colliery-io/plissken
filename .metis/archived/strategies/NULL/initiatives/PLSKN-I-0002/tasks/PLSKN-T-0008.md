---
id: badge-and-inline-styling
level: task
title: "Badge and Inline Styling"
short_code: "PLSKN-T-0008"
created_at: 2026-01-14T16:56:03.965465+00:00
updated_at: 2026-01-14T20:56:13.308001+00:00
parent: PLSKN-I-0002
blocked_by: []
archived: true

tags:
  - "#task"
  - "#phase/completed"


exit_criteria_met: false
strategy_id: NULL
initiative_id: PLSKN-I-0002
---

# Badge and Inline Styling

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[PLSKN-I-0002]]

## Objective

Create badge rendering with inline styles that use theme adapter CSS variables. Badges indicate async, unsafe, deprecated, visibility, and source type.

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [x] Badge template partial with inline style using theme variables
- [x] Badge types: `async`, `unsafe`, `deprecated`
- [x] Visibility badges: `pub`, `pub(crate)`, `private`
- [x] Source type badges: 🐍 Python, 🦀 Rust, 🐍↔🦀 Binding
- [x] Badges include `class="plissken-badge"` for optional user override
- [x] Test badge output contains correct inline CSS variable references

## Implementation Notes

### Technical Approach
Extended the existing badge template with semantic badge types and added convenience methods. The badge template now includes `class="plissken-badge plissken-badge-{type}"` for user CSS overrides while still providing inline styles.

### Badge API
- `render_badge(text, color_type, badge_type)` - Low-level method (now takes 3 args)
- `badge_async()` - Blue badge for async functions
- `badge_unsafe()` - Red badge for unsafe code
- `badge_deprecated()` - Yellow badge for deprecated items
- `badge_visibility(vis)` - Green/orange/gray based on pub/pub(crate)/private
- `badge_source(type)` - Emoji badges for Python/Rust/Binding sources

### Color Palette
| Color | Hex | Usage |
|-------|-----|-------|
| blue | theme.primary | async, Python source |
| green | #4caf50 | pub visibility |
| yellow | #ff9800 | deprecated |
| red | #f44336 | unsafe |
| purple | #9c27b0 | binding/pyo3 |
| orange | #ff5722 | Rust source, pub(crate) |
| gray | theme.muted | private, unknown |

## Status Updates **[REQUIRED]**

### 2026-01-14 - Implementation Complete

Extended the badge system with semantic badge types and CSS override support.

**Changes to `crates/plissken-core/src/render/renderer.rs`:**

1. **Updated badge template** - Added `class="plissken-badge plissken-badge-{{ badge_type }}"` and new colors (purple, orange)

2. **Updated `render_badge()`** - Now takes 3 arguments: text, color_type, badge_type

3. **Added semantic badge methods:**
   - `badge_async()` - Blue "async" badge
   - `badge_unsafe()` - Red "unsafe" badge  
   - `badge_deprecated()` - Yellow "deprecated" badge
   - `badge_visibility(vis)` - pub (green), pub(crate) (orange), private (gray)
   - `badge_source(type)` - 🐍 Python (blue), 🦀 Rust (orange), 🐍↔🦀 Binding (purple)

4. **Updated doc examples** to use new API

**Tests added:** 6 new tests for semantic badges + updated existing tests
- `test_badge_async`
- `test_badge_unsafe`
- `test_badge_deprecated`
- `test_badge_visibility`
- `test_badge_source`
- `test_badge_has_override_class`

**Total tests:** 84 unit tests + 4 doc-tests, all passing.