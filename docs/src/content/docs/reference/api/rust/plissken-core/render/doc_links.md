---
title: "plissken-core::render::doc_links"
description: "Intra-doc links of Rust doc comments"
editUrl: false
sidebar:
  order: 18
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>

Intra-doc links of Rust doc comments

Rustdoc resolves links whose target is a Rust path: ``[`Config`]``,
``[`Config`](crate::config::Config)``, `[handler](super::handler)`, or a
reference definition `[Config]: crate::config::Config`. Markdown knows
none of them: it shows a bare link as literal brackets, and an explicit
one as a relative URL that does not exist. This module rewrites them:
a link that the resolver finds points to its page, any other one becomes
its text.

## Structs

### <span id="struct.DocTarget"></span><code style="font-size: 0.85em">struct DocTarget</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`, `PartialEq`, `Eq`

Where a Rust item is documented: its page (relative to the output folder) and its anchor.

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `page` | `String` | Page file path (`rust/my_crate/config.md`) |
| `id` | `Option<String>` | Explicit anchor id of the item (`struct.Config`); `None` for a module |
| `md_anchor` | `Option<String>` | Anchor of the item heading in MkDocs and mdBook (`struct-config`) |

### <span id="struct.RustDocIndex"></span><code style="font-size: 0.85em">struct RustDocIndex</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`, `Default`

The Rust items of the documentation, by path, to resolve intra-doc links.

#### Methods

##### <span id="RustDocIndex.new"></span><code>new</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn new(modules: &[RustModule], page_of: impl Fn(&str) -> String) -> Self
```

Index the items of the modules that are rendered. `page_of` gives the page file of a module path.

<details>
<summary>Source</summary>

```rust
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
        // A re-exported name points where its path points
        let mut aliases = Vec::new();
        for module in modules {
            for r in module.reexports.iter().filter(|r| r.name != "*") {
                if let Some(target) = index.resolve(&module.path, &r.path) {
                    aliases.push((format!("{}::{}", key(&module.path), r.name), target.clone()));
                }
            }
        }
        for (alias, target) in aliases {
            index.items.entry(alias).or_insert(target);
        }
        index
    }
```

</details>

##### <span id="RustDocIndex.module_of"></span><code>module_​of</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn module_of(&self, item_path: &str) -> String
```

The module of an item path: the longest prefix that is a module.

<details>
<summary>Source</summary>

```rust
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
```

</details>

##### <span id="RustDocIndex.resolve"></span><code>resolve</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn resolve(&self, scope: &str, path: &str) -> Option<&DocTarget>
```

Resolve an intra-doc link path written in the module `scope`.

`crate`, `self` and `super` are taken from the scope. A path is tried
in the scope, as written, and from the root of the crate; then, by its
last segments, as the only item of that name in the crate, then in
all crates (for re-exports and `use` imports, which plissken does not
follow).

<details>
<summary>Source</summary>

```rust
    pub fn resolve(&self, scope: &str, path: &str) -> Option<&DocTarget> {
        let scope = key(scope);
        let scope_parts: Vec<&str> = if scope.is_empty() {
            Vec::new()
        } else {
            scope.split("::").collect()
        };
        let krate = scope_parts.first().copied().unwrap_or("");

        let mut parts: Vec<&str> = path.split("::").collect();
        let absolute = parts.first() == Some(&"");
        if absolute {
            parts.remove(0);
        }
        let mut base: Option<Vec<&str>> = None;
        match parts.first().copied() {
            Some("crate") => {
                base = Some(vec![krate]);
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
        let rest = parts.join("::");
        let join = |b: &[&str]| -> String {
            let mut all: Vec<&str> = b.to_vec();
            if !rest.is_empty() {
                all.push(&rest);
            }
            key(&all.join("::"))
        };

        let candidates: Vec<String> = match base {
            Some(b) => vec![join(&b)],
            None if absolute => vec![key(&rest)],
            None => vec![join(&scope_parts), key(&rest), join(&[krate])],
        };
        for candidate in &candidates {
            if let Some(target) = self.items.get(candidate) {
                return Some(target);
            }
        }

        // By name: the only item whose path ends with the last segments
        for n in (1..=parts.len().min(2)).rev() {
            let suffix = format!("::{}", parts[parts.len() - n..].join("::"));
            let in_crate = format!("{}::", krate);
            for restrict in [true, false] {
                let mut found = self.items.iter().filter(|(k, _)| {
                    k.ends_with(&suffix)
                        && !self.modules.contains(k.as_str())
                        && (!restrict || k.starts_with(&in_crate))
                });
                if let (Some((_, target)), None) = (found.next(), found.next()) {
                    return Some(target);
                }
            }
        }
        None
    }
```

</details>

## Functions

### <span id="fn.intra_doc_path"></span><code style="font-size: 0.85em">fn intra_​doc_​path</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn intra_doc_path(target: &str) -> Option<&str>
```

The Rust path of an intra-doc link target, without its disambiguator (`struct@`) and suffix (`()`, `!`). `None` if the target is not a Rust path (a URL, a file, an anchor).

<details>
<summary>Source</summary>

```rust
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
```

</details>

### <span id="fn.rewrite_intra_doc_links"></span><code style="font-size: 0.85em">fn rewrite_​intra_​doc_​links</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn rewrite_intra_doc_links(
    markdown: &str,
    resolve: &dyn Fn(&str) -> Option<String>,
) -> String
```

Rewrite the intra-doc links of Rust doc Markdown.

`resolve` gets the Rust path of a link and gives its href, or `None`.
Code blocks and code spans are left as they are, and so are links to
URLs. A reference definition to a Rust path is removed.

<details>
<summary>Source</summary>

```rust
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
```

</details>

### <span id="fn.relative_file_href"></span><code style="font-size: 0.85em">fn relative_​file_​href</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn relative_file_href(from_file: &str, to_file: &str) -> String
```

A relative link between two page files (`.md` links: MkDocs, mdBook).

<details>
<summary>Source</summary>

```rust
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
```

</details>
