//! Array functions that support both function and filter syntax.
//!
//! # Function Syntax
//! ```jinja
//! {{ array_sum(array=numbers) }}
//! {{ array_unique(array=items) }}
//! ```
//!
//! # Filter Syntax
//! ```jinja
//! {{ numbers | array_sum }}
//! {{ items | array_unique }}
//! ```
//!
//! # Chaining
//! ```jinja
//! {{ items | array_unique | array_sum }}
//! ```

use super::FilterFunction;
use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};
use std::collections::HashSet;

/// Helper to extract array from Value
fn extract_array(value: &Value, fn_name: &str) -> Result<Value, Error> {
    if !matches!(value.kind(), minijinja::value::ValueKind::Seq) {
        return Err(Error::new(
            ErrorKind::InvalidOperation,
            format!("{} requires an array", fn_name),
        ));
    }
    Ok(value.clone())
}

/// Helper to convert Value item to f64
fn value_to_f64(item: &Value, fn_name: &str) -> Result<f64, Error> {
    let json_value: serde_json::Value = serde_json::to_value(item).map_err(|e| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("Failed to convert value: {}", e),
        )
    })?;

    json_value.as_f64().ok_or_else(|| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("{} requires numeric values, found: {}", fn_name, item),
        )
    })
}

/// Helper to format result as integer if possible
fn format_number(num: f64) -> Value {
    if num.fract() == 0.0 {
        Value::from(num as i64)
    } else {
        Value::from(num)
    }
}

// ============================================
// ArraySum
// ============================================

/// Calculate sum of array values.
///
/// # Function Syntax
/// ```jinja
/// {{ array_sum(array=numbers) }}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ numbers | array_sum }}
/// {{ [1, 2, 3, 4, 5] | array_sum }}
/// ```
pub struct ArraySum;

impl ArraySum {
    fn compute(array: &Value) -> Result<Value, Error> {
        let mut sum = 0.0_f64;

        if let Ok(seq) = array.try_iter() {
            for item in seq {
                sum += value_to_f64(&item, "array_sum")?;
            }
        }

        Ok(format_number(sum))
    }
}

impl FilterFunction for ArraySum {
    const NAME: &'static str = "array_sum";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let array: Value = kwargs.get("array")?;
        extract_array(&array, "array_sum")?;
        Self::compute(&array)
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        extract_array(value, "array_sum")?;
        Self::compute(value)
    }
}

// ============================================
// ArrayAvg
// ============================================

/// Calculate average of array values.
///
/// # Function Syntax
/// ```jinja
/// {{ array_avg(array=numbers) }}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ numbers | array_avg }}
/// {{ scores | array_avg }}
/// ```
pub struct ArrayAvg;

impl ArrayAvg {
    fn compute(array: &Value) -> Result<Value, Error> {
        let mut sum = 0.0_f64;
        let mut count = 0;

        if let Ok(seq) = array.try_iter() {
            for item in seq {
                sum += value_to_f64(&item, "array_avg")?;
                count += 1;
            }
        }

        if count == 0 {
            return Ok(Value::from(0));
        }

        Ok(format_number(sum / count as f64))
    }
}

impl FilterFunction for ArrayAvg {
    const NAME: &'static str = "array_avg";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let array: Value = kwargs.get("array")?;
        extract_array(&array, "array_avg")?;
        Self::compute(&array)
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        extract_array(value, "array_avg")?;
        Self::compute(value)
    }
}

// ============================================
// ArrayMedian
// ============================================

/// Calculate median of array values.
///
/// # Function Syntax
/// ```jinja
/// {{ array_median(array=numbers) }}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ numbers | array_median }}
/// {{ [1, 3, 5, 7, 9] | array_median }}
/// ```
pub struct ArrayMedian;

