//! Unified error types for plissken-core.
//!
//! This module provides a single error enum that covers all error cases
//! in the library, replacing the previous mix of `anyhow::Error`,
//! `tera::Error`, and other error types.

use std::path::PathBuf;
use thiserror::Error;

/// The primary error type for plissken-core operations.
///
/// This enum covers all error categories that can occur during parsing,
/// rendering, and configuration handling.
#[derive(Debug, Error)]
pub enum PlisskenError {
    // =========================================================================
    // Configuration Errors
    // =========================================================================
    /// Configuration file not found at the expected path.
    #[error("config not found: {path}")]
    ConfigNotFound {
        /// Path where config was expected
        path: PathBuf,
    },

    /// Failed to parse configuration file.
    #[error("config parse error: {message}")]
    ConfigParse {
        /// Description of the parse error
        message: String,
        /// The underlying TOML parse error, if available
        #[source]
        source: Option<toml::de::Error>,
    },

    /// Configuration validation failed.
    #[error("config validation failed: {message}")]
    ConfigValidation {
        /// Description of the validation error
        message: String,
    },

    // =========================================================================
    // Parse Errors
    // =========================================================================
    /// Failed to parse a source file.
    #[error("failed to parse {language} file '{path}': {message}")]
    Parse {
        /// The language being parsed (e.g., "Rust", "Python")
        language: String,
        /// Path to the file that failed to parse
        path: PathBuf,
        /// Line number where error occurred, if known
        line: Option<usize>,
        /// Description of the parse error
        message: String,
    },

    /// Failed to read a source file.
    #[error("failed to read file '{path}': {message}")]
    FileRead {
        /// Path to the file that couldn't be read
        path: PathBuf,
        /// Description of the error
        message: String,
        /// The underlying IO error
        #[source]
        source: std::io::Error,
    },

    // =========================================================================
    // Render Errors
    // =========================================================================
    /// Template rendering failed.
    #[error("template error: {message}")]
    Template {
        /// Description of the template error
        message: String,
        /// The underlying Tera error
        #[source]
        source: tera::Error,
    },

    /// Failed to write output file.
    #[error("failed to write output '{path}': {message}")]
    OutputWrite {
        /// Path to the output file
        path: PathBuf,
        /// Description of the error
        message: String,
        /// The underlying IO error
        #[source]
        source: std::io::Error,
    },

    // =========================================================================
    // Cross-Reference Errors
    // =========================================================================
    /// Cross-reference resolution failed.
    #[error("cross-reference error: {message}")]
    CrossRef {
        /// Description of the cross-reference error
        message: String,
    },

    // =========================================================================
    // IO Errors
    // =========================================================================
    /// Generic IO error with context.
    #[error("{context}: {source}")]
    Io {
        /// Context describing what operation failed
        context: String,
        /// The underlying IO error
        #[source]
        source: std::io::Error,
    },

    // =========================================================================
    // Discovery Errors
    // =========================================================================
    /// Module discovery failed.
    #[error("module discovery failed in '{path}': {message}")]
    Discovery {
        /// Directory being scanned
        path: PathBuf,
        /// Description of the error
        message: String,
    },

    // =========================================================================
    // Manifest Errors
    // =========================================================================
    /// Failed to parse manifest file (Cargo.toml or pyproject.toml).
    #[error("failed to parse manifest '{path}': {message}")]
    ManifestParse {
        /// Path to the manifest file
        path: PathBuf,
        /// Description of the error
        message: String,
    },
}


/// A specialized Result type for plissken operations.
pub type Result<T> = std::result::Result<T, PlisskenError>;

// =============================================================================
// From implementations for automatic error conversion
// =============================================================================

impl From<std::io::Error> for PlisskenError {
    fn from(err: std::io::Error) -> Self {
        PlisskenError::Io {
            context: "IO operation failed".into(),
            source: err,
        }
    }
}

impl From<tera::Error> for PlisskenError {
    fn from(err: tera::Error) -> Self {
        PlisskenError::Template {
            message: err.to_string(),
            source: err,
        }
    }
}

impl From<toml::de::Error> for PlisskenError {
    fn from(err: toml::de::Error) -> Self {
        PlisskenError::ConfigParse {
            message: err.to_string(),
            source: Some(err),
        }
    }
}

// Convert from our existing ConfigError
impl From<crate::config::ConfigError> for PlisskenError {
    fn from(err: crate::config::ConfigError) -> Self {
        PlisskenError::ConfigValidation {
            message: err.to_string(),
        }
    }
}

