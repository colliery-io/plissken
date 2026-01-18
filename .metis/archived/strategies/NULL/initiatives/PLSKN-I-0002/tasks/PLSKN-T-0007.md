---
id: tera-template-engine
level: task
title: "Tera Template Engine"
short_code: "PLSKN-T-0007"
created_at: 2026-01-14T16:56:03.925833+00:00
updated_at: 2026-01-14T20:56:13.269087+00:00
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

# Tera Template Engine

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[PLSKN-I-0002]]

## Objective

Set up Tera templating engine with base templates for rendering documentation. Inject theme adapter values into template context.

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [x] Add `tera` dependency to plissken-core or plissken-cli (already in workspace)
- [x] Create `Renderer` struct that holds Tera instance and ThemeAdapter
- [x] Load templates from embedded strings (not external files for MVP)
- [x] Inject theme adapter values into Tera context (`theme.code_bg`, etc.)
- [x] Base template for module page with standard structure
- [x] Partial templates for: signature, badge, code block
- [x] Test rendering a simple template with theme values

## Implementation Notes

### Technical Approach
Used Tera's embedded template support to bundle templates as const strings. Theme values are injected as a serializable `ThemeContext` struct that templates access via `{{ theme.* }}` syntax.

### Files
- `crates/plissken-core/src/render/renderer.rs` - Renderer + templates
- `crates/plissken-core/src/render/mod.rs` - Module exports
- `crates/plissken-core/src/lib.rs` - Public API exports

## Status Updates **[REQUIRED]**

### 2026-01-14 - Implementation Complete

Created the `Renderer` struct with Tera templating and theme integration.

**Files created:**
- `crates/plissken-core/src/render/renderer.rs` - Renderer struct and embedded templates

**Files modified:**
- `crates/plissken-core/src/render/mod.rs` - Added renderer module export
- `crates/plissken-core/src/lib.rs` - Added Renderer to public exports

**Implementation details:**

1. **Renderer struct** - Holds Tera instance + boxed ThemeAdapter
   - `new(template: Option<&str>)` - Creates renderer with specified theme
   - `base_context()` - Returns Tera Context with theme values injected
   - `theme_name()` - Returns the active theme adapter name

2. **Rendering methods:**
   - `render_badge(text, color_type)` - Inline badge with color variants (blue/green/yellow/red/gray)
   - `render_code_block(code, language, caption)` - Styled code block with optional caption
   - `render_signature(name, params, return_type, is_async)` - Function signature block
   - `render_module(name, description, functions, classes)` - Full module page
   - `render_template(name, context)` - Arbitrary template rendering

3. **Embedded templates:**
   - `partials/badge.html` - Inline badge/tag styling
   - `partials/code_block.html` - Code block with border and background
   - `partials/signature.html` - Function signature with left border accent
   - `module.html` - Module page structure (markdown with sections)

4. **Theme context injection:**
   - Templates access theme via `{{ theme.code_bg }}`, `{{ theme.primary }}`, etc.
   - ThemeContext struct serializes all 6 color values + theme name

**Tests:** 11 new unit tests + 2 new doc-tests. Total: 78 tests + 4 doc-tests, all passing.