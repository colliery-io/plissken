# render <span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #ff5722; color: white;">Rust</span>


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

