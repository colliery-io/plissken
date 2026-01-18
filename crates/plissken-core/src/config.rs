//! Configuration for plissken projects

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use thiserror::Error;

// =============================================================================
// Constants
// =============================================================================

/// Default output format for documentation
pub const DEFAULT_OUTPUT_FORMAT: &str = "markdown";

/// Default output path for generated documentation
pub const DEFAULT_OUTPUT_PATH: &str = "docs/api";

/// Default docs.rs base URL for external links
pub const DEFAULT_DOCS_RS_URL: &str = "https://docs.rs";

/// Default SSG template name
pub const DEFAULT_TEMPLATE: &str = "mkdocs-material";

/// Version source identifier for Cargo.toml
pub const VERSION_SOURCE_CARGO: &str = "cargo";

/// Version source identifier for pyproject.toml
pub const VERSION_SOURCE_PYPROJECT: &str = "pyproject";

/// Cargo manifest filename
pub const CARGO_MANIFEST: &str = "Cargo.toml";

/// Python project manifest filename
pub const PYPROJECT_MANIFEST: &str = "pyproject.toml";

/// Plissken configuration filename
pub const PLISSKEN_CONFIG: &str = "plissken.toml";

/// MkDocs Material template name
pub const TEMPLATE_MKDOCS_MATERIAL: &str = "mkdocs-material";

/// mdBook template name
pub const TEMPLATE_MDBOOK: &str = "mdbook";

/// Default crates configuration value
pub const DEFAULT_CRATES: &str = ".";

/// Configuration validation error
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("no language configured: add [rust] or [python] section")]
    NoLanguageConfigured,

    #[error("version_from is '{0}' but {1} not found")]
    VersionSourceNotFound(String, String),

    #[error("rust crate path '{0}' does not exist")]
    RustCrateNotFound(PathBuf),

    #[error("python source path '{0}' does not exist")]
    PythonSourceNotFound(PathBuf),

    #[error("git repository not found (version_from = 'git')")]
    GitRepoNotFound,
}

/// Configuration warning (non-fatal issue)
#[derive(Debug, Clone, Serialize)]
pub struct ConfigWarning {
    /// The config field that triggered the warning
    pub field: String,
    /// Human-readable warning message
    pub message: String,
    /// Optional hint for resolution
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint: Option<String>,
}

impl ConfigWarning {
    /// Create a new warning
    pub fn new(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            message: message.into(),
            hint: None,
        }
    }

    /// Add a hint to the warning
    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }
}

/// Result of configuration validation
#[derive(Debug)]
pub struct ValidationResult {
    /// Whether validation passed (no errors)
    pub valid: bool,
    /// Validation warnings (non-fatal)
    pub warnings: Vec<ConfigWarning>,
}

/// Root configuration from plissken.toml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub project: ProjectConfig,
    pub output: OutputConfig,
    #[serde(default)]
    pub rust: Option<RustConfig>,
    #[serde(default)]
    pub python: Option<PythonConfig>,
    #[serde(default)]
    pub links: LinksConfig,
    #[serde(default)]
    pub quality: QualityConfig,
}

/// Project metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    #[serde(default = "default_version_from")]
    pub version_from: VersionSource,
}

fn default_version_from() -> VersionSource {
    VersionSource::Git
}

/// Where to get version information
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum VersionSource {
    #[default]
    Git,
    Cargo,
    Pyproject,
}

/// Output configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    #[serde(default = "default_format")]
    pub format: String,
    #[serde(default = "default_output_path")]
    pub path: PathBuf,
    #[serde(default)]
    pub template: Option<String>,
}

fn default_format() -> String {
    DEFAULT_OUTPUT_FORMAT.to_string()
}

fn default_output_path() -> PathBuf {
    PathBuf::from(DEFAULT_OUTPUT_PATH)
}

/// Rust source configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustConfig {
    pub crates: Vec<PathBuf>,
    #[serde(default)]
    pub entry_point: Option<String>,
}

