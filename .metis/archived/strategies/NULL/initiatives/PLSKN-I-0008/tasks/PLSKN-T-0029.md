---
id: create-unified-plisskenerror-type
level: task
title: "Create unified PlisskenError type"
short_code: "PLSKN-T-0029"
created_at: 2026-01-16T18:00:11.549555+00:00
updated_at: 2026-01-17T01:31:14.434749+00:00
parent: PLSKN-I-0008
blocked_by: []
archived: true

tags:
  - "#task"
  - "#phase/completed"


exit_criteria_met: false
strategy_id: NULL
initiative_id: PLSKN-I-0008
---

# Create unified PlisskenError type

## Parent Initiative

[[PLSKN-I-0008]] Core API Stabilization

## Objective

Replace the inconsistent mix of `anyhow::Error`, `tera::Error`, and raw strings with a unified `PlisskenError` enum. This provides consistent error handling, better error messages, and a stable error API for library consumers.

## Current Problem

The codebase currently has inconsistent error handling:
- Some functions return `anyhow::Result<T>` (opaque errors)
- Some return `Result<T, tera::Error>` (leaking implementation detail)
- Some functions that can fail are infallible (silently swallow errors)
- Error messages lack context (no file paths, no hints)

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [x] Create `PlisskenError` enum with variants for all error categories
- [x] Implement `std::error::Error` and `Display` for consistent formatting
- [x] Add source location context (file, line) where applicable
- [x] Implement `From` conversions for underlying error types
- [x] Migrate all public API functions to return `Result<T, PlisskenError>`
- [x] Ensure error messages include actionable hints
- [x] Add `thiserror` derive for ergonomic implementation

## Error Type Design

```rust
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PlisskenError {
    // Config errors
    #[error("config not found: {path}")]
    ConfigNotFound { path: PathBuf },
    
    #[error("config parse error: {message}")]
    ConfigParse { 
        message: String,
        #[source] source: Option<toml::de::Error>,
    },
    
    #[error("config validation failed: {message}")]
    ConfigValidation { message: String },
    
    // Parse errors
    #[error("failed to parse {language} file: {path}")]
    Parse {
        language: Language,
        path: PathBuf,
        line: Option<usize>,
        message: String,
    },
    
    // Render errors  
    #[error("template error: {message}")]
    Template {
        message: String,
        #[source] source: tera::Error,
    },
    
    // IO errors
    #[error("IO error: {context}")]
    Io {
        context: String,
        #[source] source: std::io::Error,
    },
    
    // Cross-reference errors
    #[error("cross-reference error: {message}")]
    CrossRef { message: String },
}

#[derive(Debug, Clone, Copy)]
pub enum Language {
    Rust,
    Python,
}
```

## Implementation Notes

### Files to Create/Modify

1. **`crates/plissken-core/src/error.rs`** (new) - Error type definition

2. **`crates/plissken-core/src/lib.rs`** - Re-export error type:
   ```rust
   pub mod error;
   pub use error::PlisskenError;
   
   pub type Result<T> = std::result::Result<T, PlisskenError>;
   ```

3. **All modules** - Update function signatures to use new error type

### Migration Strategy

1. Create the error type with all variants
2. Add `From` implementations for easy conversion
3. Update one module at a time, starting with config
4. Run tests after each module migration
5. Remove `anyhow` dependency when complete

### From Implementations

```rust
impl From<std::io::Error> for PlisskenError {
    fn from(err: std::io::Error) -> Self {
        PlisskenError::Io {
            context: "file operation failed".into(),
            source: err,
        }
    }
}

impl From<tera::Error> for PlisskenError {
    fn from(err: tera::Error) -> Self {
        PlisskenError::Template {
            message: err.to_string(),
            source: err,
        }
    }
}
```

### Integration with PLSKN-T-0023

This task creates the error infrastructure that T-0023 (error message improvements) will enhance with hints and better formatting for CLI output.

## Status Updates

### Session 2026-01-17

**Completed implementation:**

1. **Created `error.rs` module** (`crates/plissken-core/src/error.rs`):
   - `PlisskenError` enum with variants:
     - `ConfigNotFound`, `ConfigParse`, `ConfigValidation` - config errors
     - `Parse` - language parsing errors with path and line info
     - `FileRead`, `OutputWrite` - file operation errors
     - `Template` - Tera rendering errors
     - `CrossRef` - cross-reference errors
     - `Discovery` - module discovery errors
     - `ManifestParse` - Cargo.toml/pyproject.toml errors
     - `Io` - generic IO with context
   - Helper constructors: `rust_parse()`, `python_parse()`, `file_read()`, etc.
   - `Result<T>` type alias for convenience

2. **Added `From` implementations**:
   - `From<std::io::Error>` → `PlisskenError::Io`
   - `From<tera::Error>` → `PlisskenError::Template`
   - `From<toml::de::Error>` → `PlisskenError::ConfigParse`
   - `From<ConfigError>` → `PlisskenError::ConfigValidation`
   - `From<ManifestError>` → `PlisskenError::ManifestParse`

3. **Migrated public API functions**:
   - `Config::from_file()` - returns specific `ConfigNotFound` or `ConfigParse`
   - `RustParser::parse_file/parse_str()` - returns `FileRead` or `Parse` with line number
   - `PythonParser::parse_file/parse_str()` - returns `FileRead` or `Parse`
   - `Renderer::new()` - returns `Template` error

**Tests added:** 9 new tests for error module
**All 176 tests passing**

**Files modified:**
- `crates/plissken-core/src/error.rs` (new)
- `crates/plissken-core/src/lib.rs` (exports)
- `crates/plissken-core/src/config.rs` (from_file)
- `crates/plissken-core/src/parser/rust.rs` (parse_file, parse_str)
- `crates/plissken-core/src/parser/python.rs` (parse_file, parse_str)
- `crates/plissken-core/src/render/renderer.rs` (new)