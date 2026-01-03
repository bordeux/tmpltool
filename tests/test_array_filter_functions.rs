//! Tests for array filter-functions.
//!
//! Tests both function and filter syntax for:
//! - array_sum, array_avg, array_median, array_min, array_max
//! - array_unique, array_flatten

use minijinja::Value;
use minijinja::value::Kwargs;
use tmpltool::filter_functions::FilterFunction;
use tmpltool::filter_functions::array::{
    ArrayAvg, ArrayFlatten, ArrayMax, ArrayMedian, ArrayMin, ArraySum, ArrayUnique,
};

/// Helper to create empty kwargs
fn empty_kwargs() -> Kwargs {
    Kwargs::from_iter(Vec::<(&str, Value)>::new())
}

/// Helper to create array value
fn make_array(items: Vec<i64>) -> Value {
    Value::from_iter(items.into_iter().map(Value::from))
}

// ============================================
// ArraySum tests
// ============================================

#[test]
fn test_array_sum_filter_syntax() {
    let array = make_array(vec![1, 2, 3, 4, 5]);
    let result = ArraySum::call_as_filter(&array, empty_kwargs()).unwrap();
    assert_eq!(result.as_i64().unwrap(), 15);
}

#[test]
fn test_array_sum_function_syntax() {
    let array = make_array(vec![1, 2, 3, 4, 5]);
    let kwargs = Kwargs::from_iter(vec![("array", array)]);
    let result = ArraySum::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_i64().unwrap(), 15);
}

#[test]
fn test_array_sum_empty() {
    let array = make_array(vec![]);
    let result = ArraySum::call_as_filter(&array, empty_kwargs()).unwrap();
    assert_eq!(result.as_i64().unwrap(), 0);
}

#[test]
fn test_array_sum_error_not_array() {
    let result = ArraySum::call_as_filter(&Value::from("not an array"), empty_kwargs());
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires an array")
    );
}

// ============================================
// ArrayAvg tests
// ============================================

#[test]
fn test_array_avg_filter_syntax() {
    let array = make_array(vec![2, 4, 6, 8, 10]);
    let result = ArrayAvg::call_as_filter(&array, empty_kwargs()).unwrap();
    assert_eq!(result.as_i64().unwrap(), 6);
}

#[test]
fn test_array_avg_function_syntax() {
    let array = make_array(vec![2, 4, 6, 8, 10]);
    let kwargs = Kwargs::from_iter(vec![("array", array)]);
    let result = ArrayAvg::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_i64().unwrap(), 6);
}

#[test]
fn test_array_avg_empty() {
    let array = make_array(vec![]);
    let result = ArrayAvg::call_as_filter(&array, empty_kwargs()).unwrap();
    assert_eq!(result.as_i64().unwrap(), 0);
}

#[test]
fn test_array_avg_single() {
    let array = make_array(vec![42]);
    let result = ArrayAvg::call_as_filter(&array, empty_kwargs()).unwrap();
    assert_eq!(result.as_i64().unwrap(), 42);
}

#[test]
fn test_array_avg_error_not_array() {
    let result = ArrayAvg::call_as_filter(&Value::from(123), empty_kwargs());
    assert!(result.is_err());
}

// ============================================
// ArrayMedian tests
// ============================================

#[test]
fn test_array_median_filter_odd() {
    let array = make_array(vec![1, 3, 5, 7, 9]);
    let result = ArrayMedian::call_as_filter(&array, empty_kwargs()).unwrap();
    assert_eq!(result.as_i64().unwrap(), 5);
}

#[test]
fn test_array_median_filter_even() {
    let array = make_array(vec![1, 2, 3, 4]);
    let result = ArrayMedian::call_as_filter(&array, empty_kwargs()).unwrap();
    let val: f64 = serde_json::from_value(serde_json::to_value(&result).unwrap()).unwrap();
    assert!((val - 2.5).abs() < 0.001);
}

