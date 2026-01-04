use minijinja::Value;
use minijinja::value::Kwargs;
use tmpltool::functions::Function;
use tmpltool::functions::logic::{Coalesce, Default, InRange, Ternary};

// ============================================================================
// Default Tests
// ============================================================================

#[test]
fn test_default_with_truthy_value() {
    let result = Default::call(Kwargs::from_iter(vec![
        ("value", Value::from("Hello")),
        ("default", Value::from("N/A")),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "Hello");
}

#[test]
fn test_default_with_empty_string() {
    let result = Default::call(Kwargs::from_iter(vec![
        ("value", Value::from("")),
        ("default", Value::from("N/A")),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "N/A");
}

#[test]
fn test_default_with_none() {
    let result = Default::call(Kwargs::from_iter(vec![
        ("value", Value::UNDEFINED),
        ("default", Value::from("Not set")),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "Not set");
}

#[test]
fn test_default_with_false() {
    let result = Default::call(Kwargs::from_iter(vec![
        ("value", Value::from(false)),
        ("default", Value::from("Default")),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "Default");
}

#[test]
fn test_default_with_true() {
    let result = Default::call(Kwargs::from_iter(vec![
        ("value", Value::from(true)),
        ("default", Value::from("Default")),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "true");
}

#[test]
fn test_default_with_number() {
    let result = Default::call(Kwargs::from_iter(vec![
        ("value", Value::from(42)),
        ("default", Value::from(0)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "42");
}

#[test]
fn test_default_with_empty_array() {
    let empty: Vec<i32> = vec![];
    let result = Default::call(Kwargs::from_iter(vec![
        ("value", Value::from(empty)),
        ("default", Value::from("Empty")),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "Empty");
}

#[test]
fn test_default_with_non_empty_array() {
    let result = Default::call(Kwargs::from_iter(vec![
        ("value", Value::from(vec![1, 2, 3])),
        ("default", Value::from("Empty")),
    ]))
    .unwrap();

    assert!(result.to_string().contains("1"));
}

#[test]
fn test_default_missing_params() {
    let result = Default::call(Kwargs::from_iter(vec![("value", Value::from("test"))]));

    assert!(result.is_err());
}

// ============================================================================
// Coalesce Tests
// ============================================================================

#[test]
fn test_coalesce_first_non_null() {
    let values = serde_json::json!([null, null, "found", "other"]);

    let result = Coalesce::call(Kwargs::from_iter(vec![(
        "values",
        Value::from_serialize(&values),
    )]))
    .unwrap();

    assert_eq!(result.to_string(), "found");
}

#[test]
fn test_coalesce_first_value() {
    let values = serde_json::json!(["first", "second", "third"]);

    let result = Coalesce::call(Kwargs::from_iter(vec![(
        "values",
        Value::from_serialize(&values),
    )]))
    .unwrap();

    assert_eq!(result.to_string(), "first");
}

#[test]
fn test_coalesce_all_null() {
    let values = serde_json::json!([null, null]);

    let result = Coalesce::call(Kwargs::from_iter(vec![(
        "values",
        Value::from_serialize(&values),
    )]))
    .unwrap();

    assert!(result.is_undefined());
}

#[test]
fn test_coalesce_empty_array() {
    let empty: Vec<serde_json::Value> = vec![];

    let result = Coalesce::call(Kwargs::from_iter(vec![(
        "values",
        Value::from_serialize(&empty),
    )]))
    .unwrap();

    assert!(result.is_undefined());
}

#[test]
fn test_coalesce_with_numbers() {
    let values = serde_json::json!([null, 0, 42]);

    let result = Coalesce::call(Kwargs::from_iter(vec![(
        "values",
        Value::from_serialize(&values),
    )]))
    .unwrap();

    assert_eq!(result.to_string(), "0");
}

#[test]
fn test_coalesce_with_false() {
    let values = serde_json::json!([null, false, true]);

    let result = Coalesce::call(Kwargs::from_iter(vec![(
        "values",
        Value::from_serialize(&values),
    )]))
    .unwrap();

    assert_eq!(result.to_string(), "false");
}

#[test]
fn test_coalesce_error_not_array() {
    let result = Coalesce::call(Kwargs::from_iter(vec![("values", Value::from("test"))]));

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires an array")
    );
}

#[test]
fn test_coalesce_missing_param() {
    let result = Coalesce::call(Kwargs::from_iter(vec![("dummy", Value::from(0))]));

    assert!(result.is_err());
}

// ============================================================================
// Ternary Tests
// ============================================================================

#[test]
fn test_ternary_true_condition() {
    let result = Ternary::call(Kwargs::from_iter(vec![
        ("condition", Value::from(true)),
        ("true_val", Value::from("Yes")),
        ("false_val", Value::from("No")),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "Yes");
}

#[test]
fn test_ternary_false_condition() {
    let result = Ternary::call(Kwargs::from_iter(vec![
        ("condition", Value::from(false)),
        ("true_val", Value::from("Yes")),
        ("false_val", Value::from("No")),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "No");
}

#[test]
fn test_ternary_truthy_string() {
    let result = Ternary::call(Kwargs::from_iter(vec![
        ("condition", Value::from("hello")),
        ("true_val", Value::from("Yes")),
        ("false_val", Value::from("No")),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "Yes");
}

#[test]
fn test_ternary_falsy_empty_string() {
    let result = Ternary::call(Kwargs::from_iter(vec![
        ("condition", Value::from("")),
        ("true_val", Value::from("Yes")),
        ("false_val", Value::from("No")),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "No");
}

#[test]
fn test_ternary_truthy_number() {
    let result = Ternary::call(Kwargs::from_iter(vec![
        ("condition", Value::from(1)),
        ("true_val", Value::from("Yes")),
        ("false_val", Value::from("No")),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "Yes");
}

#[test]
fn test_ternary_with_numbers() {
    let result = Ternary::call(Kwargs::from_iter(vec![
        ("condition", Value::from(true)),
        ("true_val", Value::from(100)),
        ("false_val", Value::from(200)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "100");
}

#[test]
fn test_ternary_undefined_condition() {
    let result = Ternary::call(Kwargs::from_iter(vec![
        ("condition", Value::UNDEFINED),
        ("true_val", Value::from("Yes")),
        ("false_val", Value::from("No")),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "No");
}

#[test]
fn test_ternary_missing_params() {
    let result = Ternary::call(Kwargs::from_iter(vec![
        ("condition", Value::from(true)),
        ("true_val", Value::from("Yes")),
    ]));

    assert!(result.is_err());
}

// ============================================================================
// In Range Tests
// ============================================================================

#[test]
fn test_in_range_within_range() {
    let result = InRange::call(Kwargs::from_iter(vec![
        ("value", Value::from(50)),
        ("min", Value::from(0)),
        ("max", Value::from(100)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "true");
}

#[test]
fn test_in_range_below_range() {
    let result = InRange::call(Kwargs::from_iter(vec![
        ("value", Value::from(-10)),
        ("min", Value::from(0)),
        ("max", Value::from(100)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "false");
}

#[test]
fn test_in_range_above_range() {
    let result = InRange::call(Kwargs::from_iter(vec![
        ("value", Value::from(150)),
        ("min", Value::from(0)),
        ("max", Value::from(100)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "false");
}

#[test]
fn test_in_range_at_min() {
    let result = InRange::call(Kwargs::from_iter(vec![
        ("value", Value::from(0)),
        ("min", Value::from(0)),
        ("max", Value::from(100)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "true");
}

#[test]
fn test_in_range_at_max() {
    let result = InRange::call(Kwargs::from_iter(vec![
        ("value", Value::from(100)),
        ("min", Value::from(0)),
        ("max", Value::from(100)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "true");
}

#[test]
fn test_in_range_floats() {
    let result = InRange::call(Kwargs::from_iter(vec![
        ("value", Value::from(75.5)),
        ("min", Value::from(0.0)),
        ("max", Value::from(80.0)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "true");
}

#[test]
fn test_in_range_negative_range() {
    let result = InRange::call(Kwargs::from_iter(vec![
        ("value", Value::from(-5)),
        ("min", Value::from(-10)),
        ("max", Value::from(0)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "true");
}

#[test]
fn test_in_range_error_non_numeric_value() {
    let result = InRange::call(Kwargs::from_iter(vec![
        ("value", Value::from("test")),
        ("min", Value::from(0)),
        ("max", Value::from(100)),
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
fn test_in_range_error_non_numeric_min() {
    let result = InRange::call(Kwargs::from_iter(vec![
        ("value", Value::from(50)),
        ("min", Value::from("test")),
        ("max", Value::from(100)),
    ]));

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires numeric min")
    );
}

#[test]
fn test_in_range_error_non_numeric_max() {
    let result = InRange::call(Kwargs::from_iter(vec![
        ("value", Value::from(50)),
        ("min", Value::from(0)),
        ("max", Value::from("test")),
    ]));

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires numeric max")
    );
}

#[test]
fn test_in_range_missing_params() {
    let result = InRange::call(Kwargs::from_iter(vec![
        ("value", Value::from(50)),
        ("min", Value::from(0)),
    ]));

    assert!(result.is_err());
}
