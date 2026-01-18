---
id: expand-theme-system-with-badge
level: task
title: "Expand theme system with badge colors and dark mode support"
short_code: "PLSKN-T-0035"
created_at: 2026-01-16T18:01:47.593798+00:00
updated_at: 2026-01-17T02:07:25.934030+00:00
parent: PLSKN-I-0009
blocked_by: []
archived: true

tags:
  - "#task"
  - "#phase/completed"


exit_criteria_met: false
strategy_id: NULL
initiative_id: PLSKN-I-0009
---

# Expand theme system with badge colors and dark mode support

## Parent Initiative

[[PLSKN-I-0009]] Rendering System Refactor

## Objective

Extend the theme system to include badge-specific colors and dark mode variants, eliminating hardcoded hex values scattered throughout the codebase. All colors should flow through the theme system for consistency.

## Current Problem

Badge colors are currently hardcoded:
```rust
// Scattered throughout module_renderer.rs
let async_color = "#4CAF50";   // Hardcoded green
let unsafe_color = "#FF5722";  // Hardcoded orange
let deprecated_color = "#9E9E9E"; // Hardcoded gray
```

The theme system only has 6 semantic colors and no dark mode support.

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [x] Add badge-specific color roles to theme trait
- [x] Add semantic color roles (success, warning, error)
- [x] Remove hardcoded hex values from badge template
- [ ] Add `dark_mode()` method to theme trait (deferred - CSS vars handle this)
- [x] Update MkDocs Material theme with CSS variable-based badge colors
- [x] Badges use theme colors exclusively

## Extended ColorRole Enum

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorRole {
    // Existing
    Primary,
    Accent,
    Muted,
    Border,
    CodeBg,
    CodeFg,
    
    // Semantic (new)
    Success,
    Warning,
    Error,
    Info,
    
    // Badge-specific (new)
    BadgeAsync,
    BadgeUnsafe,
    BadgeDeprecated,
    BadgePyo3,
    BadgeConstGeneric,
    BadgeClassMethod,
    BadgeStaticMethod,
    BadgeProperty,
}
```

## Extended Theme Trait

```rust
pub trait Theme: Send + Sync {
    /// Get color for a role.
    fn color(&self, role: ColorRole) -> &str;
    
    /// Get dark mode variant (if available).
    fn dark_mode(&self) -> Option<&dyn Theme> {
        None
    }
    
    /// Theme name for CSS class generation.
    fn name(&self) -> &str;
    
