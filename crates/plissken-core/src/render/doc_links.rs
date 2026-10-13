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
use crate::model::{RustItem, RustModule};
use std::collections::{HashMap, HashSet};

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

/// Crates of the Rust distribution: a path into them is never an item of
/// the project.
const EXTERNAL_CRATES: &[&str] = &["std", "core", "alloc", "proc_macro", "test"];

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

/// Where a Rust item is documented: its page (relative to the output
/// folder) and its anchor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocTarget {
    /// Page file path (`rust/my_crate/config.md`)
    pub page: String,
    /// Explicit anchor id of the item (`struct.Config`); `None` for a module
    pub id: Option<String>,
    /// Anchor of the item heading in MkDocs and mdBook (`struct-config`)
    pub md_anchor: Option<String>,
}

/// The Rust items of the documentation, by path, to resolve intra-doc links.
#[derive(Debug, Clone, Default)]
pub struct RustDocIndex {
    items: HashMap<String, DocTarget>,
    modules: HashSet<String>,
    /// The `pub use` of each module: the name (`*` for a glob) and the
    /// path as written
    reexports: HashMap<String, Vec<(String, String)>>,
}

/// How many `pub use` steps a path may follow.
const MAX_REEXPORT_DEPTH: usize = 8;

/// A Rust path for comparison: a crate name has `_` where Cargo allows `-`.
fn key(path: &str) -> String {
    match path.split_once("::") {
        Some((krate, rest)) => format!("{}::{}", krate.replace('-', "_"), rest),
        None => path.replace('-', "_"),
    }
}

impl RustDocIndex {
    /// Index the items of the modules that are rendered. `page_of` gives
    /// the page file of a module path.
    pub fn new(modules: &[RustModule], page_of: impl Fn(&str) -> String) -> Self {
        let mut index = Self::default();
        for module in modules {
            let page = page_of(&module.path);
            let module_key = key(&module.path);
            index.modules.insert(module_key.clone());
            index.items.insert(
                module_key.clone(),
                DocTarget {
                    page: page.clone(),
                    id: None,
                    md_anchor: None,
                },
            );
            let mut add = |name: &str, id: String, md: String| {
                index
                    .items
                    .entry(format!("{}::{}", module_key, name))
                    .or_insert(DocTarget {
                        page: page.clone(),
                        id: Some(id),
                        md_anchor: Some(md),
                    });
            };
            for item in &module.items {
                match item {
                    RustItem::Struct(s) => {
                        let lower = s.name.to_lowercase();
                        add(
                            &s.name,
                            format!("struct.{}", s.name),
                            format!("struct-{}", lower),
                        );
                    }
                    RustItem::Enum(e) => {
                        let lower = e.name.to_lowercase();
                        add(
                            &e.name,
                            format!("enum.{}", e.name),
                            format!("enum-{}", lower),
                        );
                        for v in &e.variants {
                            add(
                                &format!("{}::{}", e.name, v.name),
                                format!("enum.{}", e.name),
                                format!("enum-{}", lower),
                            );
                        }
                    }
                    RustItem::Function(f) => {
                        add(
                            &f.name,
                            format!("fn.{}", f.name),
                            format!("fn-{}", f.name.to_lowercase()),
                        );
                    }
                    RustItem::Trait(t) => {
                        let lower = t.name.to_lowercase();
                        add(
                            &t.name,
                            format!("trait.{}", t.name),
                            format!("trait-{}", lower),
                        );
                        for m in &t.methods {
                            add(
                                &format!("{}::{}", t.name, m.name),
                                format!("{}.{}", t.name, m.name),
                                m.name.to_lowercase(),
                            );
                        }
                    }
                    RustItem::TypeAlias(t) => {
                        add(
                            &t.name,
                            format!("type.{}", t.name),
                            "type-aliases".to_string(),
                        );
                    }
                    RustItem::Const(c) => {
                        add(&c.name, format!("const.{}", c.name), c.name.to_lowercase());
                    }
                    RustItem::Impl(i) if i.trait_.is_none() => {
                        let target = i.target.split('<').next().unwrap_or(&i.target).trim();
                        for m in &i.methods {
                            add(
                                &format!("{}::{}", target, m.name),
                                format!("{}.{}", target, m.name),
                                m.name.to_lowercase(),
                            );
                        }
                    }
                    _ => {}
                }
            }
        }
        // A re-exported name points where its path points (see `lookup`)
        for module in modules {
            let reexports = index.reexports.entry(key(&module.path)).or_default();
            for r in &module.reexports {
                reexports.push((r.name.clone(), r.path.clone()));
            }
        }
        index
    }

