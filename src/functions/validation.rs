//! Validation functions for templates
//!
//! Note: The following validation functions have been migrated to `is_functions` module
//! and support both function and "is" test syntax:
//! - `is_email` / `{% if x is email %}`
//! - `is_url` / `{% if x is url %}`
//! - `is_ip` / `{% if x is ip %}`
//! - `is_uuid` / `{% if x is uuid %}`
//!
//! This module contains `matches_regex` which has a different pattern (takes 2 args).

use super::metadata::{ArgumentMetadata, FunctionMetadata, SyntaxVariants};
use super::traits::Function;
use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};
use regex::Regex;

/// Check if string matches regex pattern
pub struct MatchesRegex;

impl Function for MatchesRegex {
    const NAME: &'static str = "matches_regex";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "matches_regex",
        category: "validation",
        description: "Check if string matches a regular expression pattern",
        arguments: &[
            ArgumentMetadata {
                name: "pattern",
                arg_type: "string",
                required: true,
                default: None,
                description: "Regular expression pattern",
            },
            ArgumentMetadata {
                name: "string",
                arg_type: "string",
                required: true,
                default: None,
                description: "String to test against the pattern",
            },
        ],
        return_type: "boolean",
        examples: &[
            "{{ matches_regex(pattern=\"^[A-Z]+$\", string=\"HELLO\") }}",
            "{% if matches_regex(pattern=\"\\\\d+\", string=value) %}has digits{% endif %}",
        ],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let pattern: String = kwargs.get("pattern")?;
        let string: String = kwargs.get("string")?;

        let re = Regex::new(&pattern).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Invalid regex pattern '{}': {}", pattern, e),
            )
        })?;

        Ok(Value::from(re.is_match(&string)))
    }
}

// Legacy function export for backward compatibility during migration
