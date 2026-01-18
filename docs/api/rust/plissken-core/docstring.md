# docstring <span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #ff5722; color: white;">Rust</span>


Docstring parser for Google, NumPy, and Rust doc comment styles

This module parses docstrings into structured `ParsedDocstring` objects,
extracting summary, parameters, returns, raises, and examples.
Supported formats:
- **Google style**: `Args:`, `Returns:`, `Raises:`, `Example:`
- **NumPy style**: Underlined section headers
- **Rust style**: `# Arguments`, `# Returns`, `# Errors`, `# Panics`, `# Examples`

## Enums

### `enum DocstringStyle` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


#### Variants

- **`Google`**
- **`NumPy`**
- **`Plain`**



## Functions

### `fn parse_docstring`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn parse_docstring (docstring : & str) -> ParsedDocstring
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



### `fn detect_style`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn detect_style (docstring : & str) -> DocstringStyle
```

Detect the docstring style based on section markers

<details>
<summary>Source</summary>

```rust
fn detect_style(docstring: &str) -> DocstringStyle {
    // NumPy style uses underlined section headers like:
    // Parameters
    // ----------
    if docstring.contains("\n----------")
        || docstring.contains("\n---------")
        || docstring.contains("\n--------")
    {
        return DocstringStyle::NumPy;
    }

    // Google style uses "Section:" format
    let google_markers = [
        "Args:",
        "Arguments:",
        "Parameters:",
        "Returns:",
        "Raises:",
        "Raises:",
        "Example:",
        "Examples:",
        "Attributes:",
        "Note:",
        "Notes:",
        "Yields:",
    ];

    for marker in &google_markers {
        if docstring.contains(marker) {
            return DocstringStyle::Google;
        }
    }

    DocstringStyle::Plain
}
```

</details>



### `fn parse_google_style`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn parse_google_style (docstring : & str) -> ParsedDocstring
```

Parse Google-style docstring

<details>
<summary>Source</summary>

```rust
fn parse_google_style(docstring: &str) -> ParsedDocstring {
    let lines: Vec<&str> = docstring.lines().collect();

    // Find summary - everything before first section or blank line
    let (summary, description, section_start) = extract_summary_and_description(&lines);

    let mut params = Vec::new();
    let mut returns = None;
    let mut raises = Vec::new();
    let mut examples = Vec::new();

    // Parse sections
    let mut i = section_start;
    while i < lines.len() {
        let line = lines[i].trim();

        if line.ends_with(':') && !line.contains(' ') {
            // This might be a section header
            let section_name = &line[..line.len() - 1];
            match section_name.to_lowercase().as_str() {
                "args" | "arguments" | "parameters" | "params" => {
                    let (parsed, next_i) = parse_google_params(&lines, i + 1);
                    params = parsed;
                    i = next_i;
                }
                "returns" | "return" => {
                    let (parsed, next_i) = parse_google_returns(&lines, i + 1);
                    returns = parsed;
                    i = next_i;
                }
                "raises" | "raise" | "exceptions" | "except" => {
                    let (parsed, next_i) = parse_google_raises(&lines, i + 1);
                    raises = parsed;
                    i = next_i;
                }
                "example" | "examples" => {
                    let (parsed, next_i) = parse_google_examples(&lines, i + 1);
                    examples = parsed;
                    i = next_i;
                }
                _ => {
                    i += 1;
                }
            }
        } else {
            i += 1;
        }
    }

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



### `fn parse_numpy_style`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn parse_numpy_style (docstring : & str) -> ParsedDocstring
```

Parse NumPy-style docstring

<details>
<summary>Source</summary>

