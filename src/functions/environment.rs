/// Environment variable access functions for templates
use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};
use std::collections::HashMap;

/// Get environment variable with optional default
///
/// Replacement for Tera's built-in get_env() function
///
/// # Arguments
///
/// * `name` - Environment variable name
/// * `default` - Optional default value if variable is not set
///
/// # Example
///
/// ```jinja
/// {{ get_env(name="HOME") }}
/// {{ get_env(name="MISSING", default="/tmp") }}
/// ```
pub fn env_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let name: String = kwargs.get("name")?;
    let default: Option<String> = kwargs.get("default").ok();

    match std::env::var(&name) {
        Ok(value) => Ok(Value::from(value)),
        Err(_) => {
            if let Some(def) = default {
                Ok(Value::from(def))
            } else {
                Err(Error::new(
                    ErrorKind::UndefinedError,
                    format!(
                        "Environment variable '{}' is not set and no default provided",
                        name
                    ),
                ))
            }
        }
    }
}

/// Filter environment variables by pattern
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
    let mut results: Vec<HashMap<String, String>> = std::env::vars()
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
