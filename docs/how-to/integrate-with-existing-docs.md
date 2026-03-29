# How To: Integrate with an Existing Doc Site

When plissken generates API documentation, you often need to mount it as a
subsection of a larger documentation site. The `--prefix` flag and
`output.prefix` config make this straightforward.

## The Problem

Without a prefix, plissken generates nav entries relative to the output
directory:

```yaml
# _nav.yml (no prefix)
nav:
  - Rust:
    - mycrate: rust/mycrate.md
```

But if your output lives in `docs/api/` and your MkDocs content root is
`docs/`, the nav entries need the `api/` prefix to resolve correctly:

```yaml
# What mkdocs.yml actually needs
nav:
  - API Reference:
    - Rust:
      - mycrate: api/rust/mycrate.md
```

## Solution: Use prefix

### Option 1: Config-driven (recommended for projects)

Add `prefix` to your `plissken.toml`:

```toml
[output]
path = "docs/api"
template = "mkdocs-material"
prefix = "api"
```

Then render as usual:

```bash
plissken render
```

The generated `_nav.yml` entries will be prefixed with `api/`.

### Option 2: CLI flag (recommended for CI/CD)

Pass `--prefix` on the command line:

```bash
plissken render . -o docs/api --prefix api
```

The CLI flag overrides any `prefix` in the config, making it easy to adapt
the same project config for different deployment targets.

## Using the Generated Nav in mkdocs.yml

The generated `_nav.yml` contains the correctly prefixed navigation structure.
Copy its contents into your `mkdocs.yml` under the appropriate section:

```yaml
# mkdocs.yml
nav:
  - Home: index.md
  - Tutorials:
    - Getting Started: tutorials/quickstart.md
  - API Reference:
    - Rust:
      - mycrate: api/rust/mycrate.md
      - mycrate::config: api/rust/mycrate/config.md
```

## Nested Nav Sections

plissken generates hierarchical navigation — modules with submodules become
collapsible sections. This works with MkDocs Material's `navigation.indexes`
feature:

```yaml
# Generated _nav.yml with prefix = "api"
nav:
  - Rust:
    - mycrate:
      - api/rust/mycrate.md           # Section index page
      - config: api/rust/mycrate/config.md
      - utils: api/rust/mycrate/utils.md
```

Enable `navigation.indexes` in your `mkdocs.yml` theme features for the best
experience:

```yaml
theme:
  features:
    - navigation.indexes
```

## mdBook Integration

The prefix also works with mdBook. SUMMARY.md link targets are prefixed:

```markdown
- [mycrate](api/rust/mycrate.md)
  - [config](api/rust/mycrate/config.md)
```

## CI/CD Pipeline Example

A typical GitHub Actions workflow that renders plissken docs into an existing
site:

```yaml
- name: Build plissken
  run: cargo build --release -p plissken

- name: Generate API docs
  run: ./target/release/plissken render . --prefix api

- name: Build full site
  run: mkdocs build
```

See also: [Integrate with CI/CD](ci-integration.md) for complete workflow
examples.
