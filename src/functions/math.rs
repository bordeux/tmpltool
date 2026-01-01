//! Math functions for MiniJinja templates
//!
//! This module provides mathematical calculation functions:
//! - Basic operations (min, max, abs)
//! - Rounding functions (round, ceil, floor)
//! - Percentage calculations

use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};

/// Return minimum of two values
///
/// # Arguments
///
/// * `a` (required) - First number
/// * `b` (required) - Second number
///
/// # Returns
///
/// Returns the smaller of the two values
///
/// # Example
///
/// ```jinja
/// {# Find minimum #}
/// {{ min(a=10, b=20) }}
/// {# Output: 10 #}
///
/// {# With variables #}
/// {% set cpu1 = 45.2 %}
/// {% set cpu2 = 38.7 %}
/// Lowest CPU: {{ min(a=cpu1, b=cpu2) }}%
/// ```
pub fn min_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let a: Value = kwargs.get("a")?;
    let b: Value = kwargs.get("b")?;

    // Convert to serde_json::Value to extract numbers
    let json_a: serde_json::Value = serde_json::to_value(&a).map_err(|e| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("Failed to convert value: {}", e),
        )
    })?;

    let json_b: serde_json::Value = serde_json::to_value(&b).map_err(|e| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("Failed to convert value: {}", e),
        )
    })?;

    let num_a = json_a.as_f64().ok_or_else(|| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("min requires numeric values, found: {}", a),
        )
    })?;

    let num_b = json_b.as_f64().ok_or_else(|| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("min requires numeric values, found: {}", b),
        )
    })?;

    let result = num_a.min(num_b);

    // Return as integer if no decimal part, otherwise as float
    if result.fract() == 0.0 {
        Ok(Value::from(result as i64))
    } else {
        Ok(Value::from(result))
    }
}

/// Return maximum of two values
///
/// # Arguments
///
/// * `a` (required) - First number
/// * `b` (required) - Second number
///
/// # Returns
///
/// Returns the larger of the two values
///
/// # Example
///
/// ```jinja
/// {# Find maximum #}
/// {{ max(a=10, b=20) }}
/// {# Output: 20 #}
///
/// {# With variables #}
/// {% set memory1 = 2048 %}
/// {% set memory2 = 4096 %}
/// Peak memory: {{ max(a=memory1, b=memory2) }}MB
/// ```
pub fn max_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let a: Value = kwargs.get("a")?;
    let b: Value = kwargs.get("b")?;

    // Convert to serde_json::Value to extract numbers
    let json_a: serde_json::Value = serde_json::to_value(&a).map_err(|e| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("Failed to convert value: {}", e),
        )
    })?;

    let json_b: serde_json::Value = serde_json::to_value(&b).map_err(|e| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("Failed to convert value: {}", e),
        )
    })?;

    let num_a = json_a.as_f64().ok_or_else(|| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("max requires numeric values, found: {}", a),
        )
    })?;

    let num_b = json_b.as_f64().ok_or_else(|| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("max requires numeric values, found: {}", b),
        )
    })?;

    let result = num_a.max(num_b);

    // Return as integer if no decimal part, otherwise as float
    if result.fract() == 0.0 {
        Ok(Value::from(result as i64))
    } else {
        Ok(Value::from(result))
    }
}

/// Return absolute value
///
/// # Arguments
///
/// * `number` (required) - Number to get absolute value of
///
/// # Returns
///
/// Returns the absolute value (always positive)
///
/// # Example
///
/// ```jinja
/// {# Absolute value #}
/// {{ abs(number=-42) }}
/// {# Output: 42 #}
///
/// {# Temperature difference #}
/// {% set temp1 = 25 %}
/// {% set temp2 = 18 %}
/// Difference: {{ abs(number=temp1 - temp2) }}°C
/// ```
pub fn abs_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let number: Value = kwargs.get("number")?;

    // Convert to serde_json::Value to extract number
    let json_value: serde_json::Value = serde_json::to_value(&number).map_err(|e| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("Failed to convert value: {}", e),
        )
    })?;

    let num = json_value.as_f64().ok_or_else(|| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("abs requires a numeric value, found: {}", number),
        )
    })?;

    let result = num.abs();

    // Return as integer if no decimal part, otherwise as float
    if result.fract() == 0.0 {
        Ok(Value::from(result as i64))
    } else {
        Ok(Value::from(result))
    }
}

