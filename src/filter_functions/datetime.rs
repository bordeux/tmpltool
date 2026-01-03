//! DateTime functions that support both function and filter syntax.
//!
//! # Function Syntax
//! ```jinja
//! {{ format_date(timestamp=ts, format="%Y-%m-%d") }}
//! {{ get_year(timestamp=ts) }}
//! ```
//!
//! # Filter Syntax
//! ```jinja
//! {{ ts | format_date(format="%Y-%m-%d") }}
//! {{ ts | get_year }}
//! ```
//!
//! # Chaining
//! ```jinja
//! {{ now() | format_date(format="%Y") }}
//! ```

use super::FilterFunction;
use chrono::{DateTime, Datelike, Timelike, Utc};
use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};

/// Helper to extract timestamp from Value
fn extract_timestamp(value: &Value, fn_name: &str) -> Result<i64, Error> {
    // Try direct i64 conversion first
    if let Some(ts) = value.as_i64() {
        return Ok(ts);
    }

    // Try to convert via serde for other numeric types
    let json_value: serde_json::Value = serde_json::to_value(value).map_err(|e| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("{}: failed to convert value: {}", fn_name, e),
        )
    })?;

    json_value.as_i64().ok_or_else(|| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("{} requires a numeric timestamp, found: {}", fn_name, value),
        )
    })
}

/// Helper to convert timestamp to DateTime
fn timestamp_to_datetime(timestamp: i64, _fn_name: &str) -> Result<DateTime<Utc>, Error> {
    DateTime::from_timestamp(timestamp, 0).ok_or_else(|| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("Invalid timestamp: {}", timestamp),
        )
    })
}

// ============================================
// FormatDate
// ============================================

/// Format a Unix timestamp with a custom format string.
///
/// # Function Syntax
/// ```jinja
/// {{ format_date(timestamp=1704067200, format="%Y-%m-%d") }}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ 1704067200 | format_date(format="%Y-%m-%d") }}
/// {{ now() | format_date(format="%B %d, %Y") }}
/// ```
pub struct FormatDate;

impl FormatDate {
    fn compute(timestamp: i64, format: &str) -> Result<Value, Error> {
        let dt = timestamp_to_datetime(timestamp, "format_date")?;
        let formatted = dt.format(format).to_string();
        Ok(Value::from(formatted))
    }
}

impl FilterFunction for FormatDate {
    const NAME: &'static str = "format_date";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let timestamp: i64 = kwargs.get("timestamp")?;
        let format: String = kwargs
            .get("format")
            .unwrap_or_else(|_| "%Y-%m-%d %H:%M:%S".to_string());
        Self::compute(timestamp, &format)
    }

    fn call_as_filter(value: &Value, kwargs: Kwargs) -> Result<Value, Error> {
        let timestamp = extract_timestamp(value, "format_date")?;
        let format: String = kwargs
            .get("format")
            .unwrap_or_else(|_| "%Y-%m-%d %H:%M:%S".to_string());
        Self::compute(timestamp, &format)
    }
}

// ============================================
// GetYear
// ============================================

/// Extract the year from a Unix timestamp.
///
/// # Function Syntax
/// ```jinja
/// {{ get_year(timestamp=1704067200) }}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ 1704067200 | get_year }}
/// {{ now() | get_year }}
/// ```
pub struct GetYear;

impl GetYear {
    fn compute(timestamp: i64) -> Result<Value, Error> {
        let dt = timestamp_to_datetime(timestamp, "get_year")?;
        Ok(Value::from(dt.year()))
    }
}

impl FilterFunction for GetYear {
    const NAME: &'static str = "get_year";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let timestamp: i64 = kwargs.get("timestamp")?;
        Self::compute(timestamp)
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        let timestamp = extract_timestamp(value, "get_year")?;
        Self::compute(timestamp)
    }
}

// ============================================
// GetMonth
// ============================================

