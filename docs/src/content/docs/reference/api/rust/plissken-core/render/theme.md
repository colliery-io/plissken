---
title: "plissken-core::render::theme"
description: "Theme adapters for SSG-native CSS variable integration"
editUrl: false
sidebar:
  order: 31
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>

Theme adapters for SSG-native CSS variable integration

This module provides adapters that map semantic color names to the native
CSS variables of various static site generators. By using inline styles
that reference these variables, generated documentation automatically
inherits the SSG's theme (including dark mode support) without shipping
any external CSS.

**Supported SSGs**

- **MkDocs Material**: Uses `var(--md-*)` CSS variables
- **mdBook**: Uses `var(--*)` CSS variables
- **Starlight**: Uses `var(--sl-color-*)` CSS variables (Aurora tokens with
  `starlight-aurora`)
- **Minimal**: Hardcoded fallback colors for unstyled contexts

**Examples:**

```rust
use plissken_core::render::{get_theme_adapter, ThemeAdapter};

let adapter = get_theme_adapter(Some("mkdocs-material"));
let style = format!("background: {}; color: {}", adapter.code_bg(), adapter.code_fg());
// Results in: "background: var(--md-code-bg-color); color: var(--md-code-fg-color)"
```

## Structs

### <span id="struct.MkDocsMaterial"></span><code style="font-size: 0.85em">struct MkDocsMaterial</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`, `Copy`, `Default`

MkDocs Material theme adapter

Uses MkDocs Material's `--md-*` CSS custom properties for seamless
integration with Material for MkDocs themes.

Reference: <https://squidfunk.github.io/mkdocs-material/setup/changing-the-colors/>

### <span id="struct.MdBook"></span><code style="font-size: 0.85em">struct MdBook</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`, `Copy`, `Default`

mdBook theme adapter

Uses mdBook's CSS custom properties for integration with mdBook themes.
mdBook uses simpler variable names without a prefix.

Reference: <https://rust-lang.github.io/mdBook/format/theme/index.html>

### <span id="struct.Starlight"></span><code style="font-size: 0.85em">struct Starlight</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`, `Copy`, `Default`

Astro Starlight theme adapter

Uses Starlight's `--sl-color-*` CSS custom properties. Each value follows
the site's light and dark themes. With the `starlight-aurora` theme, these
variables are set from Aurora's tokens (for example `--sl-color-green-low`
is `--ok-bg`), so badges take Aurora's status colors, hyper included.

A badge is a fill (`-low`) with text in the same hue (`-high`), the pair
that Starlight and Aurora make for text on a tint.

Reference: <https://starlight.astro.build/guides/css-and-tailwind/#theming>

### <span id="struct.Minimal"></span><code style="font-size: 0.85em">struct Minimal</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`, `Copy`, `Default`

Minimal theme adapter with hardcoded fallback colors

Provides reasonable default colors for contexts where CSS variables
are not available (plain markdown viewers, unstyled HTML, etc.).
Uses a light theme with accessible contrast ratios.

## Traits

### <span id="trait.ThemeAdapter"></span><code style="font-size: 0.85em">trait ThemeAdapter</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Supertraits:** `Send + Sync`

Trait for mapping semantic color names to SSG-specific CSS variables.

Implementations of this trait provide the actual CSS variable references
(or hardcoded colors) for each semantic color role. Templates use these
methods to generate inline styles that integrate with the target SSG's
theming system.

#### Methods

##### <span id="ThemeAdapter.code_bg"></span><code>code_​bg</code>

```rust
fn code_bg(&self) -> &str
```

Background color for code blocks and inline code

<details>
<summary>Source</summary>

```rust
    fn code_bg(&self) -> &str;
```

</details>

##### <span id="ThemeAdapter.code_fg"></span><code>code_​fg</code>

```rust
fn code_fg(&self) -> &str
```

Foreground (text) color for code

<details>
<summary>Source</summary>

```rust
    fn code_fg(&self) -> &str;
```

</details>

##### <span id="ThemeAdapter.primary"></span><code>primary</code>

