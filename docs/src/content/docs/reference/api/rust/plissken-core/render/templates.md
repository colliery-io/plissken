---
title: "plissken-core::render::templates"
description: "Template loading with user override support"
editUrl: false
sidebar:
  order: 30
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>

Template loading with user override support

This module provides template loading functionality that supports:
- Bundled default templates (embedded at compile time)
- User overrides from `.plissken/templates/` directory

User templates take precedence over bundled defaults on a per-file basis.

## Structs

### <span id="struct.TemplateLoader"></span><code style="font-size: 0.85em">struct TemplateLoader</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

Template loader with user override support.

The loader first checks for user-provided templates in the configured
override directory, then falls back to bundled defaults.

**Examples:**

```rust
use plissken_core::render::TemplateLoader;
use std::path::Path;

// Create loader without user overrides (uses bundled only)
let loader = TemplateLoader::new(None);

// Create loader with project root for user overrides
let loader = TemplateLoader::new(Some(Path::new("/path/to/project")));

// Get a template
let badge_template = loader.get("partials/badge.html").unwrap();
```

#### Methods

##### <span id="TemplateLoader.new"></span><code>new</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn new(project_root: Option<&Path>) -> Self
```

Create a new template loader.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `project_root` | `Option<&Path>` | Optional path to project root. If provided, user templates will be loaded from `{project_root}/.plissken/templates/`. |

<details>
<summary>Source</summary>

```rust
    pub fn new(project_root: Option<&Path>) -> Self {
        let user_dir = project_root.and_then(|root| {
            let dir = root.join(".plissken").join("templates");
            if dir.exists() && dir.is_dir() {
                Some(dir)
            } else {
                None
            }
        });

        Self {
            bundled: Self::load_bundled(),
            user_dir,
        }
    }
```

</details>

##### <span id="TemplateLoader.get"></span><code>get</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn get(&self, name: &str) -> crate::error::Result<String>
```

Get a template by name.

First checks for a user override, then falls back to the bundled default.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `name` | `&str` | Template name (e.g., "partials/badge.html", "module.html") |

**Returns:**

The template content as a string, or an error if not found.

<details>
<summary>Source</summary>

```rust
    pub fn get(&self, name: &str) -> crate::error::Result<String> {
        // Check user override first
        if let Some(ref dir) = self.user_dir {
            let user_path = dir.join(name);
            if user_path.exists() {
                return std::fs::read_to_string(&user_path)
                    .map_err(|e| crate::error::PlisskenError::file_read(&user_path, e));
            }
        }

        // Fall back to bundled
        self.bundled
            .get(name)
            .map(|s| s.to_string())
            .ok_or_else(|| crate::error::PlisskenError::Template {
                message: format!("template not found: {}", name),
                source: tera::Error::msg(format!(
                    "template '{}' not found in bundled templates",
                    name
                )),
            })
    }
```

</details>

##### <span id="TemplateLoader.has_user_override"></span><code>has_​user_​override</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn has_user_override(&self, name: &str) -> bool
```

Check if a user override exists for the given template.

<details>
<summary>Source</summary>

```rust
    pub fn has_user_override(&self, name: &str) -> bool {
        if let Some(ref dir) = self.user_dir {
            dir.join(name).exists()
        } else {
            false
        }
    }
```

</details>

##### <span id="TemplateLoader.template_names"></span><code>template_​names</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn template_names(&self) -> Vec<&'static str>
```

List all available template names.

<details>
<summary>Source</summary>

```rust
    pub fn template_names(&self) -> Vec<&'static str> {
        self.bundled.keys().copied().collect()
    }
```

</details>

##### <span id="TemplateLoader.user_override_dir"></span><code>user_​override_​dir</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn user_override_dir(&self) -> Option<&Path>
```

Get the user override directory, if configured and exists.

<details>
<summary>Source</summary>

```rust
    pub fn user_override_dir(&self) -> Option<&Path> {
        self.user_dir.as_deref()
    }
```

</details>
