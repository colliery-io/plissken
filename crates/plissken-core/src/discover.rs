//! Python module auto-discovery
//!
//! Walks the filesystem to find Python modules, converting file paths
//! to dotted module names.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::config::ModuleSourceType;

/// A discovered Python module
#[derive(Debug, Clone)]
pub struct DiscoveredModule {
    /// Dotted module name (e.g., "mypackage.utils.helpers")
    pub name: String,
    /// Path to the Python file
    pub path: PathBuf,
    /// Detected module type (Python or PyO3)
    pub module_type: ModuleSourceType,
}

/// Directories to skip during discovery
const SKIP_DIRS: &[&str] = &[
    "__pycache__",
    ".venv",
    "venv",
    ".env",
    "env",
    ".tox",
    ".nox",
    ".pytest_cache",
    ".mypy_cache",
    ".ruff_cache",
    "node_modules",
    ".git",
    "build",
    "dist",
    "egg-info",
];

/// Discover Python modules by walking the filesystem.
///
/// # Arguments
/// * `source_dir` - The directory to search for Python files
/// * `package_name` - The root package name for module path generation
///
/// # Returns
/// A vector of discovered modules with their dotted names and paths.
pub fn discover_python_modules(
    source_dir: &Path,
    package_name: &str,
) -> Result<Vec<DiscoveredModule>, std::io::Error> {
    let mut modules = Vec::new();

    if !source_dir.exists() {
        return Ok(modules);
    }

    for entry in WalkDir::new(source_dir)
        .follow_links(true)
        .into_iter()
        .filter_entry(|e| !should_skip_entry(e))
    {
        let entry = entry?;
        let path = entry.path();

        // Only process .py files
        if path.extension().map(|e| e == "py").unwrap_or(false) {
            if let Some(module) = path_to_module(path, source_dir, package_name) {
                modules.push(module);
            }
        }
    }

    // Sort modules by name for consistent ordering
    modules.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(modules)
}

/// Check if an entry should be skipped during directory traversal.
fn should_skip_entry(entry: &walkdir::DirEntry) -> bool {
    let file_name = entry.file_name().to_string_lossy();

    // Skip hidden files/directories (except the source dir itself)
    if file_name.starts_with('.') && entry.depth() > 0 {
        return true;
    }

    // Skip known non-module directories
    if entry.file_type().is_dir() {
        if SKIP_DIRS.iter().any(|&skip| file_name == skip) {
            return true;
        }
        // Skip directories ending in .egg-info
        if file_name.ends_with(".egg-info") {
            return true;
        }
    }

    false
}

/// Convert a file path to a Python module.
fn path_to_module(
    file_path: &Path,
    source_dir: &Path,
    package_name: &str,
) -> Option<DiscoveredModule> {
    // Get relative path from source directory
    let relative = file_path.strip_prefix(source_dir).ok()?;

    // Convert path to module name
    let module_name = path_to_module_name(relative, package_name)?;

    // Detect module type by scanning file content
    let module_type = detect_module_type(file_path);

    Some(DiscoveredModule {
        name: module_name,
        path: file_path.to_owned(),
        module_type,
    })
}

/// Convert a relative file path to a dotted module name.
///
/// Examples:
/// - `mypackage/__init__.py` → `mypackage`
/// - `mypackage/utils.py` → `mypackage.utils`
/// - `mypackage/sub/helpers.py` → `mypackage.sub.helpers`
fn path_to_module_name(relative_path: &Path, package_name: &str) -> Option<String> {
    let mut components: Vec<&str> = Vec::new();

    for component in relative_path.components() {
        if let std::path::Component::Normal(name) = component {
            let name_str = name.to_str()?;
            components.push(name_str);
        }
    }

    if components.is_empty() {
        return None;
    }

    // Remove .py extension from the last component
    let last_idx = components.len() - 1;
    let last = components[last_idx];
    let last_without_ext = last.strip_suffix(".py")?;

    // Handle __init__.py - represents the package itself
    if last_without_ext == "__init__" {
        if components.len() == 1 {
            // Root __init__.py
            return Some(package_name.to_string());
        }
        // Sub-package __init__.py - remove the __init__ part
        components.pop();
    } else {
        components[last_idx] = last_without_ext;
    }

    if components.is_empty() {
        return Some(package_name.to_string());
    }

    // Check if the first component matches the package name
    // If source dir already contains the package, don't duplicate
    if components[0] == package_name {
        Some(components.join("."))
    } else {
        // Prepend package name
        Some(format!("{}.{}", package_name, components.join(".")))
    }
}

/// Detect if a Python file is a PyO3 stub module.
///
/// Looks for markers that indicate the module imports from a native extension:
/// - Import from a module with underscore prefix (e.g., `from ._native import`)
/// - Comment marker `# pyo3` or `# pyo3-stub`
fn detect_module_type(file_path: &Path) -> ModuleSourceType {
    // Read the first part of the file to check for markers
    if let Ok(content) = std::fs::read_to_string(file_path) {
        // Only check the first ~2KB for performance
        let preview = if content.len() > 2048 {
            &content[..2048]
        } else {
            &content
        };

        // Check for PyO3 markers
        if preview.contains("# pyo3")
            || preview.contains("#pyo3")
            || preview.contains("# type: ignore[import]")  // Common in PyO3 stubs
        {
            return ModuleSourceType::Pyo3;
        }

        // Check for imports from native modules (underscore prefix convention)
        for line in preview.lines() {
            let line = line.trim();
            if (line.starts_with("from ._") || line.starts_with("from _"))
                && line.contains(" import ")
            {
                return ModuleSourceType::Pyo3;
            }
        }
    }

    ModuleSourceType::Python
}

