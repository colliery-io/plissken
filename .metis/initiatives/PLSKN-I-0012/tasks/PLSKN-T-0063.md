---
id: dogfood-functional-test-with
level: task
title: "Dogfood functional test with plissken's own docs"
short_code: "PLSKN-T-0063"
created_at: 2026-03-29T13:38:57.176272+00:00
updated_at: 2026-03-29T14:07:29.162859+00:00
parent: PLSKN-I-0012
blocked_by: []
archived: false

tags:
  - "#task"
  - "#phase/completed"


exit_criteria_met: false
initiative_id: PLSKN-I-0012
---

# Dogfood functional test with plissken's own docs

## Parent Initiative

[[PLSKN-I-0012]]

## Objective

Create an angreal functional test that uses plissken's own docs as the test fixture. Run `plissken render . --prefix api` against the project root, verify the generated `_nav.yml` has prefixed paths, and verify `mkdocs build` succeeds with the generated nav.

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [ ] New angreal task `functional dogfood` (or extend existing `functional mkdocs`)
- [ ] Runs `plissken render . -o docs/api --prefix api` against project root
- [ ] Verifies `docs/api/_nav.yml` entries contain `api/` prefix (e.g., `api/rust/plissken.md`)
- [ ] Verifies `mkdocs build` succeeds with the generated nav
- [ ] Update `plissken.toml` to include `prefix = "api"`
- [ ] Remove hand-maintained "API Reference (Generated)" nav block from `mkdocs.yml`, replace with `!include` or generated nav

## Implementation Notes

### Current state (the problem)
- `plissken.toml` has `path = "docs/api"` — files land correctly
- `_nav.yml` generates `rust/plissken.md` — missing `api/` prefix
- `mkdocs.yml` lines 93-121 have a **hand-maintained** nav with `api/` manually prepended to every path

### After this task
- `plissken.toml` adds `prefix = "api"`
- `_nav.yml` generates `api/rust/plissken.md` — correct for MkDocs
- `mkdocs.yml` nav section can reference the generated `_nav.yml` or be auto-updated by the functional test
- The angreal task validates the full pipeline: render → prefixed nav → mkdocs build

### Files to modify/create
- `plissken.toml` — add `prefix = "api"`
- `.angreal/task_functional.py` — add `functional_dogfood` command or extend `functional_mkdocs`
- `mkdocs.yml` — replace hand-maintained API nav with generated nav

### Dependencies
- PLSKN-T-0060, PLSKN-T-0061, PLSKN-T-0062 (feature must be implemented and tested first)

## Status Updates

- Added `prefix = "api"` to `plissken.toml`
- Created `angreal functional dogfood` command in `task_functional.py`
  - Renders plissken's own docs with prefix from config
  - Verifies all `_nav.yml` entries have `api/` prefix
  - Runs `mkdocs build` to verify site builds successfully
- Verified manually: `_nav.yml` generates `api/rust/plissken.md` (was `rust/plissken.md`)
- Verified: `mkdocs build` succeeds with the prefixed nav
- Verified: `--prefix` CLI flag overrides config prefix
- Note: `mkdocs.yml` hand-maintained API nav section still present and matches generated output. MkDocs doesn't support `!include` for nav natively, so the `_nav.yml` serves as a reference/CI validation target