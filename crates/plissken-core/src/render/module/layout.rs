//! Page layout and path computation for documentation output.
//!
//! This module handles the translation of module/item names to file paths,
//! providing a consistent mapping between documentation structure and
//! filesystem organization.

use std::path::PathBuf;

/// Computes file paths and directory structure for documentation pages.
///
/// `PageLayout` encapsulates the rules for translating Python and Rust
/// module paths into filesystem paths for the generated documentation.
///
/// # Path Rules
///
/// - **Python modules**: `python/{module.path.replace('.', '/')}/`
///   - Example: `mypackage.submodule` → `python/mypackage/submodule/`
/// - **Rust modules**: `rust/{module.path.replace('::', '/')}/`
///   - Example: `mycrate::submod` → `rust/mycrate/submod/`
/// - **Index pages**: `{module_dir}/index.md`
/// - **Item pages**: `{module_dir}/{item_name}.md`
///
/// # Example
///
/// ```rust
/// use plissken_core::render::module::PageLayout;
/// use std::path::PathBuf;
///
/// let layout = PageLayout::new();
///
/// // Python module paths
/// assert_eq!(
///     layout.python_module_dir("mypackage.submodule"),
///     "python/mypackage/submodule"
/// );
/// assert_eq!(
///     layout.python_index_path("mypackage.submodule"),
///     PathBuf::from("python/mypackage/submodule/index.md")
/// );
/// assert_eq!(
///     layout.python_item_path("mypackage.submodule", "MyClass"),
///     PathBuf::from("python/mypackage/submodule/MyClass.md")
/// );
///
/// // Rust module paths
/// assert_eq!(
///     layout.rust_module_dir("mycrate::submod"),
///     "rust/mycrate/submod"
/// );
/// ```
#[derive(Debug, Clone, Default)]
pub struct PageLayout;

impl PageLayout {
    /// Create a new PageLayout.
    pub fn new() -> Self {
        Self
    }

    // =========================================================================
    // Python Path Computation
    // =========================================================================

    /// Compute the directory path for a Python module.
    ///
    /// Converts dots to slashes and prepends "python/".
    ///
    /// # Example
    ///
    /// ```rust
    /// use plissken_core::render::module::PageLayout;
    ///
    /// let layout = PageLayout::new();
    /// assert_eq!(layout.python_module_dir("mypackage.sub"), "python/mypackage/sub");
    /// ```
    pub fn python_module_dir(&self, module_path: &str) -> String {
        let parts: Vec<&str> = module_path.split('.').collect();
        format!("python/{}", parts.join("/"))
    }

    /// Compute the path for a Python module's index page.
    pub fn python_index_path(&self, module_path: &str) -> PathBuf {
        PathBuf::from(format!("{}/index.md", self.python_module_dir(module_path)))
    }

    /// Compute the path for a Python module as a single page (inline format).
    ///
    /// For top-level modules, returns just `{module}.md`.
    /// For nested modules, returns `{parent}/{module}.md`.
    pub fn python_module_page(&self, module_path: &str) -> PathBuf {
        let parts: Vec<&str> = module_path.split('.').collect();
        if parts.len() == 1 {
            // Top-level module: just module_name.md
            PathBuf::from(format!("{}.md", parts[0]))
        } else {
            // Nested module: parent/child.md
            let last = parts.last().unwrap();
            let parent_parts = &parts[..parts.len() - 1];
            PathBuf::from(format!("{}/{}.md", parent_parts.join("/"), last))
        }
    }

    /// Compute the path for a Python item (class, function) page.
    pub fn python_item_path(&self, module_path: &str, item_name: &str) -> PathBuf {
        PathBuf::from(format!(
            "{}/{}.md",
            self.python_module_dir(module_path),
            item_name
        ))
    }

    /// Compute the depth of a Python module (number of dots).
    ///
    /// Used for indentation in navigation.
    pub fn python_module_depth(&self, module_path: &str) -> usize {
        module_path.matches('.').count()
    }

