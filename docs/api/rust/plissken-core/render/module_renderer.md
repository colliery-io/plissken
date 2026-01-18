# module_renderer <span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #ff5722; color: white;">Rust</span>


Module page rendering for Python and Rust documentation

This module provides rendering functionality for converting `PythonModule`
and `RustModule` structures into Markdown documentation files.

## Structs

### `struct RenderedPage`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Clone`

Rendered output for a documentation file

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `path` | `PathBuf` | Relative path for output (e.g., "my_module.md" or "rust/my_crate.md") |
| `content` | `String` | The rendered Markdown content |



### `struct ModulePageBuilder`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


Builder for constructing module documentation pages.

This provides a common structure for both Python and Rust module pages,
reducing code duplication between `render_python_module_inline` and
`render_rust_module_inline`.

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `content` | `String` |  |

#### Methods

##### `new` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


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



##### `add_header` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn add_header (& mut self , module_name : & str , badge : & str)
```

Add the module header with a badge

<details>
<summary>Source</summary>

```rust
    fn add_header(&mut self, module_name: &str, badge: &str) {
        self.content.push_str(&format!("# {} {}\n\n", module_name, badge));
    }
```

</details>



##### `add_docstring` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


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



##### `add_section` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


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



##### `add_item` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


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



##### `add_variables_table` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


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
            self.content.push_str(&format!("| `{}` | `{}` | {} |\n", name, ty, desc));
        }
        self.content.push('\n');
    }
```

</details>



##### `build` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


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





### `struct ModuleRenderer`<'a>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


Module page renderer that converts DocModel modules into Markdown files.

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `renderer` | `& 'a Renderer` |  |
| `linker` | `CrossRefLinker` |  |



