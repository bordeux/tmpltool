use minijinja::value::Kwargs;
use tmpltool::functions::validation::{
    is_email_fn, is_ip_fn, is_url_fn, is_uuid_fn, matches_regex_fn,
};

// Helper to create kwargs for testing
fn create_kwargs(args: Vec<(&str, &str)>) -> Kwargs {
    Kwargs::from_iter(args.iter().map(|(k, v)| (*k, minijinja::Value::from(*v))))
}

// ========== is_email tests ==========

#[test]
fn test_is_email_valid() {
    let kwargs = create_kwargs(vec![("string", "test@example.com")]);

    let result = is_email_fn(kwargs).unwrap();
    assert!(result.is_true());
}

#[test]
fn test_is_email_valid_with_subdomain() {
    let kwargs = create_kwargs(vec![("string", "user@mail.example.com")]);

    let result = is_email_fn(kwargs).unwrap();
    assert!(result.is_true());
}

#[test]
fn test_is_email_valid_with_plus() {
    let kwargs = create_kwargs(vec![("string", "user+tag@example.com")]);

    let result = is_email_fn(kwargs).unwrap();
    assert!(result.is_true());
}

#[test]
fn test_is_email_valid_with_numbers() {
    let kwargs = create_kwargs(vec![("string", "user123@example123.com")]);

    let result = is_email_fn(kwargs).unwrap();
    assert!(result.is_true());
}

#[test]
fn test_is_email_invalid_no_at() {
    let kwargs = create_kwargs(vec![("string", "userexample.com")]);

    let result = is_email_fn(kwargs).unwrap();
    assert!(!result.is_true());
}

#[test]
fn test_is_email_invalid_no_domain() {
    let kwargs = create_kwargs(vec![("string", "user@")]);

    let result = is_email_fn(kwargs).unwrap();
    assert!(!result.is_true());
}

#[test]
fn test_is_email_invalid_no_tld() {
    let kwargs = create_kwargs(vec![("string", "user@example")]);

    let result = is_email_fn(kwargs).unwrap();
    assert!(!result.is_true());
}

#[test]
fn test_is_email_missing_argument() {
    let kwargs = create_kwargs(vec![]);
    let result = is_email_fn(kwargs);
    assert!(result.is_err());
}

// ========== is_url tests ==========

#[test]
fn test_is_url_valid_https() {
    let kwargs = create_kwargs(vec![("string", "https://example.com")]);

    let result = is_url_fn(kwargs).unwrap();
    assert!(result.is_true());
}

#[test]
fn test_is_url_valid_http() {
    let kwargs = create_kwargs(vec![("string", "http://example.com")]);

    let result = is_url_fn(kwargs).unwrap();
    assert!(result.is_true());
}

#[test]
fn test_is_url_valid_with_path() {
    let kwargs = create_kwargs(vec![("string", "https://example.com/path/to/page")]);

    let result = is_url_fn(kwargs).unwrap();
    assert!(result.is_true());
}

#[test]
fn test_is_url_valid_with_query() {
    let kwargs = create_kwargs(vec![("string", "https://example.com/search?q=test&page=1")]);

    let result = is_url_fn(kwargs).unwrap();
    assert!(result.is_true());
}

#[test]
fn test_is_url_valid_ftp() {
    let kwargs = create_kwargs(vec![("string", "ftp://files.example.com/file.txt")]);

    let result = is_url_fn(kwargs).unwrap();
    assert!(result.is_true());
}

#[test]
fn test_is_url_valid_with_port() {
    let kwargs = create_kwargs(vec![("string", "https://example.com:8080/path")]);

    let result = is_url_fn(kwargs).unwrap();
    assert!(result.is_true());
}

#[test]
fn test_is_url_invalid_no_scheme() {
    let kwargs = create_kwargs(vec![("string", "example.com")]);

    let result = is_url_fn(kwargs).unwrap();
    assert!(!result.is_true());
}

#[test]
fn test_is_url_invalid_scheme() {
    let kwargs = create_kwargs(vec![("string", "invalid://example.com")]);

    let result = is_url_fn(kwargs).unwrap();
    assert!(!result.is_true());
}

