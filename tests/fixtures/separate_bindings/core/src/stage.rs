//! Pipeline stages.

use crate::batch::DataBatch;
use crate::error::StageError;

/// A processing stage in a pipeline.
///
/// Stages are the unit of work in a pipeline. Each stage receives
/// a data batch, processes it, and produces an output batch.
pub struct Stage {
    name: String,
    processor: Box<dyn Fn(DataBatch) -> Result<DataBatch, StageError> + Send + Sync>,
}

impl Stage {
    /// Create a new stage with the given name and processor function.
    pub fn new<F>(name: impl Into<String>, processor: F) -> Self
    where
        F: Fn(DataBatch) -> Result<DataBatch, StageError> + Send + Sync + 'static,
    {
        Self {
            name: name.into(),
            processor: Box::new(processor),
        }
    }

    /// Get the stage name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Process a data batch.
    pub fn process(&self, input: DataBatch) -> Result<DataBatch, StageError> {
        (self.processor)(input)
    }
}

/// Builder for creating stages with additional configuration.
pub struct StageBuilder {
    name: String,
    retryable: bool,
    timeout_ms: Option<u64>,
}

impl StageBuilder {
    /// Start building a new stage.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            retryable: true,
            timeout_ms: None,
        }
    }

    /// Mark this stage as non-retryable.
    pub fn no_retry(mut self) -> Self {
        self.retryable = false;
        self
    }

    /// Set a timeout for this stage in milliseconds.
    pub fn timeout_ms(mut self, ms: u64) -> Self {
        self.timeout_ms = Some(ms);
        self
    }

    /// Build the stage with the given processor.
    pub fn build<F>(self, processor: F) -> Stage
    where
        F: Fn(DataBatch) -> Result<DataBatch, StageError> + Send + Sync + 'static,
    {
        // In a real impl, would wrap processor with retry/timeout logic
        let _ = (self.retryable, self.timeout_ms);
        Stage::new(self.name, processor)
    }
}
