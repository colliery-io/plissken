---
id: core-api-stabilization
level: initiative
title: "Core API Stabilization"
short_code: "PLSKN-I-0008"
created_at: 2026-01-16T17:49:13.566204+00:00
updated_at: 2026-01-17T01:37:59.658138+00:00
parent: PLSKN-V-0001
blocked_by: []
archived: true

tags:
  - "#initiative"
  - "#phase/completed"


exit_criteria_met: false
estimated_complexity: M
strategy_id: NULL
initiative_id: core-api-stabilization
---

# Core API Stabilization Initiative

## Context

The plissken-core library has architectural issues that affect maintainability and extensibility:

1. **Massive public API surface**: `pub use model::*` exports 130+ types including internal ones like `PyClassMeta`, `RustVariant`, `RaisesDoc`
2. **Inconsistent error handling**: Mix of `anyhow::Error`, `tera::Error`, and infallible functions that should fail
3. **No Parser trait**: Adding a new language parser requires changes everywhere
4. **Large modules**: `module_renderer.rs` is 1,916 LOC with 20+ methods
5. **Code duplication**: Parallel structures for Python/Rust (60% similar but separate types)

## Goals & Non-Goals

**Goals:**
- Create unified `PlisskenError` type for consistent error handling
- Reduce public API surface to essential types only
- Add `Parser` trait for language extensibility
- Establish clear API tiers (stable vs experimental)

**Non-Goals:**
- Breaking API changes without migration path
- Generic "any language" support (focus remains Rust+Python)
- Rewriting working code for theoretical purity

## Detailed Design

### Custom Error Type
```rust
#[derive(Debug, thiserror::Error)]
pub enum PlisskenError {
    #[error("Parse error in {file}: {message}")]
    Parse { file: PathBuf, message: String },
    
    #[error("Render error: {0}")]
    Render(#[from] tera::Error),
    
    #[error("Config error: {0}")]
    Config(String),
    
    #[error("Cross-reference error: {0}")]
    CrossRef(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
```

### API Tiers
```rust
// Core API - stable, minimal
pub use model::{DocModel, RustModule, PythonModule, CrossRef};
pub use parser::{RustParser, PythonParser};
pub use render::Renderer;

// Detail API - stable but advanced
pub mod detail {
    pub use crate::model::{RustFunction, PythonFunction, RustStruct, ...};
}

// Internal - not re-exported
// (PyClassMeta, RaisesDoc, etc.)
```

### Parser Trait
```rust
pub trait Parser: Send + Sync {
    fn parse_file(&mut self, path: &Path) -> Result<Module, PlisskenError>;
    fn parse_str(&mut self, content: &str, path: &Path) -> Result<Module, PlisskenError>;
    fn language(&self) -> Language;
}
```

## Implementation Plan

**Phase 1: Error Type (4-6 hours)**
- Create `PlisskenError` enum
- Migrate all public functions to use it
- Add context to error messages

**Phase 2: API Surface Reduction (3-4 hours)**
- Audit all `pub use` statements
- Move internal types to private modules
- Document stable vs experimental APIs

**Phase 3: Parser Trait (4-6 hours)**
- Define `Parser` trait
- Implement for RustParser and PythonParser
- Update CLI to use trait objects where appropriate