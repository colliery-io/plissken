---
title: "plissken-core::render::module_renderer"
description: "Module page rendering for Python and Rust documentation"
editUrl: false
sidebar:
  order: 23
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>

Module page rendering for Python and Rust documentation

This module provides rendering functionality for converting `PythonModule`
and `RustModule` structures into Markdown documentation files.

## Structs

### <span id="struct.RenderedPage"></span><code style="font-size: 0.85em">struct RenderedPage</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`

Rendered output for a documentation file

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `path` | `PathBuf` | Relative path for output (e.g., "my_module.md" or "rust/my_crate.md") |
| `content` | `String` | The rendered Markdown content |

### <span id="struct.ModuleRenderer"></span><code style="font-size: 0.85em">struct ModuleRenderer&lt;&#39;a&gt;</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

Module page renderer that converts DocModel modules into Markdown files.

#### Methods

##### <span id="ModuleRenderer.new"></span><code>new</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn new(renderer: &'a Renderer) -> Self
```

Create a new module renderer

<details>
<summary>Source</summary>

```rust
    pub fn new(renderer: &'a Renderer) -> Self {
        Self::with_cross_refs(renderer, Vec::new())
    }
```

</details>

##### <span id="ModuleRenderer.with_cross_refs"></span><code>with_​cross_​refs</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_cross_refs(
    renderer: &'a Renderer,
    cross_refs: Vec<CrossRef>,
) -> Self
```

Create a module renderer with cross-references for bi-directional linking

<details>
<summary>Source</summary>

```rust
    pub fn with_cross_refs(renderer: &'a Renderer, cross_refs: Vec<CrossRef>) -> Self {
        let starlight = renderer.is_starlight();
        let style = if starlight {
            LinkStyle::Route
        } else {
            LinkStyle::MarkdownFile
        };
        Self {
            renderer,
            linker: CrossRefLinker::new(cross_refs).with_style(style),
            starlight,
            module_order: HashMap::new(),
            rust_index: RustDocIndex::default(),
        }
    }
```

</details>

##### <span id="ModuleRenderer.with_module_order"></span><code>with_​module_​order</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_module_order(
    mut self,
    python_modules: &[PythonModule],
    rust_modules: &[RustModule],
) -> Self
```

Set the sidebar order of the module pages from all modules of the project: Python modules first, then Rust modules, each sorted by path (the order of the navigation). Used in Starlight front matter.

<details>
<summary>Source</summary>

```rust
    pub fn with_module_order(
        mut self,
        python_modules: &[PythonModule],
        rust_modules: &[RustModule],
    ) -> Self {
        let python = python_nav_entries(python_modules)
            .into_iter()
            .map(|e| format!("py:{}", e.path));
        let rust = rust_nav_entries(rust_modules)
            .into_iter()
            .map(|e| format!("rs:{}", e.path));
        self.module_order = python
            .chain(rust)
            .enumerate()
            .map(|(i, key)| (key, i + 1))
            .collect();
        self
    }
```

</details>

##### <span id="ModuleRenderer.with_rust_index"></span><code>with_​rust_​index</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_rust_index(mut self, rust_modules: &[RustModule]) -> Self
```

Index the Rust items of the project, so that the intra-doc links of Rust doc comments (``[`Config`]``, `[x](crate::config::Config)`) point to their pages. Without it, such a link shows as its text.

<details>
<summary>Source</summary>

```rust
    pub fn with_rust_index(mut self, rust_modules: &[RustModule]) -> Self {
        let layout = PageLayout::new();
        let index = RustDocIndex::new(rust_modules, |m| self.page_file(layout.rust_module_page(m)));
        self.rust_index = index;
        self
    }
```

</details>

