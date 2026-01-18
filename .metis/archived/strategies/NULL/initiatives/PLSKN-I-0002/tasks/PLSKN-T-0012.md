---
id: cli-render-command
level: task
title: "CLI Render Command"
short_code: "PLSKN-T-0012"
created_at: 2026-01-14T16:56:04.186517+00:00
updated_at: 2026-01-14T20:56:13.433297+00:00
parent: PLSKN-I-0002
blocked_by: []
archived: true

tags:
  - "#task"
  - "#phase/completed"


exit_criteria_met: false
strategy_id: NULL
initiative_id: PLSKN-I-0002
---

# CLI Render Command

*This template includes sections for various types of tasks. Delete sections that don't apply to your specific use case.*

## Parent Initiative **[CONDITIONAL: Assigned Task]**

[[PLSKN-I-0002]]

## Objective

Add `plissken render` CLI command that generates documentation from DocModel to the configured output path.

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [x] `plissken render <path>` command reads config and renders docs
- [x] Uses `output.path` from config for destination directory
- [x] Uses `output.template` from config to select ThemeAdapter
- [x] Creates output directory structure if it doesn't exist
- [x] Reports files written to stderr
- [x] Integration test: render hybrid_binary fixture, verify output files exist

## Implementation Notes

### Technical Approach
- Added `Render` subcommand to CLI with `-o/--output` and `-t/--template` options
- Uses `ModuleRenderer` from plissken-core to render Python and Rust modules
- Module paths normalized from file paths to logical paths (e.g., `/path/to/project/src/lib.rs` → `crate_name`)
- Creates nested directory structure as needed for output files
- Reports progress to stderr, keeps stdout clean for potential piping

### Files Created/Modified
- `crates/plissken-cli/src/main.rs` - Added `render()` function and path normalization helpers
- `crates/plissken-cli/Cargo.toml` - Added `tempfile` dev dependency for tests
- `crates/plissken-cli/tests/integration_test.rs` - 4 integration tests

## Status Updates **[REQUIRED]**

### 2026-01-14: Completed

Implemented `plissken render <path>` CLI command with full functionality:

**CLI Options:**
```
plissken render [OPTIONS] [PATH]

Arguments:
  [PATH]  Path to plissken.toml or project root [default: .]

Options:
  -o, --output <OUTPUT>      Output directory (overrides config)
  -t, --template <TEMPLATE>  Theme template (e.g., "mkdocs-material", "mdbook")
```

**Key Features:**
- Reads config from `plissken.toml`
- Uses `output.path` from config (default: `docs/api`)
- Uses `output.template` from config for theme selection
- `-o` flag overrides output directory
- `-t` flag overrides template
- Creates output directory structure automatically
- Reports each file written to stderr
- Normalizes file paths to logical module paths for clean output structure

**Path Normalization:**
- Python: `/path/project/src/module.py` → `module.md`
- Rust: `/path/project/src/lib.rs` → `rust/crate_name.md`
- Handles `__init__.py`, `lib.rs`, `mod.rs` specially

**Integration Tests (4 tests):**
1. `test_render_hybrid_binary_fixture` - Full render test with content verification
2. `test_render_with_template_override` - Template override flag works
3. `test_generate_json_output` - Generate command still works
4. `test_render_creates_directory_structure` - Creates nested dirs

**Example Output:**
```
$ plissken render tests/fixtures/hybrid_binary -o /tmp/output
Loading config from: .../plissken.toml
Output directory: /tmp/output
Using template: mkdocs-material
Parsed 3 Rust module(s)
Parsed 1 Python module(s)
Built 0 cross-reference(s)
  Wrote: /tmp/output/helpers.md
  Wrote: /tmp/output/rust/hybrid_binary.md
  Wrote: /tmp/output/rust/hybrid_binary/internal.md
  Wrote: /tmp/output/rust/hybrid_binary/decorators.md

Rendered 4 file(s) to /tmp/output
```

**Test Results:** 140 tests passing (128 unit + 4 integration + 8 doctests)