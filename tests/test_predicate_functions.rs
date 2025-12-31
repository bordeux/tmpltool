use minijinja::Value;
use minijinja::value::Kwargs;
use tmpltool::functions::predicates;

// ============================================================================
// Array Any Tests
// ============================================================================

#[test]
fn test_array_any_found() {
    let result = predicates::array_any_fn(Kwargs::from_iter(vec![
        ("array", Value::from(vec![1, 2, 3, 4, 5])),
        ("predicate", Value::from(3)),
    ]))
    .unwrap();

    assert_eq!(result, Value::from(true));
}

#[test]
fn test_array_any_not_found() {
    let result = predicates::array_any_fn(Kwargs::from_iter(vec![
        ("array", Value::from(vec![1, 2, 3, 4, 5])),
        ("predicate", Value::from(99)),
    ]))
    .unwrap();

    assert_eq!(result, Value::from(false));
}

#[test]
fn test_array_any_empty_array() {
    let empty: Vec<i32> = vec![];
    let result = predicates::array_any_fn(Kwargs::from_iter(vec![
        ("array", Value::from(empty)),
        ("predicate", Value::from(1)),
    ]))
    .unwrap();

    assert_eq!(result, Value::from(false));
}

#[test]
fn test_array_any_strings() {
    let result = predicates::array_any_fn(Kwargs::from_iter(vec![
        ("array", Value::from(vec!["apple", "banana", "cherry"])),
        ("predicate", Value::from("banana")),
    ]))
    .unwrap();

    assert_eq!(result, Value::from(true));
}

#[test]
fn test_array_any_first_element() {
    let result = predicates::array_any_fn(Kwargs::from_iter(vec![
        ("array", Value::from(vec![1, 2, 3])),
        ("predicate", Value::from(1)),
    ]))
    .unwrap();

    assert_eq!(result, Value::from(true));
}

#[test]
fn test_array_any_last_element() {
    let result = predicates::array_any_fn(Kwargs::from_iter(vec![
        ("array", Value::from(vec![1, 2, 3])),
        ("predicate", Value::from(3)),
    ]))
    .unwrap();

    assert_eq!(result, Value::from(true));
}

#[test]
fn test_array_any_error_not_array() {
    let result = predicates::array_any_fn(Kwargs::from_iter(vec![
        ("array", Value::from(42)),
        ("predicate", Value::from(1)),
    ]));

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires an array")
    );
}

// ============================================================================
// Array All Tests
// ============================================================================

#[test]
fn test_array_all_match() {
    let result = predicates::array_all_fn(Kwargs::from_iter(vec![
        ("array", Value::from(vec![5, 5, 5, 5])),
        ("predicate", Value::from(5)),
    ]))
    .unwrap();

    assert_eq!(result, Value::from(true));
}

#[test]
fn test_array_all_no_match() {
    let result = predicates::array_all_fn(Kwargs::from_iter(vec![
        ("array", Value::from(vec![5, 5, 3, 5])),
        ("predicate", Value::from(5)),
    ]))
    .unwrap();

    assert_eq!(result, Value::from(false));
}

#[test]
fn test_array_all_empty_array() {
    // Empty arrays should return true (vacuous truth)
    let empty: Vec<i32> = vec![];
    let result = predicates::array_all_fn(Kwargs::from_iter(vec![
        ("array", Value::from(empty)),
        ("predicate", Value::from(5)),
    ]))
    .unwrap();

    assert_eq!(result, Value::from(true));
}

#[test]
fn test_array_all_strings() {
    let result = predicates::array_all_fn(Kwargs::from_iter(vec![
        ("array", Value::from(vec!["test", "test", "test"])),
        ("predicate", Value::from("test")),
    ]))
    .unwrap();

    assert_eq!(result, Value::from(true));
}

#[test]
fn test_array_all_single_element_match() {
    let result = predicates::array_all_fn(Kwargs::from_iter(vec![
        ("array", Value::from(vec![42])),
        ("predicate", Value::from(42)),
    ]))
    .unwrap();

    assert_eq!(result, Value::from(true));
}

#[test]
fn test_array_all_single_element_no_match() {
    let result = predicates::array_all_fn(Kwargs::from_iter(vec![
        ("array", Value::from(vec![42])),
        ("predicate", Value::from(99)),
    ]))
    .unwrap();

    assert_eq!(result, Value::from(false));
}

