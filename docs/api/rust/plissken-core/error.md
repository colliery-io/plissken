# error <span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #ff5722; color: white;">Rust</span>


Unified error types for plissken-core.

This module provides a single error enum that covers all error cases
in the library, replacing the previous mix of `anyhow::Error`,
`tera::Error`, and other error types.

## Enums

### `enum PlisskenError` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


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