    /// The module of an item path: the longest prefix that is a module.
    pub fn module_of(&self, item_path: &str) -> String {
        let mut path = item_path.to_string();
        while let Some((parent, _)) = path.rsplit_once("::") {
            path = parent.to_string();
            if self.modules.contains(&key(&path)) {
                return path;
            }
        }
        path
    }

    /// Resolve an intra-doc link path written in the module `scope`.
    ///
    /// The rules of rustdoc, within what plissken knows. `crate`, `self` and
    /// `super` are taken from the scope. A path is tried in the scope (with
    /// its `pub use` names), as written, and from the root of the crate.
    /// Then, for private `use` imports, which plissken does not see, the
    /// whole path is looked up after any module: the one item of the crate,
    /// else of the project. So `Frame::Error` finds the variant `Error` of
    /// `Frame`, but a bare `Error` finds no variant or method.
    ///
    /// A path into a crate outside the project (`std::io::Error`), a path
    /// from a crate root (`::x::Y`), and a name that is ambiguous or not
    /// found give `None`: the link shows as its text.
    pub fn resolve(&self, scope: &str, path: &str) -> Option<&DocTarget> {
        let written = WrittenPath::new(scope, path)?;
        if let Some(found) = written
            .candidates()
            .iter()
            .find_map(|c| self.lookup(c, MAX_REEXPORT_DEPTH))
        {
            return self.items.get(&found);
        }

        // Paths from a crate root or into the standard crates are not ours
        let rest = written.rest();
        if written.absolute || rest.is_empty() {
            return None;
        }
        if written.base.is_none()
            && written.parts.len() > 1
            && EXTERNAL_CRATES.contains(&written.parts[0])
        {
            return None;
        }

        // By name: the item whose path is a module and then the whole path
        // as written; the one in the crate, else the one in the project.
        // `crate::`, `self::` and `super::` paths stay in the crate.
        let in_crate = format!("{}::", written.krate);
        for restrict in [true, false] {
            if !restrict && written.base.is_some() {
                break;
            }
            let mut found: Vec<&DocTarget> = Vec::new();
            for (k, target) in &self.items {
                let after_module = k
                    .strip_suffix(rest.as_str())
                    .and_then(|m| m.strip_suffix("::"))
                    .is_some_and(|m| self.modules.contains(m));
                if after_module
                    && !self.modules.contains(k.as_str())
                    && (!restrict || k.starts_with(&in_crate))
                    && !found.contains(&target)
                {
                    found.push(target);
                }
            }
            match found.as_slice() {
                [] => {}
                [target] => return Some(target),
                _ => return None,
            }
        }
        None
    }

    /// The key of the item that a path names exactly, written in `scope`.
    fn exact(&self, scope: &str, path: &str, depth: usize) -> Option<String> {
        let written = WrittenPath::new(scope, path)?;
        written
            .candidates()
            .iter()
            .find_map(|c| self.lookup(c, depth))
    }

    /// The key of the item at an absolute path, through the `pub use` of
    /// its modules (`a::b::C` where `a` re-exports the module `b`).
    fn lookup(&self, path: &str, depth: usize) -> Option<String> {
        if self.items.contains_key(path) {
            return Some(path.to_string());
        }
        if depth == 0 {
            return None;
        }
        let (parent, name) = path.rsplit_once("::")?;
        let parent = self.lookup(parent, depth - 1)?;
        let direct = format!("{}::{}", parent, name);
        if self.items.contains_key(&direct) {
            return Some(direct);
        }
        let reexports = self.reexports.get(&parent)?;
        reexports
            .iter()
            .filter(|(n, _)| n == name)
            .find_map(|(_, p)| self.exact(&parent, p, depth - 1))
            .or_else(|| {
                reexports
                    .iter()
                    .filter(|(n, _)| n == "*")
                    .find_map(|(_, p)| self.exact(&parent, &format!("{}::{}", p, name), depth - 1))
            })
    }
}

/// A Rust path as written in a module, with `crate`, `self` and `super`
/// taken from the module.
struct WrittenPath<'a> {
    scope_parts: Vec<String>,
    krate: String,
    parts: Vec<&'a str>,
    absolute: bool,
    base: Option<Vec<String>>,
}

