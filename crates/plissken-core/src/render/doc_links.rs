//! Intra-doc links of Rust doc comments
//!
//! Rustdoc resolves links whose target is a Rust path: ``[`Config`]``,
//! ``[`Config`](crate::config::Config)``, `[handler](super::handler)`, or a
//! reference definition `[Config]: crate::config::Config`. Markdown knows
//! none of them: it shows a bare link as literal brackets, and an explicit
//! one as a relative URL that does not exist. This module rewrites them:
//! a link that the resolver finds points to its page, any other one becomes
//! its text.

use crate::docstring::code_block_lines;
use std::collections::HashMap;

/// Disambiguator prefixes of rustdoc (`struct@Foo`).
const DISAMBIGUATORS: &[&str] = &[
    "struct",
    "enum",
    "trait",
    "union",
    "fn",
    "mod",
    "module",
    "const",
    "static",
    "type",
    "macro",
    "method",
    "field",
    "variant",
    "tymethod",
    "derive",
    "prim",
    "primitive",
    "value",
];

/// The Rust path of an intra-doc link target, without its disambiguator
/// (`struct@`) and suffix (`()`, `!`). `None` if the target is not a Rust
/// path (a URL, a file, an anchor).
pub fn intra_doc_path(target: &str) -> Option<&str> {
    let mut path = target.trim();
    if let Some((prefix, rest)) = path.split_once('@')
        && DISAMBIGUATORS.contains(&prefix)
    {
        path = rest;
    }
    for suffix in ["!()", "()", "!"] {
        if let Some(rest) = path.strip_suffix(suffix) {
            path = rest;
            break;
        }
    }
    let body = path.strip_prefix("::").unwrap_or(path);
    let valid = !body.is_empty()
        && body.split("::").all(|seg| {
            let mut chars = seg.chars();
            chars
                .next()
                .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
                && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
        });
    valid.then_some(path)
}

/// Whether the text of a bare link (`[Foo]`) is meant as an intra-doc link:
/// code (``[`foo`]``), or a path that is not a plain word (`Foo`,
/// `foo::bar`, `foo()`).
fn bare_link_path(text: &str) -> Option<&str> {
    if let Some(code) = text.strip_prefix('`').and_then(|t| t.strip_suffix('`')) {
        return intra_doc_path(code);
    }
    let path = intra_doc_path(text)?;
    let looks_like_item = path.contains("::")
        || path.chars().next().is_some_and(|c| c.is_ascii_uppercase())
        || path.len() < text.trim().len();
    looks_like_item.then_some(path)
}

/// Rewrite the intra-doc links of Rust doc Markdown.
///
/// `resolve` gets the Rust path of a link and gives its href, or `None`.
/// Code blocks and code spans are left as they are, and so are links to
/// URLs. A reference definition to a Rust path is removed.
pub fn rewrite_intra_doc_links(markdown: &str, resolve: &dyn Fn(&str) -> Option<String>) -> String {
    let lines: Vec<&str> = markdown.lines().collect();
    let in_code = code_block_lines(&lines);

    // Reference definitions: `[label]: target`
    let mut rust_defs: HashMap<String, String> = HashMap::new();
    let mut url_defs: Vec<String> = Vec::new();
    let mut is_def = vec![false; lines.len()];
    for (i, line) in lines.iter().enumerate() {
        if in_code[i] {
            continue;
        }
        if let Some((label, target)) = reference_definition(line) {
            match intra_doc_path(target) {
                Some(_) => {
                    rust_defs.insert(label.to_lowercase(), target.to_string());
                    is_def[i] = true;
                }
                None => url_defs.push(label.to_lowercase()),
            }
        }
    }

    let rewriter = Rewriter {
        rust_defs,
        url_defs,
        resolve,
    };
    let mut out: Vec<String> = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        if is_def[i] {
            continue;
        }
        if in_code[i] {
            out.push(line.to_string());
        } else {
            out.push(rewriter.line(line));
        }
    }
    // A removed definition at the end leaves blank lines
    while out.last().is_some_and(|l| l.trim().is_empty()) {
        out.pop();
    }
    out.join("\n")
}

