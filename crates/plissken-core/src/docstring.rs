//! Docstring parser for Google, NumPy, and Rust doc comment styles
//!
//! This module parses docstrings into structured `ParsedDocstring` objects,
//! extracting summary, parameters, returns, raises, and examples.
//!
//! Supported formats:
//! - **Google style**: `Args:`, `Returns:`, `Raises:`, `Example:`
//! - **NumPy style**: Underlined section headers
//! - **Rust style**: `# Arguments`, `# Returns`, `# Errors`, `# Panics`, `# Examples`

use crate::model::{ParamDoc, ParsedDocstring, RaisesDoc, ReturnDoc};

/// Parse a docstring into structured form
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

#[derive(Debug, Clone, Copy, PartialEq)]
enum DocstringStyle {
    Google,
    NumPy,
    Plain,
}

/// Detect the docstring style based on section markers
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

/// Parse Google-style docstring
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

/// Parse NumPy-style docstring
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

/// Parse plain docstring (no structured sections)
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

/// Extract summary and description from the beginning of a docstring
fn extract_summary_and_description(lines: &[&str]) -> (Option<String>, Option<String>, usize) {
    if lines.is_empty() {
        return (None, None, 0);
    }

    let mut summary_lines = Vec::new();
    let mut description_lines: Vec<&str> = Vec::new();
    let mut in_description = false;
    let mut i = 0;

    // Collect summary (first paragraph)
    while i < lines.len() {
        let raw = lines[i];
        let line = raw.trim();

        // Empty line ends summary; in the description it ends a paragraph
        if line.is_empty() {
            if !summary_lines.is_empty() {
                in_description = true;
            }
            if in_description {
                description_lines.push("");
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
            description_lines.push(raw);
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

    // Keep the indentation of the description relative to its least
    // indented line (code blocks, nested lists)
    let indent = description_lines
        .iter()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.len() - l.trim_start().len())
        .min()
        .unwrap_or(0);
    let dedented: Vec<&str> = description_lines
        .iter()
        .map(|l| l.get(indent..).unwrap_or_else(|| l.trim_start()).trim_end())
        .collect();
    let description_lines = tidy_markdown_lines(&dedented);
    let description = if description_lines.is_empty() {
        None
    } else {
        Some(description_lines.join("\n"))
    };

    (summary, description, i)
}

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

/// Parse Google-style Args/Parameters section
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

/// Parse a Google-style parameter line: "name (type): description" or "name: description"
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

/// Parse Google-style Returns section
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

/// Parse Google-style Raises section
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

/// Parse Google-style Examples section
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

/// Parse NumPy-style Parameters section
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

/// Parse NumPy-style Returns section
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

/// Parse NumPy-style Raises section
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

/// Parse NumPy-style Examples section
fn parse_numpy_examples(lines: &[&str], start: usize) -> (Vec<String>, usize) {
    // NumPy examples are similar to Google style
    parse_google_examples(lines, start)
}

impl ParsedDocstring {
    /// Create an empty parsed docstring
    pub fn empty() -> Self {
        Self {
            summary: None,
            description: None,
            params: Vec::new(),
            returns: None,
            raises: Vec::new(),
            examples: Vec::new(),
        }
    }

    /// Check if the docstring has any content
    pub fn is_empty(&self) -> bool {
        self.summary.is_none()
            && self.description.is_none()
            && self.params.is_empty()
            && self.returns.is_none()
            && self.raises.is_empty()
            && self.examples.is_empty()
    }
}

// ============================================================================
// Rust Doc Comment Parser
// ============================================================================

/// The `RaisesDoc` kind of an entry of a Rust `# Errors` section.
pub const RUST_ERROR_KIND: &str = "Error";

/// The `RaisesDoc` kind of an entry of a Rust `# Panics` section.
pub const RUST_PANIC_KIND: &str = "Panic";

/// Parse a Rust doc comment into structured form
///
/// Looks for conventional markdown sections:
/// - `# Arguments` / `# Parameters` - function parameters
/// - `# Returns` - return value documentation
/// - `# Errors` - error conditions (raises of kind [`RUST_ERROR_KIND`])
/// - `# Panics` - panic conditions (raises of kind [`RUST_PANIC_KIND`])
/// - `# Examples` - code examples
///
/// Other sections (`# Safety`, `# Overview`, ...) stay in the description,
/// with the heading as a bold line. The description keeps its Markdown:
/// paragraphs, lists and code blocks. Code blocks hold Rust unless they name
/// another language, as in rustdoc: they get the language `rust`, and the
/// hidden lines of a doc test (`# use foo;`) are left out.
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

/// Rust doc text made ready for Markdown output.
///
/// Code blocks get the language `rust` (see [`parse_rust_doc`]), and
/// runs of blank lines become one. For doc comments that are not parsed into
/// sections: fields, variants.
pub fn normalize_rust_doc(doc: &str) -> String {
    let doc = normalize_rust_doc_code(doc);
    let lines: Vec<&str> = doc.lines().collect();
    tidy_markdown_lines(&lines).join("\n")
}

/// Code fence attributes of rustdoc (`ignore`, `no_run`, ...).
const RUSTDOC_FENCE_ATTRIBUTES: &[&str] = &[
    "rust",
    "ignore",
    "no_run",
    "should_panic",
    "compile_fail",
    "test_harness",
    "allow_fail",
    "edition2015",
    "edition2018",
    "edition2021",
    "edition2024",
];

/// The open code fence of a Markdown text: its character and its length.
#[derive(Clone, Copy)]
struct Fence {
    ch: char,
    len: usize,
}

/// A fence line: its fence and its info string.
fn fence_of(line: &str) -> Option<(Fence, &str)> {
    let trimmed = line.trim_start();
    let ch = trimmed.chars().next()?;
    if ch != '`' && ch != '~' {
        return None;
    }
    let len = trimmed.chars().take_while(|c| *c == ch).count();
    let info = trimmed[len..].trim();
    // A backtick fence has no backtick in its info string (else the line
    // starts with a code span)
    if len < 3 || ch == '`' && info.contains('`') {
        return None;
    }
    Some((Fence { ch, len }, info))
}

/// Whether a line closes the open fence.
fn closes(fence: Fence, line: &str) -> bool {
    matches!(fence_of(line), Some((f, info)) if f.ch == fence.ch && f.len >= fence.len && info.is_empty())
}

/// For each line of a Markdown text: whether it is in a code block
/// (fence lines included).
pub(crate) fn code_block_lines(lines: &[&str]) -> Vec<bool> {
    let mut open: Option<Fence> = None;
    lines
        .iter()
        .map(|line| match open {
            Some(fence) => {
                if closes(fence, line) {
                    open = None;
                }
                true
            }
            None => {
                if let Some((fence, _)) = fence_of(line) {
                    open = Some(fence);
                    true
                } else {
                    false
                }
            }
        })
        .collect()
}

/// Rustdoc code blocks as Markdown: a block with no language, or with only
/// rustdoc attributes (`ignore`, `rust,no_run`), gets the language `rust`,
/// and the hidden lines of a doc test (`# use foo;`, `#`) are left out
/// (`##` stands for a literal `#`).
fn normalize_rust_doc_code(doc: &str) -> String {
    let mut out: Vec<String> = Vec::new();
    // The open fence, and whether it holds Rust
    let mut open: Option<(Fence, bool)> = None;
    for line in doc.lines() {
        match open {
            Some((fence, is_rust)) => {
                if closes(fence, line) {
                    open = None;
                    out.push(line.to_string());
                    continue;
                }
                if is_rust {
                    let trimmed = line.trim_start();
                    let indent = &line[..line.len() - trimmed.len()];
                    if trimmed == "#" || trimmed.starts_with("# ") {
                        continue;
                    }
                    if let Some(rest) = trimmed.strip_prefix("##") {
                        out.push(format!("{}#{}", indent, rest));
                        continue;
                    }
                }
                out.push(line.to_string());
            }
            None => {
                if let Some((fence, info)) = fence_of(line) {
                    let is_rust = info.is_empty()
                        || info
                            .split(',')
                            .map(str::trim)
                            .all(|a| RUSTDOC_FENCE_ATTRIBUTES.contains(&a));
                    open = Some((fence, is_rust));
                    if is_rust && info != "rust" {
                        let indent = &line[..line.len() - line.trim_start().len()];
                        let marker: String = std::iter::repeat_n(fence.ch, fence.len).collect();
                        out.push(format!("{}{}rust", indent, marker));
                        continue;
                    }
                }
                out.push(line.to_string());
            }
        }
    }
    out.join("\n")
}

/// Split Rust doc lines at the section headings (`# Errors`) outside code
/// blocks: the text before the first heading (no heading), then each
/// heading with its body.
fn split_rust_sections<'a, 'b>(lines: &'b [&'a str]) -> Vec<(Option<&'a str>, &'b [&'a str])> {
    let in_code = code_block_lines(lines);
    let mut sections = Vec::new();
    let mut header: Option<&'a str> = None;
    let mut start = 0;
    for (i, line) in lines.iter().enumerate() {
        if in_code[i] {
            continue;
        }
        if let Some(name) = parse_markdown_header(line) {
            sections.push((header, &lines[start..i]));
            header = Some(name);
            start = i + 1;
        }
    }
    sections.push((header, &lines[start..]));
    sections
}

/// Whether a line starts a block that is not a paragraph: a list item, a
/// code fence, a table row, a quote or an HTML block.
fn starts_block(line: &str) -> bool {
    let trimmed = line.trim_start();
    if trimmed.starts_with("- ")
        || trimmed.starts_with("* ")
        || trimmed.starts_with("+ ")
        || trimmed.starts_with('|')
        || trimmed.starts_with('>')
        || trimmed.starts_with('<')
        || fence_of(line).is_some()
    {
        return true;
    }
    // Ordered list item: `1. ` or `1) `
    let digits = trimmed.chars().take_while(|c| c.is_ascii_digit()).count();
    digits > 0 && (trimmed[digits..].starts_with(". ") || trimmed[digits..].starts_with(") "))
}

/// Blank lines trimmed at both ends, and runs of blank lines (outside
/// code blocks) made one.
fn tidy_markdown_lines<S: AsRef<str>>(lines: &[S]) -> Vec<String> {
    let refs: Vec<&str> = lines.iter().map(|l| l.as_ref()).collect();
    let in_code = code_block_lines(&refs);
    let mut out: Vec<String> = Vec::new();
    for (i, line) in refs.iter().enumerate() {
        let blank = line.trim().is_empty();
        if blank && !in_code[i] {
            if out.last().is_none_or(|l| l.trim().is_empty()) {
                continue;
            }
            out.push(String::new());
        } else {
            out.push(line.trim_end().to_string());
        }
    }
    while out.last().is_some_and(|l| l.trim().is_empty()) {
        out.pop();
    }
    out
}

/// The summary (the first paragraph, as one line) and the description (the
/// rest, as Markdown) of Rust doc text.
fn split_rust_summary(text: &[String]) -> (Option<String>, Option<String>) {
    let lines = tidy_markdown_lines(text);

    let mut summary_lines = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        let line = &lines[i];
        if line.trim().is_empty() {
            i += 1;
            break;
        }
        if starts_block(line) || line.starts_with("**") && line.ends_with("**") {
            break;
        }
        summary_lines.push(line.trim());
        i += 1;
    }

    let summary = if summary_lines.is_empty() {
        None
    } else {
        Some(summary_lines.join(" "))
    };

    let rest = tidy_markdown_lines(&lines[i..]);
    let description = if rest.is_empty() {
        None
    } else {
        Some(rest.join("\n"))
    };

    (summary, description)
}

/// Parse a markdown header line, returning the section name if found
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

/// Parse the body of a Rust-style Arguments section
///
/// Expects format like:
/// ```text
/// * `name` - Description of the parameter
/// * `other` - Another parameter
/// ```
/// or
/// ```text
/// - `name`: Description
/// ```
fn parse_rust_arguments(lines: &[&str]) -> Vec<ParamDoc> {
    let mut params = Vec::new();
    let mut current_name = String::new();
    let mut current_desc = Vec::new();

    for line in lines {
        let trimmed = line.trim();

        // Empty line might end the section
        if trimmed.is_empty() {
            if !current_name.is_empty() {
                params.push(ParamDoc {
                    name: std::mem::take(&mut current_name),
                    ty: None,
                    description: current_desc.join(" ").trim().to_string(),
                });
                current_desc.clear();
            }
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
        } else if !current_name.is_empty() {
            // Continuation line
            current_desc.push(trimmed.to_string());
        }
    }

    // Don't forget the last one
    if !current_name.is_empty() {
        params.push(ParamDoc {
            name: current_name,
            ty: None,
            description: current_desc.join(" ").trim().to_string(),
        });
    }

    params
}

/// Parse a single Rust parameter line
/// Formats: `* `name` - description` or `- `name`: description` or `* name - description`
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

/// Parse the body of a Rust-style Returns section (kept as Markdown)
fn parse_rust_returns(lines: &[&str]) -> Option<ReturnDoc> {
    let text = tidy_markdown_lines(lines).join("\n");

    if text.is_empty() {
        return None;
    }

    Some(ReturnDoc {
        ty: None,
        description: text,
    })
}

/// Parse the body of a Rust-style Errors/Panics section
///
/// A list gives one entry per item; an item that starts with a type in
/// backticks (``* `IoError` - when ...``) gives that type. Text that is not a
/// list gives one entry of the kind, with its Markdown.
fn parse_rust_errors(lines: &[&str], error_kind: &str) -> Vec<RaisesDoc> {
    let is_item = |l: &str| {
        let t = l.trim_start();
        l.len() - t.len() < 2 && (t.starts_with("* ") || t.starts_with("- "))
    };
    if !lines.iter().any(|l| is_item(l)) {
        let text = tidy_markdown_lines(lines).join("\n");
        if text.is_empty() {
            return Vec::new();
        }
        return vec![RaisesDoc {
            ty: error_kind.to_string(),
            description: text,
        }];
    }

    let mut raises = Vec::new();
    let mut current: Option<(String, Vec<String>)> = None;
    let push = |current: &mut Option<(String, Vec<String>)>, raises: &mut Vec<RaisesDoc>| {
        if let Some((ty, desc)) = current.take() {
            raises.push(RaisesDoc {
                ty,
                description: desc.join(" ").trim().to_string(),
            });
        }
    };

    for line in lines {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            push(&mut current, &mut raises);
            continue;
        }

        if is_item(line) {
            push(&mut current, &mut raises);
            let rest = trimmed[1..].trim();

            // Try to extract error type from backticks
            let item = rest
                .strip_prefix('`')
                .and_then(|after_tick| {
                    let end_tick = after_tick.find('`')?;
                    let after = after_tick[end_tick + 1..].trim();
                    // A type is followed by a separator, not by more text
                    let desc = after
                        .strip_prefix('-')
                        .or_else(|| after.strip_prefix(':'))
                        .or_else(|| after.is_empty().then_some(""))?;
                    Some((after_tick[..end_tick].to_string(), desc.trim().to_string()))
                })
                .unwrap_or_else(|| (error_kind.to_string(), rest.to_string()));
            current = Some((item.0, vec![item.1]));
        } else if let Some((_, desc)) = current.as_mut() {
            // Continuation
            desc.push(trimmed.to_string());
        } else {
            // Plain text before the list
            current = Some((error_kind.to_string(), vec![trimmed.to_string()]));
        }
    }

    push(&mut current, &mut raises);
    raises
}

