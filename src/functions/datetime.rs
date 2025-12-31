/// Date and time functions for templates
use chrono::Utc;
use minijinja::{Error, Value};

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
