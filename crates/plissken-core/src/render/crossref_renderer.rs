//! Cross-reference link generation for Python-Rust documentation
//!
//! This module provides utilities for generating Markdown links between
//! Python items and their Rust implementations, and vice versa.

use crate::model::{CrossRef, CrossRefKind, RustItemRef};

/// Generate a relative Markdown link from a Python page to a Rust item.
///
/// # Arguments
///
/// * `rust_ref` - Reference to the Rust item
/// * `from_python_path` - The Python module path (e.g., "mypackage.submodule")
///
/// # Returns
///
/// A Markdown link string like `[RustStruct](../rust/crate/module.md#struct-ruststruct)`
///
/// # Example
///
/// ```rust
/// use plissken_core::model::RustItemRef;
/// use plissken_core::render::link_to_rust;
///
/// let rust_ref = RustItemRef::new("crate::utils", "Config");
/// let link = link_to_rust(&rust_ref, "mypackage.config");
/// assert!(link.contains("rust/crate/utils.md"));
/// assert!(link.contains("#struct-config"));
/// ```
pub fn link_to_rust(rust_ref: &RustItemRef, from_python_path: &str) -> String {
    let rust_page_path = rust_path_to_file_path(&rust_ref.path);
    let anchor = item_to_anchor(&rust_ref.name, "struct");
    // from_python_path is now python/module, rust is rust/module
    let from_path = format!("python/{}", from_python_path.replace('.', "/"));
    let relative_path = compute_relative_path(&from_path, &format!("rust/{}", rust_page_path));

    format!("[{}]({}#{})", rust_ref.name, relative_path, anchor)
}

/// Generate a relative Markdown link from a Rust page to a Python item.
///
/// # Arguments
///
/// * `python_path` - Full Python path (e.g., "mypackage.module.ClassName")
/// * `from_rust_path` - The Rust module path (e.g., "crate::utils")
///
/// # Returns
///
/// A Markdown link string like `[ClassName](../../mypackage/module.md#class-classname)`
///
/// # Example
///
/// ```rust
/// use plissken_core::render::link_to_python;
///
/// let link = link_to_python("mypackage.utils.Config", "crate::utils");
/// assert!(link.contains("mypackage/utils.md"));
/// assert!(link.contains("#class-config"));
/// ```
pub fn link_to_python(python_path: &str, from_rust_path: &str) -> String {
    let (module_path, item_name) = split_python_path(python_path);
    let python_page_path = python_path_to_file_path(&module_path);
    let rust_page_path = rust_path_to_file_path(from_rust_path);
    // python_page_path now includes python/ prefix
    let relative_path =
        compute_relative_path(&format!("rust/{}", rust_page_path), &python_page_path);
    let anchor = item_to_anchor(&item_name, "class");

    format!("[{}]({}#{})", item_name, relative_path, anchor)
}

/// Generate a cross-reference link based on the CrossRef relationship.
///
/// Returns a tuple of (link_text, link_url, relationship_badge).
///
/// `from_path` should be the module path without python/ or rust/ prefix.
pub fn crossref_link(xref: &CrossRef, from_path: &str, from_language: Language) -> CrossRefLink {
    match from_language {
        Language::Python => {
            // Generate link from Python to Rust
            let (module_path, item_name) = split_rust_path(&xref.rust_path);
            let rust_page = rust_path_to_file_path(&module_path);
            // from_path is Python module path, needs python/ prefix
            let from_full = format!("python/{}", from_path.replace('.', "/"));
            let relative = compute_relative_path(&from_full, &format!("rust/{}", rust_page));
            let anchor = item_to_anchor(&item_name, "struct");

            CrossRefLink {
                text: item_name,
                url: format!("{}#{}", relative, anchor),
                relationship: xref.relationship.clone(),
            }
        }
        Language::Rust => {
            // Generate link from Rust to Python
            let (module_path, item_name) = split_python_path(&xref.python_path);
            let python_page = python_path_to_file_path(&module_path);
            let rust_page = rust_path_to_file_path(from_path);
            let relative = compute_relative_path(&format!("rust/{}", rust_page), &python_page);
            let anchor = item_to_anchor(&item_name, "class");

            CrossRefLink {
                text: item_name,
                url: format!("{}#{}", relative, anchor),
                relationship: xref.relationship.clone(),
            }
        }
    }
}

/// Represents a generated cross-reference link
#[derive(Debug, Clone)]
pub struct CrossRefLink {
    /// Display text for the link
    pub text: String,
    /// Relative URL path
    pub url: String,
    /// Type of cross-reference relationship
    pub relationship: CrossRefKind,
}

impl CrossRefLink {
    /// Render as a Markdown link
    pub fn to_markdown(&self) -> String {
        format!("[{}]({})", self.text, self.url)
    }

    /// Render as a Markdown link with relationship indicator
    pub fn to_markdown_with_badge(&self) -> String {
        let indicator = match self.relationship {
            CrossRefKind::Binding => "[binding]",
            CrossRefKind::Wraps => "[wraps]",
            CrossRefKind::Delegates => "[delegates]",
        };
        format!("{} [{}]({})", indicator, self.text, self.url)
    }
}

