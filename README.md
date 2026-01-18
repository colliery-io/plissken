<p align="center">
  <img src="image.png" alt="plissken logo" width="200">
</p>

# plissken

[![CI](https://github.com/colliery-io/plissken/actions/workflows/ci.yml/badge.svg)](https://github.com/colliery-io/plissken/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/plissken.svg)](https://crates.io/crates/plissken)
[![Documentation](https://docs.rs/plissken/badge.svg)](https://docs.rs/plissken)

**Documentation generator for Rust-Python hybrid projects**

plissken generates beautiful, unified documentation for projects that combine Rust and Python code, with special support for [PyO3](https://pyo3.rs/) and [maturin](https://www.maturin.rs/) bindings.

## Features

- **Unified Documentation** - Single documentation site for both Python and Rust APIs
- **Cross-Reference Links** - Automatic bidirectional links between Python classes and their Rust implementations
- **Multiple SSG Support** - Generate docs for MkDocs Material or mdBook
- **Smart Discovery** - Auto-detect Python packages and Rust crates
- **Customizable Themes** - Full theme support with CSS variables for dark mode
- **Template Overrides** - Customize any template without forking

## Installation

### Quick Install (Linux/macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/colliery-io/plissken/main/install.sh | bash
```

### From crates.io

```bash
cargo install plissken
```

### From Source

```bash
git clone https://github.com/colliery-io/plissken.git
cd plissken
cargo install --path crates/plissken-cli
```

## Quick Start

```bash
# Initialize configuration in your project
plissken init

# Generate documentation
plissken render

# Serve with MkDocs
mkdocs serve
```

## Documentation

Full documentation is available at [https://colliery-io.github.io/plissken/](https://colliery-io.github.io/plissken/)

## License

MIT License - see [LICENSE](LICENSE) for details.
