use minijinja::Value;
use minijinja::value::Kwargs;
use tmpltool::functions::statistics;

// ============================================================================
// Array Sum Tests
// ============================================================================

#[test]
fn test_array_sum_integers() {
    let result = statistics::array_sum_fn(Kwargs::from_iter(vec![(
        "array",
        Value::from(vec![1, 2, 3, 4, 5]),
    )]))
    .unwrap();

    assert_eq!(result, Value::from(15));
}

#[test]
fn test_array_sum_floats() {
    let result = statistics::array_sum_fn(Kwargs::from_iter(vec![(
        "array",
        Value::from(vec![1.5, 2.5, 3.0]),
    )]))
    .unwrap();

    assert_eq!(result, Value::from(7.0));
}

#[test]
fn test_array_sum_mixed() {
    let result = statistics::array_sum_fn(Kwargs::from_iter(vec![(
        "array",
        Value::from(vec![10, 20, 30]),
    )]))
    .unwrap();

    assert_eq!(result, Value::from(60));
}

#[test]
fn test_array_sum_single_element() {
    let result =
        statistics::array_sum_fn(Kwargs::from_iter(vec![("array", Value::from(vec![42]))]))
            .unwrap();

    assert_eq!(result, Value::from(42));
}

#[test]
fn test_array_sum_empty_array() {
    let empty: Vec<i32> = vec![];
    let result =
        statistics::array_sum_fn(Kwargs::from_iter(vec![("array", Value::from(empty))])).unwrap();

    assert_eq!(result, Value::from(0));
}

#[test]
fn test_array_sum_negative_numbers() {
    let result = statistics::array_sum_fn(Kwargs::from_iter(vec![(
        "array",
        Value::from(vec![-5, -10, 15]),
    )]))
    .unwrap();

    assert_eq!(result, Value::from(0));
}

#[test]
fn test_array_sum_error_not_array() {
    let result = statistics::array_sum_fn(Kwargs::from_iter(vec![("array", Value::from(42))]));

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires an array")
    );
}

#[test]
fn test_array_sum_error_non_numeric() {
    let result = statistics::array_sum_fn(Kwargs::from_iter(vec![(
        "array",
        Value::from(vec!["a", "b", "c"]),
    )]));

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires numeric values")
    );
}

#[test]
fn test_array_sum_missing_array() {
    let result = statistics::array_sum_fn(Kwargs::from_iter(vec![("dummy", Value::from(0))]));

    assert!(result.is_err());
}

// ============================================================================
// Array Average Tests
// ============================================================================

#[test]
fn test_array_avg_integers() {
    let result = statistics::array_avg_fn(Kwargs::from_iter(vec![(
        "array",
        Value::from(vec![10, 20, 30, 40]),
    )]))
    .unwrap();

    assert_eq!(result, Value::from(25.0));
}

#[test]
fn test_array_avg_floats() {
    let result = statistics::array_avg_fn(Kwargs::from_iter(vec![(
        "array",
        Value::from(vec![1.5, 2.5, 3.0]),
    )]))
    .unwrap();

    // Convert to serde_json to extract f64
    let json_val: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert!((json_val.as_f64().unwrap() - 2.333333).abs() < 0.001);
}

#[test]
fn test_array_avg_single_element() {
    let result =
        statistics::array_avg_fn(Kwargs::from_iter(vec![("array", Value::from(vec![42]))]))
            .unwrap();

    assert_eq!(result, Value::from(42.0));
}

#[test]
fn test_array_avg_empty_array() {
    let empty: Vec<i32> = vec![];
    let result =
        statistics::array_avg_fn(Kwargs::from_iter(vec![("array", Value::from(empty))])).unwrap();

    assert_eq!(result, Value::from(0));
}

#[test]
fn test_array_avg_negative_numbers() {
    let result = statistics::array_avg_fn(Kwargs::from_iter(vec![(
        "array",
        Value::from(vec![-10, 10]),
    )]))
    .unwrap();

    assert_eq!(result, Value::from(0.0));
}