    // =========================================================================
    // Rust Path Computation
    // =========================================================================

    /// Compute the directory path for a Rust module.
    ///
    /// Converts `::` to `/` and prepends "rust/".
    ///
    /// # Example
    ///
    /// ```rust
    /// use plissken_core::render::module::PageLayout;
    ///
    /// let layout = PageLayout::new();
    /// assert_eq!(layout.rust_module_dir("mycrate::submod"), "rust/mycrate/submod");
    /// ```
    pub fn rust_module_dir(&self, module_path: &str) -> String {
        let parts: Vec<&str> = module_path.split("::").collect();
        format!("rust/{}", parts.join("/"))
    }

    /// Compute the path for a Rust module's index page.
    pub fn rust_index_path(&self, module_path: &str) -> PathBuf {
        PathBuf::from(format!("{}/index.md", self.rust_module_dir(module_path)))
    }

    /// Compute the path for a Rust module as a single page (inline format).
    ///
    /// For crate roots (no `::` in path), returns `rust/{crate_name}.md`.
    /// For submodules, returns `rust/{crate_name}/{submodule}.md`.
    ///
    /// # Examples
    ///
    /// - `plissken_core` → `rust/plissken_core.md` (crate root)
    /// - `plissken_core::config` → `rust/plissken_core/config.md` (submodule)
    /// - `plissken_core::render::ssg` → `rust/plissken_core/render/ssg.md` (nested)
    pub fn rust_module_page(&self, module_path: &str) -> PathBuf {
        if !module_path.contains("::") {
            // Crate root - use crate_name.md directly
            PathBuf::from(format!("rust/{}.md", module_path))
        } else {
            // Submodule - convert :: to /
            let path = module_path.replace("::", "/");
            PathBuf::from(format!("rust/{}.md", path))
        }
    }

    /// Compute the path for a Rust item (struct, enum, function) page.
    pub fn rust_item_path(&self, module_path: &str, item_name: &str) -> PathBuf {
        PathBuf::from(format!(
            "{}/{}.md",
            self.rust_module_dir(module_path),
            item_name
        ))
    }

    /// Compute the depth of a Rust module (number of `::` separators).
    ///
    /// Used for indentation in navigation.
    pub fn rust_module_depth(&self, module_path: &str) -> usize {
        module_path.matches("::").count()
    }

    // =========================================================================
    // Cross-reference Path Computation
    // =========================================================================

    /// Compute a relative path from a Rust module to a Python item.
    ///
    /// Used for cross-reference links between Rust and Python documentation.
    ///
    /// # Arguments
    ///
    /// * `python_module` - The Python module path (e.g., "mypackage.submodule")
    /// * `item_name` - The item name (e.g., "MyClass")
    /// * `anchor` - Optional anchor within the page (e.g., "my_method")
    pub fn python_link_from_rust(
        &self,
        python_module: &str,
        item_name: &str,
        anchor: Option<&str>,
    ) -> String {
        let python_dir = python_module.replace('.', "/");
        let prefix = "../".repeat(2); // rust/crate/file.md -> ../../python/...
        match anchor {
            Some(a) => format!("{}python/{}/{}.md#{}", prefix, python_dir, item_name, a),
            None => format!("{}python/{}/{}.md", prefix, python_dir, item_name),
        }
    }

