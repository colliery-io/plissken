---
id: cli-ux-invisible-by-default
level: initiative
title: "CLI UX: Invisible by Default"
short_code: "PLSKN-I-0006"
created_at: 2026-01-16T17:49:13.468213+00:00
updated_at: 2026-01-17T01:26:15.005838+00:00
parent: PLSKN-V-0001
blocked_by: []
archived: true

tags:
  - "#initiative"
  - "#phase/completed"


exit_criteria_met: false
estimated_complexity: S
strategy_id: NULL
initiative_id: cli-ux-invisible-by-default
---

# CLI UX: Invisible by Default Initiative

## Context

The plissken CLI currently violates its core "invisible documentation" philosophy:

1. **Verbose by default**: Every run dumps 30+ lines of progress to stderr, creating noise in CI/CD logs
2. **Init command is a stub**: First-time users run `plissken init` and get "not yet implemented"
3. **Generate dumps JSON to stdout**: Running `plissken generate` without flags dumps 145KB+ of JSON
4. **Cryptic error messages**: "plissken.toml not found" provides no recovery guidance

Comparison to established tools:
- `cargo build` is silent on success
- `npm init` works interactively  
- Both provide helpful error messages with suggestions

## Goals & Non-Goals

**Goals:**
- Silent success by default (cargo-style), verbose opt-in with `--verbose`
- Working `plissken init` command with interactive project detection
- Helpful error messages that suggest recovery actions
- `plissken check` command to validate config without running

**Non-Goals:**
- Full wizard-style setup (simple prompts are sufficient)
- Watch mode or incremental builds (future initiative)
- Colorized output or progress bars (keep it simple)

## Detailed Design

### Verbosity Control
- Add `--verbose/-v` flag to `generate` and `render` commands
- Default behavior: silent on success, only show errors/warnings
- Verbose mode: show current progress messages
- Final summary line on success: `Generated docs for 9 modules → docs/api/`

### Init Command
- Detect project type by scanning for Cargo.toml / pyproject.toml
- Generate minimal plissken.toml with sensible defaults
- Show what was detected and created

### Error Messages
- Include context about what went wrong
- Suggest recovery actions
- Example: "plissken.toml not found. Run `plissken init` to create one."

### Check Command
- Validate config syntax and semantics
- Verify paths exist
- List discovered modules without generating

## Implementation Plan

**Phase 1: Quick Wins (2-3 hours)**
- Add --verbose flag, make quiet the default
- Improve error messages with recovery hints

**Phase 2: Init Command (4-6 hours)**
- Implement project detection (Cargo.toml, pyproject.toml)
- Generate minimal valid config
- Test with various project layouts

**Phase 3: Check Command (2-3 hours)**
- Add `plissken check` subcommand
- Validate config and show discovered modules