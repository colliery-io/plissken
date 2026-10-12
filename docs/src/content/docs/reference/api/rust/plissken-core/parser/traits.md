---
title: "plissken-core::parser::traits"
description: "Parser trait and related types for language-agnostic parsing."
editUrl: false
sidebar:
  order: 15
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>

Parser trait and related types for language-agnostic parsing.

This module defines the `Parser` trait which provides a common interface
for parsing source code from different languages.

## Enums

### <span id="enum.ParserLanguage"></span><code style="font-size: 0.85em">enum ParserLanguage</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

Language identifier for parsers.

#### Variants

- **`Rust`** - Rust programming language
- **`Python`** - Python programming language

### <span id="enum.Module"></span><code style="font-size: 0.85em">enum Module</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

A parsed module, either Rust or Python.

This enum allows parsers to return a unified type while preserving
the specific module information for each language.

#### Variants

- **`Rust`** - A parsed Rust module
- **`Python`** - A parsed Python module

#### Methods

##### <span id="Module.path"></span><code>path</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn path(&self) -> &str
```

Get the module path.

<details>
<summary>Source</summary>

```rust
    pub fn path(&self) -> &str {
        match self {
            Module::Rust(m) => &m.path,
            Module::Python(m) => &m.path,
        }
    }
```

</details>

##### <span id="Module.language"></span><code>language</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn language(&self) -> ParserLanguage
```

Get the language of this module.

<details>
<summary>Source</summary>

```rust
    pub fn language(&self) -> ParserLanguage {
        match self {
            Module::Rust(_) => ParserLanguage::Rust,
            Module::Python(_) => ParserLanguage::Python,
        }
    }
```

</details>

##### <span id="Module.as_rust"></span><code>as_​rust</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn as_rust(&self) -> Option<&RustModule>
```

Try to get as a Rust module.

<details>
<summary>Source</summary>

```rust
    pub fn as_rust(&self) -> Option<&RustModule> {
        match self {
            Module::Rust(m) => Some(m),
            Module::Python(_) => None,
        }
    }
```

</details>

##### <span id="Module.as_python"></span><code>as_​python</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn as_python(&self) -> Option<&PythonModule>
```

Try to get as a Python module.

<details>
<summary>Source</summary>

```rust
    pub fn as_python(&self) -> Option<&PythonModule> {
        match self {
            Module::Rust(_) => None,
            Module::Python(m) => Some(m),
        }
    }
```

</details>

##### <span id="Module.into_rust"></span><code>into_​rust</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn into_rust(self) -> Option<RustModule>
```

Convert into a Rust module, if applicable.

<details>
<summary>Source</summary>

```rust
    pub fn into_rust(self) -> Option<RustModule> {
        match self {
            Module::Rust(m) => Some(m),
            Module::Python(_) => None,
        }
    }
```

</details>

##### <span id="Module.into_python"></span><code>into_​python</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn into_python(self) -> Option<PythonModule>
```

Convert into a Python module, if applicable.

<details>
<summary>Source</summary>

```rust
    pub fn into_python(self) -> Option<PythonModule> {
        match self {
            Module::Rust(_) => None,
            Module::Python(m) => Some(m),
        }
    }
```

</details>

## Traits

### <span id="trait.Parser"></span><code style="font-size: 0.85em">trait Parser</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Supertraits:** `Send`

A language-specific documentation parser.

This trait provides a unified interface for parsing source code from
different programming languages. Implementations handle the language-specific
parsing logic while exposing a common API.

**Examples:**

```rust
use plissken_core::parser::{Parser, ParserLanguage, create_parser};

// Create a parser dynamically
let mut parser = create_parser(ParserLanguage::Rust);
let module = parser.parse_file(Path::new("src/lib.rs"))?;

// Or use concrete types directly
let mut rust_parser = RustParser::new();
let rust_module = rust_parser.parse_file(Path::new("src/lib.rs"))?;
```

#### Methods

##### <span id="Parser.parse_file"></span><code>parse_​file</code>

```rust
fn parse_file(&mut self, path: &Path) -> Result<Module>
```

Parse a source file and return its module representation.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `path` | `&Path` | Path to the source file to parse |

**Errors:**

Returns an error if the file cannot be read or contains invalid syntax.

<details>
<summary>Source</summary>

```rust
    fn parse_file(&mut self, path: &Path) -> Result<Module>;
```

</details>

##### <span id="Parser.parse_str"></span><code>parse_​str</code>

```rust
fn parse_str(
    &mut self,
    content: &str,
    virtual_path: &Path,
) -> Result<Module>
```

Parse source code from a string.

This is useful for testing or when source code is available in memory.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `content` | `&str` | The source code to parse |
| `virtual_path` | `&Path` | A path to use for error messages and source locations |

**Errors:**

Returns an error if the source contains invalid syntax.

<details>
<summary>Source</summary>

```rust
    fn parse_str(&mut self, content: &str, virtual_path: &Path) -> Result<Module>;
```

</details>

##### <span id="Parser.language"></span><code>language</code>

```rust
fn language(&self) -> ParserLanguage
```

The language this parser handles.

<details>
<summary>Source</summary>

```rust
    fn language(&self) -> ParserLanguage;
```

</details>

##### <span id="Parser.name"></span><code>name</code>

```rust
fn name(&self) -> &'static str
```

Human-readable name for error messages.

<details>
<summary>Source</summary>

```rust
    fn name(&self) -> &'static str;
```

</details>

##### <span id="Parser.extensions"></span><code>extensions</code>

```rust
fn extensions(&self) -> &'static [&'static str]
```

File extensions this parser handles.

<details>
<summary>Source</summary>

```rust
    fn extensions(&self) -> &'static [&'static str];
```

</details>

##### <span id="Parser.can_parse_extension"></span><code>can_​parse_​extension</code>

```rust
fn can_parse_extension(&self, ext: &str) -> bool
```

Check if this parser can handle the given file extension.

<details>
<summary>Source</summary>

```rust
    fn can_parse_extension(&self, ext: &str) -> bool {
        self.extensions()
            .iter()
            .any(|e| e.eq_ignore_ascii_case(ext))
    }
```

</details>

## Functions

### <span id="fn.create_parser"></span><code style="font-size: 0.85em">fn create_​parser</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn create_parser(language: ParserLanguage) -> Box<dyn Parser>
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

### <span id="fn.parser_for_extension"></span><code style="font-size: 0.85em">fn parser_​for_​extension</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn parser_for_extension(ext: &str) -> Option<Box<dyn Parser>>
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