/// Parse the body of a Rust-style Examples section
fn parse_rust_examples(lines: &[&str]) -> Vec<String> {
    let mut examples = Vec::new();
    let mut current_example = Vec::new();
    let in_code = code_block_lines(lines);

    for (i, line) in lines.iter().enumerate() {
        // Empty line outside code block might separate examples
        if line.trim().is_empty() && !in_code[i] {
            if !current_example.is_empty() {
                examples.push(current_example.join("\n"));
                current_example.clear();
            }
            continue;
        }

        current_example.push(line.to_string());
    }

    if !current_example.is_empty() {
        examples.push(current_example.join("\n"));
    }

    examples
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty() {
        let result = parse_docstring("");
        assert!(result.is_empty());
    }

    #[test]
    fn test_parse_summary_only() {
        let docstring = "A simple summary.";
        let result = parse_docstring(docstring);

        assert_eq!(result.summary, Some("A simple summary.".to_string()));
        assert!(result.description.is_none());
        assert!(result.params.is_empty());
    }

    #[test]
    fn test_parse_summary_and_description() {
        let docstring = "A short summary.

This is a longer description that spans
multiple lines and provides more detail.";

        let result = parse_docstring(docstring);

        assert_eq!(result.summary, Some("A short summary.".to_string()));
        assert!(result.description.is_some());
        assert!(
            result
                .description
                .as_ref()
                .unwrap()
                .contains("longer description")
        );
    }

    #[test]
    fn test_parse_google_args() {
        let docstring = "Do something.

Args:
    name: The name of the thing.
    value (int): The value to use.
    optional: An optional parameter that
        spans multiple lines.
";

        let result = parse_docstring(docstring);

        assert_eq!(result.summary, Some("Do something.".to_string()));
        assert_eq!(result.params.len(), 3);

        assert_eq!(result.params[0].name, "name");
        assert!(result.params[0].ty.is_none());
        assert_eq!(result.params[0].description, "The name of the thing.");

        assert_eq!(result.params[1].name, "value");
        assert_eq!(result.params[1].ty, Some("int".to_string()));
        assert_eq!(result.params[1].description, "The value to use.");

        assert_eq!(result.params[2].name, "optional");
        assert!(result.params[2].description.contains("multiple lines"));
    }

    #[test]
    fn test_parse_google_returns() {
        let docstring = "Calculate result.

Returns:
    The calculated result as an integer.
";

        let result = parse_docstring(docstring);

        assert!(result.returns.is_some());
        let ret = result.returns.unwrap();
        assert!(ret.description.contains("calculated result"));
    }

    #[test]
    fn test_parse_google_returns_with_type() {
        let docstring = "Get value.

Returns:
    int: The integer value.
";

        let result = parse_docstring(docstring);

        assert!(result.returns.is_some());
        let ret = result.returns.unwrap();
        assert_eq!(ret.ty, Some("int".to_string()));
        assert_eq!(ret.description, "The integer value.");
    }

    #[test]
    fn test_parse_google_raises() {
        let docstring = "Do dangerous thing.

Raises:
    ValueError: If the value is invalid.
    RuntimeError: If something goes wrong
        during execution.
";

        let result = parse_docstring(docstring);

        assert_eq!(result.raises.len(), 2);
        assert_eq!(result.raises[0].ty, "ValueError");
        assert!(result.raises[0].description.contains("invalid"));
        assert_eq!(result.raises[1].ty, "RuntimeError");
        assert!(result.raises[1].description.contains("execution"));
    }

    #[test]
    fn test_parse_google_examples() {
        let docstring = "Do something.

Example:
    >>> x = do_something()
    >>> print(x)
    42
";

        let result = parse_docstring(docstring);

        assert_eq!(result.examples.len(), 1);
        assert!(result.examples[0].contains(">>> x = do_something()"));
    }

    #[test]
    fn test_parse_google_full() {
        let docstring = "Create a new task runner.

Args:
    max_parallel: Maximum number of concurrent tasks (default: 4).

Returns:
    A new Runner instance.

Raises:
    RuntimeError: If initialization fails.

Example:
    >>> runner = Runner(max_parallel=8)
";

        let result = parse_docstring(docstring);

        assert_eq!(
            result.summary,
            Some("Create a new task runner.".to_string())
        );
        assert_eq!(result.params.len(), 1);
        assert_eq!(result.params[0].name, "max_parallel");
        assert!(result.returns.is_some());
        assert_eq!(result.raises.len(), 1);
        assert_eq!(result.examples.len(), 1);
    }

    #[test]
    fn test_parse_numpy_style() {
        let docstring = "Calculate the mean.

Parameters
----------
values : array-like
    The values to average.
weights : array-like, optional
    Optional weights.

Returns
-------
float
    The weighted mean.

Raises
------
ValueError
    If arrays have different lengths.
";

        let result = parse_docstring(docstring);

        assert_eq!(result.summary, Some("Calculate the mean.".to_string()));

        assert_eq!(result.params.len(), 2);
        assert_eq!(result.params[0].name, "values");
        assert_eq!(result.params[0].ty, Some("array-like".to_string()));
        assert_eq!(result.params[1].name, "weights");

        assert!(result.returns.is_some());
        let ret = result.returns.unwrap();
        assert_eq!(ret.ty, Some("float".to_string()));

        assert_eq!(result.raises.len(), 1);
        assert_eq!(result.raises[0].ty, "ValueError");
    }

    #[test]
    fn test_parse_rust_docstring() {
        // Rust docstrings are similar to Google style
        let docstring = "Create a new task.

Args:
    name: The unique identifier for this task.
    description: Optional human-readable description.

Returns:
    A new Task instance.";

        let result = parse_docstring(docstring);

        assert_eq!(result.summary, Some("Create a new task.".to_string()));
        assert_eq!(result.params.len(), 2);
        assert!(result.returns.is_some());
    }

    #[test]
    fn test_detect_google_style() {
        assert_eq!(
            detect_style("Summary.\n\nArgs:\n    x: value"),
            DocstringStyle::Google
        );
        assert_eq!(
            detect_style("Summary.\n\nReturns:\n    value"),
            DocstringStyle::Google
        );
    }

    #[test]
    fn test_detect_numpy_style() {
        assert_eq!(
            detect_style("Summary.\n\nParameters\n----------\n"),
            DocstringStyle::NumPy
        );
    }

    #[test]
    fn test_detect_plain_style() {
        assert_eq!(
            detect_style("Just a simple docstring."),
            DocstringStyle::Plain
        );
    }

    #[test]
    fn test_parse_scheduler_docstring() {
        // Test with the actual scheduler.py docstring
        let docstring = r#"A task scheduler that runs tasks on configured schedules.

The scheduler supports both interval-based and cron-based scheduling.
Tasks are registered using the `@scheduler.task()` decorator.

Attributes:
    tasks: Dictionary of registered tasks by name.
    running: Whether the scheduler is currently running.

Example:
    >>> scheduler = Scheduler()
    >>> @scheduler.task(every(seconds=30))
    ... def heartbeat():
    ...     print("alive")
    >>> scheduler.run()"#;

        let result = parse_docstring(docstring);

        assert!(result.summary.is_some());
        assert!(result.summary.as_ref().unwrap().contains("task scheduler"));
        assert!(result.description.is_some());
        assert_eq!(result.examples.len(), 1);
    }

    #[test]
    fn test_parse_fixture_method_docstring() {
        let docstring = r#"Decorator to register a function as a scheduled task.

Args:
    schedule: When to run the task (use `every()` or `cron()`).
    name: Optional task name. Defaults to the function name.
    max_retries: Number of retry attempts on failure.
    timeout_seconds: Maximum execution time before killing the task.

Returns:
    A decorator that registers the function.

Raises:
    ValueError: If a task with this name already exists.

Example:
    >>> @scheduler.task(every(hours=1), max_retries=3)
    ... def sync_data():
    ...     external_api.sync()"#;

        let result = parse_docstring(docstring);

        assert_eq!(result.params.len(), 4);
        assert_eq!(result.params[0].name, "schedule");
        assert!(result.returns.is_some());
        assert_eq!(result.raises.len(), 1);
        assert_eq!(result.raises[0].ty, "ValueError");
        assert_eq!(result.examples.len(), 1);
    }

    // ========================================================================
    // Rust Doc Comment Parser Tests
    // ========================================================================

    #[test]
    fn test_parse_rust_doc_empty() {
        let result = parse_rust_doc("");
        assert!(result.is_empty());
    }

    #[test]
    fn test_parse_rust_doc_summary_only() {
        let doc = "Returns the length of the string.";
        let result = parse_rust_doc(doc);

        assert_eq!(
            result.summary,
            Some("Returns the length of the string.".to_string())
        );
        assert!(result.description.is_none());
    }

    #[test]
    fn test_parse_rust_doc_summary_and_description() {
        let doc = "Returns the length of the string.

This is a longer description that provides
more detail about how the function works.";

        let result = parse_rust_doc(doc);

        assert_eq!(
            result.summary,
            Some("Returns the length of the string.".to_string())
        );
        assert!(result.description.is_some());
        assert!(
            result
                .description
                .as_ref()
                .unwrap()
                .contains("longer description")
        );
    }

    #[test]
    fn test_parse_rust_doc_arguments() {
        let doc = "Creates a new buffer.

# Arguments

* `capacity` - The initial capacity of the buffer
* `fill` - The value to fill the buffer with";

        let result = parse_rust_doc(doc);

        assert_eq!(result.params.len(), 2);
        assert_eq!(result.params[0].name, "capacity");
        assert!(result.params[0].description.contains("initial capacity"));
        assert_eq!(result.params[1].name, "fill");
    }

    #[test]
    fn test_parse_rust_doc_arguments_backticks() {
        let doc = "Process data.

# Arguments

* `data` - The data to process
* `options` - Processing options";

        let result = parse_rust_doc(doc);

        assert_eq!(result.params.len(), 2);
        assert_eq!(result.params[0].name, "data");
        assert_eq!(result.params[1].name, "options");
    }

    #[test]
    fn test_parse_rust_doc_returns() {
        let doc = "Computes the hash.

# Returns

The computed hash value as a 64-bit integer.";

        let result = parse_rust_doc(doc);

        assert!(result.returns.is_some());
        let ret = result.returns.unwrap();
        assert!(ret.description.contains("64-bit integer"));
    }

    #[test]
    fn test_parse_rust_doc_errors() {
        let doc = "Opens a file.

# Errors

Returns an error if the file does not exist or
cannot be opened.";

        let result = parse_rust_doc(doc);

        assert_eq!(result.raises.len(), 1);
        assert_eq!(result.raises[0].ty, "Error");
        assert!(result.raises[0].description.contains("file does not exist"));
    }

    #[test]
    fn test_parse_rust_doc_errors_with_types() {
        let doc = "Parses the input.

# Errors

* `ParseError` - If the input is malformed
* `IoError` - If reading fails";

        let result = parse_rust_doc(doc);

        assert_eq!(result.raises.len(), 2);
        assert_eq!(result.raises[0].ty, "ParseError");
        assert!(result.raises[0].description.contains("malformed"));
        assert_eq!(result.raises[1].ty, "IoError");
    }

    #[test]
    fn test_parse_rust_doc_panics() {
        let doc = "Gets the element.

# Panics

Panics if the index is out of bounds.";

        let result = parse_rust_doc(doc);

        assert_eq!(result.raises.len(), 1);
        assert_eq!(result.raises[0].ty, "Panic");
        assert!(result.raises[0].description.contains("out of bounds"));
    }

    #[test]
    fn test_parse_rust_doc_examples() {
        let doc = r#"Creates a new instance.

# Examples

```rust
let x = MyType::new();
assert!(x.is_valid());
```"#;

        let result = parse_rust_doc(doc);

        assert_eq!(result.examples.len(), 1);
        assert!(result.examples[0].contains("let x = MyType::new()"));
        assert!(result.examples[0].contains("```"));
    }

    #[test]
    fn test_parse_rust_doc_safety() {
        let doc = "Dereferences a raw pointer.

# Safety

The pointer must be valid and properly aligned.
The caller must ensure the pointed-to data is valid.";

        let result = parse_rust_doc(doc);

        assert!(result.description.is_some());
        assert!(result.description.as_ref().unwrap().contains("Safety"));
        assert!(
            result
                .description
                .as_ref()
                .unwrap()
                .contains("pointer must be valid")
        );
    }

    #[test]
    fn test_parse_rust_doc_full() {
        let doc = r#"Processes the input data and returns the result.

This function performs complex processing on the input,
applying various transformations.

# Arguments

* `input` - The input data to process
* `config` - Configuration options

# Returns

The processed result, or an error if processing fails.

# Errors

* `InvalidInput` - If the input is malformed
* `ProcessingError` - If processing fails

# Panics

Panics if the config is invalid.

# Examples

```rust
let result = process(&data, &config)?;
println!("{:?}", result);
```"#;

        let result = parse_rust_doc(doc);

        assert!(result.summary.is_some());
        assert!(
            result
                .summary
                .as_ref()
                .unwrap()
                .contains("Processes the input")
        );
        assert!(result.description.is_some());
        assert_eq!(result.params.len(), 2);
        assert_eq!(result.params[0].name, "input");
        assert!(result.returns.is_some());
        assert_eq!(result.raises.len(), 3); // 2 errors + 1 panic
        assert_eq!(result.examples.len(), 1);
    }

    #[test]
    fn test_parse_rust_doc_no_sections() {
        // Plain doc with no sections should still work
        let doc = "A simple function that does something useful.";
        let result = parse_rust_doc(doc);

        assert_eq!(
            result.summary,
            Some("A simple function that does something useful.".to_string())
        );
        assert!(result.params.is_empty());
        assert!(result.returns.is_none());
    }

    #[test]
    fn test_parse_rust_doc_keeps_paragraphs_after_a_list() {
        // Item 5 of PLSKN-T-0057: the paragraph after a list joined its last item
        let doc = "Validate configuration semantically.

Performs validation beyond TOML parsing:
- At least one language section must be configured
- Configured paths must exist

Returns `Ok(ValidationResult)` with any warnings.";
        let result = parse_rust_doc(doc);
        assert_eq!(
            result.summary.as_deref(),
            Some("Validate configuration semantically.")
        );
        assert_eq!(
            result.description.as_deref(),
            Some(
                "Performs validation beyond TOML parsing:\n\
                 - At least one language section must be configured\n\
                 - Configured paths must exist\n\
                 \n\
                 Returns `Ok(ValidationResult)` with any warnings."
            )
        );
    }

    #[test]
    fn test_parse_rust_doc_summary_stops_at_a_list() {
        let doc = "Steps:\n- one\n- two";
        let result = parse_rust_doc(doc);
        assert_eq!(result.summary.as_deref(), Some("Steps:"));
        assert_eq!(result.description.as_deref(), Some("- one\n- two"));
    }

    #[test]
    fn test_parse_rust_doc_code_blocks_are_rust() {
        // Item 12: a bare fence holds Rust; hidden doc test lines are left out
        let doc = "Make one.

```
# use my_crate::Thing;
let t = Thing::new();
## not hidden
```

```ignore
broken();
```

```toml
# a TOML comment, not a section
key = 1
```";
        let result = parse_rust_doc(doc);
        let desc = result.description.unwrap();
        assert!(desc.starts_with("```rust\nlet t = Thing::new();\n# not hidden\n```"));
        assert!(!desc.contains("use my_crate"));
        assert!(desc.contains("```rust\nbroken();\n```"));
        assert!(desc.contains("```toml\n# a TOML comment, not a section\nkey = 1\n```"));
    }

    #[test]
    fn test_parse_rust_doc_other_sections_stay() {
        let doc = "Run it.

# Overview

Text of the overview.

# Errors

Fails if the file is missing.";
        let result = parse_rust_doc(doc);
        assert_eq!(
            result.description.as_deref(),
            Some("**Overview**\n\nText of the overview.")
        );
        assert_eq!(result.raises.len(), 1);
        assert_eq!(result.raises[0].ty, RUST_ERROR_KIND);
    }

    #[test]
    fn test_code_span_at_line_start_is_not_a_fence() {
        let doc = "Summary.\n\n```` ```rust,no_run ```` holds Rust code.\n\n# Errors\n\nFails.";
        let result = parse_rust_doc(doc);
        assert_eq!(
            result.description.as_deref(),
            Some("```` ```rust,no_run ```` holds Rust code.")
        );
        assert_eq!(result.raises.len(), 1);
    }

    #[test]
    fn test_normalize_rust_doc() {
        assert_eq!(
            normalize_rust_doc("A field.\n\n\n```\nlet x = 1;\n```\n"),
            "A field.\n\n```rust\nlet x = 1;\n```"
        );
    }

    #[test]
    fn test_parse_google_description_keeps_paragraphs() {
        let doc = "Summary line.

    First paragraph
    continues here.

    Second paragraph:

        indented code

Args:
    x: A value.";
        let result = parse_docstring(doc);
        assert_eq!(
            result.description.as_deref(),
            Some("First paragraph\ncontinues here.\n\nSecond paragraph:\n\n    indented code")
        );
        assert_eq!(result.params.len(), 1);
    }

    #[test]
    fn test_parse_markdown_header() {
        assert_eq!(parse_markdown_header("# Arguments"), Some("Arguments"));
        assert_eq!(parse_markdown_header("## Returns"), Some("Returns"));
        assert_eq!(parse_markdown_header("### Examples"), Some("Examples"));
        assert_eq!(parse_markdown_header("Not a header"), None);
        assert_eq!(parse_markdown_header("#NoSpace"), None);
    }
}
