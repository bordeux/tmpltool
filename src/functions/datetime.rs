/// Date and time functions for templates
use chrono::{DateTime, Datelike, Duration, NaiveDate, NaiveDateTime, TimeZone, Timelike, Utc};
use chrono_tz::Tz;
use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};

/// Get current timestamp in ISO 8601 format
///
/// Replacement for Tera's built-in now() function
///
/// Returns timestamp in ISO 8601 format: YYYY-MM-DDTHH:MM:SS.sss+00:00
///
/// # Example
///
/// ```jinja
/// {{ now() }}  => "2024-12-31T12:34:56.789+00:00"
/// ```
pub fn now_fn() -> Result<Value, Error> {
    let timestamp = Utc::now().to_rfc3339();
    Ok(Value::from(timestamp))
}

/// Format a Unix timestamp with a custom format string
///
/// # Arguments
///
/// * `timestamp` (required) - Unix timestamp in seconds
/// * `format` (optional) - Format string (default: "%Y-%m-%d %H:%M:%S")
///
/// Format specifiers: https://docs.rs/chrono/latest/chrono/format/strftime/index.html
///
/// # Example
///
/// ```jinja
/// {{ format_date(timestamp=1704067200) }}
/// {{ format_date(timestamp=1704067200, format="%Y-%m-%d") }}
/// {{ format_date(timestamp=1704067200, format="%B %d, %Y at %I:%M %p") }}
/// ```
pub fn format_date_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let timestamp: i64 = kwargs.get("timestamp")?;
    let format: String = kwargs
        .get("format")
        .unwrap_or_else(|_| "%Y-%m-%d %H:%M:%S".to_string());

    let dt = DateTime::from_timestamp(timestamp, 0).ok_or_else(|| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("Invalid timestamp: {}", timestamp),
        )
    })?;

    let formatted = dt.format(&format).to_string();
    Ok(Value::from(formatted))
}

/// Parse a date string into a Unix timestamp
///
/// # Arguments
///
/// * `string` (required) - Date string to parse
/// * `format` (required) - Format string matching the input
///
/// # Example
///
/// ```jinja
/// {{ parse_date(string="2024-01-01 12:00:00", format="%Y-%m-%d %H:%M:%S") }}
/// {{ parse_date(string="01/15/2024", format="%m/%d/%Y") }}
/// ```
pub fn parse_date_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let date_string: String = kwargs.get("string")?;
    let format: String = kwargs.get("format")?;

    // Try parsing as datetime first
    let naive_dt = if let Ok(dt) = NaiveDateTime::parse_from_str(&date_string, &format) {
        dt
    } else {
        // If that fails, try parsing as date-only and set time to midnight
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

/// Add days to a Unix timestamp
///
/// # Arguments
///
/// * `timestamp` (required) - Unix timestamp in seconds
/// * `days` (required) - Number of days to add (can be negative)
///
/// # Example
///
/// ```jinja
/// {{ date_add(timestamp=1704067200, days=7) }}
/// {{ date_add(timestamp=1704067200, days=-30) }}
/// ```
pub fn date_add_fn(kwargs: Kwargs) -> Result<Value, Error> {
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

/// Calculate the difference in days between two timestamps
///
/// # Arguments
///
/// * `timestamp1` (required) - First Unix timestamp in seconds
/// * `timestamp2` (required) - Second Unix timestamp in seconds
///
/// Returns the difference in days (timestamp1 - timestamp2)
///
/// # Example
///
/// ```jinja
/// {{ date_diff(timestamp1=1704067200, timestamp2=1704067200) }}  => 0
/// {{ date_diff(timestamp1=1704153600, timestamp2=1704067200) }}  => 1
/// ```
pub fn date_diff_fn(kwargs: Kwargs) -> Result<Value, Error> {
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

/// Extract the year from a Unix timestamp
///
/// # Arguments
///
/// * `timestamp` (required) - Unix timestamp in seconds
///
/// # Example
///
/// ```jinja
/// {{ get_year(timestamp=1704067200) }}  => 2024
/// ```
pub fn get_year_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let timestamp: i64 = kwargs.get("timestamp")?;

    let dt = DateTime::from_timestamp(timestamp, 0).ok_or_else(|| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("Invalid timestamp: {}", timestamp),
        )
    })?;

    Ok(Value::from(dt.year()))
}

