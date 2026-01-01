use tmpltool::cli::ValidateFormat;
use tmpltool::validator::validate_output;

// ============================================================================
// JSON Validation Tests
// ============================================================================

#[test]
fn test_validate_json_valid() {
    let valid_json = r#"{"name": "test", "value": 42, "active": true}"#;
    assert!(validate_output(valid_json, ValidateFormat::Json).is_ok());
}

#[test]
fn test_validate_json_valid_array() {
    let valid_json = r#"[1, 2, 3, 4, 5]"#;
    assert!(validate_output(valid_json, ValidateFormat::Json).is_ok());
}

#[test]
fn test_validate_json_valid_nested() {
    let valid_json = r#"{"server": {"host": "localhost", "port": 8080}}"#;
    assert!(validate_output(valid_json, ValidateFormat::Json).is_ok());
}

#[test]
fn test_validate_json_invalid_trailing_comma() {
    let invalid_json = r#"{"name": "test",}"#;
    assert!(validate_output(invalid_json, ValidateFormat::Json).is_err());
}

#[test]
fn test_validate_json_invalid_syntax() {
    let invalid_json = r#"{"name": "test", "value": }"#;
    let result = validate_output(invalid_json, ValidateFormat::Json);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("JSON validation failed"));
}

#[test]
fn test_validate_json_invalid_unclosed_brace() {
    let invalid_json = r#"{"name": "test""#;
    assert!(validate_output(invalid_json, ValidateFormat::Json).is_err());
}

// ============================================================================
// YAML Validation Tests
// ============================================================================

#[test]
fn test_validate_yaml_valid() {
    let valid_yaml = "name: test\nvalue: 42\nactive: true";
    assert!(validate_output(valid_yaml, ValidateFormat::Yaml).is_ok());
}

#[test]
fn test_validate_yaml_valid_array() {
    let valid_yaml = "- apple\n- banana\n- cherry";
    assert!(validate_output(valid_yaml, ValidateFormat::Yaml).is_ok());
}

#[test]
fn test_validate_yaml_valid_nested() {
    let valid_yaml = "server:\n  host: localhost\n  port: 8080";
    assert!(validate_output(valid_yaml, ValidateFormat::Yaml).is_ok());
}

#[test]
fn test_validate_yaml_invalid_syntax() {
    let invalid_yaml = "name: test\nvalue: : invalid";
    let result = validate_output(invalid_yaml, ValidateFormat::Yaml);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("YAML validation failed"));
}

#[test]
fn test_validate_yaml_empty() {
    // Empty YAML is valid (represents null)
    let empty_yaml = "";
    assert!(validate_output(empty_yaml, ValidateFormat::Yaml).is_ok());
}

// ============================================================================
// TOML Validation Tests
// ============================================================================

#[test]
fn test_validate_toml_valid() {
    let valid_toml = r#"title = "Test"
version = "1.0.0"
"#;
    assert!(validate_output(valid_toml, ValidateFormat::Toml).is_ok());
}

#[test]
fn test_validate_toml_valid_section() {
    let valid_toml = r#"[package]
name = "myapp"
version = "1.0.0"
"#;
    assert!(validate_output(valid_toml, ValidateFormat::Toml).is_ok());
}

#[test]
fn test_validate_toml_valid_nested() {
    let valid_toml = r#"[server]
host = "localhost"
port = 8080
"#;
    assert!(validate_output(valid_toml, ValidateFormat::Toml).is_ok());
}

#[test]
fn test_validate_toml_invalid_syntax() {
    let invalid_toml = "name = test without quotes";
    let result = validate_output(invalid_toml, ValidateFormat::Toml);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("TOML validation failed"));
}

#[test]
fn test_validate_toml_invalid_duplicate_key() {
    let invalid_toml = r#"name = "test"
name = "duplicate"
"#;
    assert!(validate_output(invalid_toml, ValidateFormat::Toml).is_err());
}

#[test]
fn test_validate_toml_empty() {
    // Empty TOML is valid (represents empty table)
    let empty_toml = "";
    assert!(validate_output(empty_toml, ValidateFormat::Toml).is_ok());
}

// ============================================================================
// validate_output Function Tests
// ============================================================================

#[test]
fn test_validate_output_json() {
    let json = r#"{"test": true}"#;
    assert!(validate_output(json, ValidateFormat::Json).is_ok());
}

#[test]
fn test_validate_output_yaml() {
    let yaml = "test: true";
    assert!(validate_output(yaml, ValidateFormat::Yaml).is_ok());
}

#[test]
fn test_validate_output_toml() {
    let toml = r#"test = true"#;
    assert!(validate_output(toml, ValidateFormat::Toml).is_ok());
}
