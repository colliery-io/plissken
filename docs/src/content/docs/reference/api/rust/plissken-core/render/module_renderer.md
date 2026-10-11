---
title: "plissken-core::render::module_renderer"
description: "Module page rendering for Python and Rust documentation"
sidebar:
  order: 20
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>


Module page rendering for Python and Rust documentation

This module provides rendering functionality for converting `PythonModule`
and `RustModule` structures into Markdown documentation files.

## Structs

### <span id="struct.RenderedPage"></span>`struct RenderedPage`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


**Derives:** `Debug`, `Clone`

Rendered output for a documentation file

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `path` | `PathBuf` | Relative path for output (e.g., "my_module.md" or "rust/my_crate.md") |
| `content` | `String` | The rendered Markdown content |



### <span id="struct.ModulePageBuilder"></span>`struct ModulePageBuilder`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-gray-6); color: var(--sl-color-gray-2);">private</span>


Builder for constructing module documentation pages.

This provides a common structure for both Python and Rust module pages,
reducing code duplication between `render_python_module_inline` and
`render_rust_module_inline`.

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `content` | `String` |  |

#### Methods

##### <span id="ModulePageBuilder.new"></span>`new`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-gray-6); color: var(--sl-color-gray-2);">private</span>


```rust
fn new () -> Self
```

Create a new page builder

<details>
<summary>Source</summary>

```rust
    fn new() -> Self {
        Self {
            content: String::new(),
        }
    }
```

</details>



##### <span id="ModulePageBuilder.add_header"></span>`add_header`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-gray-6); color: var(--sl-color-gray-2);">private</span>


```rust
fn add_header (& mut self , module_name : & str , badge : & str)
```

Add the module header with a badge

<details>
<summary>Source</summary>

```rust
    fn add_header(&mut self, module_name: &str, badge: &str) {
        self.content
            .push_str(&format!("# {} {}\n\n", module_name, badge));
    }
```

</details>



##### <span id="ModulePageBuilder.add_front_matter"></span>`add_front_matter`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-gray-6); color: var(--sl-color-gray-2);">private</span>


```rust
fn add_front_matter (& mut self , title : & str , description : & str , order : usize , badge : & str)
```

Add Starlight front matter, then the badge as the first line.

Starlight shows the `title` as the page heading, so the page has no
`#` heading of its own.

<details>
<summary>Source</summary>

```rust
    fn add_front_matter(&mut self, title: &str, description: &str, order: usize, badge: &str) {
        self.content
            .push_str(&front_matter(title, description, order));
        self.content.push_str(badge);
        self.content.push_str("\n\n");
    }
```

</details>



##### <span id="ModulePageBuilder.add_docstring"></span>`add_docstring`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-gray-6); color: var(--sl-color-gray-2);">private</span>


```rust
fn add_docstring (& mut self , docstring : & crate :: model :: ParsedDocstring)
```

Add a parsed docstring section

<details>
<summary>Source</summary>

```rust
    fn add_docstring(&mut self, docstring: &crate::model::ParsedDocstring) {
        self.content.push_str(&render_docstring(docstring));
        self.content.push_str("\n\n");
    }
```

</details>



##### <span id="ModulePageBuilder.add_section"></span>`add_section`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-gray-6); color: var(--sl-color-gray-2);">private</span>


```rust
fn add_section (& mut self , title : & str)
```

Add a section header (h2)

<details>
<summary>Source</summary>

```rust
    fn add_section(&mut self, title: &str) {
        self.content.push_str(&format!("## {}\n\n", title));
    }
```

</details>



##### <span id="ModulePageBuilder.add_item"></span>`add_item`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-gray-6); color: var(--sl-color-gray-2);">private</span>


```rust
fn add_item (& mut self , item_content : & str)
```

Add rendered item content with spacing

<details>
<summary>Source</summary>

```rust
    fn add_item(&mut self, item_content: &str) {
        self.content.push_str(item_content);
        self.content.push_str("\n\n");
    }
```

</details>



##### <span id="ModulePageBuilder.add_variables_table"></span>`add_variables_table`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-gray-6); color: var(--sl-color-gray-2);">private</span>


```rust
fn add_variables_table < T , F > (& mut self , title : & str , items : & [T] , row_renderer : F) where F : Fn (& T) -> (String , String , String) ,
```

Add a variables/constants table

<details>
<summary>Source</summary>

```rust
    fn add_variables_table<T, F>(&mut self, title: &str, items: &[T], row_renderer: F)
    where
        F: Fn(&T) -> (String, String, String), // (name, type, desc)
    {
        if items.is_empty() {
            return;
        }
        self.content.push_str(&format!("## {}\n\n", title));
        self.content.push_str("| Name | Type | Description |\n");
        self.content.push_str("|------|------|-------------|\n");
        for item in items {
            let (name, ty, desc) = row_renderer(item);
            self.content
                .push_str(&format!("| `{}` | `{}` | {} |\n", name, ty, desc));
        }
        self.content.push('\n');
    }
```

</details>



##### <span id="ModulePageBuilder.build"></span>`build`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-gray-6); color: var(--sl-color-gray-2);">private</span>


```rust
fn build (self) -> String
```

Build the final content

<details>
<summary>Source</summary>

```rust
    fn build(self) -> String {
        self.content
    }
```

</details>





### <span id="struct.ModuleRenderer"></span>`struct ModuleRenderer<'a>`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


Module page renderer that converts DocModel modules into Markdown files.

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `renderer` | `& 'a Renderer` |  |
| `linker` | `CrossRefLinker` |  |
| `starlight` | `bool` | Write pages for an Astro Starlight site |
| `module_order` | `HashMap < String , usize >` | Sidebar order of each module page (Starlight front matter) |



