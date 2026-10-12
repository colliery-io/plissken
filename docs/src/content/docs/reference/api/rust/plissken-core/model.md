---
title: "plissken-core::model"
description: "Unified documentation model for Rust and Python items"
editUrl: false
sidebar:
  order: 10
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>

Unified documentation model for Rust and Python items

## Structs

### <span id="struct.SourceLocation"></span><code style="font-size: 0.85em">struct SourceLocation</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

A reference to a location in source code

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `file` | `PathBuf` |  |
| `line_start` | `usize` |  |
| `line_end` | `usize` |  |

#### Methods

##### <span id="SourceLocation.test"></span><code>test</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn test(
    file: impl Into<PathBuf>,
    line_start: usize,
    line_end: usize,
) -> Self
```

Create a test source location

<details>
<summary>Source</summary>

```rust
    pub fn test(file: impl Into<PathBuf>, line_start: usize, line_end: usize) -> Self {
        Self {
            file: file.into(),
            line_start,
            line_end,
        }
    }
```

</details>

### <span id="struct.SourceSpan"></span><code style="font-size: 0.85em">struct SourceSpan</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

Source code span with the actual text

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `location` | `SourceLocation` |  |
| `source` | `String` | The actual source code text |

#### Methods

##### <span id="SourceSpan.test"></span><code>test</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn test(
    file: impl Into<PathBuf>,
    line_start: usize,
    line_end: usize,
) -> Self
```

Create a test source span with empty source

<details>
<summary>Source</summary>

```rust
    pub fn test(file: impl Into<PathBuf>, line_start: usize, line_end: usize) -> Self {
        Self {
            location: SourceLocation::test(file, line_start, line_end),
            source: String::new(),
        }
    }
```

</details>

##### <span id="SourceSpan.new"></span><code>new</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn new(
    file: impl Into<PathBuf>,
    line_start: usize,
    line_end: usize,
    source: impl Into<String>,
) -> Self
```

Create a source span with actual source code

<details>
<summary>Source</summary>

```rust
    pub fn new(
        file: impl Into<PathBuf>,
        line_start: usize,
        line_end: usize,
        source: impl Into<String>,
    ) -> Self {
        Self {
            location: SourceLocation::test(file, line_start, line_end),
            source: source.into(),
        }
    }
```

</details>

### <span id="struct.RustModule"></span><code style="font-size: 0.85em">struct RustModule</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

A Rust module with its items

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `path` | `String` |  |
| `doc_comment` | `Option<String>` |  |
| `parsed_doc` | `Option<ParsedDocstring>` |  |
| `items` | `Vec<RustItem>` |  |
| `reexports` | `Vec<RustReexport>` | The `pub use` declarations of the module |
| `test_modules` | `Vec<String>` | Names of the submodules declared `#[cfg(test)] mod name;` |
| `py_registrars` | `Vec<PyModuleRegistrar>` | The functions that build PyO3 modules (`#[pymodule]`, and helpers that get the module as a parameter) |
| `source` | `SourceSpan` |  |

#### Methods

##### <span id="RustModule.test"></span><code>test</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn test(path: impl Into<String>) -> Self
```

Create a test Rust module

<details>
<summary>Source</summary>

```rust
    pub fn test(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            doc_comment: None,
            parsed_doc: None,
            items: Vec::new(),
            reexports: Vec::new(),
            test_modules: Vec::new(),
            py_registrars: Vec::new(),
            source: SourceSpan::test("test.rs", 1, 1),
        }
    }
```

</details>

##### <span id="RustModule.with_doc"></span><code>with_​doc</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_doc(mut self, doc: impl Into<String>) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_doc(mut self, doc: impl Into<String>) -> Self {
        self.doc_comment = Some(doc.into());
        self
    }
```

</details>

##### <span id="RustModule.with_item"></span><code>with_​item</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_item(mut self, item: RustItem) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_item(mut self, item: RustItem) -> Self {
        self.items.push(item);
        self
    }
```

</details>

##### <span id="RustModule.with_source"></span><code>with_​source</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_source(mut self, source: SourceSpan) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_source(mut self, source: SourceSpan) -> Self {
        self.source = source;
        self
    }
```

</details>

### <span id="struct.PyModuleRegistrar"></span><code style="font-size: 0.85em">struct PyModuleRegistrar</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`, `PartialEq`

A function that adds items to a PyO3 module: a `#[pymodule]` function, or a helper that gets the module as a parameter (`register(m)`).

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `function` | `String` | The Rust name of the function |
| `pymodule` | `Option<String>` | The Python name of the module, if the function is a `#[pymodule]` |
| `doc_comment` | `Option<String>` |  |
| `adds` | `Vec<PyModuleAdd>` | What the function adds, in order |

### <span id="struct.PyModuleAdd"></span><code style="font-size: 0.85em">struct PyModuleAdd</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`, `PartialEq`

