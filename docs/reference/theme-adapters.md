# Theme Adapter Reference

Theme adapters map plissken's semantic color names to CSS values appropriate
for specific static site generators. This allows generated documentation
to inherit the SSG's theming automatically, including dark mode support.

## Available Adapters

| Adapter | Activated By | CSS Strategy |
|---------|-------------|--------------|
| MkDocs Material | `template = "mkdocs-material"` | `var(--md-*)` CSS custom properties |
| mdBook | `template = "mdbook"` | `var(--*)` CSS custom properties |
| Minimal | Any unrecognized template or `None` | Hardcoded hex colors |

## MkDocs Material

Uses Material for MkDocs CSS custom properties. Colors adapt automatically
to the user's light/dark mode selection.

| Semantic Role | CSS Value |
|---------------|-----------|
| `code_bg` | `var(--md-code-bg-color)` |
| `code_fg` | `var(--md-code-fg-color)` |
| `primary` | `var(--md-primary-fg-color)` |
| `accent` | `var(--md-accent-fg-color)` |
| `muted` | `var(--md-default-fg-color--light)` |
| `border` | `var(--md-default-fg-color--lightest)` |

Badge overrides:

| Badge | CSS Value |
|-------|-----------|
| `badge_async` | `var(--md-primary-fg-color)` |
| `badge_binding` | `var(--md-accent-fg-color)` |

All other badge colors use the default hardcoded values.

Reference: [MkDocs Material color customization](https://squidfunk.github.io/mkdocs-material/setup/changing-the-colors/)

## mdBook

Uses mdBook's CSS custom properties. Colors adapt to the selected mdBook
theme (Light, Rust, Coal, Navy, Ayu).

| Semantic Role | CSS Value |
|---------------|-----------|
| `code_bg` | `var(--code-bg)` |
| `code_fg` | `var(--inline-code-color)` |
| `primary` | `var(--links)` |
| `accent` | `var(--links)` |
| `muted` | `var(--fg)` |
| `border` | `var(--quote-border)` |

Reference: [mdBook theme customization](https://rust-lang.github.io/mdBook/format/theme/index.html)

## Minimal

Provides hardcoded hex colors that work in any context — plain Markdown
viewers, unstyled HTML, GitHub rendering, etc. Uses accessible contrast
ratios on a light background.

| Semantic Role | Value |
|---------------|-------|
| `code_bg` | `#f5f5f5` |
| `code_fg` | `#333333` |
| `primary` | `#1976d2` |
| `accent` | `#ff4081` |
| `muted` | `#757575` |
| `border` | `#e0e0e0` |

## Semantic Color Defaults

These colors are shared across all adapters (can be overridden per adapter):

| Role | Default | Used For |
|------|---------|----------|
| `success` | `#4caf50` | `pub` badges, success states |
| `warning` | `#ff9800` | `deprecated` badges, caution |
| `error` | `#f44336` | `unsafe` badges, errors |
| `info` | `#2196f3` | Informational states |

## Badge Color Defaults

| Badge | Default Value | Can Be Overridden |
|-------|---------------|-------------------|
| `badge_async` | Same as `primary` | Yes (MkDocs Material overrides) |
| `badge_unsafe` | Same as `error` | Yes |
| `badge_deprecated` | Same as `warning` | Yes |
| `badge_binding` | `#9c27b0` (purple) | Yes (MkDocs Material overrides) |
| `badge_pub` | Same as `success` | Yes |
| `badge_pub_crate` | `#ff5722` (deep orange) | Yes |
| `badge_rust` | `#ff5722` (deep orange) | Yes |
| `badge_python` | `#306998` (Python blue) | Yes |

## Using in Templates

In Tera templates, access theme values via the `theme` context variable:

```html
<span style="background: {{ theme.primary }}; color: white;">
    {{ text }}
</span>
```

The badge template resolves `color_type` to the appropriate theme variable:

| `color_type` | Resolves To |
|--------------|-------------|
| `"blue"` | `theme.primary` |
| `"green"` | `theme.success` |
| `"red"` | `theme.error` |
| `"yellow"` | `theme.warning` |
| `"purple"` | `theme.badge_binding` |
| `"orange"` | `theme.badge_pub_crate` |
| `"gray"` | `theme.muted` |
