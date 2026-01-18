---
id: set-up-ci-cd-pipeline-with-crates
level: task
title: "Set up CI/CD pipeline with crates.io and install script distribution"
short_code: "PLSKN-T-0038"
created_at: 2026-01-17T14:45:41.763388+00:00
updated_at: 2026-01-17T16:48:28.782608+00:00
parent: 
blocked_by: []
archived: true

tags:
  - "#task"
  - "#feature"
  - "#phase/completed"


exit_criteria_met: false
strategy_id: NULL
initiative_id: NULL
---

# Set up CI/CD pipeline with crates.io and install script distribution

## Objective

Set up a complete CI/CD pipeline for plissken that:
1. Runs tests and linting on PRs
2. Builds cross-platform binaries on release tags
3. Publishes to crates.io automatically
4. Creates GitHub releases with downloadable binaries
5. Provides a curl | bash install script

Repository is moving from `dylanbstorey/plissken` to `colliery-io/plissken`.

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [x] Update repository URLs in Cargo.toml to colliery-io/plissken
- [x] Add crates.io metadata (description, keywords, categories, readme, documentation)
- [x] Fix existing CI workflow (rust-action typo)
- [x] Create release workflow that triggers on version tags (v*)
- [x] Build binaries for: Linux (x86_64, aarch64), macOS (x86_64, aarch64), Windows (x86_64)
- [x] Publish plissken-core and plissken to crates.io on release
- [x] Create GitHub release with binary assets
- [x] Create install.sh script that auto-detects platform
- [x] Update installation docs to include curl | bash method

## Implementation Checklist

### 1. Cargo.toml Updates
- [x] Update workspace repository URL to colliery-io/plissken
- [x] Add description to plissken-core
- [x] Add description to plissken (CLI)
- [x] Add keywords, categories, readme, documentation fields
- [x] Add homepage field

### 2. CI Workflow Fixes
- [x] Fix `dtolnay/rust-action` → `dtolnay/rust-toolchain`
- [x] Add format check (cargo fmt)

### 3. Release Workflow
- [x] Create `.github/workflows/release.yml`
- [x] Matrix build for all target platforms
- [x] Use cross for cross-compilation
- [x] Upload artifacts to GitHub release
- [x] Publish to crates.io with CARGO_REGISTRY_TOKEN

### 4. Install Script
- [x] Create `install.sh` at repo root
- [x] Detect OS (Linux, macOS, Windows via WSL)
- [x] Detect architecture (x86_64, aarch64/arm64)
- [x] Download correct binary from GitHub releases
- [x] Install to ~/.local/bin

### 5. Documentation Updates
- [x] Update docs/getting-started/installation.md with curl | bash method
- [x] Add badges to README (crates.io version, CI status)
- [x] Create README.md (replacing old Python README.rst)
- [x] Update docs/index.md with new installation tabs

## Status Updates

**2026-01-17**: Completed all tasks
- Updated Cargo.toml (workspace) with colliery-io/plissken repo URL and crates.io metadata
- Updated crates/plissken-core/Cargo.toml and crates/plissken-cli/Cargo.toml with descriptions
- Fixed CI workflow: dtolnay/rust-action → dtolnay/rust-toolchain, added fmt job
- Created release.yml workflow with matrix builds for 6 targets
- Created install.sh script with platform detection
- Created new README.md with badges and current installation methods
- Updated docs/index.md and docs/getting-started/installation.md
- Removed Python/pip references (Python CLI is deprecated)