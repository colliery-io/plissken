# plissken-core::render::ssg::mkdocs <span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #ff5722; color: white;">Rust</span>


MkDocs adapter implementation

## Structs

### `plissken-core::render::ssg::mkdocs::MkDocsAdapter`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


MkDocs adapter for Material theme.

Generates hierarchical YAML navigation with collapsible sections
for nested modules. Works with MkDocs Material's `navigation.indexes`
feature to show section index pages.



## Functions

### `plissken-core::render::ssg::mkdocs::render_yaml_nodes`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn render_yaml_nodes (nodes : & [NavNode] , indent : usize) -> String
```

Render a list of NavNodes as nested MkDocs YAML.

<details>
<summary>Source</summary>

```rust
fn render_yaml_nodes(nodes: &[NavNode], indent: usize) -> String {
    let mut yaml = String::new();
    let pad = "  ".repeat(indent);

    for node in nodes {
        if node.is_branch() {
            // Section with collapsible children
            yaml.push_str(&format!("{}- {}:\n", pad, node.name));
            // Section index page (works with navigation.indexes)
            yaml.push_str(&format!("{}  - {}\n", pad, node.file_path));
            yaml.push_str(&render_yaml_nodes(&node.children, indent + 1));
        } else {
            // Leaf entry
            yaml.push_str(&format!("{}- {}: {}\n", pad, node.name, node.file_path));
        }
    }

    yaml
}
```

</details>



