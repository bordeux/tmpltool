//! Built-in functions that were provided by Tera
//!
//! These functions replicate the built-in functions that were automatically
//! available in Tera when the "builtins" feature was enabled. MiniJinja
//! requires these to be registered as custom functions.

use chrono::Utc;
use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};
use rand::Rng;

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

/// Get current timestamp in ISO 8601 format
///
/// Replacement for Tera's built-in now() function
///
/// Returns timestamp in ISO 8601 format: YYYY-MM-DDTHH:MM:SS.sss+00:00
///
/// # Example
///
/// ```jinja
/// {{ now() }}  => "2024-12-31T12:34:56.789+00:00"
/// ```
pub fn now_fn() -> Result<Value, Error> {
    let timestamp = Utc::now().to_rfc3339();
    Ok(Value::from(timestamp))
}

/// Generate random number in range
///
/// Replacement for Tera's built-in get_random() function
///
/// # Arguments
///
/// * `start` - Start of range (inclusive), defaults to 0
/// * `end` - End of range (exclusive), defaults to 100
///
/// # Example
///
/// ```jinja
/// {{ get_random() }}
/// {{ get_random(start=1, end=10) }}
/// ```
pub fn get_random_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let start: i64 = kwargs.get("start").unwrap_or(0);
    let end: i64 = kwargs.get("end").unwrap_or(100);

    if start >= end {
        return Err(Error::new(
            ErrorKind::InvalidOperation,
            format!("start ({}) must be less than end ({})", start, end),
        ));
    }

    let mut rng = rand::rng();
    let random = rng.random_range(start..end);

    Ok(Value::from(random))
}
