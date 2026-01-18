//! Cross-reference link generation for bidirectional Python/Rust documentation.
//!
//! This module handles the computation of cross-reference links between
//! Python and Rust documentation pages, enabling seamless navigation
//! between binding implementations and their Python APIs.

use crate::model::CrossRef;

// =============================================================================
// Path Computation Helpers
// =============================================================================

/// Compute the relative file path for a Python module page.
///
/// Single segment: `mymodule` -> `mymodule.md`
/// Nested: `mypackage.sub.module` -> `mypackage/sub/module.md`
fn compute_python_page_path(module_path: &str) -> String {
    let parts: Vec<&str> = module_path.split('.').collect();
    if parts.len() == 1 {
        format!("{}.md", parts[0])
    } else {
        let last = parts.last().unwrap();
        let parent = parts[..parts.len() - 1].join("/");
        format!("{}/{}.md", parent, last)
    }
}

/// Compute the relative file path for a Rust module page.
///
/// Single segment: `mycrate` -> `rust/mycrate.md`
/// Nested: `mycrate::sub::module` -> `rust/mycrate/sub/module.md`
fn compute_rust_page_path(module_path: &str) -> String {
    format!("rust/{}.md", module_path.replace("::", "/"))
}

/// Compute the `../` prefix to navigate from a Rust module to the root.
///
/// The depth is 1 (for `rust/` directory) plus the number of `::` separators.
fn compute_rust_relative_prefix(rust_path: &str) -> String {
    let depth = 1 + rust_path.matches("::").count();
    "../".repeat(depth)
}

/// Compute the `../` prefix to navigate from a Python module to the root.
///
/// Single segment modules (e.g., `mypackage`) are at root level (depth 0).
/// Nested modules need to go up for each parent directory.
fn compute_python_relative_prefix(python_path: &str) -> String {
    let parts: Vec<&str> = python_path.split('.').collect();
    let depth = if parts.len() == 1 {
        0
    } else {
        parts.len() - 1
    };
    "../".repeat(depth)
}

/// Generates cross-reference links between Python and Rust documentation.
///
/// `CrossRefLinker` encapsulates the logic for finding and formatting
/// links between Python classes/functions and their Rust implementations,
/// and vice versa.
///
/// # Example
///
/// ```rust
/// use plissken_core::render::module::CrossRefLinker;
/// use plissken_core::model::{CrossRef, CrossRefKind};
///
/// let cross_refs = vec![
///     CrossRef {
///         python_path: "mypackage.MyClass".to_string(),
///         rust_path: "mycrate::MyStruct".to_string(),
///         relationship: CrossRefKind::Binding,
///     },
/// ];
///
/// let linker = CrossRefLinker::new(cross_refs);
///
/// // Generate link from Rust struct page to Python class
/// let link = linker.python_link_for_rust_struct("mycrate", "MyStruct");
/// assert!(link.is_some());
/// ```
#[derive(Debug, Clone, Default)]
pub struct CrossRefLinker {
    cross_refs: Vec<CrossRef>,
}

impl CrossRefLinker {
    /// Create a new CrossRefLinker with the given cross-references.
    pub fn new(cross_refs: Vec<CrossRef>) -> Self {
        Self { cross_refs }
    }

    /// Create an empty CrossRefLinker (no cross-references).
    pub fn empty() -> Self {
        Self::default()
    }

    /// Check if any cross-references are available.
    pub fn has_refs(&self) -> bool {
        !self.cross_refs.is_empty()
    }

    // =========================================================================
    // Rust -> Python Links
    // =========================================================================