One thing that a [`PyModuleRegistrar`](../../../rust/plissken-core/model/#struct.PyModuleRegistrar) adds to a module.

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `module` | `String` | The module it adds to: empty for the module of the parameter, else the Python name of a submodule made in the function |
| `kind` | `PyModuleAddKind` |  |
| `path` | `String` | The Rust path of the class or function; the Python name of the submodule; the name of the function called |

### <span id="struct.RustReexport"></span><code style="font-size: 0.85em">struct RustReexport</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`, `PartialEq`

A name that a module re-exports: `pub use crate::context::Context;`

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` | The name in the module (`Context`, or `*` for a glob) |
| `path` | `String` | The path that is re-exported (`crate::context::Context`, or `crate::context` for a glob) |
| `visibility` | `Visibility` |  |
| `doc_comment` | `Option<String>` |  |

### <span id="struct.RustStruct"></span><code style="font-size: 0.85em">struct RustStruct</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

A Rust struct definition

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` |  |
| `visibility` | `Visibility` |  |
| `doc_comment` | `Option<String>` |  |
| `parsed_doc` | `Option<ParsedDocstring>` |  |
| `generics` | `Option<String>` | Generic parameters as string, e.g. "<T: Clone, const N: usize>" |
| `fields` | `Vec<RustField>` |  |
| `derives` | `Vec<String>` |  |
| `pyclass` | `Option<PyClassMeta>` |  |
| `source` | `SourceSpan` |  |

#### Methods

##### <span id="RustStruct.test"></span><code>test</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn test(name: impl Into<String>) -> Self
```

Create a test struct

<details>
<summary>Source</summary>

```rust
    pub fn test(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            visibility: Visibility::Public,
            doc_comment: None,
            parsed_doc: None,
            generics: None,
            fields: Vec::new(),
            derives: Vec::new(),
            pyclass: None,
            source: SourceSpan::test("test.rs", 1, 1),
        }
    }
```

</details>

##### <span id="RustStruct.with_visibility"></span><code>with_​visibility</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_visibility(mut self, vis: Visibility) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_visibility(mut self, vis: Visibility) -> Self {
        self.visibility = vis;
        self
    }
```

</details>

##### <span id="RustStruct.with_doc"></span><code>with_​doc</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_doc(mut self, doc: impl Into<String>) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_doc(mut self, doc: impl Into<String>) -> Self {
        self.doc_comment = Some(doc.into());
        self
    }
```

</details>

##### <span id="RustStruct.with_generics"></span><code>with_​generics</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_generics(mut self, generics: impl Into<String>) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_generics(mut self, generics: impl Into<String>) -> Self {
        self.generics = Some(generics.into());
        self
    }
```

</details>

##### <span id="RustStruct.with_field"></span><code>with_​field</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_field(mut self, field: RustField) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_field(mut self, field: RustField) -> Self {
        self.fields.push(field);
        self
    }
```

</details>

##### <span id="RustStruct.with_derive"></span><code>with_​derive</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_derive(mut self, derive: impl Into<String>) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_derive(mut self, derive: impl Into<String>) -> Self {
        self.derives.push(derive.into());
        self
    }
```

</details>

##### <span id="RustStruct.with_pyclass"></span><code>with_​pyclass</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_pyclass(mut self, meta: PyClassMeta) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_pyclass(mut self, meta: PyClassMeta) -> Self {
        self.pyclass = Some(meta);
        self
    }
```

</details>

##### <span id="RustStruct.with_source"></span><code>with_​source</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_source(mut self, source: SourceSpan) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_source(mut self, source: SourceSpan) -> Self {
        self.source = source;
        self
    }
```

</details>

### <span id="struct.PyClassMeta"></span><code style="font-size: 0.85em">struct PyClassMeta</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

PyO3 #[pyclass] metadata

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `Option<String>` |  |
| `module` | `Option<String>` |  |

#### Methods

##### <span id="PyClassMeta.new"></span><code>new</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn new() -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn new() -> Self {
        Self {
            name: None,
            module: None,
        }
    }
```

</details>

##### <span id="PyClassMeta.with_name"></span><code>with_​name</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_name(mut self, name: impl Into<String>) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
```

</details>

##### <span id="PyClassMeta.with_module"></span><code>with_​module</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_module(mut self, module: impl Into<String>) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_module(mut self, module: impl Into<String>) -> Self {
        self.module = Some(module.into());
        self
    }
```

</details>

### <span id="struct.RustField"></span><code style="font-size: 0.85em">struct RustField</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

A Rust field

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` |  |
| `ty` | `String` |  |
| `visibility` | `Visibility` |  |
| `doc_comment` | `Option<String>` |  |

#### Methods

