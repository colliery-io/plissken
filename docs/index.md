<p align="center">
  <img src="assets/logo.png" alt="plissken logo" width="200">
</p>

# plissken

**Documentation generator for Rust-Python hybrid projects**

plissken generates beautiful, unified documentation for projects that combine Rust and Python code, with special support for [PyO3](https://pyo3.rs/) and [maturin](https://www.maturin.rs/) bindings.

## Features

- **Unified Documentation** - Single documentation site for both Python and Rust APIs
- **Cross-Reference Links** - Automatic bidirectional links between Python classes and their Rust implementations
- **Multiple SSG Support** - Generate docs for MkDocs Material or mdBook
- **Smart Discovery** - Auto-detect Python packages and Rust crates
- **Customizable Themes** - Full theme support with CSS variables for dark mode
- **Template Overrides** - Customize any template without forking

## Quick Example

```bash
# Initialize configuration
plissken init

# Generate documentation
plissken render

# Serve with MkDocs
mkdocs serve
```

## Installation

=== "Quick Install (Linux/macOS)"

    ```bash
    curl -fsSL https://raw.githubusercontent.com/colliery-io/plissken/main/install.sh | bash
    ```

=== "From crates.io"

    ```bash
    cargo install plissken
    ```

=== "From Source"

    ```bash
    git clone https://github.com/colliery-io/plissken.git
    cd plissken
    cargo install --path crates/plissken-cli
    ```

## Documentation Structure

When you run `plissken render`, it generates a documentation structure like:

```
docs/
  api/
    python/
      mypackage/
        index.md           # Package overview
        MyClass.md         # Class documentation
        my_function.md     # Function documentation
    rust/
      mycrate/
        index.md           # Module overview
        MyStruct.md        # Struct documentation
    _nav.yml               # Navigation for mkdocs.yml
```

## Cross-Reference Magic

For PyO3 bindings, plissken automatically creates bidirectional links:

**In Python docs:**
> **Rust Implementation**: `mycrate::MyStruct` → links to Rust doc page

**In Rust docs:**
> **Python API**: `mypackage.MyClass` → links to Python doc page

## Next Steps

- [Installation](getting-started/installation.md) - Detailed installation instructions
- [Quick Start](getting-started/quickstart.md) - Get your first docs generated
- [Configuration](getting-started/configuration.md) - Customize plissken.toml
- [API Reference](api/index.md) - Full API documentation
