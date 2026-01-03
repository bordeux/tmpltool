//! Tests for serialization filter-functions.
//!
//! Tests both function and filter syntax for:
//! - to_json, to_yaml, to_toml
//! - parse_json, parse_yaml, parse_toml

use minijinja::Value;
use minijinja::value::Kwargs;
use std::collections::BTreeMap;
use tmpltool::filter_functions::FilterFunction;
use tmpltool::filter_functions::serialization::{
    ParseJson, ParseToml, ParseYaml, ToJson, ToToml, ToYaml,
};

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
// ToJson tests
// ============================================

#[test]
fn test_to_json_filter_syntax() {
    let obj = make_object(vec![("key", Value::from("value"))]);
    let result = ToJson::call_as_filter(&obj, empty_kwargs()).unwrap();
    let json_str = result.as_str().unwrap();
    assert!(json_str.contains("\"key\""));
    assert!(json_str.contains("\"value\""));
}

#[test]
fn test_to_json_function_syntax() {
    let obj = make_object(vec![("key", Value::from("value"))]);
    let kwargs = Kwargs::from_iter(vec![("object", obj)]);
    let result = ToJson::call_as_function(kwargs).unwrap();
    let json_str = result.as_str().unwrap();
    assert!(json_str.contains("\"key\""));
}

#[test]
fn test_to_json_pretty() {
    let obj = make_object(vec![("key", Value::from("value"))]);
    let kwargs = Kwargs::from_iter(vec![("pretty", Value::from(true))]);
    let result = ToJson::call_as_filter(&obj, kwargs).unwrap();
    let json_str = result.as_str().unwrap();
    // Pretty JSON should have newlines
    assert!(json_str.contains('\n'));
}

#[test]
fn test_to_json_array() {
    let array = Value::from_iter(vec![Value::from(1), Value::from(2), Value::from(3)]);
    let result = ToJson::call_as_filter(&array, empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "[1,2,3]");
}

#[test]
fn test_to_json_nested() {
    let inner = make_object(vec![("nested", Value::from(true))]);
    let outer = make_object(vec![("inner", inner)]);
    let result = ToJson::call_as_filter(&outer, empty_kwargs()).unwrap();
    let json_str = result.as_str().unwrap();
    assert!(json_str.contains("\"inner\""));
    assert!(json_str.contains("\"nested\""));
}

// ============================================
// ToYaml tests
// ============================================

#[test]
fn test_to_yaml_filter_syntax() {
    let obj = make_object(vec![("key", Value::from("value"))]);
    let result = ToYaml::call_as_filter(&obj, empty_kwargs()).unwrap();
    let yaml_str = result.as_str().unwrap();
    assert!(yaml_str.contains("key:"));
    assert!(yaml_str.contains("value"));
}

#[test]
fn test_to_yaml_function_syntax() {
    let obj = make_object(vec![("key", Value::from("value"))]);
    let kwargs = Kwargs::from_iter(vec![("object", obj)]);
    let result = ToYaml::call_as_function(kwargs).unwrap();
    let yaml_str = result.as_str().unwrap();
    assert!(yaml_str.contains("key:"));
}

#[test]
fn test_to_yaml_nested() {
    let inner = make_object(vec![("nested", Value::from(true))]);
    let outer = make_object(vec![("inner", inner)]);
    let result = ToYaml::call_as_filter(&outer, empty_kwargs()).unwrap();
    let yaml_str = result.as_str().unwrap();
    assert!(yaml_str.contains("inner:"));
}

// ============================================
// ToToml tests
// ============================================

#[test]
fn test_to_toml_filter_syntax() {
    let obj = make_object(vec![("key", Value::from("value"))]);
    let result = ToToml::call_as_filter(&obj, empty_kwargs()).unwrap();
    let toml_str = result.as_str().unwrap();
    assert!(toml_str.contains("key ="));
    assert!(toml_str.contains("\"value\""));
}

#[test]
fn test_to_toml_function_syntax() {
    let obj = make_object(vec![("key", Value::from("value"))]);
    let kwargs = Kwargs::from_iter(vec![("object", obj)]);
    let result = ToToml::call_as_function(kwargs).unwrap();
    let toml_str = result.as_str().unwrap();
    assert!(toml_str.contains("key ="));
}

#[test]
fn test_to_toml_integer() {
    let obj = make_object(vec![("count", Value::from(42))]);
    let result = ToToml::call_as_filter(&obj, empty_kwargs()).unwrap();
    let toml_str = result.as_str().unwrap();
    assert!(toml_str.contains("count = 42"));
}

// ============================================
// ParseJson tests
// ============================================

#[test]
fn test_parse_json_filter_syntax() {
    let json_str = r#"{"key": "value"}"#;
    let result = ParseJson::call_as_filter(&Value::from(json_str), empty_kwargs()).unwrap();
    assert_eq!(result.get_attr("key").unwrap().as_str().unwrap(), "value");
}

