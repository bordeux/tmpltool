//! Debugging and development functions for MiniJinja templates
//!
//! This module provides functions for:
//! - Debugging values (debug, inspect, type_of)
//! - Assertions and validation (assert, warn, abort)

use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};

/// Print value to stderr and return it (for debugging)
///
/// # Arguments
///
/// * `value` (required) - Value to debug print
///
/// # Returns
///
/// Returns the same value that was passed in (for chaining)
///
/// # Example
///
/// ```jinja
/// {# Debug a variable and continue using it #}
/// {% set config = debug(value=parse_json(string='{"port": 8080}')) %}
/// Port: {{ config.port }}
///
/// {# Debug in a pipeline #}
/// Result: {{ get_env(name="PATH") | debug }}
/// ```
pub fn debug_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let value: Value = kwargs.get("value")?;

    // Print to stderr for debugging
    eprintln!("[DEBUG] {}", value);

    // Return the value unchanged
    Ok(value)
}

/// Get the type of a value
///
/// # Arguments
///
/// * `value` (required) - Value to check type of
///
/// # Returns
///
/// Returns a string describing the value type:
/// - "undefined" - undefined/none value
/// - "bool" - boolean
/// - "number" - integer or float
/// - "string" - string
/// - "array" - sequence/list
/// - "object" - map/object
///
/// # Example
///
/// ```jinja
/// {{ type_of(value="hello") }}  {# Output: string #}
/// {{ type_of(value=123) }}  {# Output: number #}
/// {{ type_of(value=[1, 2, 3]) }}  {# Output: array #}
///
/// {# Conditional logic based on type #}
/// {% set data = get_env(name="DATA", default="[]") %}
/// {% if type_of(value=data) == "string" %}
///   {# Parse it #}
///   {% set data = parse_json(string=data) %}
/// {% endif %}
/// ```
pub fn type_of_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let value: Value = kwargs.get("value")?;

    let type_name = match value.kind() {
        minijinja::value::ValueKind::Undefined => "undefined",
        minijinja::value::ValueKind::None => "undefined",
        minijinja::value::ValueKind::Bool => "bool",
        minijinja::value::ValueKind::Number => "number",
        minijinja::value::ValueKind::String => "string",
        minijinja::value::ValueKind::Bytes => "bytes",
        minijinja::value::ValueKind::Seq => "array",
        minijinja::value::ValueKind::Map => "object",
        minijinja::value::ValueKind::Iterable => "iterable",
        _ => "unknown",
    };

    Ok(Value::from(type_name))
}

/// Pretty-print value structure to stderr and return it
///
/// # Arguments
///
/// * `value` (required) - Value to inspect
///
/// # Returns
///
/// Returns the same value that was passed in
///
/// # Example
///
/// ```jinja
/// {# Inspect complex data structures #}
/// {% set config = inspect(value=parse_json(string='{"db": {"host": "localhost", "port": 5432}}')) %}
///
/// {# Inspect and continue #}
/// {% set data = inspect(value=filter_env(pattern="SERVER_*")) %}
/// Found {{ data | length }} variables
/// ```
pub fn inspect_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let value: Value = kwargs.get("value")?;

    // Pretty-print the value structure to stderr
    eprintln!("[INSPECT] {:#?}", value);

    // Return the value unchanged
    Ok(value)
}

/// Assert a condition or fail with an error message
///
/// # Arguments
///
/// * `condition` (required) - Boolean condition to check
/// * `message` (optional) - Error message if assertion fails (default: "Assertion failed")
///
/// # Returns
///
/// Returns true if condition is true, otherwise throws an error
///
/// # Example
///
/// ```jinja
/// {# Assert environment variable exists #}
/// {% set port = get_env(name="PORT", default="") %}
/// {{ assert(condition=port != "", message="PORT environment variable is required") }}
///
/// {# Assert file exists before reading #}
/// {{ assert(condition=file_exists(path="config.json"), message="config.json not found") }}
/// {% set config = read_file(path="config.json") %}
///
/// {# Assert valid range #}
/// {% set workers = get_env(name="WORKERS", default="4") | int %}
/// {{ assert(condition=workers >= 1 and workers <= 100, message="WORKERS must be between 1 and 100") }}
/// ```
pub fn assert_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let condition: bool = kwargs.get("condition")?;
    let message: String = kwargs
        .get("message")
        .unwrap_or_else(|_| "Assertion failed".to_string());

    if !condition {
        return Err(Error::new(ErrorKind::InvalidOperation, message));
    }

    Ok(Value::from(true))
}

/// Print a warning message to stderr and continue
///
/// # Arguments
///
/// * `message` (required) - Warning message to print
///
/// # Returns
///
/// Returns empty string (so it can be used in templates without output)
///
/// # Example
///
/// ```jinja
/// {# Warn about missing optional config #}
/// {% if not file_exists(path="custom.conf") %}
///   {{ warn(message="custom.conf not found, using defaults") }}
/// {% endif %}
///
/// {# Warn about deprecated usage #}
/// {% set old_var = get_env(name="DEPRECATED_VAR", default="") %}
/// {% if old_var %}
///   {{ warn(message="DEPRECATED_VAR is deprecated, use NEW_VAR instead") }}
/// {% endif %}
///
/// {# Warn about potentially unsafe configuration #}
/// {% set debug = get_env(name="DEBUG", default="false") %}
/// {% if debug == "true" %}
///   {{ warn(message="DEBUG mode is enabled in production") }}
/// {% endif %}
/// ```
pub fn warn_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let message: String = kwargs.get("message")?;

    // Print warning to stderr
    eprintln!("[WARNING] {}", message);

    // Return empty string so it doesn't affect template output
    Ok(Value::from(""))
}

/// Abort template rendering with an error message
///
/// # Arguments
///
/// * `message` (required) - Error message
///
/// # Returns
///
/// Never returns - always throws an error
///
/// # Example
///
/// ```jinja
/// {# Abort if critical file is missing #}
/// {% if not file_exists(path="critical.conf") %}
///   {{ abort(message="Critical configuration file 'critical.conf' is missing") }}
/// {% endif %}
///
/// {# Abort if environment is invalid #}
/// {% set env = get_env(name="APP_ENV", default="") %}
/// {% if env not in ["development", "staging", "production"] %}
///   {{ abort(message="Invalid APP_ENV: must be development, staging, or production") }}
/// {% endif %}
///
/// {# Abort on validation failure #}
/// {% set port = get_env(name="PORT", default="8080") | int %}
/// {% if port < 1024 or port > 65535 %}
///   {{ abort(message="Invalid PORT: must be between 1024 and 65535") }}
/// {% endif %}
/// ```
pub fn abort_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let message: String = kwargs.get("message")?;

    // Return error to abort rendering
    Err(Error::new(ErrorKind::InvalidOperation, message))
}