// Convert from ManifestError
impl From<crate::manifest::ManifestError> for PlisskenError {
    fn from(err: crate::manifest::ManifestError) -> Self {
        PlisskenError::ManifestParse {
            path: PathBuf::new(),
            message: err.to_string(),
        }
    }
}

// =============================================================================
// Helper constructors for common error patterns
// =============================================================================

impl PlisskenError {
    /// Create a parse error for a Rust file.
    pub fn rust_parse(path: impl Into<PathBuf>, message: impl Into<String>) -> Self {
        PlisskenError::Parse {
            language: "Rust".into(),
            path: path.into(),
            line: None,
            message: message.into(),
        }
    }

    /// Create a parse error for a Rust file with line number.
    pub fn rust_parse_at(
        path: impl Into<PathBuf>,
        line: usize,
        message: impl Into<String>,
    ) -> Self {
        PlisskenError::Parse {
            language: "Rust".into(),
            path: path.into(),
            line: Some(line),
            message: message.into(),
        }
    }

    /// Create a parse error for a Python file.
    pub fn python_parse(path: impl Into<PathBuf>, message: impl Into<String>) -> Self {
        PlisskenError::Parse {
            language: "Python".into(),
            path: path.into(),
            line: None,
            message: message.into(),
        }
    }

    /// Create a parse error for a Python file with line number.
    pub fn python_parse_at(
        path: impl Into<PathBuf>,
        line: usize,
        message: impl Into<String>,
    ) -> Self {
        PlisskenError::Parse {
            language: "Python".into(),
            path: path.into(),
            line: Some(line),
            message: message.into(),
        }
    }

    /// Create a file read error.
    pub fn file_read(path: impl Into<PathBuf>, source: std::io::Error) -> Self {
        let path = path.into();
        PlisskenError::FileRead {
            message: source.to_string(),
            path,
            source,
        }
    }

    /// Create an IO error with context.
    pub fn io(context: impl Into<String>, source: std::io::Error) -> Self {
        PlisskenError::Io {
            context: context.into(),
            source,
        }
    }

    /// Create a config not found error.
    pub fn config_not_found(path: impl Into<PathBuf>) -> Self {
        PlisskenError::ConfigNotFound { path: path.into() }
    }

    /// Create a discovery error.
    pub fn discovery(path: impl Into<PathBuf>, message: impl Into<String>) -> Self {
        PlisskenError::Discovery {
            path: path.into(),
            message: message.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_error_display() {
        let err = PlisskenError::rust_parse("/path/to/file.rs", "unexpected token");
        assert!(err.to_string().contains("Rust"));
        assert!(err.to_string().contains("/path/to/file.rs"));
        assert!(err.to_string().contains("unexpected token"));
    }

    #[test]
    fn test_parse_error_with_line() {
        let err = PlisskenError::python_parse_at("/path/to/file.py", 42, "syntax error");
        assert!(err.to_string().contains("Python"));
        assert!(err.to_string().contains("/path/to/file.py"));
    }

    #[test]
    fn test_config_not_found_display() {
        let err = PlisskenError::config_not_found("/project/plissken.toml");
        assert!(err.to_string().contains("config not found"));
        assert!(err.to_string().contains("plissken.toml"));
    }

    #[test]
    fn test_config_validation_error() {
        let err = PlisskenError::ConfigValidation {
            message: "no language configured".into(),
        };
        assert!(err.to_string().contains("config validation failed"));
        assert!(err.to_string().contains("no language configured"));
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let err: PlisskenError = io_err.into();
        assert!(err.to_string().contains("IO operation failed"));
    }

    #[test]
    fn test_template_error_conversion() {
        // Create a tera error by trying to parse invalid template
        let tera_result = tera::Tera::one_off("{{ invalid", &tera::Context::new(), false);
        if let Err(tera_err) = tera_result {
            let err: PlisskenError = tera_err.into();
            assert!(err.to_string().contains("template error"));
        }
    }

    #[test]
    fn test_discovery_error() {
        let err = PlisskenError::discovery("/src/python", "permission denied");
        assert!(err.to_string().contains("module discovery failed"));
        assert!(err.to_string().contains("/src/python"));
    }

    #[test]
    fn test_file_read_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "not found");
        let err = PlisskenError::file_read("/path/to/file.rs", io_err);
        assert!(err.to_string().contains("failed to read file"));
        assert!(err.to_string().contains("/path/to/file.rs"));
    }

    #[test]
    fn test_crossref_error() {
        let err = PlisskenError::CrossRef {
            message: "unresolved reference".into(),
        };
        assert!(err.to_string().contains("cross-reference error"));
    }
}
