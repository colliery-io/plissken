# plissken-core::render::ssg::mdbook <span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #ff5722; color: white;">Rust</span>


mdBook adapter implementation

## Structs

### `plissken-core::render::ssg::mdbook::MdBookAdapter`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


mdBook adapter.

Generates Markdown SUMMARY.md navigation and book.toml configuration.
Nested modules render as indented entries for collapsible sidebar sections.



## Functions

### `plissken-core::render::ssg::mdbook::render_md_nodes`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn render_md_nodes (nodes : & [NavNode] , indent : usize) -> String
```

Render a list of NavNodes as indented mdBook SUMMARY.md entries.

<details>
<summary>Source</summary>

```rust
fn render_md_nodes(nodes: &[NavNode], indent: usize) -> String {
    let mut md = String::new();
    let pad = "  ".repeat(indent);

    for node in nodes {
        md.push_str(&format!("{}- [{}]({})\n", pad, node.name, node.file_path));
        if node.is_branch() {
            md.push_str(&render_md_nodes(&node.children, indent + 1));
        }
    }

    md
}
```

</details>



