/// Random generation functions for templates
///
/// Provides functions for generating random numbers and strings
use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};
use rand::Rng;

/// Generate random number in range
///
/// Replacement for Tera's built-in get_random() function
///
/// # Arguments
///
/// * `start` - Start of range (inclusive), defaults to 0
/// * `end` - End of range (exclusive), defaults to 100
///
/// # Example
///
/// ```jinja
/// {{ get_random() }}
/// {{ get_random(start=1, end=10) }}
/// ```
pub fn get_random_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let start: i64 = kwargs.get("start").unwrap_or(0);
    let end: i64 = kwargs.get("end").unwrap_or(100);

    if start >= end {
        return Err(Error::new(
            ErrorKind::InvalidOperation,
            format!("start ({}) must be less than end ({})", start, end),
        ));
    }

    let mut rng = rand::rng();
    let random = rng.random_range(start..end);

    Ok(Value::from(random))
}

/// Character set presets for random string generation
const CHARSET_ALPHANUMERIC: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
const CHARSET_ALPHABETIC: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const CHARSET_LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const CHARSET_UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const CHARSET_NUMERIC: &str = "0123456789";
const CHARSET_HEX: &str = "0123456789abcdef";
const CHARSET_HEX_UPPER: &str = "0123456789ABCDEF";

/// Generate random string
///
/// Generates random strings with customizable length and character sets
///
/// # Arguments
///
/// * `length` - Length of the random string
/// * `charset` - Optional character set (defaults to alphanumeric)
///
/// Available charset presets:
/// - `alphanumeric` (default): A-Z, a-z, 0-9
/// - `alphabetic` or `alpha`: A-Z, a-z
/// - `lowercase` or `lower`: a-z
/// - `uppercase` or `upper`: A-Z
/// - `numeric` or `digits`: 0-9
/// - `hex` or `hexadecimal`: 0-9, a-f
/// - `hex_upper`: 0-9, A-F
/// - Custom string: Any custom character set
///
/// # Example
///
/// ```jinja
/// {{ random_string(length=16) }}
/// {{ random_string(length=8, charset="hex") }}
/// {{ random_string(length=10, charset="abc123") }}
/// ```
pub fn random_string_fn(kwargs: Kwargs) -> Result<Value, Error> {
    // Extract parameters from kwargs
    let length: u64 = kwargs.get("length")?;
    let charset: Option<String> = kwargs.get("charset").ok();

    if length == 0 {
        return Ok(Value::from(""));
    }

    if length > 10000 {
        return Err(Error::new(
            ErrorKind::InvalidOperation,
            "random_string length must be <= 10000 to prevent excessive memory usage",
        ));
    }

    // Get charset (optional, defaults to alphanumeric)
    let charset_owned = charset.unwrap_or_else(|| "alphanumeric".to_string());
    let charset_str = match charset_owned.as_str() {
        "alphanumeric" => CHARSET_ALPHANUMERIC,
        "alphabetic" | "alpha" => CHARSET_ALPHABETIC,
        "lowercase" | "lower" => CHARSET_LOWERCASE,
        "uppercase" | "upper" => CHARSET_UPPERCASE,
        "numeric" | "digits" => CHARSET_NUMERIC,
        "hex" | "hexadecimal" => CHARSET_HEX,
        "hex_upper" => CHARSET_HEX_UPPER,
        _ => &charset_owned, // Custom charset
    };

    if charset_str.is_empty() {
        return Err(Error::new(
            ErrorKind::InvalidOperation,
            "charset cannot be empty",
        ));
    }

    // Generate random string
    let mut rng = rand::rng();
    let charset_chars: Vec<char> = charset_str.chars().collect();
    let random_string: String = (0..length)
        .map(|_| {
            let idx = rng.random_range(0..charset_chars.len());
            charset_chars[idx]
        })
        .collect();

    Ok(Value::from(random_string))
}
