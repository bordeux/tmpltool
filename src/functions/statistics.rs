//! Statistical functions for MiniJinja templates
//!
//! This module provides statistical functions for analyzing arrays:
//! - Sum, average, median
//! - Minimum and maximum values

use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};

/// Calculate sum of array values
///
/// # Arguments
///
/// * `array` (required) - Array of numbers to sum
///
/// # Returns
///
/// Returns the sum of all numeric values in the array
///
/// # Example
///
/// ```jinja
/// {# Sum of array values #}
/// {% set numbers = [1, 2, 3, 4, 5] %}
/// {{ array_sum(array=numbers) }}
/// {# Output: 15 #}
///
/// {# Sum with decimals #}
/// {% set prices = [10.5, 20.25, 5.75] %}
/// {{ array_sum(array=prices) }}
/// {# Output: 36.5 #}
/// ```
pub fn array_sum_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let array: Value = kwargs.get("array")?;

    if !matches!(array.kind(), minijinja::value::ValueKind::Seq) {
        return Err(Error::new(
            ErrorKind::InvalidOperation,
            "array_sum requires an array",
        ));
    }

    let mut sum = 0.0_f64;
    if let Ok(seq) = array.try_iter() {
        for item in seq {
            // Convert to serde_json::Value to extract number
            let json_value: serde_json::Value = serde_json::to_value(&item).map_err(|e| {
                Error::new(
                    ErrorKind::InvalidOperation,
                    format!("Failed to convert value: {}", e),
                )
            })?;

            let num = json_value.as_f64().ok_or_else(|| {
                Error::new(
                    ErrorKind::InvalidOperation,
                    format!("array_sum requires numeric values, found: {}", item),
                )
            })?;
            sum += num;
        }
    }

    // Return as integer if no decimal part, otherwise as float
    if sum.fract() == 0.0 {
        Ok(Value::from(sum as i64))
    } else {
        Ok(Value::from(sum))
    }
}

/// Calculate average of array values
///
/// # Arguments
///
/// * `array` (required) - Array of numbers
///
/// # Returns
///
/// Returns the arithmetic mean of all values. Returns 0 for empty arrays.
///
/// # Example
///
/// ```jinja
/// {# Average of array values #}
/// {% set scores = [85, 90, 78, 92, 88] %}
/// {{ array_avg(array=scores) }}
/// {# Output: 86.6 #}
///
/// {# Empty array returns 0 #}
/// {% set empty = [] %}
/// {{ array_avg(array=empty) }}
/// {# Output: 0 #}
/// ```
pub fn array_avg_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let array: Value = kwargs.get("array")?;

    if !matches!(array.kind(), minijinja::value::ValueKind::Seq) {
        return Err(Error::new(
            ErrorKind::InvalidOperation,
            "array_avg requires an array",
        ));
    }

    let mut sum = 0.0_f64;
    let mut count = 0;

    if let Ok(seq) = array.try_iter() {
        for item in seq {
            // Convert to serde_json::Value to extract number
            let json_value: serde_json::Value = serde_json::to_value(&item).map_err(|e| {
                Error::new(
                    ErrorKind::InvalidOperation,
                    format!("Failed to convert value: {}", e),
                )
            })?;

            let num = json_value.as_f64().ok_or_else(|| {
                Error::new(
                    ErrorKind::InvalidOperation,
                    format!("array_avg requires numeric values, found: {}", item),
                )
            })?;
            sum += num;
            count += 1;
        }
    }

    if count == 0 {
        return Ok(Value::from(0));
    }

    let avg = sum / count as f64;
    // Return as integer if no decimal part, otherwise as float
    if avg.fract() == 0.0 {
        Ok(Value::from(avg as i64))
    } else {
        Ok(Value::from(avg))
    }
}

/// Calculate median of array values
///
/// # Arguments
///
/// * `array` (required) - Array of numbers
///
/// # Returns
///
/// Returns the median value. For even-length arrays, returns the average of the two middle values.
///
/// # Example
///
/// ```jinja
/// {# Median of odd-length array #}
/// {% set nums = [1, 3, 5, 7, 9] %}
/// {{ array_median(array=nums) }}
/// {# Output: 5 #}
///
/// {# Median of even-length array #}
/// {% set nums = [1, 2, 3, 4] %}
/// {{ array_median(array=nums) }}
/// {# Output: 2.5 #}
/// ```
pub fn array_median_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let array: Value = kwargs.get("array")?;

    if !matches!(array.kind(), minijinja::value::ValueKind::Seq) {
        return Err(Error::new(
            ErrorKind::InvalidOperation,
            "array_median requires an array",
        ));
    }

    let mut numbers: Vec<f64> = Vec::new();

    if let Ok(seq) = array.try_iter() {
        for item in seq {
            // Convert to serde_json::Value to extract number
            let json_value: serde_json::Value = serde_json::to_value(&item).map_err(|e| {
                Error::new(
                    ErrorKind::InvalidOperation,
                    format!("Failed to convert value: {}", e),
                )
            })?;

            let num = json_value.as_f64().ok_or_else(|| {
                Error::new(
                    ErrorKind::InvalidOperation,
                    format!("array_median requires numeric values, found: {}", item),
                )
            })?;
            numbers.push(num);
        }
    }

    if numbers.is_empty() {
        return Ok(Value::from(0));
    }

    numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let len = numbers.len();
    let median = if len.is_multiple_of(2) {
        // Even length: average of two middle values
        (numbers[len / 2 - 1] + numbers[len / 2]) / 2.0
    } else {
        // Odd length: middle value
        numbers[len / 2]
    };

    // Return as integer if no decimal part, otherwise as float
    if median.fract() == 0.0 {
        Ok(Value::from(median as i64))
    } else {
        Ok(Value::from(median))
    }
}