##### <span id="RustField.test"></span><code>test</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn test(name: impl Into<String>, ty: impl Into<String>) -> Self
```

Create a test field

<details>
<summary>Source</summary>

```rust
    pub fn test(name: impl Into<String>, ty: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ty: ty.into(),
            visibility: Visibility::Public,
            doc_comment: None,
        }
    }
```

</details>

##### <span id="RustField.private"></span><code>private</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn private(mut self) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn private(mut self) -> Self {
        self.visibility = Visibility::Private;
        self
    }
```

</details>

##### <span id="RustField.with_doc"></span><code>with_​doc</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_doc(mut self, doc: impl Into<String>) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_doc(mut self, doc: impl Into<String>) -> Self {
        self.doc_comment = Some(doc.into());
        self
    }
```

</details>

### <span id="struct.RustEnum"></span><code style="font-size: 0.85em">struct RustEnum</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

A Rust enum definition

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` |  |
| `visibility` | `Visibility` |  |
| `doc_comment` | `Option<String>` |  |
| `parsed_doc` | `Option<ParsedDocstring>` |  |
| `generics` | `Option<String>` | Generic parameters as string |
| `variants` | `Vec<RustVariant>` |  |
| `source` | `SourceSpan` |  |

### <span id="struct.RustVariant"></span><code style="font-size: 0.85em">struct RustVariant</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

A Rust enum variant

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` |  |
| `doc_comment` | `Option<String>` |  |
| `fields` | `Vec<RustField>` |  |

### <span id="struct.RustFunction"></span><code style="font-size: 0.85em">struct RustFunction</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

A Rust function definition

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` |  |
| `visibility` | `Visibility` |  |
| `doc_comment` | `Option<String>` |  |
| `parsed_doc` | `Option<ParsedDocstring>` |  |
| `generics` | `Option<String>` | Generic parameters as string, e.g. "<'a, T: Clone>" |
| `signature_str` | `String` | Full signature as string for display |
| `signature` | `RustFunctionSig` | Parsed signature for structured access |
| `is_async` | `bool` |  |
| `is_unsafe` | `bool` |  |
| `is_const` | `bool` |  |
| `pyfunction` | `Option<PyFunctionMeta>` |  |
| `source` | `SourceSpan` |  |

#### Methods

##### <span id="RustFunction.test"></span><code>test</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn test(name: impl Into<String>) -> Self
```

Create a test function

<details>
<summary>Source</summary>

```rust
    pub fn test(name: impl Into<String>) -> Self {
        let name = name.into();
        Self {
            name: name.clone(),
            visibility: Visibility::Public,
            doc_comment: None,
            parsed_doc: None,
            generics: None,
            signature_str: format!("fn {}()", name),
            signature: RustFunctionSig {
                params: Vec::new(),
                return_type: None,
            },
            is_async: bool::default(),
            is_unsafe: bool::default(),
            is_const: bool::default(),
            pyfunction: None,
            source: SourceSpan::test("test.rs", 1, 1),
        }
    }
```

</details>

##### <span id="RustFunction.with_doc"></span><code>with_​doc</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_doc(mut self, doc: impl Into<String>) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_doc(mut self, doc: impl Into<String>) -> Self {
        self.doc_comment = Some(doc.into());
        self
    }
```

</details>

##### <span id="RustFunction.with_generics"></span><code>with_​generics</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_generics(mut self, generics: impl Into<String>) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_generics(mut self, generics: impl Into<String>) -> Self {
        self.generics = Some(generics.into());
        self
    }
```

</details>

##### <span id="RustFunction.with_signature"></span><code>with_​signature</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_signature(mut self, sig: impl Into<String>) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_signature(mut self, sig: impl Into<String>) -> Self {
        self.signature_str = sig.into();
        self
    }
```

</details>

##### <span id="RustFunction.with_param"></span><code>with_​param</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_param(mut self, param: RustParam) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_param(mut self, param: RustParam) -> Self {
        self.signature.params.push(param);
        self
    }
```

</details>

##### <span id="RustFunction.with_return_type"></span><code>with_​return_​type</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_return_type(mut self, ty: impl Into<String>) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_return_type(mut self, ty: impl Into<String>) -> Self {
        self.signature.return_type = Some(ty.into());
        self
    }
```

</details>

##### <span id="RustFunction.async_"></span><code>async_​</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn async_(mut self) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn async_(mut self) -> Self {
        self.is_async = true;
        self
    }
```

</details>

##### <span id="RustFunction.unsafe_"></span><code>unsafe_​</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn unsafe_(mut self) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn unsafe_(mut self) -> Self {
        self.is_unsafe = true;
        self
    }
```

</details>

##### <span id="RustFunction.const_"></span><code>const_​</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn const_(mut self) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn const_(mut self) -> Self {
        self.is_const = true;
        self
    }
