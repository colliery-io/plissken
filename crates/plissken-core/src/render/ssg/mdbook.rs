//! mdBook adapter implementation

use super::traits::{
    NavNode, SSGAdapter, build_nav_tree, python_nav_entries, rust_nav_entries,
};
use crate::model::{PythonModule, RustModule};

/// mdBook adapter.
///
/// Generates Markdown SUMMARY.md navigation and book.toml configuration.
/// Nested modules render as indented entries for collapsible sidebar sections.
///
/// # Navigation Format
///
/// ```markdown
/// # Summary
///
/// # Python
///
/// - [pysnake](pysnake.md)
///   - [handlers](pysnake/handlers.md)
///
/// # Rust
///
/// - [rustscale](rust/rustscale.md)
///   - [config](rust/rustscale/config.md)
/// ```
pub struct MdBookAdapter;

/// Render a list of NavNodes as indented mdBook SUMMARY.md entries.
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

impl SSGAdapter for MdBookAdapter {
    fn name(&self) -> &'static str {
        "mdbook"
    }

    fn content_dir(&self) -> &'static str {
        "src"
    }

    fn nav_filename(&self) -> &'static str {
        "SUMMARY.md"
    }

    fn generate_nav(
        &self,
        python_modules: &[PythonModule],
        rust_modules: &[RustModule],
        prefix: Option<&str>,
    ) -> String {
        let mut summary = String::new();

        summary.push_str("# Summary\n\n");

        // Python section
        if !python_modules.is_empty() {
            summary.push_str("# Python\n\n");
            let entries = python_nav_entries(python_modules);
            let tree = build_nav_tree(&entries, prefix, ".");
            summary.push_str(&render_md_nodes(&tree, 0));
        }

        // Rust section
        if !rust_modules.is_empty() {
            summary.push_str("\n# Rust\n\n");
            let entries = rust_nav_entries(rust_modules);
            let tree = build_nav_tree(&entries, prefix, "::");
            summary.push_str(&render_md_nodes(&tree, 0));
        }

        summary
    }

    fn generate_config(&self, title: &str, authors: &[String]) -> Option<String> {
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

        Some(format!(
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
        ))
    }

    fn generate_custom_css(&self) -> Option<String> {
        Some(
            r#"/* Hide chapter numbering for reference documentation sections only.
   Scoped to python/ and rust/ paths to avoid affecting other documentation.
   Uses *= (contains) since mdbook prepends path_to_root to hrefs. */
.chapter-item a[href*="python/"] strong[aria-hidden="true"],
.chapter-item a[href*="rust/"] strong[aria-hidden="true"] {
    display: none !important;
}
"#
            .to_string(),
        )
    }

    fn custom_css_path(&self) -> Option<&'static str> {
        Some("theme/custom.css")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{PythonModule, RustModule};

    #[test]
    fn test_mdbook_adapter_properties() {
        let adapter = MdBookAdapter;
        assert_eq!(adapter.name(), "mdbook");
        assert_eq!(adapter.content_dir(), "src");
        assert_eq!(adapter.nav_filename(), "SUMMARY.md");
        assert_eq!(adapter.content_extension(), "md");
        assert!(adapter.supports_nested_nav());
        assert_eq!(adapter.custom_css_path(), Some("theme/custom.css"));
    }

    #[test]
    fn test_generate_nav_empty() {
        let adapter = MdBookAdapter;
        let nav = adapter.generate_nav(&[], &[], None);
        assert!(nav.contains("# Summary"));
        assert!(!nav.contains("# Python"));
        assert!(!nav.contains("# Rust"));
    }

    #[test]
    fn test_generate_nav_python_only() {
        let adapter = MdBookAdapter;
        let module = PythonModule::test("mymodule");
        let nav = adapter.generate_nav(&[module], &[], None);

        assert!(nav.contains("# Summary"));
        assert!(nav.contains("# Python"));
        // Inline format: mymodule.md (not python/mymodule/index.md)
        assert!(nav.contains("[mymodule](mymodule.md)"));
    }

    #[test]
    fn test_generate_nav_rust_only() {
        let adapter = MdBookAdapter;
        let module = RustModule::test("mycrate");
        let nav = adapter.generate_nav(&[], &[module], None);

        assert!(nav.contains("# Summary"));
        assert!(nav.contains("# Rust"));
        // Inline format: rust/mycrate.md (not rust/mycrate/index.md)
        assert!(nav.contains("[mycrate](rust/mycrate.md)"));
    }

    #[test]
    fn test_generate_config() {
        let adapter = MdBookAdapter;
        let config = adapter
            .generate_config("Test Project", &["Author One".to_string()])
            .unwrap();

        assert!(config.contains("title = \"Test Project\""));
        assert!(config.contains("authors = [\"Author One\"]"));
        assert!(config.contains("src = \"src\""));
        assert!(config.contains("[output.html.fold]"));
    }

    #[test]
    fn test_generate_config_no_authors() {
        let adapter = MdBookAdapter;
        let config = adapter.generate_config("Test Project", &[]).unwrap();

        assert!(config.contains("authors = []"));
    }

    #[test]
    fn test_generate_custom_css() {
        let adapter = MdBookAdapter;
        let css = adapter.generate_custom_css().unwrap();

        assert!(css.contains(".chapter-item"));
        assert!(css.contains("python/"));
        assert!(css.contains("rust/"));
        assert!(css.contains("display: none"));
    }

    #[test]
    fn test_generate_nav_with_prefix_python() {
        let adapter = MdBookAdapter;
        let module = PythonModule::test("mymodule");
        let nav = adapter.generate_nav(&[module], &[], Some("api"));

        assert!(nav.contains("# Python"));
        assert!(nav.contains("[mymodule](api/mymodule.md)"));
    }

    #[test]
    fn test_generate_nav_with_prefix_rust() {
        let adapter = MdBookAdapter;
        let module = RustModule::test("mycrate");
        let nav = adapter.generate_nav(&[], &[module], Some("api/reference"));

        assert!(nav.contains("# Rust"));
        assert!(nav.contains("[mycrate](api/reference/rust/mycrate.md)"));
    }

    #[test]
    fn test_generate_nav_none_prefix_unchanged() {
        let adapter = MdBookAdapter;
        let module = RustModule::test("mycrate");
        let nav = adapter.generate_nav(&[], &[module], None);

        assert!(nav.contains("[mycrate](rust/mycrate.md)"));
        // No prefix — path should not start with /
        assert!(!nav.contains("(/rust/"));
    }
}