```rust
fn primary(&self) -> &str
```

Primary/brand color for headings, important elements

<details>
<summary>Source</summary>

```rust
    fn primary(&self) -> &str;
```

</details>

##### <span id="ThemeAdapter.accent"></span><code>accent</code>

```rust
fn accent(&self) -> &str
```

Accent color for links, interactive elements

<details>
<summary>Source</summary>

```rust
    fn accent(&self) -> &str;
```

</details>

##### <span id="ThemeAdapter.muted"></span><code>muted</code>

```rust
fn muted(&self) -> &str
```

Muted/secondary text color

<details>
<summary>Source</summary>

```rust
    fn muted(&self) -> &str;
```

</details>

##### <span id="ThemeAdapter.border"></span><code>border</code>

```rust
fn border(&self) -> &str
```

Border color for tables, separators

<details>
<summary>Source</summary>

```rust
    fn border(&self) -> &str;
```

</details>

##### <span id="ThemeAdapter.name"></span><code>name</code>

```rust
fn name(&self) -> &str
```

Name of this theme adapter (for debugging/logging)

<details>
<summary>Source</summary>

```rust
    fn name(&self) -> &str;
```

</details>

##### <span id="ThemeAdapter.success"></span><code>success</code>

```rust
fn success(&self) -> &str
```

Success color (typically green) - for positive states

<details>
<summary>Source</summary>

```rust
    fn success(&self) -> &str {
        "#4caf50"
    }
```

</details>

##### <span id="ThemeAdapter.warning"></span><code>warning</code>

```rust
fn warning(&self) -> &str
```

Warning color (typically yellow/orange) - for caution states

<details>
<summary>Source</summary>

```rust
    fn warning(&self) -> &str {
        "#ff9800"
    }
```

</details>

##### <span id="ThemeAdapter.error"></span><code>error</code>

```rust
fn error(&self) -> &str
```

Error color (typically red) - for error/danger states

<details>
<summary>Source</summary>

```rust
    fn error(&self) -> &str {
        "#f44336"
    }
```

</details>

##### <span id="ThemeAdapter.info"></span><code>info</code>

```rust
fn info(&self) -> &str
```

Info color (typically blue) - for informational states

<details>
<summary>Source</summary>

```rust
    fn info(&self) -> &str {
        "#2196f3"
    }
```

</details>

##### <span id="ThemeAdapter.badge_async"></span><code>badge_​async</code>

```rust
fn badge_async(&self) -> &str
```

Color for async badges

<details>
<summary>Source</summary>

```rust
    fn badge_async(&self) -> &str {
        self.primary()
    }
```

</details>

##### <span id="ThemeAdapter.badge_unsafe"></span><code>badge_​unsafe</code>

```rust
fn badge_unsafe(&self) -> &str
```

Color for unsafe badges

<details>
<summary>Source</summary>

```rust
    fn badge_unsafe(&self) -> &str {
        self.error()
    }
```

</details>

##### <span id="ThemeAdapter.badge_deprecated"></span><code>badge_​deprecated</code>

```rust
fn badge_deprecated(&self) -> &str
```

Color for deprecated badges

<details>
<summary>Source</summary>

```rust
    fn badge_deprecated(&self) -> &str {
        self.warning()
    }
```

</details>

##### <span id="ThemeAdapter.badge_binding"></span><code>badge_​binding</code>

```rust
fn badge_binding(&self) -> &str
```

Color for PyO3/binding badges

<details>
<summary>Source</summary>

```rust
    fn badge_binding(&self) -> &str {
        "#9c27b0" // purple
    }
```

</details>

##### <span id="ThemeAdapter.badge_pub"></span><code>badge_​pub</code>

```rust
fn badge_pub(&self) -> &str
```

Color for visibility pub badges

<details>
<summary>Source</summary>

```rust
    fn badge_pub(&self) -> &str {
        self.success()
    }
```

</details>

##### <span id="ThemeAdapter.badge_pub_crate"></span><code>badge_​pub_​crate</code>

