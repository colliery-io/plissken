---
title: "plissken-core::render::ssg::mkdocs"
description: "MkDocs adapter implementation"
editUrl: false
sidebar:
  order: 27
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>

MkDocs adapter implementation

## Structs

### <span id="struct.MkDocsAdapter"></span><code style="font-size: 0.85em">struct MkDocsAdapter</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

MkDocs adapter for Material theme.

Generates YAML navigation and integrates with MkDocs Material's features.

**Navigation Format (Inline)**

All content is rendered inline in single module files:
```yaml
nav:
  - Python:
    - pysnake: pysnake.md
  - Rust:
    - rustscale: rust/rustscale.md
    - rustscale::config: rust/rustscale/config.md
```
