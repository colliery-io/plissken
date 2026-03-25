# Data Model Reference

The documentation model (`DocModel`) is the intermediate representation
that plissken builds from source code. It can be output as JSON via
`plissken generate` or rendered to Markdown via `plissken render`.

## Top-Level Structure

```
DocModel
├── metadata: ProjectMetadata
├── rust_modules: [RustModule]
├── python_modules: [PythonModule]
└── cross_refs: [CrossRef]
```

---

## `ProjectMetadata`

| Field | Type | Description |
|-------|------|-------------|
| `name` | string | Project name from config. |
| `version` | string? | Project version (from Cargo.toml, pyproject.toml, or git). |
| `description` | string? | Project description, if available. |
| `git_ref` | string? | Current git branch or tag name. |
| `git_commit` | string? | Current git commit hash (short). |
| `generated_at` | string | ISO 8601 timestamp of when the model was generated. |

---

## `RustModule`

A parsed Rust source file.

| Field | Type | Description |
|-------|------|-------------|
| `path` | string | Module path (e.g., `"mycrate"`, `"mycrate::utils"`). |
| `doc_comment` | string? | The `//!` module-level doc comment. |
| `parsed_doc` | ParsedDocstring? | Structured version of the doc comment. |
| `items` | [RustItem] | Top-level items in the module. |
| `source` | SourceSpan | Source file location and content. |

### `RustItem`

Tagged union with `kind` discriminator:

| Kind | Type | Description |
|------|------|-------------|
| `"Struct"` | RustStruct | Struct definition. |
| `"Enum"` | RustEnum | Enum definition. |
| `"Function"` | RustFunction | Free function. |
| `"Trait"` | RustTrait | Trait definition. |
| `"Impl"` | RustImpl | Impl block (inherent or trait). |
| `"Const"` | RustConst | Const item. |
| `"TypeAlias"` | RustTypeAlias | Type alias. |

### `RustStruct`

| Field | Type | Description |
|-------|------|-------------|
| `name` | string | Struct name. |
| `visibility` | Visibility | `"Public"`, `"PubCrate"`, `"PubSuper"`, or `"Private"`. |
| `doc_comment` | string? | `///` doc comment. |
| `parsed_doc` | ParsedDocstring? | Structured doc. |
| `generics` | string? | Generic parameters (e.g., `"<T: Clone, const N: usize>"`). |
| `fields` | [RustField] | Struct fields. |
| `derives` | [string] | Derive macro names. |
| `pyclass` | PyClassMeta? | Present if `#[pyclass]` attribute found. |
| `source` | SourceSpan | Source location. |

### `RustEnum`

| Field | Type | Description |
|-------|------|-------------|
| `name` | string | Enum name. |
| `visibility` | Visibility | Visibility level. |
| `doc_comment` | string? | Doc comment. |
| `parsed_doc` | ParsedDocstring? | Structured doc. |
| `generics` | string? | Generic parameters. |
| `variants` | [RustVariant] | Enum variants. |
| `source` | SourceSpan | Source location. |

### `RustFunction`

| Field | Type | Description |
|-------|------|-------------|
| `name` | string | Function name. |
| `visibility` | Visibility | Visibility level. |
| `doc_comment` | string? | Doc comment. |
| `parsed_doc` | ParsedDocstring? | Structured doc. |
| `generics` | string? | Generic parameters. |
| `signature_str` | string | Full signature as display string. |
| `signature` | RustFunctionSig | Parsed signature. |
| `is_async` | bool | Whether the function is `async`. |
| `is_unsafe` | bool | Whether the function is `unsafe`. |
| `is_const` | bool | Whether the function is `const`. |
| `pyfunction` | PyFunctionMeta? | Present if `#[pyfunction]` attribute found. |
| `source` | SourceSpan | Source location. |

### `RustTrait`

| Field | Type | Description |
|-------|------|-------------|
| `name` | string | Trait name. |
| `visibility` | Visibility | Visibility level. |
| `doc_comment` | string? | Doc comment. |
| `parsed_doc` | ParsedDocstring? | Structured doc. |
| `generics` | string? | Generic parameters. |
| `bounds` | string? | Supertraits (e.g., `": Clone + Send"`). |
| `associated_types` | [RustAssociatedType] | Associated type declarations. |
| `methods` | [RustFunction] | Trait methods. |
| `source` | SourceSpan | Source location. |

### `RustImpl`

| Field | Type | Description |
|-------|------|-------------|
| `generics` | string? | Impl-level generic parameters. |
| `target` | string | Type being implemented for. |
| `trait_` | string? | Trait name, if this is a trait impl. |
| `where_clause` | string? | Where clause constraints. |
| `methods` | [RustFunction] | Methods in the impl block. |
| `pymethods` | bool | Whether this is a `#[pymethods]` block. |
| `source` | SourceSpan | Source location. |

### `RustConst`

| Field | Type | Description |
|-------|------|-------------|
| `name` | string | Constant name. |
| `visibility` | Visibility | Visibility level. |
| `doc_comment` | string? | Doc comment. |
| `ty` | string | Type as string. |
| `value` | string? | Value expression as string. |
| `source` | SourceSpan | Source location. |

### `RustTypeAlias`

| Field | Type | Description |
|-------|------|-------------|
| `name` | string | Alias name. |
| `visibility` | Visibility | Visibility level. |
| `doc_comment` | string? | Doc comment. |
| `generics` | string? | Generic parameters. |
| `ty` | string | The aliased type as string. |
| `source` | SourceSpan | Source location. |

