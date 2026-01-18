---
id: theme-adapter-system
level: task
title: "Theme Adapter System"
short_code: "PLSKN-T-0006"
created_at: 2026-01-14T16:56:03.878081+00:00
updated_at: 2026-01-14T18:57:39.059722+00:00
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

# Theme Adapter System

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[PLSKN-I-0002]]

## Objective

Create the `ThemeAdapter` trait and implementations that map semantic style names to SSG-specific CSS variables. This is the foundation for zero-CSS-file theming.

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [x] `ThemeAdapter` trait with methods: `code_bg`, `code_fg`, `primary`, `accent`, `muted`, `border`
- [x] `MkDocsMaterial` impl using `var(--md-*)` variables
- [x] `MdBook` impl using `var(--*)` variables  
- [x] `Minimal` impl with hardcoded fallback colors
- [x] Factory function to get adapter from config template name
- [x] Unit tests verifying each adapter returns correct CSS variable strings

## Implementation Notes

### Technical Approach
Created a new `render` module in plissken-core with a `ThemeAdapter` trait that provides semantic color mappings. Each SSG implementation returns its native CSS variable syntax, allowing generated HTML to inherit the SSG's theming automatically.

### Files
- `crates/plissken-core/src/render/mod.rs`
- `crates/plissken-core/src/render/theme.rs`
- Updated `crates/plissken-core/src/lib.rs` to export render module

## Status Updates **[REQUIRED]**

### 2026-01-14 - Implementation Complete

Created the render module with theme adapter system:

**Files created:**
- `crates/plissken-core/src/render/mod.rs` - Module exports
- `crates/plissken-core/src/render/theme.rs` - ThemeAdapter trait and implementations

**Implementation details:**
- `ThemeAdapter` trait with `Send + Sync` bounds for thread safety
- `name()` method added for debugging/logging
- `MkDocsMaterial` returns `var(--md-*)` CSS variables
- `MdBook` returns `var(--*)` CSS variables
- `Minimal` returns hardcoded hex colors (`#f5f5f5`, `#333333`, etc.)
- `get_theme_adapter()` factory function handles template name variations (case-insensitive, supports hyphens/underscores)

**Tests:** 7 new unit tests + 2 doc-tests, all passing. Total test count: 67 + 2 doc-tests.