# model <span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #ff5722; color: white;">Rust</span>


Unified documentation model for Rust and Python items

## Structs

### `struct SourceLocation`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

A reference to a location in source code

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `file` | `PathBuf` |  |
| `line_start` | `usize` |  |
| `line_end` | `usize` |  |

#### Methods

##### `test` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn test (file : impl Into < PathBuf > , line_start : usize , line_end : usize) -> Self
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





### `struct SourceSpan`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

Source code span with the actual text

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `location` | `SourceLocation` |  |
| `source` | `String` | The actual source code text |

#### Methods

##### `test` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn test (file : impl Into < PathBuf > , line_start : usize , line_end : usize) -> Self
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



##### `new` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn new (file : impl Into < PathBuf > , line_start : usize , line_end : usize , source : impl Into < String > ,) -> Self
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





### `struct RustModule`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

A Rust module with its items

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `path` | `String` |  |
| `doc_comment` | `Option < String >` |  |
| `items` | `Vec < RustItem >` |  |
| `source` | `SourceSpan` |  |

#### Methods

##### `test` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn test (path : impl Into < String >) -> Self
```

Create a test Rust module

<details>
<summary>Source</summary>

```rust
    pub fn test(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            doc_comment: None,
            items: Vec::new(),
            source: SourceSpan::test("test.rs", 1, 1),
        }
    }
```

</details>



##### `with_doc` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_doc (mut self , doc : impl Into < String >) -> Self
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



##### `with_item` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_item (mut self , item : RustItem) -> Self
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



##### `with_source` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_source (mut self , source : SourceSpan) -> Self
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





### `struct RustStruct`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

A Rust struct definition

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` |  |
| `visibility` | `Visibility` |  |
| `doc_comment` | `Option < String >` |  |
| `generics` | `Option < String >` | Generic parameters as string, e.g. "<T: Clone, const N: usize>" |
| `fields` | `Vec < RustField >` |  |
| `derives` | `Vec < String >` |  |
| `pyclass` | `Option < PyClassMeta >` |  |
| `source` | `SourceSpan` |  |

#### Methods

##### `test` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn test (name : impl Into < String >) -> Self
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
            generics: None,
            fields: Vec::new(),
            derives: Vec::new(),
            pyclass: None,
            source: SourceSpan::test("test.rs", 1, 1),
        }
    }
```

</details>



##### `with_visibility` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_visibility (mut self , vis : Visibility) -> Self
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



##### `with_doc` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_doc (mut self , doc : impl Into < String >) -> Self
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



##### `with_generics` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_generics (mut self , generics : impl Into < String >) -> Self
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



##### `with_field` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_field (mut self , field : RustField) -> Self
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



##### `with_derive` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_derive (mut self , derive : impl Into < String >) -> Self
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



##### `with_pyclass` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_pyclass (mut self , meta : PyClassMeta) -> Self
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



##### `with_source` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_source (mut self , source : SourceSpan) -> Self
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





### `struct PyClassMeta`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

PyO3 #[pyclass] metadata

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `Option < String >` |  |
| `module` | `Option < String >` |  |

#### Methods

##### `new` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn new () -> Self
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



##### `with_name` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_name (mut self , name : impl Into < String >) -> Self
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



##### `with_module` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_module (mut self , module : impl Into < String >) -> Self
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





### `struct RustField`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

A Rust field

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` |  |
| `ty` | `String` |  |
| `visibility` | `Visibility` |  |
| `doc_comment` | `Option < String >` |  |

#### Methods

##### `test` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn test (name : impl Into < String > , ty : impl Into < String >) -> Self
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



##### `private` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn private (mut self) -> Self
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



##### `with_doc` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_doc (mut self , doc : impl Into < String >) -> Self
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





### `struct RustEnum`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

A Rust enum definition

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` |  |
| `visibility` | `Visibility` |  |
| `doc_comment` | `Option < String >` |  |
| `generics` | `Option < String >` | Generic parameters as string |
| `variants` | `Vec < RustVariant >` |  |
| `source` | `SourceSpan` |  |



### `struct RustVariant`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

A Rust enum variant

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` |  |
| `doc_comment` | `Option < String >` |  |
| `fields` | `Vec < RustField >` |  |



### `struct RustFunction`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