#[test]
fn test_is_url_missing_argument() {
    let kwargs = create_kwargs(vec![]);
    let result = is_url_fn(kwargs);
    assert!(result.is_err());
}

// ========== is_ip tests ==========

#[test]
fn test_is_ip_valid_ipv4() {
    let kwargs = create_kwargs(vec![("string", "192.168.1.1")]);

    let result = is_ip_fn(kwargs).unwrap();
    assert!(result.is_true());
}

#[test]
fn test_is_ip_valid_ipv4_localhost() {
    let kwargs = create_kwargs(vec![("string", "127.0.0.1")]);

    let result = is_ip_fn(kwargs).unwrap();
    assert!(result.is_true());
}

#[test]
fn test_is_ip_valid_ipv4_zero() {
    let kwargs = create_kwargs(vec![("string", "0.0.0.0")]);

    let result = is_ip_fn(kwargs).unwrap();
    assert!(result.is_true());
}

#[test]
fn test_is_ip_valid_ipv4_broadcast() {
    let kwargs = create_kwargs(vec![("string", "255.255.255.255")]);

    let result = is_ip_fn(kwargs).unwrap();
    assert!(result.is_true());
}

#[test]
fn test_is_ip_valid_ipv6() {
    let kwargs = create_kwargs(vec![("string", "2001:0db8:85a3:0000:0000:8a2e:0370:7334")]);

    let result = is_ip_fn(kwargs).unwrap();
    assert!(result.is_true());
}

#[test]
fn test_is_ip_valid_ipv6_compressed() {
    let kwargs = create_kwargs(vec![("string", "2001:db8::1")]);

    let result = is_ip_fn(kwargs).unwrap();
    assert!(result.is_true());
}

#[test]
fn test_is_ip_valid_ipv6_localhost() {
    let kwargs = create_kwargs(vec![("string", "::1")]);

    let result = is_ip_fn(kwargs).unwrap();
    assert!(result.is_true());
}

#[test]
fn test_is_ip_invalid_ipv4_out_of_range() {
    let kwargs = create_kwargs(vec![("string", "256.1.1.1")]);

    let result = is_ip_fn(kwargs).unwrap();
    assert!(!result.is_true());
}

#[test]
fn test_is_ip_invalid_ipv4_incomplete() {
    let kwargs = create_kwargs(vec![("string", "192.168.1")]);

    let result = is_ip_fn(kwargs).unwrap();
    assert!(!result.is_true());
}

#[test]
fn test_is_ip_invalid_not_ip() {
    let kwargs = create_kwargs(vec![("string", "not-an-ip")]);

    let result = is_ip_fn(kwargs).unwrap();
    assert!(!result.is_true());
}

#[test]
fn test_is_ip_missing_argument() {
    let kwargs = create_kwargs(vec![]);
    let result = is_ip_fn(kwargs);
    assert!(result.is_err());
}

// ========== is_uuid tests ==========

#[test]
fn test_is_uuid_valid_v4() {
    let kwargs = create_kwargs(vec![("string", "550e8400-e29b-41d4-a716-446655440000")]);

    let result = is_uuid_fn(kwargs).unwrap();
    assert!(result.is_true());
}

#[test]
fn test_is_uuid_valid_lowercase() {
    let kwargs = create_kwargs(vec![("string", "f47ac10b-58cc-4372-a567-0e02b2c3d479")]);

    let result = is_uuid_fn(kwargs).unwrap();
    assert!(result.is_true());
}

#[test]
fn test_is_uuid_valid_uppercase() {
    let kwargs = create_kwargs(vec![("string", "F47AC10B-58CC-4372-A567-0E02B2C3D479")]);

    let result = is_uuid_fn(kwargs).unwrap();
    assert!(result.is_true());
}

#[test]
fn test_is_uuid_valid_mixed_case() {
    let kwargs = create_kwargs(vec![("string", "F47ac10b-58Cc-4372-A567-0e02b2C3d479")]);

    let result = is_uuid_fn(kwargs).unwrap();
    assert!(result.is_true());
}