```

</details>

##### <span id="RustFunction.with_pyfunction"></span><code>with_​pyfunction</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_pyfunction(mut self, meta: PyFunctionMeta) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_pyfunction(mut self, meta: PyFunctionMeta) -> Self {
        self.pyfunction = Some(meta);
        self
    }
```

</details>

##### <span id="RustFunction.with_source"></span><code>with_​source</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_source(mut self, source: SourceSpan) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_source(mut self, source: SourceSpan) -> Self {
        self.source = source;
        self
    }
```

</details>

### <span id="struct.RustFunctionSig"></span><code style="font-size: 0.85em">struct RustFunctionSig</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

Rust function signature

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `params` | `Vec<RustParam>` |  |
| `return_type` | `Option<String>` |  |

### <span id="struct.RustParam"></span><code style="font-size: 0.85em">struct RustParam</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

A Rust function parameter

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` |  |
| `ty` | `String` |  |
| `default` | `Option<String>` |  |

#### Methods

##### <span id="RustParam.test"></span><code>test</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn test(name: impl Into<String>, ty: impl Into<String>) -> Self
```

Create a test parameter

<details>
<summary>Source</summary>

```rust
    pub fn test(name: impl Into<String>, ty: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ty: ty.into(),
            default: None,
        }
    }
```

</details>

##### <span id="RustParam.with_default"></span><code>with_​default</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_default(mut self, default: impl Into<String>) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_default(mut self, default: impl Into<String>) -> Self {
        self.default = Some(default.into());
        self
    }
```

</details>

### <span id="struct.PyFunctionMeta"></span><code style="font-size: 0.85em">struct PyFunctionMeta</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

PyO3 #[pyfunction] metadata

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `Option<String>` |  |
| `signature` | `Option<String>` |  |

#### Methods

##### <span id="PyFunctionMeta.new"></span><code>new</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn new() -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn new() -> Self {
        Self {
            name: None,
            signature: None,
        }
    }
```

</details>

##### <span id="PyFunctionMeta.with_name"></span><code>with_​name</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_name(mut self, name: impl Into<String>) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
```

</details>

##### <span id="PyFunctionMeta.with_signature"></span><code>with_​signature</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_signature(mut self, sig: impl Into<String>) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_signature(mut self, sig: impl Into<String>) -> Self {
        self.signature = Some(sig.into());
        self
    }
```

</details>

### <span id="struct.RustTrait"></span><code style="font-size: 0.85em">struct RustTrait</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

A Rust trait definition

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` |  |
| `visibility` | `Visibility` |  |
| `doc_comment` | `Option<String>` |  |
| `parsed_doc` | `Option<ParsedDocstring>` |  |
| `generics` | `Option<String>` | Generic parameters as string |
| `bounds` | `Option<String>` | Supertraits as string, e.g. ": Clone + Send" |
| `associated_types` | `Vec<RustAssociatedType>` |  |
| `methods` | `Vec<RustFunction>` |  |
| `source` | `SourceSpan` |  |

### <span id="struct.RustAssociatedType"></span><code style="font-size: 0.85em">struct RustAssociatedType</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

A Rust associated type in a trait

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` |  |
| `doc_comment` | `Option<String>` |  |
| `generics` | `Option<String>` | Generic parameters (for GATs) |
| `bounds` | `Option<String>` | Bounds on the associated type |

### <span id="struct.RustImpl"></span><code style="font-size: 0.85em">struct RustImpl</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

A Rust impl block

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `generics` | `Option<String>` | Generic parameters on the impl block |
| `target` | `String` | The type being implemented for |
| `trait_` | `Option<String>` | Trait being implemented (if any) |
| `where_clause` | `Option<String>` | Where clause constraints |
| `methods` | `Vec<RustFunction>` |  |
| `pymethods` | `bool` | Whether this is a #[pymethods] block |
| `source` | `SourceSpan` |  |

### <span id="struct.RustConst"></span><code style="font-size: 0.85em">struct RustConst</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

A Rust const item

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` |  |
| `visibility` | `Visibility` |  |
| `doc_comment` | `Option<String>` |  |
| `ty` | `String` |  |
| `value` | `Option<String>` |  |
| `source` | `SourceSpan` |  |

### <span id="struct.RustTypeAlias"></span><code style="font-size: 0.85em">struct RustTypeAlias</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

A Rust type alias

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` |  |
| `visibility` | `Visibility` |  |
| `doc_comment` | `Option<String>` |  |
| `generics` | `Option<String>` |  |
| `ty` | `String` | The aliased type as string |
| `source` | `SourceSpan` |  |

### <span id="struct.PythonModule"></span><code style="font-size: 0.85em">struct PythonModule</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

A Python module

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `path` | `String` |  |
| `docstring` | `Option<String>` |  |
| `parsed_doc` | `Option<ParsedDocstring>` |  |
| `items` | `Vec<PythonItem>` |  |
| `source_type` | `SourceType` |  |
| `source` | `SourceSpan` |  |

#### Methods

##### <span id="PythonModule.test"></span><code>test</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn test(path: impl Into<String>) -> Self
```

