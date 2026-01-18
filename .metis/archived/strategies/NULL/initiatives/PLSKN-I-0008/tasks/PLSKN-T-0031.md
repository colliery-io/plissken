---
id: add-parser-trait-for-language
level: task
title: "Add Parser trait for language extensibility"
short_code: "PLSKN-T-0031"
created_at: 2026-01-16T18:00:11.659186+00:00
updated_at: 2026-01-17T01:37:45.397090+00:00
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

# Add Parser trait for language extensibility

## Parent Initiative

[[PLSKN-I-0008]] Core API Stabilization

## Objective

Define a `Parser` trait that abstracts language-specific parsing. This enables future language support, simplifies testing with mock parsers, and establishes a consistent parsing interface.

## Current Problem

Rust and Python parsers are separate concrete types with no shared interface:
- `RustParser::parse_crate(path)` vs `parse_python_module(path, name)`
- Different method signatures and return types
- Adding a new language requires changes across the codebase
- Can't easily swap parsers for testing

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [x] Define `Parser` trait with common interface
- [x] Implement `Parser` for `RustParser`
- [x] Implement `Parser` for `PythonParser`
- [x] Update CLI to use `Box<dyn Parser>` where appropriate (via `create_parser()` and `parser_for_extension()`)
- [x] Add `language()` method to identify parser type
- [x] Support both file and string parsing
- [x] Maintain backwards compatibility with existing concrete types

## Trait Design

```rust
use std::path::Path;

/// A language-specific documentation parser.
pub trait Parser: Send + Sync {
    /// Parse a single file and return its module representation.
    fn parse_file(&mut self, path: &Path) -> Result<Module>;
    
    /// Parse source code from a string (useful for testing).
    fn parse_str(&mut self, content: &str, virtual_path: &Path) -> Result<Module>;
    
    /// The language this parser handles.
    fn language(&self) -> Language;
    
    /// Human-readable name for error messages.
    fn name(&self) -> &'static str;
    
    /// File extensions this parser handles.
    fn extensions(&self) -> &[&str];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Rust,
    Python,
}
```

## Implementation Notes

### Files to Create/Modify

1. **`crates/plissken-core/src/parser/mod.rs`** (new) - Trait definition:
   ```rust
   mod rust_parser;
   mod python_parser;
   mod traits;
   
   pub use traits::{Parser, Language};
   pub use rust_parser::RustParser;
   pub use python_parser::PythonParser;
   ```

2. **`crates/plissken-core/src/parser/traits.rs`** (new) - Trait and Language enum

3. **`crates/plissken-core/src/parser/rust_parser.rs`** - Implement trait:
   ```rust
   impl Parser for RustParser {
       fn parse_file(&mut self, path: &Path) -> Result<Module> {
           // Existing logic
       }
       
       fn parse_str(&mut self, content: &str, virtual_path: &Path) -> Result<Module> {
           // Parse from string for testing
       }
       
       fn language(&self) -> Language {
           Language::Rust
       }
       
       fn name(&self) -> &'static str {
           "Rust"
       }
       
       fn extensions(&self) -> &[&str] {
           &["rs"]
       }
   }
   ```

4. **`crates/plissken-core/src/parser/python_parser.rs`** - Same pattern

### Parser Factory

```rust
/// Create a parser for the given language.
pub fn create_parser(language: Language) -> Box<dyn Parser> {
    match language {
        Language::Rust => Box::new(RustParser::new()),
        Language::Python => Box::new(PythonParser::new()),
    }
}

/// Detect parser from file extension.
pub fn parser_for_extension(ext: &str) -> Option<Box<dyn Parser>> {
    match ext {
        "rs" => Some(Box::new(RustParser::new())),
        "py" => Some(Box::new(PythonParser::new())),
        _ => None,
    }
}
```

### Testing Benefits

```rust
#[cfg(test)]
mod tests {
    struct MockParser {
        modules: Vec<Module>,
    }
    
    impl Parser for MockParser {
        fn parse_file(&mut self, _path: &Path) -> Result<Module> {
            Ok(self.modules.pop().unwrap())
        }
        // ...
    }
    
    #[test]
    fn test_with_mock_parser() {
        let parser = MockParser { modules: vec![test_module()] };
        let result = render_with_parser(Box::new(parser));
        // ...
    }
}
```

## Non-Goals

- Generic "plugin" system for arbitrary languages
- Runtime language detection from file content
- Supporting languages beyond Rust and Python (for now)

## Status Updates

### 2026-01-17: Implementation Complete

**Files created/modified:**
- `crates/plissken-core/src/parser/traits.rs` (new) - Parser trait, ParserLanguage enum, Module enum, factory functions
- `crates/plissken-core/src/parser/mod.rs` - Updated exports
- `crates/plissken-core/src/parser/rust.rs` - Added Parser trait impl
- `crates/plissken-core/src/parser/python.rs` - Added Parser trait impl
- `crates/plissken-core/src/lib.rs` - Added new exports

**Implementation details:**
- `ParserLanguage` enum: `Rust`, `Python` with Display impl
- `Module` enum wrapping `RustModule` and `PythonModule` with helper methods
- `Parser` trait with: `parse_file`, `parse_str`, `language`, `name`, `extensions`, `can_parse_extension`
- Factory functions: `create_parser(ParserLanguage)`, `parser_for_extension(&str)`
- Exported as `ParsedModule` in lib.rs to avoid potential naming conflicts
- All 177 tests passing