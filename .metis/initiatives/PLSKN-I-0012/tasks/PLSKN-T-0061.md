---
id: thread-prefix-through-ssg-nav
level: task
title: "Thread prefix through SSG nav generation"
short_code: "PLSKN-T-0061"
created_at: 2026-03-29T13:38:57.112540+00:00
updated_at: 2026-03-29T13:56:23.671276+00:00
parent: PLSKN-I-0012
blocked_by: []
archived: false

tags:
  - "#task"
  - "#phase/completed"


exit_criteria_met: false
initiative_id: PLSKN-I-0012
---

# Thread prefix through SSG nav generation

## Parent Initiative

[[PLSKN-I-0012]]

## Objective

Modify `SSGAdapter::generate_nav()` to accept an optional prefix and prepend it to all nav entry file paths. Update both MkDocs and mdBook adapter implementations. Also update the CLI-level `generate_ssg_files()` to pass the prefix through.

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [ ] `SSGAdapter::generate_nav()` signature accepts `prefix: Option<&str>`
- [ ] `MkDocsAdapter` prepends prefix to all nav entry paths (e.g., `rust/plissken.md` → `api/rust/plissken.md`)
- [ ] `MdBookAdapter` prepends prefix to all SUMMARY.md link paths
- [ ] `None` prefix produces identical output to current behavior
- [ ] Prefix is normalized (no trailing slash issues)
- [ ] `ModuleRenderer` nav generation methods (`generate_nav_yaml`, `generate_mdbook_summary`) pass prefix through

## Implementation Notes

### Files to modify
- `crates/plissken-core/src/render/ssg/traits.rs` — update `SSGAdapter` trait signature
- `crates/plissken-core/src/render/ssg/mkdocs.rs` — prepend prefix in `generate_nav`
- `crates/plissken-core/src/render/ssg/mdbook.rs` — prepend prefix in `generate_nav`
- `crates/plissken-core/src/render/module_renderer.rs` — thread prefix through `generate_nav_yaml()` and `generate_mdbook_summary()`
- `crates/plissken-cli/src/main.rs` — pass prefix to `generate_ssg_files()`

### Approach
Simplest change: prepend at the formatting point in each adapter's `generate_nav`, not in `NavEntry` itself. This keeps the data model clean and localizes the prefix logic to output formatting.

```rust
// In mkdocs.rs generate_nav:
let path_str = match prefix {
    Some(p) => format!("{}/{}", p, entry.file_path.display()),
    None => format!("{}", entry.file_path.display()),
};
```

### Dependencies
- PLSKN-T-0060 (prefix field must exist in config/CLI first)

## Status Updates

- Added `prefix_path()` helper in `traits.rs` — shared by both adapters
- Updated `SSGAdapter::generate_nav()` trait to accept `prefix: Option<&str>`
- Updated `MkDocsAdapter` and `MdBookAdapter` to use `prefix_path()` on all nav entry paths
- Updated `ModuleRenderer::generate_nav_yaml()`, `generate_mdbook_summary()`, `generate_nav()` to accept and pass prefix
- Fixed existing tests to pass `None` for backward compat
- Fixed `OutputConfig` constructors in config.rs and crossref.rs test fixtures
- Added `#[allow(clippy::too_many_arguments)]` to `generate_ssg_files` (8 args, was 7)
- All unit tests pass, clippy clean