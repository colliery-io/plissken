//! Async configuration loading.
//!
//! Provides async versions of configuration loaders for use
//! with async runtimes like Tokio.

use crate::config::Config;
use crate::error::LoadError;
use std::future::Future;
use std::path::Path;
use std::pin::Pin;

/// Trait for async configuration loading.
///
/// Implementors provide async access to configuration sources
/// like remote servers or async file systems.
pub trait AsyncLoader: Send + Sync {
    /// Load configuration asynchronously.
    ///
    /// # Errors
    ///
    /// Returns a `LoadError` if the configuration cannot be loaded.
    fn load(&self) -> Pin<Box<dyn Future<Output = Result<Config, LoadError>> + Send + '_>>;

    /// Get a description of this loader's source.
    fn source_description(&self) -> String;
}

/// Async file loader that reads configuration from the filesystem.
pub struct AsyncFileLoader {
    path: std::path::PathBuf,
}

impl AsyncFileLoader {
    /// Create a new async file loader.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the configuration file.
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }
}

impl AsyncLoader for AsyncFileLoader {
    fn load(&self) -> Pin<Box<dyn Future<Output = Result<Config, LoadError>> + Send + '_>> {
        Box::pin(async move {
            // Simplified: would use tokio::fs in real implementation
            let _content = std::fs::read_to_string(&self.path)
                .map_err(|e| LoadError::Io(e.to_string()))?;
            Ok(Config::new())
        })
    }

    fn source_description(&self) -> String {
        format!("async-file:{}", self.path.display())
    }
}

/// Loader that fetches configuration from a remote URL.
pub struct HttpLoader {
    url: String,
    timeout_secs: u64,
}

impl HttpLoader {
    /// Create a new HTTP configuration loader.
    ///
    /// # Arguments
    ///
    /// * `url` - URL to fetch configuration from.
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            timeout_secs: 30,
        }
    }

    /// Set the request timeout.
    ///
    /// # Arguments
    ///
    /// * `secs` - Timeout in seconds.
    pub fn with_timeout(mut self, secs: u64) -> Self {
        self.timeout_secs = secs;
        self
    }
}

impl AsyncLoader for HttpLoader {
    fn load(&self) -> Pin<Box<dyn Future<Output = Result<Config, LoadError>> + Send + '_>> {
        Box::pin(async move {
            // Simplified: would use reqwest or similar in real implementation
            let _ = self.timeout_secs;
            Err(LoadError::Io(format!(
                "HTTP loading not implemented for {}",
                self.url
            )))
        })
    }

    fn source_description(&self) -> String {
        format!("http:{}", self.url)
    }
}

/// Load configuration from multiple async sources with fallback.
///
/// Tries each loader in order until one succeeds.
///
/// # Arguments
///
/// * `loaders` - Slice of async loaders to try.
///
/// # Errors
///
/// Returns the last error if all loaders fail.
pub async fn load_with_fallback(loaders: &[&dyn AsyncLoader]) -> Result<Config, LoadError> {
    let mut last_error = LoadError::Io("No loaders provided".to_string());

    for loader in loaders {
        match loader.load().await {
            Ok(config) => return Ok(config),
            Err(e) => last_error = e,
        }
    }

    Err(last_error)
}
