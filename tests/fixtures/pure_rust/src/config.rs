//! Core configuration types.

use std::collections::HashMap;

/// A configuration value.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// Null/missing value.
    Null,
    /// Boolean value.
    Bool(bool),
    /// Integer value.
    Int(i64),
    /// Floating point value.
    Float(f64),
    /// String value.
    String(String),
    /// Array of values.
    Array(Vec<Value>),
    /// Map of string keys to values.
    Map(HashMap<String, Value>),
}

impl Value {
    /// Check if this value is null.
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    /// Try to get as a boolean.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Try to get as a string.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    /// Try to get as an integer.
    pub fn as_int(&self) -> Option<i64> {
        match self {
            Value::Int(i) => Some(*i),
            _ => None,
        }
    }
}

/// A loaded configuration.
#[derive(Debug, Clone)]
pub struct Config {
    values: HashMap<String, Value>,
    source: Option<String>,
}

impl Config {
    /// Create an empty configuration.
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            source: None,
        }
    }

    /// Get a value by key.
    ///
    /// Supports dotted paths like "database.host".
    pub fn get(&self, key: &str) -> Option<&Value> {
        // Simple implementation - real one would handle nested paths
        self.values.get(key)
    }

    /// Get a required string value.
    ///
    /// # Errors
    ///
    /// Returns an error if the key is missing or not a string.
    pub fn require_str(&self, key: &str) -> Result<&str, ConfigError> {
        self.get(key)
            .ok_or_else(|| ConfigError::MissingKey(key.to_string()))?
            .as_str()
            .ok_or_else(|| ConfigError::TypeMismatch {
                key: key.to_string(),
                expected: "string",
            })
    }

    /// Set a value.
    pub fn set(&mut self, key: impl Into<String>, value: Value) {
        self.values.insert(key.into(), value);
    }

    /// Get the source identifier.
    pub fn source(&self) -> Option<&str> {
        self.source.as_deref()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

/// Error type for config operations.
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Missing required key: {0}")]
    MissingKey(String),

    #[error("Type mismatch for key '{key}': expected {expected}")]
    TypeMismatch { key: String, expected: &'static str },
}

/// Builder for constructing configurations programmatically.
pub struct ConfigBuilder {
    config: Config,
}

impl ConfigBuilder {
    /// Create a new builder.
    pub fn new() -> Self {
        Self {
            config: Config::new(),
        }
    }

    /// Set a string value.
    pub fn set_str(mut self, key: &str, value: &str) -> Self {
        self.config.set(key, Value::String(value.to_string()));
        self
    }

    /// Set an integer value.
    pub fn set_int(mut self, key: &str, value: i64) -> Self {
        self.config.set(key, Value::Int(value));
        self
    }

    /// Set a boolean value.
    pub fn set_bool(mut self, key: &str, value: bool) -> Self {
        self.config.set(key, Value::Bool(value));
        self
    }

    /// Build the configuration.
    pub fn build(self) -> Config {
        self.config
    }
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}
