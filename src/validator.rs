//! Output format validation
//!
//! This module provides validation for rendered template output to ensure
//! it conforms to the expected format (JSON, YAML, or TOML).

use crate::cli::ValidateFormat;

/// Validate output string against the specified format
///
/// # Arguments
///
/// * `output` - The rendered template output to validate
/// * `format` - The expected format (JSON, YAML, or TOML)
///
/// # Returns
///
/// Returns `Ok(())` if validation succeeds, or an error message if validation fails
///
/// # Example
///
/// ```
/// use tmpltool::validator::validate_output;
/// use tmpltool::cli::ValidateFormat;
///
/// let json_output = r#"{"name": "test", "value": 42}"#;
/// assert!(validate_output(json_output, ValidateFormat::Json).is_ok());
///
/// let invalid_json = r#"{"name": "test", "value": }"#;
/// assert!(validate_output(invalid_json, ValidateFormat::Json).is_err());
/// ```
pub fn validate_output(output: &str, format: ValidateFormat) -> Result<(), String> {
    match format {
        ValidateFormat::Json => validate_json(output),
        ValidateFormat::Yaml => validate_yaml(output),
        ValidateFormat::Toml => validate_toml(output),
    }
}

/// Validate JSON format
fn validate_json(output: &str) -> Result<(), String> {
    serde_json::from_str::<serde_json::Value>(output).map_err(|e| {
        format!(
            "JSON validation failed: {}\n\nThis usually means:\n\
             - Missing or extra commas\n\
             - Unclosed brackets or braces\n\
             - Invalid escape sequences\n\
             - Trailing commas (not allowed in JSON)\n\
             - Unquoted keys or values",
            e
        )
    })?;
    Ok(())
}

/// Validate YAML format
fn validate_yaml(output: &str) -> Result<(), String> {
    serde_yaml::from_str::<serde_yaml::Value>(output).map_err(|e| {
        format!(
            "YAML validation failed: {}\n\nThis usually means:\n\
             - Incorrect indentation (use spaces, not tabs)\n\
             - Missing or misplaced colons\n\
             - Invalid list syntax (- item)\n\
             - Unclosed quotes\n\
             - Invalid escape sequences",
            e
        )
    })?;
    Ok(())
}

/// Validate TOML format
fn validate_toml(output: &str) -> Result<(), String> {
    toml::from_str::<toml::Value>(output).map_err(|e| {
        format!(
            "TOML validation failed: {}\n\nThis usually means:\n\
             - Invalid section headers [section]\n\
             - Duplicate keys\n\
             - Invalid value types in arrays\n\
             - Missing quotes around strings\n\
             - Invalid datetime format\n\
             - Incorrect table array syntax [[array]]",
            e
        )
    })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_json_valid() {
        let valid_json = r#"{"name": "test", "value": 42, "active": true}"#;
        assert!(validate_json(valid_json).is_ok());
    }

    #[test]
    fn test_validate_json_valid_array() {
        let valid_json = r#"[1, 2, 3, 4, 5]"#;
        assert!(validate_json(valid_json).is_ok());
    }

    #[test]
    fn test_validate_json_valid_nested() {
        let valid_json = r#"{"server": {"host": "localhost", "port": 8080}}"#;
        assert!(validate_json(valid_json).is_ok());
    }

    #[test]
    fn test_validate_json_invalid_trailing_comma() {
        let invalid_json = r#"{"name": "test",}"#;
        assert!(validate_json(invalid_json).is_err());
    }

    #[test]
    fn test_validate_json_invalid_syntax() {
        let invalid_json = r#"{"name": "test", "value": }"#;
        let result = validate_json(invalid_json);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("JSON validation failed"));
    }

    #[test]
    fn test_validate_json_invalid_unclosed_brace() {
        let invalid_json = r#"{"name": "test""#;
        assert!(validate_json(invalid_json).is_err());
    }

    #[test]
    fn test_validate_yaml_valid() {
        let valid_yaml = "name: test\nvalue: 42\nactive: true";
        assert!(validate_yaml(valid_yaml).is_ok());
    }

    #[test]
    fn test_validate_yaml_valid_array() {
        let valid_yaml = "- apple\n- banana\n- cherry";
        assert!(validate_yaml(valid_yaml).is_ok());
    }

    #[test]
    fn test_validate_yaml_valid_nested() {
        let valid_yaml = "server:\n  host: localhost\n  port: 8080";
        assert!(validate_yaml(valid_yaml).is_ok());
    }

    #[test]
    fn test_validate_yaml_invalid_syntax() {
        let invalid_yaml = "name: test\nvalue: : invalid";
        let result = validate_yaml(invalid_yaml);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("YAML validation failed"));
    }

    #[test]
    fn test_validate_yaml_empty() {
        // Empty YAML is valid (represents null)
        let empty_yaml = "";
        assert!(validate_yaml(empty_yaml).is_ok());
    }

    #[test]
    fn test_validate_toml_valid() {
        let valid_toml = r#"title = "Test"
version = "1.0.0"
"#;
        assert!(validate_toml(valid_toml).is_ok());
    }

    #[test]
    fn test_validate_toml_valid_section() {
        let valid_toml = r#"[package]
name = "myapp"
version = "1.0.0"
"#;
        assert!(validate_toml(valid_toml).is_ok());
    }

    #[test]
    fn test_validate_toml_valid_nested() {
        let valid_toml = r#"[server]
host = "localhost"
port = 8080
"#;
        assert!(validate_toml(valid_toml).is_ok());
    }

    #[test]
    fn test_validate_toml_invalid_syntax() {
        let invalid_toml = "name = test without quotes";
        let result = validate_toml(invalid_toml);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("TOML validation failed"));
    }

    #[test]
    fn test_validate_toml_invalid_duplicate_key() {
        let invalid_toml = r#"name = "test"
name = "duplicate"
"#;
        assert!(validate_toml(invalid_toml).is_err());
    }

    #[test]
    fn test_validate_toml_empty() {
        // Empty TOML is valid (represents empty table)
        let empty_toml = "";
        assert!(validate_toml(empty_toml).is_ok());
    }

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
}
