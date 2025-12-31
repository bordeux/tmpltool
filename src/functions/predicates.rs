//! Predicate functions for MiniJinja templates
//!
//! This module provides predicate functions for checking conditions on arrays and strings:
//! - Array predicates: any, all, contains
//! - String predicates: starts_with, ends_with

use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};

/// Check if any element in array matches a condition
///
/// # Arguments
///
/// * `array` (required) - The array to check
/// * `predicate` (required) - The condition to check (e.g., value to compare)
///
/// # Returns
///
/// Returns true if any element equals the predicate value
///
/// # Example
///
/// ```jinja
/// {# Check if any value equals 5 #}
/// {% if array_any(array=[1, 2, 5, 8], predicate=5) %}
///   Found 5!
/// {% endif %}
///
/// {# Check if any string contains "test" #}
/// {% set items = ["hello", "test123", "world"] %}
/// {{ array_any(array=items, predicate="test123") }}
/// {# Output: true #}
/// ```
pub fn array_any_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let array: Value = kwargs.get("array")?;
    let predicate: Value = kwargs.get("predicate")?;

    if !matches!(array.kind(), minijinja::value::ValueKind::Seq) {
        return Err(Error::new(
            ErrorKind::InvalidOperation,
            "array_any requires an array",
        ));
    }

    // Check if any element matches the predicate
    if let Ok(seq) = array.try_iter() {
        for item in seq {
            // Simple equality check
            if item == predicate {
                return Ok(Value::from(true));
            }
        }
    }

    Ok(Value::from(false))
}

/// Check if all elements in array match a condition
///
/// # Arguments
///
/// * `array` (required) - The array to check
/// * `predicate` (required) - The condition to check (e.g., value to compare)
///
/// # Returns
///
/// Returns true if all elements equal the predicate value
///
/// # Example
///
/// ```jinja
/// {# Check if all values equal 5 #}
/// {% if array_all(array=[5, 5, 5], predicate=5) %}
///   All are 5!
/// {% endif %}
///
/// {# Check if all strings equal "test" #}
/// {% set items = ["test", "test", "test"] %}
/// {{ array_all(array=items, predicate="test") }}
/// {# Output: true #}
/// ```
pub fn array_all_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let array: Value = kwargs.get("array")?;
    let predicate: Value = kwargs.get("predicate")?;

    if !matches!(array.kind(), minijinja::value::ValueKind::Seq) {
        return Err(Error::new(
            ErrorKind::InvalidOperation,
            "array_all requires an array",
        ));
    }

    // Empty arrays return true (vacuous truth)
    if let Ok(seq) = array.try_iter() {
        let items: Vec<_> = seq.collect();
        if items.is_empty() {
            return Ok(Value::from(true));
        }

        // Check if all elements match the predicate
        for item in items {
            if item != predicate {
                return Ok(Value::from(false));
            }
        }
    }

    Ok(Value::from(true))
}

/// Check if array contains a specific value
///
/// # Arguments
///
/// * `array` (required) - The array to search
/// * `value` (required) - The value to find
///
/// # Returns
///
/// Returns true if the array contains the value
///
/// # Example
///
/// ```jinja
/// {# Check if array contains 42 #}
/// {% if array_contains(array=[1, 2, 42, 3], value=42) %}
///   Found it!
/// {% endif %}
///
/// {# Check if array contains a string #}
/// {% set fruits = ["apple", "banana", "cherry"] %}
/// {{ array_contains(array=fruits, value="banana") }}
/// {# Output: true #}
/// ```
pub fn array_contains_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let array: Value = kwargs.get("array")?;
    let value: Value = kwargs.get("value")?;

    if !matches!(array.kind(), minijinja::value::ValueKind::Seq) {
        return Err(Error::new(
            ErrorKind::InvalidOperation,
            "array_contains requires an array",
        ));
    }

    // Check if array contains the value
    if let Ok(seq) = array.try_iter() {
        for item in seq {
            if item == value {
                return Ok(Value::from(true));
            }
        }
    }

    Ok(Value::from(false))
}

/// Check if string starts with a prefix
///
/// # Arguments
///
/// * `string` (required) - The string to check
/// * `prefix` (required) - The prefix to look for
///
/// # Returns
///
/// Returns true if the string starts with the prefix
///
/// # Example
///
/// ```jinja
/// {# Check if string starts with "Hello" #}
/// {% if starts_with(string="Hello World", prefix="Hello") %}
///   Starts with Hello!
/// {% endif %}
///
/// {# Check file extension #}
/// {% set filename = "config.yaml" %}
/// {{ starts_with(string=filename, prefix="config") }}
/// {# Output: true #}
/// ```
pub fn starts_with_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let string: String = kwargs.get("string")?;
    let prefix: String = kwargs.get("prefix")?;

    Ok(Value::from(string.starts_with(&prefix)))
}

/// Check if string ends with a suffix
///
/// # Arguments
///
/// * `string` (required) - The string to check
/// * `suffix` (required) - The suffix to look for
///
/// # Returns
///
/// Returns true if the string ends with the suffix
///
/// # Example
///
/// ```jinja
/// {# Check if string ends with ".txt" #}
/// {% if ends_with(string="readme.txt", suffix=".txt") %}
///   Text file detected!
/// {% endif %}
///
/// {# Check URL protocol #}
/// {% set url = "https://example.com" %}
/// {{ ends_with(string=url, suffix=".com") }}
/// {# Output: true #}
/// ```
pub fn ends_with_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let string: String = kwargs.get("string")?;
    let suffix: String = kwargs.get("suffix")?;

    Ok(Value::from(string.ends_with(&suffix)))
}
