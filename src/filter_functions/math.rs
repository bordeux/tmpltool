//! Math functions that support both function and filter syntax.
//!
//! # Function Syntax
//! ```jinja
//! {{ abs(number=-42) }}
//! {{ round(number=3.14159, decimals=2) }}
//! {{ ceil(number=3.1) }}
//! {{ floor(number=3.9) }}
//! ```
//!
//! # Filter Syntax
//! ```jinja
//! {{ -42 | abs }}
//! {{ 3.14159 | round(decimals=2) }}
//! {{ 3.1 | ceil }}
//! {{ 3.9 | floor }}
//! ```
//!
//! # Chaining
//! ```jinja
//! {{ -3.7 | abs | ceil }}
//! {{ value | round(decimals=2) | abs }}
//! ```

use super::FilterFunction;
use crate::functions::metadata::{ArgumentMetadata, FunctionMetadata, SyntaxVariants};
use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};

/// Common metadata for single-argument number functions
const NUMBER_ARG: ArgumentMetadata = ArgumentMetadata {
    name: "number",
    arg_type: "number",
    required: true,
    default: None,
    description: "The number to process",
};

/// Helper to extract a numeric value from a MiniJinja Value
fn extract_number(value: &Value, fn_name: &str) -> Result<f64, Error> {
    if let Some(n) = value.as_i64() {
        return Ok(n as f64);
    }

    let json_value: serde_json::Value = serde_json::to_value(value).map_err(|e| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("Failed to convert value: {}", e),
        )
    })?;

    json_value.as_f64().ok_or_else(|| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("{} requires a numeric value, found: {}", fn_name, value),
        )
    })
}

/// Helper to return number as integer if no decimal part
fn smart_number(result: f64) -> Value {
    if result.fract() == 0.0 {
        Value::from(result as i64)
    } else {
        Value::from(result)
    }
}

// ============================================
// Abs
// ============================================

/// Return absolute value of a number.
pub struct Abs;

impl Abs {
    fn compute(num: f64) -> f64 {
        num.abs()
    }
}

impl FilterFunction for Abs {
    const NAME: &'static str = "abs";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "abs",
        category: "math",
        description: "Return absolute value of a number",
        arguments: &[NUMBER_ARG],
        return_type: "number",
        examples: &["{{ abs(number=-42) }}", "{{ -42 | abs }}"],
        syntax: SyntaxVariants::FUNCTION_AND_FILTER,
    };

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let number: Value = kwargs.get("number")?;
        let num = extract_number(&number, "abs")?;
        Ok(smart_number(Self::compute(num)))
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        let num = extract_number(value, "abs")?;
        Ok(smart_number(Self::compute(num)))
    }
}

// ============================================
// Round
// ============================================

/// Round a number to N decimal places.
pub struct Round;

impl Round {
    fn compute(num: f64, decimals: i32) -> Result<f64, Error> {
        if decimals < 0 {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                "decimals must be non-negative",
            ));
        }
        let multiplier = 10_f64.powi(decimals);
        Ok((num * multiplier).round() / multiplier)
    }
}

impl FilterFunction for Round {
    const NAME: &'static str = "round";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "round",
        category: "math",
        description: "Round a number to N decimal places",
        arguments: &[
            NUMBER_ARG,
            ArgumentMetadata {
                name: "decimals",
                arg_type: "integer",
                required: false,
                default: Some("0"),
                description: "Number of decimal places",
            },
        ],
        return_type: "number",
        examples: &[
            "{{ round(number=3.14159, decimals=2) }}",
            "{{ 3.14159 | round(decimals=2) }}",
        ],
        syntax: SyntaxVariants::FUNCTION_AND_FILTER,
    };

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let number: Value = kwargs.get("number")?;
        let decimals: i32 = kwargs.get("decimals").unwrap_or(0);
        let num = extract_number(&number, "round")?;
        Ok(smart_number(Self::compute(num, decimals)?))
    }

    fn call_as_filter(value: &Value, kwargs: Kwargs) -> Result<Value, Error> {
        let decimals: i32 = kwargs.get("decimals").unwrap_or(0);
        let num = extract_number(value, "round")?;
        Ok(smart_number(Self::compute(num, decimals)?))
    }
}

// ============================================
// Ceil
// ============================================

/// Round up to the nearest integer.
pub struct Ceil;

impl Ceil {
    fn compute(num: f64) -> i64 {
        num.ceil() as i64
    }
}

impl FilterFunction for Ceil {
    const NAME: &'static str = "ceil";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "ceil",
        category: "math",
        description: "Round up to the nearest integer",
        arguments: &[NUMBER_ARG],
        return_type: "integer",
        examples: &["{{ ceil(number=3.1) }}", "{{ 3.1 | ceil }}"],
        syntax: SyntaxVariants::FUNCTION_AND_FILTER,
    };

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let number: Value = kwargs.get("number")?;
        let num = extract_number(&number, "ceil")?;
        Ok(Value::from(Self::compute(num)))
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        let num = extract_number(value, "ceil")?;
        Ok(Value::from(Self::compute(num)))
    }
}

// ============================================
// Floor
// ============================================

/// Round down to the nearest integer.
pub struct Floor;

impl Floor {
    fn compute(num: f64) -> i64 {
        num.floor() as i64
    }
}

impl FilterFunction for Floor {
    const NAME: &'static str = "floor";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "floor",
        category: "math",
        description: "Round down to the nearest integer",
        arguments: &[NUMBER_ARG],
        return_type: "integer",
        examples: &["{{ floor(number=3.9) }}", "{{ 3.9 | floor }}"],
        syntax: SyntaxVariants::FUNCTION_AND_FILTER,
    };

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let number: Value = kwargs.get("number")?;
        let num = extract_number(&number, "floor")?;
        Ok(Value::from(Self::compute(num)))
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        let num = extract_number(value, "floor")?;
        Ok(Value::from(Self::compute(num)))
    }
}
