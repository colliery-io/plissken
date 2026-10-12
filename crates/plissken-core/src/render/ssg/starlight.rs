//! Astro Starlight adapter implementation
//!
//! Pages go into a folder of the Starlight content collection
//! (`src/content/docs/...`). Starlight builds a route for each page from its
//! file path, so this module also holds the rules that turn a page path into
//! a route, and a route into a relative link that works under any `base`.

use super::traits::{NavEntry, SSGAdapter, python_nav_entries, rust_nav_entries};
use crate::model::{PythonModule, RustModule};
use std::path::{Component, Path, PathBuf};

/// File name of the generated sidebar module.
///
/// The leading `_` keeps it out of the content collection.
pub const STARLIGHT_SIDEBAR_FILE: &str = "_sidebar.mjs";

/// The route base used when the output path is not in `src/content/docs/`.
pub const STARLIGHT_DEFAULT_ROUTE_BASE: &str = "reference/api";

/// Slug a string as Astro does (`github-slugger`).
///
/// Astro slugs each segment of a content file path with it, and Starlight
/// slugs heading text with it. Lowercase; keep letters, numbers, `_`, `-`
/// and spaces; spaces become `-`.
pub fn github_slug(value: &str) -> String {
    value
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-' || *c == ' ')
        .map(|c| if c == ' ' { '-' } else { c })
        .collect()
}

/// The route of a page, relative to the output folder.
///
/// `rust/my_crate/sub.md` -> `rust/my_crate/sub`; `pkg/_core/index.md` ->
/// `pkg/_core`. Each segment is slugged as Astro does.
pub fn page_route(file_path: &str) -> String {
    let without_ext = file_path.strip_suffix(".md").unwrap_or(file_path);
    let route = without_ext
        .split('/')
        .filter(|s| !s.is_empty())
        .map(github_slug)
        .collect::<Vec<_>>()
        .join("/");
    match route.strip_suffix("/index") {
        Some(r) => r.to_string(),
        None if route == "index" => String::new(),
        None => route,
    }
}

/// The file path of a page in a Starlight content folder.
///
/// Starlight skips a file whose name starts with `_`, so `pkg/_core.md` is
/// written as `pkg/_core/index.md`. Its route stays `pkg/_core`.
pub fn starlight_file_path(path: &Path) -> PathBuf {
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
    if stem.starts_with('_') {
        path.with_extension("").join("index.md")
    } else {
        path.to_path_buf()
    }
}

/// A relative link from one page to another, as a route link.
///
/// Both paths are page paths relative to the output folder (`foo/bar.md`).
/// Starlight serves the page `foo/bar.md` at `foo/bar/`, so the link climbs
/// one level per segment of the source route. The result has no `.md` and
/// does not start with `/`, so it works under any `base`.
pub fn route_href(from_file: &str, to_file: &str, anchor: Option<&str>) -> String {
    let from = page_route(from_file);
    let depth = if from.is_empty() {
        0
    } else {
        from.split('/').count()
    };
    let to = page_route(to_file);
    let mut href = "../".repeat(depth);
    if !to.is_empty() {
        href.push_str(&to);
        href.push('/');
    }
    if href.is_empty() {
        href.push_str("./");
    }
    if let Some(anchor) = anchor {
        href.push('#');
        href.push_str(anchor);
    }
    href
}

/// The route of the output folder in the Starlight site.
///
/// The part of `output_path` after `src/content/docs/`, slugged:
/// `docs/src/content/docs/reference/api` -> `Some("reference/api")`.
/// `None` if the path is not in a Starlight content folder.
pub fn content_route_base(output_path: &Path) -> Option<String> {
    let parts: Vec<String> = output_path
        .components()
        .filter_map(|c| match c {
            Component::Normal(s) => Some(s.to_string_lossy().to_string()),
            _ => None,
        })
        .collect();
    let start = parts
        .windows(3)
        .rposition(|w| w[0] == "src" && w[1] == "content" && w[2] == "docs")?;
    Some(
        parts[start + 3..]
            .iter()
            .map(|s| github_slug(s))
            .collect::<Vec<_>>()
            .join("/"),
    )
}

