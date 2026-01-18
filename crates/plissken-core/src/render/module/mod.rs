//! Module documentation components
//!
//! This module contains extracted components from the monolithic ModuleRenderer,
//! providing focused, single-responsibility types for documentation generation.
//!
//! # Components
//!
//! - [`PageLayout`] - File path computation and directory structure
//! - [`CrossRefLinker`] - Cross-reference link generation between Python/Rust docs
//!
//! # Example
//!
//! ```rust
//! use plissken_core::render::module::{PageLayout, CrossRefLinker};
//!
//! let layout = PageLayout::new();
//!
//! // Get paths for Python modules
//! let index = layout.python_index_path("mypackage.submodule");
//! let item = layout.python_item_path("mypackage.submodule", "MyClass");
//!
//! // Get paths for Rust modules
//! let rust_index = layout.rust_index_path("mycrate::submod");
//!
//! // Cross-reference linking (empty linker for pure Python/Rust projects)
//! let linker = CrossRefLinker::empty();
//! ```

mod crossref;
mod layout;

pub use crossref::CrossRefLinker;
pub use layout::PageLayout;
