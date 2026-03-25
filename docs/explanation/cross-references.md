# How Cross-References Work

Cross-references are the feature that makes plissken uniquely useful for
PyO3/maturin hybrid projects. They create bidirectional links between
Python items and their Rust implementations. This page explains the
algorithm in detail.

## The Problem

In a PyO3 project, a single concept often exists in two places:

- A Rust struct `MyStruct` annotated with `#[pyclass]`
- A Python class `MyClass` that users import and use

Standard documentation tools document each side independently. Users
reading the Python docs have no easy way to find the Rust implementation,
and vice versa. plissken solves this by detecting these relationships
automatically.

## The Algorithm

### Step 1: Index Rust Items

plissken scans all parsed Rust modules and builds three lookup tables:

**pyclass_map**: Maps Python-visible class names to Rust structs.

```
"MyClass" → ("MyStruct", "mycrate::module")
```

The Python-visible name comes from the `#[pyclass(name = "MyClass")]`
attribute. If no `name` argument is provided, the Rust struct name is used
directly.

**pyfunction_map**: Maps Python-visible function names to Rust functions.

```
"process" → ("process_data", "mycrate::utils")
```

Similarly, `#[pyfunction(name = "process")]` overrides the name. Without
it, the Rust function name is used.

**pymethod_map**: Maps (struct name, method name) pairs from `#[pymethods]`
blocks.

```
("MyStruct", "compute") → "compute_inner"
```

### Step 2: Match Python Items

For each Python module that is marked as `pyo3` in the configuration (via
`[python.modules]`), plissken walks the Python items and tries to match
them against the indexed Rust items:

1. **Classes**: Look up the class name in `pyclass_map`. If found, attach
   a `RustItemRef` to the Python class and create a `CrossRef` record.

2. **Functions**: Look up the function name in `pyfunction_map`. If found,
   attach a `RustItemRef` and create a `CrossRef`.

3. **Methods**: For each method in a matched class, look up the
   (struct, method) pair in `pymethod_map`.

### Step 3: Produce Cross-References

Each match produces a `CrossRef` record:

```json
{
  "python_path": "mypackage.MyClass",
  "rust_path": "mycrate::module::MyStruct",
  "relationship": "Binding"
}
```

### Step 4: Render Links

During rendering, cross-references produce bidirectional links:

**On the Python page for `mypackage.MyClass`:**
> Rust Implementation: `mycrate::module::MyStruct` [link]

**On the Rust page for `mycrate::module::MyStruct`:**
> Python API: `mypackage.MyClass` [link]

## Relationship Types

| Type | Meaning | How It's Detected |
|------|---------|-------------------|
| `Binding` | Direct PyO3 binding. The Python item IS the Rust item, exposed to Python. | `#[pyclass]`, `#[pyfunction]` on the Rust item. |
| `Wraps` | The Python class wraps a Rust type (composition). | Manual annotation or convention-based detection. |
| `Delegates` | The Python function calls through to a Rust function. | Manual annotation or convention-based detection. |

Currently, automatic detection produces `Binding` relationships. `Wraps`
and `Delegates` are supported in the data model for future use or manual
annotation.

## Synthesized Modules

When a Python module is marked as `pyo3` in the config but has no
corresponding Python source file, plissken can synthesize Python
documentation directly from the Rust code:

1. Finds all `#[pyclass]` structs in the entry point crate.
2. Creates `PythonClass` records with documentation from the Rust doc
   comments.
3. Converts `#[pymethods]` to `PythonFunction` records.
4. Converts `#[pyfunction]` items to module-level functions.

This means you can generate Python API documentation even before writing
any Python code — the Rust definitions alone are sufficient.

## Module Source Type Resolution

The `[python.modules]` config section determines how each module is
treated:

```toml
[python.modules]
"mypackage" = "pyo3"        # Cross-reference enabled
"mypackage.helpers" = "python"  # No cross-references
```

If `auto_discover` is enabled, plissken also heuristically detects PyO3
modules by scanning the first 2KB of each Python file for:

- Comment markers: `# pyo3` or `#pyo3`
- Native imports: `from ._native import` or `from _native import`
- Type ignore comments: `# type: ignore[import]`

Explicit module mappings always override auto-detection.

## Merging Strategy

When a Python module has both Python source code AND Rust PyO3 bindings
(the common case), plissken merges them:

1. Parse the Python source file to get the "real" Python items.
2. Parse the Rust source to get the PyO3 bindings.
3. For items that appear in both, prefer the Python source (it may have
   richer type annotations or docstrings).
4. For items that appear only in Rust (e.g., methods added in `#[pymethods]`
   that aren't in the Python source), synthesize Python items from the
   Rust definitions.
5. Attach `RustItemRef` cross-references to all matched items.

This gives the best of both worlds: Python-native documentation enriched
with Rust implementation links.
