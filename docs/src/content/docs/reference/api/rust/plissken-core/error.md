---
title: "plissken-core::error"
description: "Unified error types for plissken-core."
editUrl: false
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

### <span id="enum.PlisskenError"></span><code style="font-size: 0.85em">enum PlisskenError</code>

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

#### Methods

##### <span id="PlisskenError.rust_parse"></span><code>rust_​parse</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn rust_parse(
    path: impl Into<PathBuf>,
    message: impl Into<String>,
) -> Self
```

Create a parse error for a Rust file.

<details>
<summary>Source</summary>

```rust
    pub fn rust_parse(path: impl Into<PathBuf>, message: impl Into<String>) -> Self {
        PlisskenError::Parse {
            language: "Rust".into(),
            path: path.into(),
            line: None,
            message: message.into(),
        }
    }
```

</details>

##### <span id="PlisskenError.rust_parse_at"></span><code>rust_​parse_​at</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn rust_parse_at(
    path: impl Into<PathBuf>,
    line: usize,
    message: impl Into<String>,
) -> Self
```

Create a parse error for a Rust file with line number.

<details>
<summary>Source</summary>

```rust
    pub fn rust_parse_at(
        path: impl Into<PathBuf>,
        line: usize,
        message: impl Into<String>,
    ) -> Self {
        PlisskenError::Parse {
            language: "Rust".into(),
            path: path.into(),
            line: Some(line),
            message: message.into(),
        }
    }
```

</details>

##### <span id="PlisskenError.python_parse"></span><code>python_​parse</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn python_parse(
    path: impl Into<PathBuf>,
    message: impl Into<String>,
) -> Self
```

Create a parse error for a Python file.

<details>
<summary>Source</summary>

```rust
    pub fn python_parse(path: impl Into<PathBuf>, message: impl Into<String>) -> Self {
        PlisskenError::Parse {
            language: "Python".into(),
            path: path.into(),
            line: None,
            message: message.into(),
        }
    }
```

</details>

##### <span id="PlisskenError.python_parse_at"></span><code>python_​parse_​at</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn python_parse_at(
    path: impl Into<PathBuf>,
    line: usize,
    message: impl Into<String>,
) -> Self
```

Create a parse error for a Python file with line number.

<details>
<summary>Source</summary>

```rust
    pub fn python_parse_at(
        path: impl Into<PathBuf>,
        line: usize,
        message: impl Into<String>,
    ) -> Self {
        PlisskenError::Parse {
            language: "Python".into(),
            path: path.into(),
            line: Some(line),
            message: message.into(),
        }
    }
```

</details>

##### <span id="PlisskenError.file_read"></span><code>file_​read</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn file_read(path: impl Into<PathBuf>, source: std::io::Error) -> Self
```

Create a file read error.

<details>
<summary>Source</summary>

```rust
    pub fn file_read(path: impl Into<PathBuf>, source: std::io::Error) -> Self {
        let path = path.into();
        PlisskenError::FileRead {
            message: source.to_string(),
            path,
            source,
        }
    }
```

</details>

##### <span id="PlisskenError.io"></span><code>io</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn io(context: impl Into<String>, source: std::io::Error) -> Self
```

Create an IO error with context.

<details>
<summary>Source</summary>

```rust
    pub fn io(context: impl Into<String>, source: std::io::Error) -> Self {
        PlisskenError::Io {
            context: context.into(),
            source,
        }
    }
```

</details>

##### <span id="PlisskenError.config_not_found"></span><code>config_​not_​found</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn config_not_found(path: impl Into<PathBuf>) -> Self
```

Create a config not found error.

<details>
<summary>Source</summary>

```rust
    pub fn config_not_found(path: impl Into<PathBuf>) -> Self {
        PlisskenError::ConfigNotFound { path: path.into() }
    }
```

</details>

##### <span id="PlisskenError.discovery"></span><code>discovery</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn discovery(
    path: impl Into<PathBuf>,
    message: impl Into<String>,
) -> Self
```

Create a discovery error.

<details>
<summary>Source</summary>

```rust
    pub fn discovery(path: impl Into<PathBuf>, message: impl Into<String>) -> Self {
        PlisskenError::Discovery {
            path: path.into(),
            message: message.into(),
        }
    }
```

</details>

## Type aliases

| Name | Type | Description |
|------|------|-------------|
| <span id="type.Result"></span>`Result<T>` | `std::result::Result<T, PlisskenError>` | A specialized Result type for plissken operations. |