/// Merge discovered modules with explicitly configured modules.
///
/// Explicit modules take precedence over discovered ones.
pub fn merge_modules(
    discovered: Vec<DiscoveredModule>,
    explicit: &HashMap<String, ModuleSourceType>,
) -> HashMap<String, ModuleSourceType> {
    let mut result: HashMap<String, ModuleSourceType> = discovered
        .into_iter()
        .map(|m| (m.name, m.module_type))
        .collect();

    // Explicit modules override discovered ones
    for (name, module_type) in explicit {
        result.insert(name.clone(), module_type.clone());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_path_to_module_name_simple() {
        let path = Path::new("utils.py");
        assert_eq!(
            path_to_module_name(path, "mypackage"),
            Some("mypackage.utils".to_string())
        );
    }

    #[test]
    fn test_path_to_module_name_nested() {
        let path = Path::new("sub/helpers.py");
        assert_eq!(
            path_to_module_name(path, "mypackage"),
            Some("mypackage.sub.helpers".to_string())
        );
    }

    #[test]
    fn test_path_to_module_name_init() {
        let path = Path::new("__init__.py");
        assert_eq!(
            path_to_module_name(path, "mypackage"),
            Some("mypackage".to_string())
        );
    }

    #[test]
    fn test_path_to_module_name_subpackage_init() {
        let path = Path::new("sub/__init__.py");
        assert_eq!(
            path_to_module_name(path, "mypackage"),
            Some("mypackage.sub".to_string())
        );
    }

    #[test]
    fn test_path_to_module_name_with_package_in_path() {
        let path = Path::new("mypackage/utils.py");
        assert_eq!(
            path_to_module_name(path, "mypackage"),
            Some("mypackage.utils".to_string())
        );
    }

    #[test]
    fn test_discover_python_modules() {
        let temp_dir = TempDir::new().unwrap();
        let pkg_dir = temp_dir.path().join("mypackage");
        std::fs::create_dir(&pkg_dir).unwrap();

        // Create some Python files
        std::fs::write(pkg_dir.join("__init__.py"), "").unwrap();
        std::fs::write(pkg_dir.join("utils.py"), "def helper(): pass").unwrap();
        std::fs::write(pkg_dir.join("core.py"), "class Engine: pass").unwrap();

        // Create a subpackage
        let sub_dir = pkg_dir.join("sub");
        std::fs::create_dir(&sub_dir).unwrap();
        std::fs::write(sub_dir.join("__init__.py"), "").unwrap();
        std::fs::write(sub_dir.join("helpers.py"), "").unwrap();

        // Create a __pycache__ directory (should be skipped)
        let pycache = pkg_dir.join("__pycache__");
        std::fs::create_dir(&pycache).unwrap();
        std::fs::write(pycache.join("utils.cpython-311.pyc"), "").unwrap();

        let modules = discover_python_modules(&pkg_dir, "mypackage").unwrap();

        let names: Vec<&str> = modules.iter().map(|m| m.name.as_str()).collect();
        assert!(names.contains(&"mypackage"));
        assert!(names.contains(&"mypackage.utils"));
        assert!(names.contains(&"mypackage.core"));
        assert!(names.contains(&"mypackage.sub"));
        assert!(names.contains(&"mypackage.sub.helpers"));
        // Should NOT contain pycache files
        assert!(!names.iter().any(|n| n.contains("pycache")));
    }

    #[test]
    fn test_detect_module_type_python() {
        let temp_dir = TempDir::new().unwrap();
        let file = temp_dir.path().join("module.py");
        std::fs::write(&file, "def foo(): pass\n").unwrap();

        assert!(matches!(detect_module_type(&file), ModuleSourceType::Python));
    }

    #[test]
    fn test_detect_module_type_pyo3_marker() {
        let temp_dir = TempDir::new().unwrap();
        let file = temp_dir.path().join("module.py");
        std::fs::write(&file, "# pyo3\nfrom ._native import Foo\n").unwrap();

        assert!(matches!(detect_module_type(&file), ModuleSourceType::Pyo3));
    }

    #[test]
    fn test_detect_module_type_native_import() {
        let temp_dir = TempDir::new().unwrap();
        let file = temp_dir.path().join("module.py");
        std::fs::write(&file, "from ._impl import SomeClass\n").unwrap();

        assert!(matches!(detect_module_type(&file), ModuleSourceType::Pyo3));
    }

    #[test]
    fn test_merge_modules() {
        let discovered = vec![
            DiscoveredModule {
                name: "pkg.a".to_string(),
                path: PathBuf::from("a.py"),
                module_type: ModuleSourceType::Python,
            },
            DiscoveredModule {
                name: "pkg.b".to_string(),
                path: PathBuf::from("b.py"),
                module_type: ModuleSourceType::Python,
            },
        ];

        let mut explicit = HashMap::new();
        explicit.insert("pkg.b".to_string(), ModuleSourceType::Pyo3); // Override
        explicit.insert("pkg.c".to_string(), ModuleSourceType::Python); // Add new

        let merged = merge_modules(discovered, &explicit);

        assert_eq!(merged.len(), 3);
        assert!(matches!(merged.get("pkg.a"), Some(ModuleSourceType::Python)));
        assert!(matches!(merged.get("pkg.b"), Some(ModuleSourceType::Pyo3))); // Overridden
        assert!(matches!(merged.get("pkg.c"), Some(ModuleSourceType::Python))); // Added
    }
}