#[test]
fn test_array_all_error_not_array() {
    let result = predicates::array_all_fn(Kwargs::from_iter(vec![
        ("array", Value::from("not an array")),
        ("predicate", Value::from(1)),
    ]));

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires an array")
    );
}

// ============================================================================
// Array Contains Tests
// ============================================================================

#[test]
fn test_array_contains_found() {
    let result = predicates::array_contains_fn(Kwargs::from_iter(vec![
        ("array", Value::from(vec![10, 20, 30, 40])),
        ("value", Value::from(30)),
    ]))
    .unwrap();

    assert_eq!(result, Value::from(true));
}

#[test]
fn test_array_contains_not_found() {
    let result = predicates::array_contains_fn(Kwargs::from_iter(vec![
        ("array", Value::from(vec![10, 20, 30, 40])),
        ("value", Value::from(99)),
    ]))
    .unwrap();

    assert_eq!(result, Value::from(false));
}

#[test]
fn test_array_contains_empty_array() {
    let empty: Vec<i32> = vec![];
    let result = predicates::array_contains_fn(Kwargs::from_iter(vec![
        ("array", Value::from(empty)),
        ("value", Value::from(1)),
    ]))
    .unwrap();

    assert_eq!(result, Value::from(false));
}

#[test]
fn test_array_contains_strings() {
    let result = predicates::array_contains_fn(Kwargs::from_iter(vec![
        ("array", Value::from(vec!["apple", "banana", "cherry"])),
        ("value", Value::from("banana")),
    ]))
    .unwrap();

    assert_eq!(result, Value::from(true));
}

#[test]
fn test_array_contains_first_element() {
    let result = predicates::array_contains_fn(Kwargs::from_iter(vec![
        ("array", Value::from(vec![1, 2, 3])),
        ("value", Value::from(1)),
    ]))
    .unwrap();

    assert_eq!(result, Value::from(true));
}

#[test]
fn test_array_contains_last_element() {
    let result = predicates::array_contains_fn(Kwargs::from_iter(vec![
        ("array", Value::from(vec![1, 2, 3])),
        ("value", Value::from(3)),
    ]))
    .unwrap();

    assert_eq!(result, Value::from(true));
}

#[test]
fn test_array_contains_duplicate_values() {
    let result = predicates::array_contains_fn(Kwargs::from_iter(vec![
        ("array", Value::from(vec![1, 2, 2, 3])),
        ("value", Value::from(2)),
    ]))
    .unwrap();

    assert_eq!(result, Value::from(true));
}

#[test]
fn test_array_contains_error_not_array() {
    let result = predicates::array_contains_fn(Kwargs::from_iter(vec![
        ("array", Value::from(42)),
        ("value", Value::from(1)),
    ]));

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires an array")
    );
}

// ============================================================================
// Starts With Tests
// ============================================================================

#[test]
fn test_starts_with_true() {
    let result = predicates::starts_with_fn(Kwargs::from_iter(vec![
        ("string", Value::from("Hello World")),
        ("prefix", Value::from("Hello")),
    ]))
    .unwrap();

    assert_eq!(result, Value::from(true));
}

#[test]
fn test_starts_with_false() {
    let result = predicates::starts_with_fn(Kwargs::from_iter(vec![
        ("string", Value::from("Hello World")),
        ("prefix", Value::from("World")),
    ]))
    .unwrap();

    assert_eq!(result, Value::from(false));
}

#[test]
fn test_starts_with_empty_prefix() {
    let result = predicates::starts_with_fn(Kwargs::from_iter(vec![
        ("string", Value::from("Hello")),
        ("prefix", Value::from("")),
    ]))
    .unwrap();

    assert_eq!(result, Value::from(true));
}

#[test]
fn test_starts_with_same_string() {
    let result = predicates::starts_with_fn(Kwargs::from_iter(vec![
        ("string", Value::from("test")),
        ("prefix", Value::from("test")),
    ]))
    .unwrap();

    assert_eq!(result, Value::from(true));
}

#[test]
fn test_starts_with_case_sensitive() {
    let result = predicates::starts_with_fn(Kwargs::from_iter(vec![
        ("string", Value::from("Hello")),
        ("prefix", Value::from("hello")),
    ]))
    .unwrap();

    assert_eq!(result, Value::from(false));
}

