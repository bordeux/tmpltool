use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};
/// Random string generation function
///
/// Generates random strings with customizable length and character sets
use rand::Rng;

/// Character set presets
const CHARSET_ALPHANUMERIC: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
const CHARSET_ALPHABETIC: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const CHARSET_LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const CHARSET_UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const CHARSET_NUMERIC: &str = "0123456789";
const CHARSET_HEX: &str = "0123456789abcdef";
const CHARSET_HEX_UPPER: &str = "0123456789ABCDEF";

/// Generate random string
///
/// # Arguments
///
/// * `length` - Length of the random string
/// * `charset` - Optional character set (defaults to alphanumeric)
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
