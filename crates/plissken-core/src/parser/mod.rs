//! Parsing infrastructure for Rust and Python source code
//!
//! This module provides parsers for extracting documentation from source code.
//! The [`Parser`] trait provides a language-agnostic interface, while
//! [`RustParser`] and [`PythonParser`] provide concrete implementations.

mod traits;
pub mod python;
pub mod rust;

// Concrete parsers
pub use python::PythonParser;
pub use rust::RustParser;

// Trait and related types
pub use traits::{create_parser, parser_for_extension, Module, Parser, ParserLanguage};
