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
use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};

/// Helper to extract a numeric value from a MiniJinja Value
fn extract_number(value: &Value, fn_name: &str) -> Result<f64, Error> {
    // Try direct integer conversion first
    if let Some(n) = value.as_i64() {
        return Ok(n as f64);
    }

    // Try via serde_json for floats and other numeric types
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
///
/// # Function Syntax
/// ```jinja
/// {{ abs(number=-42) }}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ -42 | abs }}
/// {{ temperature_diff | abs }}
/// ```
pub struct Abs;

impl Abs {
    fn compute(num: f64) -> f64 {
        num.abs()
    }
}

impl FilterFunction for Abs {
    const NAME: &'static str = "abs";

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
///
/// # Function Syntax
/// ```jinja
/// {{ round(number=3.14159, decimals=2) }}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ 3.14159 | round }}
/// {{ 3.14159 | round(decimals=2) }}
/// {{ price | round(decimals=2) }}
/// ```
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
///
/// # Function Syntax
/// ```jinja
/// {{ ceil(number=3.1) }}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ 3.1 | ceil }}
/// {{ items_needed | ceil }}
/// ```
pub struct Ceil;

impl Ceil {
    fn compute(num: f64) -> i64 {
        num.ceil() as i64
    }
}

impl FilterFunction for Ceil {
    const NAME: &'static str = "ceil";

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
///
/// # Function Syntax
/// ```jinja
/// {{ floor(number=3.9) }}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ 3.9 | floor }}
/// {{ full_pages | floor }}
/// ```
pub struct Floor;

impl Floor {
    fn compute(num: f64) -> i64 {
        num.floor() as i64
    }
}

impl FilterFunction for Floor {
    const NAME: &'static str = "floor";

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
