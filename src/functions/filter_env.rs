use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};
/// Filter environment variables by pattern
///
/// This module provides a MiniJinja function to filter environment variables
/// matching a glob pattern (e.g., "SERVER_*", "DB_*", etc.)
use std::collections::HashMap;
use std::env;

/// A MiniJinja function that filters environment variables by pattern
///
/// Returns a list of objects with `key` and `value` fields for all
/// environment variables matching the given glob pattern.
///
/// # Arguments
///
/// * `pattern` - A glob pattern to match environment variable names
///   - Use `*` to match any characters
///   - Use `?` to match a single character
///   - Examples: "SERVER_*", "DB_*", "*_PORT", "APP_?_NAME"
///
/// # Returns
///
/// A list of objects, each containing:
/// * `key` - The environment variable name
/// * `value` - The environment variable value
///
/// # Examples
///
/// ```jinja
/// {% for var in filter_env(pattern="SERVER_*") %}
///   {{ var.key }}={{ var.value }}
/// {% endfor %}
/// ```
pub fn filter_env_fn(kwargs: Kwargs) -> Result<Value, Error> {
    // Extract pattern from kwargs
    let pattern: String = kwargs.get("pattern")?;

    // Convert glob pattern to regex
    let regex_pattern = glob_to_regex(&pattern);
    let re = regex::Regex::new(&regex_pattern).map_err(|e| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("Invalid pattern: {}", e),
        )
    })?;

    // Filter environment variables
    let mut results: Vec<HashMap<String, String>> = env::vars()
        .filter(|(key, _)| re.is_match(key))
        .map(|(key, value)| {
            let mut map = HashMap::new();
            map.insert("key".to_string(), key);
            map.insert("value".to_string(), value);
            map
        })
        .collect();

    // Sort by key for consistent output
    results.sort_by(|a, b| a.get("key").cmp(&b.get("key")));

    Ok(Value::from_serialize(&results))
}

/// Convert a glob pattern to a regex pattern
///
/// Supports:
/// * `*` - matches any characters (including none)
/// * `?` - matches exactly one character
/// * All other characters are escaped for literal matching
fn glob_to_regex(pattern: &str) -> String {
    let mut regex = String::from("^");

    for ch in pattern.chars() {
        match ch {
            '*' => regex.push_str(".*"),
            '?' => regex.push('.'),
            // Escape regex special characters
            '.' | '+' | '^' | '$' | '(' | ')' | '[' | ']' | '{' | '}' | '|' | '\\' => {
                regex.push('\\');
                regex.push(ch);
            }
            _ => regex.push(ch),
        }
    }

    regex.push('$');
    regex
}