Create a test Python module

<details>
<summary>Source</summary>

```rust
    pub fn test(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            docstring: None,
            parsed_doc: None,
            items: Vec::new(),
            source_type: SourceType::Python,
            source: SourceSpan::test("test.py", 1, 1),
        }
    }
```

</details>

##### <span id="PythonModule.with_docstring"></span><code>with_​docstring</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_docstring(mut self, doc: impl Into<String>) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_docstring(mut self, doc: impl Into<String>) -> Self {
        self.docstring = Some(doc.into());
        self
    }
```

</details>

##### <span id="PythonModule.with_item"></span><code>with_​item</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_item(mut self, item: PythonItem) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_item(mut self, item: PythonItem) -> Self {
        self.items.push(item);
        self
    }
```

</details>

##### <span id="PythonModule.pyo3_binding"></span><code>pyo3_​binding</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn pyo3_binding(mut self) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn pyo3_binding(mut self) -> Self {
        self.source_type = SourceType::PyO3Binding;
        self
    }
```

</details>

##### <span id="PythonModule.with_source"></span><code>with_​source</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_source(mut self, source: SourceSpan) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_source(mut self, source: SourceSpan) -> Self {
        self.source = source;
        self
    }
```

</details>

### <span id="struct.PythonClass"></span><code style="font-size: 0.85em">struct PythonClass</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

A Python class

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` |  |
| `docstring` | `Option<String>` |  |
| `parsed_doc` | `Option<ParsedDocstring>` |  |
| `bases` | `Vec<String>` |  |
| `methods` | `Vec<PythonFunction>` |  |
| `attributes` | `Vec<PythonVariable>` |  |
| `decorators` | `Vec<String>` |  |
| `rust_impl` | `Option<RustItemRef>` |  |
| `source` | `SourceSpan` |  |

#### Methods

##### <span id="PythonClass.test"></span><code>test</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn test(name: impl Into<String>) -> Self
```

Create a test Python class

<details>
<summary>Source</summary>

```rust
    pub fn test(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            docstring: None,
            parsed_doc: None,
            bases: Vec::new(),
            methods: Vec::new(),
            attributes: Vec::new(),
            decorators: Vec::new(),
            rust_impl: None,
            source: SourceSpan::test("test.py", 1, 1),
        }
    }
```

</details>

##### <span id="PythonClass.with_docstring"></span><code>with_​docstring</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_docstring(mut self, doc: impl Into<String>) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_docstring(mut self, doc: impl Into<String>) -> Self {
        self.docstring = Some(doc.into());
        self
    }
```

</details>

##### <span id="PythonClass.with_base"></span><code>with_​base</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_base(mut self, base: impl Into<String>) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_base(mut self, base: impl Into<String>) -> Self {
        self.bases.push(base.into());
        self
    }
```

</details>

##### <span id="PythonClass.with_method"></span><code>with_​method</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_method(mut self, method: PythonFunction) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_method(mut self, method: PythonFunction) -> Self {
        self.methods.push(method);
        self
    }
```

</details>

##### <span id="PythonClass.with_attribute"></span><code>with_​attribute</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_attribute(mut self, attr: PythonVariable) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_attribute(mut self, attr: PythonVariable) -> Self {
        self.attributes.push(attr);
        self
    }
```

</details>

##### <span id="PythonClass.with_decorator"></span><code>with_​decorator</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_decorator(mut self, decorator: impl Into<String>) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_decorator(mut self, decorator: impl Into<String>) -> Self {
        self.decorators.push(decorator.into());
        self
    }
```

</details>

##### <span id="PythonClass.with_rust_impl"></span><code>with_​rust_​impl</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_rust_impl(mut self, rust_ref: RustItemRef) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_rust_impl(mut self, rust_ref: RustItemRef) -> Self {
        self.rust_impl = Some(rust_ref);
        self
    }
```

</details>

##### <span id="PythonClass.with_source"></span><code>with_​source</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_source(mut self, source: SourceSpan) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_source(mut self, source: SourceSpan) -> Self {
        self.source = source;
        self
    }
```

</details>

### <span id="struct.PythonFunction"></span><code style="font-size: 0.85em">struct PythonFunction</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

A Python function

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` |  |
| `docstring` | `Option<String>` |  |
| `signature_str` | `String` | Full signature as string for display |
| `signature` | `PythonFunctionSig` | Parsed signature for structured access |
| `decorators` | `Vec<String>` |  |
| `is_async` | `bool` |  |
| `is_staticmethod` | `bool` |  |
| `is_classmethod` | `bool` |  |
| `is_property` | `bool` |  |
| `parsed_doc` | `Option<ParsedDocstring>` |  |
| `rust_impl` | `Option<RustItemRef>` |  |
| `source` | `SourceSpan` |  |

