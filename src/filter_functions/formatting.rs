//! Formatting functions that support both function and filter syntax.
//!
//! # Function Syntax
//! ```jinja
//! {{ filesizeformat(bytes=1048576) }}
//! {{ urlencode(string="hello world") }}
//! ```
//!
//! # Filter Syntax
//! ```jinja
//! {{ 1048576 | filesizeformat }}
//! {{ "hello world" | urlencode }}
//! ```

use super::FilterFunction;
use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};

/// Helper to extract string from Value
fn extract_string(value: &Value, fn_name: &str) -> Result<String, Error> {
    value.as_str().map(|s| s.to_string()).ok_or_else(|| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("{} requires a string, found: {}", fn_name, value),
        )
    })
}

// ============================================
// Filesizeformat
// ============================================

/// Format file size in human-readable format (bytes, KB, MB, GB, etc.)
///
/// # Function Syntax
/// ```jinja
/// {{ filesizeformat(bytes=1024) }}
/// {# Output: 1 KB #}
///
/// {{ filesizeformat(bytes=1048576) }}
/// {# Output: 1 MB #}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ 1024 | filesizeformat }}
/// {# Output: 1 KB #}
///
/// {{ file_size | filesizeformat }}
/// ```
pub struct Filesizeformat;

impl Filesizeformat {
    fn compute(bytes: f64) -> String {
        const UNITS: &[&str] = &["bytes", "KB", "MB", "GB", "TB", "PB"];
        const THRESHOLD: f64 = 1024.0;

        if bytes < THRESHOLD {
            return format!("{} bytes", bytes as i64);
        }

        let mut size = bytes;
        let mut unit_index = 0;

        while size >= THRESHOLD && unit_index < UNITS.len() - 1 {
            size /= THRESHOLD;
            unit_index += 1;
        }

        // Format with appropriate precision
        if (size - size.round()).abs() < 0.01 {
            format!("{:.0} {}", size, UNITS[unit_index])
        } else if size < 10.0 {
            format!("{:.2} {}", size, UNITS[unit_index])
        } else if size < 100.0 {
            format!("{:.1} {}", size, UNITS[unit_index])
        } else {
            format!("{:.0} {}", size, UNITS[unit_index])
        }
    }
}

impl FilterFunction for Filesizeformat {
    const NAME: &'static str = "filesizeformat";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let bytes: i64 = kwargs.get("bytes")?;
        Ok(Value::from(Self::compute(bytes as f64)))
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        let bytes = value.as_i64().ok_or_else(|| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("filesizeformat requires a number, found: {}", value),
            )
        })?;
        Ok(Value::from(Self::compute(bytes as f64)))
    }
}

// ============================================
// Urlencode
// ============================================

/// URL encode a string - encode special characters for use in URLs.
///
/// This uses percent-encoding for all non-alphanumeric characters.
///
/// # Function Syntax
/// ```jinja
/// {{ urlencode(string="hello world & foo=bar") }}
/// {# Output: hello%20world%20%26%20foo%3Dbar #}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ "hello world" | urlencode }}
/// {# Output: hello%20world #}
/// ```
///
/// # Note
/// See also `url_encode` which uses a slightly different encoding scheme
/// (preserves some additional characters like `_`, `.`, `-`, `~`).
pub struct Urlencode;

impl Urlencode {
    fn compute(input: &str) -> String {
        utf8_percent_encode(input, NON_ALPHANUMERIC).to_string()
    }
}

impl FilterFunction for Urlencode {
    const NAME: &'static str = "urlencode";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let input: String = kwargs.get("string")?;
        Ok(Value::from(Self::compute(&input)))
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        let input = extract_string(value, "urlencode")?;
        Ok(Value::from(Self::compute(&input)))
    }
}