/// Extract the month from a Unix timestamp (1-12)
///
/// # Arguments
///
/// * `timestamp` (required) - Unix timestamp in seconds
///
/// # Example
///
/// ```jinja
/// {{ get_month(timestamp=1704067200) }}  => 1
/// ```
pub fn get_month_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let timestamp: i64 = kwargs.get("timestamp")?;

    let dt = DateTime::from_timestamp(timestamp, 0).ok_or_else(|| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("Invalid timestamp: {}", timestamp),
        )
    })?;

    Ok(Value::from(dt.month()))
}

/// Extract the day from a Unix timestamp (1-31)
///
/// # Arguments
///
/// * `timestamp` (required) - Unix timestamp in seconds
///
/// # Example
///
/// ```jinja
/// {{ get_day(timestamp=1704067200) }}  => 1
/// ```
pub fn get_day_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let timestamp: i64 = kwargs.get("timestamp")?;

    let dt = DateTime::from_timestamp(timestamp, 0).ok_or_else(|| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("Invalid timestamp: {}", timestamp),
        )
    })?;

    Ok(Value::from(dt.day()))
}

/// Extract the hour from a Unix timestamp (0-23)
///
/// # Arguments
///
/// * `timestamp` (required) - Unix timestamp in seconds
///
/// # Example
///
/// ```jinja
/// {{ get_hour(timestamp=1704067200) }}  => 12
/// ```
pub fn get_hour_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let timestamp: i64 = kwargs.get("timestamp")?;

    let dt = DateTime::from_timestamp(timestamp, 0).ok_or_else(|| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("Invalid timestamp: {}", timestamp),
        )
    })?;

    Ok(Value::from(dt.hour()))
}

/// Extract the minute from a Unix timestamp (0-59)
///
/// # Arguments
///
/// * `timestamp` (required) - Unix timestamp in seconds
///
/// # Example
///
/// ```jinja
/// {{ get_minute(timestamp=1704067200) }}  => 0
/// ```
pub fn get_minute_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let timestamp: i64 = kwargs.get("timestamp")?;

    let dt = DateTime::from_timestamp(timestamp, 0).ok_or_else(|| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("Invalid timestamp: {}", timestamp),
        )
    })?;

    Ok(Value::from(dt.minute()))
}

/// Convert a timestamp from one timezone to another
///
/// # Arguments
///
/// * `timestamp` (required) - Unix timestamp in seconds
/// * `from_tz` (required) - Source timezone (e.g., "UTC", "America/New_York")
/// * `to_tz` (required) - Target timezone (e.g., "Europe/London", "Asia/Tokyo")
///
/// # Example
///
/// ```jinja
/// {{ timezone_convert(timestamp=1704067200, from_tz="UTC", to_tz="America/New_York") }}
/// {{ timezone_convert(timestamp=1704067200, from_tz="America/Los_Angeles", to_tz="Europe/Paris") }}
/// ```
pub fn timezone_convert_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let timestamp: i64 = kwargs.get("timestamp")?;
    let from_tz_str: String = kwargs.get("from_tz")?;
    let to_tz_str: String = kwargs.get("to_tz")?;

    // Parse timezones
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

    // Convert timestamp to datetime in source timezone
    let dt_utc = DateTime::from_timestamp(timestamp, 0).ok_or_else(|| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("Invalid timestamp: {}", timestamp),
        )
    })?;

    // Convert to target timezone
    let dt_from = from_tz.from_utc_datetime(&dt_utc.naive_utc());
    let dt_to = dt_from.with_timezone(&to_tz);

    // Return new timestamp
    Ok(Value::from(dt_to.timestamp()))
}

/// Check if a year is a leap year
///
/// # Arguments
///
/// * `year` (required) - Year to check
///
/// # Example
///
/// ```jinja
/// {{ is_leap_year(year=2024) }}  => true
/// {{ is_leap_year(year=2023) }}  => false
/// ```
pub fn is_leap_year_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let year: i32 = kwargs.get("year")?;

    // Leap year rules:
    // - Divisible by 4: leap year
    // - Divisible by 100: not a leap year
    // - Divisible by 400: leap year
    let is_leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);

    Ok(Value::from(is_leap))
}