/// Quote a string for YAML front matter (a JSON string is valid YAML).
fn yaml_string(value: &str) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| "\"\"".to_string())
}

/// Front matter of a Starlight page.
///
/// `sidebar.hidden` keeps the page out of an `autogenerate` sidebar group:
/// the generated sidebar module lists the pages instead. `editUrl: false`:
/// a generated page has no source file to edit.
pub fn front_matter(title: &str, description: &str, order: usize) -> String {
    let description = description.split_whitespace().collect::<Vec<_>>().join(" ");
    format!(
        "---\ntitle: {}\ndescription: {}\neditUrl: false\nsidebar:\n  order: {}\n  hidden: true\n---\n\n",
        yaml_string(title),
        yaml_string(&description),
        order
    )
}

/// The file of the index page of the API reference (item 15 of
/// PLSKN-T-0057): the page at the route of the output folder.
pub const STARLIGHT_INDEX_FILE: &str = "index.md";

/// Heading text that a narrow column (the table of contents) can break
/// after each `_`: a zero-width space follows it. Slugs drop the space, so
/// anchors do not change.
pub fn breakable_name(text: &str) -> String {
    text.replace('_', "_\u{200B}")
}

/// An HTML element that gives an item an explicit anchor id.
///
/// Put at the start of a Markdown heading. It has no text, so the slug and
/// the table of contents entry of the heading do not change.
pub fn anchor_span(id: &str) -> String {
    format!("<span id=\"{}\"></span>", id)
}

/// Code fence attributes of rustdoc (`ignore`, `no_run`, ...).
const RUSTDOC_FENCE_ATTRIBUTES: &[&str] = &[
    "rust",
    "ignore",
    "no_run",
    "should_panic",
    "compile_fail",
    "test_harness",
    "allow_fail",
    "edition2015",
    "edition2018",
    "edition2021",
    "edition2024",
];

/// Give rustdoc code fences the language `rust`.
///
/// In Rust doc comments, a fence such as ```` ```ignore ```` or
/// ```` ```rust,no_run ```` holds Rust code. Starlight's highlighter knows no
/// language `ignore`, so it would show plain text.
pub fn normalize_rustdoc_fences(content: &str) -> String {
    let mut out = String::with_capacity(content.len());
    for line in content.split_inclusive('\n') {
        let trimmed = line.trim_start();
        let indent = &line[..line.len() - trimmed.len()];
        let fence_len = trimmed.chars().take_while(|c| *c == '`').count();
        let info = trimmed[fence_len..].trim_end();
        let is_rustdoc = fence_len >= 3
            && !info.is_empty()
            && info
                .split(',')
                .map(str::trim)
                .all(|a| RUSTDOC_FENCE_ATTRIBUTES.contains(&a));
        if is_rustdoc && info != "rust" {
            out.push_str(indent);
            out.push_str(&trimmed[..fence_len]);
            out.push_str("rust");
            out.push_str(&line[line.trim_end().len()..]);
        } else {
            out.push_str(line);
        }
    }
    out
}

/// Astro Starlight adapter.
///
/// Generates `_sidebar.mjs`: an ES module whose default export is a
/// Starlight sidebar group of the Python and Rust modules. The site's
/// `astro.config.mjs` imports it:
///
/// ```js
/// import apiSidebar from './src/content/docs/reference/api/_sidebar.mjs';
/// // ...
/// sidebar: [ /* other groups */ apiSidebar ],
/// ```
///
/// Each link is a `slug` entry, so Starlight adds the site `base` to it.
pub struct StarlightAdapter {
    route_base: String,
}