#[test]
fn test_starts_with_longer_prefix() {
    let result = predicates::starts_with_fn(Kwargs::from_iter(vec![
        ("string", Value::from("Hi")),
        ("prefix", Value::from("Hello")),
    ]))
    .unwrap();

    assert_eq!(result, Value::from(false));
}

#[test]
fn test_starts_with_file_path() {
    let result = predicates::starts_with_fn(Kwargs::from_iter(vec![
        ("string", Value::from("/usr/local/bin/app")),
        ("prefix", Value::from("/usr/")),
    ]))
    .unwrap();

    assert_eq!(result, Value::from(true));
}

// ============================================================================
// Ends With Tests
// ============================================================================

#[test]
fn test_ends_with_true() {
    let result = predicates::ends_with_fn(Kwargs::from_iter(vec![
        ("string", Value::from("readme.txt")),
        ("suffix", Value::from(".txt")),
    ]))
    .unwrap();

    assert_eq!(result, Value::from(true));
}

#[test]
fn test_ends_with_false() {
    let result = predicates::ends_with_fn(Kwargs::from_iter(vec![
        ("string", Value::from("readme.txt")),
        ("suffix", Value::from(".md")),
    ]))
    .unwrap();

    assert_eq!(result, Value::from(false));
}

#[test]
fn test_ends_with_empty_suffix() {
    let result = predicates::ends_with_fn(Kwargs::from_iter(vec![
        ("string", Value::from("test")),
        ("suffix", Value::from("")),
    ]))
    .unwrap();

    assert_eq!(result, Value::from(true));
}

#[test]
fn test_ends_with_same_string() {
    let result = predicates::ends_with_fn(Kwargs::from_iter(vec![
        ("string", Value::from("test")),
        ("suffix", Value::from("test")),
    ]))
    .unwrap();

    assert_eq!(result, Value::from(true));
}

#[test]
fn test_ends_with_case_sensitive() {
    let result = predicates::ends_with_fn(Kwargs::from_iter(vec![
        ("string", Value::from("Hello")),
        ("suffix", Value::from("LO")),
    ]))
    .unwrap();

    assert_eq!(result, Value::from(false));
}

#[test]
fn test_ends_with_longer_suffix() {
    let result = predicates::ends_with_fn(Kwargs::from_iter(vec![
        ("string", Value::from("Hi")),
        ("suffix", Value::from("Hello")),
    ]))
    .unwrap();

    assert_eq!(result, Value::from(false));
}

#[test]
fn test_ends_with_url() {
    let result = predicates::ends_with_fn(Kwargs::from_iter(vec![
        ("string", Value::from("https://example.com")),
        ("suffix", Value::from(".com")),
    ]))
    .unwrap();

    assert_eq!(result, Value::from(true));
}

#[test]
fn test_ends_with_multiple_extensions() {
    let result = predicates::ends_with_fn(Kwargs::from_iter(vec![
        ("string", Value::from("archive.tar.gz")),
        ("suffix", Value::from(".tar.gz")),
    ]))
    .unwrap();

    assert_eq!(result, Value::from(true));
}

// ============================================================================
// Error Handling Tests
// ============================================================================

#[test]
fn test_array_any_missing_array() {
    let result = predicates::array_any_fn(Kwargs::from_iter(vec![("predicate", Value::from(1))]));

    assert!(result.is_err());
}

#[test]
fn test_array_all_missing_predicate() {
    let result = predicates::array_all_fn(Kwargs::from_iter(vec![(
        "array",
        Value::from(vec![1, 2, 3]),
    )]));

    assert!(result.is_err());
}

#[test]
fn test_array_contains_missing_value() {
    let result = predicates::array_contains_fn(Kwargs::from_iter(vec![(
        "array",
        Value::from(vec![1, 2, 3]),
    )]));

    assert!(result.is_err());
}

#[test]
fn test_starts_with_missing_string() {
    let result =
        predicates::starts_with_fn(Kwargs::from_iter(vec![("prefix", Value::from("test"))]));

    assert!(result.is_err());
}

#[test]
fn test_ends_with_missing_suffix() {
    let result = predicates::ends_with_fn(Kwargs::from_iter(vec![("string", Value::from("test"))]));

    assert!(result.is_err());
}
