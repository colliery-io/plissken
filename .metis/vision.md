---
id: plissken
level: vision
title: "plissken"
short_code: "PLSKN-V-0001"
created_at: 2026-01-14T01:53:52.659764+00:00
updated_at: 2026-01-14T03:01:54.828737+00:00
archived: false

tags:
  - "#vision"
  - "#phase/published"


exit_criteria_met: false
strategy_id: NULL
initiative_id: NULL
---

# Plissken Vision

## Purpose

Plissken is **documentation for the Rust-Python bridge**.

If you're building a Python library with Rust (PyO3, maturin), you have a documentation problem: rustdoc shows your Rust internals, Sphinx/mkdocstrings shows your Python API, but **nothing shows them together**. Users see Python signatures with no understanding of the Rust beneath. Contributors see Rust code with no context for how it surfaces to Python.

Plissken generates unified documentation that shows both sides—Python API and Rust implementation—with automatic cross-linking, version tracking, and quality metrics. Output is Markdown (with embedded HTML/CSS where needed) for integration into your existing static site.

## The Problem

**The PyO3/maturin ecosystem is exploding**: polars, pydantic-core, ruff, cryptography, orjson, tokenizers...

**But documentation tooling hasn't caught up:**

| Tool | What it does | What's missing |
|------|--------------|----------------|
| rustdoc | Documents Rust code | No Python context, HTML-only output |
| Sphinx/mkdocstrings | Documents Python API | No visibility into Rust implementation |
| Neither | — | No cross-linking between the two |

When someone reads your Python docs and wonders "what does this actually do under the hood?", they're on their own. When a contributor reads the Rust code and wonders "how does this surface to users?", same problem.

## The Solution

Plissken parses **both** your Rust source (including `#[pyfunction]`, `#[pyclass]`, `#[pymethods]`) and your Python stubs/modules, then generates unified documentation:

```
┌─────────────────────────────────────────────────────────┐
│  DataFrame.filter(predicate: Expr) -> DataFrame         │  ← Python signature
├─────────────────────────────────────────────────────────┤
│  Filters rows based on a boolean expression.            │  ← Python docstring
│                                                         │
│  ## Parameters                                          │
│  | Name      | Type | Description           |          │
│  |-----------|------|------------------------|          │
│  | predicate | Expr | Boolean filter expr    |          │
│                                                         │
│  ┌─ Rust Implementation ──────────────────────────────┐ │
│  │ pub fn filter(&self, predicate: PyExpr) -> Self    │ │  ← Rust signature
│  │ → See: rust::LazyFrame::filter                     │ │  ← Link to Rust docs
│  └────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────┘
```

## Target Audience

- **Primary**: Maintainers of PyO3/maturin projects who want unified docs
- **Secondary**: Pure Rust or pure Python projects wanting better Markdown doc generation

## Key Capabilities

### 1. Unified Python + Rust Documentation
- Parse `#[pyfunction]`, `#[pyclass]`, `#[pymethods]` to understand the bridge
- Show Python API with Rust implementation context
- Automatic cross-links: Python item ↔ Rust implementation

### 2. Smart Cross-References
- **Auto-linking**: Types in signatures become clickable links
- **Manual syntax**: `[[symbol]]` in docstrings for explicit references  
- **Cross-language**: `[[rust::module::item]]` or `[[py::module.item]]`
- **Broken link detection**: Warnings at generation time

### 3. Version-Aware Documentation
- Git-based: generates docs for `main`/`master` + all tags
- API diff: "What changed in the public API since v1.2?"
- Deprecation tracking across versions

### 4. Documentation Quality Metrics
- Coverage: "78% of public API is documented"
- Completeness warnings: missing descriptions, no examples
- CI-friendly: fail builds on undocumented public items (optional)

### 5. Design That Doesn't Suck
- Smart signature formatting (line breaks, type highlighting)
- Semantic badges: `async`, `unsafe`, `deprecated`, visibility
- HTML-in-Markdown for rich formatting where pure Markdown falls short
- Bundled CSS that works with MkDocs Material, mdBook, Docusaurus

