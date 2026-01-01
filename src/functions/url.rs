//! URL and HTTP utility functions for templates
//!
//! This module provides functions for working with URLs and HTTP authentication:
//! - `basic_auth`: Generate HTTP Basic Authentication headers
//! - `parse_url`: Parse URLs into components
//! - `build_url`: Construct URLs from components
//! - `query_string`: Build URL query strings from objects

use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};
use std::collections::BTreeMap;
use url::Url;

/// Generate HTTP Basic Authentication header value
///
/// # Arguments
///
/// * `username` - The username for authentication
/// * `password` - The password for authentication
///
/// # Returns
///
/// Returns the Base64-encoded "Basic" authentication header value
///
/// # Example
///
/// ```jinja
/// Authorization: {{ basic_auth(username="admin", password="secret") }}
/// ```
pub fn basic_auth_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let username: String = kwargs.get("username")?;
    let password: String = kwargs.get("password")?;

    let credentials = format!("{}:{}", username, password);
    let encoded = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, credentials);

    Ok(Value::from(format!("Basic {}", encoded)))
}

/// Parse a URL into its components
///
/// # Arguments
///
/// * `url` - The URL string to parse
///
/// # Returns
///
/// Returns an object with the following fields:
/// - `scheme`: The URL scheme (http, https, etc.)
/// - `host`: The hostname
/// - `port`: The port number (or default for scheme)
/// - `path`: The path component
/// - `query`: The query string (without ?)
/// - `fragment`: The fragment/hash (without #)
/// - `username`: Username from URL (if present)
/// - `password`: Password from URL (if present)
///
/// # Example
///
/// ```jinja
/// {% set url_parts = parse_url(url="https://user:pass@example.com:8080/path?foo=bar#section") %}
/// Scheme: {{ url_parts.scheme }}
/// Host: {{ url_parts.host }}
/// Port: {{ url_parts.port }}
/// ```
pub fn parse_url_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let url_str: String = kwargs.get("url")?;

    let parsed = Url::parse(&url_str).map_err(|e| {
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

/// Build a URL from components
///
/// # Arguments
///
/// * `scheme` - Optional URL scheme (default: "https")
/// * `host` - The hostname (required)
/// * `port` - Optional port number
/// * `path` - Optional path component (default: "/")
/// * `query` - Optional query string (string) or object (will be serialized)
///
/// # Returns
///
/// Returns the constructed URL string
///
/// # Example
///
/// ```jinja
/// {{ build_url(host="api.example.com", port=8080, path="/v1/users", query="limit=10") }}
/// {{ build_url(host="api.example.com", query={"page": 1, "limit": 20}) }}
/// ```
pub fn build_url_fn(kwargs: Kwargs) -> Result<Value, Error> {
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
            // Query is an object, serialize it using query_string logic
            let json_value: serde_json::Value = serde_json::to_value(&q).map_err(|e| {
                Error::new(
                    ErrorKind::InvalidOperation,
                    format!("Failed to convert query parameter: {}", e),
                )
            })?;

            if !json_value.is_object() {
                return Err(Error::new(
                    ErrorKind::InvalidOperation,
                    "query parameter must be a string or object",
                ));
            }

            let mut parts = Vec::new();
            if let Some(obj) = json_value.as_object() {
                for (key, value) in obj {
                    let encoded_key = urlencoding::encode(key);
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
            parts.join("&")
        };

        if !query_str.is_empty() {
            url.push('?');
            url.push_str(&query_str);
        }
    }

    Ok(Value::from(url))
}

/// Build a URL query string from an object
///
/// # Arguments
///
/// * `params` - An object containing key-value pairs for the query string
///
/// # Returns
///
/// Returns a URL-encoded query string (without leading ?)
///
/// # Example
///
/// ```jinja
/// {% set params = {"name": "John Doe", "age": 30, "city": "New York"} %}
/// {{ query_string(params=params) }}
/// ```
pub fn query_string_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let params: Value = kwargs.get("params")?;

    // Convert to serde_json::Value for easier iteration
    let json_value: serde_json::Value = serde_json::to_value(&params).map_err(|e| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("Failed to convert params: {}", e),
        )
    })?;

    if !json_value.is_object() {
        return Err(Error::new(
            ErrorKind::InvalidOperation,
            "query_string() requires an object for 'params' parameter",
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

    Ok(Value::from(parts.join("&")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_auth_simple() {
        let result = basic_auth_fn(Kwargs::from_iter(vec![
            ("username", Value::from("admin")),
            ("password", Value::from("secret")),
        ]))
        .unwrap();

        assert_eq!(result.to_string(), "Basic YWRtaW46c2VjcmV0");
    }

    #[test]
    fn test_parse_url_simple() {
        let result = parse_url_fn(Kwargs::from_iter(vec![(
            "url",
            Value::from("https://example.com/path"),
        )]))
        .unwrap();

        let obj = result.as_object().unwrap();
        assert_eq!(
            obj.get_value(&Value::from("scheme")).unwrap().as_str(),
            Some("https")
        );
        assert_eq!(
            obj.get_value(&Value::from("host")).unwrap().as_str(),
            Some("example.com")
        );
        assert_eq!(
            obj.get_value(&Value::from("path")).unwrap().as_str(),
            Some("/path")
        );
    }

    #[test]
    fn test_build_url_simple() {
        let result = build_url_fn(Kwargs::from_iter(vec![
            ("scheme", Value::from("https")),
            ("host", Value::from("example.com")),
            ("path", Value::from("/api")),
        ]))
        .unwrap();

        assert_eq!(result.to_string(), "https://example.com/api");
    }

    #[test]
    fn test_query_string_simple() {
        let mut params = BTreeMap::new();
        params.insert("name".to_string(), Value::from("test"));
        params.insert("value".to_string(), Value::from(42));

        let result = query_string_fn(Kwargs::from_iter(vec![(
            "params",
            Value::from_object(params),
        )]))
        .unwrap();

        let output = result.to_string();
        assert!(output.contains("name=test"));
        assert!(output.contains("value=42"));
    }
}
