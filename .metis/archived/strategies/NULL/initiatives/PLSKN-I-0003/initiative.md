---
id: namespace-pyo3-bindings-with
level: initiative
title: "Namespace PyO3 Bindings with Python Modules"
short_code: "PLSKN-I-0003"
created_at: 2026-01-15T01:51:36.765509+00:00
updated_at: 2026-01-16T02:36:04.685938+00:00
parent: PLSKN-V-0001
blocked_by: []
archived: true

tags:
  - "#initiative"
  - "#phase/completed"


exit_criteria_met: false
estimated_complexity: S
strategy_id: NULL
initiative_id: namespace-pyo3-bindings-with
---

# Namespace PyO3 Bindings with Python Modules Initiative

*This template includes sections for various types of initiatives. Delete sections that don't apply to your specific use case.*

## Context

Documentation should provide two complementary views:

1. **Python Reference** - APIs organized by exported Python namespace

   - Pure Python modules
   - PyO3 bindings rendered as Python-style APIs

2. **Rust Reference** - Code organized by Rust module structure

   - Full Rust implementation
   - Includes binding code with Rust signatures and source

These are not duplicates - they serve different audiences. Python users see the API they consume. Rust users see the implementation they maintain.

## Goals & Non-Goals

**Goals:**

- **Python Reference** in `docs/python/` - organized by Python namespace
  - `python/helpers.md` - Pure Python helpers
  - `python/hybrid_binary.md` - PyO3 bindings as Python APIs (Python-style signatures)
- **Rust Reference** in `docs/rust/` - organized by Rust modules
  - `rust/hybrid_binary.md` - Root module
  - `rust/hybrid_binary/decorators.md` - Binding code with Rust signatures and source
  - `rust/hybrid_binary/internal.md` - Pure Rust internals
- Top-level `index.md` with navigation to both sections
- Bi-directional links between Python and Rust views for bindings

**Non-Goals:**

- Removing content from either view
- Merging the two views into one

## Approach: Bimodal Documentation Output

Generate two parallel reference sections with explicit paths:

### Python Reference (`docs/python/`)

- Pure Python modules rendered from `.py` files
- PyO3 bindings synthesized as Python modules with **Python-style signatures**
- Organized by Python package/module namespace

### Rust Reference (`docs/rust/`)

- Full Rust code rendered from `.rs` files
- **Rust-style signatures** for all items (including bindings)
- Organized by Rust module hierarchy
- Binding code shows `#[pyclass]`, `#[pyfunction]` attributes

### Bindings Appear in Both

- Python docs: `task(runner: Runner, **kwargs) -> TaskDecorator`
- Rust docs: `fn task(runner: Py<PyRunner>, kwargs: Option<...>) -> PyResult<TaskDecorator>`
- Bi-directional links connect the two views

## Detailed Design

### Current State Issue

Earlier in this session, `render_rust_function_with_context()` was modified to show Python-style signatures for bindings (lines 501-507 in module_renderer.rs):

```rust
if is_binding {
    let py_sig = self.rust_to_python_signature(f);
    // renders Python-style signature in Rust docs
}
```

**This needs to be reverted** - Rust docs should always show Rust signatures.

### Changes Required

#### 1. `crates/plissken-cli/src/main.rs`

**Update output paths:**

- Python modules → `{output}/python/{module}.md`
- Rust modules → `{output}/rust/{module}.md`
- Generate `{output}/index.md` with links to both sections

#### 2. `crates/plissken-core/src/render/module_renderer.rs`

**Rust docs use Rust signatures:**

- Revert the `is_binding` conditional that calls `rust_to_python_signature()`
- Rust functions always render with Rust signatures
- Keep the `Binding` badge to indicate PyO3 exposure
- Add link to Python counterpart for bindings

**Python docs use Python signatures:**

- `render_python_function()` - Already renders Python-style (unchanged)
- Synthesized bindings render as Python (unchanged)
- Add link to Rust implementation for bindings

#### 3. Bi-directional cross-links

**In Rust binding docs:**

```markdown
> **Python API**: [hybrid_binary.task](../python/hybrid_binary.md#task)
```

**In Python binding docs:**

```markdown
> **Rust Implementation**: [decorators::task](../rust/hybrid_binary/decorators.md#task)
```

#### 4. Top-level index

Generate `index.md`:

```markdown
# API Reference

## Python Reference
- [helpers](python/helpers.md)
- [hybrid_binary](python/hybrid_binary.md)

## Rust Reference  
- [hybrid_binary](rust/hybrid_binary.md)
```

## Expected Output Structure

```
docs/
  index.md                      # Top-level index with Python/Rust sections
  
  # Python Reference (organized by Python namespace)
  python/
    helpers.md                  # Pure Python - TaskBuilder, run_task()
    hybrid_binary.md            # PyO3 bindings as Python API - Runner, Task, task()
  
  # Rust Reference (organized by Rust module structure)  
  rust/
    hybrid_binary.md            # Root module with Rust signatures
    hybrid_binary/
      decorators.md             # Binding code - TaskDecorator, task() with Rust source
      internal.md               # Pure Rust - Task, TaskExecutor, ExecutorError
```

## Verification

1. Run `cargo build --release`
2. Generate docs: `./target/release/plissken render tests/fixtures/hybrid_binary -o /tmp/test-docs -t mkdocs-material`
3. Verify **directory structure**:
   - `docs/index.md` - Top-level with Python/Rust sections
   - `docs/python/` - Python reference
   - `docs/rust/` - Rust reference
4. Verify **Python Reference** (`docs/python/`):
   - `helpers.md` - Pure Python with Python signatures
   - `hybrid_binary.md` - PyO3 bindings with Python-style signatures
5. Verify **Rust Reference** (`docs/rust/`):
   - `hybrid_binary/decorators.md` - Binding code with Rust signatures
   - `hybrid_binary/internal.md` - Pure Rust internals
6. Verify **bi-directional links**:
   - Rust binding docs link to Python counterpart
   - Python binding docs link to Rust implementation

## Alternatives Considered

1. **Filter bindings from Rust docs** - Would lose implementation visibility for Rust maintainers
2. **Single unified view** - Conflates API usage with implementation details
3. **Separate sites** - Overhead of maintaining two doc builds

## Resolved Questions

1. **Rust docs link to Python counterparts?** → Yes, add links
2. **Top-level index?** → Yes, create `index.md` with Python/Rust sections
3. **Cross-references between views?** → Bi-directional links