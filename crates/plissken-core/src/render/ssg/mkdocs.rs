//! MkDocs adapter implementation

use super::traits::{SSGAdapter, python_nav_entries, rust_nav_entries};
use crate::model::{PythonModule, RustModule};

/// MkDocs adapter for Material theme.
///
/// Generates YAML navigation and integrates with MkDocs Material's features.
///
/// # Navigation Format (Inline)
///
/// All content is rendered inline in single module files:
/// ```yaml
/// nav:
///   - Python:
///     - pysnake: pysnake.md
///   - Rust:
///     - rustscale: rust/rustscale.md
///     - rustscale::config: rust/rustscale/config.md
/// ```
pub struct MkDocsAdapter;

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

    fn generate_nav(&self, python_modules: &[PythonModule], rust_modules: &[RustModule]) -> String {
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
        let nav = adapter.generate_nav(&[], &[]);
        assert!(nav.contains("nav:"));
        assert!(!nav.contains("Python:"));
        assert!(!nav.contains("Rust:"));
    }

    #[test]
    fn test_generate_nav_python_only() {
        let adapter = MkDocsAdapter;
        let module = PythonModule::test("mymodule");
        let nav = adapter.generate_nav(&[module], &[]);

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
}
