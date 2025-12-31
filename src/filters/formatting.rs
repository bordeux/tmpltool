/// Formatting filters for MiniJinja templates
use minijinja::Value;
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};

/// Format file size in human-readable format (bytes, KB, MB, GB, etc.)
///
/// # Arguments
///
/// * `value` - The file size in bytes (as number)
///
/// # Example
///
/// ```jinja
/// {{ 1024 | filesizeformat }}  => "1 KB"
/// {{ 1048576 | filesizeformat }}  => "1 MB"
/// ```
pub fn filesizeformat_filter(value: &Value) -> Result<String, minijinja::Error> {
    let bytes = if let Some(n) = value.as_i64() {
        n as f64
    } else {
        return Err(minijinja::Error::new(
            minijinja::ErrorKind::InvalidOperation,
            "filesizeformat filter requires a number",
        ));
    };

    const UNITS: &[&str] = &["bytes", "KB", "MB", "GB", "TB", "PB"];
    const THRESHOLD: f64 = 1024.0;

    if bytes < THRESHOLD {
        return Ok(format!("{} bytes", bytes as i64));
    }

    let mut size = bytes;
    let mut unit_index = 0;

    while size >= THRESHOLD && unit_index < UNITS.len() - 1 {
        size /= THRESHOLD;
        unit_index += 1;
    }

    // Format with appropriate precision
    // If it's a whole number (or very close to one), show without decimals
    if (size - size.round()).abs() < 0.01 {
        Ok(format!("{:.0} {}", size, UNITS[unit_index]))
    } else if size < 10.0 {
        Ok(format!("{:.2} {}", size, UNITS[unit_index]))
    } else if size < 100.0 {
        Ok(format!("{:.1} {}", size, UNITS[unit_index]))
    } else {
        Ok(format!("{:.0} {}", size, UNITS[unit_index]))
    }
}

/// URL encode a string - encode special characters for use in URLs
///
/// # Arguments
///
/// * `value` - The string to URL encode
///
/// # Example
///
/// ```jinja
/// {{ "hello world & foo=bar" | urlencode }}  => "hello%20world%20%26%20foo%3Dbar"
/// ```
pub fn urlencode_filter(value: &Value) -> Result<String, minijinja::Error> {
    let s = value.as_str().ok_or_else(|| {
        minijinja::Error::new(
            minijinja::ErrorKind::InvalidOperation,
            "urlencode filter requires a string",
        )
    })?;

    Ok(utf8_percent_encode(s, NON_ALPHANUMERIC).to_string())
}
