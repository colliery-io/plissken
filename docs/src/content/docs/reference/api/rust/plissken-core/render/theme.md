---
title: "plissken-core::render::theme"
description: "Theme adapters for SSG-native CSS variable integration"
sidebar:
  order: 28
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>


Theme adapters for SSG-native CSS variable integration

This module provides adapters that map semantic color names to the native
CSS variables of various static site generators. By using inline styles
that reference these variables, generated documentation automatically
inherits the SSG's theme (including dark mode support) without shipping
any external CSS.

**Examples:**

```rust
use plissken_core::render::{get_theme_adapter, ThemeAdapter};

let adapter = get_theme_adapter(Some("mkdocs-material"));
let style = format!("background: {}; color: {}", adapter.code_bg(), adapter.code_fg());
// Results in: "background: var(--md-code-bg-color); color: var(--md-code-fg-color)"
```

## Structs

### <span id="struct.MkDocsMaterial"></span>`struct MkDocsMaterial`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


**Derives:** `Debug`, `Clone`, `Copy`, `Default`

MkDocs Material theme adapter

Uses MkDocs Material's `--md-*` CSS custom properties for seamless
integration with Material for MkDocs themes.
Reference: <https://squidfunk.github.io/mkdocs-material/setup/changing-the-colors/>



### <span id="struct.MdBook"></span>`struct MdBook`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


**Derives:** `Debug`, `Clone`, `Copy`, `Default`

mdBook theme adapter

Uses mdBook's CSS custom properties for integration with mdBook themes.
mdBook uses simpler variable names without a prefix.
Reference: <https://rust-lang.github.io/mdBook/format/theme/index.html>



### <span id="struct.Starlight"></span>`struct Starlight`

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

#### Methods

##### <span id="Starlight.hue"></span>`hue`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-gray-6); color: var(--sl-color-gray-2);">private</span>


```rust
fn hue (color_type : & str) -> Option < & 'static str >
```

The Starlight hue of a badge color type.

<details>
<summary>Source</summary>

```rust
    fn hue(color_type: &str) -> Option<&'static str> {
        match color_type {
            "blue" => Some("blue"),
            "green" => Some("green"),
            "yellow" | "orange" => Some("orange"),
            "red" => Some("red"),
            "purple" => Some("purple"),
            _ => None,
        }
    }
```

</details>





### <span id="struct.Minimal"></span>`struct Minimal`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


**Derives:** `Debug`, `Clone`, `Copy`, `Default`

Minimal theme adapter with hardcoded fallback colors

Provides reasonable default colors for contexts where CSS variables
are not available (plain markdown viewers, unstyled HTML, etc.).
Uses a light theme with accessible contrast ratios.



## Functions

### <span id="fn.get_theme_adapter"></span>`fn get_theme_adapter`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


```rust
fn get_theme_adapter (template : Option < & str >) -> Box < dyn ThemeAdapter >
```

Get a theme adapter based on the template name from config.

Returns the appropriate adapter for known SSG templates, or the
`Minimal` adapter as a fallback for unknown templates.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `template` | `-` | Optional template name from `output.template` in config |


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



