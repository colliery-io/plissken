//! Value filtering utilities.
//!
//! Provides traits and implementations for filtering configuration values.

use crate::config::Value;

/// A filter that can accept or reject values.
pub trait Filter: Send + Sync {
    /// Check if the value passes this filter.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to test.
    ///
    /// # Returns
    ///
    /// `true` if the value should be kept, `false` if it should be filtered out.
    fn accept(&self, value: &Value) -> bool;

    /// Get a description of this filter.
    fn description(&self) -> &str;
}

/// A filter implemented as a function.
///
/// # Type Parameters
///
/// * `F` - The predicate function type.
pub struct FilterFn<F>
where
    F: Fn(&Value) -> bool + Send + Sync,
{
    predicate: F,
    description: String,
}

impl<F> FilterFn<F>
where
    F: Fn(&Value) -> bool + Send + Sync,
{
    /// Create a new function-based filter.
    ///
    /// # Arguments
    ///
    /// * `predicate` - Function that returns true for values to keep.
    /// * `description` - Human-readable description of the filter.
    pub fn new(predicate: F, description: impl Into<String>) -> Self {
        Self {
            predicate,
            description: description.into(),
        }
    }
}

impl<F> Filter for FilterFn<F>
where
    F: Fn(&Value) -> bool + Send + Sync,
{
    fn accept(&self, value: &Value) -> bool {
        (self.predicate)(value)
    }

    fn description(&self) -> &str {
        &self.description
    }
}

/// Filters for string values.
pub struct StringFilter {
    kind: StringFilterKind,
}

enum StringFilterKind {
    MinLength(usize),
    MaxLength(usize),
    Contains(String),
    StartsWith(String),
    EndsWith(String),
    Matches(regex::Regex),
}

impl StringFilter {
    /// Create a filter requiring minimum string length.
    pub fn min_length(len: usize) -> Self {
        Self {
            kind: StringFilterKind::MinLength(len),
        }
    }

    /// Create a filter requiring maximum string length.
    pub fn max_length(len: usize) -> Self {
        Self {
            kind: StringFilterKind::MaxLength(len),
        }
    }

    /// Create a filter requiring the string to contain a substring.
    pub fn contains(substring: impl Into<String>) -> Self {
        Self {
            kind: StringFilterKind::Contains(substring.into()),
        }
    }

    /// Create a filter requiring the string to start with a prefix.
    pub fn starts_with(prefix: impl Into<String>) -> Self {
        Self {
            kind: StringFilterKind::StartsWith(prefix.into()),
        }
    }

    /// Create a filter requiring the string to end with a suffix.
    pub fn ends_with(suffix: impl Into<String>) -> Self {
        Self {
            kind: StringFilterKind::EndsWith(suffix.into()),
        }
    }

    /// Create a filter requiring the string to match a regex pattern.
    ///
    /// # Panics
    ///
    /// Panics if the pattern is invalid.
    pub fn matches(pattern: &str) -> Self {
        Self {
            kind: StringFilterKind::Matches(regex::Regex::new(pattern).unwrap()),
        }
    }
}

impl Filter for StringFilter {
    fn accept(&self, value: &Value) -> bool {
        let s = match value.as_str() {
            Some(s) => s,
            None => return false,
        };

        match &self.kind {
            StringFilterKind::MinLength(len) => s.len() >= *len,
            StringFilterKind::MaxLength(len) => s.len() <= *len,
            StringFilterKind::Contains(sub) => s.contains(sub.as_str()),
            StringFilterKind::StartsWith(prefix) => s.starts_with(prefix.as_str()),
            StringFilterKind::EndsWith(suffix) => s.ends_with(suffix.as_str()),
            StringFilterKind::Matches(re) => re.is_match(s),
        }
    }

    fn description(&self) -> &str {
        match &self.kind {
            StringFilterKind::MinLength(_) => "minimum length",
            StringFilterKind::MaxLength(_) => "maximum length",
            StringFilterKind::Contains(_) => "contains",
            StringFilterKind::StartsWith(_) => "starts with",
            StringFilterKind::EndsWith(_) => "ends with",
            StringFilterKind::Matches(_) => "matches pattern",
        }
    }
}

/// Combine multiple filters with AND logic.
pub struct AllOf {
    filters: Vec<Box<dyn Filter>>,
}

impl AllOf {
    /// Create a new composite filter.
    pub fn new(filters: Vec<Box<dyn Filter>>) -> Self {
        Self { filters }
    }
}

impl Filter for AllOf {
    fn accept(&self, value: &Value) -> bool {
        self.filters.iter().all(|f| f.accept(value))
    }

    fn description(&self) -> &str {
        "all of"
    }
}

/// Combine multiple filters with OR logic.
pub struct AnyOf {
    filters: Vec<Box<dyn Filter>>,
}

impl AnyOf {
    /// Create a new composite filter.
    pub fn new(filters: Vec<Box<dyn Filter>>) -> Self {
        Self { filters }
    }
}

impl Filter for AnyOf {
    fn accept(&self, value: &Value) -> bool {
        self.filters.iter().any(|f| f.accept(value))
    }

    fn description(&self) -> &str {
        "any of"
    }
}

// Stub for regex - in real code this would use the regex crate
mod regex {
    pub struct Regex(String);

    impl Regex {
        pub fn new(pattern: &str) -> Result<Self, ()> {
            Ok(Self(pattern.to_string()))
        }

        pub fn is_match(&self, _text: &str) -> bool {
            // Simplified stub
            true
        }
    }
}
