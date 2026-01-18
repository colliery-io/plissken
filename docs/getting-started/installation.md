# Installation

## Quick Install (Linux/macOS)

The fastest way to install plissken:

```bash
curl -fsSL https://raw.githubusercontent.com/colliery-io/plissken/main/install.sh | bash
```

This downloads the latest release binary and installs it to `~/.local/bin`.

## From crates.io

If you have Rust installed, you can install via Cargo:

```bash
cargo install plissken
```

## From Source

Clone the repository and build with Cargo:

```bash
git clone https://github.com/colliery-io/plissken.git
cd plissken
cargo build --release
```

The binary will be at `target/release/plissken`.

### Add to PATH

```bash
# Linux/macOS
cp target/release/plissken ~/.local/bin/

# Or install via cargo
cargo install --path crates/plissken-cli
```

## Verify Installation

```bash
plissken --help
```

## MkDocs Material (Recommended)

For the best documentation experience, install MkDocs with the Material theme:

```bash
pip install mkdocs-material mkdocs-autorefs
```

## mdBook (Alternative)

For Rust-style documentation, install mdBook:

```bash
cargo install mdbook
```
