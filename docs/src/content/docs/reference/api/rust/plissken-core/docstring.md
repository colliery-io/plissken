---
title: "plissken-core::docstring"
description: "Docstring parser for Google, NumPy, and Rust doc comment styles"
editUrl: false
sidebar:
  order: 6
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>

Docstring parser for Google, NumPy, and Rust doc comment styles

This module parses docstrings into structured `ParsedDocstring` objects,
extracting summary, parameters, returns, raises, and examples.

Supported formats:
- **Google style**: `Args:`, `Returns:`, `Raises:`, `Example:`
- **NumPy style**: Underlined section headers
- **Rust style**: `# Arguments`, `# Returns`, `# Errors`, `# Panics`, `# Examples`

## Functions

### <span id="fn.parse_docstring"></span><code style="font-size: 0.85em">fn parse_​docstring</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn parse_docstring(docstring: &str) -> ParsedDocstring
```

Parse a docstring into structured form

<details>
<summary>Source</summary>

```rust
pub fn parse_docstring(docstring: &str) -> ParsedDocstring {
    let docstring = docstring.trim();
    if docstring.is_empty() {
        return ParsedDocstring::empty();
    }

    // Detect style based on section format
    let style = detect_style(docstring);

    match style {
        DocstringStyle::Google => parse_google_style(docstring),
        DocstringStyle::NumPy => parse_numpy_style(docstring),
        DocstringStyle::Plain => parse_plain(docstring),
    }
}
```

</details>

### <span id="fn.parse_rust_doc"></span><code style="font-size: 0.85em">fn parse_​rust_​doc</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn parse_rust_doc(doc: &str) -> ParsedDocstring
```

Parse a Rust doc comment into structured form

Looks for conventional markdown sections:
- `# Arguments` / `# Parameters` - function parameters
- `# Returns` - return value documentation
- `# Errors` - error conditions (raises of kind [`RUST_ERROR_KIND`](../../../rust/plissken-core/docstring/#const.RUST_ERROR_KIND))
- `# Panics` - panic conditions (raises of kind [`RUST_PANIC_KIND`](../../../rust/plissken-core/docstring/#const.RUST_PANIC_KIND))
- `# Examples` - code examples

Other sections (`# Safety`, `# Overview`, ...) stay in the description,
with the heading as a bold line. The description keeps its Markdown:
paragraphs, lists and code blocks. Code blocks hold Rust unless they name
another language, as in rustdoc: they get the language `rust`, and the
hidden lines of a doc test (`# use foo;`) are left out.

<details>
<summary>Source</summary>

```rust
pub fn parse_rust_doc(doc: &str) -> ParsedDocstring {
    let doc = normalize_rust_doc_code(doc);
    let doc = doc.trim_matches(|c: char| c == '\n' || c == '\r');
    if doc.trim().is_empty() {
        return ParsedDocstring::empty();
    }

    let lines: Vec<&str> = doc.lines().collect();

    let mut text: Vec<String> = Vec::new();
    let mut params = Vec::new();
    let mut returns = None;
    let mut raises = Vec::new();
    let mut examples = Vec::new();

    for (header, body) in split_rust_sections(&lines) {
        let Some(header) = header else {
            text.extend(body.iter().map(|l| l.to_string()));
            continue;
        };
        match header.to_lowercase().as_str() {
            "arguments" | "parameters" | "args" | "params" => {
                params = parse_rust_arguments(body);
            }
            "returns" | "return" => {
                returns = parse_rust_returns(body);
            }
            "errors" | "error" => {
                raises.extend(parse_rust_errors(body, RUST_ERROR_KIND));
            }
            "panics" | "panic" => {
                raises.extend(parse_rust_errors(body, RUST_PANIC_KIND));
            }
            "examples" | "example" => {
                examples = parse_rust_examples(body);
            }
            _ => {
                // Any other section stays in the text, its heading in bold
                text.push(String::new());
                text.push(format!("**{}**", header));
                text.push(String::new());
                text.extend(body.iter().map(|l| l.to_string()));
            }
        }
    }

    let (summary, description) = split_rust_summary(&text);

    ParsedDocstring {
        summary,
        description,
        params,
        returns,
        raises,
        examples,
    }
}
```

</details>

### <span id="fn.normalize_rust_doc"></span><code style="font-size: 0.85em">fn normalize_​rust_​doc</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn normalize_rust_doc(doc: &str) -> String
```

Rust doc text made ready for Markdown output.

Code blocks get the language `rust` (see [`parse_rust_doc`](../../../rust/plissken-core/docstring/#fn.parse_rust_doc)), and
runs of blank lines become one. For doc comments that are not parsed into
sections: fields, variants.

<details>
<summary>Source</summary>

```rust
pub fn normalize_rust_doc(doc: &str) -> String {
    let doc = normalize_rust_doc_code(doc);
    let lines: Vec<&str> = doc.lines().collect();
    tidy_markdown_lines(&lines).join("\n")
}
```

</details>

## Constants

| Name | Type | Value | Description |
|------|------|-------|-------------|
| <span id="const.RUST_ERROR_KIND"></span>`RUST_ERROR_KIND` | `&str` | `"Error"` | The `RaisesDoc` kind of an entry of a Rust `# Errors` section. |
| <span id="const.RUST_PANIC_KIND"></span>`RUST_PANIC_KIND` | `&str` | `"Panic"` | The `RaisesDoc` kind of an entry of a Rust `# Panics` section. |
