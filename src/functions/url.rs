//! URL and HTTP utility functions for templates
//!
//! This module provides functions for working with URLs and HTTP authentication:
//! - `basic_auth`: Generate HTTP Basic Authentication headers
//! - `build_url`: Construct URLs from components
//! - `query_string`: Build URL query strings from objects
//!
//! Note: parse_url is now in filter_functions/url.rs with dual function+filter syntax support.

use super::metadata::{ArgumentMetadata, FunctionMetadata, SyntaxVariants};
use super::traits::Function;
use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};

/// Convert a MiniJinja Value (object) to a URL-encoded query string
///
/// This is a helper function used by both `query_string_fn` and `build_url_fn`
/// to avoid code duplication.
fn serialize_query_params(params: &Value) -> Result<String, Error> {
    // Convert to serde_json::Value for easier iteration
    let json_value: serde_json::Value = serde_json::to_value(params).map_err(|e| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("Failed to convert params: {}", e),
        )
    })?;

    if !json_value.is_object() {
        return Err(Error::new(
            ErrorKind::InvalidOperation,
            "query parameter must be an object",
        ));
    }

    let mut parts = Vec::new();

    // Iterate over object fields
    if let Some(obj) = json_value.as_object() {
        for (key, value) in obj {
            let encoded_key = urlencoding::encode(key);
            // Convert value to string properly (without JSON quotes)
            let value_str = match value {
                serde_json::Value::String(s) => s.clone(),
                serde_json::Value::Number(n) => n.to_string(),
                serde_json::Value::Bool(b) => b.to_string(),
                serde_json::Value::Null => String::from("null"),
                _ => value.to_string(),
            };
            let encoded_value = urlencoding::encode(&value_str);
            parts.push(format!("{}={}", encoded_key, encoded_value));
        }
    }

    Ok(parts.join("&"))
}

/// Generate HTTP Basic Authentication header value
pub struct BasicAuth;

impl Function for BasicAuth {
    const NAME: &'static str = "basic_auth";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "basic_auth",
        category: "url",
        description: "Generate HTTP Basic Authentication header value",
        arguments: &[
            ArgumentMetadata {
                name: "username",
                arg_type: "string",
                required: true,
                default: None,
                description: "The username for authentication",
            },
            ArgumentMetadata {
                name: "password",
                arg_type: "string",
                required: true,
                default: None,
                description: "The password for authentication",
            },
        ],
        return_type: "string",
        examples: &["Authorization: {{ basic_auth(username=\"admin\", password=\"secret\") }}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let username: String = kwargs.get("username")?;
        let password: String = kwargs.get("password")?;

        let credentials = format!("{}:{}", username, password);
        let encoded =
            base64::Engine::encode(&base64::engine::general_purpose::STANDARD, credentials);

        Ok(Value::from(format!("Basic {}", encoded)))
    }
}

/// Build a URL from components
pub struct BuildUrl;

impl Function for BuildUrl {
    const NAME: &'static str = "build_url";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "build_url",
        category: "url",
        description: "Build a URL from components",
        arguments: &[
            ArgumentMetadata {
                name: "host",
                arg_type: "string",
                required: true,
                default: None,
                description: "The hostname",
            },
            ArgumentMetadata {
                name: "scheme",
                arg_type: "string",
                required: false,
                default: Some("\"https\""),
                description: "URL scheme (default: \"https\")",
            },
            ArgumentMetadata {
                name: "port",
                arg_type: "integer",
                required: false,
                default: None,
                description: "Port number",
            },
            ArgumentMetadata {
                name: "path",
                arg_type: "string",
                required: false,
                default: Some("\"/\""),
                description: "Path component (default: \"/\")",
            },
            ArgumentMetadata {
                name: "query",
                arg_type: "string|object",
                required: false,
                default: None,
                description: "Query string or object to serialize",
            },
        ],
        return_type: "string",
        examples: &[
            "{{ build_url(host=\"api.example.com\", port=8080, path=\"/v1/users\") }}",
            "{{ build_url(host=\"api.example.com\", query={\"page\": 1}) }}",
        ],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let scheme: String = kwargs.get("scheme").unwrap_or_else(|_| "https".to_string());
        let host: String = kwargs.get("host")?;
        let port: Option<u16> = kwargs.get("port").ok();
        let path: Option<String> = kwargs.get("path").ok();
        let query: Option<Value> = kwargs.get("query").ok();

        // Start with scheme and host
        let mut url = format!("{}://{}", scheme, host);

        // Add port if specified
        if let Some(p) = port {
            url.push_str(&format!(":{}", p));
        }

        // Add path (default to "/" if not specified)
        let path_str = path.unwrap_or_else(|| "/".to_string());
        if !path_str.starts_with('/') {
            url.push('/');
        }
        url.push_str(&path_str);

        // Add query string if specified
        if let Some(q) = query {
            let query_str = if let Some(s) = q.as_str() {
                // Query is a string, use it directly
                s.to_string()
            } else {
                // Query is an object, serialize it
                serialize_query_params(&q)?
            };

            if !query_str.is_empty() {
                url.push('?');
                url.push_str(&query_str);
            }
        }

        Ok(Value::from(url))
    }
}

/// Build a URL query string from an object
pub struct QueryString;

impl Function for QueryString {
    const NAME: &'static str = "query_string";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "query_string",
        category: "url",
        description: "Build a URL query string from an object",
        arguments: &[ArgumentMetadata {
            name: "params",
            arg_type: "object",
            required: true,
            default: None,
            description: "Object containing key-value pairs for the query string",
        }],
        return_type: "string",
        examples: &["{{ query_string(params={\"name\": \"John\", \"age\": 30}) }}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let params: Value = kwargs.get("params")?;
        let query_str = serialize_query_params(&params)?;
        Ok(Value::from(query_str))
    }
}