    /// Whether this is a dark theme.
    fn is_dark(&self) -> bool {
        false
    }
}
```

## MkDocs Material Theme Colors

```rust
impl Theme for MkDocsMaterialTheme {
    fn color(&self, role: ColorRole) -> &str {
        match role {
            // Existing
            ColorRole::Primary => "#7C4DFF",
            ColorRole::Accent => "#448AFF",
            ColorRole::Muted => "#757575",
            ColorRole::Border => "#E0E0E0",
            ColorRole::CodeBg => "#F5F5F5",
            ColorRole::CodeFg => "#37474F",
            
            // Semantic
            ColorRole::Success => "#4CAF50",
            ColorRole::Warning => "#FF9800",
            ColorRole::Error => "#F44336",
            ColorRole::Info => "#2196F3",
            
            // Badge colors (Material Design palette)
            ColorRole::BadgeAsync => "#4CAF50",      // Green
            ColorRole::BadgeUnsafe => "#FF5722",     // Deep Orange
            ColorRole::BadgeDeprecated => "#9E9E9E", // Gray
            ColorRole::BadgePyo3 => "#306998",       // Python blue
            ColorRole::BadgeConstGeneric => "#7C4DFF", // Purple
            ColorRole::BadgeClassMethod => "#2196F3", // Blue
            ColorRole::BadgeStaticMethod => "#00BCD4", // Cyan
            ColorRole::BadgeProperty => "#8BC34A",    // Light Green
        }
    }
}
```

## Implementation Notes

### Files to Modify

1. **`crates/plissken-core/src/render/theme.rs`** - Extend trait and enum

2. **`crates/plissken-core/src/render/themes/mkdocs_material.rs`** - Full palette:
   ```rust
   pub struct MkDocsMaterialTheme {
       dark: bool,
   }
   
   impl MkDocsMaterialTheme {
       pub fn light() -> Self { Self { dark: false } }
       pub fn dark() -> Self { Self { dark: true } }
   }
   
   impl Theme for MkDocsMaterialTheme {
       fn color(&self, role: ColorRole) -> &str {
           if self.dark {
               self.dark_color(role)
           } else {
               self.light_color(role)
           }
       }
       
       fn dark_mode(&self) -> Option<&dyn Theme> {
           if self.dark { None } else { Some(&Self::dark()) }
       }
   }
   ```

3. **`crates/plissken-core/src/render/components/badge.rs`** - Use theme:
   ```rust
   impl<'a> BadgeRenderer<'a> {
       pub fn render(&self, badge_type: BadgeType) -> String {
           let color_role = match badge_type {
               BadgeType::Async => ColorRole::BadgeAsync,
               BadgeType::Unsafe => ColorRole::BadgeUnsafe,
               // ...
           };
           let color = self.theme.color(color_role);
           // Use color in template
       }
   }
   ```

### Dark Mode Support

Dark mode colors adjust for readability on dark backgrounds:
```rust
fn dark_color(&self, role: ColorRole) -> &str {
    match role {
        ColorRole::CodeBg => "#1E1E1E",
        ColorRole::CodeFg => "#D4D4D4",
        ColorRole::Border => "#424242",
        // Badge colors stay similar but slightly desaturated
        ColorRole::BadgeAsync => "#66BB6A",
        // ...
    }
}
```

### CSS Variable Generation (Future)

Prepare for CSS custom properties:
```rust
impl Theme for MkDocsMaterialTheme {
    fn to_css_variables(&self) -> String {
        let mut css = String::from(":root {\n");
        for role in ColorRole::iter() {
            css.push_str(&format!(
                "  --plissken-{}: {};\n",
                role.css_name(),
                self.color(role)
            ));
        }
        css.push_str("}\n");
        css
    }
}
```

## Status Updates

### Session 1 - Theme Extension Complete

**Files Modified:**
- `crates/plissken-core/src/render/theme.rs` - Extended ThemeAdapter trait
- `crates/plissken-core/src/render/renderer.rs` - Extended ThemeContext
- `crates/plissken-core/templates/partials/badge.html` - Use theme colors

**ThemeAdapter New Methods:**
Semantic colors (with defaults):
- `success()` → "#4caf50" (green)
- `warning()` → "#ff9800" (yellow/orange)
- `error()` → "#f44336" (red)
- `info()` → "#2196f3" (blue)

Badge colors (with defaults):
- `badge_async()` → `self.primary()` (inherits from primary)
- `badge_unsafe()` → `self.error()` (inherits from error)
- `badge_deprecated()` → `self.warning()` (inherits from warning)
- `badge_binding()` → "#9c27b0" (purple)
- `badge_pub()` → `self.success()` (inherits from success)
- `badge_pub_crate()` → "#ff5722" (deep orange)
- `badge_rust()` → "#ff5722" (Rust brand color)
- `badge_python()` → "#306998" (Python blue)

**MkDocs Material Overrides:**
- `badge_async()` → "var(--md-primary-fg-color)" (CSS variable)
- `badge_binding()` → "var(--md-accent-fg-color)" (CSS variable)

**Badge Template Updated:**
- Replaced hardcoded `#4caf50` with `{{ theme.success }}`
- Replaced hardcoded `#ff9800` with `{{ theme.warning }}`
- Replaced hardcoded `#f44336` with `{{ theme.error }}`
- Replaced hardcoded `#9c27b0` with `{{ theme.badge_binding }}`
- Replaced hardcoded `#ff5722` with `{{ theme.badge_pub_crate }}`

**Dark Mode Note:**
The `dark_mode()` method was not implemented because MkDocs Material's CSS variables automatically adapt to dark mode. The current approach using CSS variables like `var(--md-primary-fg-color)` provides better dark mode support than hardcoded color switching.

**Tests:** All 223 unit tests + 16 doctests pass (4 new theme tests added)