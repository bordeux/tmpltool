//! Tests for URL filter-functions.
//!
//! Tests both function and filter syntax for:
//! - url_encode, url_decode, parse_url

use minijinja::Value;
use minijinja::value::Kwargs;
use tmpltool::filter_functions::FilterFunction;
use tmpltool::filter_functions::url::{ParseUrl, UrlDecode, UrlEncode};

/// Helper to create empty kwargs
fn empty_kwargs() -> Kwargs {
    Kwargs::from_iter(Vec::<(&str, Value)>::new())
}

// ============================================
// UrlEncode tests
// ============================================

#[test]
fn test_url_encode_filter_syntax() {
    let result = UrlEncode::call_as_filter(&Value::from("hello world"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "hello%20world");
}

#[test]
fn test_url_encode_function_syntax() {
    let kwargs = Kwargs::from_iter(vec![("string", Value::from("hello world"))]);
    let result = UrlEncode::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "hello%20world");
}

#[test]
fn test_url_encode_special_chars() {
    let result =
        UrlEncode::call_as_filter(&Value::from("foo=bar&baz=qux"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "foo%3Dbar%26baz%3Dqux");
}

#[test]
fn test_url_encode_empty() {
    let result = UrlEncode::call_as_filter(&Value::from(""), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "");
}

#[test]
fn test_url_encode_unicode() {
    let result = UrlEncode::call_as_filter(&Value::from("hello 世界"), empty_kwargs()).unwrap();
    let s = result.as_str().unwrap();
    assert!(s.contains("%E4%B8%96%E7%95%8C"));
}

#[test]
fn test_url_encode_error_not_string() {
    let result = UrlEncode::call_as_filter(&Value::from(123), empty_kwargs());
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires a string")
    );
}

// ============================================
// UrlDecode tests
// ============================================

#[test]
fn test_url_decode_filter_syntax() {
    let result = UrlDecode::call_as_filter(&Value::from("hello%20world"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "hello world");
}

#[test]
fn test_url_decode_function_syntax() {
    let kwargs = Kwargs::from_iter(vec![("string", Value::from("hello%20world"))]);
    let result = UrlDecode::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "hello world");
}

#[test]
fn test_url_decode_special_chars() {
    let result =
        UrlDecode::call_as_filter(&Value::from("foo%3Dbar%26baz%3Dqux"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "foo=bar&baz=qux");
}

#[test]
fn test_url_decode_empty() {
    let result = UrlDecode::call_as_filter(&Value::from(""), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "");
}

#[test]
fn test_url_decode_already_decoded() {
    let result = UrlDecode::call_as_filter(&Value::from("hello world"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "hello world");
}

#[test]
fn test_url_decode_error_not_string() {
    let result = UrlDecode::call_as_filter(&Value::from(123), empty_kwargs());
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires a string")
    );
}

// ============================================
// ParseUrl tests
// ============================================

#[test]
fn test_parse_url_filter_syntax() {
    let result = ParseUrl::call_as_filter(
        &Value::from("https://example.com:8080/path?query=value#fragment"),
        empty_kwargs(),
    )
    .unwrap();

    assert_eq!(
        result.get_attr("scheme").unwrap().as_str().unwrap(),
        "https"
    );
    assert_eq!(
        result.get_attr("host").unwrap().as_str().unwrap(),
        "example.com"
    );
    assert_eq!(result.get_attr("port").unwrap().as_i64().unwrap(), 8080);
    assert_eq!(result.get_attr("path").unwrap().as_str().unwrap(), "/path");
    assert_eq!(
        result.get_attr("query").unwrap().as_str().unwrap(),
        "query=value"
    );
    assert_eq!(
        result.get_attr("fragment").unwrap().as_str().unwrap(),
        "fragment"
    );
}

#[test]
fn test_parse_url_function_syntax() {
    let kwargs = Kwargs::from_iter(vec![("url", Value::from("https://example.com/path"))]);
    let result = ParseUrl::call_as_function(kwargs).unwrap();

    assert_eq!(
        result.get_attr("scheme").unwrap().as_str().unwrap(),
        "https"
    );
    assert_eq!(
        result.get_attr("host").unwrap().as_str().unwrap(),
        "example.com"
    );
    assert_eq!(result.get_attr("path").unwrap().as_str().unwrap(), "/path");
}

#[test]
fn test_parse_url_with_credentials() {
    let result = ParseUrl::call_as_filter(
        &Value::from("https://user:pass@example.com/path"),
        empty_kwargs(),
    )
    .unwrap();

    assert_eq!(
        result.get_attr("username").unwrap().as_str().unwrap(),
        "user"
    );
    assert_eq!(
        result.get_attr("password").unwrap().as_str().unwrap(),
        "pass"
    );
}

#[test]
fn test_parse_url_default_port() {
    let result =
        ParseUrl::call_as_filter(&Value::from("https://example.com/path"), empty_kwargs()).unwrap();
    // HTTPS default port is 443
    assert_eq!(result.get_attr("port").unwrap().as_i64().unwrap(), 443);
}

#[test]
fn test_parse_url_invalid() {
    let result = ParseUrl::call_as_filter(&Value::from("not a url"), empty_kwargs());
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Failed to parse"));
}

#[test]
fn test_parse_url_error_not_string() {
    let result = ParseUrl::call_as_filter(&Value::from(123), empty_kwargs());
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires a string")
    );
}
