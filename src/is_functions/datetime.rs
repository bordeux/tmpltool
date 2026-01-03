//! DateTime is-functions for tmpltool
//!
//! This module provides datetime validation functions that work with both syntaxes:
//! - Function syntax: `{{ is_leap_year(year=2024) }}`
//! - Is-test syntax: `{% if 2024 is leap_year %}`
//!
//! # Available DateTime Functions
//!
//! - `is_leap_year` / `leap_year` - Check if a year is a leap year
//!
//! # Example Usage
//!
//! ```jinja
//! {# Function syntax #}
//! {% if is_leap_year(year=2024) %}leap year{% endif %}
//!
//! {# Is-test syntax (preferred for readability) #}
//! {% if 2024 is leap_year %}leap year{% endif %}
//! ```

use crate::is_functions::IsFunction;
use minijinja::value::Kwargs;
use minijinja::{Environment, Error, Value};

/// Leap year validation is-function
///
/// Checks if a given year is a leap year using standard leap year rules:
/// - Divisible by 4, AND
/// - Either not divisible by 100, OR divisible by 400
///
/// # Function Syntax
/// ```jinja
/// {{ is_leap_year(year=2024) }}
/// {% if is_leap_year(year=year_var) %}...{% endif %}
/// ```
///
/// # Is-Test Syntax
/// ```jinja
/// {% if 2024 is leap_year %}leap year{% endif %}
/// {% if year_var is leap_year %}leap year{% endif %}
/// ```
pub struct LeapYear;

impl LeapYear {
    /// Check if a year is a leap year
    ///
    /// A year is a leap year if:
    /// - It is divisible by 4, AND
    /// - Either not divisible by 100, OR divisible by 400
    pub fn is_leap(year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }
}

impl IsFunction for LeapYear {
    const FUNCTION_NAME: &'static str = "is_leap_year";
    const IS_NAME: &'static str = "leap_year";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let year: i32 = kwargs.get("year")?;
        Ok(Value::from(Self::is_leap(year)))
    }

    fn call_as_is(value: &Value) -> bool {
        // Try to extract an integer from the value
        if let Some(n) = value.as_i64() {
            Self::is_leap(n as i32)
        } else if let Some(s) = value.as_str() {
            // Also support string representation of years
            s.parse::<i32>().map(Self::is_leap).unwrap_or(false)
        } else {
            false
        }
    }
}

/// Register all datetime is-functions with the MiniJinja environment
pub fn register_all(env: &mut Environment) {
    LeapYear::register(env);
}
