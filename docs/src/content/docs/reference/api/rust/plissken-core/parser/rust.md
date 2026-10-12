---
title: "plissken-core::parser::rust"
description: "Rust source code parser using syn"
editUrl: false
sidebar:
  order: 14
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>

Rust source code parser using syn

## Structs

### <span id="struct.RustParser"></span><code style="font-size: 0.85em">struct RustParser</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

#### Methods

##### <span id="RustParser.new"></span><code>new</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn new() -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn new() -> Self {
        Self
    }
```

</details>

##### <span id="RustParser.parse_file"></span><code>parse_​file</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn parse_file(&self, path: &Path) -> crate::error::Result<RustModule>
```

Parse a Rust source file.

**Errors:**

Returns `PlisskenError::FileRead` if the file cannot be read,
`PlisskenError::Parse` if the Rust syntax is invalid.

<details>
<summary>Source</summary>

```rust
    pub fn parse_file(&self, path: &Path) -> crate::error::Result<RustModule> {
        use crate::error::PlisskenError;

        let content =
            std::fs::read_to_string(path).map_err(|e| PlisskenError::file_read(path, e))?;
        self.parse_str(&content, path)
    }
```

</details>

##### <span id="RustParser.parse_str"></span><code>parse_​str</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn parse_str(
    &self,
    content: &str,
    path: &Path,
) -> crate::error::Result<RustModule>
```

Parse Rust source from a string.

**Errors:**

Returns `PlisskenError::Parse` if the Rust syntax is invalid.

<details>
<summary>Source</summary>

```rust
    pub fn parse_str(&self, content: &str, path: &Path) -> crate::error::Result<RustModule> {
        use crate::error::PlisskenError;

        let syntax = syn::parse_file(content).map_err(|e| PlisskenError::Parse {
            language: "Rust".into(),
            path: path.to_path_buf(),
            line: Some(e.span().start().line),
            message: e.to_string(),
        })?;

        // Extract module doc comment from inner attributes
        let doc_comment = extract_inner_doc_comments(&syntax.attrs);
        // Parse doc comment into structured form
        let parsed_doc = doc_comment.as_ref().map(|d| parse_rust_doc(d));

        // Extract items; test code is not documented
        let items = syntax
            .items
            .iter()
            .filter(|item| !is_test_only(item_attrs(item)))
            .filter_map(|item| self.extract_item(item, content, path))
            .collect();

        let mut reexports = Vec::new();
        let mut test_modules = Vec::new();
        let mut py_registrars = Vec::new();
        for item in &syntax.items {
            match item {
                Item::Fn(f) if !is_test_only(&f.attrs) => {
                    if let Some(registrar) = super::pymodule::registrar(f) {
                        py_registrars.push(registrar);
                    }
                }
                Item::Use(u) if !matches!(u.vis, SynVisibility::Inherited) => {
                    if is_test_only(&u.attrs) {
                        continue;
                    }
                    let visibility = convert_visibility(&u.vis);
                    let doc_comment = extract_doc_comments(&u.attrs);
                    collect_use_tree(&u.tree, String::new(), &mut |name, path| {
                        reexports.push(RustReexport {
                            name,
                            path,
                            visibility: visibility.clone(),
                            doc_comment: doc_comment.clone(),
                        });
                    });
                }
                Item::Mod(m) if m.content.is_none() && is_test_only(&m.attrs) => {
                    test_modules.push(m.ident.to_string());
                }
                _ => {}
            }
        }

        Ok(RustModule {
            path: path.display().to_string(),
            doc_comment,
            parsed_doc,
            items,
            reexports,
            test_modules,
            py_registrars,
            source: SourceSpan::new(
                path.to_path_buf(),
                1,
                content.lines().count().max(1),
                content,
            ),
        })
    }
```

</details>
