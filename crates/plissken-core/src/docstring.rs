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
    let mut in_code_block = false;
    let mut i = start;

    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();

        // Check for new section header (but not if we're in a code block)
        if !in_code_block && trimmed.ends_with(':') && !trimmed.contains(' ') {
            let section = &trimmed[..trimmed.len() - 1].to_lowercase();
            if is_known_section(section) {
                break;
            }
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

/// Parse a Rust doc comment into structured form
///
/// Looks for conventional markdown sections:
/// - `# Arguments` / `# Parameters` - function parameters
/// - `# Returns` - return value documentation
/// - `# Errors` - error conditions (maps to raises)
/// - `# Panics` - panic conditions (maps to raises)
/// - `# Safety` - safety requirements (stored in description)
/// - `# Examples` - code examples
///
/// If parsing fails or no sections are found, returns a basic ParsedDocstring
/// with just the summary/description extracted.
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

/// Extract summary and description from Rust doc before any # sections
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

/// Parse Rust-style Arguments section
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

/// Parse Rust-style Returns section
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

/// Parse Rust-style Errors/Panics section
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

/// Parse Rust-style Examples section
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

/// Parse a section as plain text until next section
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
    fn test_parse_google_examples_with_code_fence() {
        // Test that code fences inside examples are preserved as a single example
        // and not split at empty lines within the fence
        let docstring = r#"A data processing pipeline.

Example:
    ```python
    from separate_bindings import Pipeline, DataBatch

    pipeline = Pipeline("etl")
    pipeline.add_stage("transform", lambda batch: batch)

    result = pipeline.run(DataBatch.from_dicts([{"a": 1}]))
    print(f"Processed {result.rows_out} rows")
    ```
"#;

        let result = parse_docstring(docstring);

        // Should be exactly one example, not split at the empty lines
        assert_eq!(result.examples.len(), 1);
        // Should contain the code fence markers
        assert!(result.examples[0].contains("```python"));
        assert!(result.examples[0].contains("```"));
        // Should contain all the code
        assert!(result.examples[0].contains("Pipeline"));
        assert!(result.examples[0].contains("rows_out"));
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
    fn test_parse_markdown_header() {
        assert_eq!(parse_markdown_header("# Arguments"), Some("Arguments"));
        assert_eq!(parse_markdown_header("## Returns"), Some("Returns"));
        assert_eq!(parse_markdown_header("### Examples"), Some("Examples"));
        assert_eq!(parse_markdown_header("Not a header"), None);
        assert_eq!(parse_markdown_header("#NoSpace"), None);
    }
}