impl ArrayMedian {
    fn compute(array: &Value) -> Result<Value, Error> {
        let mut numbers: Vec<f64> = Vec::new();

        if let Ok(seq) = array.try_iter() {
            for item in seq {
                numbers.push(value_to_f64(&item, "array_median")?);
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

        Ok(format_number(median))
    }
}

impl FilterFunction for ArrayMedian {
    const NAME: &'static str = "array_median";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let array: Value = kwargs.get("array")?;
        extract_array(&array, "array_median")?;
        Self::compute(&array)
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        extract_array(value, "array_median")?;
        Self::compute(value)
    }
}

// ============================================
// ArrayMin
// ============================================

/// Find minimum value in array.
///
/// # Function Syntax
/// ```jinja
/// {{ array_min(array=numbers) }}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ numbers | array_min }}
/// {{ prices | array_min }}
/// ```
pub struct ArrayMin;

impl ArrayMin {
    fn compute(array: &Value) -> Result<Value, Error> {
        let mut min_value: Option<f64> = None;

        if let Ok(seq) = array.try_iter() {
            for item in seq {
                let num = value_to_f64(&item, "array_min")?;
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
            Some(min) => Ok(format_number(min)),
        }
    }
}

impl FilterFunction for ArrayMin {
    const NAME: &'static str = "array_min";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let array: Value = kwargs.get("array")?;
        extract_array(&array, "array_min")?;
        Self::compute(&array)
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        extract_array(value, "array_min")?;
        Self::compute(value)
    }
}

// ============================================
// ArrayMax
// ============================================

/// Find maximum value in array.
///
/// # Function Syntax
/// ```jinja
/// {{ array_max(array=numbers) }}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ numbers | array_max }}
/// {{ prices | array_max }}
/// ```
pub struct ArrayMax;

impl ArrayMax {
    fn compute(array: &Value) -> Result<Value, Error> {
        let mut max_value: Option<f64> = None;

        if let Ok(seq) = array.try_iter() {
            for item in seq {
                let num = value_to_f64(&item, "array_max")?;
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
            Some(max) => Ok(format_number(max)),
        }
    }
}

impl FilterFunction for ArrayMax {
    const NAME: &'static str = "array_max";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let array: Value = kwargs.get("array")?;
        extract_array(&array, "array_max")?;
        Self::compute(&array)
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        extract_array(value, "array_max")?;
        Self::compute(value)
    }
}

// ============================================
// ArrayUnique
// ============================================

/// Remove duplicate values from array.
///
/// # Function Syntax
/// ```jinja
/// {{ array_unique(array=items) }}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ items | array_unique }}
/// {{ [1, 2, 2, 3, 3, 3] | array_unique }}
/// ```
pub struct ArrayUnique;

impl ArrayUnique {
    fn compute(array: &Value) -> Result<Value, Error> {
        let mut seen: HashSet<String> = HashSet::new();
        let mut unique: Vec<serde_json::Value> = Vec::new();

        if let Ok(seq) = array.try_iter() {
            for item in seq {
                let json_value: serde_json::Value = serde_json::to_value(&item).map_err(|e| {
                    Error::new(
                        ErrorKind::InvalidOperation,
                        format!("Failed to convert item: {}", e),
                    )
                })?;

                // Create a string representation for comparison
                let item_str = serde_json::to_string(&json_value).unwrap_or_default();

                if seen.insert(item_str) {
                    unique.push(json_value);
                }
            }
        }

        Ok(Value::from_serialize(unique))
    }
}

impl FilterFunction for ArrayUnique {
    const NAME: &'static str = "array_unique";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let array: Value = kwargs.get("array")?;
        extract_array(&array, "array_unique")?;
        Self::compute(&array)
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        extract_array(value, "array_unique")?;
        Self::compute(value)
    }
}

// ============================================
// ArrayFlatten
// ============================================

/// Flatten nested arrays by one level.
///
/// # Function Syntax
/// ```jinja
/// {{ array_flatten(array=nested) }}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ nested | array_flatten }}
/// {{ [[1, 2], [3, 4]] | array_flatten }}
/// ```
pub struct ArrayFlatten;

impl ArrayFlatten {
    fn compute(array: &Value) -> Result<Value, Error> {
        let mut flattened: Vec<serde_json::Value> = Vec::new();

        if let Ok(seq) = array.try_iter() {
            for item in seq {
                let json_value: serde_json::Value = serde_json::to_value(&item).map_err(|e| {
                    Error::new(
                        ErrorKind::InvalidOperation,
                        format!("Failed to convert item: {}", e),
                    )
                })?;

                // If item is an array, flatten it one level
                if let Some(nested_array) = json_value.as_array() {
                    for nested_item in nested_array {
                        flattened.push(nested_item.clone());
                    }
                } else {
                    // Not an array, just add the item
                    flattened.push(json_value);
                }
            }
        }

        Ok(Value::from_serialize(flattened))
    }
}

impl FilterFunction for ArrayFlatten {
    const NAME: &'static str = "array_flatten";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let array: Value = kwargs.get("array")?;
        extract_array(&array, "array_flatten")?;
        Self::compute(&array)
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        extract_array(value, "array_flatten")?;
        Self::compute(value)
    }
}
