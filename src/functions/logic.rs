//! Logic functions for MiniJinja templates
//!
//! This module provides logical operations and conditional functions:
//! - Default value handling
//! - Coalescing (first non-null)
//! - Ternary operator
//! - Range checking

use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};

/// Return default if value is falsy
///
/// # Arguments
///
/// * `value` (required) - Value to check
/// * `default` (required) - Default value to return if value is falsy
///
/// # Returns
///
/// Returns the value if truthy, otherwise returns the default
///
/// # Example
///
/// ```jinja
/// {# Use default for empty string #}
/// {{ default(value="", default="N/A") }}
/// {# Output: N/A #}
///
/// {# Use actual value if truthy #}
/// {{ default(value="Hello", default="N/A") }}
/// {# Output: Hello #}
///
/// {# Use default for null/undefined #}
/// {% set missing = none %}
/// {{ default(value=missing, default="Not set") }}
/// {# Output: Not set #}
///
/// {# Configuration with defaults #}
/// {% set config = {"port": 8080} %}
/// Host: {{ default(value=config.host, default="localhost") }}
/// Port: {{ default(value=config.port, default=3000) }}
/// ```
pub fn default_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let value: Value = kwargs.get("value")?;
    let default: Value = kwargs.get("default")?;

    // Check if value is falsy (null, undefined, false, 0, empty string, empty array)
    if value.is_undefined()
        || value.is_none()
        || (!value.is_true())
        || (value.as_str().is_some() && value.as_str().unwrap().is_empty())
        || (matches!(value.kind(), minijinja::value::ValueKind::Seq)
            && value.len().unwrap_or(1) == 0)
    {
        Ok(default)
    } else {
        Ok(value)
    }
}

/// Return first non-null value
///
/// # Arguments
///
/// * `values` (required) - Array of values to check
///
/// # Returns
///
/// Returns the first value that is not null/undefined, or null if all are null
///
/// # Example
///
/// ```jinja
/// {# Find first non-null value #}
/// {% set a = none %}
/// {% set b = none %}
/// {% set c = "found" %}
/// {{ coalesce(values=[a, b, c]) }}
/// {# Output: found #}
///
/// {# Configuration precedence #}
/// {% set env_host = none %}
/// {% set config_host = "prod.example.com" %}
/// {% set default_host = "localhost" %}
/// Host: {{ coalesce(values=[env_host, config_host, default_host]) }}
/// {# Output: Host: prod.example.com #}
///
/// {# All null returns null #}
/// {{ coalesce(values=[none, none]) }}
/// {# Output: (empty/null) #}
/// ```
pub fn coalesce_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let values: Value = kwargs.get("values")?;

    if !matches!(values.kind(), minijinja::value::ValueKind::Seq) {
        return Err(Error::new(
            ErrorKind::InvalidOperation,
            "coalesce requires an array of values",
        ));
    }

    if let Ok(seq) = values.try_iter() {
        for item in seq {
            if !item.is_undefined() && !item.is_none() {
                return Ok(item);
            }
        }
    }

    // All values are null/undefined
    Ok(Value::UNDEFINED)
}

/// Ternary operator - return one value based on condition
///
/// # Arguments
///
/// * `condition` (required) - Boolean condition to evaluate
/// * `true_val` (required) - Value to return if condition is true
/// * `false_val` (required) - Value to return if condition is false
///
/// # Returns
///
/// Returns true_val if condition is truthy, otherwise false_val
///
/// # Example
///
/// ```jinja
/// {# Simple ternary #}
/// {{ ternary(condition=true, true_val="Yes", false_val="No") }}
/// {# Output: Yes #}
///
/// {# With comparison #}
/// {% set score = 85 %}
/// Result: {{ ternary(condition=score >= 60, true_val="Pass", false_val="Fail") }}
/// {# Output: Result: Pass #}
///
/// {# Nested ternary #}
/// {% set temp = 25 %}
/// Weather: {{ ternary(
///   condition=temp > 30,
///   true_val="Hot",
///   false_val=ternary(condition=temp > 20, true_val="Warm", false_val="Cold")
/// ) }}
/// {# Output: Weather: Warm #}
///
/// {# Status indicator #}
/// {% set cpu_usage = 75 %}
/// Status: {{ ternary(
///   condition=cpu_usage > 90,
///   true_val="Critical",
///   false_val="Normal"
/// ) }}
/// ```
pub fn ternary_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let condition: Value = kwargs.get("condition")?;
    let true_val: Value = kwargs.get("true_val")?;
    let false_val: Value = kwargs.get("false_val")?;

    // Evaluate condition as boolean
    // For non-boolean values, treat as truthy/falsy
    let is_true = condition.is_true();

    if is_true { Ok(true_val) } else { Ok(false_val) }
}

/// Check if value is within range (inclusive)
///
/// # Arguments
///
/// * `value` (required) - Numeric value to check
/// * `min` (required) - Minimum value (inclusive)
/// * `max` (required) - Maximum value (inclusive)
///
/// # Returns
///
/// Returns true if min <= value <= max, false otherwise
///
/// # Example
///
/// ```jinja
/// {# Check if in range #}
/// {{ in_range(value=50, min=0, max=100) }}
/// {# Output: true #}
///
/// {# Out of range #}
/// {{ in_range(value=150, min=0, max=100) }}
/// {# Output: false #}
///
/// {# Validate port number #}
/// {% set port = 8080 %}
/// {% if in_range(value=port, min=1024, max=65535) %}
///   Valid port number
/// {% else %}
///   Invalid port number
/// {% endif %}
///
/// {# Temperature range check #}
/// {% set temp = 22 %}
/// Comfortable: {{ in_range(value=temp, min=18, max=25) }}
///
/// {# Resource usage validation #}
/// {% set cpu = 75.5 %}
/// {% if in_range(value=cpu, min=0, max=80) %}
///   CPU usage normal
/// {% else %}
///   CPU usage high
/// {% endif %}
/// ```
pub fn in_range_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let value: Value = kwargs.get("value")?;
    let min: Value = kwargs.get("min")?;
    let max: Value = kwargs.get("max")?;

    // Convert to serde_json::Value to extract numbers
    let json_value: serde_json::Value = serde_json::to_value(&value).map_err(|e| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("Failed to convert value: {}", e),
        )
    })?;

    let json_min: serde_json::Value = serde_json::to_value(&min).map_err(|e| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("Failed to convert min: {}", e),
        )
    })?;

    let json_max: serde_json::Value = serde_json::to_value(&max).map_err(|e| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("Failed to convert max: {}", e),
        )
    })?;

    let num_value = json_value.as_f64().ok_or_else(|| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("in_range requires numeric value, found: {}", value),
        )
    })?;

    let num_min = json_min.as_f64().ok_or_else(|| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("in_range requires numeric min, found: {}", min),
        )
    })?;

    let num_max = json_max.as_f64().ok_or_else(|| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("in_range requires numeric max, found: {}", max),
        )
    })?;

    Ok(Value::from(num_value >= num_min && num_value <= num_max))
}
