//! Manifest file parsing for Cargo.toml and pyproject.toml
//!
//! This module provides functionality to parse project manifest files and
//! extract metadata that can be used to infer default configuration values.

use serde::Deserialize;
use std::path::{Path, PathBuf};
use thiserror::Error;

/// Errors that can occur when parsing manifest files
#[derive(Debug, Error)]
pub enum ManifestError {
    #[error("failed to read file: {0}")]
    IoError(#[from] std::io::Error),

    #[error("failed to parse TOML: {0}")]
    TomlError(#[from] toml::de::Error),
}

/// Parsed Cargo.toml manifest
#[derive(Debug, Clone)]
pub struct CargoManifest {
    /// Package name from [package].name
    pub name: Option<String>,
    /// Package version from [package].version
    pub version: Option<String>,
    /// Whether this is a workspace root (has [workspace] section)
    pub is_workspace: bool,
    /// Workspace members if this is a workspace root
    pub workspace_members: Vec<String>,
}

/// Parsed pyproject.toml manifest
#[derive(Debug, Clone)]
pub struct PyProjectManifest {
    /// Project name from [project].name
    pub name: Option<String>,
    /// Project version from [project].version
    pub version: Option<String>,
    /// Package directory from [tool.setuptools.package-dir] or similar
    pub package_dir: Option<PathBuf>,
}

// Internal structs for TOML parsing

#[derive(Deserialize)]
struct CargoToml {
    package: Option<CargoPackage>,
    workspace: Option<CargoWorkspace>,
}

#[derive(Deserialize)]
struct CargoPackage {
    name: Option<String>,
    version: Option<String>,
}

#[derive(Deserialize)]
struct CargoWorkspace {
    members: Option<Vec<String>>,
}

#[derive(Deserialize)]
struct PyProjectToml {
    project: Option<PyProject>,
    tool: Option<PyProjectTool>,
}

#[derive(Deserialize)]
struct PyProject {
    name: Option<String>,
    version: Option<String>,
}

#[derive(Deserialize)]
struct PyProjectTool {
    setuptools: Option<SetuptoolsConfig>,
    maturin: Option<MaturinConfig>,
}

#[derive(Deserialize)]
struct SetuptoolsConfig {
    #[serde(rename = "package-dir")]
    package_dir: Option<std::collections::HashMap<String, String>>,
}

#[derive(Deserialize)]
struct MaturinConfig {
    #[serde(rename = "python-source")]
    python_source: Option<String>,
    #[serde(rename = "module-name")]
    module_name: Option<String>,
}

impl CargoManifest {
    /// Parse a Cargo.toml file
    pub fn parse(path: &Path) -> Result<Self, ManifestError> {
        let content = std::fs::read_to_string(path)?;
        Self::parse_str(&content)
    }

    /// Parse Cargo.toml content from a string
    pub fn parse_str(content: &str) -> Result<Self, ManifestError> {
        let toml: CargoToml = toml::from_str(content)?;

        let (name, version) = if let Some(pkg) = toml.package {
            (pkg.name, pkg.version)
        } else {
            (None, None)
        };

        let (is_workspace, workspace_members) = if let Some(ws) = toml.workspace {
            (true, ws.members.unwrap_or_default())
        } else {
            (false, Vec::new())
        };

        Ok(CargoManifest {
            name,
            version,
            is_workspace,
            workspace_members,
        })
    }
}

impl PyProjectManifest {
    /// Parse a pyproject.toml file
    pub fn parse(path: &Path) -> Result<Self, ManifestError> {
        let content = std::fs::read_to_string(path)?;
        Self::parse_str(&content)
    }

    /// Parse pyproject.toml content from a string
    pub fn parse_str(content: &str) -> Result<Self, ManifestError> {
        let toml: PyProjectToml = toml::from_str(content)?;

        let (name, version) = if let Some(proj) = toml.project {
            (proj.name, proj.version)
        } else {
            (None, None)
        };

        // Try to find package directory from various tool configurations
        let package_dir = if let Some(tool) = toml.tool {
            // First check maturin python-source
            if let Some(maturin) = tool.maturin {
                if let Some(src) = maturin.python_source {
                    Some(PathBuf::from(src))
                } else {
                    None
                }
            // Then check setuptools package-dir
            } else if let Some(setuptools) = tool.setuptools {
                if let Some(pkg_dir) = setuptools.package_dir {
                    // Get the root package directory (empty string key or "")
                    pkg_dir.get("").map(PathBuf::from)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        Ok(PyProjectManifest {
            name,
            version,
            package_dir,
        })
    }
}

/// Inferred configuration values from manifest files
#[derive(Debug, Default)]
pub struct InferredConfig {
    /// Project name (from Cargo.toml or pyproject.toml)
    pub project_name: Option<String>,
    /// Rust crate paths (from workspace members or current directory)
    pub rust_crates: Option<Vec<PathBuf>>,
    /// Rust entry point (package name from Cargo.toml)
    pub rust_entry_point: Option<String>,
    /// Python package name (from pyproject.toml)
    pub python_package: Option<String>,
    /// Python source directory (from tool.maturin.python-source)
    pub python_source: Option<PathBuf>,
}

impl InferredConfig {
    /// Infer configuration from manifest files in the given directory
    pub fn from_directory(project_root: &Path) -> Self {
        let mut inferred = InferredConfig::default();

        // Try to parse Cargo.toml
        let cargo_path = project_root.join("Cargo.toml");
        if cargo_path.exists() {
            if let Ok(cargo) = CargoManifest::parse(&cargo_path) {
                // Project name from package name
                if let Some(name) = &cargo.name {
                    inferred.project_name = Some(name.clone());
                    inferred.rust_entry_point = Some(name.clone());
                }

                // Rust crates from workspace members or current directory
                if cargo.is_workspace && !cargo.workspace_members.is_empty() {
                    inferred.rust_crates =
                        Some(cargo.workspace_members.iter().map(PathBuf::from).collect());
                } else if cargo.name.is_some() {
                    // Single crate project
                    inferred.rust_crates = Some(vec![PathBuf::from(".")]);
                }
            }
        }

        // Try to parse pyproject.toml
        let pyproject_path = project_root.join("pyproject.toml");
        if pyproject_path.exists() {
            if let Ok(pyproject) = PyProjectManifest::parse(&pyproject_path) {
                // Project name from pyproject takes precedence
                if let Some(name) = &pyproject.name {
                    inferred.project_name = Some(name.clone());
                    // Python package name (convert dashes to underscores)
                    inferred.python_package = Some(name.replace('-', "_"));
                }

                // Python source directory
                if let Some(pkg_dir) = pyproject.package_dir {
                    inferred.python_source = Some(pkg_dir);
                }
            }
        }

        inferred
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cargo_toml_simple() {
        let content = r#"
[package]
name = "my-crate"
version = "1.0.0"
"#;
        let manifest = CargoManifest::parse_str(content).unwrap();
        assert_eq!(manifest.name, Some("my-crate".to_string()));
        assert_eq!(manifest.version, Some("1.0.0".to_string()));
        assert!(!manifest.is_workspace);
        assert!(manifest.workspace_members.is_empty());
    }

    #[test]
    fn test_parse_cargo_toml_workspace() {
        let content = r#"
[workspace]
members = ["crates/core", "crates/cli"]

[package]
name = "my-workspace"
version = "0.1.0"
"#;
        let manifest = CargoManifest::parse_str(content).unwrap();
        assert_eq!(manifest.name, Some("my-workspace".to_string()));
        assert!(manifest.is_workspace);
        assert_eq!(
            manifest.workspace_members,
            vec!["crates/core", "crates/cli"]
        );
    }

    #[test]
    fn test_parse_cargo_toml_workspace_only() {
        let content = r#"
[workspace]
members = ["crates/core", "crates/cli"]
"#;
        let manifest = CargoManifest::parse_str(content).unwrap();
        assert_eq!(manifest.name, None);
        assert!(manifest.is_workspace);
        assert_eq!(
            manifest.workspace_members,
            vec!["crates/core", "crates/cli"]
        );
    }

    #[test]
    fn test_parse_pyproject_toml_simple() {
        let content = r#"
[project]
name = "my-package"
version = "1.0.0"
"#;
        let manifest = PyProjectManifest::parse_str(content).unwrap();
        assert_eq!(manifest.name, Some("my-package".to_string()));
        assert_eq!(manifest.version, Some("1.0.0".to_string()));
        assert_eq!(manifest.package_dir, None);
    }

    #[test]
    fn test_parse_pyproject_toml_with_maturin() {
        let content = r#"
[project]
name = "my-package"
version = "1.0.0"

[tool.maturin]
python-source = "python"
module-name = "my_package._core"
"#;
        let manifest = PyProjectManifest::parse_str(content).unwrap();
        assert_eq!(manifest.name, Some("my-package".to_string()));
        assert_eq!(manifest.package_dir, Some(PathBuf::from("python")));
    }

    #[test]
    fn test_parse_pyproject_toml_with_setuptools() {
        let content = r#"
[project]
name = "my-package"
version = "1.0.0"

[tool.setuptools.package-dir]
"" = "src"
"#;
        let manifest = PyProjectManifest::parse_str(content).unwrap();
        assert_eq!(manifest.name, Some("my-package".to_string()));
        assert_eq!(manifest.package_dir, Some(PathBuf::from("src")));
    }

    #[test]
    fn test_inferred_config_from_cargo_only() {
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        std::fs::write(
            temp_dir.path().join("Cargo.toml"),
            r#"
[package]
name = "my-rust-crate"
version = "0.1.0"
"#,
        )
        .unwrap();

        let inferred = InferredConfig::from_directory(temp_dir.path());
        assert_eq!(inferred.project_name, Some("my-rust-crate".to_string()));
        assert_eq!(inferred.rust_entry_point, Some("my-rust-crate".to_string()));
        assert_eq!(inferred.rust_crates, Some(vec![PathBuf::from(".")]));
        assert_eq!(inferred.python_package, None);
    }

    #[test]
    fn test_inferred_config_from_pyproject_only() {
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        std::fs::write(
            temp_dir.path().join("pyproject.toml"),
            r#"
[project]
name = "my-python-pkg"
version = "1.0.0"

[tool.maturin]
python-source = "python"
"#,
        )
        .unwrap();

        let inferred = InferredConfig::from_directory(temp_dir.path());
        assert_eq!(inferred.project_name, Some("my-python-pkg".to_string()));
        assert_eq!(inferred.python_package, Some("my_python_pkg".to_string()));
        assert_eq!(inferred.python_source, Some(PathBuf::from("python")));
        assert_eq!(inferred.rust_crates, None);
    }

    #[test]
    fn test_inferred_config_hybrid_project() {
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();

        // Create Cargo.toml with workspace
        std::fs::write(
            temp_dir.path().join("Cargo.toml"),
            r#"
[workspace]
members = ["crates/core", "crates/bindings"]

[package]
name = "my-hybrid"
version = "0.1.0"
"#,
        )
        .unwrap();

        // Create pyproject.toml
        std::fs::write(
            temp_dir.path().join("pyproject.toml"),
            r#"
[project]
name = "my-hybrid"
version = "0.1.0"

[tool.maturin]
python-source = "python"
"#,
        )
        .unwrap();

        let inferred = InferredConfig::from_directory(temp_dir.path());
        // pyproject name takes precedence
        assert_eq!(inferred.project_name, Some("my-hybrid".to_string()));
        // But rust info still comes from Cargo.toml
        assert_eq!(
            inferred.rust_crates,
            Some(vec![
                PathBuf::from("crates/core"),
                PathBuf::from("crates/bindings")
            ])
        );
        assert_eq!(inferred.rust_entry_point, Some("my-hybrid".to_string()));
        assert_eq!(inferred.python_package, Some("my_hybrid".to_string()));
        assert_eq!(inferred.python_source, Some(PathBuf::from("python")));
    }
}