    /// Find cross-ref for a Rust method and generate link to Python counterpart (inline format).
    ///
    /// For methods (parent_struct is Some), links to the method anchor within the Python module page.
    /// For standalone functions (parent_struct is None), links to the function anchor.
    pub fn python_link_for_rust_method(
        &self,
        rust_path: &str,
        method_name: &str,
        parent_struct: Option<&str>,
    ) -> Option<String> {
        match parent_struct {
            Some(struct_name) => {
                // Look up the parent struct's cross-ref
                let full_rust_path = format!("{}::{}", rust_path, struct_name);
                for xref in &self.cross_refs {
                    if xref.rust_path == full_rust_path
                        || xref.rust_path.ends_with(&format!("::{}", struct_name))
                    {
                        // Get Python module and class name
                        let (python_module, _python_class) =
                            if let Some(pos) = xref.python_path.rfind('.') {
                                (&xref.python_path[..pos], &xref.python_path[pos + 1..])
                            } else {
                                (xref.python_path.as_str(), xref.python_path.as_str())
                            };

                        // Compute path to Python module page
                        let prefix = compute_rust_relative_prefix(rust_path);
                        let python_page = compute_python_page_path(python_module);
                        let anchor = method_name.to_lowercase();
                        let python_method_path = format!("{}.{}", xref.python_path, method_name);
                        return Some(format!(
                            "> **Python API**: [{}]({}{}#{})\n\n",
                            python_method_path, prefix, python_page, anchor
                        ));
                    }
                }
                None
            }
            None => {
                // Fall back to function linking for standalone functions
                self.python_link_for_rust_function(rust_path, method_name)
            }
        }
    }

    /// Find cross-ref for a Rust function and generate link to Python function (inline format).
    ///
    /// With inline rendering, links go to the module page with an anchor for the function.
    /// From: `rust/{crate}.md` -> To: `{module}.md#{funcname}`
    pub fn python_link_for_rust_function(
        &self,
        rust_path: &str,
        func_name: &str,
    ) -> Option<String> {
        let full_rust_path = format!("{}::{}", rust_path, func_name);
        for xref in &self.cross_refs {
            if xref.rust_path == full_rust_path
                || xref.rust_path.ends_with(&format!("::{}", func_name))
            {
                // Get Python module path
                let python_module = if let Some(pos) = xref.python_path.rfind('.') {
                    &xref.python_path[..pos]
                } else {
                    xref.python_path.as_str()
                };

                // Compute path to Python module page
                let prefix = compute_rust_relative_prefix(rust_path);
                let python_page = compute_python_page_path(python_module);
                let anchor = func_name.to_lowercase();

                return Some(format!(
                    "> **Python API**: [{}]({}{}#{})\n\n",
                    xref.python_path, prefix, python_page, anchor
                ));
            }
        }
        None
    }

    /// Find cross-ref for a Rust struct and generate link to Python class (inline format).
    ///
    /// With inline rendering, links go to the module page with an anchor for the class.
    /// From: `rust/{crate}.md` -> To: `{module}.md#class-{classname}`
    ///
    /// Note: The anchor uses the Python class name (e.g., `Task`), not the Rust
    /// struct name (e.g., `PyTask`), because the rendered markdown heading shows
    /// the Python class name.
    pub fn python_link_for_rust_struct(
        &self,
        rust_path: &str,
        struct_name: &str,
    ) -> Option<String> {
        let full_rust_path = format!("{}::{}", rust_path, struct_name);
        for xref in &self.cross_refs {
            if xref.rust_path == full_rust_path
                || xref.rust_path.ends_with(&format!("::{}", struct_name))
            {
                // Get Python module and class name
                let (python_module, python_class) =
                    if let Some(pos) = xref.python_path.rfind('.') {
                        (&xref.python_path[..pos], &xref.python_path[pos + 1..])
                    } else {
                        (xref.python_path.as_str(), xref.python_path.as_str())
                    };

                // Compute path to Python module page
                let prefix = compute_rust_relative_prefix(rust_path);
                let python_page = compute_python_page_path(python_module);
                // Use Python class name for anchor (matches the rendered heading)
                let anchor = python_class.to_lowercase();
                return Some(format!(
                    "> **Python API**: [{}]({}{}#class-{})\n\n",
                    xref.python_path, prefix, python_page, anchor
                ));
            }
        }
        None
    }

    // =========================================================================
    // Python -> Rust Links
    // =========================================================================

