# How To: Install plissken

## Quick Install (Linux/macOS)

Download and install the latest release binary:

```bash
curl -fsSL https://raw.githubusercontent.com/colliery-io/plissken/main/install.sh | bash
```

This installs to `~/.local/bin` by default. To change the install location:

```bash
PLISSKEN_INSTALL_DIR=/usr/local/bin curl -fsSL https://raw.githubusercontent.com/colliery-io/plissken/main/install.sh | bash
```

The installer detects your OS and architecture automatically. Supported
platforms:

| OS | Architectures |
|----|---------------|
| Linux | x86_64, aarch64 |
| macOS | x86_64 (Intel), aarch64 (Apple Silicon) |
| Windows | x86_64 (via MSYS2/Cygwin) |

## From crates.io

If you have Rust installed:

```bash
cargo install plissken
```

This builds from source and installs the binary to `~/.cargo/bin`.

## From Source

```bash
git clone https://github.com/colliery-io/plissken.git
cd plissken
cargo install --path crates/plissken-cli
```

Or build without installing:

```bash
cargo build --release
# Binary at: target/release/plissken
```

## Verify Installation

```bash
plissken --help
```

Expected output:

```
Documentation generator for Rust-Python hybrid projects

Usage: plissken [OPTIONS] <COMMAND>

Commands:
  generate  Generate documentation model as JSON
  render    Render documentation to Markdown files
  init      Initialize a new plissken.toml configuration file
  check     Validate configuration without generating documentation
  help      Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...  Increase output verbosity (-v for progress, -vv for debug)
  -h, --help        Print help
```

## Install a Documentation Server

plissken generates Markdown files. To browse them, you need a static site
generator.

### MkDocs Material (Recommended)

```bash
pip install mkdocs-material
```

### mdBook (Alternative)

```bash
cargo install mdbook
```

## PATH Troubleshooting

If `plissken` is not found after installation, ensure the install directory
is in your PATH:

**bash:**
```bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

**zsh:**
```bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

**fish:**
```bash
fish_add_path ~/.local/bin
```
