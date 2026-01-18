//! Internal implementation details (not exposed to Python).

use pyo3::PyObject;
use std::collections::HashMap;
use std::time::Duration;

/// A task definition.
#[derive(Clone)]
pub struct Task {
    pub name: String,
    pub description: Option<String>,
    pub dependencies: Vec<String>,
    pub is_async: bool,
}

impl Task {
    pub fn new(name: &str, description: Option<&str>) -> Self {
        Self {
            name: name.to_string(),
            description: description.map(String::from),
            dependencies: vec![],
            is_async: false,
        }
    }

    pub fn add_dependency(&mut self, name: &str) {
        self.dependencies.push(name.to_string());
    }
}

/// Executes tasks with dependency resolution.
pub struct TaskExecutor {
    max_parallel: usize,
    tasks: HashMap<String, (Task, PyObject)>,
}

impl TaskExecutor {
    pub fn new(max_parallel: usize) -> Result<Self, ExecutorError> {
        if max_parallel == 0 {
            return Err(ExecutorError::InvalidConfig("max_parallel must be > 0".into()));
        }
        Ok(Self {
            max_parallel,
            tasks: HashMap::new(),
        })
    }

    pub fn register(&mut self, task: Task, handler: PyObject) -> Result<(), ExecutorError> {
        if self.tasks.contains_key(&task.name) {
            return Err(ExecutorError::DuplicateTask(task.name.clone()));
        }
        self.tasks.insert(task.name.clone(), (task, handler));
        Ok(())
    }

    pub fn run(&self, task_name: &str, dry_run: bool) -> Result<RunResult, ExecutorError> {
        if !self.tasks.contains_key(task_name) {
            return Err(ExecutorError::TaskNotFound(task_name.to_string()));
        }

        // Simplified: in reality would do topological sort and parallel execution
        let _ = (self.max_parallel, dry_run);

        Ok(RunResult {
            success: true,
            tasks_run: 1,
            duration: Duration::from_millis(100),
            failed: vec![],
        })
    }

    pub fn list_tasks(&self) -> Vec<String> {
        self.tasks.keys().cloned().collect()
    }
}

/// Result of running tasks.
pub struct RunResult {
    pub success: bool,
    pub tasks_run: usize,
    pub duration: Duration,
    pub failed: Vec<String>,
}

/// Errors that can occur during execution.
#[derive(Debug, thiserror::Error)]
pub enum ExecutorError {
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Task '{0}' already registered")]
    DuplicateTask(String),

    #[error("Task '{0}' not found")]
    TaskNotFound(String),

    #[error("Circular dependency detected: {0}")]
    CircularDependency(String),

    #[error("Task execution failed: {0}")]
    ExecutionFailed(String),
}
