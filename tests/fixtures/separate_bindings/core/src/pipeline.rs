//! Pipeline orchestration.

use crate::batch::DataBatch;
use crate::error::PipelineError;
use crate::stage::Stage;
use std::time::{Duration, Instant};

/// Configuration options for pipeline execution.
#[derive(Debug, Clone)]
pub struct PipelineConfig {
    /// Maximum retries per stage on failure.
    pub max_retries: usize,
    /// Whether to continue on stage failure.
    pub continue_on_error: bool,
    /// Timeout for the entire pipeline.
    pub timeout: Option<Duration>,
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            continue_on_error: false,
            timeout: None,
        }
    }
}

/// A data processing pipeline.
///
/// Pipelines consist of ordered stages that process data batches
/// sequentially. Each stage can transform, filter, or enrich the data.
#[derive(Default)]
pub struct Pipeline {
    name: String,
    stages: Vec<Stage>,
    config: PipelineConfig,
}

impl Pipeline {
    /// Create a new pipeline with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            stages: vec![],
            config: PipelineConfig::default(),
        }
    }

    /// Get the pipeline name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Add a stage to the pipeline.
    ///
    /// Stages are executed in the order they are added.
    pub fn stage(mut self, stage: Stage) -> Self {
        self.stages.push(stage);
        self
    }

    /// Configure the pipeline.
    pub fn config(mut self, config: PipelineConfig) -> Self {
        self.config = config;
        self
    }

    /// Get the number of stages.
    pub fn stage_count(&self) -> usize {
        self.stages.len()
    }

    /// Run the pipeline with the given input batch.
    ///
    /// Returns the final processed batch and execution statistics.
    pub fn run(&self, input: DataBatch) -> Result<PipelineResult, PipelineError> {
        let start = Instant::now();
        let mut current = input;
        let mut stages_run = 0;

        for stage in &self.stages {
            let stage_result = self.run_stage(stage, current)?;
            current = stage_result;
            stages_run += 1;
        }

        Ok(PipelineResult {
            output: current,
            stages_run,
            duration: start.elapsed(),
        })
    }

    fn run_stage(&self, stage: &Stage, input: DataBatch) -> Result<DataBatch, PipelineError> {
        let mut attempts = 0;
        let mut last_error = None;

        while attempts <= self.config.max_retries {
            match stage.process(input.clone()) {
                Ok(output) => return Ok(output),
                Err(e) => {
                    last_error = Some(e);
                    attempts += 1;
                }
            }
        }

        Err(PipelineError::StageFailedAfterRetries {
            stage: stage.name().to_string(),
            attempts,
            source: last_error.unwrap(),
        })
    }
}

/// Result of a pipeline execution.
#[derive(Debug)]
pub struct PipelineResult {
    /// The final output batch.
    pub output: DataBatch,
    /// Number of stages that were run.
    pub stages_run: usize,
    /// Total execution time.
    pub duration: Duration,
}