    /// Find cross-ref for a Python function and generate link to Rust function (inline format).
    ///
    /// With inline rendering, links go to the Rust module page with an anchor for the function.
    /// From: `{module}.md` -> To: `rust/{crate}.md#fn-{funcname}`
    ///
    /// Note: Rust function headings use `### \`fn funcname\`` format, generating `#fn-funcname` anchors.
    pub fn rust_link_for_python_function(
        &self,
        python_path: &str,
        func_name: &str,
    ) -> Option<String> {
        let full_python_path = format!("{}.{}", python_path, func_name);
        for xref in &self.cross_refs {
            if xref.python_path == full_python_path
                || xref.python_path.ends_with(&format!(".{}", func_name))
            {
                // Get the Rust module and function name
                let (rust_module, rust_func) = if let Some(pos) = xref.rust_path.rfind("::") {
                    (&xref.rust_path[..pos], &xref.rust_path[pos + 2..])
                } else {
                    (xref.rust_path.as_str(), xref.rust_path.as_str())
                };

                // Compute path from Python module page to Rust module page
                let prefix = compute_python_relative_prefix(python_path);
                let rust_page = compute_rust_page_path(rust_module);
                // Rust function anchor includes "fn-" prefix (from heading `### \`fn funcname\``)
                let anchor = format!("fn-{}", rust_func.to_lowercase());

                return Some(format!(
                    "> **Rust Implementation**: [{}]({}{}#{})\n\n",
                    xref.rust_path, prefix, rust_page, anchor
                ));
            }
        }
        None
    }

    /// Find cross-ref for a Python class and generate link to Rust struct (inline format).
    ///
    /// With inline rendering, links go to the Rust module page with an anchor for the struct.
    /// From: `{module}.md` -> To: `rust/{crate}.md#class-{classname}`
    ///
    /// Note: The anchor uses the Python class name (e.g., `Task`), not the Rust
    /// struct name (e.g., `PyTask`), because the Rust doc heading shows the
    /// Python binding name.
    pub fn rust_link_for_python_class(
        &self,
        python_path: &str,
        class_name: &str,
    ) -> Option<String> {
        let full_python_path = format!("{}.{}", python_path, class_name);
        for xref in &self.cross_refs {
            if xref.python_path == full_python_path
                || xref.python_path.ends_with(&format!(".{}", class_name))
            {
                // Get the Rust module
                let rust_module = if let Some(pos) = xref.rust_path.rfind("::") {
                    &xref.rust_path[..pos]
                } else {
                    xref.rust_path.as_str()
                };

                // Compute path from Python module page to Rust module page
                let prefix = compute_python_relative_prefix(python_path);
                let rust_page = compute_rust_page_path(rust_module);
                // Use Python class name for anchor (Rust docs show binding name in heading)
                let anchor = class_name.to_lowercase();

                return Some(format!(
                    "> **Rust Implementation**: [{}]({}{}#class-{})\n\n",
                    xref.rust_path, prefix, rust_page, anchor
                ));
            }
        }
        None
    }

