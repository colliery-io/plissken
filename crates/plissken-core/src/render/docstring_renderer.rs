//! Docstring rendering for parsed documentation
//!
//! This module provides rendering functionality for converting `ParsedDocstring`
//! structures into formatted Markdown with parameter tables, return sections,
//! raises tables, and code examples.

use crate::model::{ParamDoc, ParsedDocstring, RaisesDoc, ReturnDoc};

/// Render a parsed docstring to Markdown.
///
/// Produces formatted Markdown with:
/// - Summary and description paragraphs
/// - Parameters table (Name | Type | Description)
/// - Returns section with type and description
/// - Raises table (Exception | Description)
/// - Examples as fenced code blocks
///
/// Empty sections are omitted entirely.
///
/// # Example
///
/// ```rust
/// use plissken_core::model::{ParsedDocstring, ParamDoc, ReturnDoc};
/// use plissken_core::render::render_docstring;
///
/// let doc = ParsedDocstring {
///     summary: Some("Calculate the sum of two numbers.".to_string()),
///     description: None,
///     params: vec![
///         ParamDoc { name: "a".to_string(), ty: Some("int".to_string()), description: "First number".to_string() },
///         ParamDoc { name: "b".to_string(), ty: Some("int".to_string()), description: "Second number".to_string() },
///     ],
///     returns: Some(ReturnDoc { ty: Some("int".to_string()), description: "The sum".to_string() }),
///     raises: vec![],
///     examples: vec![],
/// };
///
/// let output = render_docstring(&doc);
/// assert!(output.contains("Calculate the sum"));
/// assert!(output.contains("| `a` | `int` |"));
/// ```
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

/// Render parameters as a Markdown table.
///
/// Format:
/// ```markdown
/// **Parameters:**
///
/// | Name | Type | Description |
/// |------|------|-------------|
/// | `param` | `type` | Description text |
/// ```
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

/// Render returns section.
///
/// Format:
/// ```markdown
/// **Returns:** `type`
///
/// Description text
/// ```
pub fn render_returns(returns: &ReturnDoc) -> String {
    let mut output = String::from("**Returns:**");

    if let Some(ref ty) = returns.ty {
        output.push_str(&format!(" `{}`", ty));
    }

    output.push_str("\n\n");
    output.push_str(&returns.description);

    output
}

/// Render raises/exceptions as a Markdown table.
///
/// Format:
/// ```markdown
/// **Raises:**
///
/// | Exception | Description |
/// |-----------|-------------|
/// | `ValueError` | When value is invalid |
/// ```
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

/// Render examples as fenced code blocks.
///
/// Format:
/// ```markdown
/// **Examples:**
///
/// ```python
/// example code here
/// ```
/// ```
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

/// Detect the programming language of an example code block.
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

