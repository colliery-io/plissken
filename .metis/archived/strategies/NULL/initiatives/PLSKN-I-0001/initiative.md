---
id: unified-documentation-model
level: initiative
title: "Unified Documentation Model"
short_code: "PLSKN-I-0001"
created_at: 2026-01-14T03:09:26.307074+00:00
updated_at: 2026-01-14T14:24:02.746246+00:00
parent: PLSKN-V-0001
blocked_by: []
archived: true

tags:
  - "#initiative"
  - "#phase/completed"


exit_criteria_met: false
estimated_complexity: M
strategy_id: NULL
initiative_id: unified-documentation-model
---

# Unified Documentation Model Initiative

## Context

Before building parsers (Rust/Python) or renderers (Markdown/HTML), we need a well-defined data model that:
1. Captures everything we need from Rust source (structs, functions, traits, PyO3 metadata)
2. Captures everything we need from Python source (classes, functions, docstrings)
3. Enables cross-referencing between Rust and Python items
4. Provides enough structure for rendering decisions (collapsible code, badges, etc.)

A sketch exists in `crates/plissken-core/src/model.rs` but needs validation against our test fixtures.

## Goals & Non-Goals

**Goals:**
- Define Rust item types (Module, Struct, Enum, Function, Trait, Impl)
- Define Python item types (Module, Class, Function, Variable)
- Capture PyO3 metadata (#[pyclass], #[pyfunction], #[pyo3(signature)])
- Support cross-reference graph (Python item ↔ Rust implementation)
- Handle complex generics, lifetimes, and type signatures as strings (don't over-parse)
- Support parsed docstrings (Google/NumPy/rustdoc sections)
- Include source locations (file, line range) for all items

**Non-Goals:**
- Full type system modeling (we store signatures as strings, not ASTs)
- Semantic analysis (type checking, trait resolution)
- Incremental updates (model is built fresh each run)

## Detailed Design

### Core Principles

1. **Strings for complex types**: Store `Vec<HashMap<String, Option<T>>>` as a string, not a nested type structure. Parsing is expensive and rendering just needs the string anyway.

2. **Source locations everywhere**: Every item tracks its file and line range for "view source" links.

3. **Optional PyO3 metadata**: Rust items optionally have PyO3 annotations. When present, they're linked to Python items.

4. **Parsed docstrings**: Docstrings are parsed into structured sections (summary, params, returns, raises, examples) but original text is preserved.

### Rust Model

```
RustModule
├── path: String ("crate::module::submodule")
├── doc_comment: Option<String>
├── items: Vec<RustItem>
└── location: SourceLocation

RustItem = Struct | Enum | Function | Trait | Impl | Const | TypeAlias

RustStruct
├── name, visibility, doc_comment
├── generics: String (e.g., "<T: Clone, const N: usize>")
├── fields: Vec<RustField>
├── derives: Vec<String>
├── pyclass: Option<PyClassMeta>
└── location

RustFunction
├── name, visibility, doc_comment
├── signature: String (full signature as string)
├── params: Vec<RustParam> (parsed for structured access)
├── return_type: Option<String>
├── is_async, is_unsafe
├── pyfunction: Option<PyFunctionMeta>
└── location
```

### Python Model

```
PythonModule
├── path: String ("package.module")
├── docstring: Option<String>
├── items: Vec<PythonItem>
├── source_type: SourceType (Python | PyO3Binding)
└── location

PythonItem = Class | Function | Variable

PythonClass
├── name, docstring
├── bases: Vec<String>
├── methods: Vec<PythonFunction>
├── decorators: Vec<String>
├── rust_impl: Option<RustItemRef>
└── location

PythonFunction
├── name, docstring
├── signature: String
├── params: Vec<PythonParam>
├── return_type: Option<String>
├── is_async
├── parsed_doc: Option<ParsedDocstring>
├── rust_impl: Option<RustItemRef>
└── location
```

### Cross-Reference Graph

```
CrossRef
├── python_path: String
├── rust_path: String
└── relationship: Binding | Wraps | Delegates

DocModel (top-level)
├── rust_modules: Vec<RustModule>
├── python_modules: Vec<PythonModule>
├── cross_refs: Vec<CrossRef>
└── metadata: ProjectMetadata
```

### Key Decisions

| Decision | Rationale |
|----------|-----------|
| Signatures as strings | Complex generics/lifetimes are hard to model; rendering needs strings anyway |
| Separate parsed params | Structured access for parameter tables, but original signature preserved |
| SourceType from config | Don't try to auto-detect PyO3 vs pure Python; user specifies in config |
| CrossRef is explicit | Built after parsing both sides; links by path strings |

## Testing Strategy

### Validation Against Fixtures

For each test fixture, manually specify expected model output:
- `hybrid_binary/` → expected Rust items with PyO3 metadata, Python items linked
- `separate_bindings/` → pure Rust items in core, PyO3 items in bindings, cross-refs
- `pure_rust/` → Rust items only, no PyO3 metadata
- `pure_python/` → Python items only, no Rust links
- `complex_generics/` → verify signature strings capture full complexity

### Unit Tests

- Serialization roundtrip (model → JSON → model)
- Builder APIs for constructing test models
- Equality and debug derive validation

## Alternatives Considered

**Full type AST**: Model generic types as nested structures (e.g., `Type::Vec(Type::Option(Type::Named("T")))`). Rejected because: too complex, rendering doesn't need it, edge cases are endless.

**Single unified item type**: One `DocItem` enum for both Rust and Python. Rejected because: languages are different enough that separate types are clearer.

**Auto-detect source type**: Inspect imports or .so files to determine if Python module is PyO3. Rejected because: fragile, config is explicit and clear.

## Implementation Plan

1. ~~Refine `model.rs` based on this design~~ DONE
2. ~~Add serde Serialize/Deserialize~~ DONE (already had it)
3. ~~Add builder APIs for testing~~ DONE
4. ~~Write expected model JSON for each test fixture~~ DONE (hybrid_binary.json)
5. Document the model (this becomes plissken's own dogfood)

## Progress

### Session 1 (2026-01-14)
- Refined model.rs with all item types from design
- Added generics fields to RustStruct, RustEnum, RustFunction, RustTrait, RustImpl
- Added RustConst and RustTypeAlias variants
- Added signature_str for full string signatures plus parsed signature structs
- Added RustAssociatedType for GATs support
- Added DocModel and ProjectMetadata top-level containers
- Added comprehensive builder APIs for testing
- Created expected JSON for hybrid_binary fixture
- All 6 model tests passing