#[test]
fn test_is_uuid_invalid_no_dashes() {
    let kwargs = create_kwargs(vec![("string", "550e8400e29b41d4a716446655440000")]);

    let result = is_uuid_fn(kwargs).unwrap();
    assert!(!result.is_true());
}

#[test]
fn test_is_uuid_invalid_wrong_length() {
    let kwargs = create_kwargs(vec![("string", "550e8400-e29b-41d4-a716-4466554400")]);

    let result = is_uuid_fn(kwargs).unwrap();
    assert!(!result.is_true());
}

#[test]
fn test_is_uuid_invalid_wrong_format() {
    let kwargs = create_kwargs(vec![("string", "not-a-uuid-string")]);

    let result = is_uuid_fn(kwargs).unwrap();
    assert!(!result.is_true());
}

#[test]
fn test_is_uuid_missing_argument() {
    let kwargs = create_kwargs(vec![]);
    let result = is_uuid_fn(kwargs);
    assert!(result.is_err());
}

// ========== matches_regex tests ==========

#[test]
fn test_matches_regex_simple_match() {
    let kwargs = create_kwargs(vec![("pattern", "^hello$"), ("string", "hello")]);

    let result = matches_regex_fn(kwargs).unwrap();
    assert!(result.is_true());
}

#[test]
fn test_matches_regex_simple_no_match() {
    let kwargs = create_kwargs(vec![("pattern", "^hello$"), ("string", "world")]);

    let result = matches_regex_fn(kwargs).unwrap();
    assert!(!result.is_true());
}

#[test]
fn test_matches_regex_digit_pattern() {
    let kwargs = create_kwargs(vec![("pattern", r"^\d+$"), ("string", "12345")]);

    let result = matches_regex_fn(kwargs).unwrap();
    assert!(result.is_true());
}

#[test]
fn test_matches_regex_digit_pattern_no_match() {
    let kwargs = create_kwargs(vec![("pattern", r"^\d+$"), ("string", "abc123")]);

    let result = matches_regex_fn(kwargs).unwrap();
    assert!(!result.is_true());
}

#[test]
fn test_matches_regex_email_pattern() {
    let kwargs = create_kwargs(vec![
        ("pattern", r"^[a-z]+@[a-z]+\.[a-z]+$"),
        ("string", "test@example.com"),
    ]);

    let result = matches_regex_fn(kwargs).unwrap();
    assert!(result.is_true());
}

#[test]
fn test_matches_regex_case_insensitive() {
    let kwargs = create_kwargs(vec![("pattern", "(?i)^hello$"), ("string", "HELLO")]);

    let result = matches_regex_fn(kwargs).unwrap();
    assert!(result.is_true());
}

#[test]
fn test_matches_regex_partial_match() {
    let kwargs = create_kwargs(vec![("pattern", "world"), ("string", "hello world!")]);

    let result = matches_regex_fn(kwargs).unwrap();
    assert!(result.is_true());
}

#[test]
fn test_matches_regex_complex_pattern() {
    let kwargs = create_kwargs(vec![
        ("pattern", r"^[A-Z]{2,4}-\d{3,5}$"),
        ("string", "ABC-1234"),
    ]);

    let result = matches_regex_fn(kwargs).unwrap();
    assert!(result.is_true());
}

#[test]
fn test_matches_regex_invalid_pattern() {
    let kwargs = create_kwargs(vec![("pattern", "[invalid("), ("string", "test")]);

    let result = matches_regex_fn(kwargs);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Invalid regex"));
}

#[test]
fn test_matches_regex_missing_pattern() {
    let kwargs = create_kwargs(vec![("string", "test")]);

    let result = matches_regex_fn(kwargs);
    assert!(result.is_err());
}

#[test]
fn test_matches_regex_missing_string() {
    let kwargs = create_kwargs(vec![("pattern", "^test$")]);

    let result = matches_regex_fn(kwargs);
    assert!(result.is_err());
}