```rust
fn parse_numpy_style(docstring: &str) -> ParsedDocstring {
    let lines: Vec<&str> = docstring.lines().collect();

    // Find summary
    let (summary, description, section_start) = extract_summary_and_description(&lines);

    let mut params = Vec::new();
    let mut returns = None;
    let mut raises = Vec::new();
    let mut examples = Vec::new();

    // Parse sections - NumPy uses underlined headers
    let mut i = section_start;
    while i < lines.len() {
        let line = lines[i].trim();

        // Check if this is a section header (followed by dashes)
        if i + 1 < lines.len() {
            let next_line = lines[i + 1].trim();
            if next_line.chars().all(|c| c == '-') && !next_line.is_empty() {
                match line.to_lowercase().as_str() {
                    "parameters" | "params" | "arguments" => {
                        let (parsed, next_i) = parse_numpy_params(&lines, i + 2);
                        params = parsed;
                        i = next_i;
                        continue;
                    }
                    "returns" => {
                        let (parsed, next_i) = parse_numpy_returns(&lines, i + 2);
                        returns = parsed;
                        i = next_i;
                        continue;
                    }
                    "raises" | "exceptions" => {
                        let (parsed, next_i) = parse_numpy_raises(&lines, i + 2);
                        raises = parsed;
                        i = next_i;
                        continue;
                    }
                    "examples" | "example" => {
                        let (parsed, next_i) = parse_numpy_examples(&lines, i + 2);
                        examples = parsed;
                        i = next_i;
                        continue;
                    }
                    _ => {}
                }
            }
        }
        i += 1;
    }

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



### `fn parse_plain`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn parse_plain (docstring : & str) -> ParsedDocstring
```

Parse plain docstring (no structured sections)

<details>
<summary>Source</summary>

```rust
fn parse_plain(docstring: &str) -> ParsedDocstring {
    let lines: Vec<&str> = docstring.lines().collect();
    let (summary, description, _) = extract_summary_and_description(&lines);

    ParsedDocstring {
        summary,
        description,
        params: Vec::new(),
        returns: None,
        raises: Vec::new(),
        examples: Vec::new(),
    }
}
```

</details>



### `fn extract_summary_and_description`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn extract_summary_and_description (lines : & [& str]) -> (Option < String > , Option < String > , usize)
```

Extract summary and description from the beginning of a docstring

<details>
<summary>Source</summary>

```rust
fn extract_summary_and_description(lines: &[&str]) -> (Option<String>, Option<String>, usize) {
    if lines.is_empty() {
        return (None, None, 0);
    }

    let mut summary_lines = Vec::new();
    let mut description_lines = Vec::new();
    let mut in_description = false;
    let mut i = 0;

    // Collect summary (first paragraph)
    while i < lines.len() {
        let line = lines[i].trim();

        // Empty line ends summary
        if line.is_empty() {
            if !summary_lines.is_empty() {
                in_description = true;
            }
            i += 1;
            continue;
        }

        // Check if this is a section header (Google style)
        if line.ends_with(':') && !line.contains(' ') {
            let section = &line[..line.len() - 1].to_lowercase();
            if is_known_section(section) {
                break;
            }
        }

        // Check for NumPy style section (line followed by dashes)
        if i + 1 < lines.len() {
            let next_line = lines[i + 1].trim();
            if next_line.chars().all(|c| c == '-')
                && !next_line.is_empty()
                && is_known_section(&line.to_lowercase())
            {
                break;
            }
        }

        if in_description {
            description_lines.push(line);
        } else {
            summary_lines.push(line);
        }
        i += 1;
    }

    let summary = if summary_lines.is_empty() {
        None
    } else {
        Some(summary_lines.join(" "))
    };

    let description = if description_lines.is_empty() {
        None
    } else {
        Some(description_lines.join("\n"))
    };

    (summary, description, i)
}
```

</details>



### `fn is_known_section`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn is_known_section (name : & str) -> bool
```

<details>
<summary>Source</summary>

```rust
fn is_known_section(name: &str) -> bool {
    matches!(
        name,
        "args"
            | "arguments"
            | "parameters"
            | "params"
            | "returns"
            | "return"
            | "raises"
            | "raise"
            | "exceptions"
            | "except"
            | "example"
            | "examples"
            | "attributes"
            | "note"
            | "notes"
            | "yields"
            | "yield"
            | "see also"
            | "references"
            | "warnings"
            | "warning"
    )
}
```

</details>