#[test]
fn test_array_median_function_syntax() {
    let array = make_array(vec![1, 3, 5]);
    let kwargs = Kwargs::from_iter(vec![("array", array)]);
    let result = ArrayMedian::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_i64().unwrap(), 3);
}

#[test]
fn test_array_median_unsorted() {
    let array = make_array(vec![5, 1, 3]);
    let result = ArrayMedian::call_as_filter(&array, empty_kwargs()).unwrap();
    assert_eq!(result.as_i64().unwrap(), 3);
}

#[test]
fn test_array_median_empty() {
    let array = make_array(vec![]);
    let result = ArrayMedian::call_as_filter(&array, empty_kwargs()).unwrap();
    assert_eq!(result.as_i64().unwrap(), 0);
}

#[test]
fn test_array_median_error_not_array() {
    let result = ArrayMedian::call_as_filter(&Value::from(123), empty_kwargs());
    assert!(result.is_err());
}

// ============================================
// ArrayMin tests
// ============================================

#[test]
fn test_array_min_filter_syntax() {
    let array = make_array(vec![5, 2, 8, 1, 9]);
    let result = ArrayMin::call_as_filter(&array, empty_kwargs()).unwrap();
    assert_eq!(result.as_i64().unwrap(), 1);
}

#[test]
fn test_array_min_function_syntax() {
    let array = make_array(vec![5, 2, 8, 1, 9]);
    let kwargs = Kwargs::from_iter(vec![("array", array)]);
    let result = ArrayMin::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_i64().unwrap(), 1);
}

#[test]
fn test_array_min_negative() {
    let array = make_array(vec![-5, 2, -8, 1]);
    let result = ArrayMin::call_as_filter(&array, empty_kwargs()).unwrap();
    assert_eq!(result.as_i64().unwrap(), -8);
}

#[test]
fn test_array_min_empty_error() {
    let array = make_array(vec![]);
    let result = ArrayMin::call_as_filter(&array, empty_kwargs());
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("non-empty"));
}

#[test]
fn test_array_min_error_not_array() {
    let result = ArrayMin::call_as_filter(&Value::from(123), empty_kwargs());
    assert!(result.is_err());
}

// ============================================
// ArrayMax tests
// ============================================

#[test]
fn test_array_max_filter_syntax() {
    let array = make_array(vec![5, 2, 8, 1, 9]);
    let result = ArrayMax::call_as_filter(&array, empty_kwargs()).unwrap();
    assert_eq!(result.as_i64().unwrap(), 9);
}

#[test]
fn test_array_max_function_syntax() {
    let array = make_array(vec![5, 2, 8, 1, 9]);
    let kwargs = Kwargs::from_iter(vec![("array", array)]);
    let result = ArrayMax::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_i64().unwrap(), 9);
}

#[test]
fn test_array_max_negative() {
    let array = make_array(vec![-5, -2, -8, -1]);
    let result = ArrayMax::call_as_filter(&array, empty_kwargs()).unwrap();
    assert_eq!(result.as_i64().unwrap(), -1);
}

#[test]
fn test_array_max_empty_error() {
    let array = make_array(vec![]);
    let result = ArrayMax::call_as_filter(&array, empty_kwargs());
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("non-empty"));
}

#[test]
fn test_array_max_error_not_array() {
    let result = ArrayMax::call_as_filter(&Value::from(123), empty_kwargs());
    assert!(result.is_err());
}

// ============================================
// ArrayUnique tests
// ============================================

#[test]
fn test_array_unique_filter_syntax() {
    let array = make_array(vec![1, 2, 2, 3, 3, 3]);
    let result = ArrayUnique::call_as_filter(&array, empty_kwargs()).unwrap();
    let json: Vec<i64> = serde_json::from_value(serde_json::to_value(&result).unwrap()).unwrap();
    assert_eq!(json, vec![1, 2, 3]);
}

