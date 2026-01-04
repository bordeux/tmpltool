//! Environment variable access functions for templates
//!
//! This module provides functions for accessing and filtering environment variables:
//! - `get_env`: Get an environment variable with optional default value
//! - `filter_env`: Filter environment variables by glob pattern

use super::metadata::{ArgumentMetadata, FunctionMetadata, SyntaxVariants};
use super::traits::Function;
use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};
use std::collections::HashMap;

/// Get environment variable with optional default
pub struct GetEnv;

impl Function for GetEnv {
    const NAME: &'static str = "get_env";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "get_env",
        category: "environment",
        description: "Get environment variable with optional default value",
        arguments: &[
            ArgumentMetadata {
                name: "name",
                arg_type: "string",
                required: true,
                default: None,
                description: "Environment variable name",
            },
            ArgumentMetadata {
                name: "default",
                arg_type: "string",
                required: false,
                default: None,
                description: "Default value if variable is not set",
            },
        ],
        return_type: "string",
        examples: &[
            "{{ get_env(name=\"HOME\") }}",
            "{{ get_env(name=\"PORT\", default=\"8080\") }}",
        ],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
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
}

/// Filter environment variables by glob pattern
pub struct FilterEnv;

impl Function for FilterEnv {
    const NAME: &'static str = "filter_env";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "filter_env",
        category: "environment",
        description: "Filter environment variables by glob pattern",
        arguments: &[ArgumentMetadata {
            name: "pattern",
            arg_type: "string",
            required: true,
            default: None,
            description: "Glob pattern to match variable names (e.g., \"SERVER_*\", \"DB_*\")",
        }],
        return_type: "array",
        examples: &[
            "{% for var in filter_env(pattern=\"SERVER_*\") %}{{ var.key }}={{ var.value }}{% endfor %}",
            "{{ filter_env(pattern=\"DB_*\") }}",
        ],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
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
}

/// Convert a glob pattern to a regex pattern
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
