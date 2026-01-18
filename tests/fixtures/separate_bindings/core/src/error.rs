//! Error types for the pipeline library.

use thiserror::Error;

/// Errors that can occur during pipeline execution.
#[derive(Debug, Error)]
pub enum PipelineError {
    /// A stage failed after exhausting retries.
    #[error("Stage '{stage}' failed after {attempts} attempts: {source}")]
    StageFailedAfterRetries {
        stage: String,
        attempts: usize,
        #[source]
        source: StageError,
    },

    /// Pipeline timed out.
    #[error("Pipeline timed out after {0:?}")]
    Timeout(std::time::Duration),

    /// Invalid pipeline configuration.
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
}

/// Errors that can occur during stage execution.
#[derive(Debug, Error)]
pub enum StageError {
    /// Data validation failed.
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// Transformation failed.
    #[error("Transform error: {0}")]
    TransformError(String),

    /// I/O error during processing.
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// Generic processing error.
    #[error("Processing error: {0}")]
    ProcessingError(String),
}
