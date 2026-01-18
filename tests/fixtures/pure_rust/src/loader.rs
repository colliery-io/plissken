//! Configuration loaders.

use crate::config::{Config, Value};
use crate::error::LoadError;
use std::path::Path;

/// Trait for loading configuration from various sources.
pub trait Loader {
    /// Load configuration from this source.
    fn load(&self) -> Result<Config, LoadError>;

    /// Get a description of this loader's source.
    fn source_description(&self) -> String;
}

/// Loads configuration from a file.
pub struct FileLoader {
    path: std::path::PathBuf,
    format: FileFormat,
}

/// Supported file formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileFormat {
    /// JSON format.
    Json,
    /// TOML format.
    Toml,
    /// YAML format.
    Yaml,
}

impl FileLoader {
    /// Create a new file loader.
    ///
    /// The format is auto-detected from the file extension.
    pub fn new(path: impl AsRef<Path>) -> Result<Self, LoadError> {
        let path = path.as_ref();
        let format = Self::detect_format(path)?;
        Ok(Self {
            path: path.to_path_buf(),
            format,
        })
    }

    /// Create a file loader with explicit format.
    pub fn with_format(path: impl AsRef<Path>, format: FileFormat) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            format,
        }
    }

    fn detect_format(path: &Path) -> Result<FileFormat, LoadError> {
        match path.extension().and_then(|e| e.to_str()) {
            Some("json") => Ok(FileFormat::Json),
            Some("toml") => Ok(FileFormat::Toml),
            Some("yaml") | Some("yml") => Ok(FileFormat::Yaml),
            _ => Err(LoadError::UnknownFormat(path.display().to_string())),
        }
    }
}

impl Loader for FileLoader {
    fn load(&self) -> Result<Config, LoadError> {
        let _content = std::fs::read_to_string(&self.path)
            .map_err(|e| LoadError::Io(e.to_string()))?;

        // Simplified: would parse based on format
        let _ = self.format;
        Ok(Config::new())
    }

    fn source_description(&self) -> String {
        format!("file:{}", self.path.display())
    }
}

/// Loads configuration from environment variables.
pub struct EnvLoader {
    prefix: Option<String>,
    separator: String,
}

impl EnvLoader {
    /// Create a new environment loader.
    pub fn new() -> Self {
        Self {
            prefix: None,
            separator: "_".to_string(),
        }
    }

    /// Only load variables with this prefix.
    ///
    /// The prefix is stripped from the key names.
    pub fn with_prefix(mut self, prefix: &str) -> Self {
        self.prefix = Some(prefix.to_string());
        self
    }

    /// Use a custom separator for nested keys.
    ///
    /// Default is underscore. E.g., `DATABASE_HOST` becomes `database.host`.
    pub fn with_separator(mut self, sep: &str) -> Self {
        self.separator = sep.to_string();
        self
    }
}

impl Default for EnvLoader {
    fn default() -> Self {
        Self::new()
    }
}

impl Loader for EnvLoader {
    fn load(&self) -> Result<Config, LoadError> {
        let mut config = Config::new();

        for (key, value) in std::env::vars() {
            let key = match &self.prefix {
                Some(prefix) => {
                    if key.starts_with(prefix) {
                        key.strip_prefix(prefix)
                            .unwrap()
                            .trim_start_matches('_')
                            .to_string()
                    } else {
                        continue;
                    }
                }
                None => key,
            };

            let key = key.to_lowercase().replace(&self.separator, ".");
            config.set(key, Value::String(value));
        }

        Ok(config)
    }

    fn source_description(&self) -> String {
        match &self.prefix {
            Some(p) => format!("env:{}_*", p),
            None => "env:*".to_string(),
        }
    }
}