### `fn parse_google_params`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn parse_google_params (lines : & [& str] , start : usize) -> (Vec < ParamDoc > , usize)
```

Parse Google-style Args/Parameters section

<details>
<summary>Source</summary>

```rust
fn parse_google_params(lines: &[&str], start: usize) -> (Vec<ParamDoc>, usize) {
    let mut params = Vec::new();
    let mut i = start;
    let mut current_name = String::new();
    let mut current_ty: Option<String> = None;
    let mut current_desc = Vec::new();

    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();

        // Empty line might end section
        if trimmed.is_empty() {
            // Save current param if any
            if !current_name.is_empty() {
                params.push(ParamDoc {
                    name: current_name.clone(),
                    ty: current_ty.clone(),
                    description: current_desc.join(" ").trim().to_string(),
                });
                current_name.clear();
                current_ty = None;
                current_desc.clear();
            }
            i += 1;
            continue;
        }

        // Check for new section header
        if trimmed.ends_with(':') && !trimmed.contains(' ') {
            let section = &trimmed[..trimmed.len() - 1].to_lowercase();
            if is_known_section(section) {
                break;
            }
        }

        // Check if this is a new parameter (starts with non-space, contains colon)
        let leading_spaces = line.len() - line.trim_start().len();

        // New parameter line: "name (type): description" or "name: description"
        if leading_spaces <= 4 && trimmed.contains(':') {
            // Save previous param
            if !current_name.is_empty() {
                params.push(ParamDoc {
                    name: current_name,
                    ty: current_ty,
                    description: current_desc.join(" ").trim().to_string(),
                });
            }

            // Parse new param
            let (name, ty, desc) = parse_param_line(trimmed);
            current_name = name;
            current_ty = ty;
            current_desc = vec![desc];
        } else if !current_name.is_empty() {
            // Continuation of previous description
            current_desc.push(trimmed.to_string());
        }

        i += 1;
    }

    // Don't forget the last parameter
    if !current_name.is_empty() {
        params.push(ParamDoc {
            name: current_name,
            ty: current_ty,
            description: current_desc.join(" ").trim().to_string(),
        });
    }

    (params, i)
}
```

</details>



### `fn parse_param_line`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn parse_param_line (line : & str) -> (String , Option < String > , String)
```

Parse a Google-style parameter line: "name (type): description" or "name: description"

<details>
<summary>Source</summary>

```rust
fn parse_param_line(line: &str) -> (String, Option<String>, String) {
    // First, find the colon that separates name/type from description
    // The colon should come after any type annotation in parentheses

    // Look for pattern "name (type): description"
    // The key insight: the colon for the description comes after the closing paren
    if let Some(colon_pos) = line.find(':') {
        let before_colon = &line[..colon_pos];

        // Check if there's a type annotation "(type)" before the colon
        if let Some(paren_start) = before_colon.find('(')
            && let Some(paren_end) = before_colon.rfind(')')
            && paren_start < paren_end
        {
            let name = before_colon[..paren_start].trim().to_string();
            let ty = before_colon[paren_start + 1..paren_end].trim().to_string();
            let desc = line[colon_pos + 1..].trim().to_string();
            return (name, Some(ty), desc);
        }

        // No type annotation, just "name: description"
        let name = before_colon.trim().to_string();
        let desc = line[colon_pos + 1..].trim().to_string();
        return (name, None, desc);
    }

    (line.trim().to_string(), None, String::new())
}
```

</details>



### `fn parse_google_returns`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn parse_google_returns (lines : & [& str] , start : usize) -> (Option < ReturnDoc > , usize)
```

Parse Google-style Returns section

<details>
<summary>Source</summary>

```rust
fn parse_google_returns(lines: &[&str], start: usize) -> (Option<ReturnDoc>, usize) {
    let mut i = start;
    let mut desc_lines = Vec::new();
    let mut ty: Option<String> = None;

    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();

        if trimmed.is_empty() {
            if !desc_lines.is_empty() {
                break;
            }
            i += 1;
            continue;
        }

        // Check for new section header
        if trimmed.ends_with(':') && !trimmed.contains(' ') {
            let section = &trimmed[..trimmed.len() - 1].to_lowercase();
            if is_known_section(section) {
                break;
            }
        }

        // First non-empty line might have type: "type: description"
        if desc_lines.is_empty() && trimmed.contains(':') {
            let colon_pos = trimmed.find(':').unwrap();
            let potential_type = &trimmed[..colon_pos];
            // If it looks like a type (no spaces, reasonable length)
            if !potential_type.contains(' ') || potential_type.contains('[') {
                ty = Some(potential_type.trim().to_string());
                desc_lines.push(trimmed[colon_pos + 1..].trim().to_string());
            } else {
                desc_lines.push(trimmed.to_string());
            }
        } else {
            desc_lines.push(trimmed.to_string());
        }

        i += 1;
    }

    if desc_lines.is_empty() {
        return (None, i);
    }

    let description = desc_lines.join(" ").trim().to_string();
    (Some(ReturnDoc { ty, description }), i)
}
```

</details>



### `fn parse_google_raises`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn parse_google_raises (lines : & [& str] , start : usize) -> (Vec < RaisesDoc > , usize)
```

Parse Google-style Raises section