/// Language enum for determining link direction
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Language {
    Python,
    Rust,
}

/// Render a collapsible details block with a link to the Rust implementation.
///
/// This is an enhanced version that includes an actual clickable link.
pub fn render_rust_impl_details(rust_ref: &RustItemRef, from_python_path: &str) -> String {
    let link = link_to_rust(rust_ref, from_python_path);

    format!(
        "<details>\n\
         <summary>Rust Implementation</summary>\n\n\
         Implemented by {} in `{}`\n\n\
         </details>",
        link, rust_ref.path
    )
}

/// Render a collapsible details block with a link to the Python exposure.
///
/// For Rust items that are exposed to Python.
pub fn render_python_exposure_details(python_path: &str, from_rust_path: &str) -> String {
    let link = link_to_python(python_path, from_rust_path);

    format!(
        "<details>\n\
         <summary>Python API</summary>\n\n\
         Exposed as {} in `{}`\n\n\
         </details>",
        link, python_path
    )
}

// =============================================================================
// Path Utilities
// =============================================================================

/// Convert a Rust module path to a file path.
///
/// `crate::utils::helpers` -> `crate/utils/helpers.md`
fn rust_path_to_file_path(rust_path: &str) -> String {
    format!("{}.md", rust_path.replace("::", "/"))
}

/// Convert a Python module path to a file path.
///
/// `mypackage.utils.helpers` -> `python/mypackage/utils/helpers.md`
fn python_path_to_file_path(python_path: &str) -> String {
    format!("python/{}.md", python_path.replace('.', "/"))
}

/// Split a Python path into module path and item name.
///
/// `mypackage.utils.Config` -> ("mypackage.utils", "Config")
fn split_python_path(path: &str) -> (String, String) {
    if let Some(pos) = path.rfind('.') {
        (path[..pos].to_string(), path[pos + 1..].to_string())
    } else {
        (path.to_string(), path.to_string())
    }
}

/// Split a Rust path into module path and item name.
///
/// `crate::utils::Config` -> ("crate::utils", "Config")
fn split_rust_path(path: &str) -> (String, String) {
    if let Some(pos) = path.rfind("::") {
        (path[..pos].to_string(), path[pos + 2..].to_string())
    } else {
        (path.to_string(), path.to_string())
    }
}

/// Convert an item name to a Markdown anchor.
///
/// Uses the common Markdown anchor format: lowercase, spaces to hyphens.
fn item_to_anchor(name: &str, item_type: &str) -> String {
    format!("{}-{}", item_type, name.to_lowercase().replace(' ', "-"))
}

