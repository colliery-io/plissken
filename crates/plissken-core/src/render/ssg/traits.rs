//! SSG adapter trait definition

use crate::model::{PythonModule, RustModule};
use std::path::PathBuf;

/// Prepend an optional prefix to a file path string.
/// Returns `"prefix/path"` if prefix is Some, or `"path"` unchanged if None.
pub fn prefix_path(prefix: Option<&str>, path: &str) -> String {
    match prefix {
        Some(p) => format!("{}/{}", p, path),
        None => path.to_string(),
    }
}

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

/// A node in the hierarchical navigation tree.
///
/// Leaf nodes have no children and render as simple links.
/// Branch nodes have children and render as collapsible sections
/// with their own page as the section index.
pub struct NavNode {
    /// Short display name (last segment of the module path)
    pub name: String,
    /// File path for this node's page (with prefix applied)
    pub file_path: String,
    /// Child nodes
    pub children: Vec<NavNode>,
}

impl NavNode {
    /// Whether this node has child nodes (is a section, not a leaf).
    pub fn is_branch(&self) -> bool {
        !self.children.is_empty()
    }
}

/// Extract the short display name (last segment) from a module path.
fn short_name(full_path: &str, separator: &str) -> String {
    full_path
        .rsplit_once(separator)
        .map(|(_, last)| last.to_string())
        .unwrap_or_else(|| full_path.to_string())
}

/// Build a hierarchical navigation tree from a sorted flat list of entries.
///
/// Entries must be sorted alphabetically. The `separator` is `"::"` for Rust
/// and `"."` for Python. An optional `prefix` is prepended to all file paths.
///
/// Modules whose path is a prefix of subsequent entries become branch nodes
/// (collapsible sections). Modules with no children become leaf nodes.
pub fn build_nav_tree(entries: &[NavEntry], prefix: Option<&str>, separator: &str) -> Vec<NavNode> {
    let mut nodes = Vec::new();
    let mut i = 0;

    while i < entries.len() {
        let entry = &entries[i];
        let file_path = prefix_path(prefix, &entry.file_path.display().to_string());
        let name = short_name(&entry.path, separator);

        // Collect children: entries whose path starts with this entry's path + separator
        let child_prefix = format!("{}{}", entry.path, separator);
        let mut children_end = i + 1;
        while children_end < entries.len() && entries[children_end].path.starts_with(&child_prefix)
        {
            children_end += 1;
        }

        let children = if children_end > i + 1 {
            build_nav_tree(&entries[i + 1..children_end], prefix, separator)
        } else {
            Vec::new()
        };

        nodes.push(NavNode {
            name,
            file_path,
            children,
        });

        i = children_end;
    }

    nodes
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
    ///
    /// The optional `prefix` is prepended to all file paths in nav entries,
    /// enabling output to be mounted in a subfolder of an existing doc site.
    /// E.g., `prefix = Some("api")` produces `api/rust/mycrate.md` instead of `rust/mycrate.md`.
    fn generate_nav(
        &self,
        python_modules: &[PythonModule],
        rust_modules: &[RustModule],
        prefix: Option<&str>,
    ) -> String;

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
