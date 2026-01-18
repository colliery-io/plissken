---
id: add-expandable-symbol-entries-to
level: task
title: "Add expandable symbol entries to nav"
short_code: "PLSKN-T-0021"
created_at: 2026-01-16T02:44:51.100386+00:00
updated_at: 2026-01-16T02:55:27.147003+00:00
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

# Add expandable symbol entries to nav

## Parent Initiative

[[PLSKN-I-0005]]

## Objective

Extend nav generation to include symbol entries (classes, functions, etc.) as children of each namespace, making them expandable in the sidebar.

**Target nav structure:**
```yaml
nav:
  - Python:
    - pysnake:
      - python/pysnake/index.md
      - Snake: python/pysnake/Snake.md
      - SnakeColor: python/pysnake/SnakeColor.md
      - Functions: python/pysnake/functions.md
    - pysnake.handlers:
      - python/pysnake/handlers/index.md
      - EventHandler: python/pysnake/handlers/EventHandler.md
      - Functions: python/pysnake/handlers/functions.md
```

This allows users to expand a namespace in the sidebar and see all symbols within it.

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [ ] Each namespace in nav has child entries for its symbols
- [ ] Classes/structs link to their individual pages
- [ ] Functions grouped under "Functions" entry
- [ ] Nav is expandable/collapsible in MkDocs Material
- [ ] Symbol order: Classes first, then Enums, then Functions

## Implementation Notes

### MkDocs Material Requirements

MkDocs Material supports nested nav with `navigation.expand` feature. The structure needs to be:
```yaml
- Section Label:
  - path/to/index.md  # Section landing page
  - Child 1: path/to/child1.md
  - Child 2: path/to/child2.md
```

### Dependencies

- Depends on PLSKN-T-0020 (basic nav generation)
- Requires access to parsed module data (classes, functions per module)

### Complexity

This is the most complex nav task - needs to track all symbols per module and organize them in the nav structure.

## Status Updates

*To be added during implementation*