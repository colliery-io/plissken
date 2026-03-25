# CLI Reference

## Global Options

| Flag | Description |
|------|-------------|
| `-v`, `--verbose` | Increase verbosity. Use `-v` for progress output, `-vv` for debug output. |
| `-h`, `--help` | Print help information. |

Verbosity is global and applies to all subcommands.

---

## `plissken init`

Initialize a new `plissken.toml` configuration file in the current directory.

```
plissken init [OPTIONS]
```

### Options

| Flag | Description |
|------|-------------|
| `--force` | Overwrite an existing `plissken.toml`. Without this flag, init refuses if the file already exists. |

### Behavior

1. Scans the current directory for `Cargo.toml` and `pyproject.toml`.
2. Detects project type (Rust, Python, or hybrid).
3. For Cargo workspaces, reads `[workspace].members` to populate `rust.crates`.
4. For pyproject.toml, reads `[project].name` and `[tool.maturin].python-source`.
5. Writes `plissken.toml` with inferred values.

### Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Configuration file created successfully. |
| 1 | Error (file already exists without `--force`, or detection failed). |

---

## `plissken generate`

Generate the documentation model as JSON.

```
plissken generate [OPTIONS] [PATH]
```

### Arguments

| Argument | Default | Description |
|----------|---------|-------------|
| `PATH` | `.` | Path to `plissken.toml` or the project root directory containing it. |

### Options

| Flag | Description |
|------|-------------|
| `-o`, `--output <FILE>` | Write JSON to a file instead of stdout. |
| `--pretty` | Pretty-print the JSON output with indentation. |

### Behavior

1. Loads and validates `plissken.toml`.
2. Parses all configured Rust and Python source files.
3. Builds cross-references between Python and Rust items.
4. Serializes the complete `DocModel` to JSON.
5. Writes to stdout or the specified output file.

Parser errors are non-fatal — files that fail to parse are skipped with a
warning, and the remaining files are still included in the output.

### Exit Codes

| Code | Meaning |
|------|---------|
| 0 | JSON model generated successfully (even if some files had parse warnings). |
| 1 | Fatal error (config not found, config invalid, output write failed). |

---

## `plissken render`

Render documentation to Markdown files.

```
plissken render [OPTIONS] [PATH]
```

### Arguments

| Argument | Default | Description |
|----------|---------|-------------|
| `PATH` | `.` | Path to `plissken.toml` or the project root directory. |

### Options

| Flag | Description |
|------|-------------|
| `-o`, `--output <DIR>` | Override the output directory from config. |
| `-t`, `--template <NAME>` | Override the template/theme from config. Values: `mkdocs-material`, `mdbook`. |

### Behavior

1. Loads and validates `plissken.toml`.
2. Parses all configured source files.
3. Builds cross-references.
4. Creates the output directory structure.
5. Renders each module to a Markdown file using the configured template.
6. Generates SSG-specific files:
   - **MkDocs**: `_nav.yml` navigation file
   - **mdBook**: `SUMMARY.md` navigation, `book.toml` config, `theme/custom.css`

### Output Structure

For MkDocs (`template = "mkdocs-material"`):

```
{output.path}/
  {module}.md                    # Python module pages
  {module}/{submodule}.md        # Nested Python modules
  rust/{crate}.md                # Rust crate pages
  rust/{crate}/{module}.md       # Rust submodules
  _nav.yml                       # MkDocs navigation
```

For mdBook (`template = "mdbook"`):

```
{output.path}/
  {module}.md
  rust/{crate}.md
  SUMMARY.md                     # mdBook navigation
  book.toml                      # mdBook config (if not present)
  theme/custom.css               # Custom CSS
```

### Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Documentation rendered successfully. |
| 1 | Fatal error. |

---

## `plissken check`

Validate configuration without generating documentation.

```
plissken check [OPTIONS] [PATH]
```

### Arguments

| Argument | Default | Description |
|----------|---------|-------------|
| `PATH` | `.` | Path to `plissken.toml` or the project root directory. |

### Options

| Flag | Description |
|------|-------------|
| `--format <FORMAT>` | Output format: `text` (default) or `json`. |

### Validation Checks

1. **TOML syntax** — File must be valid TOML.
2. **Language requirement** — At least one of `[rust]` or `[python]` must be present.
3. **Version source** — The file referenced by `version_from` must exist:
   - `"cargo"` requires `Cargo.toml`
   - `"pyproject"` requires `pyproject.toml`
   - `"git"` requires a git repository
4. **Rust paths** — Each path in `rust.crates` must exist.
5. **Python paths** — The Python source directory must exist.

### JSON Output Format

```json
{
  "valid": true,
  "config_path": "/absolute/path/to/plissken.toml",
  "issues": [
    {
      "severity": "warning",
      "message": "description of the issue",
      "hint": "suggested fix or null"
    }
  ]
}
```

### Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Configuration is valid (warnings may be present). |
| 1 | Configuration has errors. |
