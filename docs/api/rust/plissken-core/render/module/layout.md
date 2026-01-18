# layout <span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #ff5722; color: white;">Rust</span>


Page layout and path computation for documentation output.

This module handles the translation of module/item names to file paths,
providing a consistent mapping between documentation structure and
filesystem organization.

## Structs

### `struct PageLayout`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Clone`, `Default`

Computes file paths and directory structure for documentation pages.

`PageLayout` encapsulates the rules for translating Python and Rust
module paths into filesystem paths for the generated documentation.

**Examples:**

```rust
use plissken_core::render::module::PageLayout;
use std::path::PathBuf;

let layout = PageLayout::new();

// Python module paths
assert_eq!(
    layout.python_module_dir("mypackage.submodule"),
    "python/mypackage/submodule"
);
assert_eq!(
    layout.python_index_path("mypackage.submodule"),
    PathBuf::from("python/mypackage/submodule/index.md")
);
assert_eq!(
    layout.python_item_path("mypackage.submodule", "MyClass"),
    PathBuf::from("python/mypackage/submodule/MyClass.md")
);

// Rust module paths
assert_eq!(
    layout.rust_module_dir("mycrate::submod"),
    "rust/mycrate/submod"
);
```

#### Methods

##### `new` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn new () -> Self
```

Create a new PageLayout.

<details>
<summary>Source</summary>

```rust
    pub fn new() -> Self {
        Self
    }
```

</details>



##### `python_module_dir` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn python_module_dir (& self , module_path : & str) -> String
```

Compute the directory path for a Python module.

Converts dots to slashes and prepends "python/".

**Examples:**

```rust
use plissken_core::render::module::PageLayout;

let layout = PageLayout::new();
assert_eq!(layout.python_module_dir("mypackage.sub"), "python/mypackage/sub");
```

<details>
<summary>Source</summary>

```rust
    pub fn python_module_dir(&self, module_path: &str) -> String {
        let parts: Vec<&str> = module_path.split('.').collect();
        format!("python/{}", parts.join("/"))
    }
```

</details>



##### `python_index_path` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn python_index_path (& self , module_path : & str) -> PathBuf
```

Compute the path for a Python module's index page.

<details>
<summary>Source</summary>

```rust
    pub fn python_index_path(&self, module_path: &str) -> PathBuf {
        PathBuf::from(format!("{}/index.md", self.python_module_dir(module_path)))
    }
```

</details>



##### `python_module_page` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn python_module_page (& self , module_path : & str) -> PathBuf
```

Compute the path for a Python module as a single page (inline format).

For top-level modules, returns just `{module}.md`.
For nested modules, returns `{parent}/{module}.md`.

<details>
<summary>Source</summary>

```rust
    pub fn python_module_page(&self, module_path: &str) -> PathBuf {
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



##### `python_item_path` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn python_item_path (& self , module_path : & str , item_name : & str) -> PathBuf
```

Compute the path for a Python item (class, function) page.

<details>
<summary>Source</summary>

```rust
    pub fn python_item_path(&self, module_path: &str, item_name: &str) -> PathBuf {
        PathBuf::from(format!(
            "{}/{}.md",
            self.python_module_dir(module_path),
            item_name
        ))
    }
```

</details>



##### `python_module_depth` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn python_module_depth (& self , module_path : & str) -> usize
```

Compute the depth of a Python module (number of dots).

Used for indentation in navigation.

<details>
<summary>Source</summary>

```rust
    pub fn python_module_depth(&self, module_path: &str) -> usize {
        module_path.matches('.').count()
    }
```

</details>



##### `rust_module_dir` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn rust_module_dir (& self , module_path : & str) -> String
```

Compute the directory path for a Rust module.

Converts `::` to `/` and prepends "rust/".

**Examples:**

```rust
use plissken_core::render::module::PageLayout;

let layout = PageLayout::new();
assert_eq!(layout.rust_module_dir("mycrate::submod"), "rust/mycrate/submod");
```

<details>
<summary>Source</summary>

```rust
    pub fn rust_module_dir(&self, module_path: &str) -> String {
        let parts: Vec<&str> = module_path.split("::").collect();
        format!("rust/{}", parts.join("/"))
    }
```