/// Python source configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PythonConfig {
    /// The Python package name
    pub package: String,
    /// Source directory containing Python files (defaults to package name)
    #[serde(default)]
    pub source: Option<PathBuf>,
    /// Automatically discover Python modules by walking the filesystem
    #[serde(default)]
    pub auto_discover: bool,
    /// Explicit module mappings (overrides auto-discovered modules)
    #[serde(default)]
    pub modules: HashMap<String, ModuleSourceType>,
}

/// Source type for a Python module
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ModuleSourceType {
    Pyo3,
    Python,
}

/// Linking configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinksConfig {
    #[serde(default = "default_dependencies")]
    pub dependencies: DependencySource,
    #[serde(default = "default_docs_rs")]
    pub docs_rs_base: String,
}

fn default_dependencies() -> DependencySource {
    DependencySource::CargoLock
}

fn default_docs_rs() -> String {
    DEFAULT_DOCS_RS_URL.to_string()
}

/// Where to get dependency versions
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum DependencySource {
    #[default]
    CargoLock,
    CargoToml,
    None,
}

/// Quality/linting configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QualityConfig {
    #[serde(default)]
    pub require_docstrings: bool,
    #[serde(default)]
    pub min_coverage: Option<f64>,
    #[serde(default)]
    pub fail_on_broken_links: bool,
}

