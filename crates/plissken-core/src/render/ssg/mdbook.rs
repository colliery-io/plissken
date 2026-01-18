//! mdBook adapter implementation

use super::traits::{SSGAdapter, python_nav_entries, rust_nav_entries};
use crate::model::{PythonModule, RustModule};

/// mdBook adapter.
///
/// Generates Markdown SUMMARY.md navigation and book.toml configuration.
///
/// # Navigation Format (Inline)
///
/// All content is rendered inline in single module files:
/// ```markdown
/// # Summary
///
/// # Python
///
/// - [pysnake](pysnake.md)
///
/// # Rust
///
/// - [rustscale](rust/rustscale.md)
///   - [rustscale::config](rust/rustscale/config.md)
/// ```
pub struct MdBookAdapter;

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

    fn generate_nav(&self, python_modules: &[PythonModule], rust_modules: &[RustModule]) -> String {
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
        let nav = adapter.generate_nav(&[], &[]);
        assert!(nav.contains("# Summary"));
        assert!(!nav.contains("# Python"));
        assert!(!nav.contains("# Rust"));
    }

    #[test]
    fn test_generate_nav_python_only() {
        let adapter = MdBookAdapter;
        let module = PythonModule::test("mymodule");
        let nav = adapter.generate_nav(&[module], &[]);

        assert!(nav.contains("# Summary"));
        assert!(nav.contains("# Python"));
        // Inline format: mymodule.md (not python/mymodule/index.md)
        assert!(nav.contains("[mymodule](mymodule.md)"));
    }

    #[test]
    fn test_generate_nav_rust_only() {
        let adapter = MdBookAdapter;
        let module = RustModule::test("mycrate");
        let nav = adapter.generate_nav(&[], &[module]);

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
}
