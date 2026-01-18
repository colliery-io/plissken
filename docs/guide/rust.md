# Rust Projects

Guide for documenting pure Rust projects with plissken.

## Configuration

```toml
[project]
name = "mycrate"
version_from = "cargo"

[output]
path = "docs/api"
template = "mkdocs-material"

[rust]
crates = ["."]
```

## Doc Comments

### Module Documentation

```rust
//! # My Crate
//!
//! A library for processing data efficiently.
//!
//! ## Features
//!
//! - Fast processing
//! - Memory efficient
//! - Easy to use
```

### Item Documentation

```rust
/// Process input data and return results.
///
/// Takes a slice of strings and processes them with configurable
/// options.
///
/// # Arguments
///
/// * `data` - Slice of strings to process
/// * `options` - Processing configuration
///
/// # Returns
///
/// A vector of processed results.
///
/// # Errors
///
/// Returns `ProcessError` if:
/// - Input data is empty
/// - Options are invalid
///
/// # Examples
///
/// ```rust
/// let result = process_data(&["a", "b"], Options::default())?;
/// assert_eq!(result.len(), 2);
/// ```
pub fn process_data(data: &[&str], options: Options) -> Result<Vec<String>, ProcessError> {
    // ...
}
```

## Struct Documentation

```rust
/// A data processor with configurable behavior.
///
/// `DataProcessor` provides a flexible interface for transforming
/// data with various processing strategies.
///
/// # Examples
///
/// ```rust
/// let processor = DataProcessor::new("main");
/// let result = processor.process(&[1, 2, 3]);
/// ```
#[derive(Debug, Clone)]
pub struct DataProcessor {
    /// Processor identifier.
    pub name: String,
    /// Configuration options.
    config: Config,
}

impl DataProcessor {
    /// Create a new processor with the given name.
    ///
    /// # Arguments
    ///
    /// * `name` - Identifier for this processor
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            config: Config::default(),
        }
    }

    /// Process input data.
    ///
    /// # Arguments
    ///
    /// * `data` - Slice of integers to process
    ///
    /// # Returns
    ///
    /// Vector of processed integers.
    pub fn process(&self, data: &[i32]) -> Vec<i32> {
        data.iter().map(|x| x * 2).collect()
    }
}
```

## Enum Documentation

```rust
/// Processing mode selection.
///
/// Determines how data is processed and validated.
#[derive(Debug, Clone, Copy)]
pub enum ProcessMode {
    /// Fast processing with minimal validation.
    Fast,
    /// Standard processing with full validation.
    Standard,
    /// Strict processing with extra checks.
    ///
    /// Use this mode for critical data that requires
    /// additional integrity verification.
    Strict,
}
```

## Visibility Badges

| Visibility | Badge |
|------------|-------|
| `pub` | pub |
| `pub(crate)` | pub(crate) |
| `pub(super)` | pub(super) |
| (private) | private |

## Unsafe Code

Unsafe functions get a special badge:

```rust
/// Perform an unchecked operation.
///
/// # Safety
///
/// Caller must ensure:
/// - Pointer is valid and aligned
/// - Data is initialized
pub unsafe fn unchecked_operation(ptr: *mut u8) {
    // ...
}
```

## Workspace Projects

For workspaces, list each crate:

```toml
[rust]
crates = [
    "crates/core",
    "crates/utils",
    "crates/cli",
]
```

Each crate gets its own section in the generated docs.
