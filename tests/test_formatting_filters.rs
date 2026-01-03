//! Tests for formatting filter-functions (filesizeformat, urlencode)
//!
//! Tests both function and filter syntax.

use minijinja::Value;
use minijinja::value::Kwargs;
use tmpltool::filter_functions::FilterFunction;
use tmpltool::filter_functions::formatting::{Filesizeformat, Urlencode};

/// Helper to create empty kwargs
fn empty_kwargs() -> Kwargs {
    Kwargs::from_iter(Vec::<(&str, Value)>::new())
}

// ========== filesizeformat tests ==========

#[test]
fn test_filesizeformat_bytes() {
    let result = Filesizeformat::call_as_filter(&Value::from(500), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "500 bytes");
}

#[test]
fn test_filesizeformat_kb() {
    let result = Filesizeformat::call_as_filter(&Value::from(2048), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "2 KB");
}

#[test]
fn test_filesizeformat_mb() {
    let result = Filesizeformat::call_as_filter(&Value::from(1048576), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "1 MB");
}

#[test]
fn test_filesizeformat_gb() {
    let result =
        Filesizeformat::call_as_filter(&Value::from(1073741824_i64), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "1 GB");
}

#[test]
fn test_filesizeformat_decimal_kb() {
    let result = Filesizeformat::call_as_filter(&Value::from(1536), empty_kwargs()).unwrap();
    let s = result.as_str().unwrap();
    assert!(s.starts_with("1.5") && s.ends_with("KB"));
}

#[test]
fn test_filesizeformat_decimal_mb() {
    let result = Filesizeformat::call_as_filter(&Value::from(2621440), empty_kwargs()).unwrap();
    let s = result.as_str().unwrap();
    assert!(s.starts_with("2.5") && s.ends_with("MB"));
}

#[test]
fn test_filesizeformat_zero() {
    let result = Filesizeformat::call_as_filter(&Value::from(0), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "0 bytes");
}

#[test]
fn test_filesizeformat_one_byte() {
    let result = Filesizeformat::call_as_filter(&Value::from(1), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "1 bytes");
}

#[test]
fn test_filesizeformat_large_tb() {
    let result =
        Filesizeformat::call_as_filter(&Value::from(1099511627776_i64), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "1 TB");
}

#[test]
fn test_filesizeformat_error_not_number() {
    let result = Filesizeformat::call_as_filter(&Value::from("not a number"), empty_kwargs());
    assert!(result.is_err());
    assert!(
        result
            .err()
            .unwrap()
            .to_string()
            .contains("requires a number")
    );
}

// ========== filesizeformat function syntax tests ==========

#[test]
fn test_filesizeformat_function_syntax() {
    let kwargs = Kwargs::from_iter(vec![("bytes", Value::from(1048576))]);
    let result = Filesizeformat::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "1 MB");
}

#[test]
fn test_filesizeformat_function_syntax_gb() {
    let kwargs = Kwargs::from_iter(vec![("bytes", Value::from(1073741824_i64))]);
    let result = Filesizeformat::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "1 GB");
}

// ========== urlencode tests ==========

#[test]
fn test_urlencode_basic() {
    let result = Urlencode::call_as_filter(&Value::from("hello world"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "hello%20world");
}

#[test]
fn test_urlencode_special_chars() {
    let result =
        Urlencode::call_as_filter(&Value::from("hello world & foo=bar"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "hello%20world%20%26%20foo%3Dbar");
}

#[test]
fn test_urlencode_already_encoded() {
    let result = Urlencode::call_as_filter(&Value::from("hello%20world"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "hello%2520world");
}

#[test]
fn test_urlencode_alphanumeric() {
    let result = Urlencode::call_as_filter(&Value::from("abc123"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "abc123");
}

#[test]
fn test_urlencode_url() {
    let result = Urlencode::call_as_filter(
        &Value::from("https://example.com/path?query=value"),
        empty_kwargs(),
    )
    .unwrap();
    assert_eq!(
        result.as_str().unwrap(),
        "https%3A%2F%2Fexample%2Ecom%2Fpath%3Fquery%3Dvalue"
    );
}

#[test]
fn test_urlencode_empty_string() {
    let result = Urlencode::call_as_filter(&Value::from(""), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "");
}

#[test]
fn test_urlencode_unicode() {
    let result = Urlencode::call_as_filter(&Value::from("hello 世界"), empty_kwargs()).unwrap();
    let s = result.as_str().unwrap();
    assert!(s.contains("%E4%B8%96%E7%95%8C")); // UTF-8 encoding of 世界
}

#[test]
fn test_urlencode_slash() {
    let result = Urlencode::call_as_filter(&Value::from("path/to/file"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "path%2Fto%2Ffile");
}

#[test]
fn test_urlencode_error_not_string() {
    let result = Urlencode::call_as_filter(&Value::from(123), empty_kwargs());
    assert!(result.is_err());
    assert!(
        result
            .err()
            .unwrap()
            .to_string()
            .contains("requires a string")
    );
}

// ========== urlencode function syntax tests ==========

#[test]
fn test_urlencode_function_syntax() {
    let kwargs = Kwargs::from_iter(vec![("string", Value::from("hello world"))]);
    let result = Urlencode::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "hello%20world");
}

#[test]
fn test_urlencode_function_syntax_special() {
    let kwargs = Kwargs::from_iter(vec![("string", Value::from("foo=bar&baz"))]);
    let result = Urlencode::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "foo%3Dbar%26baz");
}