#### Methods

##### <span id="PythonFunction.test"></span><code>test</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn test(name: impl Into<String>) -> Self
```

Create a test Python function

<details>
<summary>Source</summary>

```rust
    pub fn test(name: impl Into<String>) -> Self {
        let name = name.into();
        Self {
            name: name.clone(),
            docstring: None,
            signature_str: format!("def {}()", name),
            signature: PythonFunctionSig {
                params: Vec::new(),
                return_type: None,
            },
            decorators: Vec::new(),
            is_async: bool::default(),
            is_staticmethod: bool::default(),
            is_classmethod: bool::default(),
            is_property: bool::default(),
            parsed_doc: None,
            rust_impl: None,
            source: SourceSpan::test("test.py", 1, 1),
        }
    }
```

</details>

##### <span id="PythonFunction.with_docstring"></span><code>with_​docstring</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_docstring(mut self, doc: impl Into<String>) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_docstring(mut self, doc: impl Into<String>) -> Self {
        self.docstring = Some(doc.into());
        self
    }
```

</details>

##### <span id="PythonFunction.with_signature"></span><code>with_​signature</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_signature(mut self, sig: impl Into<String>) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_signature(mut self, sig: impl Into<String>) -> Self {
        self.signature_str = sig.into();
        self
    }
```

</details>

##### <span id="PythonFunction.with_param"></span><code>with_​param</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_param(mut self, param: PythonParam) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_param(mut self, param: PythonParam) -> Self {
        self.signature.params.push(param);
        self
    }
```

</details>

##### <span id="PythonFunction.with_return_type"></span><code>with_​return_​type</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_return_type(mut self, ty: impl Into<String>) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_return_type(mut self, ty: impl Into<String>) -> Self {
        self.signature.return_type = Some(ty.into());
        self
    }
```

</details>

##### <span id="PythonFunction.with_decorator"></span><code>with_​decorator</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_decorator(mut self, decorator: impl Into<String>) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_decorator(mut self, decorator: impl Into<String>) -> Self {
        self.decorators.push(decorator.into());
        self
    }
```

</details>

##### <span id="PythonFunction.async_"></span><code>async_​</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn async_(mut self) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn async_(mut self) -> Self {
        self.is_async = true;
        self
    }
```

</details>

##### <span id="PythonFunction.staticmethod"></span><code>staticmethod</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn staticmethod(mut self) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn staticmethod(mut self) -> Self {
        self.is_staticmethod = true;
        self
    }
```

</details>

##### <span id="PythonFunction.classmethod"></span><code>classmethod</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn classmethod(mut self) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn classmethod(mut self) -> Self {
        self.is_classmethod = true;
        self
    }
```

</details>

##### <span id="PythonFunction.property"></span><code>property</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn property(mut self) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn property(mut self) -> Self {
        self.is_property = true;
        self
    }
```

</details>

##### <span id="PythonFunction.with_rust_impl"></span><code>with_​rust_​impl</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_rust_impl(mut self, rust_ref: RustItemRef) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_rust_impl(mut self, rust_ref: RustItemRef) -> Self {
        self.rust_impl = Some(rust_ref);
        self
    }
```

</details>

##### <span id="PythonFunction.with_source"></span><code>with_​source</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_source(mut self, source: SourceSpan) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_source(mut self, source: SourceSpan) -> Self {
        self.source = source;
        self
    }
```

</details>

### <span id="struct.PythonFunctionSig"></span><code style="font-size: 0.85em">struct PythonFunctionSig</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

Python function signature

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `params` | `Vec<PythonParam>` |  |
| `return_type` | `Option<String>` |  |

### <span id="struct.PythonParam"></span><code style="font-size: 0.85em">struct PythonParam</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

A Python function parameter

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` |  |
| `ty` | `Option<String>` |  |
| `default` | `Option<String>` |  |

#### Methods

##### <span id="PythonParam.test"></span><code>test</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn test(name: impl Into<String>) -> Self
```

Create a test parameter

<details>
<summary>Source</summary>

```rust
    pub fn test(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ty: None,
            default: None,
        }
    }
```

</details>

##### <span id="PythonParam.with_type"></span><code>with_​type</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_type(mut self, ty: impl Into<String>) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_type(mut self, ty: impl Into<String>) -> Self {
        self.ty = Some(ty.into());
        self
    }
```

</details>

##### <span id="PythonParam.with_default"></span><code>with_​default</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_default(mut self, default: impl Into<String>) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_default(mut self, default: impl Into<String>) -> Self {
        self.default = Some(default.into());
        self
    }
```

</details>

