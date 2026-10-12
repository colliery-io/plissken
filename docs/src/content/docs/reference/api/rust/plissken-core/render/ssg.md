---
title: "plissken-core::render::ssg"
description: "Static Site Generator (SSG) adapters"
editUrl: false
sidebar:
  order: 25
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>

Static Site Generator (SSG) adapters

This module provides adapters for different static site generators,
abstracting away SSG-specific differences in navigation format,
config files, and directory structure.

**Supported SSGs**

- **MkDocs** (with Material theme): YAML navigation, `docs/` content directory
- **mdBook**: Markdown SUMMARY.md, `src/` content directory
- **Astro Starlight**: a sidebar group in `_sidebar.mjs`, pages in a folder
  of `src/content/docs/`

**Examples:**

```rust
use plissken_core::render::ssg::{SSGAdapter, get_ssg_adapter};

let adapter = get_ssg_adapter(Some("mkdocs-material"));
assert_eq!(adapter.name(), "mkdocs");
assert_eq!(adapter.content_dir(), "docs");
assert_eq!(adapter.nav_filename(), "_nav.yml");
```

## Re-exports

- [`MdBookAdapter`](../../../../rust/plissken-core/render/ssg/mdbook/#struct.MdBookAdapter)
- [`MkDocsAdapter`](../../../../rust/plissken-core/render/ssg/mkdocs/#struct.MkDocsAdapter)
- [`STARLIGHT_DEFAULT_ROUTE_BASE`](../../../../rust/plissken-core/render/ssg/starlight/#const.STARLIGHT_DEFAULT_ROUTE_BASE)
- [`STARLIGHT_INDEX_FILE`](../../../../rust/plissken-core/render/ssg/starlight/#const.STARLIGHT_INDEX_FILE)
- [`STARLIGHT_SIDEBAR_FILE`](../../../../rust/plissken-core/render/ssg/starlight/#const.STARLIGHT_SIDEBAR_FILE)
- [`StarlightAdapter`](../../../../rust/plissken-core/render/ssg/starlight/#struct.StarlightAdapter)
- [`anchor_span`](../../../../rust/plissken-core/render/ssg/starlight/#fn.anchor_span)
- [`breakable_name`](../../../../rust/plissken-core/render/ssg/starlight/#fn.breakable_name)
- [`content_route_base`](../../../../rust/plissken-core/render/ssg/starlight/#fn.content_route_base)
- [`front_matter`](../../../../rust/plissken-core/render/ssg/starlight/#fn.front_matter)
- [`github_slug`](../../../../rust/plissken-core/render/ssg/starlight/#fn.github_slug)
- [`normalize_rustdoc_fences`](../../../../rust/plissken-core/render/ssg/starlight/#fn.normalize_rustdoc_fences)
- [`page_route`](../../../../rust/plissken-core/render/ssg/starlight/#fn.page_route)
- [`route_href`](../../../../rust/plissken-core/render/ssg/starlight/#fn.route_href)
- [`starlight_file_path`](../../../../rust/plissken-core/render/ssg/starlight/#fn.starlight_file_path)
- [`NavEntry`](../../../../rust/plissken-core/render/ssg/traits/#struct.NavEntry)
- [`SSGAdapter`](../../../../rust/plissken-core/render/ssg/traits/#trait.SSGAdapter)
- [`python_nav_entries`](../../../../rust/plissken-core/render/ssg/traits/#fn.python_nav_entries)
- [`rust_nav_entries`](../../../../rust/plissken-core/render/ssg/traits/#fn.rust_nav_entries)

## Functions

### <span id="fn.get_ssg_adapter"></span><code style="font-size: 0.85em">fn get_​ssg_​adapter</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn get_ssg_adapter(template: Option<&str>) -> Box<dyn SSGAdapter>
```

Get an SSG adapter for the given template name.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `template` | `Option<&str>` | Optional template name. If None or unrecognized, defaults to MkDocs. |

**Examples:**

```rust
use plissken_core::render::ssg::get_ssg_adapter;

let mkdocs = get_ssg_adapter(Some("mkdocs-material"));
assert_eq!(mkdocs.name(), "mkdocs");

let mdbook = get_ssg_adapter(Some("mdbook"));
assert_eq!(mdbook.name(), "mdbook");

let default = get_ssg_adapter(None);
assert_eq!(default.name(), "mkdocs");
```

<details>
<summary>Source</summary>

```rust
pub fn get_ssg_adapter(template: Option<&str>) -> Box<dyn SSGAdapter> {
    match template.map(|s| s.to_lowercase()).as_deref() {
        Some("mdbook") | Some("md-book") | Some("md_book") => Box::new(MdBookAdapter),
        Some("starlight") | Some("astro-starlight") => Box::new(StarlightAdapter::default()),
        _ => Box::new(MkDocsAdapter),
    }
}
```

</details>
