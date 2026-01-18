//! Rendering module for documentation output
//!
//! This module provides theme adapters and rendering utilities for generating
//! styled documentation that integrates with various static site generators.
//!
//! # Architecture
//!
//! The rendering system consists of two main components:
//!
//! 1. **Theme Adapters** - Map semantic color names to SSG-specific CSS variables
//! 2. **Renderer** - Tera-based template engine with theme injection
//!
//! # Example
//!
//! ```rust
//! use plissken_core::render::Renderer;
//!
//! let renderer = Renderer::new(Some("mkdocs-material"), None).unwrap();
//! let badge = renderer.badge_async().unwrap();
//! // Badge uses MkDocs Material's CSS variables
//! ```

mod crossref_renderer;
mod docstring_renderer;
pub mod module;
mod module_renderer;
mod renderer;
pub mod ssg;
mod templates;
mod theme;

pub use crossref_renderer::{CrossRefLink, Language, crossref_link, link_to_python, link_to_rust};
pub use crossref_renderer::{render_python_exposure_details, render_rust_impl_details};
pub use docstring_renderer::{
    render_docstring, render_examples, render_params_table, render_raises_table, render_returns,
};
pub use module_renderer::{ModuleRenderer, RenderedPage};
pub use renderer::Renderer;
pub use templates::TemplateLoader;
pub use theme::{MdBook, Minimal, MkDocsMaterial};
pub use theme::{ThemeAdapter, get_theme_adapter};
