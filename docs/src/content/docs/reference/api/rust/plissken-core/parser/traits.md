---
title: "plissken-core::parser::traits"
description: "Parser trait and related types for language-agnostic parsing."
sidebar:
  order: 13
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>


Parser trait and related types for language-agnostic parsing.

This module defines the `Parser` trait which provides a common interface
for parsing source code from different languages.

## Enums

### <span id="enum.ParserLanguage"></span>`enum ParserLanguage`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


Language identifier for parsers.

#### Variants

- **`Rust`** - Rust programming language
- **`Python`** - Python programming language



### <span id="enum.Module"></span>`enum Module`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


A parsed module, either Rust or Python.

This enum allows parsers to return a unified type while preserving
the specific module information for each language.

#### Variants

- **`Rust`** - A parsed Rust module
- **`Python`** - A parsed Python module



## Functions

### <span id="fn.create_parser"></span>`fn create_parser`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


```rust
fn create_parser (language : ParserLanguage) -> Box < dyn Parser >
```

Create a parser for the given language.

**Examples:**

```rust
use plissken_core::parser::{create_parser, ParserLanguage};

let mut parser = create_parser(ParserLanguage::Python);
let module = parser.parse_file(Path::new("module.py"))?;
```

<details>
<summary>Source</summary>

```rust
pub fn create_parser(language: ParserLanguage) -> Box<dyn Parser> {
    match language {
        ParserLanguage::Rust => Box::new(super::RustParser::new()),
        ParserLanguage::Python => Box::new(super::PythonParser::new()),
    }
}
```

</details>



### <span id="fn.parser_for_extension"></span>`fn parser_for_extension`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


```rust
fn parser_for_extension (ext : & str) -> Option < Box < dyn Parser > >
```

Get a parser for the given file extension.

Returns `None` if the extension is not recognized.

**Examples:**

```rust
use plissken_core::parser::parser_for_extension;

if let Some(mut parser) = parser_for_extension("py") {
    let module = parser.parse_file(Path::new("module.py"))?;
}
```

<details>
<summary>Source</summary>

```rust
pub fn parser_for_extension(ext: &str) -> Option<Box<dyn Parser>> {
    match ext.to_lowercase().as_str() {
        "rs" => Some(Box::new(super::RustParser::new())),
        "py" | "pyi" => Some(Box::new(super::PythonParser::new())),
        _ => None,
    }
}
```

</details>



