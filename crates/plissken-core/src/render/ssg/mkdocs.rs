//! MkDocs adapter implementation

use super::traits::{
    NavNode, SSGAdapter, build_nav_tree, python_nav_entries, rust_nav_entries,
};
use crate::model::{PythonModule, RustModule};

/// MkDocs adapter for Material theme.
///
/// Generates hierarchical YAML navigation with collapsible sections
/// for nested modules. Works with MkDocs Material's `navigation.indexes`
/// feature to show section index pages.
///
/// # Navigation Format
///
/// Leaf modules (no children) render as simple entries:
/// ```yaml
/// - module_name: module_name.md
/// ```
///
/// Modules with children render as collapsible sections:
/// ```yaml
/// - parent_module:
///   - parent_module.md
///   - child: parent_module/child.md
/// ```
pub struct MkDocsAdapter;

/// Render a list of NavNodes as nested MkDocs YAML.
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

impl SSGAdapter for MkDocsAdapter {
    fn name(&self) -> &'static str {
        "mkdocs"
    }

    fn content_dir(&self) -> &'static str {
        "docs"
    }

    fn nav_filename(&self) -> &'static str {
        "_nav.yml"
    }

    fn generate_nav(
        &self,
        python_modules: &[PythonModule],
        rust_modules: &[RustModule],
        prefix: Option<&str>,
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
            let entries = python_nav_entries(python_modules);
            let tree = build_nav_tree(&entries, prefix, ".");
            yaml.push_str(&render_yaml_nodes(&tree, 2));
        }

        // Rust section
        if !rust_modules.is_empty() {
            yaml.push_str("  - Rust:\n");
            let entries = rust_nav_entries(rust_modules);
            let tree = build_nav_tree(&entries, prefix, "::");
            yaml.push_str(&render_yaml_nodes(&tree, 2));
        }

        yaml
    }

    fn generate_config(&self, _title: &str, _authors: &[String]) -> Option<String> {
        // MkDocs config (mkdocs.yml) typically pre-exists and is user-managed
        // We don't generate it, only the _nav.yml snippet
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::PythonModule;

    #[test]
    fn test_mkdocs_adapter_properties() {
        let adapter = MkDocsAdapter;
        assert_eq!(adapter.name(), "mkdocs");
        assert_eq!(adapter.content_dir(), "docs");
        assert_eq!(adapter.nav_filename(), "_nav.yml");
        assert_eq!(adapter.content_extension(), "md");
        assert!(adapter.supports_nested_nav());
    }

    #[test]
    fn test_generate_nav_empty() {
        let adapter = MkDocsAdapter;
        let nav = adapter.generate_nav(&[], &[], None);
        assert!(nav.contains("nav:"));
        assert!(!nav.contains("Python:"));
        assert!(!nav.contains("Rust:"));
    }

    #[test]
    fn test_generate_nav_python_only() {
        let adapter = MkDocsAdapter;
        let module = PythonModule::test("mymodule");
        let nav = adapter.generate_nav(&[module], &[], None);

        assert!(nav.contains("- Python:"));
        assert!(nav.contains("mymodule"));
        // Inline format: module_name.md (not python/mymodule/index.md)
        assert!(nav.contains("mymodule.md"));
    }

    #[test]
    fn test_generate_config_returns_none() {
        let adapter = MkDocsAdapter;
        assert!(adapter.generate_config("Test", &[]).is_none());
    }

    #[test]
    fn test_generate_nav_with_prefix() {
        let adapter = MkDocsAdapter;
        let module = PythonModule::test("mymodule");
        let nav = adapter.generate_nav(&[module], &[], Some("api"));

        assert!(nav.contains("- Python:"));
        assert!(nav.contains("api/mymodule.md"));
        // Should NOT contain unprefixed path
        assert!(!nav.contains("    - mymodule: mymodule.md"));
    }

    #[test]
    fn test_generate_nav_with_prefix_rust() {
        use crate::model::RustModule;
        let adapter = MkDocsAdapter;
        let module = RustModule::test("mycrate");
        let nav = adapter.generate_nav(&[], &[module], Some("api/reference"));

        assert!(nav.contains("- Rust:"));
        assert!(nav.contains("api/reference/rust/mycrate.md"));
    }

    #[test]
    fn test_generate_nav_none_prefix_unchanged() {
        let adapter = MkDocsAdapter;
        let module = PythonModule::test("mymodule");
        let nav_none = adapter.generate_nav(&[module.clone()], &[], None);
        let nav_explicit_none = adapter.generate_nav(&[module], &[], None);

        assert_eq!(nav_none, nav_explicit_none);
        assert!(nav_none.contains("mymodule.md"));
        assert!(!nav_none.contains("/mymodule.md")); // no leading slash
    }
}
