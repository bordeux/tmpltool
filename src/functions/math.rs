//! Math functions for MiniJinja templates
//!
//! This module provides mathematical calculation functions:
//! - `min`: Return minimum of two values
//! - `max`: Return maximum of two values
//! - `percentage`: Calculate percentage
//!
//! Note: abs, round, ceil, floor are now in filter_functions/math.rs
//! with dual function+filter syntax support.

use super::metadata::{ArgumentMetadata, FunctionMetadata, SyntaxVariants};
use super::traits::Function;
use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};

/// Return minimum of two values
pub struct Min;

impl Function for Min {
    const NAME: &'static str = "min";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "min",
        category: "math",
        description: "Return the minimum of two numeric values",
        arguments: &[
            ArgumentMetadata {
                name: "a",
                arg_type: "number",
                required: true,
                default: None,
                description: "First number",
            },
            ArgumentMetadata {
                name: "b",
                arg_type: "number",
                required: true,
                default: None,
                description: "Second number",
            },
        ],
        return_type: "number",
        examples: &["{{ min(a=10, b=20) }}", "{{ min(a=cpu1, b=cpu2) }}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let a: Value = kwargs.get("a")?;
        let b: Value = kwargs.get("b")?;

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

        if result.fract() == 0.0 {
            Ok(Value::from(result as i64))
        } else {
            Ok(Value::from(result))
        }
    }
}

/// Return maximum of two values
pub struct Max;

impl Function for Max {
    const NAME: &'static str = "max";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "max",
        category: "math",
        description: "Return the maximum of two numeric values",
        arguments: &[
            ArgumentMetadata {
                name: "a",
                arg_type: "number",
                required: true,
                default: None,
                description: "First number",
            },
            ArgumentMetadata {
                name: "b",
                arg_type: "number",
                required: true,
                default: None,
                description: "Second number",
            },
        ],
        return_type: "number",
        examples: &["{{ max(a=10, b=20) }}", "{{ max(a=memory1, b=memory2) }}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let a: Value = kwargs.get("a")?;
        let b: Value = kwargs.get("b")?;

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

        if result.fract() == 0.0 {
            Ok(Value::from(result as i64))
        } else {
            Ok(Value::from(result))
        }
    }
}

/// Calculate percentage
pub struct Percentage;

impl Function for Percentage {
    const NAME: &'static str = "percentage";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "percentage",
        category: "math",
        description: "Calculate percentage (value / total * 100)",
        arguments: &[
            ArgumentMetadata {
                name: "value",
                arg_type: "number",
                required: true,
                default: None,
                description: "The part value",
            },
            ArgumentMetadata {
                name: "total",
                arg_type: "number",
                required: true,
                default: None,
                description: "The total/whole value",
            },
        ],
        return_type: "number",
        examples: &[
            "{{ percentage(value=25, total=100) }}",
            "{{ percentage(value=completed, total=total_tasks) }}",
        ],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let value: Value = kwargs.get("value")?;
        let total: Value = kwargs.get("total")?;

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

        if result.fract() == 0.0 {
            Ok(Value::from(result as i64))
        } else {
            Ok(Value::from(result))
        }
    }
}
