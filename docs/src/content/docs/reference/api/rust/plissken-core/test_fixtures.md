---
title: "plissken-core::test_fixtures"
description: "Test fixture utilities for plissken-core tests."
sidebar:
  order: 29
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>


Test fixture utilities for plissken-core tests.

This module provides helpers for accessing test fixtures in the workspace's
`tests/fixtures/` directory. It centralizes fixture path construction and
provides convenient accessors for common fixture files.

**Examples:**

```rust
use crate::test_fixtures::{fixtures_root, fixture_path};

let hybrid_binary = fixture_path("hybrid_binary/rust/lib.rs");
let pure_python = fixtures_root().join("pure_python/scheduler.py");
```

## Functions

### <span id="fn.fixtures_root"></span>`fn fixtures_root`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


```rust
fn fixtures_root () -> PathBuf
```

Returns the path to the workspace's `tests/fixtures/` directory.

This navigates from the crate's manifest directory up to the workspace
root and into the fixtures directory.

<details>
<summary>Source</summary>

```rust
pub fn fixtures_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crates/ directory")
        .parent()
        .expect("workspace root")
        .join("tests/fixtures")
}
```

</details>



### <span id="fn.fixture_path"></span>`fn fixture_path`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


```rust
fn fixture_path (relative_path : & str) -> PathBuf
```

Returns the full path to a fixture file or directory.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `relative_path` | `-` | Path relative to the fixtures directory |


**Examples:**

```rust
let lib_rs = fixture_path("hybrid_binary/rust/lib.rs");
let scheduler = fixture_path("pure_python/scheduler.py");
```

<details>
<summary>Source</summary>

```rust
pub fn fixture_path(relative_path: &str) -> PathBuf {
    fixtures_root().join(relative_path)
}
```

</details>



