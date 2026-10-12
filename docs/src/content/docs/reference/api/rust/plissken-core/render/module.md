---
title: "plissken-core::render::module"
description: "Module documentation components"
editUrl: false
sidebar:
  order: 20
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>

Module documentation components

This module contains extracted components from the monolithic ModuleRenderer,
providing focused, single-responsibility types for documentation generation.

**Components**

- [`PageLayout`](../../../../rust/plissken-core/render/module/layout/#struct.PageLayout) - File path computation and directory structure
- [`CrossRefLinker`](../../../../rust/plissken-core/render/module/crossref/#struct.CrossRefLinker) - Cross-reference link generation between Python/Rust docs

**Examples:**

```rust
use plissken_core::render::module::{PageLayout, CrossRefLinker};

let layout = PageLayout::new();

// Get paths for Python modules
let index = layout.python_index_path("mypackage.submodule");
let item = layout.python_item_path("mypackage.submodule", "MyClass");

// Get paths for Rust modules
let rust_index = layout.rust_index_path("mycrate::submod");

// Cross-reference linking (empty linker for pure Python/Rust projects)
let linker = CrossRefLinker::empty();
```

## Re-exports

- [`CrossRefLinker`](../../../../rust/plissken-core/render/module/crossref/#struct.CrossRefLinker)
- [`LinkStyle`](../../../../rust/plissken-core/render/module/crossref/#enum.LinkStyle)
- [`PageLayout`](../../../../rust/plissken-core/render/module/layout/#struct.PageLayout)
