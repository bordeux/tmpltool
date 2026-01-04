//! Date and time functions for templates
//!
//! This module provides date/time functions:
//! - `now`: Get current timestamp
//! - `parse_date`: Parse date string to timestamp
//! - `date_add`: Add days to timestamp
//! - `date_diff`: Calculate difference between timestamps
//! - `timezone_convert`: Convert timestamp between timezones
//!
//! Note: format_date, get_year, get_month, get_day, get_hour, get_minute, get_second
//! are now in filter_functions/datetime.rs with dual function+filter syntax support.
//!
//! Note: is_leap_year is now in is_functions/datetime.rs with dual function+is-test syntax.

use super::metadata::{ArgumentMetadata, FunctionMetadata, SyntaxVariants};
use super::traits::Function;
use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, TimeZone, Utc};
use chrono_tz::Tz;
use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};

/// Get current Unix timestamp, optionally formatted
pub struct Now;

impl Function for Now {
    const NAME: &'static str = "now";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "now",
        category: "datetime",
        description: "Get current Unix timestamp (seconds since epoch), optionally formatted",
        arguments: &[ArgumentMetadata {
            name: "format",
            arg_type: "string",
            required: false,
            default: None,
            description: "Optional format string (e.g., %Y-%m-%d). If not provided, returns Unix timestamp",
        }],
        return_type: "integer|string",
        examples: &["{{ now() }}", "{{ now(format=\"%Y-%m-%d %H:%M:%S\") }}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let format: Option<String> = kwargs.get("format").ok();
        let now = Utc::now();

        match format {
            Some(fmt) => Ok(Value::from(now.format(&fmt).to_string())),
            None => Ok(Value::from(now.timestamp())),
        }
    }
}

/// Parse a date string into a Unix timestamp
pub struct ParseDate;

impl Function for ParseDate {
    const NAME: &'static str = "parse_date";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "parse_date",
        category: "datetime",
        description: "Parse a date string into a Unix timestamp",
        arguments: &[
            ArgumentMetadata {
                name: "string",
                arg_type: "string",
                required: true,
                default: None,
                description: "Date string to parse",
            },
            ArgumentMetadata {
                name: "format",
                arg_type: "string",
                required: true,
                default: None,
                description: "Format string matching the input (e.g., %Y-%m-%d)",
            },
        ],
        return_type: "integer",
        examples: &[
            "{{ parse_date(string=\"2024-01-01 12:00:00\", format=\"%Y-%m-%d %H:%M:%S\") }}",
            "{{ parse_date(string=\"01/15/2024\", format=\"%m/%d/%Y\") }}",
        ],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let date_string: String = kwargs.get("string")?;
        let format: String = kwargs.get("format")?;

        let naive_dt = if let Ok(dt) = NaiveDateTime::parse_from_str(&date_string, &format) {
            dt
        } else {
            let naive_date = NaiveDate::parse_from_str(&date_string, &format).map_err(|e| {
                Error::new(
                    ErrorKind::InvalidOperation,
                    format!(
                        "Failed to parse date '{}' with format '{}': {}",
                        date_string, format, e
                    ),
                )
            })?;
            naive_date.and_hms_opt(0, 0, 0).ok_or_else(|| {
                Error::new(
                    ErrorKind::InvalidOperation,
                    "Failed to create datetime at midnight".to_string(),
                )
            })?
        };

        let dt = DateTime::<Utc>::from_naive_utc_and_offset(naive_dt, Utc);
        Ok(Value::from(dt.timestamp()))
    }
}

/// Add days to a Unix timestamp
pub struct DateAdd;

