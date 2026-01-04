//! Random generation functions for templates
//!
//! This module provides functions for generating random numbers and strings:
//! - `get_random`: Generate random integer in range
//! - `random_string`: Generate random string with customizable charset

use super::metadata::{ArgumentMetadata, FunctionMetadata, SyntaxVariants};
use super::traits::Function;
use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};
use rand::Rng;

/// Character set presets for random string generation
const CHARSET_ALPHANUMERIC: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
const CHARSET_ALPHABETIC: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const CHARSET_LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const CHARSET_UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const CHARSET_NUMERIC: &str = "0123456789";
const CHARSET_HEX: &str = "0123456789abcdef";
const CHARSET_HEX_UPPER: &str = "0123456789ABCDEF";

/// Generate random number in range
pub struct GetRandom;

impl Function for GetRandom {
    const NAME: &'static str = "get_random";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "get_random",
        category: "random",
        description: "Generate random integer in specified range",
        arguments: &[
            ArgumentMetadata {
                name: "start",
                arg_type: "integer",
                required: false,
                default: Some("0"),
                description: "Start of range (inclusive)",
            },
            ArgumentMetadata {
                name: "end",
                arg_type: "integer",
                required: false,
                default: Some("100"),
                description: "End of range (exclusive)",
            },
        ],
        return_type: "integer",
        examples: &["{{ get_random() }}", "{{ get_random(start=1, end=10) }}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
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
}

/// Generate random string
pub struct RandomString;

impl Function for RandomString {
    const NAME: &'static str = "random_string";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "random_string",
        category: "random",
        description: "Generate random string with customizable length and charset",
        arguments: &[
            ArgumentMetadata {
                name: "length",
                arg_type: "integer",
                required: true,
                default: None,
                description: "Length of the random string",
            },
            ArgumentMetadata {
                name: "charset",
                arg_type: "string",
                required: false,
                default: Some("alphanumeric"),
                description: "Character set: alphanumeric, alphabetic, lowercase, uppercase, numeric, hex, hex_upper, or custom",
            },
        ],
        return_type: "string",
        examples: &[
            "{{ random_string(length=16) }}",
            "{{ random_string(length=8, charset=\"hex\") }}",
            "{{ random_string(length=10, charset=\"abc123\") }}",
        ],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
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

        let charset_owned = charset.unwrap_or_else(|| "alphanumeric".to_string());
        let charset_str = match charset_owned.as_str() {
            "alphanumeric" => CHARSET_ALPHANUMERIC,
            "alphabetic" | "alpha" => CHARSET_ALPHABETIC,
            "lowercase" | "lower" => CHARSET_LOWERCASE,
            "uppercase" | "upper" => CHARSET_UPPERCASE,
            "numeric" | "digits" => CHARSET_NUMERIC,
            "hex" | "hexadecimal" => CHARSET_HEX,
            "hex_upper" => CHARSET_HEX_UPPER,
            _ => &charset_owned,
        };

        if charset_str.is_empty() {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                "charset cannot be empty",
            ));
        }

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
}
