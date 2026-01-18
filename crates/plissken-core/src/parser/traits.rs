//! Parser trait and related types for language-agnostic parsing.
//!
//! This module defines the `Parser` trait which provides a common interface
//! for parsing source code from different languages.

use crate::error::Result;
use crate::model::{PythonModule, RustModule};
use std::path::Path;

/// Language identifier for parsers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParserLanguage {
    /// Rust programming language
    Rust,
    /// Python programming language
    Python,
}

impl std::fmt::Display for ParserLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserLanguage::Rust => write!(f, "Rust"),
            ParserLanguage::Python => write!(f, "Python"),
        }
    }
}

/// A parsed module, either Rust or Python.
///
/// This enum allows parsers to return a unified type while preserving
/// the specific module information for each language.
#[derive(Debug, Clone)]
pub enum Module {
    /// A parsed Rust module
    Rust(RustModule),
    /// A parsed Python module
    Python(PythonModule),
}

impl Module {
    /// Get the module path.
    pub fn path(&self) -> &str {
        match self {
            Module::Rust(m) => &m.path,
            Module::Python(m) => &m.path,
        }
    }

    /// Get the language of this module.
    pub fn language(&self) -> ParserLanguage {
        match self {
            Module::Rust(_) => ParserLanguage::Rust,
            Module::Python(_) => ParserLanguage::Python,
        }
    }

    /// Try to get as a Rust module.
    pub fn as_rust(&self) -> Option<&RustModule> {
        match self {
            Module::Rust(m) => Some(m),
            Module::Python(_) => None,
        }
    }

    /// Try to get as a Python module.
    pub fn as_python(&self) -> Option<&PythonModule> {
        match self {
            Module::Rust(_) => None,
            Module::Python(m) => Some(m),
        }
    }

    /// Convert into a Rust module, if applicable.
    pub fn into_rust(self) -> Option<RustModule> {
        match self {
            Module::Rust(m) => Some(m),
            Module::Python(_) => None,
        }
    }

    /// Convert into a Python module, if applicable.
    pub fn into_python(self) -> Option<PythonModule> {
        match self {
            Module::Rust(_) => None,
            Module::Python(m) => Some(m),
        }
    }
}

impl From<RustModule> for Module {
    fn from(m: RustModule) -> Self {
        Module::Rust(m)
    }
}

impl From<PythonModule> for Module {
    fn from(m: PythonModule) -> Self {
        Module::Python(m)
    }
}

/// A language-specific documentation parser.
///
/// This trait provides a unified interface for parsing source code from
/// different programming languages. Implementations handle the language-specific
/// parsing logic while exposing a common API.
///
/// # Example
///
/// ```ignore
/// use plissken_core::parser::{Parser, ParserLanguage, create_parser};
///
/// // Create a parser dynamically
/// let mut parser = create_parser(ParserLanguage::Rust);
/// let module = parser.parse_file(Path::new("src/lib.rs"))?;
///
/// // Or use concrete types directly
/// let mut rust_parser = RustParser::new();
/// let rust_module = rust_parser.parse_file(Path::new("src/lib.rs"))?;
/// ```
pub trait Parser: Send {
    /// Parse a source file and return its module representation.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the source file to parse
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or contains invalid syntax.
    fn parse_file(&mut self, path: &Path) -> Result<Module>;

    /// Parse source code from a string.
    ///
    /// This is useful for testing or when source code is available in memory.
    ///
    /// # Arguments
    ///
    /// * `content` - The source code to parse
    /// * `virtual_path` - A path to use for error messages and source locations
    ///
    /// # Errors
    ///
    /// Returns an error if the source contains invalid syntax.
    fn parse_str(&mut self, content: &str, virtual_path: &Path) -> Result<Module>;

    /// The language this parser handles.
    fn language(&self) -> ParserLanguage;

    /// Human-readable name for error messages.
    fn name(&self) -> &'static str;

    /// File extensions this parser handles.
    fn extensions(&self) -> &'static [&'static str];

    /// Check if this parser can handle the given file extension.
    fn can_parse_extension(&self, ext: &str) -> bool {
        self.extensions()
            .iter()
            .any(|e| e.eq_ignore_ascii_case(ext))
    }
}

/// Create a parser for the given language.
///
/// # Example
///
/// ```ignore
/// use plissken_core::parser::{create_parser, ParserLanguage};
///
/// let mut parser = create_parser(ParserLanguage::Python);
/// let module = parser.parse_file(Path::new("module.py"))?;
/// ```
pub fn create_parser(language: ParserLanguage) -> Box<dyn Parser> {
    match language {
        ParserLanguage::Rust => Box::new(super::RustParser::new()),
        ParserLanguage::Python => Box::new(super::PythonParser::new()),
    }
}

/// Get a parser for the given file extension.
///
/// Returns `None` if the extension is not recognized.
///
/// # Example
///
/// ```ignore
/// use plissken_core::parser::parser_for_extension;
///
/// if let Some(mut parser) = parser_for_extension("py") {
///     let module = parser.parse_file(Path::new("module.py"))?;
/// }
/// ```
pub fn parser_for_extension(ext: &str) -> Option<Box<dyn Parser>> {
    match ext.to_lowercase().as_str() {
        "rs" => Some(Box::new(super::RustParser::new())),
        "py" | "pyi" => Some(Box::new(super::PythonParser::new())),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_language_display() {
        assert_eq!(ParserLanguage::Rust.to_string(), "Rust");
        assert_eq!(ParserLanguage::Python.to_string(), "Python");
    }

    #[test]
    fn test_module_conversions() {
        let rust_module = RustModule::test("crate::test");
        let module: Module = rust_module.into();

        assert!(matches!(module.language(), ParserLanguage::Rust));
        assert!(module.as_rust().is_some());
        assert!(module.as_python().is_none());
    }

    #[test]
    fn test_create_parser() {
        let rust_parser = create_parser(ParserLanguage::Rust);
        assert_eq!(rust_parser.language(), ParserLanguage::Rust);
        assert_eq!(rust_parser.name(), "Rust");
        assert!(rust_parser.extensions().contains(&"rs"));

        let python_parser = create_parser(ParserLanguage::Python);
        assert_eq!(python_parser.language(), ParserLanguage::Python);
        assert_eq!(python_parser.name(), "Python");
        assert!(python_parser.extensions().contains(&"py"));
    }

    #[test]
    fn test_parser_for_extension() {
        assert!(parser_for_extension("rs").is_some());
        assert!(parser_for_extension("RS").is_some()); // case insensitive
        assert!(parser_for_extension("py").is_some());
        assert!(parser_for_extension("pyi").is_some());
        assert!(parser_for_extension("js").is_none());
        assert!(parser_for_extension("").is_none());
    }

    #[test]
    fn test_can_parse_extension() {
        let parser = create_parser(ParserLanguage::Rust);
        assert!(parser.can_parse_extension("rs"));
        assert!(parser.can_parse_extension("RS"));
        assert!(!parser.can_parse_extension("py"));
    }
}
