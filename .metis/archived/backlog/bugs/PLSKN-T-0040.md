---
id: rust-module-paths-use-filesystem
level: task
title: "Rust module paths use filesystem paths instead of logical crate paths"
short_code: "PLSKN-T-0040"
created_at: 2026-01-17T18:04:41.730513+00:00
updated_at: 2026-01-17T18:52:53.574637+00:00
parent: 
blocked_by: []
archived: true

tags:
  - "#task"
  - "#bug"
  - "#phase/completed"


exit_criteria_met: false
strategy_id: NULL
initiative_id: NULL
---

# Rust module paths use filesystem paths instead of logical crate paths



## Objective

Fix the Rust parser to compute logical module paths (e.g., `plissken_core::discover`) instead of raw filesystem paths (e.g., `/Users/.../crates/plissken-core/src/discover.rs`), enabling proper documentation structure where `lib.rs` becomes the crate index.

## Problem Description

### Current Behavior
The Rust parser stores the raw filesystem path in `RustModule.path`:
```rust
// crates/plissken-core/src/parser/rust.rs:59
Ok(RustModule {
    path: path.display().to_string(),  // <- full filesystem path
    ...
})
```

This results in documentation output like:
```
docs/api/rust/plissken/crates/plissken-core/src.md       <- lib.rs content
docs/api/rust/plissken/crates/plissken-core/src/discover.md
docs/api/rust/plissken/crates/plissken-core/src/config.md
```

### Expected Behavior
Module paths should be logical Rust paths:
- `lib.rs` / `main.rs` → crate name (e.g., `plissken_core`)
- `src/foo.rs` → `plissken_core::foo`
- `src/foo/mod.rs` → `plissken_core::foo`
- `src/foo/bar.rs` → `plissken_core::foo::bar`

Documentation output should be:
```
docs/api/rust/plissken-core/index.md       <- lib.rs content (crate entry)
docs/api/rust/plissken-core/discover.md    <- sibling modules
docs/api/rust/plissken-core/config.md
```

### Priority
- [x] P1 - High (important for user experience)

### Impact Assessment
- **Affected Users**: All users generating Rust documentation
- **Reproduction Steps**: 
  1. Run `plissken render` on any Rust crate
  2. Check output directory structure
  3. Note that `lib.rs` becomes `src.md` instead of `index.md`
- **Expected vs Actual**: `lib.rs` should be crate index, not a `src.md` file alongside `src/` directory

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [x] CLI reads crate name from Cargo.toml and passes to parser
- [x] Parser computes logical module paths from file paths relative to crate root
- [x] `lib.rs` content becomes `{crate_name}.md` for the crate (inline format)
- [x] Submodules become sibling `.md` files (not nested under `src/`)
- [x] Layout module correctly handles crate root vs submodule paths
- [x] Existing tests pass
- [x] Documentation renders correctly with new path structure

## Implementation Notes

### Technical Approach

1. **CLI Changes** (`crates/plissken-cli/src/main.rs`):
   - Read crate name from `Cargo.toml` in each crate directory
   - Pass crate name and crate root path to parser

2. **Parser Changes** (`crates/plissken-core/src/parser/rust.rs`):
   - Add method to compute logical module path from file path
   - Handle special cases: `lib.rs`/`main.rs` → crate root, `mod.rs` → parent dir name
   - Transform: `{crate_root}/src/foo/bar.rs` → `{crate_name}::foo::bar`

3. **Layout Changes** (`crates/plissken-core/src/render/module/layout.rs`):
   - Update `rust_module_page()` to output `index.md` for crate root modules
   - Submodules become siblings: `{crate_name}/foo.md` not `{crate_name}/src/foo.md`

### Files to Modify
- `crates/plissken-cli/src/main.rs` - Add Cargo.toml parsing
- `crates/plissken-core/src/parser/rust.rs` - Module path computation
- `crates/plissken-core/src/render/module/layout.rs` - Output path generation

### Risk Considerations
- Breaking change to output structure - users may have existing nav configs pointing to old paths
- Need to ensure cross-references still resolve correctly with new paths

## Status Updates

### Session 2026-01-17

**Implementation Completed:**

1. **CLI Changes** (`crates/plissken-cli/src/main.rs`):
   - Added `read_crate_name()` function to parse crate name from Cargo.toml
   - Added helpful error when users point to workspace manifests instead of individual crates
   - Added `file_to_module_path()` function to convert file paths to logical module paths
   - Updated `parse_rust_sources()` to compute logical module paths directly
   - Updated `find_rust_files()` to detect both `src/` and `rust/` source directories
   - Removed unused `normalize_rust_module_path()` function

2. **Layout Changes** (`crates/plissken-core/src/render/module/layout.rs`):
   - Updated `rust_module_page()` to output `{crate_name}.md` for crate roots
   - Submodules output as `{crate_name}/{submodule}.md`

3. **Test Updates**:
   - Updated integration tests to expect inline format output structure
   - Updated all unit tests in `module_renderer.rs` to match inline format (single page per module)

**Module Path Logic:**
- `lib.rs` / `main.rs` → crate name (e.g., `hybrid_binary`)
- `src/config.rs` → `{crate_name}::config` (e.g., `hybrid_binary::config`)
- `src/foo/mod.rs` → `{crate_name}::foo`
- Supports both `src/` and `rust/` source directories

**Test Results:**
- All 223 unit tests pass
- All 4 integration tests pass
- All 16 doctests pass

**Output Structure Example:**
```
docs/
  helpers.md                      <- Python module
  rust/
    hybrid_binary.md              <- Rust crate root (lib.rs content)
    hybrid_binary/
      internal.md                 <- Submodule
      config.md                   <- Submodule
```