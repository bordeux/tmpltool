//! Tests for math filter-functions (abs, round, ceil, floor).
//!
//! Tests both function and filter syntax.

use minijinja::Value;
use minijinja::value::Kwargs;
use tmpltool::filter_functions::FilterFunction;
use tmpltool::filter_functions::math::{Abs, Ceil, Floor, Round};

/// Helper to create empty kwargs
fn empty_kwargs() -> Kwargs {
    Kwargs::from_iter(Vec::<(&str, Value)>::new())
}

// ============================================
// Abs tests
// ============================================

#[test]
fn test_abs_filter_positive() {
    let result = Abs::call_as_filter(&Value::from(42), empty_kwargs()).unwrap();
    assert_eq!(result.as_i64().unwrap(), 42);
}

#[test]
fn test_abs_filter_negative() {
    let result = Abs::call_as_filter(&Value::from(-42), empty_kwargs()).unwrap();
    assert_eq!(result.as_i64().unwrap(), 42);
}

#[test]
fn test_abs_function_syntax() {
    let kwargs = Kwargs::from_iter(vec![("number", Value::from(-42))]);
    let result = Abs::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_i64().unwrap(), 42);
}

#[test]
fn test_abs_filter_float() {
    let result = Abs::call_as_filter(&Value::from(-3.25), empty_kwargs()).unwrap();
    let val: f64 = serde_json::from_value(serde_json::to_value(&result).unwrap()).unwrap();
    assert!((val - 3.25).abs() < 0.001);
}

#[test]
fn test_abs_filter_zero() {
    let result = Abs::call_as_filter(&Value::from(0), empty_kwargs()).unwrap();
    assert_eq!(result.as_i64().unwrap(), 0);
}

#[test]
fn test_abs_error_not_number() {
    let result = Abs::call_as_filter(&Value::from("not a number"), empty_kwargs());
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires a numeric value")
    );
}

// ============================================
// Round tests
// ============================================

#[test]
fn test_round_filter_default() {
    let result = Round::call_as_filter(&Value::from(3.6), empty_kwargs()).unwrap();
    assert_eq!(result.as_i64().unwrap(), 4);
}

#[test]
fn test_round_filter_down() {
    let result = Round::call_as_filter(&Value::from(3.4), empty_kwargs()).unwrap();
    assert_eq!(result.as_i64().unwrap(), 3);
}

#[test]
fn test_round_filter_decimals() {
    let kwargs = Kwargs::from_iter(vec![("decimals", Value::from(2))]);
    let result = Round::call_as_filter(&Value::from(3.12345), kwargs).unwrap();
    let val: f64 = serde_json::from_value(serde_json::to_value(&result).unwrap()).unwrap();
    assert!((val - 3.12).abs() < 0.001);
}

#[test]
fn test_round_function_syntax() {
    let kwargs = Kwargs::from_iter(vec![
        ("number", Value::from(3.12345)),
        ("decimals", Value::from(2)),
    ]);
    let result = Round::call_as_function(kwargs).unwrap();
    let val: f64 = serde_json::from_value(serde_json::to_value(&result).unwrap()).unwrap();
    assert!((val - 3.12).abs() < 0.001);
}

#[test]
fn test_round_negative_decimals_error() {
    let kwargs = Kwargs::from_iter(vec![("decimals", Value::from(-1))]);
    let result = Round::call_as_filter(&Value::from(3.25), kwargs);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("non-negative"));
}

#[test]
fn test_round_error_not_number() {
    let result = Round::call_as_filter(&Value::from("not a number"), empty_kwargs());
    assert!(result.is_err());
}

// ============================================
// Ceil tests
// ============================================

#[test]
fn test_ceil_filter_up() {
    let result = Ceil::call_as_filter(&Value::from(3.1), empty_kwargs()).unwrap();
    assert_eq!(result.as_i64().unwrap(), 4);
}

#[test]
fn test_ceil_filter_exact() {
    let result = Ceil::call_as_filter(&Value::from(3.0), empty_kwargs()).unwrap();
    assert_eq!(result.as_i64().unwrap(), 3);
}

#[test]
fn test_ceil_function_syntax() {
    let kwargs = Kwargs::from_iter(vec![("number", Value::from(3.1))]);
    let result = Ceil::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_i64().unwrap(), 4);
}

#[test]
fn test_ceil_filter_negative() {
    let result = Ceil::call_as_filter(&Value::from(-3.9), empty_kwargs()).unwrap();
    assert_eq!(result.as_i64().unwrap(), -3);
}

#[test]
fn test_ceil_error_not_number() {
    let result = Ceil::call_as_filter(&Value::from("not a number"), empty_kwargs());
    assert!(result.is_err());
}

// ============================================
// Floor tests
// ============================================

#[test]
fn test_floor_filter_down() {
    let result = Floor::call_as_filter(&Value::from(3.9), empty_kwargs()).unwrap();
    assert_eq!(result.as_i64().unwrap(), 3);
}

#[test]
fn test_floor_filter_exact() {
    let result = Floor::call_as_filter(&Value::from(3.0), empty_kwargs()).unwrap();
    assert_eq!(result.as_i64().unwrap(), 3);
}

#[test]
fn test_floor_function_syntax() {
    let kwargs = Kwargs::from_iter(vec![("number", Value::from(3.9))]);
    let result = Floor::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_i64().unwrap(), 3);
}

#[test]
fn test_floor_filter_negative() {
    let result = Floor::call_as_filter(&Value::from(-3.1), empty_kwargs()).unwrap();
    assert_eq!(result.as_i64().unwrap(), -4);
}

#[test]
fn test_floor_error_not_number() {
    let result = Floor::call_as_filter(&Value::from("not a number"), empty_kwargs());
    assert!(result.is_err());
}