##### <span id="ModuleRenderer.render_starlight_index"></span><code>render_​starlight_​index</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn render_starlight_index(
    &self,
    python_modules: &[PythonModule],
    rust_modules: &[RustModule],
) -> RenderedPage
```

The index page of a Starlight API reference: each module, linked to its page, with the summary of its doc comment.

<details>
<summary>Source</summary>

```rust
    pub fn render_starlight_index(
        &self,
        python_modules: &[PythonModule],
        rust_modules: &[RustModule],
    ) -> RenderedPage {
        let index = STARLIGHT_INDEX_FILE;
        let mut content = front_matter(
            "API reference",
            "The API reference, written by plissken from the source.",
            0,
        );
        let link = |page: PathBuf, label: &str, summary: Option<String>| {
            let href = route_href(index, &self.page_file(page), None);
            match summary
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
            {
                Some(s) => format!("- [`{}`]({}): {}", label, href, s),
                None => format!("- [`{}`]({})", label, href),
            }
        };
        let layout = PageLayout::new();
        if !python_modules.is_empty() {
            content.push_str("## Python\n\n");
            let mut modules: Vec<&PythonModule> = python_modules.iter().collect();
            modules.sort_by(|a, b| a.path.cmp(&b.path));
            for m in modules {
                let summary = m.docstring.as_deref().and_then(|d| {
                    if m.source_type == SourceType::PyO3Binding {
                        parse_rust_doc(d).summary
                    } else {
                        parse_docstring(d).summary
                    }
                });
                content.push_str(&link(layout.python_module_page(&m.path), &m.path, summary));
                content.push('\n');
            }
            content.push('\n');
        }
        if !rust_modules.is_empty() {
            content.push_str("## Rust\n\n");
            let mut modules: Vec<&RustModule> = rust_modules.iter().collect();
            modules.sort_by(|a, b| a.path.cmp(&b.path));
            for m in modules {
                let summary = m
                    .doc_comment
                    .as_deref()
                    .and_then(|d| parse_rust_doc(d).summary);
                let summary = summary.map(|s| self.rust_doc_links(&s, index, &m.path));
                content.push_str(&link(layout.rust_module_page(&m.path), &m.path, summary));
                content.push('\n');
            }
        }
        RenderedPage {
            path: PathBuf::from(index),
            content: self.finish_page(content),
        }
    }
```

</details>

##### <span id="ModuleRenderer.render_python_module"></span><code>render_​python_​module</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn render_python_module(
    &self,
    module: &PythonModule,
) -> Result<Vec<RenderedPage>, tera::Error>
```

Render a Python module to a single Markdown file with all content inline.

This creates one file per module with classes, methods, and functions
all rendered inline rather than as separate pages.

<details>
<summary>Source</summary>

```rust
    pub fn render_python_module(
        &self,
        module: &PythonModule,
    ) -> Result<Vec<RenderedPage>, tera::Error> {
        // Separate classes and functions
        let mut classes = Vec::new();
        let mut functions = Vec::new();
        let mut variables = Vec::new();

        for item in &module.items {
            match item {
                PythonItem::Class(c) => classes.push(c),
                PythonItem::Function(f) => functions.push(f),
                PythonItem::Variable(v) => variables.push(v),
            }
        }

        // Render everything into a single page
        let page = self.render_python_module_inline(module, &classes, &functions, &variables)?;

        Ok(vec![page])
    }
```

</details>

##### <span id="ModuleRenderer.render_rust_module"></span><code>render_​rust_​module</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn render_rust_module(
    &self,
    module: &RustModule,
) -> Result<Vec<RenderedPage>, tera::Error>
```

Render a Rust module to a single Markdown file with all content inline.

This creates one file per module with structs, enums, and functions
all rendered inline rather than as separate pages.

<details>
<summary>Source</summary>

```rust
    pub fn render_rust_module(
        &self,
        module: &RustModule,
    ) -> Result<Vec<RenderedPage>, tera::Error> {
        // Categorize items
        let mut structs = Vec::new();
        let mut enums = Vec::new();
        let mut functions = Vec::new();
        let mut consts = Vec::new();
        let mut traits = Vec::new();
        let mut type_aliases = Vec::new();
        let mut impls: HashMap<String, Vec<&RustImpl>> = HashMap::new();

        for item in &module.items {
            match item {
                RustItem::Struct(s) => structs.push(s),
                RustItem::Enum(e) => enums.push(e),
                RustItem::Function(f) => functions.push(f),
                RustItem::Const(c) => consts.push(c),
                RustItem::Trait(t) => traits.push(t),
                RustItem::TypeAlias(t) => type_aliases.push(t),
                RustItem::Impl(i) => {
                    // `Foo<T>` has the methods of struct `Foo`
                    let name = i.target.split('<').next().unwrap_or(&i.target).trim();
                    impls.entry(name.to_string()).or_default().push(i);
                }
            }
        }

        // Render everything into a single page
        let items = RustModuleItems {
            structs: &structs,
            enums: &enums,
            functions: &functions,
            consts: &consts,
            traits: &traits,
            type_aliases: &type_aliases,
            impls: &impls,
        };
        let page = self.render_rust_module_inline(module, &items)?;

        Ok(vec![page])
    }