impl<'a> WrittenPath<'a> {
    fn new(scope: &str, path: &'a str) -> Option<Self> {
        let scope = key(scope);
        let scope_parts: Vec<String> = if scope.is_empty() {
            Vec::new()
        } else {
            scope.split("::").map(String::from).collect()
        };
        let krate = scope_parts.first().cloned().unwrap_or_default();

        let mut parts: Vec<&str> = path.split("::").collect();
        let absolute = parts.first() == Some(&"");
        if absolute {
            parts.remove(0);
        }
        let mut base: Option<Vec<String>> = None;
        match parts.first().copied() {
            Some("crate") => {
                base = Some(vec![krate.clone()]);
                parts.remove(0);
            }
            Some("self") => {
                base = Some(scope_parts.clone());
                parts.remove(0);
            }
            Some("super") => {
                let mut b = scope_parts.clone();
                while parts.first() == Some(&"super") {
                    parts.remove(0);
                    b.pop();
                }
                base = Some(b);
            }
            Some("Self") => return None,
            _ => {}
        }
        if parts.is_empty() && base.is_none() {
            return None;
        }
        Some(Self {
            scope_parts,
            krate,
            parts,
            absolute,
            base,
        })
    }

    /// The path after `crate`, `self` and `super`.
    fn rest(&self) -> String {
        self.parts.join("::")
    }

    /// The absolute paths it may name: in the scope, as written, and from
    /// the root of the crate.
    fn candidates(&self) -> Vec<String> {
        let rest = self.rest();
        let join = |b: &[String]| -> String {
            let mut all: Vec<&str> = b.iter().map(String::as_str).collect();
            if !rest.is_empty() {
                all.push(&rest);
            }
            key(&all.join("::"))
        };
        match &self.base {
            Some(b) => vec![join(b)],
            None if self.absolute => vec![key(&rest)],
            None => vec![
                join(&self.scope_parts),
                key(&rest),
                join(std::slice::from_ref(&self.krate)),
            ],
        }
    }
}

