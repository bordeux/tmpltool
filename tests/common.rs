/// Common test utilities for tmpltool integration tests

use std::fs;
use std::path::PathBuf;

/// Get a unique temporary file path for testing
pub fn get_test_file_path(name: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!("tmpltool_integration_test_{}", name));
    path
}

/// Clean up a test file (remove it if it exists)
pub fn cleanup_test_file(path: &PathBuf) {
    let _ = fs::remove_file(path);
}

/// Get the path to a template fixture file
pub fn get_fixture_template(name: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("fixtures");
    path.push("templates");
    path.push(name);
    path
}

/// Get the path to an expected output fixture file
pub fn get_fixture_expected(name: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("fixtures");
    path.push("expected");
    path.push(name);
    path
}

/// Read a fixture template file
pub fn read_fixture_template(name: &str) -> String {
    let path = get_fixture_template(name);
    fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to read fixture template '{}': {}", path.display(), e))
}

/// Read an expected output fixture file
pub fn read_fixture_expected(name: &str) -> String {
    let path = get_fixture_expected(name);
    fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to read fixture expected '{}': {}", path.display(), e))
}