```

</details>

##### <span id="ModuleRenderer.render_python_modules"></span><code>render_​python_​modules</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn render_python_modules(
    &self,
    modules: &[PythonModule],
) -> Result<Vec<RenderedPage>, tera::Error>
```

Render all Python modules from a list

<details>
<summary>Source</summary>

```rust
    pub fn render_python_modules(
        &self,
        modules: &[PythonModule],
    ) -> Result<Vec<RenderedPage>, tera::Error> {
        let mut all_pages = Vec::new();
        for module in modules {
            let pages = self.render_python_module(module)?;
            all_pages.extend(pages);
        }
        Ok(all_pages)
    }
```

</details>

##### <span id="ModuleRenderer.render_rust_modules"></span><code>render_​rust_​modules</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn render_rust_modules(
    &self,
    modules: &[RustModule],
) -> Result<Vec<RenderedPage>, tera::Error>
```

Render all Rust modules from a list

<details>
<summary>Source</summary>

```rust
    pub fn render_rust_modules(
        &self,
        modules: &[RustModule],
    ) -> Result<Vec<RenderedPage>, tera::Error> {
        let mut all_pages = Vec::new();
        for module in modules {
            let pages = self.render_rust_module(module)?;
            all_pages.extend(pages);
        }
        Ok(all_pages)
    }
```

</details>

##### <span id="ModuleRenderer.generate_nav"></span><code>generate_​nav</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn generate_nav(
    &self,
    adapter: &dyn super::ssg::SSGAdapter,
    python_modules: &[PythonModule],
    rust_modules: &[RustModule],
) -> String
```

Generate navigation using an SSG adapter.

This method provides a unified interface for generating navigation
regardless of the target SSG. Use [`super::ssg::get_ssg_adapter`](../../../../rust/plissken-core/render/ssg/#fn.get_ssg_adapter) to
get the appropriate adapter for your template.

**Examples:**

```rust
use plissken_core::render::ssg::get_ssg_adapter;

let adapter = get_ssg_adapter(Some("mkdocs-material"));
let nav = module_renderer.generate_nav(&adapter, &python_modules, &rust_modules);
```

<details>
<summary>Source</summary>

```rust
    pub fn generate_nav(
        &self,
        adapter: &dyn super::ssg::SSGAdapter,
        python_modules: &[PythonModule],
        rust_modules: &[RustModule],
    ) -> String {
        adapter.generate_nav(python_modules, rust_modules)
    }
```

</details>

##### <span id="ModuleRenderer.generate_config"></span><code>generate_​config</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn generate_config(
    &self,
    adapter: &dyn super::ssg::SSGAdapter,
    title: &str,
    authors: &[String],
) -> Option<String>
```

Generate SSG config file using an adapter.

Returns `None` for SSGs that don't need generated config (e.g., MkDocs).

<details>
<summary>Source</summary>

```rust
    pub fn generate_config(
        &self,
        adapter: &dyn super::ssg::SSGAdapter,
        title: &str,
        authors: &[String],
    ) -> Option<String> {
        adapter.generate_config(title, authors)
    }
```

</details>

##### <span id="ModuleRenderer.generate_custom_css"></span><code>generate_​custom_​css</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn generate_custom_css(
    &self,
    adapter: &dyn super::ssg::SSGAdapter,
) -> Option<String>
```

Generate custom CSS using an adapter.

Returns `None` for SSGs that don't need custom CSS.

<details>
<summary>Source</summary>

```rust
    pub fn generate_custom_css(&self, adapter: &dyn super::ssg::SSGAdapter) -> Option<String> {
        adapter.generate_custom_css()
    }
```

</details>

##### <span id="ModuleRenderer.generate_nav_yaml"></span><code>generate_​nav_​yaml</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn generate_nav_yaml(
    &self,
    python_modules: &[PythonModule],
    rust_modules: &[RustModule],
) -> String
```

Generate navigation YAML for mkdocs.yml

Creates a nav structure with inline format (one file per module):
```yaml
nav:
  - Python:
    - pysnake: pysnake.md
  - Rust:
    - rustscale: rust/rustscale.md
    - rustscale::config: rust/rustscale/config.md
