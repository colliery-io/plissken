---
id: define-constants-for-magic-strings
level: task
title: "Define constants for magic strings"
short_code: "PLSKN-T-0047"
created_at: 2026-01-17T19:48:45.531700+00:00
updated_at: 2026-01-17T20:19:51.234850+00:00
parent: PLSKN-I-0011
blocked_by: []
archived: false

tags:
  - "#task"
  - "#phase/active"


exit_criteria_met: false
strategy_id: NULL
initiative_id: PLSKN-I-0011
---

# Define constants for magic strings

## Parent Initiative

[[PLSKN-I-0011]]

## Objective

Replace hardcoded magic strings throughout the codebase with named constants for better maintainability.

## Problem

14 instances of magic strings found across `config.rs` and `main.rs`:

### In config.rs
```rust
fn default_format() -> String { "markdown".to_string() }
fn default_output_path() -> PathBuf { PathBuf::from("docs/api") }
fn default_docs_rs() -> String { "https://docs.rs".to_string() }
```

### In main.rs (config generation)
```rust
config.push_str("template = \"mkdocs-material\"\n");
config.push_str("version_from = \"cargo\"\n");
config.push_str("version_from = \"pyproject\"\n");
config.push_str("path = \"docs/api\"\n");
config.push_str("format = \"markdown\"\n");
config.push_str("crates = [\".\"]\n");
```

### In error messages
```rust
PlisskenError::VersionSourceNotFound("cargo".to_string(), "Cargo.toml".to_string());
PlisskenError::VersionSourceNotFound("pyproject".to_string(), "pyproject.toml".to_string());
```

## Acceptance Criteria

## Acceptance Criteria

- [x] Create constants module or section in config.rs
- [x] Define all magic strings as named constants
- [x] Replace all instances with constant references
- [x] No change in runtime behavior
- [x] All existing tests pass

## Implementation Notes

### Technical Approach

Add to `config.rs` or create `constants.rs`:

```rust
// Default values
pub const DEFAULT_OUTPUT_FORMAT: &str = "markdown";
pub const DEFAULT_OUTPUT_PATH: &str = "docs/api";
pub const DEFAULT_TEMPLATE: &str = "mkdocs-material";
pub const DEFAULT_DOCS_RS_URL: &str = "https://docs.rs";

// Version sources
pub const VERSION_SOURCE_CARGO: &str = "cargo";
pub const VERSION_SOURCE_PYPROJECT: &str = "pyproject";

// Manifest files
pub const CARGO_MANIFEST: &str = "Cargo.toml";
pub const PYPROJECT_MANIFEST: &str = "pyproject.toml";
pub const PLISSKEN_CONFIG: &str = "plissken.toml";

// Template names
pub const TEMPLATE_MKDOCS_MATERIAL: &str = "mkdocs-material";
pub const TEMPLATE_MDBOOK: &str = "mdbook";
```

Then update usages:
```rust
fn default_format() -> String { DEFAULT_OUTPUT_FORMAT.to_string() }
fn default_output_path() -> PathBuf { PathBuf::from(DEFAULT_OUTPUT_PATH) }
```

## Status Updates

### Session 1 - Completed

**Created 12 constants in `config.rs`:**

| Constant | Value | Purpose |
|----------|-------|---------|
| `DEFAULT_OUTPUT_FORMAT` | `"markdown"` | Default documentation format |
| `DEFAULT_OUTPUT_PATH` | `"docs/api"` | Default output directory |
| `DEFAULT_DOCS_RS_URL` | `"https://docs.rs"` | External docs link base |
| `DEFAULT_TEMPLATE` | `"mkdocs-material"` | Default SSG template |
| `VERSION_SOURCE_CARGO` | `"cargo"` | Version source identifier |
| `VERSION_SOURCE_PYPROJECT` | `"pyproject"` | Version source identifier |
| `CARGO_MANIFEST` | `"Cargo.toml"` | Cargo manifest filename |
| `PYPROJECT_MANIFEST` | `"pyproject.toml"` | Python manifest filename |
| `PLISSKEN_CONFIG` | `"plissken.toml"` | Config filename |
| `TEMPLATE_MKDOCS_MATERIAL` | `"mkdocs-material"` | MkDocs template name |
| `TEMPLATE_MDBOOK` | `"mdbook"` | mdBook template name |
| `DEFAULT_CRATES` | `"."` | Default crates value |

**Updated files:**

1. **`crates/plissken-core/src/config.rs`**:
   - Added constants section at top of file
   - Updated `default_format()`, `default_output_path()`, `default_docs_rs()` to use constants
   - Updated `validate_version_source()` to use `CARGO_MANIFEST`, `PYPROJECT_MANIFEST`, `VERSION_SOURCE_CARGO`, `VERSION_SOURCE_PYPROJECT`
   - Updated `validate_rust_config()` to use `CARGO_MANIFEST`, `DEFAULT_CRATES`

2. **`crates/plissken-core/src/lib.rs`**:
   - Exported all constants from `config` module

3. **`crates/plissken-cli/src/main.rs`**:
   - Imported constants from plissken_core
   - Updated 20+ usages across: `init()`, `generate_config()`, `find_config()`, `check()`, `detect_project()`, `read_crate_name()`, `get_project_version()`, `resolve_content_directory()`, `generate_ssg_files()`, `create_renderer()`

**Test Results:** 229 unit tests, 4 integration tests, 16 doc tests - all pass

**Benefits:**
- Single source of truth for all magic strings
- Easy to change values without hunting through codebase
- Better IDE support (autocomplete, find references)
- Clearer intent in code