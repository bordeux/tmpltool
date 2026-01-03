//! Unit tests for validation functions (matches_regex)
//!
//! Note: is_email, is_url, is_ip, and is_uuid have been migrated to
//! src/is_functions/validation.rs and are tested in tests/test_is_validation.rs

use minijinja::value::Kwargs;
use tmpltool::functions::validation::matches_regex_fn;

// Helper to create kwargs for testing
fn create_kwargs(args: Vec<(&str, &str)>) -> Kwargs {
    Kwargs::from_iter(args.iter().map(|(k, v)| (*k, minijinja::Value::from(*v))))
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
