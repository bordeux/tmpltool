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
use crate::functions::metadata::{ArgumentMetadata, FunctionMetadata, SyntaxVariants};
use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};

/// Common metadata for single-argument string encoding functions
const STRING_ARG: ArgumentMetadata = ArgumentMetadata {
    name: "string",
    arg_type: "string",
    required: true,
    default: None,
    description: "The string to process",
};

/// Base64 encode function.
pub struct Base64Encode;

impl Base64Encode {
    fn encode(input: &str) -> String {
        base64::Engine::encode(&base64::engine::general_purpose::STANDARD, input.as_bytes())
    }
}

impl FilterFunction for Base64Encode {
    const NAME: &'static str = "base64_encode";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "base64_encode",
        category: "encoding",
        description: "Encode a string to Base64",
        arguments: &[STRING_ARG],
        return_type: "string",
        examples: &[
            "{{ base64_encode(string=\"hello\") }}",
            "{{ \"hello\" | base64_encode }}",
        ],
        syntax: SyntaxVariants::FUNCTION_AND_FILTER,
    };

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
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "base64_decode",
        category: "encoding",
        description: "Decode a Base64 string",
        arguments: &[STRING_ARG],
        return_type: "string",
        examples: &[
            "{{ base64_decode(string=\"aGVsbG8=\") }}",
            "{{ \"aGVsbG8=\" | base64_decode }}",
        ],
        syntax: SyntaxVariants::FUNCTION_AND_FILTER,
    };

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
pub struct HexEncode;

impl HexEncode {
    fn encode(input: &str) -> String {
        hex::encode(input.as_bytes())
    }
}

impl FilterFunction for HexEncode {
    const NAME: &'static str = "hex_encode";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "hex_encode",
        category: "encoding",
        description: "Encode a string to hexadecimal",
        arguments: &[STRING_ARG],
        return_type: "string",
        examples: &[
            "{{ hex_encode(string=\"hello\") }}",
            "{{ \"hello\" | hex_encode }}",
        ],
        syntax: SyntaxVariants::FUNCTION_AND_FILTER,
    };

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
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "hex_decode",
        category: "encoding",
        description: "Decode a hexadecimal string",
        arguments: &[STRING_ARG],
        return_type: "string",
        examples: &[
            "{{ hex_decode(string=\"68656c6c6f\") }}",
            "{{ \"68656c6c6f\" | hex_decode }}",
        ],
        syntax: SyntaxVariants::FUNCTION_AND_FILTER,
    };

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
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "escape_html",
        category: "encoding",
        description: "Escape HTML special characters",
        arguments: &[STRING_ARG],
        return_type: "string",
        examples: &[
            "{{ escape_html(string=\"<script>alert('xss')</script>\") }}",
            "{{ \"<div>\" | escape_html }}",
        ],
        syntax: SyntaxVariants::FUNCTION_AND_FILTER,
    };

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
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "escape_xml",
        category: "encoding",
        description: "Escape XML special characters",
        arguments: &[STRING_ARG],
        return_type: "string",
        examples: &[
            "{{ escape_xml(string=\"<tag attr='value'>\") }}",
            "{{ \"<xml>\" | escape_xml }}",
        ],
        syntax: SyntaxVariants::FUNCTION_AND_FILTER,
    };

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
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "escape_shell",
        category: "encoding",
        description: "Escape shell special characters for safe command execution",
        arguments: &[STRING_ARG],
        return_type: "string",
        examples: &[
            "{{ escape_shell(string=\"file name.txt\") }}",
            "{{ user_input | escape_shell }}",
        ],
        syntax: SyntaxVariants::FUNCTION_AND_FILTER,
    };

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