```rust
fn badge_pub_crate(&self) -> &str
```

Color for visibility pub(crate) badges

<details>
<summary>Source</summary>

```rust
    fn badge_pub_crate(&self) -> &str {
        "#ff5722" // deep orange
    }
```

</details>

##### <span id="ThemeAdapter.badge_rust"></span><code>badge_​rust</code>

```rust
fn badge_rust(&self) -> &str
```

Color for Rust source badges

<details>
<summary>Source</summary>

```rust
    fn badge_rust(&self) -> &str {
        "#ff5722" // deep orange (Rust's brand color is orange-ish)
    }
```

</details>

##### <span id="ThemeAdapter.badge_python"></span><code>badge_​python</code>

```rust
fn badge_python(&self) -> &str
```

Color for Python source badges

<details>
<summary>Source</summary>

```rust
    fn badge_python(&self) -> &str {
        "#306998" // Python blue
    }
```

</details>

##### <span id="ThemeAdapter.badge_fill"></span><code>badge_​fill</code>

```rust
fn badge_fill(&self, color_type: &str) -> String
```

Fill of a badge of the given color type.

Color types: "blue", "green", "yellow", "red", "purple", "orange",
and "gray" (any other value).

<details>
<summary>Source</summary>

```rust
    fn badge_fill(&self, color_type: &str) -> String {
        match color_type {
            "blue" => self.primary(),
            "green" => self.success(),
            "yellow" => self.warning(),
            "red" => self.error(),
            "purple" => self.badge_binding(),
            "orange" => self.badge_pub_crate(),
            _ => self.muted(),
        }
        .to_string()
    }
```

</details>

##### <span id="ThemeAdapter.badge_text"></span><code>badge_​text</code>

```rust
fn badge_text(&self, color_type: &str) -> String
```

Text color of a badge of the given color type (on its fill).

<details>
<summary>Source</summary>

```rust
    fn badge_text(&self, color_type: &str) -> String {
        match color_type {
            "yellow" => "black",
            _ => "white",
        }
        .to_string()
    }
```

</details>

##### <span id="ThemeAdapter.badge_base_style"></span><code>badge_​base_​style</code>

```rust
fn badge_base_style(&self) -> &str
```

CSS for the shape of a badge (everything but its colors).

<details>
<summary>Source</summary>

```rust
    fn badge_base_style(&self) -> &str {
        "display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle;"
    }
```

</details>

## Functions

### <span id="fn.get_theme_adapter"></span><code style="font-size: 0.85em">fn get_​theme_​adapter</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn get_theme_adapter(template: Option<&str>) -> Box<dyn ThemeAdapter>
```

Get a theme adapter based on the template name from config.

Returns the appropriate adapter for known SSG templates, or the
`Minimal` adapter as a fallback for unknown templates.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `template` | `Option<&str>` | Optional template name from `output.template` in config |

**Returns:**

A boxed trait object implementing `ThemeAdapter`

**Examples:**

```rust
use plissken_core::render::get_theme_adapter;

// Known templates
let mkdocs = get_theme_adapter(Some("mkdocs-material"));
assert_eq!(mkdocs.name(), "mkdocs-material");

let mdbook = get_theme_adapter(Some("mdbook"));
assert_eq!(mdbook.name(), "mdbook");

// Unknown or missing template falls back to minimal
let minimal = get_theme_adapter(None);
assert_eq!(minimal.name(), "minimal");
```

<details>
<summary>Source</summary>

```rust
pub fn get_theme_adapter(template: Option<&str>) -> Box<dyn ThemeAdapter> {
    match template {
        Some(t) => match t.to_lowercase().as_str() {
            "mkdocs-material" | "mkdocs_material" | "material" => Box::new(MkDocsMaterial),
            "mdbook" | "md-book" | "md_book" => Box::new(MdBook),
            "starlight" | "astro-starlight" => Box::new(Starlight),
            _ => Box::new(Minimal),
        },
        None => Box::new(Minimal),
    }
}
```

</details>