impl Config {
    /// Load configuration from a plissken.toml file.
    ///
    /// # Errors
    ///
    /// Returns `PlisskenError::ConfigNotFound` if the file doesn't exist,
    /// `PlisskenError::ConfigParse` if the TOML is invalid.
    pub fn from_file(path: &std::path::Path) -> crate::error::Result<Self> {
        use crate::error::PlisskenError;

        let content = std::fs::read_to_string(path).map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                PlisskenError::ConfigNotFound {
                    path: path.to_path_buf(),
                }
            } else {
                PlisskenError::Io {
                    context: format!("failed to read config file '{}'", path.display()),
                    source: e,
                }
            }
        })?;

        let config: Config = toml::from_str(&content).map_err(|e| PlisskenError::ConfigParse {
            message: e.to_string(),
            source: Some(e),
        })?;

        Ok(config)
    }

    /// Apply inferred defaults from manifest files (Cargo.toml, pyproject.toml).
    ///
    /// Infers project metadata from existing manifest files and fills in missing
    /// configuration values. Explicit configuration always takes precedence over
    /// inferred values.
    ///
    /// # Arguments
    /// * `project_root` - The directory containing manifest files
    ///
    /// # Inferred values
    /// - `project.name` - From pyproject.toml [project].name or Cargo.toml [package].name
    /// - `rust.crates` - From Cargo.toml [workspace].members or single crate root
    /// - `rust.entry_point` - From Cargo.toml [package].name
    /// - `python.package` - From pyproject.toml [project].name (with dash-to-underscore)
    /// - `python.source` - From pyproject.toml [tool.maturin].python-source
    pub fn with_inferred_defaults(mut self, project_root: &Path) -> Self {
        use crate::manifest::InferredConfig;

        let inferred = InferredConfig::from_directory(project_root);

        // Fill in project name if empty
        if self.project.name.is_empty()
            && let Some(name) = inferred.project_name
        {
            self.project.name = name;
        }

        // Fill in Rust config if present but incomplete
        if let Some(ref mut rust) = self.rust {
            // Fill in crates if empty
            if rust.crates.is_empty()
                && let Some(crates) = inferred.rust_crates
            {
                rust.crates = crates;
            }
            // Fill in entry_point if not set
            if rust.entry_point.is_none() {
                rust.entry_point = inferred.rust_entry_point;
            }
        }

        // Fill in Python config if present but incomplete
        if let Some(ref mut python) = self.python {
            // Fill in package name if empty
            if python.package.is_empty()
                && let Some(pkg) = inferred.python_package
            {
                python.package = pkg;
            }
            // Fill in source if not set
            if python.source.is_none() {
                python.source = inferred.python_source;
            }
        }

        self
    }

    /// Validate configuration semantically.
    ///
    /// Performs validation beyond TOML parsing:
    /// - At least one language section must be configured
    /// - version_from source file must exist
    /// - Configured paths must exist
    ///
    /// Returns `Ok(ValidationResult)` with any warnings if validation passes,
    /// or `Err(ConfigError)` if validation fails.
    ///
    /// # Arguments
    /// * `project_root` - The directory containing the plissken.toml file
    pub fn validate(&self, project_root: &Path) -> Result<ValidationResult, ConfigError> {
        let mut warnings = Vec::new();

        // Must have at least one language configured
        if self.rust.is_none() && self.python.is_none() {
            return Err(ConfigError::NoLanguageConfigured);
        }

        // Validate version_from source exists
        self.validate_version_source(project_root)?;

        // Validate Rust configuration
        if let Some(ref rust_config) = self.rust {
            self.validate_rust_config(rust_config, project_root, &mut warnings)?;
        }

        // Validate Python configuration
        if let Some(ref python_config) = self.python {
            self.validate_python_config(python_config, project_root, &mut warnings)?;
        }

        Ok(ValidationResult {
            valid: true,
            warnings,
        })
    }

    fn validate_version_source(&self, project_root: &Path) -> Result<(), ConfigError> {
        match self.project.version_from {
            VersionSource::Cargo => {
                let cargo_toml = project_root.join(CARGO_MANIFEST);
                if !cargo_toml.exists() {
                    return Err(ConfigError::VersionSourceNotFound(
                        VERSION_SOURCE_CARGO.to_string(),
                        CARGO_MANIFEST.to_string(),
                    ));
                }
            }
            VersionSource::Pyproject => {
                let pyproject = project_root.join(PYPROJECT_MANIFEST);
                if !pyproject.exists() {
                    return Err(ConfigError::VersionSourceNotFound(
                        VERSION_SOURCE_PYPROJECT.to_string(),
                        PYPROJECT_MANIFEST.to_string(),
                    ));
                }
            }
            VersionSource::Git => {
                // Check if we're in a git repository
                let git_check = std::process::Command::new("git")
                    .args(["rev-parse", "--git-dir"])
                    .current_dir(project_root)
                    .output();

                match git_check {
                    Ok(output) if output.status.success() => {}
                    _ => return Err(ConfigError::GitRepoNotFound),
                }
            }
        }
        Ok(())
    }

    fn validate_rust_config(
        &self,
        rust_config: &RustConfig,
        project_root: &Path,
        warnings: &mut Vec<ConfigWarning>,
    ) -> Result<(), ConfigError> {
        if rust_config.crates.is_empty() {
            warnings.push(
                ConfigWarning::new(
                    "rust.crates",
                    "no crates configured; no Rust docs will be generated",
                )
                .with_hint("add crate paths to the crates array"),
            );
            return Ok(());
        }

        for crate_path in &rust_config.crates {
            let crate_dir = project_root.join(crate_path);

            if !crate_dir.exists() {
                return Err(ConfigError::RustCrateNotFound(crate_path.clone()));
            }

            // Check for Cargo.toml in crate directory (warning, not error)
            let cargo_toml = crate_dir.join(CARGO_MANIFEST);
            if !cargo_toml.exists() && crate_path.as_os_str() != DEFAULT_CRATES {
                warnings.push(ConfigWarning::new(
                    "rust.crates",
                    format!("no Cargo.toml found in crate '{}'", crate_path.display()),
                ));
            }

            // Check for src directory (warning, not error)
            let src_dir = crate_dir.join("src");
            if !src_dir.exists() {
                warnings.push(
                    ConfigWarning::new(
                        "rust.crates",
                        format!("no src/ directory in crate '{}'", crate_path.display()),
                    )
                    .with_hint("Rust source files are typically in a src/ directory"),
                );
            }
        }

        Ok(())
    }

    fn validate_python_config(
        &self,
        python_config: &PythonConfig,
        project_root: &Path,
        warnings: &mut Vec<ConfigWarning>,
    ) -> Result<(), ConfigError> {
        // Determine Python source directory
        let python_dir = if let Some(ref source) = python_config.source {
            project_root.join(source)
        } else {
            project_root.join(&python_config.package)
        };

        if !python_dir.exists() {
            return Err(ConfigError::PythonSourceNotFound(python_dir));
        }

        // Check for __init__.py (warning, not error)
        let init_py = python_dir.join("__init__.py");
        if !init_py.exists() {
            warnings.push(
                ConfigWarning::new(
                    "python.package",
                    format!(
                        "no __init__.py in '{}' - may not be a proper Python package",
                        python_dir.display()
                    ),
                )
                .with_hint("add __init__.py to make it a Python package"),
            );
        }

        // Check for empty modules list (warning)
        if python_config.modules.is_empty() {
            warnings.push(
                ConfigWarning::new(
                    "python.modules",
                    "no modules listed; consider using auto_discover or listing modules explicitly",
                )
                .with_hint("modules will be discovered from filesystem if not listed"),
            );
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use tempfile::TempDir;

    fn minimal_config() -> Config {
        Config {
            project: ProjectConfig {
                name: "test".to_string(),
                version_from: VersionSource::Git,
            },
            output: OutputConfig {
                format: "markdown".to_string(),
                path: PathBuf::from("docs/api"),
                template: None,
            },
            rust: None,
            python: None,
            links: LinksConfig::default(),
            quality: QualityConfig::default(),
        }
    }

    #[test]
    fn test_validate_no_language_configured() {
        let config = minimal_config();
        let temp_dir = TempDir::new().unwrap();

        let result = config.validate(temp_dir.path());
        assert!(matches!(result, Err(ConfigError::NoLanguageConfigured)));
    }

    #[test]
    fn test_validate_rust_crate_not_found() {
        let mut config = minimal_config();
        config.rust = Some(RustConfig {
            crates: vec![PathBuf::from("nonexistent")],
            entry_point: None,
        });

        let temp_dir = TempDir::new().unwrap();
        // Create a git repo so version_from works
        std::process::Command::new("git")
            .args(["init"])
            .current_dir(temp_dir.path())
            .output()
            .unwrap();

        let result = config.validate(temp_dir.path());
        assert!(matches!(result, Err(ConfigError::RustCrateNotFound(_))));
    }

    #[test]
    fn test_validate_python_source_not_found() {
        let mut config = minimal_config();
        config.python = Some(PythonConfig {
            package: "nonexistent".to_string(),
            source: None,
            auto_discover: false,
            modules: HashMap::new(),
        });

        let temp_dir = TempDir::new().unwrap();
        // Create a git repo so version_from works
        std::process::Command::new("git")
            .args(["init"])
            .current_dir(temp_dir.path())
            .output()
            .unwrap();

        let result = config.validate(temp_dir.path());
        assert!(matches!(result, Err(ConfigError::PythonSourceNotFound(_))));
    }

    #[test]
    fn test_validate_version_source_cargo_not_found() {
        let mut config = minimal_config();
        config.project.version_from = VersionSource::Cargo;
        config.rust = Some(RustConfig {
            crates: vec![PathBuf::from(".")],
            entry_point: None,
        });

        let temp_dir = TempDir::new().unwrap();
        // Create src directory but no Cargo.toml
        std::fs::create_dir(temp_dir.path().join("src")).unwrap();

        let result = config.validate(temp_dir.path());
        assert!(matches!(
            result,
            Err(ConfigError::VersionSourceNotFound(_, _))
        ));
    }

    #[test]
    fn test_validate_version_source_pyproject_not_found() {
        let mut config = minimal_config();
        config.project.version_from = VersionSource::Pyproject;
        config.python = Some(PythonConfig {
            package: "mypackage".to_string(),
            source: None,
            auto_discover: false,
            modules: HashMap::new(),
        });

        let temp_dir = TempDir::new().unwrap();
        // Create package directory but no pyproject.toml
        std::fs::create_dir(temp_dir.path().join("mypackage")).unwrap();

        let result = config.validate(temp_dir.path());
        assert!(matches!(
            result,
            Err(ConfigError::VersionSourceNotFound(_, _))
        ));
    }

    #[test]
    fn test_validate_valid_rust_config() {
        let mut config = minimal_config();
        config.project.version_from = VersionSource::Cargo;
        config.rust = Some(RustConfig {
            crates: vec![PathBuf::from(".")],
            entry_point: None,
        });

        let temp_dir = TempDir::new().unwrap();
        // Create Cargo.toml and src directory
        std::fs::write(
            temp_dir.path().join("Cargo.toml"),
            "[package]\nname = \"test\"\nversion = \"0.1.0\"\n",
        )
        .unwrap();
        std::fs::create_dir(temp_dir.path().join("src")).unwrap();

        let result = config.validate(temp_dir.path());
        assert!(result.is_ok());
        let validation = result.unwrap();
        assert!(validation.valid);
    }

    #[test]
    fn test_validate_valid_python_config() {
        let mut config = minimal_config();
        config.project.version_from = VersionSource::Pyproject;
        config.python = Some(PythonConfig {
            package: "mypackage".to_string(),
            source: None,
            auto_discover: false,
            modules: HashMap::new(),
        });

        let temp_dir = TempDir::new().unwrap();
        // Create pyproject.toml and package directory with __init__.py
        std::fs::write(
            temp_dir.path().join("pyproject.toml"),
            "[project]\nname = \"mypackage\"\nversion = \"1.0.0\"\n",
        )
        .unwrap();
        let pkg_dir = temp_dir.path().join("mypackage");
        std::fs::create_dir(&pkg_dir).unwrap();
        std::fs::write(pkg_dir.join("__init__.py"), "").unwrap();

        let result = config.validate(temp_dir.path());
        assert!(result.is_ok());
        let validation = result.unwrap();
        assert!(validation.valid);
        // Should have warning about empty modules
        assert!(!validation.warnings.is_empty());
    }

    #[test]
    fn test_validate_warnings_for_missing_init_py() {
        let mut config = minimal_config();
        config.project.version_from = VersionSource::Pyproject;
        config.python = Some(PythonConfig {
            package: "mypackage".to_string(),
            source: None,
            auto_discover: false,
            modules: HashMap::new(),
        });

        let temp_dir = TempDir::new().unwrap();
        std::fs::write(
            temp_dir.path().join("pyproject.toml"),
            "[project]\nname = \"mypackage\"\nversion = \"1.0.0\"\n",
        )
        .unwrap();
        // Create package directory WITHOUT __init__.py
        std::fs::create_dir(temp_dir.path().join("mypackage")).unwrap();

        let result = config.validate(temp_dir.path());
        assert!(result.is_ok());
        let validation = result.unwrap();
        // Should have warning about missing __init__.py
        assert!(
            validation
                .warnings
                .iter()
                .any(|w| w.message.contains("__init__.py"))
        );
    }

    #[test]
    fn test_validate_warnings_for_missing_src_dir() {
        let mut config = minimal_config();
        config.project.version_from = VersionSource::Cargo;
        config.rust = Some(RustConfig {
            crates: vec![PathBuf::from(".")],
            entry_point: None,
        });

        let temp_dir = TempDir::new().unwrap();
        // Create Cargo.toml but no src directory
        std::fs::write(
            temp_dir.path().join("Cargo.toml"),
            "[package]\nname = \"test\"\nversion = \"0.1.0\"\n",
        )
        .unwrap();

        let result = config.validate(temp_dir.path());
        assert!(result.is_ok());
        let validation = result.unwrap();
        // Should have warning about missing src/
        assert!(
            validation
                .warnings
                .iter()
                .any(|w| w.message.contains("src/"))
        );
    }

    #[test]
    fn test_with_inferred_defaults_fills_project_name() {
        let mut config = minimal_config();
        config.project.name = "".to_string(); // Empty name to be filled

        let temp_dir = TempDir::new().unwrap();
        std::fs::write(
            temp_dir.path().join("Cargo.toml"),
            "[package]\nname = \"inferred-name\"\nversion = \"0.1.0\"\n",
        )
        .unwrap();

        let config = config.with_inferred_defaults(temp_dir.path());
        assert_eq!(config.project.name, "inferred-name");
    }

    #[test]
    fn test_with_inferred_defaults_preserves_explicit_name() {
        let mut config = minimal_config();
        config.project.name = "explicit-name".to_string();

        let temp_dir = TempDir::new().unwrap();
        std::fs::write(
            temp_dir.path().join("Cargo.toml"),
            "[package]\nname = \"inferred-name\"\nversion = \"0.1.0\"\n",
        )
        .unwrap();

        let config = config.with_inferred_defaults(temp_dir.path());
        // Explicit name should be preserved
        assert_eq!(config.project.name, "explicit-name");
    }

    #[test]
    fn test_with_inferred_defaults_fills_rust_config() {
        let mut config = minimal_config();
        config.rust = Some(RustConfig {
            crates: vec![], // Empty crates to be filled
            entry_point: None,
        });

        let temp_dir = TempDir::new().unwrap();
        std::fs::write(
            temp_dir.path().join("Cargo.toml"),
            r#"
[workspace]
members = ["crates/core", "crates/cli"]

[package]
name = "my-project"
version = "0.1.0"
"#,
        )
        .unwrap();

        let config = config.with_inferred_defaults(temp_dir.path());
        assert_eq!(
            config.rust.as_ref().unwrap().crates,
            vec![PathBuf::from("crates/core"), PathBuf::from("crates/cli")]
        );
        assert_eq!(
            config.rust.as_ref().unwrap().entry_point,
            Some("my-project".to_string())
        );
    }

    #[test]
    fn test_with_inferred_defaults_fills_python_config() {
        let mut config = minimal_config();
        config.python = Some(PythonConfig {
            package: "".to_string(), // Empty package to be filled
            source: None,
            auto_discover: false,
            modules: HashMap::new(),
        });

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

        let config = config.with_inferred_defaults(temp_dir.path());
        assert_eq!(config.python.as_ref().unwrap().package, "my_python_pkg");
        assert_eq!(
            config.python.as_ref().unwrap().source,
            Some(PathBuf::from("python"))
        );
    }

    #[test]
    fn test_with_inferred_defaults_preserves_explicit_python_config() {
        let mut config = minimal_config();
        config.python = Some(PythonConfig {
            package: "explicit_pkg".to_string(),
            source: Some(PathBuf::from("src")),
            auto_discover: false,
            modules: HashMap::new(),
        });

        let temp_dir = TempDir::new().unwrap();
        std::fs::write(
            temp_dir.path().join("pyproject.toml"),
            r#"
[project]
name = "inferred-pkg"
version = "1.0.0"

[tool.maturin]
python-source = "python"
"#,
        )
        .unwrap();

        let config = config.with_inferred_defaults(temp_dir.path());
        // Explicit values should be preserved
        assert_eq!(config.python.as_ref().unwrap().package, "explicit_pkg");
        assert_eq!(
            config.python.as_ref().unwrap().source,
            Some(PathBuf::from("src"))
        );
    }

    #[test]
    fn test_with_inferred_defaults_pyproject_takes_precedence_for_name() {
        let mut config = minimal_config();
        config.project.name = "".to_string();

        let temp_dir = TempDir::new().unwrap();
        std::fs::write(
            temp_dir.path().join("Cargo.toml"),
            "[package]\nname = \"cargo-name\"\nversion = \"0.1.0\"\n",
        )
        .unwrap();
        std::fs::write(
            temp_dir.path().join("pyproject.toml"),
            "[project]\nname = \"pyproject-name\"\nversion = \"1.0.0\"\n",
        )
        .unwrap();

        let config = config.with_inferred_defaults(temp_dir.path());
        // pyproject.toml name should take precedence
        assert_eq!(config.project.name, "pyproject-name");
    }
}