    /// Find cross-ref for a Python method and generate link to Rust implementation (inline format).
    ///
    /// With inline rendering, links go to the Rust module page with an anchor for the method.
    /// From: `{module}.md` -> To: `rust/{crate}.md#{method}`
    ///
    /// For methods (parent_class is Some), looks up the parent class's cross-ref
    /// and generates a link with the method as an anchor.
    /// For standalone functions (parent_class is None), falls back to function linking.
    pub fn rust_link_for_python_method(
        &self,
        python_path: &str,
        method_name: &str,
        parent_class: Option<&str>,
    ) -> Option<String> {
        match parent_class {
            Some(class_name) => {
                // Look up the parent class's cross-ref
                let full_python_path = format!("{}.{}", python_path, class_name);
                for xref in &self.cross_refs {
                    if xref.python_path == full_python_path
                        || xref.python_path.ends_with(&format!(".{}", class_name))
                    {
                        // Get the Rust module
                        let rust_module = if let Some(pos) = xref.rust_path.rfind("::") {
                            &xref.rust_path[..pos]
                        } else {
                            xref.rust_path.as_str()
                        };

                        // Compute path from Python module page to Rust module page
                        let prefix = compute_python_relative_prefix(python_path);
                        let rust_page = compute_rust_page_path(rust_module);
                        let anchor = method_name.to_lowercase();
                        let rust_method_path = format!("{}::{}", xref.rust_path, method_name);

                        return Some(format!(
                            "> **Rust Implementation**: [{}]({}{}#{})\n\n",
                            rust_method_path, prefix, rust_page, anchor
                        ));
                    }
                }
                None
            }
            None => {
                // Fall back to function-level linking for standalone functions
                self.rust_link_for_python_function(python_path, method_name)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::CrossRefKind;

    fn test_cross_refs() -> Vec<CrossRef> {
        vec![
            CrossRef {
                python_path: "mypackage.MyClass".to_string(),
                rust_path: "mycrate::MyStruct".to_string(),
                relationship: CrossRefKind::Binding,
            },
            CrossRef {
                python_path: "mypackage.my_func".to_string(),
                rust_path: "mycrate::my_func".to_string(),
                relationship: CrossRefKind::Binding,
            },
            CrossRef {
                python_path: "mypackage.sub.NestedClass".to_string(),
                rust_path: "mycrate::sub::NestedStruct".to_string(),
                relationship: CrossRefKind::Binding,
            },
        ]
    }

    // =========================================================================
    // Helper Function Tests
    // =========================================================================

    #[test]
    fn test_compute_python_page_path_single_segment() {
        assert_eq!(compute_python_page_path("mypackage"), "mypackage.md");
    }

    #[test]
    fn test_compute_python_page_path_nested() {
        assert_eq!(
            compute_python_page_path("mypackage.sub"),
            "mypackage/sub.md"
        );
        assert_eq!(
            compute_python_page_path("mypackage.sub.deep"),
            "mypackage/sub/deep.md"
        );
    }

    #[test]
    fn test_compute_rust_page_path_single_segment() {
        assert_eq!(compute_rust_page_path("mycrate"), "rust/mycrate.md");
    }

    #[test]
    fn test_compute_rust_page_path_nested() {
        assert_eq!(
            compute_rust_page_path("mycrate::sub"),
            "rust/mycrate/sub.md"
        );
        assert_eq!(
            compute_rust_page_path("mycrate::sub::deep"),
            "rust/mycrate/sub/deep.md"
        );
    }

    #[test]
    fn test_compute_rust_relative_prefix() {
        // Single crate: depth 1 (rust/) -> "../"
        assert_eq!(compute_rust_relative_prefix("mycrate"), "../");
        // One submodule: depth 2 (rust/mycrate/) -> "../../"
        assert_eq!(compute_rust_relative_prefix("mycrate::sub"), "../../");
        // Two submodules: depth 3 -> "../../../"
        assert_eq!(
            compute_rust_relative_prefix("mycrate::sub::deep"),
            "../../../"
        );
    }

    #[test]
    fn test_compute_python_relative_prefix() {
        // Single module at root: depth 0 -> ""
        assert_eq!(compute_python_relative_prefix("mypackage"), "");
        // One submodule: depth 1 -> "../"
        assert_eq!(compute_python_relative_prefix("mypackage.sub"), "../");
        // Two submodules: depth 2 -> "../../"
        assert_eq!(
            compute_python_relative_prefix("mypackage.sub.deep"),
            "../../"
        );
    }

    // =========================================================================
    // CrossRefLinker Tests
    // =========================================================================

    #[test]
    fn test_empty_linker() {
        let linker = CrossRefLinker::empty();
        assert!(!linker.has_refs());
        assert!(linker
            .python_link_for_rust_struct("mycrate", "MyStruct")
            .is_none());
    }

    #[test]
    fn test_has_refs() {
        let linker = CrossRefLinker::new(test_cross_refs());
        assert!(linker.has_refs());
    }

    #[test]
    fn test_python_link_for_rust_struct() {
        let linker = CrossRefLinker::new(test_cross_refs());

        let link = linker
            .python_link_for_rust_struct("mycrate", "MyStruct")
            .unwrap();
        assert!(link.contains("**Python API**"));
        assert!(link.contains("mypackage.MyClass"));
        // Inline format: links to module page with class anchor
        assert!(link.contains("mypackage.md#class-myclass"));
    }

    #[test]
    fn test_python_link_for_rust_function() {
        let linker = CrossRefLinker::new(test_cross_refs());

        let link = linker
            .python_link_for_rust_function("mycrate", "my_func")
            .unwrap();
        assert!(link.contains("**Python API**"));
        assert!(link.contains("mypackage.my_func"));
        // Inline format: links to module page with function anchor (underscores preserved)
        assert!(link.contains("mypackage.md#my_func"));
    }

    #[test]
    fn test_python_link_for_rust_method() {
        let linker = CrossRefLinker::new(test_cross_refs());

        // Method with parent struct
        let link = linker
            .python_link_for_rust_method("mycrate", "do_something", Some("MyStruct"))
            .unwrap();
        assert!(link.contains("**Python API**"));
        assert!(link.contains("mypackage.MyClass.do_something"));
        assert!(link.contains("#do_something")); // anchor with underscores preserved

        // Standalone function (no parent)
        let link = linker
            .python_link_for_rust_method("mycrate", "my_func", None)
            .unwrap();
        assert!(link.contains("mypackage.my_func"));
    }

    #[test]
    fn test_rust_link_for_python_class() {
        let linker = CrossRefLinker::new(test_cross_refs());

        let link = linker
            .rust_link_for_python_class("mypackage", "MyClass")
            .unwrap();
        assert!(link.contains("**Rust Implementation**"));
        assert!(link.contains("mycrate::MyStruct"));
        // Inline format: links to module page with Python class name as anchor
        // (Rust docs show the Python binding name in headings)
        assert!(link.contains("rust/mycrate.md#class-myclass"));
    }

    #[test]
    fn test_rust_link_for_python_function() {
        let linker = CrossRefLinker::new(test_cross_refs());

        let link = linker
            .rust_link_for_python_function("mypackage", "my_func")
            .unwrap();
        assert!(link.contains("**Rust Implementation**"));
        assert!(link.contains("mycrate::my_func"));
        // Inline format: links to module page with fn- prefix (Rust heading format)
        assert!(link.contains("rust/mycrate.md#fn-my_func"));
    }

    #[test]
    fn test_rust_link_for_python_method() {
        let linker = CrossRefLinker::new(test_cross_refs());

        // Method with parent class
        let link = linker
            .rust_link_for_python_method("mypackage", "do_something", Some("MyClass"))
            .unwrap();
        assert!(link.contains("**Rust Implementation**"));
        assert!(link.contains("mycrate::MyStruct::do_something"));
        assert!(link.contains("#do_something")); // anchor with underscores preserved

        // Standalone function (no parent)
        let link = linker
            .rust_link_for_python_method("mypackage", "my_func", None)
            .unwrap();
        assert!(link.contains("mycrate::my_func"));
    }

    #[test]
    fn test_nested_module_links() {
        let linker = CrossRefLinker::new(test_cross_refs());

        // Nested Python -> Rust (inline format)
        // Anchor uses Python class name (NestedClass), not Rust struct name
        let link = linker
            .rust_link_for_python_class("mypackage.sub", "NestedClass")
            .unwrap();
        assert!(link.contains("mycrate::sub::NestedStruct"));
        assert!(link.contains("rust/mycrate/sub.md#class-nestedclass"));

        // Nested Rust -> Python (inline format)
        let link = linker
            .python_link_for_rust_struct("mycrate::sub", "NestedStruct")
            .unwrap();
        assert!(link.contains("mypackage.sub.NestedClass"));
        assert!(link.contains("mypackage/sub.md#class-nestedclass"));
    }

    #[test]
    fn test_no_match_returns_none() {
        let linker = CrossRefLinker::new(test_cross_refs());

        assert!(linker
            .python_link_for_rust_struct("mycrate", "Unknown")
            .is_none());
        assert!(linker
            .rust_link_for_python_class("mypackage", "Unknown")
            .is_none());
    }
}
