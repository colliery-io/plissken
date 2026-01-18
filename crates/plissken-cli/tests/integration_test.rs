//! Integration tests for the plissken CLI

use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Get the path to the plissken binary
fn plissken_bin() -> PathBuf {
    // The binary is built in the workspace target directory
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop(); // crates/
    path.pop(); // workspace root
    path.push("target");
    path.push("debug");
    path.push("plissken");
    path
}

/// Get path to test fixtures
fn fixtures_path() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop(); // crates/
    path.pop(); // workspace root
    path.push("tests");
    path.push("fixtures");
    path
}

#[test]
fn test_render_hybrid_binary_fixture() {
    let bin = plissken_bin();
    let fixtures = fixtures_path();
    let hybrid_binary = fixtures.join("hybrid_binary");

    // Create a temp directory for output
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let output_dir = temp_dir.path();

    // Run the render command
    let output = Command::new(&bin)
        .args([
            "render",
            &hybrid_binary.to_string_lossy(),
            "-o",
            &output_dir.to_string_lossy(),
        ])
        .output()
        .expect("Failed to execute plissken render");

    // Check command succeeded
    assert!(
        output.status.success(),
        "Render command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Verify Python module file exists (inline format: one file per module)
    let python_helpers = output_dir.join("helpers.md");
    assert!(
        python_helpers.exists(),
        "Python module file should exist at {:?}",
        python_helpers
    );

    // Verify Rust crate root file exists
    let rust_crate = output_dir.join("rust").join("hybrid_binary.md");
    assert!(
        rust_crate.exists(),
        "Rust crate root should exist at {:?}",
        rust_crate
    );

    // Verify submodule docs exist
    let internal_mod = output_dir
        .join("rust")
        .join("hybrid_binary")
        .join("internal.md");
    assert!(
        internal_mod.exists(),
        "Rust submodule should exist at {:?}",
        internal_mod
    );

    // Check content of Python module (inline format includes all classes/functions)
    let python_content = fs::read_to_string(&python_helpers).expect("Failed to read Python module");
    assert!(
        python_content.contains("# helpers"),
        "Should have module header"
    );
    assert!(
        python_content.contains("TaskBuilder"),
        "Should contain TaskBuilder class inline"
    );

    // Check content of Rust crate root
    let rust_content = fs::read_to_string(&rust_crate).expect("Failed to read Rust crate");
    assert!(
        rust_content.contains("hybrid_binary") && rust_content.starts_with("#"),
        "Should have module header with hybrid_binary"
    );
    // Rust crate root should contain struct documentation inline
    assert!(
        rust_content.contains("Task") || rust_content.contains("PyTask"),
        "Should contain Task/PyTask struct inline"
    );
}

#[test]
fn test_render_with_template_override() {
    let bin = plissken_bin();
    let fixtures = fixtures_path();
    let hybrid_binary = fixtures.join("hybrid_binary");

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let output_dir = temp_dir.path();

    // Run with mdbook template override
    let output = Command::new(&bin)
        .args([
            "render",
            &hybrid_binary.to_string_lossy(),
            "-o",
            &output_dir.to_string_lossy(),
            "-t",
            "mdbook",
        ])
        .output()
        .expect("Failed to execute plissken render");

    assert!(
        output.status.success(),
        "Render command with template override failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Check that files were created (inline format: one file per module)
    // mdBook puts content in src/ subdirectory
    let python_helpers = output_dir.join("src").join("helpers.md");
    assert!(
        python_helpers.exists(),
        "Python module file should exist at {:?}",
        python_helpers
    );

    // mdbook uses different CSS variables - check for mdbook-specific styling
    let content = fs::read_to_string(&python_helpers).expect("Failed to read doc");
    // mdbook uses var(--*) while mkdocs uses var(--md-*)
    assert!(
        content.contains("var(--"),
        "Should contain CSS variable references"
    );
}

#[test]
fn test_generate_json_output() {
    let bin = plissken_bin();
    let fixtures = fixtures_path();
    let hybrid_binary = fixtures.join("hybrid_binary");

    // Run generate command
    let output = Command::new(&bin)
        .args(["generate", &hybrid_binary.to_string_lossy()])
        .output()
        .expect("Failed to execute plissken generate");

    assert!(
        output.status.success(),
        "Generate command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Parse the JSON output
    let json_str = String::from_utf8_lossy(&output.stdout);
    let doc_model: serde_json::Value =
        serde_json::from_str(&json_str).expect("Failed to parse JSON output");

    // Verify structure
    assert!(doc_model.get("metadata").is_some(), "Should have metadata");
    assert!(
        doc_model.get("rust_modules").is_some(),
        "Should have rust_modules"
    );
    assert!(
        doc_model.get("python_modules").is_some(),
        "Should have python_modules"
    );

    // Check metadata
    let metadata = doc_model.get("metadata").unwrap();
    assert_eq!(
        metadata.get("name").unwrap().as_str(),
        Some("hybrid_binary")
    );
}

#[test]
fn test_render_creates_directory_structure() {
    let bin = plissken_bin();
    let fixtures = fixtures_path();
    let hybrid_binary = fixtures.join("hybrid_binary");

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let output_dir = temp_dir.path().join("nested").join("output").join("docs");

    // Run render to a nested directory that doesn't exist
    let output = Command::new(&bin)
        .args([
            "render",
            &hybrid_binary.to_string_lossy(),
            "-o",
            &output_dir.to_string_lossy(),
        ])
        .output()
        .expect("Failed to execute plissken render");

    assert!(
        output.status.success(),
        "Render command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Directory structure should be created
    assert!(output_dir.exists(), "Output directory should be created");
    // Rust submodules live under rust/ directory
    assert!(
        output_dir.join("rust").exists(),
        "Rust subdirectory should be created"
    );
    // Python modules are at top level (helpers.md) in inline format
    // (no python/ directory needed for simple modules)
}