/// Escape content for use in Markdown tables.
///
/// Pipes need to be escaped and newlines replaced.
fn escape_table_content(content: &str) -> String {
    content
        .replace('|', "\\|")
        .replace('\n', " ")
        .trim()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{ParamDoc, ParsedDocstring, RaisesDoc, ReturnDoc};

    fn test_parsed_docstring() -> ParsedDocstring {
        ParsedDocstring {
            summary: Some("A test function.".to_string()),
            description: Some("This function does something useful.".to_string()),
            params: vec![
                ParamDoc {
                    name: "x".to_string(),
                    ty: Some("int".to_string()),
                    description: "The first parameter".to_string(),
                },
                ParamDoc {
                    name: "y".to_string(),
                    ty: Some("str".to_string()),
                    description: "The second parameter".to_string(),
                },
            ],
            returns: Some(ReturnDoc {
                ty: Some("bool".to_string()),
                description: "True if successful".to_string(),
            }),
            raises: vec![
                RaisesDoc {
                    ty: "ValueError".to_string(),
                    description: "If x is negative".to_string(),
                },
                RaisesDoc {
                    ty: "TypeError".to_string(),
                    description: "If y is not a string".to_string(),
                },
            ],
            examples: vec![
                ">>> result = test_func(1, 'hello')\n>>> print(result)\nTrue".to_string(),
            ],
        }
    }

    #[test]
    fn test_render_full_docstring() {
        let doc = test_parsed_docstring();
        let output = render_docstring(&doc);

        // Summary and description
        assert!(output.contains("A test function."));
        assert!(output.contains("This function does something useful."));

        // Parameters table
        assert!(output.contains("**Parameters:**"));
        assert!(output.contains("| Name | Type | Description |"));
        assert!(output.contains("| `x` | `int` | The first parameter |"));
        assert!(output.contains("| `y` | `str` | The second parameter |"));

        // Returns
        assert!(output.contains("**Returns:** `bool`"));
        assert!(output.contains("True if successful"));

        // Raises
        assert!(output.contains("**Raises:**"));
        assert!(output.contains("| Exception | Description |"));
        assert!(output.contains("| `ValueError` | If x is negative |"));
        assert!(output.contains("| `TypeError` | If y is not a string |"));

        // Examples
        assert!(output.contains("**Examples:**"));
        assert!(output.contains("```python"));
        assert!(output.contains(">>> result = test_func"));
    }

    #[test]
    fn test_render_params_only() {
        let doc = ParsedDocstring {
            summary: None,
            description: None,
            params: vec![ParamDoc {
                name: "value".to_string(),
                ty: Some("Any".to_string()),
                description: "The value to process".to_string(),
            }],
            returns: None,
            raises: vec![],
            examples: vec![],
        };

        let output = render_docstring(&doc);

        assert!(output.contains("**Parameters:**"));
        assert!(output.contains("| `value` | `Any` | The value to process |"));
        assert!(!output.contains("**Returns:**"));
        assert!(!output.contains("**Raises:**"));
        assert!(!output.contains("**Examples:**"));
    }

    #[test]
    fn test_render_empty_docstring() {
        let doc = ParsedDocstring {
            summary: None,
            description: None,
            params: vec![],
            returns: None,
            raises: vec![],
            examples: vec![],
        };

        let output = render_docstring(&doc);
        assert!(output.is_empty());
    }

    #[test]
    fn test_render_summary_only() {
        let doc = ParsedDocstring {
            summary: Some("Just a summary.".to_string()),
            description: None,
            params: vec![],
            returns: None,
            raises: vec![],
            examples: vec![],
        };

        let output = render_docstring(&doc);
        assert_eq!(output, "Just a summary.");
    }

    #[test]
    fn test_render_returns_without_type() {
        let returns = ReturnDoc {
            ty: None,
            description: "The processed result".to_string(),
        };

        let output = render_returns(&returns);
        assert!(output.contains("**Returns:**"));
        assert!(output.contains("The processed result"));
        assert!(!output.contains("`"));
    }

    #[test]
    fn test_render_params_without_type() {
        let params = vec![ParamDoc {
            name: "arg".to_string(),
            ty: None,
            description: "An argument".to_string(),
        }];

        let output = render_params_table(&params);
        assert!(output.contains("| `arg` | `-` | An argument |"));
    }

    #[test]
    fn test_escape_table_content() {
        assert_eq!(escape_table_content("normal text"), "normal text");
        assert_eq!(escape_table_content("has | pipe"), "has \\| pipe");
        assert_eq!(escape_table_content("multi\nline"), "multi line");
        assert_eq!(escape_table_content("  trimmed  "), "trimmed");
    }

    #[test]
    fn test_detect_example_language_python() {
        assert_eq!(detect_example_language(">>> print('hello')"), "python");
        assert_eq!(detect_example_language("def foo(): pass"), "python");
        assert_eq!(detect_example_language("class Bar: pass"), "python");
        assert_eq!(detect_example_language("import os"), "python");
        assert_eq!(detect_example_language("from sys import path"), "python");
    }

    #[test]
    fn test_detect_example_language_rust() {
        assert_eq!(detect_example_language("fn main() {}"), "rust");
        assert_eq!(detect_example_language("let x = 5;"), "rust");
        assert_eq!(detect_example_language("use std::io;"), "rust");
        assert_eq!(detect_example_language("struct Foo;"), "rust");
        assert_eq!(detect_example_language("println!(\"hi\");"), "rust");
    }

    #[test]
    fn test_render_multiple_examples() {
        let examples = vec![">>> x = 1".to_string(), ">>> y = 2".to_string()];

        let output = render_examples(&examples);

        // Should have two code blocks
        let block_count = output.matches("```python").count();
        assert_eq!(block_count, 2);
    }

    #[test]
    fn test_render_raises_with_long_descriptions() {
        let raises = vec![RaisesDoc {
            ty: "RuntimeError".to_string(),
            description: "When something goes | wrong with\nmultiple lines".to_string(),
        }];

        let output = render_raises_table(&raises);

        // Pipes should be escaped in the description
        assert!(output.contains("\\|"));
        // The description should have newlines converted to spaces
        assert!(output.contains("wrong with multiple lines"));
    }

    #[test]
    fn test_google_style_docstring_rendering() {
        // Simulates a Google-style parsed docstring
        let doc = ParsedDocstring {
            summary: Some("Fetch data from a URL.".to_string()),
            description: Some(
                "Makes an HTTP GET request to the specified URL and returns the response body."
                    .to_string(),
            ),
            params: vec![
                ParamDoc {
                    name: "url".to_string(),
                    ty: Some("str".to_string()),
                    description: "The URL to fetch".to_string(),
                },
                ParamDoc {
                    name: "timeout".to_string(),
                    ty: Some("float".to_string()),
                    description: "Request timeout in seconds".to_string(),
                },
            ],
            returns: Some(ReturnDoc {
                ty: Some("bytes".to_string()),
                description: "The response body".to_string(),
            }),
            raises: vec![
                RaisesDoc {
                    ty: "HTTPError".to_string(),
                    description: "If the request fails".to_string(),
                },
                RaisesDoc {
                    ty: "TimeoutError".to_string(),
                    description: "If the request times out".to_string(),
                },
            ],
            examples: vec![
                ">>> data = fetch('https://example.com')\n>>> len(data)\n1234".to_string(),
            ],
        };

        let output = render_docstring(&doc);

        // All sections should be present
        assert!(output.contains("Fetch data from a URL."));
        assert!(output.contains("Makes an HTTP GET request"));
        assert!(output.contains("| `url` | `str` |"));
        assert!(output.contains("| `timeout` | `float` |"));
        assert!(output.contains("**Returns:** `bytes`"));
        assert!(output.contains("| `HTTPError` |"));
        assert!(output.contains("| `TimeoutError` |"));
        assert!(output.contains("```python"));
    }

    #[test]
    fn test_numpy_style_docstring_rendering() {
        // Simulates a NumPy-style parsed docstring (same structure, just different source)
        let doc = ParsedDocstring {
            summary: Some("Calculate array statistics.".to_string()),
            description: Some("Computes mean, std, min, max of an array.".to_string()),
            params: vec![
                ParamDoc {
                    name: "arr".to_string(),
                    ty: Some("array_like".to_string()),
                    description: "Input array".to_string(),
                },
                ParamDoc {
                    name: "axis".to_string(),
                    ty: Some("int, optional".to_string()),
                    description: "Axis along which to compute".to_string(),
                },
            ],
            returns: Some(ReturnDoc {
                ty: Some("dict".to_string()),
                description: "Dictionary with mean, std, min, max keys".to_string(),
            }),
            raises: vec![],
            examples: vec![">>> stats([1, 2, 3])\n{'mean': 2.0, 'std': 0.816...}".to_string()],
        };

        let output = render_docstring(&doc);

        assert!(output.contains("Calculate array statistics."));
        assert!(output.contains("| `arr` | `array_like` |"));
        assert!(output.contains("| `axis` | `int, optional` |"));
        assert!(output.contains("**Returns:** `dict`"));
        // No raises section since it's empty
        assert!(!output.contains("**Raises:**"));
    }
}
