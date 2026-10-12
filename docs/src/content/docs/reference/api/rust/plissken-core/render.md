---
title: "plissken-core::render"
description: "Rendering module for documentation output"
editUrl: false
sidebar:
  order: 16
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>

Rendering module for documentation output

This module provides theme adapters and rendering utilities for generating
styled documentation that integrates with various static site generators.

**Architecture**

The rendering system consists of two main components:

1. **Theme Adapters** - Map semantic color names to SSG-specific CSS variables
2. **Renderer** - Tera-based template engine with theme injection

**Examples:**

```rust
use plissken_core::render::Renderer;

let renderer = Renderer::new(Some("mkdocs-material"), None).unwrap();
let badge = renderer.badge_async().unwrap();
// Badge uses MkDocs Material's CSS variables
```

## Re-exports

- [`CrossRefLink`](../../../rust/plissken-core/render/crossref_renderer/#struct.CrossRefLink)
- [`Language`](../../../rust/plissken-core/render/crossref_renderer/#enum.Language)
- [`crossref_link`](../../../rust/plissken-core/render/crossref_renderer/#fn.crossref_link)
- [`link_to_python`](../../../rust/plissken-core/render/crossref_renderer/#fn.link_to_python)
- [`link_to_rust`](../../../rust/plissken-core/render/crossref_renderer/#fn.link_to_rust)
- [`render_python_exposure_details`](../../../rust/plissken-core/render/crossref_renderer/#fn.render_python_exposure_details)
- [`render_rust_impl_details`](../../../rust/plissken-core/render/crossref_renderer/#fn.render_rust_impl_details)
- [`escape_table_content`](../../../rust/plissken-core/render/docstring_renderer/#fn.escape_table_content)
- [`render_docstring`](../../../rust/plissken-core/render/docstring_renderer/#fn.render_docstring)
- [`render_examples`](../../../rust/plissken-core/render/docstring_renderer/#fn.render_examples)
- [`render_params_table`](../../../rust/plissken-core/render/docstring_renderer/#fn.render_params_table)
- [`render_raises_table`](../../../rust/plissken-core/render/docstring_renderer/#fn.render_raises_table)
- [`render_returns`](../../../rust/plissken-core/render/docstring_renderer/#fn.render_returns)
- [`render_rust_docstring`](../../../rust/plissken-core/render/docstring_renderer/#fn.render_rust_docstring)
- [`ModuleRenderer`](../../../rust/plissken-core/render/module_renderer/#struct.ModuleRenderer)
- [`RenderedPage`](../../../rust/plissken-core/render/module_renderer/#struct.RenderedPage)
- [`Renderer`](../../../rust/plissken-core/render/renderer/#struct.Renderer)
- [`TemplateLoader`](../../../rust/plissken-core/render/templates/#struct.TemplateLoader)
- [`MdBook`](../../../rust/plissken-core/render/theme/#struct.MdBook)
- [`Minimal`](../../../rust/plissken-core/render/theme/#struct.Minimal)
- [`MkDocsMaterial`](../../../rust/plissken-core/render/theme/#struct.MkDocsMaterial)
- [`Starlight`](../../../rust/plissken-core/render/theme/#struct.Starlight)
- [`ThemeAdapter`](../../../rust/plissken-core/render/theme/#trait.ThemeAdapter)
- [`get_theme_adapter`](../../../rust/plissken-core/render/theme/#fn.get_theme_adapter)