/// Compute relative path from one documentation page to another.
///
/// Both paths should be relative to the docs root.
fn compute_relative_path(from_path: &str, to_path: &str) -> String {
    // Count directory depth of from_path
    let from_depth = from_path.matches('/').count();

    // Build the relative prefix (../ for each directory level)
    let prefix = if from_depth > 0 {
        "../".repeat(from_depth)
    } else {
        "./".to_string()
    };

    format!("{}{}", prefix, to_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::RustItemRef;

    #[test]
    fn test_rust_path_to_file_path() {
        assert_eq!(rust_path_to_file_path("crate::utils"), "crate/utils.md");
        assert_eq!(
            rust_path_to_file_path("crate::module::sub"),
            "crate/module/sub.md"
        );
    }

    #[test]
    fn test_python_path_to_file_path() {
        assert_eq!(
            python_path_to_file_path("mypackage.utils"),
            "python/mypackage/utils.md"
        );
        assert_eq!(
            python_path_to_file_path("mypackage.sub.module"),
            "python/mypackage/sub/module.md"
        );
    }

    #[test]
    fn test_split_python_path() {
        let (module, item) = split_python_path("mypackage.utils.Config");
        assert_eq!(module, "mypackage.utils");
        assert_eq!(item, "Config");

        let (module, item) = split_python_path("toplevel");
        assert_eq!(module, "toplevel");
        assert_eq!(item, "toplevel");
    }

    #[test]
    fn test_split_rust_path() {
        let (module, item) = split_rust_path("crate::utils::Config");
        assert_eq!(module, "crate::utils");
        assert_eq!(item, "Config");

        let (module, item) = split_rust_path("single");
        assert_eq!(module, "single");
        assert_eq!(item, "single");
    }

    #[test]
    fn test_item_to_anchor() {
        assert_eq!(item_to_anchor("Config", "struct"), "struct-config");
        assert_eq!(item_to_anchor("MyClass", "class"), "class-myclass");
        assert_eq!(item_to_anchor("process_data", "fn"), "fn-process_data");
    }

    #[test]
    fn test_compute_relative_path_same_level() {
        let rel = compute_relative_path("module_a", "module_b.md");
        assert_eq!(rel, "./module_b.md");
    }

    #[test]
    fn test_compute_relative_path_one_level_deep() {
        let rel = compute_relative_path("package/module", "rust/crate/utils.md");
        assert_eq!(rel, "../rust/crate/utils.md");
    }

    #[test]
    fn test_compute_relative_path_two_levels_deep() {
        let rel = compute_relative_path("package/sub/module", "rust/crate/utils.md");
        assert_eq!(rel, "../../rust/crate/utils.md");
    }

    #[test]
    fn test_link_to_rust() {
        let rust_ref = RustItemRef::new("crate::utils", "Config");
        let link = link_to_rust(&rust_ref, "mypackage.config");

        assert!(link.contains("[Config]"));
        assert!(link.contains("rust/crate/utils.md"));
        assert!(link.contains("#struct-config"));
    }

    #[test]
    fn test_link_to_rust_nested_module() {
        let rust_ref = RustItemRef::new("crate::module::sub", "Helper");
        let link = link_to_rust(&rust_ref, "mypackage.helpers");

        assert!(link.contains("[Helper]"));
        assert!(link.contains("rust/crate/module/sub.md"));
        assert!(link.contains("#struct-helper"));
    }

    #[test]
    fn test_link_to_python() {
        let link = link_to_python("mypackage.utils.Config", "crate::utils");

        assert!(link.contains("[Config]"));
        assert!(link.contains("mypackage/utils.md"));
        assert!(link.contains("#class-config"));
    }

    #[test]
    fn test_link_to_python_from_nested_rust() {
        let link = link_to_python("mypackage.Config", "crate::module::sub");

        assert!(link.contains("[Config]"));
        assert!(link.contains("mypackage.md"));
        // Should go up multiple levels from rust/crate/module/sub.md
        assert!(link.contains("../"));
    }

    #[test]
    fn test_crossref_link_from_python() {
        let xref = CrossRef::binding("mypackage.Config", "crate::utils::Config");
        let link = crossref_link(&xref, "mypackage", Language::Python);

        assert_eq!(link.text, "Config");
        assert!(link.url.contains("rust/crate/utils.md"));
        assert!(matches!(link.relationship, CrossRefKind::Binding));
    }

    #[test]
    fn test_crossref_link_from_rust() {
        let xref = CrossRef::binding("mypackage.utils.Config", "crate::utils");
        let link = crossref_link(&xref, "crate::utils", Language::Rust);

        assert_eq!(link.text, "Config");
        assert!(link.url.contains("mypackage/utils.md"));
        assert!(matches!(link.relationship, CrossRefKind::Binding));
    }

    #[test]
    fn test_crossref_link_markdown() {
        let xref = CrossRef::binding("pkg.Class", "crate::Class");
        let link = crossref_link(&xref, "pkg", Language::Python);

        let md = link.to_markdown();
        assert!(md.starts_with("[Class]("));
        assert!(md.contains(".md#"));

        let md_badge = link.to_markdown_with_badge();
        assert!(md_badge.starts_with("[binding]"));
    }

    #[test]
    fn test_crossref_link_wraps_relationship() {
        let xref = CrossRef::wraps("pkg.Wrapper", "crate::Inner");
        let link = crossref_link(&xref, "pkg", Language::Python);

        assert!(matches!(link.relationship, CrossRefKind::Wraps));
        let md_badge = link.to_markdown_with_badge();
        assert!(md_badge.contains("[wraps]"));
    }

    #[test]
    fn test_crossref_link_delegates_relationship() {
        let xref = CrossRef::delegates("pkg.Client", "crate::http::Client");
        let link = crossref_link(&xref, "pkg", Language::Python);

        assert!(matches!(link.relationship, CrossRefKind::Delegates));
        let md_badge = link.to_markdown_with_badge();
        assert!(md_badge.contains("[delegates]"));
    }

    #[test]
    fn test_render_rust_impl_details() {
        let rust_ref = RustItemRef::new("crate::utils", "Config");
        let details = render_rust_impl_details(&rust_ref, "mypackage.config");

        assert!(details.contains("<details>"));
        assert!(details.contains("Rust Implementation"));
        assert!(details.contains("[Config]")); // Should be a link now
        assert!(details.contains("rust/crate/utils.md"));
        assert!(details.contains("</details>"));
    }

    #[test]
    fn test_render_python_exposure_details() {
        let details = render_python_exposure_details("mypackage.utils.Config", "crate::utils");

        assert!(details.contains("<details>"));
        assert!(details.contains("Python API"));
        assert!(details.contains("[Config]")); // Should be a link
        assert!(details.contains("mypackage/utils.md"));
        assert!(details.contains("</details>"));
    }

    #[test]
    fn test_handles_missing_crossref_gracefully() {
        // When there's no cross-reference, we shouldn't crash
        // This is tested by the fact that all functions take explicit references
        // and None cases are handled at the call site
        let rust_ref = RustItemRef::new("crate::standalone", "Foo");
        let link = link_to_rust(&rust_ref, "module");
        assert!(!link.is_empty());
    }
}