<details>
<summary>Source</summary>

```rust
fn parse_google_raises(lines: &[&str], start: usize) -> (Vec<RaisesDoc>, usize) {
    let mut raises = Vec::new();
    let mut i = start;
    let mut current_ty = String::new();
    let mut current_desc = Vec::new();

    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();

        if trimmed.is_empty() {
            if !current_ty.is_empty() {
                raises.push(RaisesDoc {
                    ty: current_ty.clone(),
                    description: current_desc.join(" ").trim().to_string(),
                });
                current_ty.clear();
                current_desc.clear();
            }
            i += 1;
            continue;
        }

        // Check for new section header
        if trimmed.ends_with(':') && !trimmed.contains(' ') {
            let section = &trimmed[..trimmed.len() - 1].to_lowercase();
            if is_known_section(section) {
                break;
            }
        }

        let leading_spaces = line.len() - line.trim_start().len();

        // New exception: "ExceptionType: description"
        if leading_spaces <= 4 && trimmed.contains(':') {
            if !current_ty.is_empty() {
                raises.push(RaisesDoc {
                    ty: current_ty,
                    description: current_desc.join(" ").trim().to_string(),
                });
            }

            let colon_pos = trimmed.find(':').unwrap();
            current_ty = trimmed[..colon_pos].trim().to_string();
            current_desc = vec![trimmed[colon_pos + 1..].trim().to_string()];
        } else if !current_ty.is_empty() {
            current_desc.push(trimmed.to_string());
        }

        i += 1;
    }

    if !current_ty.is_empty() {
        raises.push(RaisesDoc {
            ty: current_ty,
            description: current_desc.join(" ").trim().to_string(),
        });
    }

    (raises, i)
}
```

</details>



### `fn parse_google_examples`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn parse_google_examples (lines : & [& str] , start : usize) -> (Vec < String > , usize)
```

Parse Google-style Examples section

<details>
<summary>Source</summary>

```rust
fn parse_google_examples(lines: &[&str], start: usize) -> (Vec<String>, usize) {
    let mut examples = Vec::new();
    let mut current_example = Vec::new();
    let mut i = start;

    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();

        // Check for new section header
        if trimmed.ends_with(':') && !trimmed.contains(' ') {
            let section = &trimmed[..trimmed.len() - 1].to_lowercase();
            if is_known_section(section) {
                break;
            }
        }

        // Empty line might separate examples
        if trimmed.is_empty() {
            if !current_example.is_empty() {
                examples.push(current_example.join("\n"));
                current_example.clear();
            }
            i += 1;
            continue;
        }

        current_example.push(line.to_string());
        i += 1;
    }

    if !current_example.is_empty() {
        examples.push(current_example.join("\n"));
    }

    (examples, i)
}
```

</details>



### `fn parse_numpy_params`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn parse_numpy_params (lines : & [& str] , start : usize) -> (Vec < ParamDoc > , usize)
```

Parse NumPy-style Parameters section

<details>
<summary>Source</summary>

```rust
fn parse_numpy_params(lines: &[&str], start: usize) -> (Vec<ParamDoc>, usize) {
    let mut params = Vec::new();
    let mut i = start;
    let mut current_name = String::new();
    let mut current_ty: Option<String> = None;
    let mut current_desc = Vec::new();

    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();

        // Check for new section (line followed by dashes)
        if i + 1 < lines.len() {
            let next_line = lines[i + 1].trim();
            if next_line.chars().all(|c| c == '-') && !next_line.is_empty() {
                break;
            }
        }

        if trimmed.is_empty() {
            i += 1;
            continue;
        }

        let leading_spaces = line.len() - line.trim_start().len();

        // NumPy format: "param_name : type" on one line, description indented below
        if leading_spaces == 0 && trimmed.contains(':') {
            // Save previous
            if !current_name.is_empty() {
                params.push(ParamDoc {
                    name: current_name,
                    ty: current_ty,
                    description: current_desc.join(" ").trim().to_string(),
                });
            }

            let colon_pos = trimmed.find(':').unwrap();
            current_name = trimmed[..colon_pos].trim().to_string();
            let type_part = trimmed[colon_pos + 1..].trim();
            current_ty = if type_part.is_empty() {
                None
            } else {
                Some(type_part.to_string())
            };
            current_desc.clear();
        } else if leading_spaces > 0 && !current_name.is_empty() {
            // Description continuation
            current_desc.push(trimmed.to_string());
        }

        i += 1;
    }

    if !current_name.is_empty() {
        params.push(ParamDoc {
            name: current_name,
            ty: current_ty,
            description: current_desc.join(" ").trim().to_string(),
        });
    }

    (params, i)
}
```

