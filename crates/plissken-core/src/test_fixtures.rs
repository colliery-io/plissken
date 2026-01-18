//! Test fixture utilities for plissken-core tests.
//!
//! This module provides helpers for accessing test fixtures in the workspace's
//! `tests/fixtures/` directory. It centralizes fixture path construction and
//! provides convenient accessors for common fixture files.
//!
//! # Example
//!
//! ```ignore
//! use crate::test_fixtures::{fixtures_root, fixture_path};
//!
//! let hybrid_binary = fixture_path("hybrid_binary/rust/lib.rs");
//! let pure_python = fixtures_root().join("pure_python/scheduler.py");
//! ```

use std::path::PathBuf;

/// Returns the path to the workspace's `tests/fixtures/` directory.
///
/// This navigates from the crate's manifest directory up to the workspace
/// root and into the fixtures directory.
pub fn fixtures_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crates/ directory")
        .parent()
        .expect("workspace root")
        .join("tests/fixtures")
}

/// Returns the full path to a fixture file or directory.
///
/// # Arguments
///
/// * `relative_path` - Path relative to the fixtures directory
///
/// # Example
///
/// ```ignore
/// let lib_rs = fixture_path("hybrid_binary/rust/lib.rs");
/// let scheduler = fixture_path("pure_python/scheduler.py");
/// ```
pub fn fixture_path(relative_path: &str) -> PathBuf {
    fixtures_root().join(relative_path)
}

/// Fixture paths for the `hybrid_binary` test project.
///
/// This fixture represents a Python/Rust hybrid project where Rust code
/// is compiled as a binary with Python bindings.
pub mod hybrid_binary {
    use super::*;

    /// Path to the Rust library source.
    pub fn rust_lib() -> PathBuf {
        fixture_path("hybrid_binary/rust/lib.rs")
    }

    /// Path to the Python package directory.
    pub fn python_dir() -> PathBuf {
        fixture_path("hybrid_binary/python")
    }

    /// Path to the Python helpers module.
    pub fn python_helpers() -> PathBuf {
        fixture_path("hybrid_binary/python/helpers.py")
    }

    /// Path to the fixture root directory.
    pub fn root() -> PathBuf {
        fixture_path("hybrid_binary")
    }
}

/// Fixture paths for the `separate_bindings` test project.
///
/// This fixture represents a project where Python bindings are in a
/// separate crate from the core Rust library.
pub mod separate_bindings {
    use super::*;

    /// Path to the bindings library source.
    pub fn bindings_lib() -> PathBuf {
        fixture_path("separate_bindings/bindings/src/lib.rs")
    }

    /// Path to the Python package directory.
    pub fn python_dir() -> PathBuf {
        fixture_path("separate_bindings/python")
    }

    /// Path to the fixture root directory.
    pub fn root() -> PathBuf {
        fixture_path("separate_bindings")
    }
}

/// Fixture paths for the `pure_python` test project.
///
/// This fixture represents a pure Python project without any Rust code.
pub mod pure_python {
    use super::*;

    /// Path to the scheduler module.
    pub fn scheduler() -> PathBuf {
        fixture_path("pure_python/scheduler.py")
    }

    /// Path to the task module.
    pub fn task() -> PathBuf {
        fixture_path("pure_python/task.py")
    }

    /// Path to the protocols module.
    pub fn protocols() -> PathBuf {
        fixture_path("pure_python/protocols.py")
    }

    /// Path to the fixture root directory.
    pub fn root() -> PathBuf {
        fixture_path("pure_python")
    }
}

/// Fixture paths for the `pure_rust` test project.
///
/// This fixture represents a pure Rust project without Python bindings.
pub mod pure_rust {
    use super::*;

    /// Path to the library source.
    pub fn lib() -> PathBuf {
        fixture_path("pure_rust/src/lib.rs")
    }

    /// Path to the fixture root directory.
    pub fn root() -> PathBuf {
        fixture_path("pure_rust")
    }
}

/// Fixture paths for the `complex_generics` test project.
///
/// This fixture contains complex generic type definitions for testing
/// generic type parsing and rendering.
pub mod complex_generics {
    use super::*;

    /// Path to the fixture root directory.
    pub fn root() -> PathBuf {
        fixture_path("complex_generics")
    }

    /// Path to the Python types module.
    pub fn python_types() -> PathBuf {
        fixture_path("complex_generics/python/types.py")
    }
}

/// Fixture paths for the `minimal_hybrid` test project.
///
/// This fixture is a minimal example of a Python/Rust hybrid project,
/// useful for quick smoke tests.
pub mod minimal_hybrid {
    use super::*;

    /// Path to the fixture root directory.
    pub fn root() -> PathBuf {
        fixture_path("minimal_hybrid")
    }

    /// Path to the Python package directory.
    pub fn python_dir() -> PathBuf {
        fixture_path("minimal_hybrid/python")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixtures_root_exists() {
        let root = fixtures_root();
        assert!(root.exists(), "fixtures root should exist at {:?}", root);
    }

    #[test]
    fn test_hybrid_binary_paths() {
        assert!(hybrid_binary::root().exists());
        assert!(hybrid_binary::rust_lib().exists());
        assert!(hybrid_binary::python_dir().exists());
    }

    #[test]
    fn test_pure_python_paths() {
        assert!(pure_python::root().exists());
        assert!(pure_python::scheduler().exists());
    }

    #[test]
    fn test_pure_rust_paths() {
        assert!(pure_rust::root().exists());
        assert!(pure_rust::lib().exists());
    }
}
