use minijinja::Value;
use minijinja::value::Kwargs;
use tmpltool::functions::math;

// ============================================================================
// Min Tests
// ============================================================================

#[test]
fn test_min_integers() {
    let result = math::min_fn(Kwargs::from_iter(vec![
        ("a", Value::from(10)),
        ("b", Value::from(20)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "10");
}

#[test]
fn test_min_floats() {
    let result = math::min_fn(Kwargs::from_iter(vec![
        ("a", Value::from(3.25)),
        ("b", Value::from(2.75)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "2.75");
}

#[test]
fn test_min_mixed() {
    let result = math::min_fn(Kwargs::from_iter(vec![
        ("a", Value::from(5)),
        ("b", Value::from(5.5)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "5");
}

#[test]
fn test_min_negative() {
    let result = math::min_fn(Kwargs::from_iter(vec![
        ("a", Value::from(-10)),
        ("b", Value::from(-5)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "-10");
}

#[test]
fn test_min_error_non_numeric_a() {
    let result = math::min_fn(Kwargs::from_iter(vec![
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
    let result = math::min_fn(Kwargs::from_iter(vec![
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
    let result = math::min_fn(Kwargs::from_iter(vec![("a", Value::from(10))]));

    assert!(result.is_err());
}

// ============================================================================
// Max Tests
// ============================================================================

#[test]
fn test_max_integers() {
    let result = math::max_fn(Kwargs::from_iter(vec![
        ("a", Value::from(10)),
        ("b", Value::from(20)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "20");
}

#[test]
fn test_max_floats() {
    let result = math::max_fn(Kwargs::from_iter(vec![
        ("a", Value::from(3.25)),
        ("b", Value::from(2.75)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "3.25");
}

#[test]
fn test_max_mixed() {
    let result = math::max_fn(Kwargs::from_iter(vec![
        ("a", Value::from(5)),
        ("b", Value::from(5.5)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "5.5");
}

#[test]
fn test_max_negative() {
    let result = math::max_fn(Kwargs::from_iter(vec![
        ("a", Value::from(-10)),
        ("b", Value::from(-5)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "-5");
}

#[test]
fn test_max_error_non_numeric() {
    let result = math::max_fn(Kwargs::from_iter(vec![
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
    let result = math::abs_fn(Kwargs::from_iter(vec![("number", Value::from(42))])).unwrap();

    assert_eq!(result.to_string(), "42");
}

#[test]
fn test_abs_negative() {
    let result = math::abs_fn(Kwargs::from_iter(vec![("number", Value::from(-42))])).unwrap();

    assert_eq!(result.to_string(), "42");
}

#[test]
fn test_abs_zero() {
    let result = math::abs_fn(Kwargs::from_iter(vec![("number", Value::from(0))])).unwrap();

    assert_eq!(result.to_string(), "0");
}

#[test]
fn test_abs_float() {
    let result = math::abs_fn(Kwargs::from_iter(vec![("number", Value::from(-3.25))])).unwrap();

    assert_eq!(result.to_string(), "3.25");
}

#[test]
fn test_abs_error_non_numeric() {
    let result = math::abs_fn(Kwargs::from_iter(vec![("number", Value::from("test"))]));

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
    let result = math::abs_fn(Kwargs::from_iter(vec![("dummy", Value::from(0))]));

    assert!(result.is_err());
}

// ============================================================================
// Round Tests
// ============================================================================

#[test]
fn test_round_default() {
    let result = math::round_fn(Kwargs::from_iter(vec![("number", Value::from(3.7))])).unwrap();

    assert_eq!(result.to_string(), "4");
}

#[test]
fn test_round_down() {
    let result = math::round_fn(Kwargs::from_iter(vec![("number", Value::from(3.4))])).unwrap();

    assert_eq!(result.to_string(), "3");
}

#[test]
fn test_round_two_decimals() {
    let result = math::round_fn(Kwargs::from_iter(vec![
        ("number", Value::from(2.34567)),
        ("decimals", Value::from(2)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "2.35");
}

#[test]
fn test_round_four_decimals() {
    let result = math::round_fn(Kwargs::from_iter(vec![
        ("number", Value::from(2.345678)),
        ("decimals", Value::from(4)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "2.3457");
}

#[test]
fn test_round_zero_decimals_explicit() {
    let result = math::round_fn(Kwargs::from_iter(vec![
        ("number", Value::from(19.999)),
        ("decimals", Value::from(0)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "20");
}

#[test]
fn test_round_negative_number() {
    let result = math::round_fn(Kwargs::from_iter(vec![
        ("number", Value::from(-3.7)),
        ("decimals", Value::from(0)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "-4");
}

#[test]
fn test_round_error_negative_decimals() {
    let result = math::round_fn(Kwargs::from_iter(vec![
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
    let result = math::round_fn(Kwargs::from_iter(vec![("number", Value::from("test"))]));

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires a numeric value")
    );
}

// ============================================================================
// Ceil Tests
// ============================================================================

#[test]
fn test_ceil_basic() {
    let result = math::ceil_fn(Kwargs::from_iter(vec![("number", Value::from(3.1))])).unwrap();

    assert_eq!(result.to_string(), "4");
}

#[test]
fn test_ceil_exact() {
    let result = math::ceil_fn(Kwargs::from_iter(vec![("number", Value::from(3.0))])).unwrap();

    assert_eq!(result.to_string(), "3");
}

#[test]
fn test_ceil_negative() {
    let result = math::ceil_fn(Kwargs::from_iter(vec![("number", Value::from(-3.9))])).unwrap();

    assert_eq!(result.to_string(), "-3");
}

#[test]
fn test_ceil_small_fraction() {
    let result = math::ceil_fn(Kwargs::from_iter(vec![("number", Value::from(3.001))])).unwrap();

    assert_eq!(result.to_string(), "4");
}

#[test]
fn test_ceil_error_non_numeric() {
    let result = math::ceil_fn(Kwargs::from_iter(vec![("number", Value::from("test"))]));

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires a numeric value")
    );
}

// ============================================================================
// Floor Tests
// ============================================================================

#[test]
fn test_floor_basic() {
    let result = math::floor_fn(Kwargs::from_iter(vec![("number", Value::from(3.9))])).unwrap();

    assert_eq!(result.to_string(), "3");
}

#[test]
fn test_floor_exact() {
    let result = math::floor_fn(Kwargs::from_iter(vec![("number", Value::from(3.0))])).unwrap();

    assert_eq!(result.to_string(), "3");
}

#[test]
fn test_floor_negative() {
    let result = math::floor_fn(Kwargs::from_iter(vec![("number", Value::from(-3.1))])).unwrap();

    assert_eq!(result.to_string(), "-4");
}

#[test]
fn test_floor_small_fraction() {
    let result = math::floor_fn(Kwargs::from_iter(vec![("number", Value::from(3.999))])).unwrap();

    assert_eq!(result.to_string(), "3");
}

#[test]
fn test_floor_error_non_numeric() {
    let result = math::floor_fn(Kwargs::from_iter(vec![("number", Value::from("test"))]));

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires a numeric value")
    );
}

// ============================================================================
// Percentage Tests
// ============================================================================

#[test]
fn test_percentage_basic() {
    let result = math::percentage_fn(Kwargs::from_iter(vec![
        ("value", Value::from(25)),
        ("total", Value::from(100)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "25.0");
}

#[test]
fn test_percentage_decimal() {
    let result = math::percentage_fn(Kwargs::from_iter(vec![
        ("value", Value::from(7)),
        ("total", Value::from(10)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "70.0");
}

#[test]
fn test_percentage_with_rounding() {
    let result = math::percentage_fn(Kwargs::from_iter(vec![
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
    let result = math::percentage_fn(Kwargs::from_iter(vec![
        ("value", Value::from(450.0)),
        ("total", Value::from(500.0)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "90.0");
}

#[test]
fn test_percentage_over_100() {
    let result = math::percentage_fn(Kwargs::from_iter(vec![
        ("value", Value::from(150)),
        ("total", Value::from(100)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "150.0");
}

#[test]
fn test_percentage_error_zero_total() {
    let result = math::percentage_fn(Kwargs::from_iter(vec![
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
    let result = math::percentage_fn(Kwargs::from_iter(vec![
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
    let result = math::percentage_fn(Kwargs::from_iter(vec![
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
    let result = math::percentage_fn(Kwargs::from_iter(vec![("value", Value::from(25))]));

    assert!(result.is_err());
}
