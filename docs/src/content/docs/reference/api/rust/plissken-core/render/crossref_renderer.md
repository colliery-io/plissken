---
title: "plissken-core::render::crossref_renderer"
description: "Cross-reference link generation for Python-Rust documentation"
editUrl: false
sidebar:
  order: 17
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>

Cross-reference link generation for Python-Rust documentation

This module provides utilities for generating Markdown links between
Python items and their Rust implementations, and vice versa.

## Structs

### <span id="struct.CrossRefLink"></span><code style="font-size: 0.85em">struct CrossRefLink</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`

Represents a generated cross-reference link

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `text` | `String` | Display text for the link |
| `url` | `String` | Relative URL path |
| `relationship` | `CrossRefKind` | Type of cross-reference relationship |

#### Methods

##### <span id="CrossRefLink.to_markdown"></span><code>to_​markdown</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn to_markdown(&self) -> String
```

Render as a Markdown link

<details>
<summary>Source</summary>

```rust
    pub fn to_markdown(&self) -> String {
        format!("[{}]({})", self.text, self.url)
    }
```

</details>

##### <span id="CrossRefLink.to_markdown_with_badge"></span><code>to_​markdown_​with_​badge</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn to_markdown_with_badge(&self) -> String
```

Render as a Markdown link with relationship indicator

<details>
<summary>Source</summary>

```rust
    pub fn to_markdown_with_badge(&self) -> String {
        let indicator = match self.relationship {
            CrossRefKind::Binding => "[binding]",
            CrossRefKind::Wraps => "[wraps]",
            CrossRefKind::Delegates => "[delegates]",
        };
        format!("{} [{}]({})", indicator, self.text, self.url)
    }
```

</details>

## Enums

### <span id="enum.Language"></span><code style="font-size: 0.85em">enum Language</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

Language enum for determining link direction

#### Variants

- **`Python`**
- **`Rust`**

## Functions