#[test]
fn test_array_avg_error_not_array() {
    let result = statistics::array_avg_fn(Kwargs::from_iter(vec![("array", Value::from("test"))]));

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires an array")
    );
}

#[test]
fn test_array_avg_error_non_numeric() {
    let result = statistics::array_avg_fn(Kwargs::from_iter(vec![(
        "array",
        Value::from(vec!["a", "b"]),
    )]));

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires numeric values")
    );
}

#[test]
fn test_array_avg_missing_array() {
    let result = statistics::array_avg_fn(Kwargs::from_iter(vec![("dummy", Value::from(0))]));

    assert!(result.is_err());
}

// ============================================================================
// Array Median Tests
// ============================================================================

#[test]
fn test_array_median_odd_length() {
    let result = statistics::array_median_fn(Kwargs::from_iter(vec![(
        "array",
        Value::from(vec![1, 3, 5, 7, 9]),
    )]))
    .unwrap();

    assert_eq!(result, Value::from(5));
}

#[test]
fn test_array_median_even_length() {
    let result = statistics::array_median_fn(Kwargs::from_iter(vec![(
        "array",
        Value::from(vec![1, 2, 3, 4]),
    )]))
    .unwrap();

    assert_eq!(result, Value::from(2.5));
}

#[test]
fn test_array_median_unsorted() {
    let result = statistics::array_median_fn(Kwargs::from_iter(vec![(
        "array",
        Value::from(vec![9, 1, 5, 3, 7]),
    )]))
    .unwrap();

    assert_eq!(result, Value::from(5));
}

#[test]
fn test_array_median_single_element() {
    let result =
        statistics::array_median_fn(Kwargs::from_iter(vec![("array", Value::from(vec![42]))]))
            .unwrap();

    assert_eq!(result, Value::from(42));
}

#[test]
fn test_array_median_empty_array() {
    let empty: Vec<i32> = vec![];
    let result =
        statistics::array_median_fn(Kwargs::from_iter(vec![("array", Value::from(empty))]))
            .unwrap();

    assert_eq!(result, Value::from(0));
}

#[test]
fn test_array_median_two_elements() {
    let result = statistics::array_median_fn(Kwargs::from_iter(vec![(
        "array",
        Value::from(vec![10, 20]),
    )]))
    .unwrap();

    assert_eq!(result, Value::from(15.0));
}

#[test]
fn test_array_median_floats() {
    let result = statistics::array_median_fn(Kwargs::from_iter(vec![(
        "array",
        Value::from(vec![1.5, 2.5, 3.5]),
    )]))
    .unwrap();

    assert_eq!(result, Value::from(2.5));
}

#[test]
fn test_array_median_error_not_array() {
    let result = statistics::array_median_fn(Kwargs::from_iter(vec![("array", Value::from(42))]));

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires an array")
    );
}

#[test]
fn test_array_median_error_non_numeric() {
    let result = statistics::array_median_fn(Kwargs::from_iter(vec![(
        "array",
        Value::from(vec!["a", "b"]),
    )]));

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires numeric values")
    );
}

#[test]
fn test_array_median_missing_array() {
    let result = statistics::array_median_fn(Kwargs::from_iter(vec![("dummy", Value::from(0))]));

    assert!(result.is_err());
}

// ============================================================================
// Array Min Tests
// ============================================================================

#[test]
fn test_array_min_integers() {
    let result = statistics::array_min_fn(Kwargs::from_iter(vec![(
        "array",
        Value::from(vec![42, 17, 99, 8, 55]),
    )]))
    .unwrap();

    assert_eq!(result, Value::from(8));
}

#[test]
fn test_array_min_floats() {
    let result = statistics::array_min_fn(Kwargs::from_iter(vec![(
        "array",
        Value::from(vec![10.99, 5.49, 15.99]),
    )]))
    .unwrap();

    assert_eq!(result, Value::from(5.49));
}