```

<details>
<summary>Source</summary>

```rust
    pub fn generate_nav_yaml(
        &self,
        python_modules: &[PythonModule],
        rust_modules: &[RustModule],
    ) -> String {
        let mut yaml = String::new();

        // Add recommended toc setting
        yaml.push_str("# Recommended: add to markdown_extensions in mkdocs.yml\n");
        yaml.push_str("# to hide method-level entries from the table of contents:\n");
        yaml.push_str("#\n");
        yaml.push_str("# markdown_extensions:\n");
        yaml.push_str("#   - toc:\n");
        yaml.push_str("#       toc_depth: 2\n");
        yaml.push('\n');

        yaml.push_str("nav:\n");

        // Python section
        if !python_modules.is_empty() {
            yaml.push_str("  - Python:\n");
            for entry in python_nav_entries(python_modules) {
                yaml.push_str(&format!(
                    "    - {}: {}\n",
                    entry.path,
                    entry.file_path.display()
                ));
            }
        }

        // Rust section
        if !rust_modules.is_empty() {
            yaml.push_str("  - Rust:\n");
            for entry in rust_nav_entries(rust_modules) {
                yaml.push_str(&format!(
                    "    - {}: {}\n",
                    entry.path,
                    entry.file_path.display()
                ));
            }
        }

        yaml
    }
```

</details>

##### <span id="ModuleRenderer.generate_mdbook_summary"></span><code>generate_​mdbook_​summary</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn generate_mdbook_summary(
    &self,
    python_modules: &[PythonModule],
    rust_modules: &[RustModule],
) -> String
```

Generate SUMMARY.md for mdBook navigation

Creates a hierarchical navigation with inline format (one file per module):
```markdown
# Summary

# Python

- [pysnake](pysnake.md)
  - [pysnake.handlers](pysnake/handlers.md)

# Rust

- [rustscale](rust/rustscale.md)
  - [rustscale::config](rust/rustscale/config.md)
```

<details>
<summary>Source</summary>

```rust
    pub fn generate_mdbook_summary(
        &self,
        python_modules: &[PythonModule],
        rust_modules: &[RustModule],
    ) -> String {
        let mut summary = String::new();
        summary.push_str("# Summary\n\n");

        // Python section
        if !python_modules.is_empty() {
            summary.push_str("# Python\n\n");
            for entry in python_nav_entries(python_modules) {
                let indent = "  ".repeat(entry.depth);
                summary.push_str(&format!(
                    "{}- [{}]({})\n",
                    indent,
                    entry.path,
                    entry.file_path.display()
                ));
            }
        }

        // Rust section
        if !rust_modules.is_empty() {
            summary.push_str("\n# Rust\n\n");
            for entry in rust_nav_entries(rust_modules) {
                let indent = "  ".repeat(entry.depth);
                summary.push_str(&format!(
                    "{}- [{}]({})\n",
                    indent,
                    entry.path,
                    entry.file_path.display()
                ));
            }
        }

        summary
    }
```

</details>

##### <span id="ModuleRenderer.generate_mdbook_config"></span><code>generate_​mdbook_​config</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn generate_mdbook_config(
    &self,
    title: &str,
    authors: &[String],
) -> String
```

Generate book.toml configuration for mdBook

Includes fold configuration for collapsible sidebar sections.

<details>
<summary>Source</summary>

```rust
    pub fn generate_mdbook_config(&self, title: &str, authors: &[String]) -> String {
        let authors_toml = if authors.is_empty() {
            String::from("[]")
        } else {
            format!(
                "[{}]",
                authors
                    .iter()
                    .map(|a| format!("\"{}\"", a))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        };

        format!(
            r#"[book]
title = "{}"
authors = {}
language = "en"
src = "src"

[build]
build-dir = "book"

[output.html]
default-theme = "rust"
preferred-dark-theme = "coal"
additional-css = ["theme/custom.css"]

[output.html.fold]
enable = true
level = 1
"#,
            title, authors_toml
        )
    }
```

</details>

##### <span id="ModuleRenderer.generate_mdbook_css"></span><code>generate_​mdbook_​css</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn generate_mdbook_css(&self) -> String
```

Generate custom CSS for mdBook

Hides chapter numbering for reference documentation sections (python/, rust/)
while preserving numbering for other documentation sections.

<details>
<summary>Source</summary>

```rust
    pub fn generate_mdbook_css(&self) -> String {
        r#"/* Hide chapter numbering for reference documentation sections only.
   Scoped to python/ and rust/ paths to avoid affecting other documentation.
   Uses *= (contains) since mdbook prepends path_to_root to hrefs. */
.chapter-item a[href*="python/"] strong[aria-hidden="true"],
.chapter-item a[href*="rust/"] strong[aria-hidden="true"] {
    display: none !important;
}
"#
        .to_string()
    }
```

</details>