#[test]
fn test_parse_json_function_syntax() {
    let json_str = r#"{"key": "value"}"#;
    let kwargs = Kwargs::from_iter(vec![("string", Value::from(json_str))]);
    let result = ParseJson::call_as_function(kwargs).unwrap();
    assert_eq!(result.get_attr("key").unwrap().as_str().unwrap(), "value");
}

#[test]
fn test_parse_json_array() {
    let json_str = "[1, 2, 3]";
    let result = ParseJson::call_as_filter(&Value::from(json_str), empty_kwargs()).unwrap();
    let json: Vec<i64> = serde_json::from_value(serde_json::to_value(&result).unwrap()).unwrap();
    assert_eq!(json, vec![1, 2, 3]);
}

#[test]
fn test_parse_json_nested() {
    let json_str = r#"{"outer": {"inner": true}}"#;
    let result = ParseJson::call_as_filter(&Value::from(json_str), empty_kwargs()).unwrap();
    let outer = result.get_attr("outer").unwrap();
    assert!(outer.get_attr("inner").unwrap().is_true());
}

#[test]
fn test_parse_json_invalid() {
    let result = ParseJson::call_as_filter(&Value::from("not json"), empty_kwargs());
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Failed to parse"));
}

#[test]
fn test_parse_json_error_not_string() {
    let result = ParseJson::call_as_filter(&Value::from(123), empty_kwargs());
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires a string")
    );
}

// ============================================
// ParseYaml tests
// ============================================

#[test]
fn test_parse_yaml_filter_syntax() {
    let yaml_str = "key: value";
    let result = ParseYaml::call_as_filter(&Value::from(yaml_str), empty_kwargs()).unwrap();
    assert_eq!(result.get_attr("key").unwrap().as_str().unwrap(), "value");
}

#[test]
fn test_parse_yaml_function_syntax() {
    let yaml_str = "key: value";
    let kwargs = Kwargs::from_iter(vec![("string", Value::from(yaml_str))]);
    let result = ParseYaml::call_as_function(kwargs).unwrap();
    assert_eq!(result.get_attr("key").unwrap().as_str().unwrap(), "value");
}

#[test]
fn test_parse_yaml_nested() {
    let yaml_str = "outer:\n  inner: true";
    let result = ParseYaml::call_as_filter(&Value::from(yaml_str), empty_kwargs()).unwrap();
    let outer = result.get_attr("outer").unwrap();
    assert!(outer.get_attr("inner").unwrap().is_true());
}

#[test]
fn test_parse_yaml_array() {
    let yaml_str = "- 1\n- 2\n- 3";
    let result = ParseYaml::call_as_filter(&Value::from(yaml_str), empty_kwargs()).unwrap();
    let json: Vec<i64> = serde_json::from_value(serde_json::to_value(&result).unwrap()).unwrap();
    assert_eq!(json, vec![1, 2, 3]);
}

#[test]
fn test_parse_yaml_error_not_string() {
    let result = ParseYaml::call_as_filter(&Value::from(123), empty_kwargs());
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires a string")
    );
}

// ============================================
// ParseToml tests
// ============================================

#[test]
fn test_parse_toml_filter_syntax() {
    let toml_str = r#"key = "value""#;
    let result = ParseToml::call_as_filter(&Value::from(toml_str), empty_kwargs()).unwrap();
    assert_eq!(result.get_attr("key").unwrap().as_str().unwrap(), "value");
}

#[test]
fn test_parse_toml_function_syntax() {
    let toml_str = r#"key = "value""#;
    let kwargs = Kwargs::from_iter(vec![("string", Value::from(toml_str))]);
    let result = ParseToml::call_as_function(kwargs).unwrap();
    assert_eq!(result.get_attr("key").unwrap().as_str().unwrap(), "value");
}

#[test]
fn test_parse_toml_integer() {
    let toml_str = "count = 42";
    let result = ParseToml::call_as_filter(&Value::from(toml_str), empty_kwargs()).unwrap();
    assert_eq!(result.get_attr("count").unwrap().as_i64().unwrap(), 42);
}

#[test]
fn test_parse_toml_section() {
    let toml_str = "[section]\nkey = \"value\"";
    let result = ParseToml::call_as_filter(&Value::from(toml_str), empty_kwargs()).unwrap();
    let section = result.get_attr("section").unwrap();
    assert_eq!(section.get_attr("key").unwrap().as_str().unwrap(), "value");
}

#[test]
fn test_parse_toml_invalid() {
    let result = ParseToml::call_as_filter(&Value::from("not = = valid"), empty_kwargs());
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Failed to parse"));
}

#[test]
fn test_parse_toml_error_not_string() {
    let result = ParseToml::call_as_filter(&Value::from(123), empty_kwargs());
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires a string")
    );
}