/// A relative link between two page files (`.md` links: MkDocs, mdBook).
pub fn relative_file_href(from_file: &str, to_file: &str) -> String {
    let from: Vec<&str> = from_file.split('/').collect();
    let to: Vec<&str> = to_file.split('/').collect();
    let from_dirs = &from[..from.len() - 1];
    let common = from_dirs
        .iter()
        .zip(to.iter())
        .take_while(|(a, b)| a == b)
        .count();
    let mut parts: Vec<&str> = vec![".."; from_dirs.len() - common];
    parts.extend(&to[common..]);
    parts.join("/")
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

    fn test_index() -> RustDocIndex {
        let mut config = RustModule::test("my-crate::config");
        config.items = vec![RustItem::Struct(crate::model::RustStruct::test("Config"))];
        let mut handler = RustModule::test("my-crate::server::handler");
        handler.items = vec![RustItem::Function(crate::model::RustFunction::test(
            "handle",
        ))];
        let server = RustModule::test("my-crate::server");
        let root = RustModule::test("my-crate");
        RustDocIndex::new(&[root, config, server, handler], |m| {
            format!("rust/{}.md", m.replace("::", "/"))
        })
    }

    #[test]
    fn test_resolve_paths() {
        let index = test_index();
        let config = Some("rust/my-crate/config.md");
        fn page(t: Option<&DocTarget>) -> Option<&str> {
            t.map(|t| t.page.as_str())
        }
        assert_eq!(
            page(index.resolve("my-crate::server", "crate::config::Config")),
            config
        );
        assert_eq!(
            page(index.resolve("my-crate::server", "my_crate::config::Config")),
            config
        );
        assert_eq!(
            page(index.resolve("my-crate::server::handler", "super::super::config::Config")),
            config
        );
        assert_eq!(
            page(index.resolve("my-crate::server::handler", "super::handler")),
            Some("rust/my-crate/server/handler.md")
        );
        assert_eq!(
            page(index.resolve("my-crate::server", "handler::handle")),
            Some("rust/my-crate/server/handler.md")
        );
        // By name only: the one `Config` of the crate
        let target = index.resolve("my-crate::server", "Config").unwrap();
        assert_eq!(target.id.as_deref(), Some("struct.Config"));
        assert!(index.resolve("my-crate::server", "Missing").is_none());
        assert!(index.resolve("my-crate::server", "Self").is_none());
    }

    /// A crate with an enum `Frame` that has the variants `Error` and
    /// `Backend`, and a second crate with its own `Config`.
    fn variant_index() -> RustDocIndex {
        let mut frame = RustModule::test("guest::frame");
        let variant = |name: &str| crate::model::RustVariant {
            name: name.to_string(),
            doc_comment: None,
            fields: Vec::new(),
        };
        frame.items = vec![RustItem::Enum(crate::model::RustEnum {
            name: "Frame".to_string(),
            visibility: crate::model::Visibility::Public,
            doc_comment: None,
            parsed_doc: None,
            generics: None,
            variants: vec![variant("Error"), variant("Backend")],
            source: crate::model::SourceSpan::test("frame.rs", 1, 1),
        })];
        let sockets = RustModule::test("guest::sockets");
        let mut config = RustModule::test("guest::config");
        config.items = vec![RustItem::Struct(crate::model::RustStruct::test("Config"))];
        let mut other = RustModule::test("host::config");
        other.items = vec![RustItem::Struct(crate::model::RustStruct::test("Config"))];
        let host = RustModule::test("host");
        let guest = RustModule::test("guest");
        RustDocIndex::new(&[guest, frame, sockets, config, host, other], |m| {
            format!("rust/{}.md", m.replace("::", "/"))
        })
    }

    #[test]
    fn test_external_paths_do_not_resolve_locally() {
        // `std::io::Error` linked to `Frame`, which has a variant `Error`
        let index = variant_index();
        for path in [
            "std::io::Error",
            "core::fmt::Error",
            "alloc::string::String",
            "::std::io::Error",
            "serde_json::Error",
            "io::Error",
        ] {
            assert!(
                index.resolve("guest::sockets", path).is_none(),
                "{path} must not resolve"
            );
        }
    }

    #[test]
    fn test_bare_names_do_not_resolve_to_variants() {
        // `Backend` linked to the enum that has a variant `Backend`
        let index = variant_index();
        assert!(index.resolve("guest::sockets", "Backend").is_none());
        assert!(index.resolve("guest::sockets", "Error").is_none());
        // Written as Type::Variant, it resolves
        let target = index.resolve("guest::sockets", "Frame::Error").unwrap();
        assert_eq!(target.page, "rust/guest/frame.md");
        let target = index.resolve("guest::sockets", "Frame").unwrap();
        assert_eq!(target.id.as_deref(), Some("enum.Frame"));
    }

    #[test]
    fn test_bare_names_prefer_the_own_crate() {
        let index = variant_index();
        let target = index.resolve("guest::sockets", "Config").unwrap();
        assert_eq!(target.page, "rust/guest/config.md");
        let target = index.resolve("host", "Config").unwrap();
        assert_eq!(target.page, "rust/host/config.md");
    }

    #[test]
    fn test_paths_follow_reexports() {
        // `core` re-exports the module `frame` of `guest`, and globs it;
        // `facade` re-exports a name through `core`
        let mut frame = RustModule::test("guest::frame");
        frame.items = vec![RustItem::Struct(crate::model::RustStruct::test("Frame"))];
        let reexport = |name: &str, path: &str| crate::model::RustReexport {
            name: name.to_string(),
            path: path.to_string(),
            visibility: crate::model::Visibility::Public,
            doc_comment: None,
        };
        let mut core = RustModule::test("core-api");
        core.reexports = vec![reexport("frame", "guest::frame"), reexport("*", "frame")];
        let mut facade = RustModule::test("facade");
        facade.reexports = vec![reexport("Frame", "core_api::frame::Frame")];
        let index = RustDocIndex::new(&[RustModule::test("guest"), frame, core, facade], |m| {
            format!("rust/{}.md", m.replace("::", "/"))
        });
        for (scope, path) in [
            ("facade", "core_api::frame::Frame"),
            ("facade", "core_api::Frame"),
            ("facade", "Frame"),
            ("guest", "facade::Frame"),
        ] {
            let target = index.resolve(scope, path);
            assert_eq!(
                target.map(|t| t.page.as_str()),
                Some("rust/guest/frame.md"),
                "{path} in {scope}"
            );
        }
        assert!(
            index
                .resolve("facade", "core_api::frame::Missing")
                .is_none()
        );
    }

    #[test]
    fn test_relative_file_href() {
        assert_eq!(relative_file_href("rust/a/b.md", "rust/a/c.md"), "c.md");
        assert_eq!(relative_file_href("rust/a/b.md", "rust/x.md"), "../x.md");
        assert_eq!(relative_file_href("pkg.md", "rust/a.md"), "rust/a.md");
    }

    #[test]
    fn test_code_blocks_are_kept() {
        let text = "```rust\nlet x = v[`a`];\n[Foo]: bar\n```";
        assert_eq!(rewrite_intra_doc_links(text, &none), text);
    }
}