impl Function for DateAdd {
    const NAME: &'static str = "date_add";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "date_add",
        category: "datetime",
        description: "Add days to a Unix timestamp",
        arguments: &[
            ArgumentMetadata {
                name: "timestamp",
                arg_type: "integer",
                required: true,
                default: None,
                description: "Unix timestamp in seconds",
            },
            ArgumentMetadata {
                name: "days",
                arg_type: "integer",
                required: true,
                default: None,
                description: "Number of days to add (can be negative)",
            },
        ],
        return_type: "integer",
        examples: &[
            "{{ date_add(timestamp=now(), days=7) }}",
            "{{ date_add(timestamp=now(), days=-30) }}",
        ],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let timestamp: i64 = kwargs.get("timestamp")?;
        let days: i64 = kwargs.get("days")?;

        let dt = DateTime::from_timestamp(timestamp, 0).ok_or_else(|| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Invalid timestamp: {}", timestamp),
            )
        })?;

        let new_dt = dt + Duration::days(days);
        Ok(Value::from(new_dt.timestamp()))
    }
}

/// Calculate the difference in days between two timestamps
pub struct DateDiff;

impl Function for DateDiff {
    const NAME: &'static str = "date_diff";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "date_diff",
        category: "datetime",
        description: "Calculate the difference in days between two timestamps",
        arguments: &[
            ArgumentMetadata {
                name: "timestamp1",
                arg_type: "integer",
                required: true,
                default: None,
                description: "First Unix timestamp",
            },
            ArgumentMetadata {
                name: "timestamp2",
                arg_type: "integer",
                required: true,
                default: None,
                description: "Second Unix timestamp",
            },
        ],
        return_type: "integer",
        examples: &["{{ date_diff(timestamp1=now(), timestamp2=yesterday) }}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let timestamp1: i64 = kwargs.get("timestamp1")?;
        let timestamp2: i64 = kwargs.get("timestamp2")?;

        let dt1 = DateTime::from_timestamp(timestamp1, 0).ok_or_else(|| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Invalid timestamp1: {}", timestamp1),
            )
        })?;

        let dt2 = DateTime::from_timestamp(timestamp2, 0).ok_or_else(|| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Invalid timestamp2: {}", timestamp2),
            )
        })?;

        let duration = dt1.signed_duration_since(dt2);
        let days = duration.num_days();

        Ok(Value::from(days))
    }
}

/// Convert a timestamp from one timezone to another
pub struct TimezoneConvert;

impl Function for TimezoneConvert {
    const NAME: &'static str = "timezone_convert";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "timezone_convert",
        category: "datetime",
        description: "Convert a timestamp from one timezone to another",
        arguments: &[
            ArgumentMetadata {
                name: "timestamp",
                arg_type: "integer",
                required: true,
                default: None,
                description: "Unix timestamp in seconds",
            },
            ArgumentMetadata {
                name: "from_tz",
                arg_type: "string",
                required: true,
                default: None,
                description: "Source timezone (e.g., UTC, America/New_York)",
            },
            ArgumentMetadata {
                name: "to_tz",
                arg_type: "string",
                required: true,
                default: None,
                description: "Target timezone (e.g., Europe/London, Asia/Tokyo)",
            },
        ],
        return_type: "integer",
        examples: &[
            "{{ timezone_convert(timestamp=now(), from_tz=\"UTC\", to_tz=\"America/New_York\") }}",
        ],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let timestamp: i64 = kwargs.get("timestamp")?;
        let from_tz_str: String = kwargs.get("from_tz")?;
        let to_tz_str: String = kwargs.get("to_tz")?;

        let from_tz: Tz = from_tz_str.parse().map_err(|_| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Invalid timezone: {}", from_tz_str),
            )
        })?;

        let to_tz: Tz = to_tz_str.parse().map_err(|_| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Invalid timezone: {}", to_tz_str),
            )
        })?;

        let dt_utc = DateTime::from_timestamp(timestamp, 0).ok_or_else(|| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Invalid timestamp: {}", timestamp),
            )
        })?;

        let dt_from = from_tz.from_utc_datetime(&dt_utc.naive_utc());
        let dt_to = dt_from.with_timezone(&to_tz);

        Ok(Value::from(dt_to.timestamp()))
    }
}
