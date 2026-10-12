---
title: "plissken-core::render"
description: "Rendering module for documentation output"
sidebar:
  order: 14
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>


Rendering module for documentation output

This module provides theme adapters and rendering utilities for generating
styled documentation that integrates with various static site generators.

**Examples:**

```rust
use plissken_core::render::Renderer;

let renderer = Renderer::new(Some("mkdocs-material"), None).unwrap();
let badge = renderer.badge_async().unwrap();
// Badge uses MkDocs Material's CSS variables
```