### Supporting Types

**`RustField`**: `{ name: string, ty: string, visibility: Visibility, doc_comment: string? }`

**`RustVariant`**: `{ name: string, doc_comment: string?, fields: [RustField] }`

**`RustFunctionSig`**: `{ params: [RustParam], return_type: string? }`

**`RustParam`**: `{ name: string, ty: string, default: string? }`

**`RustAssociatedType`**: `{ name: string, doc_comment: string?, generics: string?, bounds: string? }`

**`PyClassMeta`**: `{ name: string?, module: string? }`

**`PyFunctionMeta`**: `{ name: string?, signature: string? }`

**`Visibility`**: `"Public"` | `"PubCrate"` | `"PubSuper"` | `"Private"`

---

## `PythonModule`

A parsed Python source file.

| Field | Type | Description |
|-------|------|-------------|
| `path` | string | Dotted module path (e.g., `"mypackage.utils"`). |
| `docstring` | string? | Module docstring. |
| `parsed_doc` | ParsedDocstring? | Structured version of the docstring. |
| `items` | [PythonItem] | Top-level items in the module. |
| `source_type` | SourceType | `"Python"`, `"PyO3Binding"`, or `"Rust"`. |
| `source` | SourceSpan | Source file location. |

### `PythonItem`

Tagged union with `kind` discriminator:

| Kind | Type | Description |
|------|------|-------------|
| `"Class"` | PythonClass | Class definition. |
| `"Function"` | PythonFunction | Function definition. |
| `"Variable"` | PythonVariable | Module-level variable/constant. |

### `PythonClass`

| Field | Type | Description |
|-------|------|-------------|
| `name` | string | Class name. |
| `docstring` | string? | Class docstring. |
| `parsed_doc` | ParsedDocstring? | Structured doc. |
| `bases` | [string] | Base class names. |
| `methods` | [PythonFunction] | Methods. |
| `attributes` | [PythonVariable] | Class attributes. |
| `decorators` | [string] | Decorator names. |
| `rust_impl` | RustItemRef? | Cross-reference to Rust implementation. |
| `source` | SourceSpan | Source location. |

### `PythonFunction`

| Field | Type | Description |
|-------|------|-------------|
| `name` | string | Function name. |
| `docstring` | string? | Docstring. |
| `signature_str` | string | Full signature as display string. |
| `signature` | PythonFunctionSig | Parsed signature. |
| `decorators` | [string] | Decorator names. |
| `is_async` | bool | Whether the function uses `async def`. |
| `is_staticmethod` | bool | Whether decorated with `@staticmethod`. |
| `is_classmethod` | bool | Whether decorated with `@classmethod`. |
| `is_property` | bool | Whether decorated with `@property`. |
| `parsed_doc` | ParsedDocstring? | Structured doc. |
| `rust_impl` | RustItemRef? | Cross-reference to Rust implementation. |
| `source` | SourceSpan | Source location. |

### `PythonVariable`

| Field | Type | Description |
|-------|------|-------------|
| `name` | string | Variable name. |
| `ty` | string? | Type annotation. |
| `value` | string? | Assigned value as string. |
| `docstring` | string? | Variable docstring. |

### Supporting Types

**`PythonFunctionSig`**: `{ params: [PythonParam], return_type: string? }`

**`PythonParam`**: `{ name: string, ty: string?, default: string? }`

**`SourceType`**: `"Python"` | `"PyO3Binding"` | `"Rust"`

---

## `CrossRef`

A cross-reference between Python and Rust items.

| Field | Type | Description |
|-------|------|-------------|
| `python_path` | string | Dotted Python path (e.g., `"mypackage.MyClass"`). |
| `rust_path` | string | Rust path (e.g., `"mycrate::MyStruct"`). |
| `relationship` | CrossRefKind | Type of relationship. |

### `CrossRefKind`

| Value | Description |
|-------|-------------|
| `"Binding"` | Direct PyO3 binding (`#[pyclass]`, `#[pyfunction]`). |
| `"Wraps"` | Python class wraps a Rust type. |
| `"Delegates"` | Python function delegates to Rust. |

---

## `RustItemRef`

A reference from a Python item to its Rust implementation.

| Field | Type | Description |
|-------|------|-------------|
| `path` | string | Rust module path. |
| `name` | string | Rust item name. |

---

## `ParsedDocstring`

Structured documentation extracted from docstrings or doc comments.

| Field | Type | Description |
|-------|------|-------------|
| `summary` | string? | First paragraph (summary line). |
| `description` | string? | Remaining description after the summary. |
| `params` | [ParamDoc] | Documented parameters. |
| `returns` | ReturnDoc? | Return value documentation. |
| `raises` | [RaisesDoc] | Documented exceptions/errors. |
| `examples` | [string] | Example code blocks. |

**`ParamDoc`**: `{ name: string, ty: string?, description: string }`

**`ReturnDoc`**: `{ ty: string?, description: string }`

**`RaisesDoc`**: `{ ty: string, description: string }`

---

## `SourceSpan`

Location and content of a source code region.

| Field | Type | Description |
|-------|------|-------------|
| `location` | SourceLocation | File path and line numbers. |
| `source` | string | The actual source code text. |

**`SourceLocation`**: `{ file: string, line_start: number, line_end: number }`