/// A reference definition line: `[label]: target`.
fn reference_definition(line: &str) -> Option<(&str, &str)> {
    let trimmed = line.trim_start();
    if line.len() - trimmed.len() > 3 {
        return None;
    }
    let rest = trimmed.strip_prefix('[')?;
    let end = rest.find("]:")?;
    let label = &rest[..end];
    let target = rest[end + 2..].trim();
    if label.is_empty() || target.is_empty() || target.contains(char::is_whitespace) {
        return None;
    }
    Some((label, target.trim_start_matches('<').trim_end_matches('>')))
}

struct Rewriter<'a> {
    rust_defs: HashMap<String, String>,
    url_defs: Vec<String>,
    resolve: &'a dyn Fn(&str) -> Option<String>,
}

impl Rewriter<'_> {
    /// The link to a Rust path with the given text, or the text alone.
    fn link(&self, text: &str, path: &str) -> String {
        match (self.resolve)(path) {
            Some(href) => format!("[{}]({})", text, href),
            None => text.to_string(),
        }
    }

    /// A link to the target of a reference definition, if it is a Rust path.
    fn defined(&self, text: &str, label: &str) -> Option<String> {
        let target = self.rust_defs.get(&label.to_lowercase())?;
        let path = intra_doc_path(target)?;
        Some(self.link(text, path))
    }

    fn line(&self, line: &str) -> String {
        let chars: Vec<char> = line.chars().collect();
        let mut out = String::with_capacity(line.len());
        let mut i = 0;
        while i < chars.len() {
            let c = chars[i];
            if c == '`' {
                let end = code_span_end(&chars, i);
                out.extend(&chars[i..end]);
                i = end;
                continue;
            }
            if c == '\\' && i + 1 < chars.len() {
                out.push(c);
                out.push(chars[i + 1]);
                i += 2;
                continue;
            }
            let is_image = i > 0 && chars[i - 1] == '!';
            if c == '['
                && !is_image
                && let Some(close) = bracket_end(&chars, i)
            {
                let text: String = chars[i + 1..close].iter().collect();
                let next = chars.get(close + 1).copied();
                if next == Some('(')
                    && let Some(paren) = paren_end(&chars, close + 1)
                {
                    // Inline link: [text](target)
                    let target: String = chars[close + 2..paren].iter().collect();
                    if let Some(path) = intra_doc_path(&target) {
                        out.push_str(&self.link(&text, path));
                    } else {
                        out.extend(&chars[i..=paren]);
                    }
                    i = paren + 1;
                    continue;
                }
                if next == Some('[')
                    && let Some(label_end) = bracket_end(&chars, close + 1)
                {
                    // Reference link: [text][label], [text][]
                    let label: String = chars[close + 2..label_end].iter().collect();
                    let label = if label.is_empty() {
                        text.clone()
                    } else {
                        label
                    };
                    let rewritten = if self.url_defs.contains(&label.to_lowercase()) {
                        None
                    } else {
                        self.defined(&text, &label)
                            .or_else(|| bare_link_path(&label).map(|path| self.link(&text, path)))
                    };
                    match rewritten {
                        Some(s) => out.push_str(&s),
                        None => out.extend(&chars[i..=label_end]),
                    }
                    i = label_end + 1;
                    continue;
                }
                // Shortcut link: [text]
                let rewritten = if self.url_defs.contains(&text.to_lowercase()) {
                    None
                } else {
                    self.defined(&text, &text)
                        .or_else(|| bare_link_path(&text).map(|path| self.link(&text, path)))
                };
                match rewritten {
                    Some(s) => out.push_str(&s),
                    None => out.extend(&chars[i..=close]),
                }
                i = close + 1;
                continue;
            }
            out.push(c);
            i += 1;
        }
        out
    }
}

/// The end (exclusive) of the code span that starts at `start`, or the end
/// of the backtick run if it is not closed on the line.
fn code_span_end(chars: &[char], start: usize) -> usize {
    let run = chars[start..].iter().take_while(|c| **c == '`').count();
    let mut j = start + run;
    while j < chars.len() {
        if chars[j] == '`' {
            let len = chars[j..].iter().take_while(|c| **c == '`').count();
            if len == run {
                return j + len;
            }
            j += len;
        } else {
            j += 1;
        }
    }
    start + run
}

