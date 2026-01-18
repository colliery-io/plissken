//! Parsing infrastructure for Rust and Python source code
//!
//! This module provides parsers for extracting documentation from source code.
//! The [`Parser`] trait provides a language-agnostic interface, while
//! [`RustParser`] and [`PythonParser`] provide concrete implementations.

pub mod python;
pub mod rust;
mod traits;

// Concrete parsers
pub use python::PythonParser;
pub use rust::RustParser;

// Trait and related types
pub use traits::{Module, Parser, ParserLanguage, create_parser, parser_for_extension};
