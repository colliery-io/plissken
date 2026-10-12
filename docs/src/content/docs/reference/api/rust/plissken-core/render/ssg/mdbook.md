---
title: "plissken-core::render::ssg::mdbook"
description: "mdBook adapter implementation"
editUrl: false
sidebar:
  order: 26
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>

mdBook adapter implementation

## Structs

### <span id="struct.MdBookAdapter"></span><code style="font-size: 0.85em">struct MdBookAdapter</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

mdBook adapter.

Generates Markdown SUMMARY.md navigation and book.toml configuration.

**Navigation Format (Inline)**

All content is rendered inline in single module files:
```markdown
# Summary

# Python

- [pysnake](pysnake.md)

# Rust

- [rustscale](rust/rustscale.md)
  - [rustscale::config](rust/rustscale/config.md)
```
