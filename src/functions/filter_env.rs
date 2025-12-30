/// Filter environment variables by pattern
///
/// This module provides a Tera function to filter environment variables
/// matching a glob pattern (e.g., "SERVER_*", "DB_*", etc.)
use std::collections::HashMap;
use std::env;
use tera::{Function, Result, Value, to_value};

/// A Tera function that filters environment variables by pattern
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
/// ```tera
/// {% for var in filter_env(pattern="SERVER_*") %}
///   {{ var.key }}={{ var.value }}
/// {% endfor %}
/// ```
pub struct FilterEnv;

impl Function for FilterEnv {
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value> {
        // Get the pattern argument
        let pattern = args
            .get("pattern")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                tera::Error::msg(
                    "filter_env requires a 'pattern' argument (e.g., pattern=\"SERVER_*\")",
                )
            })?;

        // Convert glob pattern to regex
        let regex_pattern = glob_to_regex(pattern);
        let re = regex::Regex::new(&regex_pattern)
            .map_err(|e| tera::Error::msg(format!("Invalid pattern: {}", e)))?;

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

        to_value(&results)
            .map_err(|e| tera::Error::msg(format!("Failed to convert results: {}", e)))
    }
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
