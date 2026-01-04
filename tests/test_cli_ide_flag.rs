//! Integration tests for the --ide CLI flag
//!
//! These tests verify the IDE metadata export functionality works correctly.

use std::process::Command;

fn get_binary_path() -> String {
    // Try release binary first, then debug
    let release_path = "target/release/tmpltool";
    let debug_path = "target/debug/tmpltool";

    if std::path::Path::new(release_path).exists() {
        release_path.to_string()
    } else if std::path::Path::new(debug_path).exists() {
        debug_path.to_string()
    } else {
        panic!("Binary not found. Please run 'cargo build' first.");
    }
}

#[test]
fn test_ide_json_outputs_valid_json() {
    let output = Command::new(get_binary_path())
        .args(["--ide", "json"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "Command should succeed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: Result<serde_json::Value, _> = serde_json::from_str(&stdout);

    assert!(parsed.is_ok(), "Output should be valid JSON: {}", stdout);

    let json = parsed.unwrap();
    assert!(json.is_array(), "JSON output should be an array");
    assert!(
        json.as_array().unwrap().len() > 100,
        "Should have many functions"
    );
}

#[test]
fn test_ide_yaml_outputs_valid_yaml() {
    let output = Command::new(get_binary_path())
        .args(["--ide", "yaml"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "Command should succeed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: Result<serde_yaml::Value, _> = serde_yaml::from_str(&stdout);

    assert!(parsed.is_ok(), "Output should be valid YAML: {}", stdout);

    let yaml = parsed.unwrap();
    assert!(
        yaml.as_sequence().is_some(),
        "YAML output should be a sequence"
    );
}

#[test]
fn test_ide_toml_outputs_valid_toml() {
    let output = Command::new(get_binary_path())
        .args(["--ide", "toml"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "Command should succeed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: Result<toml::Value, _> = toml::from_str(&stdout);

    assert!(parsed.is_ok(), "Output should be valid TOML: {}", stdout);

    let toml_val = parsed.unwrap();
    assert!(
        toml_val.get("functions").is_some(),
        "TOML should have functions key"
    );
}

#[test]
fn test_ide_json_structure() {
    let output = Command::new(get_binary_path())
        .args(["--ide", "json"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();

    // Check first function has required fields
    let first = &json.as_array().unwrap()[0];

    assert!(first.get("name").is_some(), "Should have name field");
    assert!(
        first.get("category").is_some(),
        "Should have category field"
    );
    assert!(
        first.get("description").is_some(),
        "Should have description field"
    );
    assert!(
        first.get("arguments").is_some(),
        "Should have arguments field"
    );
    assert!(
        first.get("return_type").is_some(),
        "Should have return_type field"
    );
    assert!(
        first.get("examples").is_some(),
        "Should have examples field"
    );
    assert!(first.get("syntax").is_some(), "Should have syntax field");

    // Check syntax structure
    let syntax = first.get("syntax").unwrap();
    assert!(
        syntax.get("function").is_some(),
        "Syntax should have function field"
    );
    assert!(
        syntax.get("filter").is_some(),
        "Syntax should have filter field"
    );
    assert!(
        syntax.get("is_test").is_some(),
        "Syntax should have is_test field"
    );
}

#[test]
fn test_ide_exits_without_rendering() {
    // --ide should exit before trying to render a template
    let output = Command::new(get_binary_path())
        .args(["--ide", "json", "nonexistent_template.tmpl"])
        .output()
        .expect("Failed to execute command");

    // Should succeed because --ide exits early
    assert!(
        output.status.success(),
        "Command should succeed even with nonexistent template"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: Result<serde_json::Value, _> = serde_json::from_str(&stdout);
    assert!(parsed.is_ok(), "Should still output valid JSON");
}

#[test]
fn test_ide_invalid_format() {
    let output = Command::new(get_binary_path())
        .args(["--ide", "invalid_format"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success(), "Command should fail");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("invalid") || stderr.contains("possible values"),
        "Error should indicate invalid format"
    );
}

#[test]
fn test_ide_json_contains_expected_functions() {
    let output = Command::new(get_binary_path())
        .args(["--ide", "json"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();

    let names: Vec<&str> = json
        .as_array()
        .unwrap()
        .iter()
        .map(|f| f.get("name").unwrap().as_str().unwrap())
        .collect();

    let expected = ["get_env", "md5", "sha256", "now", "uuid", "is_email"];

    for expected_name in expected {
        assert!(
            names.contains(&expected_name),
            "Expected function '{}' in output",
            expected_name
        );
    }
}

#[test]
fn test_ide_json_categories_present() {
    let output = Command::new(get_binary_path())
        .args(["--ide", "json"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();

    let categories: std::collections::HashSet<&str> = json
        .as_array()
        .unwrap()
        .iter()
        .map(|f| f.get("category").unwrap().as_str().unwrap())
        .collect();

    let expected = ["hash", "encoding", "string", "datetime", "filesystem"];

    for cat in expected {
        assert!(
            categories.contains(cat),
            "Expected category '{}' in output. Found: {:?}",
            cat,
            categories
        );
    }
}
