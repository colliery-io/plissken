# Template Variables Reference

This page documents all variables available in plissken's Tera templates.

## Global Variables

Every template receives a `theme` object with CSS-appropriate color values.
The actual values depend on the configured template (`mkdocs-material`,
`mdbook`, or minimal fallback).

### `theme` Object

| Variable | Description | MkDocs Material | mdBook | Minimal |
|----------|-------------|----------------|--------|---------|
| `theme.name` | Theme identifier | `"mkdocs-material"` | `"mdbook"` | `"minimal"` |
| `theme.code_bg` | Code background | `var(--md-code-bg-color)` | `var(--code-bg)` | `#f5f5f5` |
| `theme.code_fg` | Code foreground | `var(--md-code-fg-color)` | `var(--inline-code-color)` | `#333333` |
| `theme.primary` | Primary/brand color | `var(--md-primary-fg-color)` | `var(--links)` | `#1976d2` |
| `theme.accent` | Accent/link color | `var(--md-accent-fg-color)` | `var(--links)` | `#ff4081` |
| `theme.muted` | Muted text color | `var(--md-default-fg-color--light)` | `var(--fg)` | `#757575` |
| `theme.border` | Border color | `var(--md-default-fg-color--lightest)` | `var(--quote-border)` | `#e0e0e0` |
| `theme.success` | Success color | `#4caf50` | `#4caf50` | `#4caf50` |
| `theme.warning` | Warning color | `#ff9800` | `#ff9800` | `#ff9800` |
| `theme.error` | Error color | `#f44336` | `#f44336` | `#f44336` |
| `theme.info` | Info color | `#2196f3` | `#2196f3` | `#2196f3` |
| `theme.badge_async` | Async badge color | `var(--md-primary-fg-color)` | primary | `#1976d2` |
| `theme.badge_unsafe` | Unsafe badge color | error | error | `#f44336` |
| `theme.badge_deprecated` | Deprecated badge color | warning | warning | `#ff9800` |
| `theme.badge_binding` | Binding badge color | `var(--md-accent-fg-color)` | `#9c27b0` | `#9c27b0` |
| `theme.badge_pub` | pub visibility color | success | success | `#4caf50` |
| `theme.badge_pub_crate` | pub(crate) color | `#ff5722` | `#ff5722` | `#ff5722` |
| `theme.badge_rust` | Rust source color | `#ff5722` | `#ff5722` | `#ff5722` |
| `theme.badge_python` | Python source color | `#306998` | `#306998` | `#306998` |

---

## `partials/badge.html`

Renders an inline badge (e.g., `async`, `pub`, `Binding`).

| Variable | Type | Description |
|----------|------|-------------|
| `text` | string | The badge label text. |
| `badge_type` | string | Semantic type for CSS class. One of: `async`, `unsafe`, `deprecated`, `visibility`, `source`. |
| `color_type` | string | Color category. One of: `blue`, `green`, `red`, `yellow`, `purple`, `orange`, `gray`. |
| `theme` | object | Theme color values (see above). |

The badge color is resolved from `color_type`:

| `color_type` | Resolved color |
|--------------|---------------|
| `blue` | `theme.primary` |
| `green` | `theme.success` |
| `red` | `theme.error` |
| `yellow` | `theme.warning` |
| `purple` | `theme.badge_binding` |
| `orange` | `theme.badge_pub_crate` |
| `gray` | `theme.muted` |

---

## `partials/signature.html`

Renders a function or method signature.

| Variable | Type | Description |
|----------|------|-------------|
| `name` | string | Function/method name. |
| `params` | string | Formatted parameter list (e.g., `"data: str, timeout: int = 30"`). |
| `return_type` | string | Return type annotation. Empty string if none. |
| `is_async` | bool | Whether the function is async. |
| `theme` | object | Theme color values. |

---

## `partials/code_block.html`

Renders a fenced code block.

| Variable | Type | Description |
|----------|------|-------------|
| `code` | string | The code content. |
| `language` | string | Language for syntax highlighting. Empty string if unspecified. |
| `caption` | string | Optional title/caption. Empty string if none. |
| `theme` | object | Theme color values. |

---

## `module.html`

Renders a complete module documentation page.

| Variable | Type | Description |
|----------|------|-------------|
| `module_name` | string | Module name (dotted for Python, `::` for Rust). |
| `description` | string | Module docstring or doc comment content. |
| `functions` | list of strings | Pre-rendered HTML for each function in the module. |
| `classes` | list of strings | Pre-rendered HTML for each class/struct in the module. |
| `theme` | object | Theme color values. |

Note: `functions` and `classes` contain already-rendered HTML strings from
the module renderer. The template arranges them on the page.
