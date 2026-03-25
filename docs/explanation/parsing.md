# How Parsing Works

plissken uses different parsing strategies for Rust and Python, each
optimized for the language's tooling ecosystem.

## Rust Parsing

Rust files are parsed using the [`syn`](https://docs.rs/syn) crate, which
provides a complete abstract syntax tree (AST). This is the same parser
used by procedural macros in the Rust ecosystem.

### Why syn?

`syn` is the standard Rust AST parser — the same library used by procedural
macros. Using it means plissken parses Rust code with the same fidelity as
the Rust compiler itself. This was a deliberate choice over simpler
approaches like regex or tree-sitter-rust because:

- It handles all Rust syntax correctly, including complex generics,
  lifetimes, and where clauses
- It provides typed access to attributes like `#[pyclass(name = "...")]`,
  which is essential for cross-reference extraction
- It's battle-tested across the entire Rust ecosystem

The parser extracts all public item types (structs, enums, functions,
traits, impl blocks, consts, type aliases). For the full list of extracted
fields, see the [Data Model Reference](../reference/data-model.md).

### Doc Comment Design

Doc comments (`///` and `//!`) are parsed into structured `ParsedDocstring`
objects. The parser maps Rust's conventional `# Section` headers to
structured fields — for example, `# Arguments` becomes a params list and
`# Errors` becomes a raises list. This means Rust doc comments and Python
docstrings produce the same structured output, enabling consistent rendering
regardless of the source language.

The `# Panics` section is treated as a variant of `# Errors` (with type
"panic") because both represent failure modes that users need to know about.
`# Safety` is folded into the description rather than getting its own field,
since it's prose rather than structured data.

### Generic Parameters

Generics are preserved as strings, including:

- Type parameters: `<T: Clone>`
- Lifetime parameters: `<'a>`
- Const generics: `<const N: usize>`
- Complex bounds: `<T: Iterator<Item = &'a str> + Send>`

### PyO3 Detection

The Rust parser doesn't just extract documentation — it also recognizes
PyO3 attributes (`#[pyclass]`, `#[pyfunction]`, `#[pymethods]`) and
captures their arguments (like `name` and `module`). This metadata is what
enables the cross-reference system to link Python items back to their Rust
implementations. The detection happens at parse time rather than as a
separate pass, because `syn` makes attribute inspection trivial when you
already have the full AST.

### Module Path Construction

File paths are converted to Rust module paths:

| File Path | Module Path |
|-----------|-------------|
| `src/lib.rs` | `mycrate` |
| `src/utils.rs` | `mycrate::utils` |
| `src/net/mod.rs` | `mycrate::net` |
| `src/net/tcp.rs` | `mycrate::net::tcp` |

The crate name is read from the `[package].name` field in the crate's
`Cargo.toml`.

## Python Parsing

Python files are parsed using
[`tree-sitter`](https://tree-sitter.github.io/tree-sitter/) with the
`tree-sitter-python` grammar. tree-sitter produces a concrete syntax tree
(CST) that preserves all syntactic detail.

### Why tree-sitter?

Python can't be parsed with `syn` (obviously), so plissken needs a
different parser. tree-sitter was chosen over Python's built-in `ast`
module for a key reason: plissken is a Rust binary and doesn't require a
Python runtime to be installed. Using Python's `ast` module would mean
either embedding a Python interpreter or shelling out to a Python process,
both of which add complexity and a runtime dependency.

tree-sitter provides a fast, dependency-free parser implemented in C with
Rust bindings. It produces a concrete syntax tree (CST) that preserves
enough syntactic detail to extract classes, functions, decorators, type
annotations, and docstrings. The trade-off is that tree-sitter doesn't
do semantic analysis (like resolving imports), but plissken doesn't need
that — it only needs syntactic structure.

### Docstring Format Detection

plissken supports two docstring formats:

**Google style** (detected by `Args:`, `Returns:`, `Raises:`, `Example:` markers):

```python
def func(x):
    """Summary line.

    Extended description.

    Args:
        x: Description of x.

    Returns:
        Description of return value.

    Raises:
        ValueError: When x is invalid.

    Examples:
        >>> func(1)
        2
    """
```

**NumPy style** (detected by underlined section headers):

```python
def func(x):
    """
    Summary line.

    Parameters
    ----------
    x : int
        Description of x.

    Returns
    -------
    int
        Description of return value.
    """
```

If neither format is detected, the entire docstring is treated as the
summary/description with no structured sections.

### Type Information Strategy

Types come from two sources: function signature annotations and docstring
type annotations. Signature annotations take precedence because they're
machine-checked (by mypy, pyright, etc.) and therefore more reliable.
Docstring types are a fallback for legacy code that predates PEP 484.

This dual-source approach means plissken produces useful type information
for both modern annotated code and older codebases that only have types
in docstrings.

### Module Path Construction

File paths are converted to dotted Python module paths:

| File Path | Module Path |
|-----------|-------------|
| `mypackage/__init__.py` | `mypackage` |
| `mypackage/utils.py` | `mypackage.utils` |
| `mypackage/net/__init__.py` | `mypackage.net` |
| `mypackage/net/tcp.py` | `mypackage.net.tcp` |

## Error Handling

Both parsers are designed to be resilient:

- **Non-fatal errors**: If a file fails to parse, plissken emits a
  structured warning and continues with the remaining files. This means
  a single syntax error doesn't block the entire documentation build.

- **Partial extraction**: Even within a successfully parsed file, if a
  specific item can't be fully extracted (e.g., a complex generic bound),
  the item is included with whatever information was captured.

- **Warning format**: Parse warnings follow a consistent format:
  ```
  warning: failed to parse Rust file
    --> src/broken.rs
    unexpected token at line 42
  ```

## Performance

- **Rust parsing** is single-threaded and processes files sequentially.
  `syn` is fast enough that even large crates (thousands of lines) parse
  in milliseconds.

- **Python parsing** uses tree-sitter's incremental parser, though
  plissken currently parses each file independently. tree-sitter is
  implemented in C with Rust bindings, making it very fast.

- **File discovery** uses `walkdir` for efficient directory traversal with
  configurable skip lists to avoid scanning virtual environments, caches,
  and build artifacts.
