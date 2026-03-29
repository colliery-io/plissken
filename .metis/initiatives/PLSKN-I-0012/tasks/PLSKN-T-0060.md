---
id: add-prefix-field-to-outputconfig
level: task
title: "Add prefix field to OutputConfig and CLI"
short_code: "PLSKN-T-0060"
created_at: 2026-03-29T13:38:50.344355+00:00
updated_at: 2026-03-29T13:51:58.393910+00:00
parent: PLSKN-I-0012
blocked_by: []
archived: false

tags:
  - "#task"
  - "#phase/completed"


exit_criteria_met: false
initiative_id: PLSKN-I-0012
---

# Add prefix field to OutputConfig and CLI

## Parent Initiative

[[PLSKN-I-0012]]

## Objective

Add `prefix: Option<String>` to `OutputConfig` in `config.rs` and `--prefix` CLI arg to the `Render` command. Thread the resolved prefix value from CLI through to where SSG files are generated.

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [ ] `OutputConfig` has `prefix: Option<String>` with `#[serde(default)]`
- [ ] `Commands::Render` has `--prefix` arg that overrides config value
- [ ] `plissken.toml` with `prefix = "api"` deserializes correctly
- [ ] `plissken.toml` without `prefix` deserializes as `None` (backward compat)
- [ ] Resolved prefix is passed to `generate_ssg_files()` and `write_rendered_pages()` call sites

## Implementation Notes

### Files to modify
- `crates/plissken-core/src/config.rs` — add field to `OutputConfig`
- `crates/plissken-cli/src/main.rs` — add `--prefix` to `Commands::Render`, resolve in `render()`, pass through

### Prefix resolution
```
let prefix = prefix_override.or(config.output.prefix.clone());
```

CLI `--prefix` wins over config `output.prefix`. Both `None` = no prefix (current behavior).

## Status Updates

- Added `prefix: Option<String>` to `OutputConfig` in `config.rs`
- Added `--prefix` CLI arg to `Commands::Render` in `main.rs`
- Added `resolve_prefix()` helper that normalizes trailing slashes and treats empty string as None
- Threaded resolved prefix through to `generate_ssg_files()` call site
- Build fails on expected `ModuleRenderer` signature mismatches (T-0061 will fix)