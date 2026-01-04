use minijinja::Value;
use minijinja::value::Kwargs;
use tmpltool::filter_functions::FilterFunction;
use tmpltool::filter_functions::math::{Abs, Ceil, Floor, Round};
use tmpltool::functions::Function;
use tmpltool::functions::math::{Max, Min, Percentage};

// ============================================================================
// Min Tests
// ============================================================================

#[test]
fn test_min_integers() {
    let result = Min::call(Kwargs::from_iter(vec![
        ("a", Value::from(10)),
        ("b", Value::from(20)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "10");
}

#[test]
fn test_min_floats() {
    let result = Min::call(Kwargs::from_iter(vec![
        ("a", Value::from(3.25)),
        ("b", Value::from(2.75)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "2.75");
}

#[test]
fn test_min_mixed() {
    let result = Min::call(Kwargs::from_iter(vec![
        ("a", Value::from(5)),
        ("b", Value::from(5.5)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "5");
}

#[test]
fn test_min_negative() {
    let result = Min::call(Kwargs::from_iter(vec![
        ("a", Value::from(-10)),
        ("b", Value::from(-5)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "-10");
}

#[test]
fn test_min_error_non_numeric_a() {
    let result = Min::call(Kwargs::from_iter(vec![
        ("a", Value::from("test")),
        ("b", Value::from(10)),
    ]));

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires numeric values")
    );
}

#[test]
fn test_min_error_non_numeric_b() {
    let result = Min::call(Kwargs::from_iter(vec![
        ("a", Value::from(10)),
        ("b", Value::from("test")),
    ]));

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires numeric values")
    );
}

#[test]
fn test_min_missing_param() {
    let result = Min::call(Kwargs::from_iter(vec![("a", Value::from(10))]));

    assert!(result.is_err());
}

// ============================================================================
// Max Tests
// ============================================================================

#[test]
fn test_max_integers() {
    let result = Max::call(Kwargs::from_iter(vec![
        ("a", Value::from(10)),
        ("b", Value::from(20)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "20");
}

#[test]
fn test_max_floats() {
    let result = Max::call(Kwargs::from_iter(vec![
        ("a", Value::from(3.25)),
        ("b", Value::from(2.75)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "3.25");
}

#[test]
fn test_max_mixed() {
    let result = Max::call(Kwargs::from_iter(vec![
        ("a", Value::from(5)),
        ("b", Value::from(5.5)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "5.5");
}

#[test]
fn test_max_negative() {
    let result = Max::call(Kwargs::from_iter(vec![
        ("a", Value::from(-10)),
        ("b", Value::from(-5)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "-5");
}

#[test]
fn test_max_error_non_numeric() {
    let result = Max::call(Kwargs::from_iter(vec![
        ("a", Value::from("test")),
        ("b", Value::from(10)),
    ]));

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires numeric values")
    );
}

// ============================================================================
// Abs Tests
// ============================================================================

#[test]
fn test_abs_positive() {
    let result =
        Abs::call_as_function(Kwargs::from_iter(vec![("number", Value::from(42))])).unwrap();

    assert_eq!(result.to_string(), "42");
}

#[test]
fn test_abs_negative() {
    let result =
        Abs::call_as_function(Kwargs::from_iter(vec![("number", Value::from(-42))])).unwrap();

    assert_eq!(result.to_string(), "42");
}

#[test]
fn test_abs_zero() {
    let result =
        Abs::call_as_function(Kwargs::from_iter(vec![("number", Value::from(0))])).unwrap();

    assert_eq!(result.to_string(), "0");
}

#[test]
fn test_abs_float() {
    let result =
        Abs::call_as_function(Kwargs::from_iter(vec![("number", Value::from(-3.25))])).unwrap();

    assert_eq!(result.to_string(), "3.25");
}

#[test]
fn test_abs_error_non_numeric() {
    let result = Abs::call_as_function(Kwargs::from_iter(vec![("number", Value::from("test"))]));

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires a numeric value")
    );
}

#[test]
fn test_abs_missing_param() {
    let result = Abs::call_as_function(Kwargs::from_iter(vec![("dummy", Value::from(0))]));

    assert!(result.is_err());
}

#[test]
fn test_abs_filter_syntax() {
    let result = Abs::call_as_filter(
        &Value::from(-42),
        Kwargs::from_iter(Vec::<(&str, Value)>::new()),
    )
    .unwrap();

    assert_eq!(result.to_string(), "42");
}

// ============================================================================
// Round Tests
// ============================================================================

#[test]
fn test_round_default() {
    let result =
        Round::call_as_function(Kwargs::from_iter(vec![("number", Value::from(3.7))])).unwrap();

    assert_eq!(result.to_string(), "4");
}

#[test]
fn test_round_down() {
    let result =
        Round::call_as_function(Kwargs::from_iter(vec![("number", Value::from(3.4))])).unwrap();

    assert_eq!(result.to_string(), "3");
}

#[test]
fn test_round_two_decimals() {
    let result = Round::call_as_function(Kwargs::from_iter(vec![
        ("number", Value::from(2.34567)),
        ("decimals", Value::from(2)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "2.35");
}

#[test]
fn test_round_four_decimals() {
    let result = Round::call_as_function(Kwargs::from_iter(vec![
        ("number", Value::from(2.345678)),
        ("decimals", Value::from(4)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "2.3457");
}

#[test]
fn test_round_zero_decimals_explicit() {
    let result = Round::call_as_function(Kwargs::from_iter(vec![
        ("number", Value::from(19.999)),
        ("decimals", Value::from(0)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "20");
}

#[test]
fn test_round_negative_number() {
    let result = Round::call_as_function(Kwargs::from_iter(vec![
        ("number", Value::from(-3.7)),
        ("decimals", Value::from(0)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "-4");
}

#[test]
fn test_round_error_negative_decimals() {
    let result = Round::call_as_function(Kwargs::from_iter(vec![
        ("number", Value::from(2.75)),
        ("decimals", Value::from(-1)),
    ]));

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("decimals must be non-negative")
    );
}

#[test]
fn test_round_error_non_numeric() {
    let result = Round::call_as_function(Kwargs::from_iter(vec![("number", Value::from("test"))]));

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires a numeric value")
    );
}

#[test]
fn test_round_filter_syntax() {
    let result = Round::call_as_filter(
        &Value::from(2.34567),
        Kwargs::from_iter(vec![("decimals", Value::from(2))]),
    )
    .unwrap();

    assert_eq!(result.to_string(), "2.35");
}

// ============================================================================
// Ceil Tests
// ============================================================================

#[test]
fn test_ceil_basic() {
    let result =
        Ceil::call_as_function(Kwargs::from_iter(vec![("number", Value::from(3.1))])).unwrap();

    assert_eq!(result.to_string(), "4");
}

#[test]
fn test_ceil_exact() {
    let result =
        Ceil::call_as_function(Kwargs::from_iter(vec![("number", Value::from(3.0))])).unwrap();

    assert_eq!(result.to_string(), "3");
}

#[test]
fn test_ceil_negative() {
    let result =
        Ceil::call_as_function(Kwargs::from_iter(vec![("number", Value::from(-3.9))])).unwrap();

    assert_eq!(result.to_string(), "-3");
}

#[test]
fn test_ceil_small_fraction() {
    let result =
        Ceil::call_as_function(Kwargs::from_iter(vec![("number", Value::from(3.001))])).unwrap();

    assert_eq!(result.to_string(), "4");
}

#[test]
fn test_ceil_error_non_numeric() {
    let result = Ceil::call_as_function(Kwargs::from_iter(vec![("number", Value::from("test"))]));

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires a numeric value")
    );
}

#[test]
fn test_ceil_filter_syntax() {
    let result = Ceil::call_as_filter(
        &Value::from(3.1),
        Kwargs::from_iter(Vec::<(&str, Value)>::new()),
    )
    .unwrap();

    assert_eq!(result.to_string(), "4");
}

// ============================================================================
// Floor Tests
// ============================================================================

#[test]
fn test_floor_basic() {
    let result =
        Floor::call_as_function(Kwargs::from_iter(vec![("number", Value::from(3.9))])).unwrap();

    assert_eq!(result.to_string(), "3");
}

#[test]
fn test_floor_exact() {
    let result =
        Floor::call_as_function(Kwargs::from_iter(vec![("number", Value::from(3.0))])).unwrap();

    assert_eq!(result.to_string(), "3");
}

#[test]
fn test_floor_negative() {
    let result =
        Floor::call_as_function(Kwargs::from_iter(vec![("number", Value::from(-3.1))])).unwrap();

    assert_eq!(result.to_string(), "-4");
}

#[test]
fn test_floor_small_fraction() {
    let result =
        Floor::call_as_function(Kwargs::from_iter(vec![("number", Value::from(3.999))])).unwrap();

    assert_eq!(result.to_string(), "3");
}

#[test]
fn test_floor_error_non_numeric() {
    let result = Floor::call_as_function(Kwargs::from_iter(vec![("number", Value::from("test"))]));

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires a numeric value")
    );
}

#[test]
fn test_floor_filter_syntax() {
    let result = Floor::call_as_filter(
        &Value::from(3.9),
        Kwargs::from_iter(Vec::<(&str, Value)>::new()),
    )
    .unwrap();

    assert_eq!(result.to_string(), "3");
}

// ============================================================================
// Percentage Tests
// ============================================================================

#[test]
fn test_percentage_basic() {
    let result = Percentage::call(Kwargs::from_iter(vec![
        ("value", Value::from(25)),
        ("total", Value::from(100)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "25");
}

#[test]
fn test_percentage_decimal() {
    let result = Percentage::call(Kwargs::from_iter(vec![
        ("value", Value::from(7)),
        ("total", Value::from(10)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "70");
}

#[test]
fn test_percentage_with_rounding() {
    let result = Percentage::call(Kwargs::from_iter(vec![
        ("value", Value::from(1)),
        ("total", Value::from(3)),
    ]))
    .unwrap();

    // 1/3 * 100 = 33.333...
    let percentage: f64 = result.to_string().parse().unwrap();
    assert!((percentage - 33.333333).abs() < 0.001);
}

#[test]
fn test_percentage_floats() {
    let result = Percentage::call(Kwargs::from_iter(vec![
        ("value", Value::from(450.0)),
        ("total", Value::from(500.0)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "90");
}

#[test]
fn test_percentage_over_100() {
    let result = Percentage::call(Kwargs::from_iter(vec![
        ("value", Value::from(150)),
        ("total", Value::from(100)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "150");
}

#[test]
fn test_percentage_error_zero_total() {
    let result = Percentage::call(Kwargs::from_iter(vec![
        ("value", Value::from(25)),
        ("total", Value::from(0)),
    ]));

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("total cannot be zero")
    );
}

#[test]
fn test_percentage_error_non_numeric_value() {
    let result = Percentage::call(Kwargs::from_iter(vec![
        ("value", Value::from("test")),
        ("total", Value::from(100)),
    ]));

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires numeric value")
    );
}

#[test]
fn test_percentage_error_non_numeric_total() {
    let result = Percentage::call(Kwargs::from_iter(vec![
        ("value", Value::from(25)),
        ("total", Value::from("test")),
    ]));

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires numeric total")
    );
}

#[test]
fn test_percentage_missing_params() {
    let result = Percentage::call(Kwargs::from_iter(vec![("value", Value::from(25))]));

    assert!(result.is_err());
}

// ============================================================================
// Additional error case tests for better coverage
// ============================================================================

#[test]
fn test_min_error_missing_a() {
    let result = Min::call(Kwargs::from_iter(vec![("b", Value::from(20))]));
    assert!(result.is_err());
}

#[test]
fn test_min_error_missing_b() {
    let result = Min::call(Kwargs::from_iter(vec![("a", Value::from(10))]));
    assert!(result.is_err());
}

#[test]
fn test_max_error_missing_a() {
    let result = Max::call(Kwargs::from_iter(vec![("b", Value::from(20))]));
    assert!(result.is_err());
}

#[test]
fn test_max_error_missing_b() {
    let result = Max::call(Kwargs::from_iter(vec![("a", Value::from(10))]));
    assert!(result.is_err());
}

// Note: abs, round, ceil, floor tests removed - functions now in filter_functions/math.rs

#[test]
fn test_percentage_error_missing_value() {
    let result = Percentage::call(Kwargs::from_iter(vec![("total", Value::from(100))]));
    assert!(result.is_err());
}

#[test]
fn test_min_with_zero() {
    let result = Min::call(Kwargs::from_iter(vec![
        ("a", Value::from(0)),
        ("b", Value::from(5)),
    ]))
    .unwrap();
    assert_eq!(result.to_string(), "0");
}

#[test]
fn test_max_with_zero() {
    let result = Max::call(Kwargs::from_iter(vec![
        ("a", Value::from(0)),
        ("b", Value::from(-5)),
    ]))
    .unwrap();
    assert_eq!(result.to_string(), "0");
}

// Note: abs_with_zero, round_with_*, ceil_negative_direct, floor_negative_direct
// tests removed - functions now in filter_functions/math.rs

#[test]
fn test_percentage_very_small() {
    let result = Percentage::call(Kwargs::from_iter(vec![
        ("value", Value::from(1)),
        ("total", Value::from(10000)),
    ]))
    .unwrap();
    assert_eq!(result.to_string(), "0.01");
}
