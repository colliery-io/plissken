//! plissken-core: Documentation extraction for Rust-Python hybrid projects
//!
//! This crate provides the core functionality for parsing Rust and Python
//! source code and extracting documentation into a unified model.
//!
//! # API Structure
//!
//! The public API is organized into tiers:
//!
//! - **Core API** (root): Essential types for typical usage
//! - **Detail API** ([`detail`]): Advanced types for fine-grained control
//! - **Modules** (`config`, `parser`, `render`, etc.): Full module access
//!
//! # Quick Start
//!
//! ```ignore
//! use plissken_core::{Config, RustParser, PythonParser, DocModel};
//!
//! // Load configuration
//! let config = Config::from_file("plissken.toml")?;
//!
//! // Parse source files
//! let mut rust_parser = RustParser::new();
//! let rust_module = rust_parser.parse_file("src/lib.rs")?;
//!
//! // Build documentation model
//! let model = DocModel::test("my-project")
//!     .with_rust_module(rust_module);
//! ```

// =============================================================================
// Module declarations
// =============================================================================

pub mod config;

// Test utilities - only compiled for tests
#[cfg(test)]
pub mod test_fixtures;
pub mod crossref;
pub mod discover;
pub mod docstring;
pub mod error;
pub mod manifest;
pub mod model;
pub mod parser;
pub mod render;

// =============================================================================
// Core API - Essential types for typical usage (~20 types)
// =============================================================================

// Configuration
pub use config::{Config, ConfigError, ConfigWarning, ValidationResult};
// Configuration constants
pub use config::{
    DEFAULT_OUTPUT_FORMAT, DEFAULT_OUTPUT_PATH, DEFAULT_DOCS_RS_URL, DEFAULT_TEMPLATE,
    VERSION_SOURCE_CARGO, VERSION_SOURCE_PYPROJECT,
    CARGO_MANIFEST, PYPROJECT_MANIFEST, PLISSKEN_CONFIG,
    TEMPLATE_MKDOCS_MATERIAL, TEMPLATE_MDBOOK, DEFAULT_CRATES,
};

// Errors
pub use error::{PlisskenError, Result};

// Documentation model - top level
pub use model::{DocModel, ProjectMetadata, CrossRef, CrossRefKind, SourceType};

// Module containers
pub use model::{RustModule, PythonModule};

// Parsers
pub use parser::{RustParser, PythonParser};
pub use parser::{Parser, ParserLanguage, Module as ParsedModule, create_parser, parser_for_extension};

// Rendering
pub use render::{ModuleRenderer, RenderedPage, Renderer};
pub use render::{ThemeAdapter, get_theme_adapter};

// =============================================================================
// Detail API - Advanced types for users who need fine-grained control
// =============================================================================

/// Advanced types for users who need fine-grained control over documentation.
///
/// These types are stable but not commonly needed for basic usage. Import from
/// here when you need to inspect or construct individual documentation items.
///
/// # Example
///
/// ```ignore
/// use plissken_core::detail::{RustStruct, PythonFunction, Visibility};
///
/// // Inspect a struct's fields
/// if let Some(struct_) = get_rust_struct(model) {
///     for field in &struct_.fields {
///         println!("{}: {}", field.name, field.ty);
///     }
/// }
/// ```
pub mod detail {
    // Rust item types
    pub use crate::model::{
        RustItem, RustStruct, RustEnum, RustFunction, RustTrait,
        RustImpl, RustConst, RustTypeAlias,
    };

    // Rust sub-types
    pub use crate::model::{
        RustField, RustFunctionSig, RustParam, RustVariant, RustAssociatedType,
        Visibility,
    };

    // Python item types
    pub use crate::model::{
        PythonItem, PythonClass, PythonFunction, PythonVariable,
    };

    // Python sub-types
    pub use crate::model::{
        PythonFunctionSig, PythonParam,
    };

    // Source location types
    pub use crate::model::{SourceSpan, SourceLocation};

    // Cross-reference types
    pub use crate::model::RustItemRef;

    // Docstring types
    pub use crate::model::{ParsedDocstring, ParamDoc, ReturnDoc, RaisesDoc};

    // PyO3 metadata (useful for cross-reference inspection)
    pub use crate::model::{PyClassMeta, PyFunctionMeta};

    // Discovery
    pub use crate::discover::{DiscoveredModule, discover_python_modules, merge_modules};

    // Manifest parsing
    pub use crate::manifest::{CargoManifest, InferredConfig, PyProjectManifest};

    // Cross-reference building
    pub use crate::crossref::{
        build_cross_refs, synthesize_python_from_rust, synthesize_python_modules_from_rust,
    };

    // Docstring parsing
    pub use crate::docstring::{parse_docstring, parse_rust_doc};

    // Render utilities
    pub use crate::render::{
        CrossRefLink, Language, crossref_link, link_to_python, link_to_rust,
        render_docstring, render_examples, render_params_table,
        render_raises_table, render_returns,
        render_python_exposure_details, render_rust_impl_details,
    };
}

// =============================================================================
// Backwards compatibility re-exports (deprecated, use detail:: instead)
// =============================================================================

// These are provided for backwards compatibility but should migrate to detail::
// TODO: Add #[deprecated] once stabilized migration period ends

// Commonly used detail types that were previously in root
pub use model::{
    RustItem, RustStruct, RustEnum, RustFunction, RustTrait, RustImpl,
    PythonItem, PythonClass, PythonFunction, PythonVariable,
    RustField, RustParam, PythonParam, Visibility,
    SourceSpan, SourceLocation, RustItemRef,
    ParsedDocstring,
};

// Discovery and manifest (commonly used)
pub use discover::{DiscoveredModule, discover_python_modules, merge_modules};
pub use manifest::{CargoManifest, InferredConfig, PyProjectManifest};

// Cross-reference utilities
pub use crossref::{build_cross_refs, synthesize_python_from_rust, synthesize_python_modules_from_rust};

// Docstring utilities
pub use docstring::{parse_docstring, parse_rust_doc};

// Render utilities
pub use render::{CrossRefLink, Language, crossref_link, link_to_python, link_to_rust};
pub use render::{
    render_docstring, render_examples, render_params_table, render_raises_table, render_returns,
    render_python_exposure_details, render_rust_impl_details,
};
