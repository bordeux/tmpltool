//! URL functions that support both function and filter syntax.
//!
//! # Function Syntax
//! ```jinja
//! {{ url_encode(string="hello world") }}
//! {{ url_decode(string="hello%20world") }}
//! {{ parse_url(url="https://example.com/path") }}
//! ```
//!
//! # Filter Syntax
//! ```jinja
//! {{ "hello world" | url_encode }}
//! {{ "hello%20world" | url_decode }}
//! {{ "https://example.com/path" | parse_url }}
//! ```
//!
//! # Chaining
//! ```jinja
//! {{ user_input | url_encode }}
//! ```

use super::FilterFunction;
use crate::functions::metadata::{ArgumentMetadata, FunctionMetadata, SyntaxVariants};
use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};
use std::collections::BTreeMap;
use url::Url;

/// Common metadata for string argument
const STRING_ARG: ArgumentMetadata = ArgumentMetadata {
    name: "string",
    arg_type: "string",
    required: true,
    default: None,
    description: "The string to process",
};

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
// UrlEncode
// ============================================

/// URL-encode a string for safe use in URLs.
///
/// # Function Syntax
/// ```jinja
/// {{ url_encode(string="hello world") }}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ "hello world" | url_encode }}
/// {{ user_input | url_encode }}
/// ```
pub struct UrlEncode;

impl UrlEncode {
    fn compute(input: &str) -> Value {
        Value::from(urlencoding::encode(input).to_string())
    }
}

impl FilterFunction for UrlEncode {
    const NAME: &'static str = "url_encode";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "url_encode",
        category: "url",
        description: "URL-encode a string for safe use in URLs",
        arguments: &[STRING_ARG],
        return_type: "string",
        examples: &[
            "{{ url_encode(string=\"hello world\") }}",
            "{{ \"hello world\" | url_encode }}",
        ],
        syntax: SyntaxVariants::FUNCTION_AND_FILTER,
    };

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let input: String = kwargs.get("string")?;
        Ok(Self::compute(&input))
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        let input = extract_string(value, "url_encode")?;
        Ok(Self::compute(&input))
    }
}

// ============================================
// UrlDecode
// ============================================

/// URL-decode a percent-encoded string.
///
/// # Function Syntax
/// ```jinja
/// {{ url_decode(string="hello%20world") }}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ "hello%20world" | url_decode }}
/// {{ encoded_value | url_decode }}
/// ```
pub struct UrlDecode;

impl UrlDecode {
    fn compute(input: &str) -> Result<Value, Error> {
        let decoded = urlencoding::decode(input).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Failed to decode URL string: {}", e),
            )
        })?;
        Ok(Value::from(decoded.to_string()))
    }
}

impl FilterFunction for UrlDecode {
    const NAME: &'static str = "url_decode";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "url_decode",
        category: "url",
        description: "URL-decode a percent-encoded string",
        arguments: &[STRING_ARG],
        return_type: "string",
        examples: &[
            "{{ url_decode(string=\"hello%20world\") }}",
            "{{ \"hello%20world\" | url_decode }}",
        ],
        syntax: SyntaxVariants::FUNCTION_AND_FILTER,
    };

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let input: String = kwargs.get("string")?;
        Self::compute(&input)
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        let input = extract_string(value, "url_decode")?;
        Self::compute(&input)
    }
}

// ============================================
// ParseUrl
// ============================================

/// Parse a URL into its components.
///
/// Returns an object with: scheme, host, port, path, query, fragment, username, password
///
/// # Function Syntax
/// ```jinja
/// {% set parts = parse_url(url="https://example.com:8080/path?q=1") %}
/// {{ parts.host }}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {% set parts = "https://example.com:8080/path?q=1" | parse_url %}
/// {{ parts.host }}
/// ```
pub struct ParseUrl;

impl ParseUrl {
    fn compute(url_str: &str) -> Result<Value, Error> {
        let parsed = Url::parse(url_str).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Failed to parse URL '{}': {}", url_str, e),
            )
        })?;

        let mut result = BTreeMap::new();
        result.insert("scheme".to_string(), Value::from(parsed.scheme()));
        result.insert(
            "host".to_string(),
            Value::from(parsed.host_str().unwrap_or("")),
        );
        result.insert(
            "port".to_string(),
            Value::from(parsed.port().or_else(|| parsed.port_or_known_default())),
        );
        result.insert("path".to_string(), Value::from(parsed.path()));
        result.insert(
            "query".to_string(),
            Value::from(parsed.query().unwrap_or("")),
        );
        result.insert(
            "fragment".to_string(),
            Value::from(parsed.fragment().unwrap_or("")),
        );
        result.insert("username".to_string(), Value::from(parsed.username()));
        result.insert(
            "password".to_string(),
            Value::from(parsed.password().unwrap_or("")),
        );

        Ok(Value::from_object(result))
    }
}

impl FilterFunction for ParseUrl {
    const NAME: &'static str = "parse_url";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "parse_url",
        category: "url",
        description: "Parse a URL into its components (scheme, host, port, path, query, fragment)",
        arguments: &[ArgumentMetadata {
            name: "url",
            arg_type: "string",
            required: true,
            default: None,
            description: "The URL to parse",
        }],
        return_type: "object",
        examples: &[
            "{{ parse_url(url=\"https://example.com:8080/path?q=1\") }}",
            "{{ \"https://example.com/path\" | parse_url }}",
        ],
        syntax: SyntaxVariants::FUNCTION_AND_FILTER,
    };

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let url_str: String = kwargs.get("url")?;
        Self::compute(&url_str)
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        let url_str = extract_string(value, "parse_url")?;
        Self::compute(&url_str)
    }
}
