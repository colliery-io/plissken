//! A configuration management library.
//!
//! Provides type-safe configuration loading and validation
//! with support for multiple formats and environments.
//!
//! # Features
//!
//! - Multiple configuration sources (files, environment variables, HTTP)
//! - Async loading support
//! - Value transformation pipelines
//! - Validation rules
//!
//! # Example
//!
//! ```ignore
//! use pure_rust::{Config, FileLoader, Loader, Validator};
//!
//! let loader = FileLoader::new("config.toml")?;
//! let config = loader.load()?;
//!
//! let validator = Validator::new()
//!     .required("database.host")
//!     .required("database.port");
//!
//! validator.validate(&config)?;
//! ```

mod config;
mod loader;
mod error;
mod validate;
mod async_loader;
pub mod transform;

pub use config::{Config, ConfigBuilder, Value};
pub use loader::{Loader, FileLoader, EnvLoader, FileFormat};
pub use error::{ConfigError, LoadError, ValidationError};
pub use validate::{Validator, Rule, FieldRule};
pub use async_loader::{AsyncLoader, AsyncFileLoader, HttpLoader, load_with_fallback};
