---
id: update-index-page-titles-to-full
level: task
title: "Update index page titles to full namespace paths"
short_code: "PLSKN-T-0019"
created_at: 2026-01-16T02:44:51.014012+00:00
updated_at: 2026-01-16T02:50:58.873419+00:00
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

# Update index page titles to full namespace paths

## Parent Initiative

[[PLSKN-I-0005]]

## Objective

Ensure all index.md page titles use the full namespace path (e.g., `pysnake.utils` not `utils`).

This may already be working correctly since `module.path` contains the full namespace. Needs verification.

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [ ] Python index pages have H1 titles like `# pysnake.handlers`
- [ ] Rust index pages have H1 titles like `# rustscale::internal`
- [ ] Root module pages show `# pysnake` and `# rustscale`

## Implementation Notes

### Current State

The renderer already uses `module.path` for titles:
```rust
content.push_str(&format!("# {} {}\n\n", module.path, source_badge));
```

This should already produce correct titles. Task is primarily verification.

### Files to Check

`crates/plissken-core/src/render/module_renderer.rs`:
- `render_python_module_index()` 
- `render_rust_module_index()`

## Status Updates

*To be added during implementation*