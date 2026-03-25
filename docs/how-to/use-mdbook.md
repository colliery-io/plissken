# How To: Use mdBook Instead of MkDocs

## Configuration

Set the template to `mdbook` in `plissken.toml`:

```toml
[output]
format = "markdown"
path = "src"
template = "mdbook"
```

Note the `path` is set to `"src"` — mdBook expects content in a `src/`
directory.

## Generate Documentation

```bash
plissken render
```

This generates:

```
src/
  rust/
    mycrate.md
    mycrate/
      module.md
  SUMMARY.md
```

The `SUMMARY.md` file contains mdBook-compatible navigation in Markdown
format.

## Set Up mdBook

Install mdBook:

```bash
cargo install mdbook
```

plissken generates a `book.toml` if one doesn't exist. If you already
have one, the generated `SUMMARY.md` integrates with your existing
configuration.

## Serve

```bash
mdbook serve
```

Open [http://localhost:3000](http://localhost:3000).

## Theme Differences

The mdBook template uses mdBook's CSS variables instead of MkDocs Material's:

| Semantic Color | mdBook Variable |
|----------------|-----------------|
| Primary | `var(--links)` |
| Code background | `var(--code-bg)` |
| Code text | `var(--inline-code-color)` |
| Border | `var(--quote-border)` |

This means badges and code blocks automatically match your mdBook theme,
including third-party themes and custom CSS.

## Custom CSS

mdBook generates a `theme/custom.css` file for plissken-specific styles.
You can override it by placing your own CSS at `theme/custom.css` before
running `plissken render`.

## Template Name Variants

These are all equivalent:

```toml
template = "mdbook"
template = "md-book"
template = "md_book"
```