A Rust function definition

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` |  |
| `visibility` | `Visibility` |  |
| `doc_comment` | `Option < String >` |  |
| `generics` | `Option < String >` | Generic parameters as string, e.g. "<'a, T: Clone>" |
| `signature_str` | `String` | Full signature as string for display |
| `signature` | `RustFunctionSig` | Parsed signature for structured access |
| `is_async` | `bool` |  |
| `is_unsafe` | `bool` |  |
| `is_const` | `bool` |  |
| `pyfunction` | `Option < PyFunctionMeta >` |  |
| `source` | `SourceSpan` |  |

#### Methods

##### `test` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn test (name : impl Into < String >) -> Self
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



##### `with_doc` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_doc (mut self , doc : impl Into < String >) -> Self
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



##### `with_generics` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_generics (mut self , generics : impl Into < String >) -> Self
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



##### `with_signature` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_signature (mut self , sig : impl Into < String >) -> Self
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



##### `with_param` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_param (mut self , param : RustParam) -> Self
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



##### `with_return_type` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_return_type (mut self , ty : impl Into < String >) -> Self
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



##### `async_` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn async_ (mut self) -> Self
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



##### `unsafe_` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn unsafe_ (mut self) -> Self
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



##### `const_` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn const_ (mut self) -> Self
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



##### `with_pyfunction` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_pyfunction (mut self , meta : PyFunctionMeta) -> Self
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



##### `with_source` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_source (mut self , source : SourceSpan) -> Self
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





### `struct RustFunctionSig`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

Rust function signature

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `params` | `Vec < RustParam >` |  |
| `return_type` | `Option < String >` |  |



### `struct RustParam`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

A Rust function parameter

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` |  |
| `ty` | `String` |  |
| `default` | `Option < String >` |  |

#### Methods

##### `test` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn test (name : impl Into < String > , ty : impl Into < String >) -> Self
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



##### `with_default` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_default (mut self , default : impl Into < String >) -> Self
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





### `struct PyFunctionMeta`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

PyO3 #[pyfunction] metadata

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `Option < String >` |  |
| `signature` | `Option < String >` |  |

#### Methods

##### `new` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn new () -> Self
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



##### `with_name` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_name (mut self , name : impl Into < String >) -> Self
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



##### `with_signature` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_signature (mut self , sig : impl Into < String >) -> Self
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





### `struct RustTrait`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

A Rust trait definition

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` |  |
| `visibility` | `Visibility` |  |
| `doc_comment` | `Option < String >` |  |
| `generics` | `Option < String >` | Generic parameters as string |
| `bounds` | `Option < String >` | Supertraits as string, e.g. ": Clone + Send" |
| `associated_types` | `Vec < RustAssociatedType >` |  |
| `methods` | `Vec < RustFunction >` |  |
| `source` | `SourceSpan` |  |



### `struct RustAssociatedType`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

A Rust associated type in a trait

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` |  |
| `doc_comment` | `Option < String >` |  |
| `generics` | `Option < String >` | Generic parameters (for GATs) |
| `bounds` | `Option < String >` | Bounds on the associated type |



### `struct RustImpl`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

A Rust impl block

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `generics` | `Option < String >` | Generic parameters on the impl block |
| `target` | `String` | The type being implemented for |
| `trait_` | `Option < String >` | Trait being implemented (if any) |
| `where_clause` | `Option < String >` | Where clause constraints |
| `methods` | `Vec < RustFunction >` |  |
| `pymethods` | `bool` | Whether this is a #[pymethods] block |
| `source` | `SourceSpan` |  |



### `struct RustConst`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

A Rust const item

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` |  |
| `visibility` | `Visibility` |  |
| `doc_comment` | `Option < String >` |  |
| `ty` | `String` |  |
| `value` | `Option < String >` |  |
| `source` | `SourceSpan` |  |



### `struct RustTypeAlias`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

A Rust type alias

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` |  |
| `visibility` | `Visibility` |  |
| `doc_comment` | `Option < String >` |  |
| `generics` | `Option < String >` |  |
| `ty` | `String` | The aliased type as string |
| `source` | `SourceSpan` |  |



### `struct PythonModule`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

A Python module

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `path` | `String` |  |
| `docstring` | `Option < String >` |  |
| `items` | `Vec < PythonItem >` |  |
| `source_type` | `SourceType` |  |
| `source` | `SourceSpan` |  |