    /// Compute a relative path from a Python module to a Rust item.
    ///
    /// Used for cross-reference links between Python and Rust documentation.
    ///
    /// # Arguments
    ///
    /// * `rust_module` - The Rust module path (e.g., "mycrate::submod")
    /// * `item_name` - The item name (e.g., "MyStruct")
    /// * `anchor` - Optional anchor within the page (e.g., "my_method")
    pub fn rust_link_from_python(
        &self,
        rust_module: &str,
        item_name: &str,
        anchor: Option<&str>,
    ) -> String {
        let rust_dir = rust_module.replace("::", "/");
        let prefix = "../".repeat(2); // python/pkg/file.md -> ../../rust/...
        match anchor {
            Some(a) => format!("{}rust/{}/{}.md#{}", prefix, rust_dir, item_name, a),
            None => format!("{}rust/{}/{}.md", prefix, rust_dir, item_name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_python_module_dir() {
        let layout = PageLayout::new();

        assert_eq!(layout.python_module_dir("mypackage"), "python/mypackage");
        assert_eq!(
            layout.python_module_dir("mypackage.submodule"),
            "python/mypackage/submodule"
        );
        assert_eq!(
            layout.python_module_dir("a.b.c.d"),
            "python/a/b/c/d"
        );
    }

    #[test]
    fn test_python_index_path() {
        let layout = PageLayout::new();

        assert_eq!(
            layout.python_index_path("mypackage"),
            PathBuf::from("python/mypackage/index.md")
        );
        assert_eq!(
            layout.python_index_path("mypackage.sub"),
            PathBuf::from("python/mypackage/sub/index.md")
        );
    }

    #[test]
    fn test_python_item_path() {
        let layout = PageLayout::new();

        assert_eq!(
            layout.python_item_path("mypackage", "MyClass"),
            PathBuf::from("python/mypackage/MyClass.md")
        );
        assert_eq!(
            layout.python_item_path("mypackage.sub", "my_function"),
            PathBuf::from("python/mypackage/sub/my_function.md")
        );
    }

    #[test]
    fn test_python_module_depth() {
        let layout = PageLayout::new();

        assert_eq!(layout.python_module_depth("mypackage"), 0);
        assert_eq!(layout.python_module_depth("mypackage.sub"), 1);
        assert_eq!(layout.python_module_depth("a.b.c.d"), 3);
    }

    #[test]
    fn test_rust_module_dir() {
        let layout = PageLayout::new();

        assert_eq!(layout.rust_module_dir("mycrate"), "rust/mycrate");
        assert_eq!(
            layout.rust_module_dir("mycrate::submod"),
            "rust/mycrate/submod"
        );
        assert_eq!(
            layout.rust_module_dir("a::b::c::d"),
            "rust/a/b/c/d"
        );
    }

    #[test]
    fn test_rust_index_path() {
        let layout = PageLayout::new();

        assert_eq!(
            layout.rust_index_path("mycrate"),
            PathBuf::from("rust/mycrate/index.md")
        );
        assert_eq!(
            layout.rust_index_path("mycrate::sub"),
            PathBuf::from("rust/mycrate/sub/index.md")
        );
    }

    #[test]
    fn test_rust_item_path() {
        let layout = PageLayout::new();

        assert_eq!(
            layout.rust_item_path("mycrate", "MyStruct"),
            PathBuf::from("rust/mycrate/MyStruct.md")
        );
        assert_eq!(
            layout.rust_item_path("mycrate::sub", "my_function"),
            PathBuf::from("rust/mycrate/sub/my_function.md")
        );
    }

    #[test]
    fn test_rust_module_depth() {
        let layout = PageLayout::new();

        assert_eq!(layout.rust_module_depth("mycrate"), 0);
        assert_eq!(layout.rust_module_depth("mycrate::sub"), 1);
        assert_eq!(layout.rust_module_depth("a::b::c::d"), 3);
    }

    #[test]
    fn test_python_link_from_rust() {
        let layout = PageLayout::new();

        assert_eq!(
            layout.python_link_from_rust("mypackage", "MyClass", None),
            "../../python/mypackage/MyClass.md"
        );
        assert_eq!(
            layout.python_link_from_rust("mypackage.sub", "MyClass", Some("method")),
            "../../python/mypackage/sub/MyClass.md#method"
        );
    }

    #[test]
    fn test_rust_link_from_python() {
        let layout = PageLayout::new();

        assert_eq!(
            layout.rust_link_from_python("mycrate", "MyStruct", None),
            "../../rust/mycrate/MyStruct.md"
        );
        assert_eq!(
            layout.rust_link_from_python("mycrate::sub", "MyStruct", Some("method")),
            "../../rust/mycrate/sub/MyStruct.md#method"
        );
    }
}