</details>



### `fn parse_numpy_returns`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn parse_numpy_returns (lines : & [& str] , start : usize) -> (Option < ReturnDoc > , usize)
```

Parse NumPy-style Returns section

<details>
<summary>Source</summary>

```rust
fn parse_numpy_returns(lines: &[&str], start: usize) -> (Option<ReturnDoc>, usize) {
    let mut i = start;
    let mut ty: Option<String> = None;
    let mut desc_lines = Vec::new();

    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();

        // Check for new section
        if i + 1 < lines.len() {
            let next_line = lines[i + 1].trim();
            if next_line.chars().all(|c| c == '-') && !next_line.is_empty() {
                break;
            }
        }

        if trimmed.is_empty() {
            if !desc_lines.is_empty() || ty.is_some() {
                break;
            }
            i += 1;
            continue;
        }

        let leading_spaces = line.len() - line.trim_start().len();

        // First line might be "type" or "name : type"
        if ty.is_none() && leading_spaces == 0 {
            if trimmed.contains(':') {
                let colon_pos = trimmed.find(':').unwrap();
                ty = Some(trimmed[colon_pos + 1..].trim().to_string());
            } else {
                ty = Some(trimmed.to_string());
            }
        } else if leading_spaces > 0 {
            desc_lines.push(trimmed.to_string());
        }

        i += 1;
    }

    if ty.is_none() && desc_lines.is_empty() {
        return (None, i);
    }

    (
        Some(ReturnDoc {
            ty,
            description: desc_lines.join(" ").trim().to_string(),
        }),
        i,
    )
}
```

</details>



### `fn parse_numpy_raises`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn parse_numpy_raises (lines : & [& str] , start : usize) -> (Vec < RaisesDoc > , usize)
```

Parse NumPy-style Raises section

<details>
<summary>Source</summary>

```rust
fn parse_numpy_raises(lines: &[&str], start: usize) -> (Vec<RaisesDoc>, usize) {
    let mut raises = Vec::new();
    let mut i = start;
    let mut current_ty = String::new();
    let mut current_desc = Vec::new();

    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();

        // Check for new section
        if i + 1 < lines.len() {
            let next_line = lines[i + 1].trim();
            if next_line.chars().all(|c| c == '-') && !next_line.is_empty() {
                break;
            }
        }

        if trimmed.is_empty() {
            i += 1;
            continue;
        }

        let leading_spaces = line.len() - line.trim_start().len();

        if leading_spaces == 0 {
            // Save previous
            if !current_ty.is_empty() {
                raises.push(RaisesDoc {
                    ty: current_ty,
                    description: current_desc.join(" ").trim().to_string(),
                });
            }
            current_ty = trimmed.to_string();
            current_desc.clear();
        } else if !current_ty.is_empty() {
            current_desc.push(trimmed.to_string());
        }

        i += 1;
    }

    if !current_ty.is_empty() {
        raises.push(RaisesDoc {
            ty: current_ty,
            description: current_desc.join(" ").trim().to_string(),
        });
    }

    (raises, i)
}
```

</details>



### `fn parse_numpy_examples`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn parse_numpy_examples (lines : & [& str] , start : usize) -> (Vec < String > , usize)
```

Parse NumPy-style Examples section

<details>
<summary>Source</summary>

```rust
fn parse_numpy_examples(lines: &[&str], start: usize) -> (Vec<String>, usize) {
    // NumPy examples are similar to Google style
    parse_google_examples(lines, start)
}
```

</details>



### `fn parse_rust_doc`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn parse_rust_doc (doc : & str) -> ParsedDocstring
```

Parse a Rust doc comment into structured form

Looks for conventional markdown sections:
- `# Arguments` / `# Parameters` - function parameters
- `# Returns` - return value documentation
- `# Errors` - error conditions (maps to raises)
- `# Panics` - panic conditions (maps to raises)
- `# Safety` - safety requirements (stored in description)
- `# Examples` - code examples
If parsing fails or no sections are found, returns a basic ParsedDocstring
with just the summary/description extracted.

<details>
<summary>Source</summary>

