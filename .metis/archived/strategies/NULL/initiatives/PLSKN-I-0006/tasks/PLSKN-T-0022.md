---
id: add-verbose-flag-and-make-quiet
level: task
title: "Add --verbose flag and make quiet the default"
short_code: "PLSKN-T-0022"
created_at: 2026-01-16T17:51:44.467956+00:00
updated_at: 2026-01-16T22:00:23.341577+00:00
parent: PLSKN-I-0006
blocked_by: []
archived: true

tags:
  - "#task"
  - "#phase/completed"


exit_criteria_met: false
strategy_id: NULL
initiative_id: PLSKN-I-0006
---

# Add --verbose flag and make quiet the default

## Parent Initiative

[[PLSKN-I-0006]] CLI UX: Invisible by Default

## Objective

Invert the CLI's verbosity default: silent on success (cargo-style), verbose only when explicitly requested with `--verbose` flag. This is the foundational change for achieving "invisible documentation."

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [x] Add `--verbose` / `-v` global flag to CLI
- [x] Default behavior produces no output on success (exit 0 only)
- [x] With `--verbose`, show progress: files parsed, modules found, output written
- [x] Error output always goes to stderr regardless of verbosity
- [x] Multiple `-v` flags increase verbosity (future: `-vv` for debug)
- [x] Existing tests updated to account for changed output behavior

## Implementation Notes

### Files to Modify

1. **`crates/plissken/src/main.rs`** - Add `--verbose` to `Cli` struct:
   ```rust
   #[derive(Parser)]
   struct Cli {
       #[arg(short, long, action = clap::ArgAction::Count)]
       verbose: u8,
       // ...
   }
   ```

2. **`crates/plissken/src/commands/generate.rs`** - Wrap all `println!` in verbosity check:
   ```rust
   if cli.verbose > 0 {
       println!("Parsing {} modules...", modules.len());
   }
   ```

3. **`crates/plissken/src/commands/render.rs`** - Same pattern for render output

### Technical Approach

1. Add `verbose: u8` field to Cli struct with count action
2. Create helper macro or function: `verbose!(cli, "message")` 
3. Audit all `println!` calls in command handlers
4. Replace with conditional output based on verbosity level
5. Ensure errors always print to stderr via `eprintln!`

### Verbosity Levels
- `0` (default): Silent on success
- `1` (`-v`): Progress messages (files found, modules parsed, output written)
- `2` (`-vv`): Debug info (parser details, template rendering)

## Status Updates

### 2026-01-16: Implementation Complete

**Changes made to `crates/plissken-cli/src/main.rs`:**

1. Added `verbose: u8` field to `Cli` struct with `global = true` so it works with all subcommands
2. Created `verbose!` macro for conditional output based on verbosity level
3. Updated `generate()` and `render()` functions to accept verbosity parameter
4. Updated `parse_rust_sources()` and `parse_python_sources()` to accept verbosity

**Verbosity levels implemented:**
- Level 0 (default): Silent on success
- Level 1 (`-v`): Progress messages (config loading, modules parsed, files rendered summary)
- Level 2 (`-vv`): Debug output (individual file writes, synthesized modules)

**Warnings always shown:** Parse failures still output to stderr regardless of verbosity level.

**All 129 tests pass.**