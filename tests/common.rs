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
