# Quick Start

Get your documentation up and running in 5 minutes.

## 1. Initialize Configuration

Navigate to your project root and run:

```bash
plissken init
```

This creates a `plissken.toml` with auto-detected settings:

```toml
[project]
name = "myproject"
version_from = "cargo"  # or "pyproject"

[output]
format = "markdown"
path = "docs/api"
template = "mkdocs-material"

[rust]
crates = ["."]

[python]
package = "mypackage"
```

## 2. Generate Documentation

```bash
plissken render
```

This parses your source code and generates Markdown files:

```
docs/api/
  python/mypackage/...
  rust/mycrate/...
  _nav.yml
```

## 3. Set Up MkDocs

Create `mkdocs.yml`:

```yaml
site_name: My Project
theme:
  name: material

nav:
  - Home: index.md
  - API Reference:
    - api/index.md
    # Copy entries from docs/api/_nav.yml here
```

Copy the navigation entries from the generated `docs/api/_nav.yml` into your mkdocs.yml.

## 4. Serve Locally

```bash
mkdocs serve
```

Visit [http://localhost:8000](http://localhost:8000) to see your docs!

## 5. Build for Production

```bash
mkdocs build
```

The static site is in `site/`, ready for deployment.

## Next Steps

- [Configuration](configuration.md) - Customize your setup
- [Python Projects](../guide/python.md) - Python-specific features
- [Rust Projects](../guide/rust.md) - Rust-specific features
- [Hybrid Projects](../guide/hybrid.md) - PyO3/maturin bindings