```rust
pub fn parse_rust_doc(doc: &str) -> ParsedDocstring {
    let doc = doc.trim();
    if doc.is_empty() {
        return ParsedDocstring::empty();
    }

    let lines: Vec<&str> = doc.lines().collect();

    // Extract summary and description (everything before first # section)
    let (summary, description, section_start) = extract_rust_summary(&lines);

    let mut params = Vec::new();
    let mut returns = None;
    let mut raises = Vec::new();
    let mut examples = Vec::new();
    let mut safety_notes = Vec::new();

    // Parse sections
    let mut i = section_start;
    while i < lines.len() {
        let line = lines[i].trim();

        // Check for markdown header: # Section or ## Section
        if let Some(section_name) = parse_markdown_header(line) {
            let section_lower = section_name.to_lowercase();
            match section_lower.as_str() {
                "arguments" | "parameters" | "args" | "params" => {
                    let (parsed, next_i) = parse_rust_arguments(&lines, i + 1);
                    params = parsed;
                    i = next_i;
                }
                "returns" | "return" => {
                    let (parsed, next_i) = parse_rust_returns(&lines, i + 1);
                    returns = parsed;
                    i = next_i;
                }
                "errors" | "error" => {
                    let (parsed, next_i) = parse_rust_errors(&lines, i + 1, "Error");
                    raises.extend(parsed);
                    i = next_i;
                }
                "panics" | "panic" => {
                    let (parsed, next_i) = parse_rust_errors(&lines, i + 1, "Panic");
                    raises.extend(parsed);
                    i = next_i;
                }
                "safety" => {
                    let (notes, next_i) = parse_rust_section_text(&lines, i + 1);
                    safety_notes.push(notes);
                    i = next_i;
                }
                "examples" | "example" => {
                    let (parsed, next_i) = parse_rust_examples(&lines, i + 1);
                    examples = parsed;
                    i = next_i;
                }
                _ => {
                    // Unknown section, skip
                    i += 1;
                }
            }
        } else {
            i += 1;
        }
    }

    // If we found safety notes, append to description
    let final_description = if safety_notes.is_empty() {
        description
    } else {
        let safety_text = format!("\n\n# Safety\n{}", safety_notes.join("\n"));
        match description {
            Some(desc) => Some(format!("{}{}", desc, safety_text)),
            None => Some(safety_text.trim_start().to_string()),
        }
    };

    ParsedDocstring {
        summary,
        description: final_description,
        params,
        returns,
        raises,
        examples,
    }
}
```

</details>



### `fn extract_rust_summary`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn extract_rust_summary (lines : & [& str]) -> (Option < String > , Option < String > , usize)
```

Extract summary and description from Rust doc before any # sections

<details>
<summary>Source</summary>

```rust
fn extract_rust_summary(lines: &[&str]) -> (Option<String>, Option<String>, usize) {
    if lines.is_empty() {
        return (None, None, 0);
    }

    let mut summary_lines = Vec::new();
    let mut description_lines = Vec::new();
    let mut in_description = false;
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();

        // Check for markdown header - this starts a section
        if parse_markdown_header(line).is_some() {
            break;
        }

        // Empty line transitions from summary to description
        if line.is_empty() {
            if !summary_lines.is_empty() {
                in_description = true;
            }
            i += 1;
            continue;
        }

        if in_description {
            description_lines.push(line);
        } else {
            summary_lines.push(line);
        }
        i += 1;
    }

    let summary = if summary_lines.is_empty() {
        None
    } else {
        Some(summary_lines.join(" "))
    };

    let description = if description_lines.is_empty() {
        None
    } else {
        Some(description_lines.join("\n"))
    };

    (summary, description, i)
}
```

</details>



### `fn parse_markdown_header`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn parse_markdown_header (line : & str) -> Option < & str >
```

Parse a markdown header line, returning the section name if found

<details>
<summary>Source</summary>

```rust
fn parse_markdown_header(line: &str) -> Option<&str> {
    let trimmed = line.trim();

    // Match # Header or ## Header (up to 3 levels)
    if let Some(rest) = trimmed.strip_prefix("### ") {
        Some(rest.trim())
    } else if let Some(rest) = trimmed.strip_prefix("## ") {
        Some(rest.trim())
    } else if let Some(rest) = trimmed.strip_prefix("# ") {
        Some(rest.trim())
    } else {
        None
    }
}
```

</details>