#### Methods

##### `test` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn test (path : impl Into < String >) -> Self
```

Create a test Python module

<details>
<summary>Source</summary>

```rust
    pub fn test(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            docstring: None,
            items: Vec::new(),
            source_type: SourceType::Python,
            source: SourceSpan::test("test.py", 1, 1),
        }
    }
```

</details>



##### `with_docstring` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_docstring (mut self , doc : impl Into < String >) -> Self
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



##### `with_item` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_item (mut self , item : PythonItem) -> Self
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



##### `pyo3_binding` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn pyo3_binding (mut self) -> Self
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



##### `with_source` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_source (mut self , source : SourceSpan) -> Self
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





### `struct PythonClass`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

A Python class

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` |  |
| `docstring` | `Option < String >` |  |
| `bases` | `Vec < String >` |  |
| `methods` | `Vec < PythonFunction >` |  |
| `attributes` | `Vec < PythonVariable >` |  |
| `decorators` | `Vec < String >` |  |
| `rust_impl` | `Option < RustItemRef >` |  |
| `source` | `SourceSpan` |  |

#### Methods

##### `test` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn test (name : impl Into < String >) -> Self
```

Create a test Python class

<details>
<summary>Source</summary>

```rust
    pub fn test(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            docstring: None,
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



##### `with_docstring` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_docstring (mut self , doc : impl Into < String >) -> Self
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



##### `with_base` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_base (mut self , base : impl Into < String >) -> Self
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



##### `with_method` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_method (mut self , method : PythonFunction) -> Self
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



##### `with_attribute` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_attribute (mut self , attr : PythonVariable) -> Self
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



##### `with_decorator` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_decorator (mut self , decorator : impl Into < String >) -> Self
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



##### `with_rust_impl` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_rust_impl (mut self , rust_ref : RustItemRef) -> Self
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



##### `with_source` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_source (mut self , source : SourceSpan) -> Self
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





### `struct PythonFunction`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

A Python function

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` |  |
| `docstring` | `Option < String >` |  |
| `signature_str` | `String` | Full signature as string for display |
| `signature` | `PythonFunctionSig` | Parsed signature for structured access |
| `decorators` | `Vec < String >` |  |
| `is_async` | `bool` |  |
| `is_staticmethod` | `bool` |  |
| `is_classmethod` | `bool` |  |
| `is_property` | `bool` |  |
| `parsed_doc` | `Option < ParsedDocstring >` |  |
| `rust_impl` | `Option < RustItemRef >` |  |
| `source` | `SourceSpan` |  |

#### Methods

##### `test` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn test (name : impl Into < String >) -> Self
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



##### `with_docstring` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_docstring (mut self , doc : impl Into < String >) -> Self
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



##### `with_signature` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_signature (mut self , sig : impl Into < String >) -> Self
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



##### `with_param` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_param (mut self , param : PythonParam) -> Self
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



##### `with_return_type` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_return_type (mut self , ty : impl Into < String >) -> Self
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



##### `with_decorator` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_decorator (mut self , decorator : impl Into < String >) -> Self
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



##### `async_` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn async_ (mut self) -> Self
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



##### `staticmethod` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn staticmethod (mut self) -> Self
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



##### `classmethod` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn classmethod (mut self) -> Self
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



##### `property` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn property (mut self) -> Self
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



##### `with_rust_impl` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_rust_impl (mut self , rust_ref : RustItemRef) -> Self
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



##### `with_source` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_source (mut self , source : SourceSpan) -> Self
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





### `struct PythonFunctionSig`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

Python function signature

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `params` | `Vec < PythonParam >` |  |
| `return_type` | `Option < String >` |  |



### `struct PythonParam`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

A Python function parameter

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` |  |
| `ty` | `Option < String >` |  |
| `default` | `Option < String >` |  |

#### Methods

##### `test` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn test (name : impl Into < String >) -> Self
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



##### `with_type` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_type (mut self , ty : impl Into < String >) -> Self
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



##### `with_default` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_default (mut self , default : impl Into < String >) -> Self
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





### `struct PythonVariable`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

A Python variable/attribute

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` |  |
| `ty` | `Option < String >` |  |
| `value` | `Option < String >` |  |
| `docstring` | `Option < String >` |  |

#### Methods

##### `test` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn test (name : impl Into < String >) -> Self
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



##### `with_type` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_type (mut self , ty : impl Into < String >) -> Self
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



##### `with_value` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_value (mut self , value : impl Into < String >) -> Self
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



##### `with_docstring` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_docstring (mut self , doc : impl Into < String >) -> Self
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





### `struct ParsedDocstring`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

Parsed docstring (Google/NumPy style)

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `summary` | `Option < String >` |  |
| `description` | `Option < String >` |  |
| `params` | `Vec < ParamDoc >` |  |
| `returns` | `Option < ReturnDoc >` |  |
| `raises` | `Vec < RaisesDoc >` |  |
| `examples` | `Vec < String >` |  |



### `struct ParamDoc`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

Documented parameter

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` |  |
| `ty` | `Option < String >` |  |
| `description` | `String` |  |



### `struct ReturnDoc`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

Documented return value

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `ty` | `Option < String >` |  |
| `description` | `String` |  |



### `struct RaisesDoc`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

Documented exception

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `ty` | `String` |  |
| `description` | `String` |  |



### `struct RustItemRef`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

Reference to a Rust item

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `path` | `String` |  |
| `name` | `String` |  |

#### Methods

##### `new` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn new (path : impl Into < String > , name : impl Into < String >) -> Self
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





### `struct CrossRef`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

Cross-reference between Python and Rust

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `python_path` | `String` |  |
| `rust_path` | `String` |  |
| `relationship` | `CrossRefKind` |  |

#### Methods

##### `binding` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn binding (python_path : impl Into < String > , rust_path : impl Into < String >) -> Self
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



##### `wraps` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn wraps (python_path : impl Into < String > , rust_path : impl Into < String >) -> Self
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



##### `delegates` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn delegates (python_path : impl Into < String > , rust_path : impl Into < String >) -> Self
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





### `struct DocModel`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

The complete documentation model for a project

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `metadata` | `ProjectMetadata` |  |
| `rust_modules` | `Vec < RustModule >` |  |
| `python_modules` | `Vec < PythonModule >` |  |
| `cross_refs` | `Vec < CrossRef >` |  |

#### Methods

##### `test` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn test (name : impl Into < String >) -> Self
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



##### `with_rust_module` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_rust_module (mut self , module : RustModule) -> Self
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



##### `with_python_module` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_python_module (mut self , module : PythonModule) -> Self
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



##### `with_cross_ref` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_cross_ref (mut self , cross_ref : CrossRef) -> Self
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





### `struct ProjectMetadata`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

Project metadata

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` | Project name |
| `version` | `Option < String >` | Project version (from Cargo.toml or pyproject.toml) |
| `description` | `Option < String >` | Project description |
| `git_ref` | `Option < String >` | Git ref (branch or tag) this was generated from |
| `git_commit` | `Option < String >` | Git commit hash |
| `generated_at` | `String` | When the documentation was generated |

#### Methods

##### `test` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn test (name : impl Into < String >) -> Self
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



##### `with_version` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_version (mut self , version : impl Into < String >) -> Self
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



##### `with_description` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_description (mut self , desc : impl Into < String >) -> Self
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



##### `with_git_ref` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_git_ref (mut self , git_ref : impl Into < String >) -> Self
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



##### `with_git_commit` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_git_commit (mut self , commit : impl Into < String >) -> Self
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

### `enum SourceType` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


Source type indicator for Python modules

#### Variants

- **`Python`** - Pure Python source code
- **`PyO3Binding`** - Rust code exposed via PyO3
- **`Rust`** - Pure Rust (no Python exposure)



### `enum Visibility` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


Visibility level for Rust items

#### Variants

- **`Public`**
- **`PubCrate`**
- **`PubSuper`**
- **`Private`**



### `enum RustItem` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


A Rust item (struct, enum, function, etc.)

#### Variants

- **`Struct`**
- **`Enum`**
- **`Function`**
- **`Trait`**
- **`Impl`**
- **`Const`**
- **`TypeAlias`**



### `enum PythonItem` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


A Python item

#### Variants

- **`Class`**
- **`Function`**
- **`Variable`**



### `enum CrossRefKind` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


Kind of cross-reference

#### Variants

- **`Binding`** - Direct PyO3 binding
- **`Wraps`** - Python wraps Rust
- **`Delegates`** - Python delegates to Rust



