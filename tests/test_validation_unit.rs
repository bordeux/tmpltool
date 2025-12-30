use std::collections::HashMap;
use tera::{Function, Value};
use tmpltool::functions::validation::{IsEmail, IsIp, IsUrl, IsUuid, MatchesRegex};

// ========== is_email tests ==========

#[test]
fn test_is_email_valid() {
    let mut args = HashMap::new();
    args.insert(
        "string".to_string(),
        Value::String("test@example.com".to_string()),
    );

    let result = IsEmail.call(&args).unwrap();
    assert!(result.as_bool().unwrap());
}

#[test]
fn test_is_email_valid_with_subdomain() {
    let mut args = HashMap::new();
    args.insert(
        "string".to_string(),
        Value::String("user@mail.example.com".to_string()),
    );

    let result = IsEmail.call(&args).unwrap();
    assert!(result.as_bool().unwrap());
}

#[test]
fn test_is_email_valid_with_plus() {
    let mut args = HashMap::new();
    args.insert(
        "string".to_string(),
        Value::String("user+tag@example.com".to_string()),
    );

    let result = IsEmail.call(&args).unwrap();
    assert!(result.as_bool().unwrap());
}

#[test]
fn test_is_email_valid_with_numbers() {
    let mut args = HashMap::new();
    args.insert(
        "string".to_string(),
        Value::String("user123@example123.com".to_string()),
    );

    let result = IsEmail.call(&args).unwrap();
    assert!(result.as_bool().unwrap());
}

#[test]
fn test_is_email_invalid_no_at() {
    let mut args = HashMap::new();
    args.insert(
        "string".to_string(),
        Value::String("userexample.com".to_string()),
    );

    let result = IsEmail.call(&args).unwrap();
    assert!(!result.as_bool().unwrap());
}

#[test]
fn test_is_email_invalid_no_domain() {
    let mut args = HashMap::new();
    args.insert("string".to_string(), Value::String("user@".to_string()));

    let result = IsEmail.call(&args).unwrap();
    assert!(!result.as_bool().unwrap());
}

#[test]
fn test_is_email_invalid_no_tld() {
    let mut args = HashMap::new();
    args.insert(
        "string".to_string(),
        Value::String("user@example".to_string()),
    );

    let result = IsEmail.call(&args).unwrap();
    assert!(!result.as_bool().unwrap());
}

#[test]
fn test_is_email_missing_argument() {
    let args = HashMap::new();
    let result = IsEmail.call(&args);
    assert!(result.is_err());
}

// ========== is_url tests ==========

#[test]
fn test_is_url_valid_https() {
    let mut args = HashMap::new();
    args.insert(
        "string".to_string(),
        Value::String("https://example.com".to_string()),
    );

    let result = IsUrl.call(&args).unwrap();
    assert!(result.as_bool().unwrap());
}

#[test]
fn test_is_url_valid_http() {
    let mut args = HashMap::new();
    args.insert(
        "string".to_string(),
        Value::String("http://example.com".to_string()),
    );

    let result = IsUrl.call(&args).unwrap();
    assert!(result.as_bool().unwrap());
}

#[test]
fn test_is_url_valid_with_path() {
    let mut args = HashMap::new();
    args.insert(
        "string".to_string(),
        Value::String("https://example.com/path/to/page".to_string()),
    );

    let result = IsUrl.call(&args).unwrap();
    assert!(result.as_bool().unwrap());
}

#[test]
fn test_is_url_valid_with_query() {
    let mut args = HashMap::new();
    args.insert(
        "string".to_string(),
        Value::String("https://example.com/search?q=test&page=1".to_string()),
    );

    let result = IsUrl.call(&args).unwrap();
    assert!(result.as_bool().unwrap());
}

#[test]
fn test_is_url_valid_ftp() {
    let mut args = HashMap::new();
    args.insert(
        "string".to_string(),
        Value::String("ftp://files.example.com/file.txt".to_string()),
    );

    let result = IsUrl.call(&args).unwrap();
    assert!(result.as_bool().unwrap());
}

#[test]
fn test_is_url_valid_with_port() {
    let mut args = HashMap::new();
    args.insert(
        "string".to_string(),
        Value::String("https://example.com:8080/path".to_string()),
    );

    let result = IsUrl.call(&args).unwrap();
    assert!(result.as_bool().unwrap());
}

#[test]
fn test_is_url_invalid_no_scheme() {
    let mut args = HashMap::new();
    args.insert(
        "string".to_string(),
        Value::String("example.com".to_string()),
    );

    let result = IsUrl.call(&args).unwrap();
    assert!(!result.as_bool().unwrap());
}

#[test]
fn test_is_url_invalid_scheme() {
    let mut args = HashMap::new();
    args.insert(
        "string".to_string(),
        Value::String("invalid://example.com".to_string()),
    );

    let result = IsUrl.call(&args).unwrap();
    assert!(!result.as_bool().unwrap());
}

