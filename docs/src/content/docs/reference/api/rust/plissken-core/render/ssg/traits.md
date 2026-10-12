---
title: "plissken-core::render::ssg::traits"
description: "SSG adapter trait definition"
editUrl: false
sidebar:
  order: 29
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>

SSG adapter trait definition

## Structs

### <span id="struct.NavEntry"></span><code style="font-size: 0.85em">struct NavEntry</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

Entry for a module in the navigation

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `path` | `String` | Module path (dotted for Python, :: for Rust) |
| `file_path` | `PathBuf` | File path for the documentation page |
| `depth` | `usize` | Nesting depth for hierarchical display |

## Traits

### <span id="trait.SSGAdapter"></span><code style="font-size: 0.85em">trait SSGAdapter</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Supertraits:** `Send + Sync`

Adapter for static site generator output.

This trait abstracts the differences between static site generators,
providing a unified interface for navigation generation, config files,
and directory structure.

**Implementors**

- [`MkDocsAdapter`](../../../../../rust/plissken-core/render/ssg/mkdocs/#struct.MkDocsAdapter) - For MkDocs with Material theme
- [`MdBookAdapter`](../../../../../rust/plissken-core/render/ssg/mdbook/#struct.MdBookAdapter) - For mdBook
- [`StarlightAdapter`](../../../../../rust/plissken-core/render/ssg/starlight/#struct.StarlightAdapter) - For Astro Starlight

#### Methods

##### <span id="SSGAdapter.name"></span><code>name</code>

```rust
fn name(&self) -> &'static str
```

Human-readable name for the SSG.

<details>
<summary>Source</summary>

```rust
    fn name(&self) -> &'static str;
```

</details>

##### <span id="SSGAdapter.content_dir"></span><code>content_​dir</code>

```rust
fn content_dir(&self) -> &'static str
```

Directory where content files are placed (relative to output root).

- MkDocs: `"docs"`
- mdBook: `"src"`

<details>
<summary>Source</summary>

```rust
    fn content_dir(&self) -> &'static str;
```

</details>

##### <span id="SSGAdapter.nav_filename"></span><code>nav_​filename</code>

```rust
fn nav_filename(&self) -> &'static str
```

Navigation file name.

- MkDocs: `"_nav.yml"` (included in mkdocs.yml)
- mdBook: `"SUMMARY.md"`

<details>
<summary>Source</summary>

```rust
    fn nav_filename(&self) -> &'static str;
```

</details>

##### <span id="SSGAdapter.generate_nav"></span><code>generate_​nav</code>

```rust
fn generate_nav(
    &self,
    python_modules: &[PythonModule],
    rust_modules: &[RustModule],
) -> String
```

Generate navigation content from modules.

Returns the navigation content in the SSG's expected format:
- MkDocs: YAML format for the `nav:` section
- mdBook: Markdown format for SUMMARY.md

<details>
<summary>Source</summary>

```rust
    fn generate_nav(&self, python_modules: &[PythonModule], rust_modules: &[RustModule]) -> String;
```

</details>

##### <span id="SSGAdapter.generate_config"></span><code>generate_​config</code>

```rust
fn generate_config(
    &self,
    title: &str,
    authors: &[String],
) -> Option<String>
```

Generate SSG config file content.

- MkDocs: Returns `None` (mkdocs.yml typically pre-exists)
- mdBook: Returns book.toml content

<details>
<summary>Source</summary>

```rust
    fn generate_config(&self, title: &str, authors: &[String]) -> Option<String>;
```

</details>

##### <span id="SSGAdapter.generate_custom_css"></span><code>generate_​custom_​css</code>

```rust
fn generate_custom_css(&self) -> Option<String>
```

Generate custom CSS for the SSG, if any.

Returns CSS content that should be added to customize the documentation
appearance, or None if no custom CSS is needed.

<details>
<summary>Source</summary>

```rust
    fn generate_custom_css(&self) -> Option<String> {
        None
    }
```

</details>

##### <span id="SSGAdapter.custom_css_path"></span><code>custom_​css_​path</code>

```rust
fn custom_css_path(&self) -> Option<&'static str>
```

Path for custom CSS file, if generated.

- mdBook: `"theme/custom.css"`

<details>
<summary>Source</summary>

```rust
    fn custom_css_path(&self) -> Option<&'static str> {
        None
    }
```

</details>

##### <span id="SSGAdapter.content_extension"></span><code>content_​extension</code>

```rust
fn content_extension(&self) -> &'static str
```

File extension for content files.

<details>
<summary>Source</summary>

```rust
    fn content_extension(&self) -> &'static str {
        "md"
    }
```

</details>

##### <span id="SSGAdapter.supports_nested_nav"></span><code>supports_​nested_​nav</code>

```rust
fn supports_nested_nav(&self) -> bool
```

Whether this SSG supports nested/hierarchical navigation.

<details>
<summary>Source</summary>

```rust
    fn supports_nested_nav(&self) -> bool {
        true
    }
```

</details>

## Functions

### <span id="fn.python_nav_entries"></span><code style="font-size: 0.85em">fn python_​nav_​entries</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn python_nav_entries(modules: &[PythonModule]) -> Vec<NavEntry>
```

Generate sorted navigation entries for Python modules

<details>
<summary>Source</summary>

```rust
pub fn python_nav_entries(modules: &[PythonModule]) -> Vec<NavEntry> {
    let mut sorted: Vec<&PythonModule> = modules.iter().collect();
    sorted.sort_by(|a, b| a.path.cmp(&b.path));

    sorted
        .iter()
        .map(|m| NavEntry {
            path: m.path.clone(),
            file_path: python_module_page(&m.path),
            depth: m.path.matches('.').count(),
        })
        .collect()
}
```

</details>

### <span id="fn.rust_nav_entries"></span><code style="font-size: 0.85em">fn rust_​nav_​entries</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn rust_nav_entries(modules: &[RustModule]) -> Vec<NavEntry>
```

Generate sorted navigation entries for Rust modules

<details>
<summary>Source</summary>

```rust
pub fn rust_nav_entries(modules: &[RustModule]) -> Vec<NavEntry> {
    let mut sorted: Vec<&RustModule> = modules.iter().collect();
    sorted.sort_by(|a, b| a.path.cmp(&b.path));

    sorted
        .iter()
        .map(|m| NavEntry {
            path: m.path.clone(),
            file_path: rust_module_page(&m.path),
            depth: m.path.matches("::").count(),
        })
        .collect()
}
```

</details>