/// Extract the month from a Unix timestamp (1-12).
///
/// # Function Syntax
/// ```jinja
/// {{ get_month(timestamp=1704067200) }}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ 1704067200 | get_month }}
/// {{ now() | get_month }}
/// ```
pub struct GetMonth;

impl GetMonth {
    fn compute(timestamp: i64) -> Result<Value, Error> {
        let dt = timestamp_to_datetime(timestamp, "get_month")?;
        Ok(Value::from(dt.month()))
    }
}

impl FilterFunction for GetMonth {
    const NAME: &'static str = "get_month";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let timestamp: i64 = kwargs.get("timestamp")?;
        Self::compute(timestamp)
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        let timestamp = extract_timestamp(value, "get_month")?;
        Self::compute(timestamp)
    }
}

// ============================================
// GetDay
// ============================================

/// Extract the day from a Unix timestamp (1-31).
///
/// # Function Syntax
/// ```jinja
/// {{ get_day(timestamp=1704067200) }}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ 1704067200 | get_day }}
/// {{ now() | get_day }}
/// ```
pub struct GetDay;

impl GetDay {
    fn compute(timestamp: i64) -> Result<Value, Error> {
        let dt = timestamp_to_datetime(timestamp, "get_day")?;
        Ok(Value::from(dt.day()))
    }
}

impl FilterFunction for GetDay {
    const NAME: &'static str = "get_day";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let timestamp: i64 = kwargs.get("timestamp")?;
        Self::compute(timestamp)
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        let timestamp = extract_timestamp(value, "get_day")?;
        Self::compute(timestamp)
    }
}

// ============================================
// GetHour
// ============================================

/// Extract the hour from a Unix timestamp (0-23).
///
/// # Function Syntax
/// ```jinja
/// {{ get_hour(timestamp=1704067200) }}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ 1704067200 | get_hour }}
/// {{ now() | get_hour }}
/// ```
pub struct GetHour;

impl GetHour {
    fn compute(timestamp: i64) -> Result<Value, Error> {
        let dt = timestamp_to_datetime(timestamp, "get_hour")?;
        Ok(Value::from(dt.hour()))
    }
}

impl FilterFunction for GetHour {
    const NAME: &'static str = "get_hour";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let timestamp: i64 = kwargs.get("timestamp")?;
        Self::compute(timestamp)
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        let timestamp = extract_timestamp(value, "get_hour")?;
        Self::compute(timestamp)
    }
}

// ============================================
// GetMinute
// ============================================

/// Extract the minute from a Unix timestamp (0-59).
///
/// # Function Syntax
/// ```jinja
/// {{ get_minute(timestamp=1704067200) }}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ 1704067200 | get_minute }}
/// {{ now() | get_minute }}
/// ```
pub struct GetMinute;

impl GetMinute {
    fn compute(timestamp: i64) -> Result<Value, Error> {
        let dt = timestamp_to_datetime(timestamp, "get_minute")?;
        Ok(Value::from(dt.minute()))
    }
}

impl FilterFunction for GetMinute {
    const NAME: &'static str = "get_minute";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let timestamp: i64 = kwargs.get("timestamp")?;
        Self::compute(timestamp)
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        let timestamp = extract_timestamp(value, "get_minute")?;
        Self::compute(timestamp)
    }
}

// ============================================
// GetSecond
// ============================================

/// Extract the second from a Unix timestamp (0-59).
///
/// # Function Syntax
/// ```jinja
/// {{ get_second(timestamp=1704067200) }}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ 1704067200 | get_second }}
/// {{ now() | get_second }}
/// ```
pub struct GetSecond;

impl GetSecond {
    fn compute(timestamp: i64) -> Result<Value, Error> {
        let dt = timestamp_to_datetime(timestamp, "get_second")?;
        Ok(Value::from(dt.second()))
    }
}

impl FilterFunction for GetSecond {
    const NAME: &'static str = "get_second";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let timestamp: i64 = kwargs.get("timestamp")?;
        Self::compute(timestamp)
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        let timestamp = extract_timestamp(value, "get_second")?;
        Self::compute(timestamp)
    }
}