impl Default for StarlightAdapter {
    fn default() -> Self {
        Self::new(STARLIGHT_DEFAULT_ROUTE_BASE)
    }
}

/// A module in the sidebar tree.
#[derive(Default)]
struct Node {
    name: String,
    slug: Option<String>,
    children: Vec<Node>,
}

impl Node {
    fn child(&mut self, name: &str) -> &mut Node {
        let pos = match self.children.iter().position(|c| c.name == name) {
            Some(pos) => pos,
            None => {
                self.children.push(Node {
                    name: name.to_string(),
                    ..Default::default()
                });
                self.children.len() - 1
            }
        };
        &mut self.children[pos]
    }

    /// The sidebar item of this module: a link, or a collapsed group of
    /// its own page ("Overview") and its submodules.
    fn to_item(&self, indent: usize) -> String {
        match (&self.slug, self.children.is_empty()) {
            (Some(slug), true) => link_item(&self.name, slug),
            (slug, _) => {
                let mut items = Vec::new();
                if let Some(slug) = slug {
                    items.push(link_item("Overview", slug));
                }
                items.extend(self.children.iter().map(|c| c.to_item(indent + 2)));
                group_item(&self.name, true, &items, indent)
            }
        }
    }
}

/// A JavaScript string literal.
fn js_string(value: &str) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| "\"\"".to_string())
}

/// A sidebar link to a page of the content collection.
fn link_item(label: &str, slug: &str) -> String {
    format!(
        "{{ label: {}, slug: {} }}",
        js_string(label),
        js_string(slug)
    )
}

/// A sidebar group, as a JavaScript object literal.
fn group_item(label: &str, collapsed: bool, items: &[String], indent: usize) -> String {
    let pad = "  ".repeat(indent);
    let mut out = format!("{{\n{pad}  label: {},\n", js_string(label));
    if collapsed {
        out.push_str(&format!("{pad}  collapsed: true,\n"));
    }
    out.push_str(&format!("{pad}  items: [\n"));
    for item in items {
        out.push_str(&format!("{pad}    {},\n", item));
    }
    out.push_str(&format!("{pad}  ],\n{pad}}}"));
    out
}

impl StarlightAdapter {
    /// Create an adapter for pages under the given route of the site
    /// (for example `reference/api`).
    pub fn new(route_base: &str) -> Self {
        Self {
            route_base: route_base.trim_matches('/').to_string(),
        }
    }

    /// Create an adapter for an output folder in a Starlight site.
    ///
    /// Returns `None` if the path is not in `src/content/docs/`.
    pub fn for_output_path(output_path: &Path) -> Option<Self> {
        content_route_base(output_path).map(|base| Self::new(&base))
    }

    /// The route of the output folder in the site.
    pub fn route_base(&self) -> &str {
        &self.route_base
    }

    /// The Starlight slug of a page (its content id).
    fn slug(&self, file_path: &Path) -> String {
        let route = page_route(&file_path.to_string_lossy().replace('\\', "/"));
        match (self.route_base.is_empty(), route.is_empty()) {
            (true, _) => route,
            (false, true) => self.route_base.clone(),
            (false, false) => format!("{}/{}", self.route_base, route),
        }
    }

    /// The sidebar group of one language, modules nested by path.
    fn group(&self, label: &str, entries: Vec<NavEntry>, separator: &str) -> String {
        let mut root = Node::default();
        for entry in entries {
            let mut node = &mut root;
            for part in entry.path.split(separator) {
                node = node.child(part);
            }
            node.slug = Some(self.slug(&entry.file_path));
        }
        let items: Vec<String> = root.children.iter().map(|c| c.to_item(4)).collect();
        group_item(label, false, &items, 2)
    }
}

