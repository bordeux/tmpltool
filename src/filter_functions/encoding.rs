//! Encoding functions that support both function and filter syntax.
//!
//! # Function Syntax
//! ```jinja
//! {{ base64_encode(string="hello") }}
//! {{ hex_encode(string="hello") }}
//! ```
//!
//! # Filter Syntax
//! ```jinja
//! {{ "hello" | base64_encode }}
//! {{ "hello" | hex_encode }}
//! ```
//!
//! # Chaining
//! ```jinja
//! {{ "hello" | base64_encode | sha256 }}
//! ```

use super::FilterFunction;
use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};

/// Base64 encode function.
///
/// # Function Syntax
/// ```jinja
/// {{ base64_encode(string="hello") }}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ "hello" | base64_encode }}
/// ```
pub struct Base64Encode;

impl Base64Encode {
    fn encode(input: &str) -> String {
        base64::Engine::encode(&base64::engine::general_purpose::STANDARD, input.as_bytes())
    }
}

impl FilterFunction for Base64Encode {
    const NAME: &'static str = "base64_encode";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let input: String = kwargs.get("string")?;
        Ok(Value::from(Self::encode(&input)))
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        let input = value.as_str().ok_or_else(|| {
            Error::new(
                ErrorKind::InvalidOperation,
                "base64_encode requires a string",
            )
        })?;
        Ok(Value::from(Self::encode(input)))
    }
}

/// Base64 decode function.
///
/// # Function Syntax
/// ```jinja
/// {{ base64_decode(string="SGVsbG8=") }}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ "SGVsbG8=" | base64_decode }}
/// ```
pub struct Base64Decode;

impl Base64Decode {
    fn decode(input: &str) -> Result<String, Error> {
        let decoded_bytes =
            base64::Engine::decode(&base64::engine::general_purpose::STANDARD, input.as_bytes())
                .map_err(|e| {
                    Error::new(
                        ErrorKind::InvalidOperation,
                        format!("Failed to decode base64: {}", e),
                    )
                })?;

        String::from_utf8(decoded_bytes).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Decoded base64 is not valid UTF-8: {}", e),
            )
        })
    }
}

impl FilterFunction for Base64Decode {
    const NAME: &'static str = "base64_decode";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let input: String = kwargs.get("string")?;
        Ok(Value::from(Self::decode(&input)?))
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        let input = value.as_str().ok_or_else(|| {
            Error::new(
                ErrorKind::InvalidOperation,
                "base64_decode requires a string",
            )
        })?;
        Ok(Value::from(Self::decode(input)?))
    }
}

/// Hex encode function.
///
/// # Function Syntax
/// ```jinja
/// {{ hex_encode(string="hello") }}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ "hello" | hex_encode }}
/// ```
pub struct HexEncode;

impl HexEncode {
    fn encode(input: &str) -> String {
        hex::encode(input.as_bytes())
    }
}

impl FilterFunction for HexEncode {
    const NAME: &'static str = "hex_encode";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let input: String = kwargs.get("string")?;
        Ok(Value::from(Self::encode(&input)))
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        let input = value.as_str().ok_or_else(|| {
            Error::new(ErrorKind::InvalidOperation, "hex_encode requires a string")
        })?;
        Ok(Value::from(Self::encode(input)))
    }
}

/// Hex decode function.
///
/// # Function Syntax
/// ```jinja
/// {{ hex_decode(string="68656c6c6f") }}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ "68656c6c6f" | hex_decode }}
/// ```
pub struct HexDecode;

impl HexDecode {
    fn decode(input: &str) -> Result<String, Error> {
        let decoded_bytes = hex::decode(input).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Failed to decode hex: {}", e),
            )
        })?;

        String::from_utf8(decoded_bytes).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Decoded hex is not valid UTF-8: {}", e),
            )
        })
    }
}

impl FilterFunction for HexDecode {
    const NAME: &'static str = "hex_decode";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let input: String = kwargs.get("string")?;
        Ok(Value::from(Self::decode(&input)?))
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        let input = value.as_str().ok_or_else(|| {
            Error::new(ErrorKind::InvalidOperation, "hex_decode requires a string")
        })?;
        Ok(Value::from(Self::decode(input)?))
    }
}

/// HTML escape function.
///
/// # Function Syntax
/// ```jinja
/// {{ escape_html(string="<script>") }}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ "<script>" | escape_html }}
/// ```
pub struct EscapeHtml;

impl EscapeHtml {
    fn escape(input: &str) -> String {
        input
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#x27;")
    }
}

impl FilterFunction for EscapeHtml {
    const NAME: &'static str = "escape_html";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let input: String = kwargs.get("string")?;
        Ok(Value::from(Self::escape(&input)))
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        let input = value.as_str().ok_or_else(|| {
            Error::new(ErrorKind::InvalidOperation, "escape_html requires a string")
        })?;
        Ok(Value::from(Self::escape(input)))
    }
}

/// XML escape function.
///
/// # Function Syntax
/// ```jinja
/// {{ escape_xml(string="<tag>") }}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ "<tag>" | escape_xml }}
/// ```
pub struct EscapeXml;

impl EscapeXml {
    fn escape(input: &str) -> String {
        input
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&apos;")
    }
}

impl FilterFunction for EscapeXml {
    const NAME: &'static str = "escape_xml";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let input: String = kwargs.get("string")?;
        Ok(Value::from(Self::escape(&input)))
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        let input = value.as_str().ok_or_else(|| {
            Error::new(ErrorKind::InvalidOperation, "escape_xml requires a string")
        })?;
        Ok(Value::from(Self::escape(input)))
    }
}

/// Shell escape function.
///
/// # Function Syntax
/// ```jinja
/// {{ escape_shell(string="hello world") }}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ "hello world" | escape_shell }}
/// ```
pub struct EscapeShell;

impl EscapeShell {
    fn escape(input: &str) -> String {
        // Use single quotes and escape any single quotes in the input
        let escaped = input.replace('\'', "'\\''");
        format!("'{}'", escaped)
    }
}

impl FilterFunction for EscapeShell {
    const NAME: &'static str = "escape_shell";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let input: String = kwargs.get("string")?;
        Ok(Value::from(Self::escape(&input)))
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        let input = value.as_str().ok_or_else(|| {
            Error::new(
                ErrorKind::InvalidOperation,
                "escape_shell requires a string",
            )
        })?;
        Ok(Value::from(Self::escape(input)))
    }
}
