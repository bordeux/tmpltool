//! Integration tests for the --ide CLI flag
//!
//! These tests verify the IDE metadata export functionality works correctly.

use assert_cmd::Command;
use predicates::prelude::*;

#[allow(deprecated)]
fn tmpltool() -> Command {
    Command::cargo_bin("tmpltool").unwrap()
}

#[test]
fn test_ide_json_outputs_valid_json() {
    let output = tmpltool()
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
    let output = tmpltool()
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
    let output = tmpltool()
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
    let output = tmpltool()
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
    tmpltool()
        .args(["--ide", "json", "nonexistent_template.tmpl"])
        .assert()
        .success();
}

#[test]
fn test_ide_invalid_format() {
    tmpltool()
        .args(["--ide", "invalid_format"])
        .assert()
        .failure()
        .stderr(
            predicate::str::contains("invalid").or(predicate::str::contains("possible values")),
        );
}

#[test]
fn test_ide_json_contains_expected_functions() {
    let output = tmpltool()
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
    let output = tmpltool()
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
