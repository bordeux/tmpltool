/// Custom filters for MiniJinja templates
///
/// Provides filters that were available in Tera but not in standard Jinja2/MiniJinja
use minijinja::Value;
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};

/// Slugify a string - convert to lowercase, replace spaces with hyphens, remove special chars
///
/// # Arguments
///
/// * `value` - The string to slugify
///
/// # Example
///
/// ```jinja
/// {{ "Hello World!" | slugify }}  => "hello-world"
/// {{ "jane smith" | slugify }}  => "jane-smith"
/// ```
pub fn slugify_filter(value: &Value) -> Result<String, minijinja::Error> {
    let s = value.as_str().ok_or_else(|| {
        minijinja::Error::new(
            minijinja::ErrorKind::InvalidOperation,
            "slugify filter requires a string",
        )
    })?;

    let slug = s
        .to_lowercase()
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c
            } else if c.is_whitespace() || c == '-' || c == '_' {
                '-'
            } else {
                '\0' // Will be filtered out
            }
        })
        .filter(|&c| c != '\0')
        .collect::<String>()
        // Remove duplicate hyphens
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-");

    Ok(slug)
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use minijinja::Value;

    #[test]
    fn test_slugify_basic() {
        let value = Value::from("Hello World");
        assert_eq!(slugify_filter(&value).unwrap(), "hello-world");
    }

    #[test]
    fn test_slugify_with_special_chars() {
        let value = Value::from("jane smith!");
        assert_eq!(slugify_filter(&value).unwrap(), "jane-smith");
    }

    #[test]
    fn test_slugify_multiple_spaces() {
        let value = Value::from("foo   bar");
        assert_eq!(slugify_filter(&value).unwrap(), "foo-bar");
    }

    #[test]
    fn test_filesizeformat_bytes() {
        let value = Value::from(500);
        assert_eq!(filesizeformat_filter(&value).unwrap(), "500 bytes");
    }

    #[test]
    fn test_filesizeformat_kb() {
        let value = Value::from(2048);
        assert_eq!(filesizeformat_filter(&value).unwrap(), "2 KB");
    }

    #[test]
    fn test_filesizeformat_mb() {
        let value = Value::from(1048576);
        assert_eq!(filesizeformat_filter(&value).unwrap(), "1 MB");
    }
}