### <span id="fn.link_to_rust"></span><code style="font-size: 0.85em">fn link_​to_​rust</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn link_to_rust(
    rust_ref: &RustItemRef,
    from_python_path: &str,
) -> String
```

Generate a relative Markdown link from a Python page to a Rust item.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `rust_ref` | `&RustItemRef` | Reference to the Rust item |
| `from_python_path` | `&str` | The Python module path (e.g., "mypackage.submodule") |

**Returns:**

A Markdown link string like `[RustStruct](../rust/crate/module.md#struct-ruststruct)`

**Examples:**

```rust
use plissken_core::model::RustItemRef;
use plissken_core::render::link_to_rust;

let rust_ref = RustItemRef::new("crate::utils", "Config");
let link = link_to_rust(&rust_ref, "mypackage.config");
assert!(link.contains("rust/crate/utils.md"));
assert!(link.contains("#struct-config"));
```

<details>
<summary>Source</summary>

```rust
pub fn link_to_rust(rust_ref: &RustItemRef, from_python_path: &str) -> String {
    let rust_page_path = rust_path_to_file_path(&rust_ref.path);
    let anchor = item_to_anchor(&rust_ref.name, "struct");
    // from_python_path is now python/module, rust is rust/module
    let from_path = format!("python/{}", from_python_path.replace('.', "/"));
    let relative_path = compute_relative_path(&from_path, &format!("rust/{}", rust_page_path));

    format!("[{}]({}#{})", rust_ref.name, relative_path, anchor)
}
```

</details>

### <span id="fn.link_to_python"></span><code style="font-size: 0.85em">fn link_​to_​python</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn link_to_python(python_path: &str, from_rust_path: &str) -> String
```

Generate a relative Markdown link from a Rust page to a Python item.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `python_path` | `&str` | Full Python path (e.g., "mypackage.module.ClassName") |
| `from_rust_path` | `&str` | The Rust module path (e.g., "crate::utils") |

**Returns:**

A Markdown link string like `[ClassName](../../mypackage/module.md#class-classname)`

**Examples:**

```rust
use plissken_core::render::link_to_python;

let link = link_to_python("mypackage.utils.Config", "crate::utils");
assert!(link.contains("mypackage/utils.md"));
assert!(link.contains("#class-config"));
```

<details>
<summary>Source</summary>

```rust
pub fn link_to_python(python_path: &str, from_rust_path: &str) -> String {
    let (module_path, item_name) = split_python_path(python_path);
    let python_page_path = python_path_to_file_path(&module_path);
    let rust_page_path = rust_path_to_file_path(from_rust_path);
    // python_page_path now includes python/ prefix
    let relative_path =
        compute_relative_path(&format!("rust/{}", rust_page_path), &python_page_path);
    let anchor = item_to_anchor(&item_name, "class");

    format!("[{}]({}#{})", item_name, relative_path, anchor)
}
```

</details>

### <span id="fn.crossref_link"></span><code style="font-size: 0.85em">fn crossref_​link</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn crossref_link(
    xref: &CrossRef,
    from_path: &str,
    from_language: Language,
) -> CrossRefLink
```

Generate a cross-reference link based on the CrossRef relationship.

Returns a tuple of (link_text, link_url, relationship_badge).

`from_path` should be the module path without python/ or rust/ prefix.

<details>
<summary>Source</summary>

```rust
pub fn crossref_link(xref: &CrossRef, from_path: &str, from_language: Language) -> CrossRefLink {
    match from_language {
        Language::Python => {
            // Generate link from Python to Rust
            let (module_path, item_name) = split_rust_path(&xref.rust_path);
            let rust_page = rust_path_to_file_path(&module_path);
            // from_path is Python module path, needs python/ prefix
            let from_full = format!("python/{}", from_path.replace('.', "/"));
            let relative = compute_relative_path(&from_full, &format!("rust/{}", rust_page));
            let anchor = item_to_anchor(&item_name, "struct");

            CrossRefLink {
                text: item_name,
                url: format!("{}#{}", relative, anchor),
                relationship: xref.relationship.clone(),
            }
        }
        Language::Rust => {
            // Generate link from Rust to Python
            let (module_path, item_name) = split_python_path(&xref.python_path);
            let python_page = python_path_to_file_path(&module_path);
            let rust_page = rust_path_to_file_path(from_path);
            let relative = compute_relative_path(&format!("rust/{}", rust_page), &python_page);
            let anchor = item_to_anchor(&item_name, "class");

            CrossRefLink {
                text: item_name,
                url: format!("{}#{}", relative, anchor),
                relationship: xref.relationship.clone(),
            }
        }
    }
}
```

</details>

### <span id="fn.render_rust_impl_details"></span><code style="font-size: 0.85em">fn render_​rust_​impl_​details</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn render_rust_impl_details(
    rust_ref: &RustItemRef,
    from_python_path: &str,
) -> String
```

Render a collapsible details block with a link to the Rust implementation.

This is an enhanced version that includes an actual clickable link.

<details>
<summary>Source</summary>

```rust
pub fn render_rust_impl_details(rust_ref: &RustItemRef, from_python_path: &str) -> String {
    let link = link_to_rust(rust_ref, from_python_path);

    format!(
        "<details>\n\
         <summary>Rust Implementation</summary>\n\n\
         Implemented by {} in `{}`\n\n\
         </details>",
        link, rust_ref.path
    )
}
```

</details>

### <span id="fn.render_python_exposure_details"></span><code style="font-size: 0.85em">fn render_​python_​exposure_​details</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn render_python_exposure_details(
    python_path: &str,
    from_rust_path: &str,
) -> String
```

Render a collapsible details block with a link to the Python exposure.

For Rust items that are exposed to Python.

<details>
<summary>Source</summary>

```rust
pub fn render_python_exposure_details(python_path: &str, from_rust_path: &str) -> String {
    let link = link_to_python(python_path, from_rust_path);

    format!(
        "<details>\n\
         <summary>Python API</summary>\n\n\
         Exposed as {} in `{}`\n\n\
         </details>",
        link, python_path
    )
}
```

</details>