### `fn parse_rust_arguments`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn parse_rust_arguments (lines : & [& str] , start : usize) -> (Vec < ParamDoc > , usize)
```

Parse Rust-style Arguments section

Expects format like:
```text
* `name` - Description of the parameter
* `other` - Another parameter
```
or
```text
- `name`: Description
```

<details>
<summary>Source</summary>

```rust
fn parse_rust_arguments(lines: &[&str], start: usize) -> (Vec<ParamDoc>, usize) {
    let mut params = Vec::new();
    let mut i = start;
    let mut current_name = String::new();
    let mut current_desc = Vec::new();

    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();

        // Stop at next section
        if parse_markdown_header(trimmed).is_some() {
            break;
        }

        // Empty line might end the section
        if trimmed.is_empty() {
            if !current_name.is_empty() {
                params.push(ParamDoc {
                    name: current_name.clone(),
                    ty: None,
                    description: current_desc.join(" ").trim().to_string(),
                });
                current_name.clear();
                current_desc.clear();
            }
            i += 1;
            continue;
        }

        // Look for list item: * `name` - desc or - `name`: desc
        if let Some(param) = parse_rust_param_line(trimmed) {
            // Save previous
            if !current_name.is_empty() {
                params.push(ParamDoc {
                    name: current_name,
                    ty: None,
                    description: current_desc.join(" ").trim().to_string(),
                });
            }
            current_name = param.0;
            current_desc = vec![param.1];
        } else if !current_name.is_empty()
            && (trimmed.starts_with(' ') || !trimmed.starts_with('*') && !trimmed.starts_with('-'))
        {
            // Continuation line
            current_desc.push(trimmed.to_string());
        }

        i += 1;
    }

    // Don't forget the last one
    if !current_name.is_empty() {
        params.push(ParamDoc {
            name: current_name,
            ty: None,
            description: current_desc.join(" ").trim().to_string(),
        });
    }

    (params, i)
}
```

</details>



### `fn parse_rust_param_line`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn parse_rust_param_line (line : & str) -> Option < (String , String) >
```

Parse a single Rust parameter line Formats: `* `name` - description` or `- `name`: description` or `* name - description`

<details>
<summary>Source</summary>

```rust
fn parse_rust_param_line(line: &str) -> Option<(String, String)> {
    let trimmed = line.trim();

    // Must start with * or -
    if !trimmed.starts_with('*') && !trimmed.starts_with('-') {
        return None;
    }

    let rest = trimmed[1..].trim();

    // Try to find backtick-quoted name: `name`
    if rest.starts_with('`')
        && let Some(end_tick) = rest[1..].find('`')
    {
        let name = rest[1..end_tick + 1].to_string();
        let after_name = rest[end_tick + 2..].trim();

        // Look for separator: - or :
        let desc = if let Some(rest) = after_name
            .strip_prefix('-')
            .or_else(|| after_name.strip_prefix(':'))
        {
            rest.trim().to_string()
        } else {
            after_name.to_string()
        };

        return Some((name, desc));
    }

    // Try plain format: name - description
    if let Some(sep_pos) = rest.find(" - ") {
        let name = rest[..sep_pos].trim().to_string();
        let desc = rest[sep_pos + 3..].trim().to_string();
        return Some((name, desc));
    }

    // Try colon format: name: description
    if let Some(sep_pos) = rest.find(':') {
        let name = rest[..sep_pos].trim().to_string();
        let desc = rest[sep_pos + 1..].trim().to_string();
        return Some((name, desc));
    }

    None
}
```

</details>



### `fn parse_rust_returns`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn parse_rust_returns (lines : & [& str] , start : usize) -> (Option < ReturnDoc > , usize)
```

Parse Rust-style Returns section

<details>
<summary>Source</summary>

```rust
fn parse_rust_returns(lines: &[&str], start: usize) -> (Option<ReturnDoc>, usize) {
    let (text, next_i) = parse_rust_section_text(lines, start);

    if text.is_empty() {
        return (None, next_i);
    }

    (
        Some(ReturnDoc {
            ty: None,
            description: text,
        }),
        next_i,
    )
}
```

</details>



