---
title: "plissken-core::error"
description: "Unified error types for plissken-core."
sidebar:
  order: 7
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>


Unified error types for plissken-core.

This module provides a single error enum that covers all error cases
in the library, replacing the previous mix of `anyhow::Error`,
`tera::Error`, and other error types.

## Enums

### <span id="enum.PlisskenError"></span>`enum PlisskenError`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


The primary error type for plissken-core operations.

This enum covers all error categories that can occur during parsing,
rendering, and configuration handling.

#### Variants

- **`ConfigNotFound`** - Configuration file not found at the expected path.
- **`ConfigParse`** - Failed to parse configuration file.
- **`ConfigValidation`** - Configuration validation failed.
- **`Parse`** - Failed to parse a source file.
- **`FileRead`** - Failed to read a source file.
- **`Template`** - Template rendering failed.
- **`OutputWrite`** - Failed to write output file.
- **`CrossRef`** - Cross-reference resolution failed.
- **`Io`** - Generic IO error with context.
- **`Discovery`** - Module discovery failed.
- **`ManifestParse`** - Failed to parse manifest file (Cargo.toml or pyproject.toml).



