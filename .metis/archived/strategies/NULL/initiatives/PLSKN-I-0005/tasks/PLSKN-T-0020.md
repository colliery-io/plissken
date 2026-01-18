---
id: generate-nav-config-with-full
level: task
title: "Generate nav config with full namespace paths"
short_code: "PLSKN-T-0020"
created_at: 2026-01-16T02:44:51.057154+00:00
updated_at: 2026-01-16T02:54:40.336998+00:00
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

# Generate nav config with full namespace paths

## Parent Initiative

[[PLSKN-I-0005]]

## Objective

Generate a `nav` section for mkdocs.yml that shows full namespace paths in the sidebar, organized by language.

**Target nav structure:**
```yaml
nav:
  - Python:
    - pysnake: python/pysnake/index.md
    - pysnake.handlers: python/pysnake/handlers/index.md
    - pysnake.utils: python/pysnake/utils/index.md
    - pysnake.utils.validation: python/pysnake/utils/validation/index.md
  - Rust:
    - rustscale: rust/rustscale/index.md
    - rustscale::handlers: rust/rustscale/handlers/index.md
    - rustscale::internal: rust/rustscale/internal/index.md
    - rustscale::internal::parser: rust/rustscale/internal/parser/index.md
```

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [ ] Plissken generates nav config automatically (no manual nav)
- [ ] Nav shows full namespace paths as labels
- [ ] Nav is grouped by language (Python / Rust)
- [ ] Nav output written to mkdocs.yml or separate nav file
- [ ] Namespaces appear in logical order (root first, then alphabetical)

## Implementation Notes

### Approach Options

1. **Generate full mkdocs.yml** - Replace entire file with generated config
2. **Generate nav include** - Write `nav.yml` that mkdocs.yml imports
3. **Append nav section** - Modify existing mkdocs.yml to add/update nav

Option 2 is cleanest - generate `_nav.yml` and have mkdocs.yml use `!include`.

### Dependencies

- Depends on PLSKN-T-0018 (correct directory structure)
- Depends on PLSKN-T-0019 (correct page titles)

### Files to Modify

- `crates/plissken-core/src/render/mod.rs` or new `nav_generator.rs`
- Called from CLI after rendering all pages

## Status Updates

*To be added during implementation*