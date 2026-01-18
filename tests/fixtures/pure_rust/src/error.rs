//! Error types.

/// Errors that can occur when loading configuration.
#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    /// I/O error reading configuration.
    #[error("I/O error: {0}")]
    Io(String),

    /// Unknown or unsupported format.
    #[error("Unknown format for: {0}")]
    UnknownFormat(String),

    /// Parse error in configuration file.
    #[error("Parse error: {0}")]
    ParseError(String),
}

/// Errors that can occur during validation.
#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    /// A required field is missing.
    #[error("Missing required field: {0}")]
    MissingField(String),

    /// A field has an invalid value.
    #[error("Invalid value for '{field}': {message}")]
    InvalidValue { field: String, message: String },

    /// Multiple validation errors occurred.
    #[error("Validation failed with {0} errors")]
    Multiple(Vec<ValidationError>),
}
