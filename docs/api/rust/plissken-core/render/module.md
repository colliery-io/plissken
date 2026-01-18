# module <span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #ff5722; color: white;">Rust</span>


Module documentation components

This module contains extracted components from the monolithic ModuleRenderer,
providing focused, single-responsibility types for documentation generation.

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