## What We're NOT Building

- Generic "any language" documentation (focus is Rust+Python)
- AI-powered doc generation (your IDE already does this)
- Semantic search (handled by indexers)
- Watch mode / incremental builds (nice-to-have, not core)

## Success Criteria

**Core functionality:**
- Correctly parse PyO3 annotations (`#[pyfunction]`, `#[pyclass]`, etc.)
- Generate unified view showing Python API + Rust implementation
- Cross-links work in both directions
- Git-based versioning works for main + tags

**Quality:**
- Documentation coverage metrics are accurate
- Broken link detection has zero false positives
- Output integrates cleanly with MkDocs Material and mdBook

**Design:**
- Generated docs pass the "would I actually read this?" test
- Complex signatures remain readable
- Visually competitive with hand-crafted documentation

**Dogfooding:**
- Plissken's own docs are generated by plissken

## Design Philosophy

Auto-generated docs are ugly because they treat documentation as a data dump, not communication.

**We reject:**
- Monospace signature blobs
- Parameter walls of text
- No visual hierarchy
- "It has the information, what more do you want?"

**We embrace:**
- Visual hierarchy (headers, badges, callouts)
- Scannable layouts (parameter tables)
- Smart signature formatting (line breaks, type highlighting)
- HTML/CSS where Markdown isn't enough
- Semantic markers (`async`, `deprecated`, `unsafe`, `pub`/`pub(crate)`/private)

## Project Types

Plissken has first-class support for the **"Rust library + Python bindings"** pattern:

### Type 1: Hybrid Binary (angreal pattern)
```
my-project/
├── Cargo.toml              # Rust workspace
├── crates/
│   └── my-crate/
│       └── src/
│           ├── lib.rs      # Core Rust + PyO3 bindings together
│           └── python_bindings/
├── pyproject.toml          # maturin build
└── python/
    └── my_package/         # Optional pure Python helpers
```

Single crate that IS both the Rust implementation and Python bindings. The `#[pyclass]` structs might wrap internal Rust types or be the primary types themselves.

### Type 2: Separate Bindings (cloacina/cloaca pattern)
```
my-project/
├── crates/
│   ├── cloacina/           # Pure Rust library (no PyO3)
│   └── cloacina-macros/    # Proc macros
├── bindings/
│   └── cloaca/             # Python bindings crate
│       ├── src/lib.rs      # PyO3 wrappers around cloacina
│       └── python/
│           └── cloaca/     # Pure Python helpers
└── Cargo.toml
```

Rust library exists standalone; bindings are a separate crate that wraps it. Python users never touch cloacina directly.

### Type 3: Pure Rust / Pure Python
Also supports single-language projects for teams that want consistent doc tooling across their stack.

## Data Model

What plissken extracts and stores (before rendering):

### Rust Items
```rust
RustModule {
    path: "crate::module::submodule",
    doc_comment: String,
    items: Vec<RustItem>,
}

RustItem = Struct | Enum | Function | Trait | Impl | Const | TypeAlias

RustStruct {
    name: String,
    visibility: Pub | PubCrate | Private,
    doc_comment: String,
    fields: Vec<Field>,
    derives: Vec<String>,
    
    // PyO3 metadata (if present)
    pyclass: Option<PyClassMeta>,  // #[pyclass(name = "...", module = "...")]
}

RustFunction {
    name: String,
    visibility: Visibility,
    doc_comment: String,
    signature: FunctionSig,
    is_async: bool,
    is_unsafe: bool,
    
    // PyO3 metadata
    pyfunction: Option<PyFunctionMeta>,
    pyo3_signature: Option<String>,  // #[pyo3(signature = (...))]
}
```

