//! Data batch types.

use std::collections::HashMap;

/// A batch of data flowing through the pipeline.
///
/// Batches are immutable containers for rows of data.
/// Each row is a map of column names to values.
#[derive(Debug, Clone, Default)]
pub struct DataBatch {
    rows: Vec<Row>,
    metadata: BatchMetadata,
}

/// A single row of data.
pub type Row = HashMap<String, Value>;

/// A data value that can appear in a row.
#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Bytes(Vec<u8>),
    List(Vec<Value>),
    Map(HashMap<String, Value>),
}

/// Metadata about a batch.
#[derive(Debug, Clone, Default)]
pub struct BatchMetadata {
    /// Source identifier.
    pub source: Option<String>,
    /// Schema version.
    pub schema_version: Option<String>,
    /// Custom attributes.
    pub attributes: HashMap<String, String>,
}

impl DataBatch {
    /// Create an empty batch.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a batch from rows.
    pub fn from_rows(rows: Vec<Row>) -> Self {
        Self {
            rows,
            metadata: BatchMetadata::default(),
        }
    }

    /// Get the number of rows.
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    /// Check if the batch is empty.
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// Get a reference to the rows.
    pub fn rows(&self) -> &[Row] {
        &self.rows
    }

    /// Get the metadata.
    pub fn metadata(&self) -> &BatchMetadata {
        &self.metadata
    }

    /// Set metadata.
    pub fn with_metadata(mut self, metadata: BatchMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    /// Map over all rows.
    pub fn map<F>(self, f: F) -> Self
    where
        F: Fn(Row) -> Row,
    {
        Self {
            rows: self.rows.into_iter().map(f).collect(),
            metadata: self.metadata,
        }
    }

    /// Filter rows.
    pub fn filter<F>(self, predicate: F) -> Self
    where
        F: Fn(&Row) -> bool,
    {
        Self {
            rows: self.rows.into_iter().filter(predicate).collect(),
            metadata: self.metadata,
        }
    }
}