#[test]
fn test_is_url_missing_argument() {
    let args = HashMap::new();
    let result = IsUrl.call(&args);
    assert!(result.is_err());
}

// ========== is_ip tests ==========

#[test]
fn test_is_ip_valid_ipv4() {
    let mut args = HashMap::new();
    args.insert(
        "string".to_string(),
        Value::String("192.168.1.1".to_string()),
    );

    let result = IsIp.call(&args).unwrap();
    assert!(result.as_bool().unwrap());
}

#[test]
fn test_is_ip_valid_ipv4_localhost() {
    let mut args = HashMap::new();
    args.insert("string".to_string(), Value::String("127.0.0.1".to_string()));

    let result = IsIp.call(&args).unwrap();
    assert!(result.as_bool().unwrap());
}

#[test]
fn test_is_ip_valid_ipv4_zero() {
    let mut args = HashMap::new();
    args.insert("string".to_string(), Value::String("0.0.0.0".to_string()));

    let result = IsIp.call(&args).unwrap();
    assert!(result.as_bool().unwrap());
}

#[test]
fn test_is_ip_valid_ipv4_broadcast() {
    let mut args = HashMap::new();
    args.insert(
        "string".to_string(),
        Value::String("255.255.255.255".to_string()),
    );

    let result = IsIp.call(&args).unwrap();
    assert!(result.as_bool().unwrap());
}

#[test]
fn test_is_ip_valid_ipv6() {
    let mut args = HashMap::new();
    args.insert(
        "string".to_string(),
        Value::String("2001:0db8:85a3:0000:0000:8a2e:0370:7334".to_string()),
    );

    let result = IsIp.call(&args).unwrap();
    assert!(result.as_bool().unwrap());
}

#[test]
fn test_is_ip_valid_ipv6_compressed() {
    let mut args = HashMap::new();
    args.insert(
        "string".to_string(),
        Value::String("2001:db8::1".to_string()),
    );

    let result = IsIp.call(&args).unwrap();
    assert!(result.as_bool().unwrap());
}

#[test]
fn test_is_ip_valid_ipv6_localhost() {
    let mut args = HashMap::new();
    args.insert("string".to_string(), Value::String("::1".to_string()));

    let result = IsIp.call(&args).unwrap();
    assert!(result.as_bool().unwrap());
}

#[test]
fn test_is_ip_invalid_ipv4_out_of_range() {
    let mut args = HashMap::new();
    args.insert("string".to_string(), Value::String("256.1.1.1".to_string()));

    let result = IsIp.call(&args).unwrap();
    assert!(!result.as_bool().unwrap());
}

#[test]
fn test_is_ip_invalid_ipv4_incomplete() {
    let mut args = HashMap::new();
    args.insert("string".to_string(), Value::String("192.168.1".to_string()));

    let result = IsIp.call(&args).unwrap();
    assert!(!result.as_bool().unwrap());
}

#[test]
fn test_is_ip_invalid_not_ip() {
    let mut args = HashMap::new();
    args.insert("string".to_string(), Value::String("not-an-ip".to_string()));

    let result = IsIp.call(&args).unwrap();
    assert!(!result.as_bool().unwrap());
}

#[test]
fn test_is_ip_missing_argument() {
    let args = HashMap::new();
    let result = IsIp.call(&args);
    assert!(result.is_err());
}

// ========== is_uuid tests ==========

#[test]
fn test_is_uuid_valid_v4() {
    let mut args = HashMap::new();
    args.insert(
        "string".to_string(),
        Value::String("550e8400-e29b-41d4-a716-446655440000".to_string()),
    );

    let result = IsUuid.call(&args).unwrap();
    assert!(result.as_bool().unwrap());
}

#[test]
fn test_is_uuid_valid_lowercase() {
    let mut args = HashMap::new();
    args.insert(
        "string".to_string(),
        Value::String("f47ac10b-58cc-4372-a567-0e02b2c3d479".to_string()),
    );

    let result = IsUuid.call(&args).unwrap();
    assert!(result.as_bool().unwrap());
}

#[test]
fn test_is_uuid_valid_uppercase() {
    let mut args = HashMap::new();
    args.insert(
        "string".to_string(),
        Value::String("F47AC10B-58CC-4372-A567-0E02B2C3D479".to_string()),
    );

    let result = IsUuid.call(&args).unwrap();
    assert!(result.as_bool().unwrap());
}

#[test]
fn test_is_uuid_valid_mixed_case() {
    let mut args = HashMap::new();
    args.insert(
        "string".to_string(),
        Value::String("F47ac10b-58Cc-4372-A567-0e02b2C3d479".to_string()),
    );

    let result = IsUuid.call(&args).unwrap();
    assert!(result.as_bool().unwrap());
}

