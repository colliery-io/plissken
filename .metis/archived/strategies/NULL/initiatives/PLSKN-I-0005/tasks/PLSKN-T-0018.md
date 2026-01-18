---
id: add-package-crate-directory-level
level: task
title: "Add package/crate directory level to output paths"
short_code: "PLSKN-T-0018"
created_at: 2026-01-16T02:44:50.972076+00:00
updated_at: 2026-01-16T02:50:51.764387+00:00
parent: PLSKN-I-0005
blocked_by: []
archived: true

tags:
  - "#task"
  - "#phase/completed"


exit_criteria_met: false
strategy_id: NULL
initiative_id: PLSKN-I-0005
---

# Add package/crate directory level to output paths

## Parent Initiative

[[PLSKN-I-0005]]

## Objective

Update the output path generation in `module_renderer.rs` to include the package/crate name as a directory level under the language separator.

**Current behavior:**
- `pysnake.handlers` → `python/handlers/`
- `rustscale::internal` → `rust/internal/`

**Expected behavior:**
- `pysnake.handlers` → `python/pysnake/handlers/`
- `rustscale::internal` → `rust/rustscale/internal/`

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [ ] Python modules output to `python/{package}/{submodule}/...`
- [ ] Rust modules output to `rust/{crate}/{submodule}/...`
- [ ] Root package outputs to `python/{package}/index.md`
- [ ] Root crate outputs to `rust/{crate}/index.md`
- [ ] Test with minimal_hybrid fixture shows correct paths

## Implementation Notes

### Files to Modify

`crates/plissken-core/src/render/module_renderer.rs`:
- Line ~72: Python `module_dir` construction
- Line ~416: Rust `module_dir` construction

### Change

Python (current):
```rust
let module_dir = if parts.len() > 1 {
    format!("python/{}", parts[1..].join("/"))
} else {
    "python".to_string()
};
```

Python (new):
```rust
let module_dir = format!("python/{}", parts.join("/"));
```

Same pattern for Rust with `::` separator.

## Status Updates

- Started implementation, modified path generation for both Python and Rust