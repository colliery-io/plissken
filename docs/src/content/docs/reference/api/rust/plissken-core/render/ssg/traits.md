---
title: "plissken-core::render::ssg::traits"
description: "SSG adapter trait definition"
sidebar:
  order: 26
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>


SSG adapter trait definition

## Structs

### <span id="struct.NavEntry"></span>`struct NavEntry`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


Entry for a module in the navigation

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `path` | `String` | Module path (dotted for Python, :: for Rust) |
| `file_path` | `PathBuf` | File path for the documentation page |
| `depth` | `usize` | Nesting depth for hierarchical display |



## Functions

### <span id="fn.python_module_page"></span>`fn python_module_page`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-gray-6); color: var(--sl-color-gray-2);">private</span>


```rust
fn python_module_page (module_path : & str) -> PathBuf
```

Compute the file path for a Python module page (inline format).

For top-level modules, returns just `{module}.md`.
For nested modules, returns `{parent}/{module}.md`.

<details>
<summary>Source</summary>

```rust
fn python_module_page(module_path: &str) -> PathBuf {
    let parts: Vec<&str> = module_path.split('.').collect();
    if parts.len() == 1 {
        // Top-level module: just module_name.md
        PathBuf::from(format!("{}.md", parts[0]))
    } else {
        // Nested module: parent/child.md
        let last = parts.last().unwrap();
        let parent_parts = &parts[..parts.len() - 1];
        PathBuf::from(format!("{}/{}.md", parent_parts.join("/"), last))
    }
}
```

</details>



### <span id="fn.rust_module_page"></span>`fn rust_module_page`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-gray-6); color: var(--sl-color-gray-2);">private</span>


```rust
fn rust_module_page (module_path : & str) -> PathBuf
```

Compute the file path for a Rust module page (inline format).

For crate roots (no `::` in path), returns `rust/{crate_name}.md`.
For submodules, returns `rust/{crate_name}/{submodule}.md`.

<details>
<summary>Source</summary>

```rust
fn rust_module_page(module_path: &str) -> PathBuf {
    if !module_path.contains("::") {
        // Crate root - use crate_name.md directly
        PathBuf::from(format!("rust/{}.md", module_path))
    } else {
        // Submodule - convert :: to /
        let path = module_path.replace("::", "/");
        PathBuf::from(format!("rust/{}.md", path))
    }
}
```

</details>



### <span id="fn.python_nav_entries"></span>`fn python_nav_entries`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


```rust
fn python_nav_entries (modules : & [PythonModule]) -> Vec < NavEntry >
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



### <span id="fn.rust_nav_entries"></span>`fn rust_nav_entries`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


```rust
fn rust_nav_entries (modules : & [RustModule]) -> Vec < NavEntry >
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



