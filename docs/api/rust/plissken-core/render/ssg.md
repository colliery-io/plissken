# ssg <span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #ff5722; color: white;">Rust</span>


Static Site Generator (SSG) adapters

This module provides adapters for different static site generators,
abstracting away SSG-specific differences in navigation format,
config files, and directory structure.

**Examples:**

```rust
use plissken_core::render::ssg::{SSGAdapter, get_ssg_adapter};

let adapter = get_ssg_adapter(Some("mkdocs-material"));
assert_eq!(adapter.name(), "mkdocs");
assert_eq!(adapter.content_dir(), "docs");
assert_eq!(adapter.nav_filename(), "_nav.yml");
```

## Functions

### `fn get_ssg_adapter`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn get_ssg_adapter (template : Option < & str >) -> Box < dyn SSGAdapter >
```

Get an SSG adapter for the given template name.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `template` | `-` | Optional template name. If None or unrecognized, defaults to MkDocs. |


**Examples:**

```rust
use plissken_core::render::ssg::get_ssg_adapter;

let mkdocs = get_ssg_adapter(Some("mkdocs-material"));
assert_eq!(mkdocs.name(), "mkdocs");

let mdbook = get_ssg_adapter(Some("mdbook"));
assert_eq!(mdbook.name(), "mdbook");

let default = get_ssg_adapter(None);
assert_eq!(default.name(), "mkdocs");
```

<details>
<summary>Source</summary>

```rust
pub fn get_ssg_adapter(template: Option<&str>) -> Box<dyn SSGAdapter> {
    match template.map(|s| s.to_lowercase()).as_deref() {
        Some("mdbook") | Some("md-book") | Some("md_book") => Box::new(MdBookAdapter),
        _ => Box::new(MkDocsAdapter),
    }
}
```

</details>