### <span id="struct.PythonVariable"></span><code style="font-size: 0.85em">struct PythonVariable</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

A Python variable/attribute

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` |  |
| `ty` | `Option<String>` |  |
| `value` | `Option<String>` |  |
| `docstring` | `Option<String>` |  |

#### Methods

##### <span id="PythonVariable.test"></span><code>test</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn test(name: impl Into<String>) -> Self
```

Create a test variable

<details>
<summary>Source</summary>

```rust
    pub fn test(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ty: None,
            value: None,
            docstring: None,
        }
    }
```

</details>

##### <span id="PythonVariable.with_type"></span><code>with_​type</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_type(mut self, ty: impl Into<String>) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_type(mut self, ty: impl Into<String>) -> Self {
        self.ty = Some(ty.into());
        self
    }
```

</details>

##### <span id="PythonVariable.with_value"></span><code>with_​value</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_value(mut self, value: impl Into<String>) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
```

</details>

##### <span id="PythonVariable.with_docstring"></span><code>with_​docstring</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_docstring(mut self, doc: impl Into<String>) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_docstring(mut self, doc: impl Into<String>) -> Self {
        self.docstring = Some(doc.into());
        self
    }
```

</details>

### <span id="struct.ParsedDocstring"></span><code style="font-size: 0.85em">struct ParsedDocstring</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

Parsed docstring (Google/NumPy style)

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `summary` | `Option<String>` |  |
| `description` | `Option<String>` |  |
| `params` | `Vec<ParamDoc>` |  |
| `returns` | `Option<ReturnDoc>` |  |
| `raises` | `Vec<RaisesDoc>` |  |
| `examples` | `Vec<String>` |  |

### <span id="struct.ParamDoc"></span><code style="font-size: 0.85em">struct ParamDoc</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

Documented parameter

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` |  |
| `ty` | `Option<String>` |  |
| `description` | `String` |  |

### <span id="struct.ReturnDoc"></span><code style="font-size: 0.85em">struct ReturnDoc</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

Documented return value

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `ty` | `Option<String>` |  |
| `description` | `String` |  |

### <span id="struct.RaisesDoc"></span><code style="font-size: 0.85em">struct RaisesDoc</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

Documented exception

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `ty` | `String` |  |
| `description` | `String` |  |

### <span id="struct.RustItemRef"></span><code style="font-size: 0.85em">struct RustItemRef</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

Reference to a Rust item

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `path` | `String` |  |
| `name` | `String` |  |

#### Methods

##### <span id="RustItemRef.new"></span><code>new</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn new(path: impl Into<String>, name: impl Into<String>) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn new(path: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            name: name.into(),
        }
    }
```

</details>

### <span id="struct.CrossRef"></span><code style="font-size: 0.85em">struct CrossRef</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

Cross-reference between Python and Rust

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `python_path` | `String` |  |
| `rust_path` | `String` |  |
| `relationship` | `CrossRefKind` |  |

#### Methods

##### <span id="CrossRef.binding"></span><code>binding</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn binding(
    python_path: impl Into<String>,
    rust_path: impl Into<String>,
) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn binding(python_path: impl Into<String>, rust_path: impl Into<String>) -> Self {
        Self {
            python_path: python_path.into(),
            rust_path: rust_path.into(),
            relationship: CrossRefKind::Binding,
        }
    }
```

</details>

##### <span id="CrossRef.wraps"></span><code>wraps</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn wraps(
    python_path: impl Into<String>,
    rust_path: impl Into<String>,
) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn wraps(python_path: impl Into<String>, rust_path: impl Into<String>) -> Self {
        Self {
            python_path: python_path.into(),
            rust_path: rust_path.into(),
            relationship: CrossRefKind::Wraps,
        }
    }
```

</details>

##### <span id="CrossRef.delegates"></span><code>delegates</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn delegates(
    python_path: impl Into<String>,
    rust_path: impl Into<String>,
) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn delegates(python_path: impl Into<String>, rust_path: impl Into<String>) -> Self {
        Self {
            python_path: python_path.into(),
            rust_path: rust_path.into(),
            relationship: CrossRefKind::Delegates,
        }
    }
```

</details>

### <span id="struct.DocModel"></span><code style="font-size: 0.85em">struct DocModel</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

The complete documentation model for a project

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `metadata` | `ProjectMetadata` |  |
| `rust_modules` | `Vec<RustModule>` |  |
| `python_modules` | `Vec<PythonModule>` |  |
| `cross_refs` | `Vec<CrossRef>` |  |

#### Methods

##### <span id="DocModel.test"></span><code>test</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn test(name: impl Into<String>) -> Self
```

Create an empty doc model for testing

<details>
<summary>Source</summary>

```rust
    pub fn test(name: impl Into<String>) -> Self {
        Self {
            metadata: ProjectMetadata::test(name),
            rust_modules: Vec::new(),
            python_modules: Vec::new(),
            cross_refs: Vec::new(),
        }
    }