### `fn parse_rust_errors`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn parse_rust_errors (lines : & [& str] , start : usize , error_kind : & str) -> (Vec < RaisesDoc > , usize)
```

Parse Rust-style Errors/Panics section

<details>
<summary>Source</summary>

```rust
fn parse_rust_errors(lines: &[&str], start: usize, error_kind: &str) -> (Vec<RaisesDoc>, usize) {
    let mut raises = Vec::new();
    let mut i = start;
    let mut current_ty = String::new();
    let mut current_desc = Vec::new();

    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();

        // Stop at next section
        if parse_markdown_header(trimmed).is_some() {
            break;
        }

        if trimmed.is_empty() {
            if !current_ty.is_empty() || !current_desc.is_empty() {
                raises.push(RaisesDoc {
                    ty: if current_ty.is_empty() {
                        error_kind.to_string()
                    } else {
                        current_ty.clone()
                    },
                    description: current_desc.join(" ").trim().to_string(),
                });
                current_ty.clear();
                current_desc.clear();
            }
            i += 1;
            continue;
        }

        // Check for list item with error type: * `ErrorType` - when...
        if trimmed.starts_with('*') || trimmed.starts_with('-') {
            // Save previous
            if !current_ty.is_empty() || !current_desc.is_empty() {
                raises.push(RaisesDoc {
                    ty: if current_ty.is_empty() {
                        error_kind.to_string()
                    } else {
                        current_ty
                    },
                    description: current_desc.join(" ").trim().to_string(),
                });
            }

            let rest = trimmed[1..].trim();

            // Try to extract error type from backticks
            if let Some(after_tick) = rest.strip_prefix('`') {
                if let Some(end_tick) = after_tick.find('`') {
                    current_ty = after_tick[..end_tick].to_string();
                    let after = after_tick[end_tick + 1..].trim();
                    current_desc = vec![
                        after
                            .trim_start_matches('-')
                            .trim_start_matches(':')
                            .trim()
                            .to_string(),
                    ];
                } else {
                    current_ty = error_kind.to_string();
                    current_desc = vec![rest.to_string()];
                }
            } else {
                current_ty = error_kind.to_string();
                current_desc = vec![rest.to_string()];
            }
        } else if !current_desc.is_empty() {
            // Continuation
            current_desc.push(trimmed.to_string());
        } else {
            // Plain text, no list format
            current_ty = error_kind.to_string();
            current_desc.push(trimmed.to_string());
        }

        i += 1;
    }

    // Don't forget the last one
    if !current_ty.is_empty() || !current_desc.is_empty() {
        raises.push(RaisesDoc {
            ty: if current_ty.is_empty() {
                error_kind.to_string()
            } else {
                current_ty
            },
            description: current_desc.join(" ").trim().to_string(),
        });
    }

    (raises, i)
}
```

</details>



### `fn parse_rust_examples`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn parse_rust_examples (lines : & [& str] , start : usize) -> (Vec < String > , usize)
```

Parse Rust-style Examples section

<details>
<summary>Source</summary>

```rust
fn parse_rust_examples(lines: &[&str], start: usize) -> (Vec<String>, usize) {
    let mut examples = Vec::new();
    let mut current_example = Vec::new();
    let mut in_code_block = false;
    let mut i = start;

    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();

        // Stop at next section (but not if we're in a code block)
        if !in_code_block && parse_markdown_header(trimmed).is_some() {
            break;
        }

        // Track code fence blocks
        if trimmed.starts_with("```") {
            in_code_block = !in_code_block;
            current_example.push(line.to_string());
            i += 1;
            continue;
        }

        // Empty line outside code block might separate examples
        if trimmed.is_empty() && !in_code_block {
            if !current_example.is_empty() {
                examples.push(current_example.join("\n"));
                current_example.clear();
            }
            i += 1;
            continue;
        }

        current_example.push(line.to_string());
        i += 1;
    }

    if !current_example.is_empty() {
        examples.push(current_example.join("\n"));
    }

    (examples, i)
}
```

</details>



### `fn parse_rust_section_text`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn parse_rust_section_text (lines : & [& str] , start : usize) -> (String , usize)
```

Parse a section as plain text until next section

<details>
<summary>Source</summary>

```rust
fn parse_rust_section_text(lines: &[&str], start: usize) -> (String, usize) {
    let mut text_lines = Vec::new();
    let mut i = start;

    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();

        // Stop at next section
        if parse_markdown_header(trimmed).is_some() {
            break;
        }

        // Skip leading empty lines
        if trimmed.is_empty() && text_lines.is_empty() {
            i += 1;
            continue;
        }

        text_lines.push(trimmed);
        i += 1;
    }

    // Trim trailing empty lines
    while text_lines.last().map(|s| s.is_empty()).unwrap_or(false) {
        text_lines.pop();
    }

    (text_lines.join(" ").trim().to_string(), i)
}
```

</details>



