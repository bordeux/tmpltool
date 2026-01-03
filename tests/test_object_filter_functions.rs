//! Tests for object filter-functions.
//!
//! Tests both function and filter syntax for:
//! - object_keys, object_values, object_flatten

use minijinja::Value;
use minijinja::value::Kwargs;
use std::collections::BTreeMap;
use tmpltool::filter_functions::FilterFunction;
use tmpltool::filter_functions::object::{ObjectFlatten, ObjectKeys, ObjectValues};

/// Helper to create empty kwargs
fn empty_kwargs() -> Kwargs {
    Kwargs::from_iter(Vec::<(&str, Value)>::new())
}

/// Helper to create object value
fn make_object(pairs: Vec<(&str, Value)>) -> Value {
    let map: BTreeMap<String, Value> = pairs.into_iter().map(|(k, v)| (k.to_string(), v)).collect();
    Value::from_object(map)
}

// ============================================
// ObjectKeys tests
// ============================================

#[test]
fn test_object_keys_filter_syntax() {
    let obj = make_object(vec![
        ("host", Value::from("localhost")),
        ("port", Value::from(8080)),
    ]);
    let result = ObjectKeys::call_as_filter(&obj, empty_kwargs()).unwrap();
    let json: Vec<String> = serde_json::from_value(serde_json::to_value(&result).unwrap()).unwrap();
    assert!(json.contains(&"host".to_string()));
    assert!(json.contains(&"port".to_string()));
    assert_eq!(json.len(), 2);
}

#[test]
fn test_object_keys_function_syntax() {
    let obj = make_object(vec![
        ("a", Value::from(1)),
        ("b", Value::from(2)),
        ("c", Value::from(3)),
    ]);
    let kwargs = Kwargs::from_iter(vec![("object", obj)]);
    let result = ObjectKeys::call_as_function(kwargs).unwrap();
    let json: Vec<String> = serde_json::from_value(serde_json::to_value(&result).unwrap()).unwrap();
    assert_eq!(json.len(), 3);
}

#[test]
fn test_object_keys_empty() {
    let obj = make_object(vec![]);
    let result = ObjectKeys::call_as_filter(&obj, empty_kwargs()).unwrap();
    let json: Vec<String> = serde_json::from_value(serde_json::to_value(&result).unwrap()).unwrap();
    assert!(json.is_empty());
}

#[test]
fn test_object_keys_error_not_object() {
    let result = ObjectKeys::call_as_filter(&Value::from("not an object"), empty_kwargs());
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires an object")
    );
}

#[test]
fn test_object_keys_error_array() {
    let array = Value::from_iter(vec![Value::from(1), Value::from(2)]);
    let result = ObjectKeys::call_as_filter(&array, empty_kwargs());
    assert!(result.is_err());
}

// ============================================
// ObjectValues tests
// ============================================

#[test]
fn test_object_values_filter_syntax() {
    let obj = make_object(vec![
        ("a", Value::from(1)),
        ("b", Value::from(2)),
        ("c", Value::from(3)),
    ]);
    let result = ObjectValues::call_as_filter(&obj, empty_kwargs()).unwrap();
    let json: Vec<i64> = serde_json::from_value(serde_json::to_value(&result).unwrap()).unwrap();
    assert!(json.contains(&1));
    assert!(json.contains(&2));
    assert!(json.contains(&3));
    assert_eq!(json.len(), 3);
}

#[test]
fn test_object_values_function_syntax() {
    let obj = make_object(vec![
        ("name", Value::from("test")),
        ("enabled", Value::from(true)),
    ]);
    let kwargs = Kwargs::from_iter(vec![("object", obj)]);
    let result = ObjectValues::call_as_function(kwargs).unwrap();
    let json: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert!(json.is_array());
}

#[test]
fn test_object_values_empty() {
    let obj = make_object(vec![]);
    let result = ObjectValues::call_as_filter(&obj, empty_kwargs()).unwrap();
    let json: Vec<serde_json::Value> =
        serde_json::from_value(serde_json::to_value(&result).unwrap()).unwrap();
    assert!(json.is_empty());
}

#[test]
fn test_object_values_error_not_object() {
    let result = ObjectValues::call_as_filter(&Value::from(123), empty_kwargs());
    assert!(result.is_err());
}

// ============================================
// ObjectFlatten tests
// ============================================

#[test]
fn test_object_flatten_filter_syntax() {
    // Create nested object
    let inner = make_object(vec![
        ("host", Value::from("localhost")),
        ("port", Value::from(8080)),
    ]);
    let outer = make_object(vec![("server", inner)]);

    let result = ObjectFlatten::call_as_filter(&outer, empty_kwargs()).unwrap();
    let json: serde_json::Value = serde_json::to_value(&result).unwrap();

    assert_eq!(json.get("server.host").unwrap(), "localhost");
    assert_eq!(json.get("server.port").unwrap(), 8080);
}

#[test]
fn test_object_flatten_function_syntax() {
    let inner = make_object(vec![("name", Value::from("mydb"))]);
    let outer = make_object(vec![("database", inner)]);

    let kwargs = Kwargs::from_iter(vec![("object", outer)]);
    let result = ObjectFlatten::call_as_function(kwargs).unwrap();
    let json: serde_json::Value = serde_json::to_value(&result).unwrap();

    assert_eq!(json.get("database.name").unwrap(), "mydb");
}

#[test]
fn test_object_flatten_custom_delimiter() {
    let inner = make_object(vec![("key", Value::from("value"))]);
    let outer = make_object(vec![("nested", inner)]);

    let kwargs = Kwargs::from_iter(vec![("delimiter", Value::from("_"))]);
    let result = ObjectFlatten::call_as_filter(&outer, kwargs).unwrap();
    let json: serde_json::Value = serde_json::to_value(&result).unwrap();

    assert_eq!(json.get("nested_key").unwrap(), "value");
}

#[test]
fn test_object_flatten_deep_nesting() {
    let level3 = make_object(vec![("value", Value::from(42))]);
    let level2 = make_object(vec![("level3", level3)]);
    let level1 = make_object(vec![("level2", level2)]);

    let result = ObjectFlatten::call_as_filter(&level1, empty_kwargs()).unwrap();
    let json: serde_json::Value = serde_json::to_value(&result).unwrap();

    assert_eq!(json.get("level2.level3.value").unwrap(), 42);
}

#[test]
fn test_object_flatten_flat_object() {
    let obj = make_object(vec![("a", Value::from(1)), ("b", Value::from(2))]);

    let result = ObjectFlatten::call_as_filter(&obj, empty_kwargs()).unwrap();
    let json: serde_json::Value = serde_json::to_value(&result).unwrap();

    assert_eq!(json.get("a").unwrap(), 1);
    assert_eq!(json.get("b").unwrap(), 2);
}

#[test]
fn test_object_flatten_error_not_object() {
    let result = ObjectFlatten::call_as_filter(&Value::from("not an object"), empty_kwargs());
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires an object")
    );
}