#[test]
fn test_array_unique_function_syntax() {
    let array = make_array(vec![1, 2, 2, 3]);
    let kwargs = Kwargs::from_iter(vec![("array", array)]);
    let result = ArrayUnique::call_as_function(kwargs).unwrap();
    let json: Vec<i64> = serde_json::from_value(serde_json::to_value(&result).unwrap()).unwrap();
    assert_eq!(json, vec![1, 2, 3]);
}

#[test]
fn test_array_unique_already_unique() {
    let array = make_array(vec![1, 2, 3, 4]);
    let result = ArrayUnique::call_as_filter(&array, empty_kwargs()).unwrap();
    let json: Vec<i64> = serde_json::from_value(serde_json::to_value(&result).unwrap()).unwrap();
    assert_eq!(json, vec![1, 2, 3, 4]);
}

#[test]
fn test_array_unique_empty() {
    let array = make_array(vec![]);
    let result = ArrayUnique::call_as_filter(&array, empty_kwargs()).unwrap();
    let json: Vec<i64> = serde_json::from_value(serde_json::to_value(&result).unwrap()).unwrap();
    assert!(json.is_empty());
}

#[test]
fn test_array_unique_error_not_array() {
    let result = ArrayUnique::call_as_filter(&Value::from(123), empty_kwargs());
    assert!(result.is_err());
}

// ============================================
// ArrayFlatten tests
// ============================================

#[test]
fn test_array_flatten_filter_syntax() {
    // Create nested array [[1, 2], [3, 4]]
    let inner1 = Value::from_iter(vec![Value::from(1), Value::from(2)]);
    let inner2 = Value::from_iter(vec![Value::from(3), Value::from(4)]);
    let array = Value::from_iter(vec![inner1, inner2]);

    let result = ArrayFlatten::call_as_filter(&array, empty_kwargs()).unwrap();
    let json: Vec<i64> = serde_json::from_value(serde_json::to_value(&result).unwrap()).unwrap();
    assert_eq!(json, vec![1, 2, 3, 4]);
}

#[test]
fn test_array_flatten_function_syntax() {
    let inner1 = Value::from_iter(vec![Value::from(1), Value::from(2)]);
    let inner2 = Value::from_iter(vec![Value::from(3), Value::from(4)]);
    let array = Value::from_iter(vec![inner1, inner2]);

    let kwargs = Kwargs::from_iter(vec![("array", array)]);
    let result = ArrayFlatten::call_as_function(kwargs).unwrap();
    let json: Vec<i64> = serde_json::from_value(serde_json::to_value(&result).unwrap()).unwrap();
    assert_eq!(json, vec![1, 2, 3, 4]);
}

#[test]
fn test_array_flatten_mixed() {
    // Create [1, [2, 3], 4]
    let inner = Value::from_iter(vec![Value::from(2), Value::from(3)]);
    let array = Value::from_iter(vec![Value::from(1), inner, Value::from(4)]);

    let result = ArrayFlatten::call_as_filter(&array, empty_kwargs()).unwrap();
    let json: Vec<i64> = serde_json::from_value(serde_json::to_value(&result).unwrap()).unwrap();
    assert_eq!(json, vec![1, 2, 3, 4]);
}

#[test]
fn test_array_flatten_already_flat() {
    let array = make_array(vec![1, 2, 3, 4]);
    let result = ArrayFlatten::call_as_filter(&array, empty_kwargs()).unwrap();
    let json: Vec<i64> = serde_json::from_value(serde_json::to_value(&result).unwrap()).unwrap();
    assert_eq!(json, vec![1, 2, 3, 4]);
}

#[test]
fn test_array_flatten_empty() {
    let array = make_array(vec![]);
    let result = ArrayFlatten::call_as_filter(&array, empty_kwargs()).unwrap();
    let json: Vec<i64> = serde_json::from_value(serde_json::to_value(&result).unwrap()).unwrap();
    assert!(json.is_empty());
}

#[test]
fn test_array_flatten_error_not_array() {
    let result = ArrayFlatten::call_as_filter(&Value::from(123), empty_kwargs());
    assert!(result.is_err());
}
