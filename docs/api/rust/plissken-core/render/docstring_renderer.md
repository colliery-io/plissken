# docstring_renderer <span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #ff5722; color: white;">Rust</span>


Docstring rendering for parsed documentation

This module provides rendering functionality for converting `ParsedDocstring`
structures into formatted Markdown with parameter tables, return sections,
raises tables, and code examples.

## Functions

### `fn render_docstring`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn render_docstring (doc : & ParsedDocstring) -> String
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
    let mut sections = Vec::new();

    // Summary
    if let Some(ref summary) = doc.summary {
        sections.push(summary.clone());
    }

    // Description (if different from summary)
    if let Some(ref description) = doc.description {
        sections.push(description.clone());
    }

    // Parameters table
    if !doc.params.is_empty() {
        sections.push(render_params_table(&doc.params));
    }

    // Returns section
    if let Some(ref returns) = doc.returns {
        sections.push(render_returns(returns));
    }

    // Raises table
    if !doc.raises.is_empty() {
        sections.push(render_raises_table(&doc.raises));
    }

    // Examples
    if !doc.examples.is_empty() {
        sections.push(render_examples(&doc.examples));
    }

    sections.join("\n\n")
}
```

</details>



### `fn render_params_table`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn render_params_table (params : & [ParamDoc]) -> String
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



### `fn render_returns`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn render_returns (returns : & ReturnDoc) -> String
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



### `fn render_raises_table`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn render_raises_table (raises : & [RaisesDoc]) -> String
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



### `fn render_examples`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn render_examples (examples : & [String]) -> String
```

Render examples as fenced code blocks.

Format:
```markdown
**Examples:**
```python
example code here
```
```

<details>
<summary>Source</summary>

```rust
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
```

</details>



### `fn detect_example_language`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn detect_example_language (example : & str) -> & 'static str
```

Detect the programming language of an example code block.

<details>
<summary>Source</summary>

```rust
fn detect_example_language(example: &str) -> &'static str {
    let trimmed = example.trim();

    // Python indicators
    if trimmed.starts_with(">>>")
        || trimmed.starts_with("def ")
        || trimmed.starts_with("class ")
        || trimmed.starts_with("import ")
        || trimmed.starts_with("from ")
        || trimmed.contains("print(")
    {
        return "python";
    }

    // Rust indicators
    if trimmed.starts_with("fn ")
        || trimmed.starts_with("let ")
        || trimmed.starts_with("use ")
        || trimmed.starts_with("struct ")
        || trimmed.starts_with("impl ")
        || trimmed.contains("println!")
    {
        return "rust";
    }

    // Default to python for docstrings
    "python"
}
```

</details>



### `fn escape_table_content`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn escape_table_content (content : & str) -> String
```

Escape content for use in Markdown tables.

Pipes need to be escaped and newlines replaced.

<details>
<summary>Source</summary>

```rust
fn escape_table_content(content: &str) -> String {
    content
        .replace('|', "\\|")
        .replace('\n', " ")
        .trim()
        .to_string()
}
```

</details>