#[test]
fn test_array_min_single_element() {
    let result =
        statistics::array_min_fn(Kwargs::from_iter(vec![("array", Value::from(vec![42]))]))
            .unwrap();

    assert_eq!(result, Value::from(42));
}

#[test]
fn test_array_min_negative_numbers() {
    let result = statistics::array_min_fn(Kwargs::from_iter(vec![(
        "array",
        Value::from(vec![-5, -10, 15, 3]),
    )]))
    .unwrap();

    assert_eq!(result, Value::from(-10));
}

#[test]
fn test_array_min_all_same() {
    let result = statistics::array_min_fn(Kwargs::from_iter(vec![(
        "array",
        Value::from(vec![7, 7, 7, 7]),
    )]))
    .unwrap();

    assert_eq!(result, Value::from(7));
}

#[test]
fn test_array_min_empty_array() {
    let empty: Vec<i32> = vec![];
    let result = statistics::array_min_fn(Kwargs::from_iter(vec![("array", Value::from(empty))]));

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("non-empty array"));
}

#[test]
fn test_array_min_error_not_array() {
    let result = statistics::array_min_fn(Kwargs::from_iter(vec![("array", Value::from("test"))]));

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires an array")
    );
}

#[test]
fn test_array_min_error_non_numeric() {
    let result = statistics::array_min_fn(Kwargs::from_iter(vec![(
        "array",
        Value::from(vec!["a", "b"]),
    )]));

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires numeric values")
    );
}

#[test]
fn test_array_min_missing_array() {
    let result = statistics::array_min_fn(Kwargs::from_iter(vec![("dummy", Value::from(0))]));

    assert!(result.is_err());
}

// ============================================================================
// Array Max Tests
// ============================================================================

#[test]
fn test_array_max_integers() {
    let result = statistics::array_max_fn(Kwargs::from_iter(vec![(
        "array",
        Value::from(vec![42, 17, 99, 8, 55]),
    )]))
    .unwrap();

    assert_eq!(result, Value::from(99));
}

#[test]
fn test_array_max_floats() {
    let result = statistics::array_max_fn(Kwargs::from_iter(vec![(
        "array",
        Value::from(vec![10.99, 5.49, 15.99]),
    )]))
    .unwrap();

    assert_eq!(result, Value::from(15.99));
}

#[test]
fn test_array_max_single_element() {
    let result =
        statistics::array_max_fn(Kwargs::from_iter(vec![("array", Value::from(vec![42]))]))
            .unwrap();

    assert_eq!(result, Value::from(42));
}

#[test]
fn test_array_max_negative_numbers() {
    let result = statistics::array_max_fn(Kwargs::from_iter(vec![(
        "array",
        Value::from(vec![-5, -10, -15, -3]),
    )]))
    .unwrap();

    assert_eq!(result, Value::from(-3));
}

#[test]
fn test_array_max_all_same() {
    let result = statistics::array_max_fn(Kwargs::from_iter(vec![(
        "array",
        Value::from(vec![7, 7, 7, 7]),
    )]))
    .unwrap();

    assert_eq!(result, Value::from(7));
}

#[test]
fn test_array_max_empty_array() {
    let empty: Vec<i32> = vec![];
    let result = statistics::array_max_fn(Kwargs::from_iter(vec![("array", Value::from(empty))]));

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("non-empty array"));
}

#[test]
fn test_array_max_error_not_array() {
    let result = statistics::array_max_fn(Kwargs::from_iter(vec![("array", Value::from(123))]));

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires an array")
    );
}

#[test]
fn test_array_max_error_non_numeric() {
    let result = statistics::array_max_fn(Kwargs::from_iter(vec![(
        "array",
        Value::from(vec!["x", "y"]),
    )]));

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires numeric values")
    );
}

#[test]
fn test_array_max_missing_array() {
    let result = statistics::array_max_fn(Kwargs::from_iter(vec![("dummy", Value::from(0))]));

    assert!(result.is_err());
}