</details>



##### `rust_index_path` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn rust_index_path (& self , module_path : & str) -> PathBuf
```

Compute the path for a Rust module's index page.

<details>
<summary>Source</summary>

```rust
    pub fn rust_index_path(&self, module_path: &str) -> PathBuf {
        PathBuf::from(format!("{}/index.md", self.rust_module_dir(module_path)))
    }
```

</details>



##### `rust_module_page` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn rust_module_page (& self , module_path : & str) -> PathBuf
```

Compute the path for a Rust module as a single page (inline format).

For crate roots (no `::` in path), returns `rust/{crate_name}.md`.
For submodules, returns `rust/{crate_name}/{submodule}.md`.

**Examples:**

```python
- `plissken_core` → `rust/plissken_core.md` (crate root)
- `plissken_core::config` → `rust/plissken_core/config.md` (submodule)
- `plissken_core::render::ssg` → `rust/plissken_core/render/ssg.md` (nested)
```

<details>
<summary>Source</summary>

```rust
    pub fn rust_module_page(&self, module_path: &str) -> PathBuf {
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



##### `rust_item_path` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn rust_item_path (& self , module_path : & str , item_name : & str) -> PathBuf
```

Compute the path for a Rust item (struct, enum, function) page.

<details>
<summary>Source</summary>

```rust
    pub fn rust_item_path(&self, module_path: &str, item_name: &str) -> PathBuf {
        PathBuf::from(format!(
            "{}/{}.md",
            self.rust_module_dir(module_path),
            item_name
        ))
    }
```

</details>



##### `rust_module_depth` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn rust_module_depth (& self , module_path : & str) -> usize
```

Compute the depth of a Rust module (number of `::` separators).

Used for indentation in navigation.

<details>
<summary>Source</summary>

```rust
    pub fn rust_module_depth(&self, module_path: &str) -> usize {
        module_path.matches("::").count()
    }
```

</details>



##### `python_link_from_rust` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn python_link_from_rust (& self , python_module : & str , item_name : & str , anchor : Option < & str > ,) -> String
```

Compute a relative path from a Rust module to a Python item.

Used for cross-reference links between Rust and Python documentation.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `python_module` | `-` | The Python module path (e.g., "mypackage.submodule") |
| `item_name` | `-` | The item name (e.g., "MyClass") |
| `anchor` | `-` | Optional anchor within the page (e.g., "my_method") |


<details>
<summary>Source</summary>

```rust
    pub fn python_link_from_rust(
        &self,
        python_module: &str,
        item_name: &str,
        anchor: Option<&str>,
    ) -> String {
        let python_dir = python_module.replace('.', "/");
        let prefix = "../".repeat(2); // rust/crate/file.md -> ../../python/...
        match anchor {
            Some(a) => format!("{}python/{}/{}.md#{}", prefix, python_dir, item_name, a),
            None => format!("{}python/{}/{}.md", prefix, python_dir, item_name),
        }
    }
```

</details>



##### `rust_link_from_python` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn rust_link_from_python (& self , rust_module : & str , item_name : & str , anchor : Option < & str > ,) -> String
```

Compute a relative path from a Python module to a Rust item.

Used for cross-reference links between Python and Rust documentation.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `rust_module` | `-` | The Rust module path (e.g., "mycrate::submod") |
| `item_name` | `-` | The item name (e.g., "MyStruct") |
| `anchor` | `-` | Optional anchor within the page (e.g., "my_method") |


<details>
<summary>Source</summary>

```rust
    pub fn rust_link_from_python(
        &self,
        rust_module: &str,
        item_name: &str,
        anchor: Option<&str>,
    ) -> String {
        let rust_dir = rust_module.replace("::", "/");
        let prefix = "../".repeat(2); // python/pkg/file.md -> ../../rust/...
        match anchor {
            Some(a) => format!("{}rust/{}/{}.md#{}", prefix, rust_dir, item_name, a),
            None => format!("{}rust/{}/{}.md", prefix, rust_dir, item_name),
        }
    }
```

</details>