impl SSGAdapter for StarlightAdapter {
    fn name(&self) -> &'static str {
        "starlight"
    }

    fn content_dir(&self) -> &'static str {
        "src/content/docs"
    }

    fn nav_filename(&self) -> &'static str {
        STARLIGHT_SIDEBAR_FILE
    }

    fn generate_nav(&self, python_modules: &[PythonModule], rust_modules: &[RustModule]) -> String {
        let mut groups = Vec::new();
        if !python_modules.is_empty() {
            groups.push(self.group("Python", python_nav_entries(python_modules), "."));
        }
        if !rust_modules.is_empty() {
            groups.push(self.group("Rust", rust_nav_entries(rust_modules), "::"));
        }
        let mut items = vec![link_item(
            "Overview",
            &self.slug(Path::new(STARLIGHT_INDEX_FILE)),
        )];
        items.extend(groups);
        let body = group_item("API reference", false, &items, 0);

        format!(
            "// Generated by plissken. Do not edit: `plissken render` writes it again.\n\
             //\n\
             // The API reference as a Starlight sidebar group. In astro.config.mjs:\n\
             //\n\
             //   import apiSidebar from './src/content/docs/{base}/{file}';\n\
             //   ...\n\
             //   sidebar: [ /* your groups */ apiSidebar ],\n\
             export default {body};\n",
            base = self.route_base,
            file = STARLIGHT_SIDEBAR_FILE,
            body = body
        )
    }

    fn generate_config(&self, _title: &str, _authors: &[String]) -> Option<String> {
        // astro.config.mjs belongs to the site; plissken does not write it.
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_starlight_adapter_properties() {
        let adapter = StarlightAdapter::default();
        assert_eq!(adapter.name(), "starlight");
        assert_eq!(adapter.content_dir(), "src/content/docs");
        assert_eq!(adapter.nav_filename(), "_sidebar.mjs");
        assert_eq!(adapter.content_extension(), "md");
        assert!(adapter.supports_nested_nav());
        assert_eq!(adapter.route_base(), "reference/api");
        assert!(adapter.generate_config("Test", &[]).is_none());
        assert!(adapter.generate_custom_css().is_none());
    }

    #[test]
    fn test_github_slug() {
        assert_eq!(github_slug("plissken_core"), "plissken_core");
        assert_eq!(github_slug("MyModule"), "mymodule");
        assert_eq!(github_slug("class Foo"), "class-foo");
        assert_eq!(github_slug("fn new()"), "fn-new");
        assert_eq!(github_slug("a.b::c"), "abc");
    }

    #[test]
    fn test_page_route() {
        assert_eq!(page_route("pkg.md"), "pkg");
        assert_eq!(page_route("pkg/Sub.md"), "pkg/sub");
        assert_eq!(
            page_route("rust/my_crate/config.md"),
            "rust/my_crate/config"
        );
        assert_eq!(page_route("pkg/_core/index.md"), "pkg/_core");
        assert_eq!(page_route("index.md"), "");
    }

    #[test]
    fn test_starlight_file_path() {
        assert_eq!(
            starlight_file_path(Path::new("pkg/sub.md")),
            PathBuf::from("pkg/sub.md")
        );
        assert_eq!(
            starlight_file_path(Path::new("pkg/_core.md")),
            PathBuf::from("pkg/_core/index.md")
        );
        assert_eq!(
            starlight_file_path(Path::new("_private.md")),
            PathBuf::from("_private/index.md")
        );
    }

    #[test]
    fn test_route_href() {
        // pkg.md is served at pkg/, rust/my_crate.md at rust/my_crate/
        assert_eq!(
            route_href("pkg.md", "rust/my_crate.md", Some("struct.Foo")),
            "../rust/my_crate/#struct.Foo"
        );
        assert_eq!(
            route_href("rust/my_crate/sub.md", "pkg/mod.md", None),
            "../../../pkg/mod/"
        );
        assert_eq!(
            route_href("rust/my_crate.md", "pkg/_core/index.md", Some("fn.run")),
            "../../pkg/_core/#fn.run"
        );
        // Never a .md link, never a root-relative link
        let href = route_href("a/b.md", "c.md", None);
        assert!(!href.contains(".md"));
        assert!(!href.starts_with('/'));
    }

    #[test]
    fn test_content_route_base() {
        assert_eq!(
            content_route_base(Path::new("docs/src/content/docs/reference/api")),
            Some("reference/api".to_string())
        );
        assert_eq!(
            content_route_base(Path::new("/x/site/src/content/docs/API")),
            Some("api".to_string())
        );
        assert_eq!(
            content_route_base(Path::new("site/src/content/docs")),
            Some(String::new())
        );
        assert_eq!(content_route_base(Path::new("docs/api")), None);
    }

    #[test]
    fn test_front_matter() {
        let fm = front_matter("my_crate::config", "Config \"loader\".\n  Two lines.", 3);
        assert!(fm.starts_with("---\n"));
        assert!(fm.contains("title: \"my_crate::config\"\n"));
        assert!(fm.contains("description: \"Config \\\"loader\\\". Two lines.\"\n"));
        assert!(fm.contains("editUrl: false\n"));
        assert!(fm.contains("sidebar:\n  order: 3\n  hidden: true\n"));
        assert!(fm.ends_with("---\n\n"));
    }

    #[test]
    fn test_normalize_rustdoc_fences() {
        let input = "a\n```ignore\nlet x = 1;\n```\n  ```rust,no_run\nfoo();\n  ```\n```python\nx\n```\n```text\nt\n```\n";
        let output = normalize_rustdoc_fences(input);
        assert_eq!(
            output,
            "a\n```rust\nlet x = 1;\n```\n  ```rust\nfoo();\n  ```\n```python\nx\n```\n```text\nt\n```\n"
        );
    }

    #[test]
    fn test_breakable_name() {
        // Item 8 of PLSKN-T-0057: "merge_synthesized_python_mo / dules"
        let name = breakable_name("fn merge_synthesized_python_modules");
        assert_eq!(
            name,
            "fn merge_\u{200B}synthesized_\u{200B}python_\u{200B}modules"
        );
        // The slug, and so the anchor, does not change
        assert_eq!(
            github_slug(&name.replace('\u{200B}', "")),
            "fn-merge_synthesized_python_modules"
        );
    }

    #[test]
    fn test_anchor_span() {
        assert_eq!(anchor_span("class.Foo"), "<span id=\"class.Foo\"></span>");
    }

    #[test]
    fn test_generate_nav_empty() {
        let adapter = StarlightAdapter::default();
        let nav = adapter.generate_nav(&[], &[]);
        assert!(nav.contains("export default {"));
        assert!(nav.contains("label: \"API reference\""));
        assert!(!nav.contains("\"Python\""));
        assert!(!nav.contains("\"Rust\""));
    }

    #[test]
    fn test_generate_nav_python_only() {
        let adapter = StarlightAdapter::default();
        let nav = adapter.generate_nav(&[PythonModule::test("mymodule")], &[]);
        assert!(nav.contains("label: \"Python\""));
        assert!(nav.contains("{ label: \"mymodule\", slug: \"reference/api/mymodule\" }"));
        assert!(!nav.contains(".md"));
    }

    #[test]
    fn test_generate_nav_nested() {
        let adapter = StarlightAdapter::new("/api/");
        let nav = adapter.generate_nav(
            &[PythonModule::test("pkg"), PythonModule::test("pkg._core")],
            &[
                RustModule::test("my_crate"),
                RustModule::test("my_crate::Config"),
            ],
        );
        insta::assert_snapshot!(nav);
    }

    #[test]
    fn test_for_output_path() {
        let adapter =
            StarlightAdapter::for_output_path(Path::new("docs/src/content/docs/reference/api"))
                .unwrap();
        assert_eq!(adapter.route_base(), "reference/api");
        assert!(StarlightAdapter::for_output_path(Path::new("docs/api")).is_none());
    }
}