```

</details>

##### <span id="DocModel.with_rust_module"></span><code>with_​rust_​module</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_rust_module(mut self, module: RustModule) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_rust_module(mut self, module: RustModule) -> Self {
        self.rust_modules.push(module);
        self
    }
```

</details>

##### <span id="DocModel.with_python_module"></span><code>with_​python_​module</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_python_module(mut self, module: PythonModule) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_python_module(mut self, module: PythonModule) -> Self {
        self.python_modules.push(module);
        self
    }
```

</details>

##### <span id="DocModel.with_cross_ref"></span><code>with_​cross_​ref</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_cross_ref(mut self, cross_ref: CrossRef) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_cross_ref(mut self, cross_ref: CrossRef) -> Self {
        self.cross_refs.push(cross_ref);
        self
    }
```

</details>

### <span id="struct.ProjectMetadata"></span><code style="font-size: 0.85em">struct ProjectMetadata</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

Project metadata

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` | Project name |
| `version` | `Option<String>` | Project version (from Cargo.toml or pyproject.toml) |
| `description` | `Option<String>` | Project description |
| `git_ref` | `Option<String>` | Git ref (branch or tag) this was generated from |
| `git_commit` | `Option<String>` | Git commit hash |
| `generated_at` | `String` | When the documentation was generated |

#### Methods

##### <span id="ProjectMetadata.test"></span><code>test</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn test(name: impl Into<String>) -> Self
```

Create test metadata

<details>
<summary>Source</summary>

```rust
    pub fn test(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: None,
            description: None,
            git_ref: None,
            git_commit: None,
            generated_at: "2024-01-01T00:00:00Z".to_string(),
        }
    }
```

</details>

##### <span id="ProjectMetadata.with_version"></span><code>with_​version</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_version(mut self, version: impl Into<String>) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }
```

</details>

##### <span id="ProjectMetadata.with_description"></span><code>with_​description</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_description(mut self, desc: impl Into<String>) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }
```

</details>

##### <span id="ProjectMetadata.with_git_ref"></span><code>with_​git_​ref</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_git_ref(mut self, git_ref: impl Into<String>) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_git_ref(mut self, git_ref: impl Into<String>) -> Self {
        self.git_ref = Some(git_ref.into());
        self
    }
```

</details>

##### <span id="ProjectMetadata.with_git_commit"></span><code>with_​git_​commit</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_git_commit(mut self, commit: impl Into<String>) -> Self
```

<details>
<summary>Source</summary>

```rust
    pub fn with_git_commit(mut self, commit: impl Into<String>) -> Self {
        self.git_commit = Some(commit.into());
        self
    }
```

</details>

## Enums

### <span id="enum.SourceType"></span><code style="font-size: 0.85em">enum SourceType</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

Source type indicator for Python modules

#### Variants

- **`Python`** - Pure Python source code
- **`PyO3Binding`** - Rust code exposed via PyO3
- **`Rust`** - Pure Rust (no Python exposure)

### <span id="enum.Visibility"></span><code style="font-size: 0.85em">enum Visibility</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

Visibility level for Rust items

#### Variants

- **`Public`**
- **`PubCrate`**
- **`PubSuper`**
- **`Private`**

### <span id="enum.PyModuleAddKind"></span><code style="font-size: 0.85em">enum PyModuleAddKind</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

What a [`PyModuleAdd`](../../../rust/plissken-core/model/#struct.PyModuleAdd) adds.

#### Variants

- **`Class`** - `m.add_class::<Type>()`
- **`Function`** - `m.add_function(wrap_pyfunction!(f, m)?)`
- **`Submodule`** - `m.add_submodule(&sub)`
- **`Call`** - `helper(m)`: the helper adds to the module
- **`Pymodule`** - `m.add_submodule(wrap_pymodule!(f))`: the `#[pymodule]` function
  `f` (the path) is a submodule

### <span id="enum.RustItem"></span><code style="font-size: 0.85em">enum RustItem</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

A Rust item (struct, enum, function, etc.)

#### Variants

- **`Struct`**
- **`Enum`**
- **`Function`**
- **`Trait`**
- **`Impl`**
- **`Const`**
- **`TypeAlias`**

### <span id="enum.PythonItem"></span><code style="font-size: 0.85em">enum PythonItem</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

A Python item

#### Variants

- **`Class`**
- **`Function`**
- **`Variable`**

### <span id="enum.CrossRefKind"></span><code style="font-size: 0.85em">enum CrossRefKind</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

Kind of cross-reference

#### Variants

- **`Binding`** - Direct PyO3 binding
- **`Wraps`** - Python wraps Rust
- **`Delegates`** - Python delegates to Rust