#[test]
fn test_is_uuid_invalid_no_dashes() {
    let mut args = HashMap::new();
    args.insert(
        "string".to_string(),
        Value::String("550e8400e29b41d4a716446655440000".to_string()),
    );

    let result = IsUuid.call(&args).unwrap();
    assert!(!result.as_bool().unwrap());
}

#[test]
fn test_is_uuid_invalid_wrong_length() {
    let mut args = HashMap::new();
    args.insert(
        "string".to_string(),
        Value::String("550e8400-e29b-41d4-a716-4466554400".to_string()),
    );

    let result = IsUuid.call(&args).unwrap();
    assert!(!result.as_bool().unwrap());
}

#[test]
fn test_is_uuid_invalid_wrong_format() {
    let mut args = HashMap::new();
    args.insert(
        "string".to_string(),
        Value::String("not-a-uuid-string".to_string()),
    );

    let result = IsUuid.call(&args).unwrap();
    assert!(!result.as_bool().unwrap());
}

#[test]
fn test_is_uuid_missing_argument() {
    let args = HashMap::new();
    let result = IsUuid.call(&args);
    assert!(result.is_err());
}

// ========== matches_regex tests ==========

#[test]
fn test_matches_regex_simple_match() {
    let mut args = HashMap::new();
    args.insert("pattern".to_string(), Value::String("^hello$".to_string()));
    args.insert("string".to_string(), Value::String("hello".to_string()));

    let result = MatchesRegex.call(&args).unwrap();
    assert!(result.as_bool().unwrap());
}

#[test]
fn test_matches_regex_simple_no_match() {
    let mut args = HashMap::new();
    args.insert("pattern".to_string(), Value::String("^hello$".to_string()));
    args.insert("string".to_string(), Value::String("world".to_string()));

    let result = MatchesRegex.call(&args).unwrap();
    assert!(!result.as_bool().unwrap());
}

#[test]
fn test_matches_regex_digit_pattern() {
    let mut args = HashMap::new();
    args.insert("pattern".to_string(), Value::String(r"^\d+$".to_string()));
    args.insert("string".to_string(), Value::String("12345".to_string()));

    let result = MatchesRegex.call(&args).unwrap();
    assert!(result.as_bool().unwrap());
}

#[test]
fn test_matches_regex_digit_pattern_no_match() {
    let mut args = HashMap::new();
    args.insert("pattern".to_string(), Value::String(r"^\d+$".to_string()));
    args.insert("string".to_string(), Value::String("abc123".to_string()));

    let result = MatchesRegex.call(&args).unwrap();
    assert!(!result.as_bool().unwrap());
}

#[test]
fn test_matches_regex_email_pattern() {
    let mut args = HashMap::new();
    args.insert(
        "pattern".to_string(),
        Value::String(r"^[a-z]+@[a-z]+\.[a-z]+$".to_string()),
    );
    args.insert(
        "string".to_string(),
        Value::String("test@example.com".to_string()),
    );

    let result = MatchesRegex.call(&args).unwrap();
    assert!(result.as_bool().unwrap());
}

#[test]
fn test_matches_regex_case_insensitive() {
    let mut args = HashMap::new();
    args.insert(
        "pattern".to_string(),
        Value::String("(?i)^hello$".to_string()),
    );
    args.insert("string".to_string(), Value::String("HELLO".to_string()));

    let result = MatchesRegex.call(&args).unwrap();
    assert!(result.as_bool().unwrap());
}

#[test]
fn test_matches_regex_partial_match() {
    let mut args = HashMap::new();
    args.insert("pattern".to_string(), Value::String("world".to_string()));
    args.insert(
        "string".to_string(),
        Value::String("hello world!".to_string()),
    );

    let result = MatchesRegex.call(&args).unwrap();
    assert!(result.as_bool().unwrap());
}

#[test]
fn test_matches_regex_complex_pattern() {
    let mut args = HashMap::new();
    args.insert(
        "pattern".to_string(),
        Value::String(r"^[A-Z]{2,4}-\d{3,5}$".to_string()),
    );
    args.insert("string".to_string(), Value::String("ABC-1234".to_string()));

    let result = MatchesRegex.call(&args).unwrap();
    assert!(result.as_bool().unwrap());
}

#[test]
fn test_matches_regex_invalid_pattern() {
    let mut args = HashMap::new();
    args.insert(
        "pattern".to_string(),
        Value::String("[invalid(".to_string()),
    );
    args.insert("string".to_string(), Value::String("test".to_string()));

    let result = MatchesRegex.call(&args);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Invalid regex"));
}

#[test]
fn test_matches_regex_missing_pattern() {
    let mut args = HashMap::new();
    args.insert("string".to_string(), Value::String("test".to_string()));

    let result = MatchesRegex.call(&args);
    assert!(result.is_err());
}

#[test]
fn test_matches_regex_missing_string() {
    let mut args = HashMap::new();
    args.insert("pattern".to_string(), Value::String("^test$".to_string()));

    let result = MatchesRegex.call(&args);
    assert!(result.is_err());
}
