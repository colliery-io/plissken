---
id: subfolder-aware-rendering-for-ci
level: initiative
title: "Subfolder-aware rendering for CI/CD integration"
short_code: "PLSKN-I-0012"
created_at: 2026-03-29T13:15:57.385560+00:00
updated_at: 2026-03-29T14:07:50.213083+00:00
parent: 
blocked_by: []
archived: false

tags:
  - "#initiative"
  - "#phase/completed"


exit_criteria_met: false
estimated_complexity: M
initiative_id: subfolder-aware-rendering-for-ci
---

# Subfolder-aware rendering for CI/CD integration Initiative

## Context

Plissken's `render` command generates markdown files and SSG navigation (MkDocs `_nav.yml`, mdBook `SUMMARY.md`) with paths hardcoded relative to the output root. This works when plissken owns the entire doc site, but breaks when integrating into an existing documentation structure — the common case for CI/CD pipelines.

**Current pain points:**

1. **Nav paths lack a prefix** — `_nav.yml` entries say `helpers.md` but if output lives in `docs/api/`, MkDocs needs `api/helpers.md`
2. **Cross-reference links assume doc root** — the `../` prefix calculations in `CrossRefLinker` don't account for nesting depth within a larger site
3. **No clean CI/CD story** — generating into a subfolder of an existing docs site requires manual path fixups, making automation fragile
4. **`generate` JSON model** — the JSON output has no awareness of where docs will be mounted, so downstream tooling can't compute correct links either

## Goals & Non-Goals

**Goals:**
- Add a `prefix` concept so rendered output and nav entries are aware of their mount point within a larger doc site
- Cross-reference links work correctly when output is nested in a subfolder
- Nav file output (`_nav.yml`, `SUMMARY.md`) uses prefix-aware paths that can be directly included in an existing SSG config
- CLI flag (`--prefix`) for CI/CD pipelines to set this without modifying config
- Config option (`output.prefix`) for projects that always render to a subfolder

**Non-Goals:**
- Multi-project aggregation (rendering multiple plissken projects into one site) — separate initiative
- Generating the full `mkdocs.yml` or `book.toml` for the parent site
- Changing the existing default behavior (no prefix = current behavior)

## Use Cases

### UC-1: CI/CD pipeline rendering into existing docs site
- **Actor**: CI pipeline
- **Scenario**: `plissken render . -o docs/api/reference --prefix api/reference` — generates files into the subfolder with nav paths prefixed so they work when included in the parent `mkdocs.yml`
- **Expected Outcome**: `_nav.yml` has entries like `api/reference/helpers.md`, cross-ref links resolve correctly

### UC-2: Config-driven subfolder for a monorepo
- **Actor**: Developer with `plissken.toml`
- **Scenario**: Config has `[output] prefix = "api"`, `plissken render .` generates into configured output path with prefixed nav entries
- **Expected Outcome**: Same as UC-1 but driven by config, no CLI flag needed

### UC-3: Backward compatibility
- **Actor**: Existing users
- **Scenario**: No prefix configured or passed — behavior is identical to today
- **Expected Outcome**: Zero breakage for existing workflows

## Detailed Design

### Approach: `output.prefix` config field + `--prefix` CLI flag

**Config (`plissken.toml`):**
```toml
[output]
path = "docs"
prefix = "api/reference"  # new optional field, default ""
template = "mkdocs-material"
```

**CLI:**
```
plissken render . --prefix api/reference
```

CLI `--prefix` overrides config `output.prefix`.

### What prefix affects

| Component | Without prefix | With `prefix = "api"` |
|-----------|---------------|----------------------|
| File output paths | `docs/helpers.md` | `docs/helpers.md` (unchanged — files go where `-o` says) |
| MkDocs nav entries | `helpers.md` | `api/helpers.md` |
| mdBook SUMMARY links | `helpers.md` | `api/helpers.md` |
| Cross-ref links (Python↔Rust) | `../rust/mycrate.md` | `../rust/mycrate.md` (relative links unaffected) |
**Key insight**: The prefix affects **nav paths** (which are relative to the SSG root, not the content directory) but does NOT affect cross-reference links between rendered pages (which are relative to each other and stay correct regardless of mount point). The `generate` JSON command is internal and does not need prefix awareness.

### Changes needed

1. **`OutputConfig`** — add `prefix: Option<String>` field with serde default
2. **`Commands::Render`** — add `--prefix` CLI arg
3. **`SSGAdapter::generate_nav()`** — accept prefix parameter, prepend to all nav entry paths
4. **`NavEntry`** — or prepend at the point where nav entries are formatted (simpler)
5. **`generate` command** — include `doc_root` in `ProjectMetadata` if prefix is set
6. **Tests** — unit tests for prefix behavior in both SSG adapters, integration test with prefix flag

## Alternatives Considered

1. **Post-processing script** — Users sed/awk the nav file after generation. Fragile, defeats the purpose of a clean pipeline.
2. **Separate `mount` command** — A post-render step that rewrites paths. Adds complexity for something that should be a render-time concern.
3. **Infer prefix from output path** — Auto-detect based on `-o docs/api` that prefix should be `api`. Too magical, breaks when output path doesn't match SSG structure. **Decision: rejected, keep explicit.**

## Implementation Plan

1. Add `prefix` to `OutputConfig` and `--prefix` CLI arg on `Render` command
2. Thread prefix through SSG nav generation (`SSGAdapter::generate_nav`)
3. Unit tests for prefix in both MkDocs and mdBook adapters
4. Functional test: dogfood on plissken's own docs
5. Update CLI help text

## Testing Strategy

### Dogfood functional test (primary)

Plissken's own docs are the real-world fixture. Today:
- `plissken.toml` outputs to `docs/api`
- `_nav.yml` generates paths like `rust/plissken.md`
- `mkdocs.yml` has a **hand-maintained** nav section with `api/` manually prefixed to every path (lines 93-121)

After this feature, the angreal functional test should:
1. `plissken render . -o docs/api --prefix api`
2. Verify `_nav.yml` entries are prefixed (e.g., `api/rust/plissken.md`)
3. Verify `mkdocs build` succeeds with the generated nav replacing the hand-maintained section
4. Eventually: update `plissken.toml` to include `prefix = "api"` and retire the manual nav block

### Unit tests
- `OutputConfig` deserializes `prefix` (defaults to `None`)
- `MkDocsAdapter::generate_nav()` with prefix prepends to all paths
- `MdBookAdapter::generate_nav()` with prefix prepends to all link paths
- No prefix = identical output to today (backward compat)