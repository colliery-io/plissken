---
title: "plissken-core::render::docstring_renderer"
description: "Docstring rendering for parsed documentation"
editUrl: false
sidebar:
  order: 19
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>

Docstring rendering for parsed documentation

This module provides rendering functionality for converting `ParsedDocstring`
structures into formatted Markdown with parameter tables, return sections,
raises tables, and code examples.

## Functions

### <span id="fn.render_docstring"></span><code style="font-size: 0.85em">fn render_​docstring</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn render_docstring(doc: &ParsedDocstring) -> String
```

Render a parsed docstring to Markdown.

Produces formatted Markdown with:
- Summary and description paragraphs
- Parameters table (Name | Type | Description)
- Returns section with type and description
- Raises table (Exception | Description)
- Examples as fenced code blocks

Empty sections are omitted entirely.

**Examples:**

```rust
use plissken_core::model::{ParsedDocstring, ParamDoc, ReturnDoc};
use plissken_core::render::render_docstring;

let doc = ParsedDocstring {
    summary: Some("Calculate the sum of two numbers.".to_string()),
    description: None,
    params: vec![
        ParamDoc { name: "a".to_string(), ty: Some("int".to_string()), description: "First number".to_string() },
        ParamDoc { name: "b".to_string(), ty: Some("int".to_string()), description: "Second number".to_string() },
    ],
    returns: Some(ReturnDoc { ty: Some("int".to_string()), description: "The sum".to_string() }),
    raises: vec![],
    examples: vec![],
};

let output = render_docstring(&doc);
assert!(output.contains("Calculate the sum"));
assert!(output.contains("| `a` | `int` |"));
```

<details>
<summary>Source</summary>

```rust
pub fn render_docstring(doc: &ParsedDocstring) -> String {
    render_docstring_as(doc, DocLanguage::Python)
}
```

</details>

### <span id="fn.render_rust_docstring"></span><code style="font-size: 0.85em">fn render_​rust_​docstring</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn render_rust_docstring(doc: &ParsedDocstring) -> String
```

Render a parsed Rust doc comment to Markdown.

As [`render_docstring`](../../../../rust/plissken-core/render/docstring_renderer/#fn.render_docstring), but in the words of Rust: an `# Errors` section
is shown as "Errors", a `# Panics` section as "Panics", each as text or a
list (not a table of exception types), and the examples as the Markdown
of the doc comment.

<details>
<summary>Source</summary>

```rust
pub fn render_rust_docstring(doc: &ParsedDocstring) -> String {
    render_docstring_as(doc, DocLanguage::Rust)
}
```

</details>

### <span id="fn.render_params_table"></span><code style="font-size: 0.85em">fn render_​params_​table</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn render_params_table(params: &[ParamDoc]) -> String
```

Render parameters as a Markdown table.

Format:
```markdown
**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `param` | `type` | Description text |
```

<details>
<summary>Source</summary>

```rust
pub fn render_params_table(params: &[ParamDoc]) -> String {
    let mut output = String::from("**Parameters:**\n\n");
    output.push_str("| Name | Type | Description |\n");
    output.push_str("|------|------|-------------|\n");

    for param in params {
        let ty = param.ty.as_deref().unwrap_or("-");
        // Escape pipe characters in description for table compatibility
        let desc = escape_table_content(&param.description);
        output.push_str(&format!("| `{}` | `{}` | {} |\n", param.name, ty, desc));
    }

    output
}
```

</details>

### <span id="fn.render_returns"></span><code style="font-size: 0.85em">fn render_​returns</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn render_returns(returns: &ReturnDoc) -> String
```

Render returns section.

Format:
```markdown
**Returns:** `type`

Description text
```

<details>
<summary>Source</summary>

```rust
pub fn render_returns(returns: &ReturnDoc) -> String {
    let mut output = String::from("**Returns:**");

    if let Some(ref ty) = returns.ty {
        output.push_str(&format!(" `{}`", ty));
    }

    output.push_str("\n\n");
    output.push_str(&returns.description);

    output
}
```

</details>

### <span id="fn.render_raises_table"></span><code style="font-size: 0.85em">fn render_​raises_​table</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn render_raises_table(raises: &[RaisesDoc]) -> String
```

Render raises/exceptions as a Markdown table.

Format:
```markdown
**Raises:**

| Exception | Description |
|-----------|-------------|
| `ValueError` | When value is invalid |
```

<details>
<summary>Source</summary>

```rust
pub fn render_raises_table(raises: &[RaisesDoc]) -> String {
    let mut output = String::from("**Raises:**\n\n");
    output.push_str("| Exception | Description |\n");
    output.push_str("|-----------|-------------|\n");

    for raise in raises {
        let desc = escape_table_content(&raise.description);
        output.push_str(&format!("| `{}` | {} |\n", raise.ty, desc));
    }

    output
}
```

</details>

### <span id="fn.render_examples"></span><code style="font-size: 0.85em">fn render_​examples</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn render_examples(examples: &[String]) -> String
```

Render examples as fenced code blocks.

Format:
```markdown
**Examples:**

```python
example code here
```
```rust

<details>
<summary>Source</summary>

````rust
pub fn render_examples(examples: &[String]) -> String {
    let mut output = String::from("**Examples:**\n\n");

    for example in examples {
        let trimmed = example.trim();

        // If example already has code fences, include directly
        if trimmed.starts_with("```") {
            output.push_str(trimmed);
            output.push_str("\n\n");
        } else {
            // Wrap in code fence with detected language
            let lang = detect_example_language(example);
            output.push_str(&format!("```{}\n{}\n```\n\n", lang, trimmed));
        }
    }

    output.trim_end().to_string()
}
````

</details>

### <span id="fn.escape_table_content"></span><code style="font-size: 0.85em">fn escape_​table_​content</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn escape_table_content(content: &str) -> String
```

Escape content for use in Markdown tables.

Pipes are escaped. A table row is one line: the lines of a paragraph are
joined with a space, and paragraphs with a line break (`<br>`).

<details>
<summary>Source</summary>

```rust
pub fn escape_table_content(content: &str) -> String {
    let paragraphs: Vec<String> = content
        .split("\n\n")
        .map(|p| {
            p.lines()
                .map(str::trim)
                .filter(|l| !l.is_empty())
                .collect::<Vec<_>>()
                .join(" ")
        })
        .filter(|p| !p.is_empty())
        .collect();
    paragraphs.join("<br>").replace('|', "\\|")
}
```

</details>
