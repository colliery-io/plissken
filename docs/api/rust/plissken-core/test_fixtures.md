# plissken-core::test_fixtures <span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #ff5722; color: white;">Rust</span>


Test fixture utilities for plissken-core tests.

This module provides helpers for accessing test fixtures in the workspace's
`tests/fixtures/` directory. It centralizes fixture path construction and
provides convenient accessors for common fixture files.

**Examples:**

```ignore
use crate::test_fixtures::{fixtures_root, fixture_path};

let hybrid_binary = fixture_path("hybrid_binary/rust/lib.rs");
let pure_python = fixtures_root().join("pure_python/scheduler.py");
```

## Functions

### `plissken-core::test_fixtures::fixtures_root`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


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



### `plissken-core::test_fixtures::fixture_path`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn fixture_path (relative_path : & str) -> PathBuf
```

Returns the full path to a fixture file or directory.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `relative_path` | `-` | Path relative to the fixtures directory |


**Examples:**

```ignore
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



