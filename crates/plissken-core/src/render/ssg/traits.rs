//! SSG adapter trait definition

use crate::model::{PythonModule, RustModule};
use std::path::PathBuf;

/// Entry for a module in the navigation
pub struct NavEntry {
    /// Module path (dotted for Python, :: for Rust)
    pub path: String,
    /// File path for the documentation page
    pub file_path: PathBuf,
    /// Nesting depth for hierarchical display
    pub depth: usize,
}

/// Compute the file path for a Python module page (inline format).
///
/// For top-level modules, returns just `{module}.md`.
/// For nested modules, returns `{parent}/{module}.md`.
fn python_module_page(module_path: &str) -> PathBuf {
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

/// Compute the file path for a Rust module page (inline format).
///
/// For crate roots (no `::` in path), returns `rust/{crate_name}.md`.
/// For submodules, returns `rust/{crate_name}/{submodule}.md`.
fn rust_module_page(module_path: &str) -> PathBuf {
    if !module_path.contains("::") {
        // Crate root - use crate_name.md directly
        PathBuf::from(format!("rust/{}.md", module_path))
    } else {
        // Submodule - convert :: to /
        let path = module_path.replace("::", "/");
        PathBuf::from(format!("rust/{}.md", path))
    }
}

/// Generate sorted navigation entries for Python modules
pub fn python_nav_entries(modules: &[PythonModule]) -> Vec<NavEntry> {
    let mut sorted: Vec<&PythonModule> = modules.iter().collect();
    sorted.sort_by(|a, b| a.path.cmp(&b.path));

    sorted
        .iter()
        .map(|m| NavEntry {
            path: m.path.clone(),
            file_path: python_module_page(&m.path),
            depth: m.path.matches('.').count(),
        })
        .collect()
}

/// Generate sorted navigation entries for Rust modules
pub fn rust_nav_entries(modules: &[RustModule]) -> Vec<NavEntry> {
    let mut sorted: Vec<&RustModule> = modules.iter().collect();
    sorted.sort_by(|a, b| a.path.cmp(&b.path));

    sorted
        .iter()
        .map(|m| NavEntry {
            path: m.path.clone(),
            file_path: rust_module_page(&m.path),
            depth: m.path.matches("::").count(),
        })
        .collect()
}

/// Adapter for static site generator output.
///
/// This trait abstracts the differences between static site generators,
/// providing a unified interface for navigation generation, config files,
/// and directory structure.
///
/// # Implementors
///
/// - [`MkDocsAdapter`](super::MkDocsAdapter) - For MkDocs with Material theme
/// - [`MdBookAdapter`](super::MdBookAdapter) - For mdBook
pub trait SSGAdapter: Send + Sync {
    /// Human-readable name for the SSG.
    fn name(&self) -> &'static str;

    /// Directory where content files are placed (relative to output root).
    ///
    /// - MkDocs: `"docs"`
    /// - mdBook: `"src"`
    fn content_dir(&self) -> &'static str;

    /// Navigation file name.
    ///
    /// - MkDocs: `"_nav.yml"` (included in mkdocs.yml)
    /// - mdBook: `"SUMMARY.md"`
    fn nav_filename(&self) -> &'static str;

    /// Generate navigation content from modules.
    ///
    /// Returns the navigation content in the SSG's expected format:
    /// - MkDocs: YAML format for the `nav:` section
    /// - mdBook: Markdown format for SUMMARY.md
    fn generate_nav(&self, python_modules: &[PythonModule], rust_modules: &[RustModule]) -> String;

    /// Generate SSG config file content.
    ///
    /// - MkDocs: Returns `None` (mkdocs.yml typically pre-exists)
    /// - mdBook: Returns book.toml content
    fn generate_config(&self, title: &str, authors: &[String]) -> Option<String>;

    /// Generate custom CSS for the SSG, if any.
    ///
    /// Returns CSS content that should be added to customize the documentation
    /// appearance, or None if no custom CSS is needed.
    fn generate_custom_css(&self) -> Option<String> {
        None
    }

    /// Path for custom CSS file, if generated.
    ///
    /// - mdBook: `"theme/custom.css"`
    fn custom_css_path(&self) -> Option<&'static str> {
        None
    }

    /// File extension for content files.
    fn content_extension(&self) -> &'static str {
        "md"
    }

    /// Whether this SSG supports nested/hierarchical navigation.
    fn supports_nested_nav(&self) -> bool {
        true
    }
}
