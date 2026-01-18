//! Configuration validation.

use crate::config::{Config, Value};
use crate::error::ValidationError;

/// A validation rule.
pub trait Rule: Send + Sync {
    /// Validate the configuration.
    ///
    /// Returns Ok(()) if valid, or an error describing the problem.
    fn validate(&self, config: &Config) -> Result<(), ValidationError>;

    /// Get a description of this rule.
    fn description(&self) -> &str;
}

/// A configuration validator that applies multiple rules.
pub struct Validator {
    rules: Vec<Box<dyn Rule>>,
}

impl Validator {
    /// Create a new validator with no rules.
    pub fn new() -> Self {
        Self { rules: vec![] }
    }

    /// Add a rule to this validator.
    pub fn rule(mut self, rule: impl Rule + 'static) -> Self {
        self.rules.push(Box::new(rule));
        self
    }

    /// Add a required field rule.
    pub fn required(self, field: &str) -> Self {
        self.rule(RequiredField(field.to_string()))
    }

    /// Validate a configuration against all rules.
    ///
    /// Returns all validation errors, not just the first one.
    pub fn validate(&self, config: &Config) -> Result<(), ValidationError> {
        let errors: Vec<_> = self
            .rules
            .iter()
            .filter_map(|r| r.validate(config).err())
            .collect();

        if errors.is_empty() {
            Ok(())
        } else if errors.len() == 1 {
            Err(errors.into_iter().next().unwrap())
        } else {
            Err(ValidationError::Multiple(errors))
        }
    }
}

impl Default for Validator {
    fn default() -> Self {
        Self::new()
    }
}

/// Rule that requires a field to be present and non-null.
struct RequiredField(String);

impl Rule for RequiredField {
    fn validate(&self, config: &Config) -> Result<(), ValidationError> {
        match config.get(&self.0) {
            Some(v) if !v.is_null() => Ok(()),
            _ => Err(ValidationError::MissingField(self.0.clone())),
        }
    }

    fn description(&self) -> &str {
        "required field"
    }
}

/// Rule that validates a field matches a predicate.
pub struct FieldRule<F> {
    field: String,
    predicate: F,
    message: String,
}

impl<F> FieldRule<F>
where
    F: Fn(&Value) -> bool + Send + Sync,
{
    /// Create a new field rule.
    pub fn new(field: &str, predicate: F, message: &str) -> Self {
        Self {
            field: field.to_string(),
            predicate,
            message: message.to_string(),
        }
    }
}

impl<F> Rule for FieldRule<F>
where
    F: Fn(&Value) -> bool + Send + Sync,
{
    fn validate(&self, config: &Config) -> Result<(), ValidationError> {
        match config.get(&self.field) {
            Some(v) if (self.predicate)(v) => Ok(()),
            Some(_) => Err(ValidationError::InvalidValue {
                field: self.field.clone(),
                message: self.message.clone(),
            }),
            None => Ok(()), // Missing is OK, use RequiredField for that
        }
    }

    fn description(&self) -> &str {
        &self.message
    }
}
