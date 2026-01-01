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
