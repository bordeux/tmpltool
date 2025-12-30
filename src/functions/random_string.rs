/// Random string generation function
///
/// Generates random strings with customizable length and character sets
use rand::Rng;
use std::collections::HashMap;
use tera::{Function, Result, Value, to_value};

/// Character set presets
const CHARSET_ALPHANUMERIC: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
const CHARSET_ALPHABETIC: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const CHARSET_LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const CHARSET_UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const CHARSET_NUMERIC: &str = "0123456789";
const CHARSET_HEX: &str = "0123456789abcdef";
const CHARSET_HEX_UPPER: &str = "0123456789ABCDEF";

/// Generate random string
pub struct RandomString;

impl Function for RandomString {
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value> {
        // Get length (required)
        let length = args.get("length").and_then(|v| v.as_u64()).ok_or_else(|| {
            tera::Error::msg("random_string requires a 'length' argument (e.g., length=16)")
        })?;

        if length == 0 {
            return to_value("")
                .map_err(|e| tera::Error::msg(format!("Failed to convert result: {}", e)));
        }

        if length > 10000 {
            return Err(tera::Error::msg(
                "random_string length must be <= 10000 to prevent excessive memory usage",
            ));
        }

        // Get charset (optional, defaults to alphanumeric)
        let charset = if let Some(charset_value) = args.get("charset") {
            if let Some(charset_str) = charset_value.as_str() {
                // Check for preset charsets
                match charset_str {
                    "alphanumeric" => CHARSET_ALPHANUMERIC,
                    "alphabetic" | "alpha" => CHARSET_ALPHABETIC,
                    "lowercase" | "lower" => CHARSET_LOWERCASE,
                    "uppercase" | "upper" => CHARSET_UPPERCASE,
                    "numeric" | "digits" => CHARSET_NUMERIC,
                    "hex" | "hexadecimal" => CHARSET_HEX,
                    "hex_upper" => CHARSET_HEX_UPPER,
                    _ => charset_str, // Custom charset
                }
            } else {
                return Err(tera::Error::msg(
                    "charset must be a string (e.g., charset=\"alphanumeric\" or charset=\"abc123\")",
                ));
            }
        } else {
            CHARSET_ALPHANUMERIC
        };

        if charset.is_empty() {
            return Err(tera::Error::msg("charset cannot be empty"));
        }

        // Generate random string
        let mut rng = rand::rng();
        let charset_chars: Vec<char> = charset.chars().collect();
        let random_string: String = (0..length)
            .map(|_| {
                let idx = rng.random_range(0..charset_chars.len());
                charset_chars[idx]
            })
            .collect();

        to_value(&random_string)
            .map_err(|e| tera::Error::msg(format!("Failed to convert result: {}", e)))
    }
}