/// The index of the `]` that closes the `[` at `start`, skipping code spans
/// and nested brackets.
fn bracket_end(chars: &[char], start: usize) -> Option<usize> {
    let mut depth = 0;
    let mut j = start;
    while j < chars.len() {
        match chars[j] {
            '`' => {
                j = code_span_end(chars, j);
                continue;
            }
            '\\' => j += 1,
            '[' => depth += 1,
            ']' => {
                depth -= 1;
                if depth == 0 {
                    return Some(j);
                }
            }
            _ => {}
        }
        j += 1;
    }
    None
}

/// The index of the `)` that closes the `(` at `start`.
fn paren_end(chars: &[char], start: usize) -> Option<usize> {
    let mut depth = 0;
    for (j, c) in chars.iter().enumerate().skip(start) {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(j);
                }
            }
            c if c.is_whitespace() => return None,
            _ => {}
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn none(_: &str) -> Option<String> {
        None
    }

    fn known(path: &str) -> Option<String> {
        match path {
            "Config" | "crate::config::Config" => Some("../config/#struct.Config".to_string()),
            "super::handler" => Some("../handler/".to_string()),
            _ => None,
        }
    }

    #[test]
    fn test_intra_doc_path() {
        assert_eq!(
            intra_doc_path("crate::config::Config"),
            Some("crate::config::Config")
        );
        assert_eq!(intra_doc_path("struct@Config"), Some("Config"));
        assert_eq!(intra_doc_path("parse()"), Some("parse"));
        assert_eq!(intra_doc_path("vec!"), Some("vec"));
        assert_eq!(intra_doc_path("super::handler"), Some("super::handler"));
        assert_eq!(intra_doc_path("https://example.com"), None);
        assert_eq!(intra_doc_path("../guide/"), None);
        assert_eq!(intra_doc_path("#anchor"), None);
        assert_eq!(intra_doc_path("file.md"), None);
        assert_eq!(intra_doc_path(""), None);
    }

    #[test]
    fn test_bare_links_become_code() {
        // Item 10 of PLSKN-T-0057: bare [`X`] showed as literal brackets
        assert_eq!(
            rewrite_intra_doc_links("See [`Config`] and [`parse()`].", &none),
            "See `Config` and `parse()`."
        );
        assert_eq!(
            rewrite_intra_doc_links("See [Config] and [crate::Foo].", &none),
            "See Config and crate::Foo."
        );
    }

    #[test]
    fn test_plain_words_and_urls_are_kept() {
        let text = "A list [optional], a task [x], [a link](https://example.com), \
                    [a page](../guide/), ![img](pic.png), `code [Foo]`.";
        assert_eq!(rewrite_intra_doc_links(text, &none), text);
    }

    #[test]
    fn test_explicit_links() {
        // Items 9 and 21: Rust paths as targets are not URLs
        assert_eq!(
            rewrite_intra_doc_links(
                "[`Config`](crate::config::Config), [h](super::handler)",
                &none
            ),
            "`Config`, h"
        );
        assert_eq!(
            rewrite_intra_doc_links(
                "[`Config`](crate::config::Config), [h](super::handler)",
                &known
            ),
            "[`Config`](../config/#struct.Config), [h](../handler/)"
        );
    }

    #[test]
    fn test_reference_definitions() {
        let text = "Uses [the config][cfg] and [Config].\n\n[cfg]: crate::config::Config\n[Config]: crate::config::Config";
        assert_eq!(
            rewrite_intra_doc_links(text, &known),
            "Uses [the config](../config/#struct.Config) and [Config](../config/#struct.Config)."
        );
        // A definition to a URL stays
        let text = "See [docs].\n\n[docs]: https://example.com";
        assert_eq!(rewrite_intra_doc_links(text, &known), text);
    }

    #[test]
    fn test_code_blocks_are_kept() {
        let text = "```rust\nlet x = v[`a`];\n[Foo]: bar\n```";
        assert_eq!(rewrite_intra_doc_links(text, &none), text);
    }
}
