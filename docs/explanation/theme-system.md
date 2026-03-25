# Theme System Design

This page explains why plissken's theme system works the way it does, and
the trade-offs involved.

## The Problem

Documentation generators that produce HTML with hardcoded styles face a
fundamental conflict: the generated styles clash with the host static site
generator's (SSG) theme.
Colors that look good in light mode break in dark mode. Custom CSS fights
with the SSG's own stylesheets.

## The Solution: CSS Variable Delegation

Instead of shipping its own CSS, plissken delegates styling to the host
SSG. When generating content for MkDocs Material, plissken emits inline
styles that reference MkDocs Material's CSS custom properties:

```html
<span style="background: var(--md-primary-fg-color); color: white;">
    async
</span>
```

This means:

1. **Automatic dark mode**: When the user toggles dark mode in MkDocs
   Material, the CSS variables change value, and plissken's badges
   automatically adapt.

2. **Theme consistency**: Badges use the same primary color the user
   configured in their mkdocs.yml palette. If they choose "deep purple",
   badges are deep purple.

3. **Zero CSS shipped**: plissken doesn't generate any CSS files for
   MkDocs. Everything is inline styles using CSS variables.

## Theme Adapter Trait

The `ThemeAdapter` trait defines the semantic color roles:

```rust
pub trait ThemeAdapter {
    fn code_bg(&self) -> &str;    // Code block background
    fn code_fg(&self) -> &str;    // Code text color
    fn primary(&self) -> &str;    // Brand/heading color
    fn accent(&self) -> &str;     // Link/interactive color
    fn muted(&self) -> &str;      // Secondary text
    fn border(&self) -> &str;     // Borders and separators
    fn name(&self) -> &str;       // Adapter identifier

    // Semantic colors with defaults
    fn success(&self) -> &str;    // Green
    fn warning(&self) -> &str;    // Yellow/orange
    fn error(&self) -> &str;      // Red
    fn info(&self) -> &str;       // Blue

    // Badge colors with defaults
    fn badge_async(&self) -> &str;
    fn badge_unsafe(&self) -> &str;
    // ... etc
}
```

Each SSG adapter implements this trait with CSS variables appropriate for
that SSG:

- **MkDocs Material** returns `var(--md-code-bg-color)` for `code_bg()`
- **mdBook** returns `var(--code-bg)` for `code_bg()`
- **Minimal** returns `#f5f5f5` for `code_bg()`

## Why Inline Styles?

Inline styles have trade-offs:

**Advantages:**
- No separate CSS file to manage or include in SSG config
- No CSS specificity conflicts with SSG themes
- Works out of the box without any user configuration
- Self-contained: the Markdown output is complete and portable

**Disadvantages:**
- Slightly larger file sizes (repeated style strings)
- Harder to override globally (requires `!important`)
- Can't use pseudo-elements or media queries inline

plissken mitigates the override problem by adding CSS classes to every
styled element (`plissken-badge`, `plissken-badge-async`, etc.). Users
can write external CSS targeting these classes, which overrides the inline
styles:

```css
.plissken-badge-async {
    background: linear-gradient(45deg, #4CAF50, #8BC34A) !important;
}
```

## Template Override System

For users who need full control, plissken supports template overrides.
Place custom Tera templates in `.plissken/templates/` to replace any
bundled template:

```
.plissken/templates/partials/badge.html
```

This file completely replaces the bundled badge template. The user has
access to all theme variables and can produce any HTML they want.

The template loader checks for user overrides first, then falls back to
bundled defaults:

1. Check `.plissken/templates/{name}` — user override
2. Fall back to compiled-in default template

This per-file override means you can customize just the badge template
while keeping everything else at the defaults.

## Why Three Adapters?

**MkDocs Material** is the primary target because it's the most popular
documentation framework in the Python ecosystem and has excellent dark
mode support via CSS custom properties.

**mdBook** is included because it's the standard in the Rust ecosystem.
Many Rust projects use mdBook for documentation, and supporting it means
plissken works for pure-Rust projects that prefer the Rust toolchain.

**Minimal** exists as a fallback for unknown SSGs or contexts where CSS
variables aren't available (like viewing raw Markdown on GitHub). It uses
hardcoded hex colors that provide reasonable contrast on a white background.

## SSG Adapter vs Theme Adapter

These are separate concerns:

- **Theme Adapter** answers: "What CSS value should I use for the primary
  color?" It controls visual appearance.

- **SSG Adapter** answers: "What format should the navigation file be in?"
  It controls structural output (directory layout, nav file format, config
  generation).

A theme adapter maps semantic colors to CSS. An SSG adapter maps modules
to file paths and navigation entries. They're used at different stages of
the pipeline.
