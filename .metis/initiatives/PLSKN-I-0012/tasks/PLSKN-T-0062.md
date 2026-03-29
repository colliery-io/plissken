----
id: unit-tests-for-prefix-behavior
level: task
title: "Unit tests for prefix behavior"
short_code: "PLSKN-T-0062"
created_at: 2026-03-29T13:38:57.143728+00:00
updated_at: 2026-03-29T13:58:10.386902+00:00
parent: PLSKN-I-0012
blocked_by: []
archived: false

tags:
  - "#task"
  - "#phase/completed"


exit_criteria_met: false
initiative_id: PLSKN-I-0012
---

# Unit tests for prefix behavior

## Parent Initiative

[[PLSKN-I-0012]]

## Objective

Add unit tests covering prefix behavior for config deserialization, MkDocs nav generation, and mdBook nav generation. Verify backward compatibility (no prefix = unchanged output).

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [ ] Config test: `OutputConfig` with `prefix = "api"` deserializes correctly
- [ ] Config test: `OutputConfig` without `prefix` deserializes as `None`
- [ ] MkDocs test: `generate_nav` with `prefix = Some("api")` produces `api/rust/...` paths
- [ ] MkDocs test: `generate_nav` with `prefix = None` produces unchanged output
- [ ] mdBook test: `generate_nav` with `prefix = Some("api")` produces `api/rust/...` link targets
- [ ] mdBook test: `generate_nav` with `prefix = None` produces unchanged output
- [ ] Prefix normalization: trailing slashes stripped, empty string treated as None

## Implementation Notes

### Files to modify
- `crates/plissken-core/src/config.rs` — add tests in `mod tests`
- `crates/plissken-core/src/render/ssg/mkdocs.rs` — add tests in `mod tests`
- `crates/plissken-core/src/render/ssg/mdbook.rs` — add tests in `mod tests`

### Dependencies
- PLSKN-T-0060, PLSKN-T-0061 (tests validate the implementation from those tasks)

## Status Updates

- Added 3 config deserialization tests: prefix present, prefix absent (defaults None), empty string
- Added 3 MkDocs nav tests: with prefix (Python), with prefix (Rust), None prefix unchanged
- Added 3 mdBook nav tests: with prefix (Python), with prefix (Rust), None prefix unchanged
- Added 2 `prefix_path` helper tests in ssg/mod.rs
- All 272 unit tests + 16 doc tests pass