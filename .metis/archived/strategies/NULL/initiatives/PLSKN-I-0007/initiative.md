---
id: configuration-simplification
level: initiative
title: "Configuration Simplification"
short_code: "PLSKN-I-0007"
created_at: 2026-01-16T17:49:13.510139+00:00
updated_at: 2026-01-17T01:25:43.654617+00:00
parent: PLSKN-V-0001
blocked_by: []
archived: true

tags:
  - "#initiative"
  - "#phase/completed"


exit_criteria_met: false
estimated_complexity: M
strategy_id: NULL
initiative_id: configuration-simplification
---

# Configuration Simplification Initiative

## Context

The current configuration system creates unnecessary friction:

1. **No semantic validation**: Empty `[rust]` + `[python]` sections silently generate nothing
2. **Python module enumeration burden**: Must explicitly list every module in `[python.modules]` (50 modules = 50 lines)
3. **No auto-detection**: Can't infer project structure from Cargo.toml/pyproject.toml
4. **Inconsistent mental model**: Rust uses `crates` (array) vs Python uses `package` (string)
5. **Missing path validation**: Configured paths aren't validated until runtime

Current minimal config for a 10-module Python project requires ~15 lines when it could be 3.

## Goals & Non-Goals

**Goals:**
- Add `Config::validate()` method with semantic validation
- Auto-discover Python modules instead of explicit enumeration
- Infer defaults from Cargo.toml and pyproject.toml
- Validate that configured paths actually exist
- Provide helpful validation error messages

**Non-Goals:**
- Zero-config operation (some config is appropriate)
- Supporting non-standard project layouts automatically
- Backwards-incompatible config format changes

## Detailed Design

### Semantic Validation
```rust
impl Config {
    pub fn validate(&self) -> Result<Vec<ConfigWarning>, ConfigError> {
        // Must have at least one language configured
        // Validate path existence
        // Validate version_from source exists
        // Warn on unused/deprecated fields
    }
}
```

### Python Auto-Discovery
```toml
[python]
package = "mylib"
auto_discover = true  # Walk filesystem instead of enumeration
```

### Manifest Inference
- Read crate name from Cargo.toml for `entry_point` default
- Read package name from pyproject.toml for `package` default
- Infer source paths from standard layouts

## Implementation Plan

**Phase 1: Validation (3-4 hours)**
- Add `Config::validate()` method
- Call validation early in generate/render commands
- Helpful error messages for common mistakes

**Phase 2: Auto-Discovery (4-6 hours)**
- Add `auto_discover` option for Python modules
- Walk filesystem to find Python files
- Detect pyo3 vs pure Python based on content

**Phase 3: Manifest Inference (3-4 hours)**
- Parse Cargo.toml for crate name
- Parse pyproject.toml for package name
- Use as defaults when not explicitly configured