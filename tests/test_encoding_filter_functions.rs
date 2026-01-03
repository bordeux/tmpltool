//! Tests for encoding filter-functions.
//!
//! Tests both function and filter syntax for:
//! - base64_encode, base64_decode
//! - hex_encode, hex_decode
//! - escape_html, escape_xml, escape_shell

use minijinja::Value;
use minijinja::value::Kwargs;
use tmpltool::filter_functions::FilterFunction;
use tmpltool::filter_functions::encoding::{
    Base64Decode, Base64Encode, EscapeHtml, EscapeShell, EscapeXml, HexDecode, HexEncode,
};

/// Helper to create empty kwargs
fn empty_kwargs() -> Kwargs {
    Kwargs::from_iter(Vec::<(&str, Value)>::new())
}

// ============================================
// Base64Encode tests
// ============================================

#[test]
fn test_base64_encode_filter_syntax() {
    let result = Base64Encode::call_as_filter(&Value::from("hello"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "aGVsbG8=");
}

#[test]
fn test_base64_encode_function_syntax() {
    let kwargs = Kwargs::from_iter(vec![("string", Value::from("hello"))]);
    let result = Base64Encode::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "aGVsbG8=");
}

#[test]
fn test_base64_encode_empty_string() {
    let result = Base64Encode::call_as_filter(&Value::from(""), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "");
}

#[test]
fn test_base64_encode_unicode() {
    let result = Base64Encode::call_as_filter(&Value::from("héllo"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "aMOpbGxv");
}

#[test]
fn test_base64_encode_error_not_string() {
    let result = Base64Encode::call_as_filter(&Value::from(123), empty_kwargs());
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires a string")
    );
}

// ============================================
// Base64Decode tests
// ============================================

#[test]
fn test_base64_decode_filter_syntax() {
    let result = Base64Decode::call_as_filter(&Value::from("aGVsbG8="), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "hello");
}

#[test]
fn test_base64_decode_function_syntax() {
    let kwargs = Kwargs::from_iter(vec![("string", Value::from("aGVsbG8="))]);
    let result = Base64Decode::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "hello");
}

#[test]
fn test_base64_decode_empty_string() {
    let result = Base64Decode::call_as_filter(&Value::from(""), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "");
}

#[test]
fn test_base64_decode_invalid_base64() {
    let result = Base64Decode::call_as_filter(&Value::from("not-valid-base64!!!"), empty_kwargs());
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Failed to decode"));
}

#[test]
fn test_base64_decode_error_not_string() {
    let result = Base64Decode::call_as_filter(&Value::from(123), empty_kwargs());
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires a string")
    );
}

// ============================================
// HexEncode tests
// ============================================

#[test]
fn test_hex_encode_filter_syntax() {
    let result = HexEncode::call_as_filter(&Value::from("hello"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "68656c6c6f");
}

#[test]
fn test_hex_encode_function_syntax() {
    let kwargs = Kwargs::from_iter(vec![("string", Value::from("hello"))]);
    let result = HexEncode::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "68656c6c6f");
}

#[test]
fn test_hex_encode_empty_string() {
    let result = HexEncode::call_as_filter(&Value::from(""), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "");
}

#[test]
fn test_hex_encode_error_not_string() {
    let result = HexEncode::call_as_filter(&Value::from(123), empty_kwargs());
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires a string")
    );
}

// ============================================
// HexDecode tests
// ============================================

#[test]
fn test_hex_decode_filter_syntax() {
    let result = HexDecode::call_as_filter(&Value::from("68656c6c6f"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "hello");
}

#[test]
fn test_hex_decode_function_syntax() {
    let kwargs = Kwargs::from_iter(vec![("string", Value::from("68656c6c6f"))]);
    let result = HexDecode::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "hello");
}

#[test]
fn test_hex_decode_empty_string() {
    let result = HexDecode::call_as_filter(&Value::from(""), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "");
}

#[test]
fn test_hex_decode_invalid_hex() {
    let result = HexDecode::call_as_filter(&Value::from("not-valid-hex"), empty_kwargs());
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Failed to decode"));
}

#[test]
fn test_hex_decode_error_not_string() {
    let result = HexDecode::call_as_filter(&Value::from(123), empty_kwargs());
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires a string")
    );
}

// ============================================
// EscapeHtml tests
// ============================================

#[test]
fn test_escape_html_filter_syntax() {
    let result = EscapeHtml::call_as_filter(
        &Value::from("<script>alert('xss')</script>"),
        empty_kwargs(),
    )
    .unwrap();
    assert_eq!(
        result.as_str().unwrap(),
        "&lt;script&gt;alert(&#x27;xss&#x27;)&lt;/script&gt;"
    );
}

#[test]
fn test_escape_html_function_syntax() {
    let kwargs = Kwargs::from_iter(vec![("string", Value::from("<div>"))]);
    let result = EscapeHtml::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "&lt;div&gt;");
}

#[test]
fn test_escape_html_ampersand() {
    let result = EscapeHtml::call_as_filter(&Value::from("foo & bar"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "foo &amp; bar");
}

#[test]
fn test_escape_html_quotes() {
    let result = EscapeHtml::call_as_filter(&Value::from("\"hello\""), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "&quot;hello&quot;");
}

#[test]
fn test_escape_html_error_not_string() {
    let result = EscapeHtml::call_as_filter(&Value::from(123), empty_kwargs());
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires a string")
    );
}

// ============================================
// EscapeXml tests
// ============================================

#[test]
fn test_escape_xml_filter_syntax() {
    let result = EscapeXml::call_as_filter(&Value::from("<tag>"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "&lt;tag&gt;");
}

#[test]
fn test_escape_xml_function_syntax() {
    let kwargs = Kwargs::from_iter(vec![("string", Value::from("<tag>"))]);
    let result = EscapeXml::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "&lt;tag&gt;");
}

#[test]
fn test_escape_xml_single_quote() {
    let result = EscapeXml::call_as_filter(&Value::from("it's"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "it&apos;s");
}

#[test]
fn test_escape_xml_all_special_chars() {
    let result = EscapeXml::call_as_filter(
        &Value::from("<tag attr=\"val\" other='val2'>"),
        empty_kwargs(),
    )
    .unwrap();
    assert!(result.as_str().unwrap().contains("&lt;"));
    assert!(result.as_str().unwrap().contains("&gt;"));
    assert!(result.as_str().unwrap().contains("&quot;"));
    assert!(result.as_str().unwrap().contains("&apos;"));
}

#[test]
fn test_escape_xml_error_not_string() {
    let result = EscapeXml::call_as_filter(&Value::from(123), empty_kwargs());
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires a string")
    );
}

// ============================================
// EscapeShell tests
// ============================================

#[test]
fn test_escape_shell_filter_syntax() {
    let result = EscapeShell::call_as_filter(&Value::from("hello world"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "'hello world'");
}

#[test]
fn test_escape_shell_function_syntax() {
    let kwargs = Kwargs::from_iter(vec![("string", Value::from("hello world"))]);
    let result = EscapeShell::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "'hello world'");
}

#[test]
fn test_escape_shell_with_single_quote() {
    let result = EscapeShell::call_as_filter(&Value::from("it's"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "'it'\\''s'");
}

#[test]
fn test_escape_shell_special_chars() {
    let result = EscapeShell::call_as_filter(&Value::from("$(rm -rf /)"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "'$(rm -rf /)'");
}

#[test]
fn test_escape_shell_error_not_string() {
    let result = EscapeShell::call_as_filter(&Value::from(123), empty_kwargs());
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires a string")
    );
}
