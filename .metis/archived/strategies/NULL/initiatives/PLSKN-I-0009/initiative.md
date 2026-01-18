---
id: rendering-system-refactor
level: initiative
title: "Rendering System Refactor"
short_code: "PLSKN-I-0009"
created_at: 2026-01-16T17:49:13.625147+00:00
updated_at: 2026-01-17T02:07:34.875186+00:00
parent: PLSKN-V-0001
blocked_by: []
archived: true

tags:
  - "#initiative"
  - "#phase/completed"


exit_criteria_met: false
estimated_complexity: L
strategy_id: NULL
initiative_id: rendering-system-refactor
---

# Rendering System Refactor Initiative

## Context

The rendering system has significant technical debt and lacks extensibility:

1. **Templates hardcoded as strings**: 600+ character badge template embedded in Rust code
2. **No user customization**: Cannot override templates, colors, or structure without recompiling
3. **ModuleRenderer does too much**: 1,916 LOC mixing page layout, content formatting, cross-ref linking
4. **SSG detection is ad-hoc**: `template == "mdbook"` string comparison instead of proper abstraction
5. **Badge colors inconsistent**: Some from theme, some hardcoded hex values
6. **Navigation generation duplicated**: Same logic in `_nav.yml` and `SUMMARY.md` generators
7. **Theme system limited**: Only 6 semantic colors, no dark mode, no typography control

## Goals & Non-Goals

**Goals:**
- External template files that users can override
- Proper SSG abstraction with `SSGAdapter` trait
- Split ModuleRenderer into focused components
- Unify navigation generation for all SSGs
- Consistent badge theming (no hardcoded colors)

**Non-Goals:**
- Plugin system for arbitrary extensions (too complex)
- Support for every possible SSG (focus on MkDocs + mdBook)
- Breaking changes to output structure

## Detailed Design

### External Templates
```
.plissken/
  templates/
    badge.html        # Override badge appearance
    module.html       # Override module page layout
    nav_mkdocs.jinja2 # Override navigation
```

Load order: user override → bundled default

### SSG Adapter Trait
```rust
pub trait SSGAdapter: Send + Sync {
    fn name(&self) -> &str;
    fn content_dir(&self) -> &str;  // "src/" for mdBook, "." for MkDocs
    fn nav_filename(&self) -> &str;
    fn generate_nav(&self, modules: &[Module]) -> String;
    fn generate_config(&self, title: &str) -> String;
}
```

### Split ModuleRenderer
```
render/
  module/
    layout.rs      # File paths and hierarchy (PageLayout)
    content.rs     # Module/class/function content (ContentRenderer)
    crossref.rs    # Link computation (CrossRefLinker)
  nav/
    generator.rs   # Unified navigation generation
```

### Extended Theme System
```rust
pub trait ThemeAdapter {
    fn color(&self, role: ColorRole) -> &str;
    fn dark_mode(&self) -> Option<&Self>;
    fn typography(&self) -> TypographyScale;
}

pub enum ColorRole {
    Primary, Accent, Muted, Border,
    CodeBg, CodeFg,
    Success, Warning, Error,  // New!
    BadgeAsync, BadgeUnsafe, BadgeDeprecated,  // New!
}
```

## Implementation Plan

**Phase 1: External Templates (6-8 hours)**
- Create template loading from filesystem
- Add fallback to bundled defaults
- Test with custom template overrides

**Phase 2: SSG Adapter (4-6 hours)**
- Define `SSGAdapter` trait
- Implement for MkDocsMaterial and MdBook
- Replace string matching with trait dispatch

**Phase 3: Split ModuleRenderer (8-12 hours)**
- Extract PageLayout component
- Extract ContentRenderer component
- Extract CrossRefLinker component
- Unified navigation generator

**Phase 4: Theme Expansion (4-6 hours)**
- Add new color roles
- Remove hardcoded badge colors
- Add dark mode support structure