# plissken-core::render::ssg::traits <span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #ff5722; color: white;">Rust</span>


SSG adapter trait definition

## Structs

### `plissken-core::render::ssg::traits::NavEntry`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


Entry for a module in the navigation

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `path` | `String` | Module path (dotted for Python, :: for Rust) |
| `file_path` | `PathBuf` | File path for the documentation page |
| `depth` | `usize` | Nesting depth for hierarchical display |



### `plissken-core::render::ssg::traits::NavNode`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


A node in the hierarchical navigation tree.

Leaf nodes have no children and render as simple links.
Branch nodes have children and render as collapsible sections
with their own page as the section index.

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` | Short display name (last segment of the module path) |
| `file_path` | `String` | File path for this node's page (with prefix applied) |
| `children` | `Vec < NavNode >` | Child nodes |

#### Methods

##### `is_branch` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn is_branch (& self) -> bool
```

Whether this node has child nodes (is a section, not a leaf).

<details>
<summary>Source</summary>

```rust
    pub fn is_branch(&self) -> bool {
        !self.children.is_empty()
    }
```

</details>





## Functions

### `plissken-core::render::ssg::traits::prefix_path`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn prefix_path (prefix : Option < & str > , path : & str) -> String
```

Prepend an optional prefix to a file path string. Returns `"prefix/path"` if prefix is Some, or `"path"` unchanged if None.

<details>
<summary>Source</summary>

```rust
pub fn prefix_path(prefix: Option<&str>, path: &str) -> String {
    match prefix {
        Some(p) => format!("{}/{}", p, path),
        None => path.to_string(),
    }
}
```

</details>



### `plissken-core::render::ssg::traits::python_module_page`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


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



### `plissken-core::render::ssg::traits::rust_module_page`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


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



### `plissken-core::render::ssg::traits::python_nav_entries`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


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



### `plissken-core::render::ssg::traits::rust_nav_entries`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


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



### `plissken-core::render::ssg::traits::short_name`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn short_name (full_path : & str , separator : & str) -> String
```

Extract the short display name (last segment) from a module path.

<details>
<summary>Source</summary>

```rust
fn short_name(full_path: &str, separator: &str) -> String {
    full_path
        .rsplit_once(separator)
        .map(|(_, last)| last.to_string())
        .unwrap_or_else(|| full_path.to_string())
}
```

</details>



### `plissken-core::render::ssg::traits::build_nav_tree`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn build_nav_tree (entries : & [NavEntry] , prefix : Option < & str > , separator : & str) -> Vec < NavNode >
```

Build a hierarchical navigation tree from a sorted flat list of entries.

Entries must be sorted alphabetically. The `separator` is `"::"` for Rust
and `"."` for Python. An optional `prefix` is prepended to all file paths.
Modules whose path is a prefix of subsequent entries become branch nodes
(collapsible sections). Modules with no children become leaf nodes.

<details>
<summary>Source</summary>

```rust
pub fn build_nav_tree(entries: &[NavEntry], prefix: Option<&str>, separator: &str) -> Vec<NavNode> {
    let mut nodes = Vec::new();
    let mut i = 0;

    while i < entries.len() {
        let entry = &entries[i];
        let file_path = prefix_path(prefix, &entry.file_path.display().to_string());
        let name = short_name(&entry.path, separator);

        // Collect children: entries whose path starts with this entry's path + separator
        let child_prefix = format!("{}{}", entry.path, separator);
        let mut children_end = i + 1;
        while children_end < entries.len() && entries[children_end].path.starts_with(&child_prefix)
        {
            children_end += 1;
        }

        let children = if children_end > i + 1 {
            build_nav_tree(&entries[i + 1..children_end], prefix, separator)
        } else {
            Vec::new()
        };

        nodes.push(NavNode {
            name,
            file_path,
            children,
        });

        i = children_end;
    }

    nodes
}
```

</details>