/// Round to N decimal places
///
/// # Arguments
///
/// * `number` (required) - Number to round
/// * `decimals` (optional) - Number of decimal places (default: 0)
///
/// # Returns
///
/// Returns the number rounded to the specified decimal places
///
/// # Example
///
/// ```jinja
/// {# Round to nearest integer #}
/// {{ round(number=3.7) }}
/// {# Output: 4 #}
///
/// {# Round to 2 decimal places #}
/// {{ round(number=3.14159, decimals=2) }}
/// {# Output: 3.14 #}
///
/// {# Price calculation #}
/// {% set price = 19.999 %}
/// Price: ${{ round(number=price, decimals=2) }}
/// ```
pub fn round_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let number: Value = kwargs.get("number")?;
    let decimals: Option<i32> = kwargs.get("decimals").ok();

    // Convert to serde_json::Value to extract number
    let json_value: serde_json::Value = serde_json::to_value(&number).map_err(|e| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("Failed to convert value: {}", e),
        )
    })?;

    let num = json_value.as_f64().ok_or_else(|| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("round requires a numeric value, found: {}", number),
        )
    })?;

    let decimals = decimals.unwrap_or(0);

    if decimals < 0 {
        return Err(Error::new(
            ErrorKind::InvalidOperation,
            "decimals must be non-negative",
        ));
    }

    let multiplier = 10_f64.powi(decimals);
    let result = (num * multiplier).round() / multiplier;

    // Return as integer if no decimal part, otherwise as float
    if result.fract() == 0.0 && decimals == 0 {
        Ok(Value::from(result as i64))
    } else {
        Ok(Value::from(result))
    }
}

/// Round up to nearest integer
///
/// # Arguments
///
/// * `number` (required) - Number to round up
///
/// # Returns
///
/// Returns the smallest integer greater than or equal to the number
///
/// # Example
///
/// ```jinja
/// {# Round up #}
/// {{ ceil(number=3.1) }}
/// {# Output: 4 #}
///
/// {# Calculate required servers #}
/// {% set users = 150 %}
/// {% set users_per_server = 50 %}
/// Servers needed: {{ ceil(number=users / users_per_server) }}
/// ```
pub fn ceil_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let number: Value = kwargs.get("number")?;

    // Convert to serde_json::Value to extract number
    let json_value: serde_json::Value = serde_json::to_value(&number).map_err(|e| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("Failed to convert value: {}", e),
        )
    })?;

    let num = json_value.as_f64().ok_or_else(|| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("ceil requires a numeric value, found: {}", number),
        )
    })?;

    Ok(Value::from(num.ceil() as i64))
}

/// Round down to nearest integer
///
/// # Arguments
///
/// * `number` (required) - Number to round down
///
/// # Returns
///
/// Returns the largest integer less than or equal to the number
///
/// # Example
///
/// ```jinja
/// {# Round down #}
/// {{ floor(number=3.9) }}
/// {# Output: 3 #}
///
/// {# Calculate filled pages #}
/// {% set items = 47 %}
/// {% set items_per_page = 10 %}
/// Full pages: {{ floor(number=items / items_per_page) }}
/// ```
pub fn floor_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let number: Value = kwargs.get("number")?;

    // Convert to serde_json::Value to extract number
    let json_value: serde_json::Value = serde_json::to_value(&number).map_err(|e| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("Failed to convert value: {}", e),
        )
    })?;

    let num = json_value.as_f64().ok_or_else(|| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("floor requires a numeric value, found: {}", number),
        )
    })?;

    Ok(Value::from(num.floor() as i64))
}

/// Calculate percentage
///
/// # Arguments
///
/// * `value` (required) - The part value
/// * `total` (required) - The total/whole value
///
/// # Returns
///
/// Returns the percentage (0-100)
///
/// # Example
///
/// ```jinja
/// {# Calculate percentage #}
/// {{ percentage(value=25, total=100) }}
/// {# Output: 25 #}
///
/// {# Progress calculation #}
/// {% set completed = 7 %}
/// {% set total_tasks = 10 %}
/// Progress: {{ round(number=percentage(value=completed, total=total_tasks), decimals=1) }}%
///
/// {# Disk usage #}
/// {% set used = 450 %}
/// {% set capacity = 500 %}
/// Disk usage: {{ round(number=percentage(value=used, total=capacity), decimals=2) }}%
/// ```
pub fn percentage_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let value: Value = kwargs.get("value")?;
    let total: Value = kwargs.get("total")?;

    // Convert to serde_json::Value to extract numbers
    let json_value: serde_json::Value = serde_json::to_value(&value).map_err(|e| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("Failed to convert value: {}", e),
        )
    })?;

    let json_total: serde_json::Value = serde_json::to_value(&total).map_err(|e| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("Failed to convert total: {}", e),
        )
    })?;

    let num_value = json_value.as_f64().ok_or_else(|| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("percentage requires numeric value, found: {}", value),
        )
    })?;

    let num_total = json_total.as_f64().ok_or_else(|| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("percentage requires numeric total, found: {}", total),
        )
    })?;

    if num_total == 0.0 {
        return Err(Error::new(
            ErrorKind::InvalidOperation,
            "percentage total cannot be zero",
        ));
    }

    let result = (num_value / num_total) * 100.0;

    Ok(Value::from(result))
}
