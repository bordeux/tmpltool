//! Logic functions for MiniJinja templates
//!
//! This module provides logical operations and conditional functions:
//! - `default`: Return default if value is falsy
//! - `coalesce`: Return first non-null value
//! - `ternary`: Ternary operator
//! - `in_range`: Check if value is within range

use super::metadata::{ArgumentMetadata, FunctionMetadata, SyntaxVariants};
use super::traits::Function;
use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};

/// Return default if value is falsy
pub struct Default;

impl Function for Default {
    const NAME: &'static str = "default";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "default",
        category: "logic",
        description: "Return the value if truthy, otherwise return the default",
        arguments: &[
            ArgumentMetadata {
                name: "value",
                arg_type: "any",
                required: true,
                default: None,
                description: "Value to check",
            },
            ArgumentMetadata {
                name: "default",
                arg_type: "any",
                required: true,
                default: None,
                description: "Default value to return if value is falsy",
            },
        ],
        return_type: "any",
        examples: &[
            "{{ default(value=\"\", default=\"N/A\") }}",
            "{{ default(value=config.host, default=\"localhost\") }}",
        ],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let value: Value = kwargs.get("value")?;
        let default: Value = kwargs.get("default")?;

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
}

/// Return first non-null value
pub struct Coalesce;

impl Function for Coalesce {
    const NAME: &'static str = "coalesce";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "coalesce",
        category: "logic",
        description: "Return the first non-null value from the array",
        arguments: &[ArgumentMetadata {
            name: "values",
            arg_type: "array",
            required: true,
            default: None,
            description: "Array of values to check",
        }],
        return_type: "any",
        examples: &[
            "{{ coalesce(values=[none, none, \"found\"]) }}",
            "{{ coalesce(values=[env_host, config_host, \"localhost\"]) }}",
        ],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
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

        Ok(Value::UNDEFINED)
    }
}

/// Ternary operator - return one value based on condition
pub struct Ternary;

impl Function for Ternary {
    const NAME: &'static str = "ternary";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "ternary",
        category: "logic",
        description: "Return true_val if condition is true, otherwise false_val",
        arguments: &[
            ArgumentMetadata {
                name: "condition",
                arg_type: "boolean",
                required: true,
                default: None,
                description: "Condition to evaluate",
            },
            ArgumentMetadata {
                name: "true_val",
                arg_type: "any",
                required: true,
                default: None,
                description: "Value to return if condition is true",
            },
            ArgumentMetadata {
                name: "false_val",
                arg_type: "any",
                required: true,
                default: None,
                description: "Value to return if condition is false",
            },
        ],
        return_type: "any",
        examples: &[
            "{{ ternary(condition=true, true_val=\"Yes\", false_val=\"No\") }}",
            "{{ ternary(condition=score >= 60, true_val=\"Pass\", false_val=\"Fail\") }}",
        ],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let condition: Value = kwargs.get("condition")?;
        let true_val: Value = kwargs.get("true_val")?;
        let false_val: Value = kwargs.get("false_val")?;

        let is_true = condition.is_true();

        if is_true { Ok(true_val) } else { Ok(false_val) }
    }
}

/// Check if value is within range (inclusive)
pub struct InRange;

impl Function for InRange {
    const NAME: &'static str = "in_range";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "in_range",
        category: "logic",
        description: "Check if value is within range (min <= value <= max)",
        arguments: &[
            ArgumentMetadata {
                name: "value",
                arg_type: "number",
                required: true,
                default: None,
                description: "Numeric value to check",
            },
            ArgumentMetadata {
                name: "min",
                arg_type: "number",
                required: true,
                default: None,
                description: "Minimum value (inclusive)",
            },
            ArgumentMetadata {
                name: "max",
                arg_type: "number",
                required: true,
                default: None,
                description: "Maximum value (inclusive)",
            },
        ],
        return_type: "boolean",
        examples: &[
            "{{ in_range(value=50, min=0, max=100) }}",
            "{% if in_range(value=port, min=1024, max=65535) %}Valid port{% endif %}",
        ],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let value: Value = kwargs.get("value")?;
        let min: Value = kwargs.get("min")?;
        let max: Value = kwargs.get("max")?;

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
}
