/// Validation functions
///
/// Provides functions for validating various string formats:
/// - is_email: Validate email address format
/// - is_url: Validate URL format
/// - is_ip: Validate IP address (IPv4 or IPv6)
/// - is_uuid: Validate UUID format
/// - matches_regex: Check if string matches regex pattern
use regex::Regex;
use minijinja::{Error, ErrorKind, Value};
use minijinja::value::Kwargs;

/// Validate email address format
///
/// # Example
///
/// ```jinja
/// {{ is_email(string="user@example.com") }}
/// ```
pub fn is_email_fn(kwargs: Kwargs) -> Result<Value, Error> {
    // Extract string from kwargs
    let string: String = kwargs.get("string")?;

    // Simple email validation regex
    // This is not exhaustive but covers most common cases
    let email_re = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    Ok(Value::from(email_re.is_match(&string)))
}

/// Validate URL format
///
/// # Example
///
/// ```jinja
/// {{ is_url(string="https://example.com") }}
/// ```
pub fn is_url_fn(kwargs: Kwargs) -> Result<Value, Error> {
    // Extract string from kwargs
    let string: String = kwargs.get("string")?;

    // URL validation regex - supports http(s), ftp, file schemes
    let url_re = Regex::new(
        r"^(https?|ftp|file)://[-A-Za-z0-9+&@#/%?=~_|!:,.;]*[-A-Za-z0-9+&@#/%=~_|]$",
    )
    .unwrap();
    Ok(Value::from(url_re.is_match(&string)))
}

/// Validate IP address (IPv4 or IPv6)
///
/// # Example
///
/// ```jinja
/// {{ is_ip(string="192.168.1.1") }}
/// {{ is_ip(string="::1") }}
/// ```
pub fn is_ip_fn(kwargs: Kwargs) -> Result<Value, Error> {
    // Extract string from kwargs
    let string: String = kwargs.get("string")?;

    // Try parsing as standard library's IP address types
    Ok(Value::from(string.parse::<std::net::IpAddr>().is_ok()))
}

/// Validate UUID format
///
/// # Example
///
/// ```jinja
/// {{ is_uuid(string="550e8400-e29b-41d4-a716-446655440000") }}
/// ```
pub fn is_uuid_fn(kwargs: Kwargs) -> Result<Value, Error> {
    // Extract string from kwargs
    let string: String = kwargs.get("string")?;

    // UUID validation regex (supports all UUID versions)
    let uuid_re = Regex::new(
        r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$",
    )
    .unwrap();
    Ok(Value::from(uuid_re.is_match(&string)))
}

/// Check if string matches regex pattern
///
/// # Example
///
/// ```jinja
/// {{ matches_regex(pattern="^[A-Z]+$", string="HELLO") }}
/// ```
pub fn matches_regex_fn(kwargs: Kwargs) -> Result<Value, Error> {
    // Extract parameters from kwargs
    let pattern: String = kwargs.get("pattern")?;
    let string: String = kwargs.get("string")?;

    // Compile and match regex
    let re = Regex::new(&pattern)
        .map_err(|e| Error::new(ErrorKind::InvalidOperation, format!("Invalid regex pattern '{}': {}", pattern, e)))?;

    Ok(Value::from(re.is_match(&string)))
}
