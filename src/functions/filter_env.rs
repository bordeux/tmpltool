/// Filter environment variables by pattern
///
/// This module provides a Tera function to filter environment variables
/// matching a glob pattern (e.g., "SERVER_*", "DB_*", etc.)

use std::collections::HashMap;
use std::env;
use tera::{to_value, Function, Result, Value};

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
                tera::Error::msg("filter_env requires a 'pattern' argument (e.g., pattern=\"SERVER_*\")")
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

        to_value(&results).map_err(|e| tera::Error::msg(format!("Failed to convert results: {}", e)))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glob_to_regex() {
        assert_eq!(glob_to_regex("SERVER_*"), "^SERVER_.*$");
        assert_eq!(glob_to_regex("DB_?"), "^DB_.$");
        assert_eq!(glob_to_regex("*_PORT"), "^.*_PORT$");
        assert_eq!(glob_to_regex("APP_*_NAME"), "^APP_.*_NAME$");
        assert_eq!(glob_to_regex("test.foo"), "^test\\.foo$");
    }

    #[test]
    fn test_filter_env_basic() {
        // Set test environment variables
        unsafe {
            env::set_var("TEST_VAR_1", "value1");
            env::set_var("TEST_VAR_2", "value2");
            env::set_var("OTHER_VAR", "other");
        }

        let filter = FilterEnv;
        let mut args = HashMap::new();
        args.insert("pattern".to_string(), to_value("TEST_VAR_*").unwrap());

        let result = filter.call(&args);
        assert!(result.is_ok());

        let result_value = result.unwrap();
        assert!(result_value.is_array());

        let array = result_value.as_array().unwrap();
        assert_eq!(array.len(), 2);

        // Cleanup
        unsafe {
            env::remove_var("TEST_VAR_1");
            env::remove_var("TEST_VAR_2");
            env::remove_var("OTHER_VAR");
        }
    }

    #[test]
    fn test_filter_env_wildcard_middle() {
        unsafe {
            env::set_var("APP_1_NAME", "app1");
            env::set_var("APP_2_NAME", "app2");
            env::set_var("APP_NAME", "app");
        }

        let filter = FilterEnv;
        let mut args = HashMap::new();
        args.insert("pattern".to_string(), to_value("APP_*_NAME").unwrap());

        let result = filter.call(&args);
        assert!(result.is_ok());

        let result_value = result.unwrap();
        let array = result_value.as_array().unwrap();
        // Should match APP_1_NAME and APP_2_NAME, but not APP_NAME
        assert_eq!(array.len(), 2);

        unsafe {
            env::remove_var("APP_1_NAME");
            env::remove_var("APP_2_NAME");
            env::remove_var("APP_NAME");
        }
    }

    #[test]
    fn test_filter_env_question_mark() {
        unsafe {
            env::set_var("DB_A", "database_a");
            env::set_var("DB_B", "database_b");
            env::set_var("DB_AB", "database_ab");
        }

        let filter = FilterEnv;
        let mut args = HashMap::new();
        args.insert("pattern".to_string(), to_value("DB_?").unwrap());

        let result = filter.call(&args);
        assert!(result.is_ok());

        let result_value = result.unwrap();
        let array = result_value.as_array().unwrap();
        // Should match DB_A and DB_B (single char), but not DB_AB (two chars)
        assert_eq!(array.len(), 2);

        unsafe {
            env::remove_var("DB_A");
            env::remove_var("DB_B");
            env::remove_var("DB_AB");
        }
    }

    #[test]
    fn test_filter_env_no_pattern() {
        let filter = FilterEnv;
        let args = HashMap::new();

        let result = filter.call(&args);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("requires a 'pattern' argument"));
    }

    #[test]
    fn test_filter_env_no_matches() {
        let filter = FilterEnv;
        let mut args = HashMap::new();
        args.insert(
            "pattern".to_string(),
            to_value("NONEXISTENT_PATTERN_*").unwrap(),
        );

        let result = filter.call(&args);
        assert!(result.is_ok());

        let result_value = result.unwrap();
        let array = result_value.as_array().unwrap();
        assert_eq!(array.len(), 0);
    }
}
