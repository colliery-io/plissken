//! Core data pipeline library.
//!
//! This crate provides a type-safe, composable pipeline system
//! for data processing workflows. It is a pure Rust library with
//! no Python dependencies.
//!
//! # Example
//!
//! ```rust
//! use separate_bindings_core::{Pipeline, Stage, DataBatch};
//!
//! let pipeline = Pipeline::new("etl")
//!     .stage(Stage::new("extract", extract_fn))
//!     .stage(Stage::new("transform", transform_fn))
//!     .stage(Stage::new("load", load_fn));
//!
//! pipeline.run()?;
//! ```
//!
//! # Async Support
//!
//! For async workloads, use the streaming API:
//!
//! ```rust,ignore
//! use separate_bindings_core::stream::{AsyncSource, AsyncSink, StreamingPipeline};
//! ```

mod pipeline;
mod stage;
mod batch;
mod error;
pub mod stream;

pub use pipeline::{Pipeline, PipelineConfig, PipelineResult};
pub use stage::{Stage, StageBuilder};
pub use batch::{DataBatch, Value, Row, BatchMetadata};
pub use error::{PipelineError, StageError};
pub use stream::{AsyncSource, AsyncSink, AsyncProcessor, StreamStats, BatchCollector};