### Python Items
```rust
PythonModule {
    path: "package.module",
    docstring: String,
    items: Vec<PythonItem>,
    source_type: PyO3Binding | PurePython,  // FROM CONFIG
}

PythonClass {
    name: String,
    docstring: String,
    bases: Vec<String>,
    methods: Vec<PythonFunction>,
    
    // Link to Rust (if PyO3 binding)
    rust_impl: Option<RustItemRef>,
    source_file: PathBuf,
    source_lines: (usize, usize),
}

PythonFunction {
    name: String,
    docstring: String,
    signature: PythonSig,
    decorators: Vec<String>,
    is_async: bool,
    
    // Parsed docstring sections
    params: Vec<ParamDoc>,      // from Google/NumPy style
    returns: Option<ReturnDoc>,
    raises: Vec<RaisesDoc>,
    examples: Vec<Example>,
    
    // Link to Rust
    rust_impl: Option<RustItemRef>,
    source_file: PathBuf,
    source_lines: (usize, usize),
}
```

### Cross-Reference Graph
```rust
CrossRef {
    python_item: PythonItemRef,
    rust_item: RustItemRef,
    relationship: Binding | Wraps | Delegates,
}
```

## Configuration (`plissken.toml`)

```toml
[project]
name = "cloaca"
version_from = "git"  # or "cargo" or "pyproject"

[output]
format = "markdown"
path = "docs/api"
template = "mkdocs-material"  # or "mdbook", "docusaurus", "custom"

# For "Rust library + Python bindings" pattern
[rust]
crates = ["crates/cloacina", "crates/cloacina-macros"]
entry_point = "cloacina"

[python]
package = "cloaca"
source = "bindings/cloaca/python/cloaca"

# Explicit source type mapping (PyO3 vs pure Python)
[python.modules]
"cloaca" = "pyo3"              # Root module is PyO3 bindings
"cloaca.helpers" = "python"    # Pure Python helpers
"cloaca.types" = "python"      # Pure Python type definitions
"cloaca._internal" = "pyo3"    # More bindings

[links]
# External crate documentation
dependencies = "cargo.lock"     # or "cargo.toml" or "none"
docs_rs_base = "https://docs.rs"

[quality]
require_docstrings = true
min_coverage = 0.8              # 80% of public API documented
fail_on_broken_links = true
```

## Source Type Indicators

Visual badges in rendered output:

| Badge | Meaning | When to use |
|-------|---------|-------------|
| `🐍 Python` | Pure Python source | Helpers, type definitions, wrappers |
| `🦀 Rust` | Pure Rust source | Internal implementation |
| `🐍↔🦀 Binding` | PyO3-bound Rust | `#[pyclass]`, `#[pyfunction]` |

Source code is shown collapsed by default, expandable for those who want to see implementation.

## Dependency Linking

For outlinks to docs.rs:

**Strategy**: Use `Cargo.lock` version when available, fall back to `Cargo.toml` semver.

```
📦 Wraps: cloacina::Pipeline
📄 docs.rs/cloacina/0.3.2  (from Cargo.lock)
```

For dependencies not on docs.rs (git deps, path deps), link to source or omit.

## Technical Approach

| Component | Choice | Rationale |
|-----------|--------|-----------|
| Language | Rust | Fast, single binary, no runtime deps |
| Rust parsing | syn | Full fidelity for PyO3 attribute parsing |
| Python parsing | tree-sitter-python | Fast, no Python runtime needed |
| Templating | Tera | Rust-native, Jinja2-like |
| Output | Markdown + HTML/CSS | Works with any SSG |
| Config | `plissken.toml` | Standard Rust convention |

## Principles

1. **Bridge-first**: The Rust↔Python connection is the core value
2. **Static analysis**: Never require compiling or importing target code
3. **Design matters**: If it's ugly, we failed
4. **Git is the source of truth**: Versions come from tags, not config
5. **Markdown+HTML**: Use HTML/CSS when Markdown isn't enough, but stay SSG-compatible

## Constraints

- Rust implementation (single static binary)
- Primary focus: PyO3/maturin projects
- Output: Markdown with optional HTML components
- Assumes git for version management