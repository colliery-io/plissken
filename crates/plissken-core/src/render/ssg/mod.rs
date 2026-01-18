//! Static Site Generator (SSG) adapters
//!
//! This module provides adapters for different static site generators,
//! abstracting away SSG-specific differences in navigation format,
//! config files, and directory structure.
//!
//! # Supported SSGs
//!
//! - **MkDocs** (with Material theme): YAML navigation, `docs/` content directory
//! - **mdBook**: Markdown SUMMARY.md, `src/` content directory
//!
//! # Example
//!
//! ```rust
//! use plissken_core::render::ssg::{SSGAdapter, get_ssg_adapter};
//!
//! let adapter = get_ssg_adapter(Some("mkdocs-material"));
//! assert_eq!(adapter.name(), "mkdocs");
//! assert_eq!(adapter.content_dir(), "docs");
//! assert_eq!(adapter.nav_filename(), "_nav.yml");
//! ```

mod mdbook;
mod mkdocs;
mod traits;

pub use mdbook::MdBookAdapter;
pub use mkdocs::MkDocsAdapter;
pub use traits::{NavEntry, SSGAdapter, python_nav_entries, rust_nav_entries};

/// Get an SSG adapter for the given template name.
///
/// # Arguments
///
/// * `template` - Optional template name. If None or unrecognized, defaults to MkDocs.
///
/// # Example
///
/// ```rust
/// use plissken_core::render::ssg::get_ssg_adapter;
///
/// let mkdocs = get_ssg_adapter(Some("mkdocs-material"));
/// assert_eq!(mkdocs.name(), "mkdocs");
///
/// let mdbook = get_ssg_adapter(Some("mdbook"));
/// assert_eq!(mdbook.name(), "mdbook");
///
/// let default = get_ssg_adapter(None);
/// assert_eq!(default.name(), "mkdocs");
/// ```
pub fn get_ssg_adapter(template: Option<&str>) -> Box<dyn SSGAdapter> {
    match template.map(|s| s.to_lowercase()).as_deref() {
        Some("mdbook") | Some("md-book") | Some("md_book") => Box::new(MdBookAdapter),
        _ => Box::new(MkDocsAdapter),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_ssg_adapter_mkdocs() {
        let adapter = get_ssg_adapter(Some("mkdocs-material"));
        assert_eq!(adapter.name(), "mkdocs");
        assert_eq!(adapter.content_dir(), "docs");
        assert_eq!(adapter.nav_filename(), "_nav.yml");
    }

    #[test]
    fn test_get_ssg_adapter_mdbook() {
        let adapter = get_ssg_adapter(Some("mdbook"));
        assert_eq!(adapter.name(), "mdbook");
        assert_eq!(adapter.content_dir(), "src");
        assert_eq!(adapter.nav_filename(), "SUMMARY.md");
    }

    #[test]
    fn test_get_ssg_adapter_mdbook_variants() {
        for name in &["mdbook", "md-book", "md_book", "MDBOOK"] {
            let adapter = get_ssg_adapter(Some(name));
            assert_eq!(adapter.name(), "mdbook", "Failed for: {}", name);
        }
    }

    #[test]
    fn test_get_ssg_adapter_default() {
        let adapter = get_ssg_adapter(None);
        assert_eq!(adapter.name(), "mkdocs");
    }

    #[test]
    fn test_get_ssg_adapter_unknown() {
        let adapter = get_ssg_adapter(Some("unknown-ssg"));
        assert_eq!(adapter.name(), "mkdocs"); // defaults to mkdocs
    }
}
