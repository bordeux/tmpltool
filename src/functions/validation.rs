//! Validation functions for templates
//!
//! Note: The following validation functions have been migrated to `is_functions` module
//! and support both function and "is" test syntax:
//! - `is_email` / `{% if x is email %}`
//! - `is_url` / `{% if x is url %}`
//! - `is_ip` / `{% if x is ip %}`
//! - `is_uuid` / `{% if x is uuid %}`
//!
//! This module now only contains `matches_regex` which has a different pattern.

use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};
use regex::Regex;

/// Check if string matches regex pattern
///
/// # Example
///
/// ```jinja
/// {{ matches_regex(pattern="^[A-Z]+$", string="HELLO") }}
/// ```
pub fn matches_regex_fn(kwargs: Kwargs) -> Result<Value, Error> {
    // Extract parameters from kwargs
    let pattern: String = kwargs.get("pattern")?;
    let string: String = kwargs.get("string")?;

    // Compile and match regex
    let re = Regex::new(&pattern).map_err(|e| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("Invalid regex pattern '{}': {}", pattern, e),
        )
    })?;

    Ok(Value::from(re.is_match(&string)))
}
