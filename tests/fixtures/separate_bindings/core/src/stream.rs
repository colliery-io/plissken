//! Async streaming support for pipelines.
//!
//! This module provides async streaming primitives for processing
//! data batches in an async context.

use crate::batch::DataBatch;
use crate::error::StageError;
use std::future::Future;
use std::pin::Pin;

/// An async data source that yields batches.
///
/// # Type Parameters
///
/// * `E` - The error type for failed reads.
pub trait AsyncSource: Send + Sync {
    /// The error type.
    type Error: std::error::Error + Send + Sync;

    /// Read the next batch from this source.
    ///
    /// Returns `Ok(None)` when the source is exhausted.
    fn read_batch(
        &mut self,
    ) -> Pin<Box<dyn Future<Output = Result<Option<DataBatch>, Self::Error>> + Send + '_>>;

    /// Get an estimated count of remaining batches.
    ///
    /// Returns `None` if the count is unknown.
    fn estimated_remaining(&self) -> Option<usize> {
        None
    }
}

/// An async data sink that consumes batches.
pub trait AsyncSink: Send + Sync {
    /// The error type.
    type Error: std::error::Error + Send + Sync;

    /// Write a batch to this sink.
    fn write_batch(
        &mut self,
        batch: DataBatch,
    ) -> Pin<Box<dyn Future<Output = Result<(), Self::Error>> + Send + '_>>;

    /// Flush any buffered data.
    fn flush(&mut self) -> Pin<Box<dyn Future<Output = Result<(), Self::Error>> + Send + '_>>;

    /// Close the sink, flushing and releasing resources.
    fn close(self) -> Pin<Box<dyn Future<Output = Result<(), Self::Error>> + Send>>;
}

/// An async processor that transforms batches.
pub trait AsyncProcessor: Send + Sync {
    /// Process a batch asynchronously.
    ///
    /// # Arguments
    ///
    /// * `batch` - The input batch to process.
    ///
    /// # Returns
    ///
    /// The processed output batch.
    fn process(
        &self,
        batch: DataBatch,
    ) -> Pin<Box<dyn Future<Output = Result<DataBatch, StageError>> + Send + '_>>;

    /// Get the processor name.
    fn name(&self) -> &str;
}

/// A streaming pipeline that processes batches from source to sink.
pub struct StreamingPipeline<S, K>
where
    S: AsyncSource,
    K: AsyncSink,
{
    source: S,
    sink: K,
    processors: Vec<Box<dyn AsyncProcessor>>,
    batch_size: usize,
}

impl<S, K> StreamingPipeline<S, K>
where
    S: AsyncSource,
    K: AsyncSink,
{
    /// Create a new streaming pipeline.
    ///
    /// # Arguments
    ///
    /// * `source` - The data source.
    /// * `sink` - The data sink.
    pub fn new(source: S, sink: K) -> Self {
        Self {
            source,
            sink,
            processors: Vec::new(),
            batch_size: 1000,
        }
    }

    /// Set the batch size for processing.
    pub fn batch_size(mut self, size: usize) -> Self {
        self.batch_size = size;
        self
    }

    /// Add a processor to the pipeline.
    pub fn add_processor(mut self, processor: impl AsyncProcessor + 'static) -> Self {
        self.processors.push(Box::new(processor));
        self
    }

    /// Get the number of processors.
    pub fn processor_count(&self) -> usize {
        self.processors.len()
    }
}

/// Statistics from a streaming run.
#[derive(Debug, Clone, Default)]
pub struct StreamStats {
    /// Number of batches read from source.
    pub batches_read: usize,
    /// Number of batches written to sink.
    pub batches_written: usize,
    /// Total rows processed.
    pub rows_processed: usize,
    /// Number of errors encountered.
    pub errors: usize,
}

impl StreamStats {
    /// Create empty statistics.
    pub fn new() -> Self {
        Self::default()
    }

    /// Merge statistics from another run.
    pub fn merge(&mut self, other: &StreamStats) {
        self.batches_read += other.batches_read;
        self.batches_written += other.batches_written;
        self.rows_processed += other.rows_processed;
        self.errors += other.errors;
    }
}

/// A batch collector that accumulates batches.
#[derive(Debug, Default)]
pub struct BatchCollector {
    batches: Vec<DataBatch>,
    max_batches: Option<usize>,
}

impl BatchCollector {
    /// Create a new collector with no limit.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a collector with a maximum batch count.
    pub fn with_limit(max: usize) -> Self {
        Self {
            batches: Vec::new(),
            max_batches: Some(max),
        }
    }

    /// Add a batch to the collector.
    ///
    /// Returns `false` if the limit was reached and the batch wasn't added.
    pub fn push(&mut self, batch: DataBatch) -> bool {
        if let Some(max) = self.max_batches {
            if self.batches.len() >= max {
                return false;
            }
        }
        self.batches.push(batch);
        true
    }

    /// Get the collected batches.
    pub fn into_batches(self) -> Vec<DataBatch> {
        self.batches
    }

    /// Get the number of collected batches.
    pub fn len(&self) -> usize {
        self.batches.len()
    }

    /// Check if the collector is empty.
    pub fn is_empty(&self) -> bool {
        self.batches.is_empty()
    }
}
