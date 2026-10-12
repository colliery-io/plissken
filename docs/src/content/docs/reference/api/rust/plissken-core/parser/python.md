---
title: "plissken-core::parser::python"
description: "Python source code parser using tree-sitter"
editUrl: false
sidebar:
  order: 13
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>

Python source code parser using tree-sitter

## Structs

### <span id="struct.PythonParser"></span><code style="font-size: 0.85em">struct PythonParser</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

#### Methods

##### <span id="PythonParser.new"></span><code>new</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn new() -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn new() -> Self {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_python::LANGUAGE.into())
            .expect("Failed to load Python grammar");
        Self { parser }
    }
```

</details>

##### <span id="PythonParser.parse_file"></span><code>parse_​file</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn parse_file(
    &mut self,
    path: &Path,
) -> crate::error::Result<PythonModule>
```

Parse a Python source file.

**Errors:**

Returns `PlisskenError::FileRead` if the file cannot be read,
`PlisskenError::Parse` if the Python syntax is invalid.

<details>
<summary>Source</summary>

```rust
    pub fn parse_file(&mut self, path: &Path) -> crate::error::Result<PythonModule> {
        use crate::error::PlisskenError;

        let content =
            std::fs::read_to_string(path).map_err(|e| PlisskenError::file_read(path, e))?;
        self.parse_str(&content, path)
    }
```

</details>

##### <span id="PythonParser.parse_str"></span><code>parse_​str</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn parse_str(
    &mut self,
    content: &str,
    path: &Path,
) -> crate::error::Result<PythonModule>
```

Parse Python source from a string.

**Errors:**

Returns `PlisskenError::Parse` if the Python syntax is invalid.

<details>
<summary>Source</summary>

```rust
    pub fn parse_str(&mut self, content: &str, path: &Path) -> crate::error::Result<PythonModule> {
        use crate::error::PlisskenError;

        let tree = self
            .parser
            .parse(content, None)
            .ok_or_else(|| PlisskenError::python_parse(path, "failed to parse Python source"))?;

        let root = tree.root_node();

        // Extract module docstring
        let docstring = extract_module_docstring(&root, content);
        // Parse docstring into structured form
        let parsed_doc = docstring.as_ref().map(|d| parse_docstring(d));

        // Extract items
        let items = extract_module_items(&root, content, path);

        Ok(PythonModule {
            path: path.display().to_string(),
            docstring,
            parsed_doc,
            items,
            source_type: SourceType::Python,
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