/// Find minimum value in array
///
/// # Arguments
///
/// * `array` (required) - Array of numbers
///
/// # Returns
///
/// Returns the minimum value in the array
///
/// # Example
///
/// ```jinja
/// {# Find minimum value #}
/// {% set numbers = [42, 17, 99, 8, 55] %}
/// {{ array_min(array=numbers) }}
/// {# Output: 8 #}
///
/// {# Works with decimals #}
/// {% set prices = [10.99, 5.49, 15.99] %}
/// {{ array_min(array=prices) }}
/// {# Output: 5.49 #}
/// ```
pub fn array_min_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let array: Value = kwargs.get("array")?;

    if !matches!(array.kind(), minijinja::value::ValueKind::Seq) {
        return Err(Error::new(
            ErrorKind::InvalidOperation,
            "array_min requires an array",
        ));
    }

    let mut min_value: Option<f64> = None;

    if let Ok(seq) = array.try_iter() {
        for item in seq {
            // Convert to serde_json::Value to extract number
            let json_value: serde_json::Value = serde_json::to_value(&item).map_err(|e| {
                Error::new(
                    ErrorKind::InvalidOperation,
                    format!("Failed to convert value: {}", e),
                )
            })?;

            let num = json_value.as_f64().ok_or_else(|| {
                Error::new(
                    ErrorKind::InvalidOperation,
                    format!("array_min requires numeric values, found: {}", item),
                )
            })?;

            min_value = Some(match min_value {
                None => num,
                Some(current_min) => num.min(current_min),
            });
        }
    }

    match min_value {
        None => Err(Error::new(
            ErrorKind::InvalidOperation,
            "array_min requires a non-empty array",
        )),
        Some(min) => {
            // Return as integer if no decimal part, otherwise as float
            if min.fract() == 0.0 {
                Ok(Value::from(min as i64))
            } else {
                Ok(Value::from(min))
            }
        }
    }
}

/// Find maximum value in array
///
/// # Arguments
///
/// * `array` (required) - Array of numbers
///
/// # Returns
///
/// Returns the maximum value in the array
///
/// # Example
///
/// ```jinja
/// {# Find maximum value #}
/// {% set numbers = [42, 17, 99, 8, 55] %}
/// {{ array_max(array=numbers) }}
/// {# Output: 99 #}
///
/// {# Works with decimals #}
/// {% set prices = [10.99, 5.49, 15.99] %}
/// {{ array_max(array=prices) }}
/// {# Output: 15.99 #}
/// ```
pub fn array_max_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let array: Value = kwargs.get("array")?;

    if !matches!(array.kind(), minijinja::value::ValueKind::Seq) {
        return Err(Error::new(
            ErrorKind::InvalidOperation,
            "array_max requires an array",
        ));
    }

    let mut max_value: Option<f64> = None;

    if let Ok(seq) = array.try_iter() {
        for item in seq {
            // Convert to serde_json::Value to extract number
            let json_value: serde_json::Value = serde_json::to_value(&item).map_err(|e| {
                Error::new(
                    ErrorKind::InvalidOperation,
                    format!("Failed to convert value: {}", e),
                )
            })?;

            let num = json_value.as_f64().ok_or_else(|| {
                Error::new(
                    ErrorKind::InvalidOperation,
                    format!("array_max requires numeric values, found: {}", item),
                )
            })?;

            max_value = Some(match max_value {
                None => num,
                Some(current_max) => num.max(current_max),
            });
        }
    }

    match max_value {
        None => Err(Error::new(
            ErrorKind::InvalidOperation,
            "array_max requires a non-empty array",
        )),
        Some(max) => {
            // Return as integer if no decimal part, otherwise as float
            if max.fract() == 0.0 {
                Ok(Value::from(max as i64))
            } else {
                Ok(Value::from(max))
            }
        }
    }
}
