use minijinja::Value;
use minijinja::value::Kwargs;
use tmpltool::functions::debug;

#[test]
fn test_debug_returns_value() {
    let result = debug::debug_fn(Kwargs::from_iter(vec![("value", Value::from("test"))])).unwrap();
    assert_eq!(result.as_str().unwrap(), "test");
}

#[test]
fn test_debug_with_number() {
    let result = debug::debug_fn(Kwargs::from_iter(vec![("value", Value::from(42))])).unwrap();
    assert_eq!(result.as_i64(), Some(42));
}

#[test]
fn test_debug_with_array() {
    let arr = vec![1, 2, 3];
    let result =
        debug::debug_fn(Kwargs::from_iter(vec![("value", Value::from(arr.clone()))])).unwrap();

    let result_vec: Vec<i64> = result
        .try_iter()
        .unwrap()
        .map(|v| v.as_i64().unwrap())
        .collect();
    assert_eq!(result_vec, arr);
}

#[test]
fn test_debug_with_object() {
    let obj = serde_json::json!({"key": "value"});
    let result = debug::debug_fn(Kwargs::from_iter(vec![(
        "value",
        Value::from_serialize(&obj),
    )]))
    .unwrap();

    assert!(result.is_true());
}

#[test]
fn test_type_of_string() {
    let result =
        debug::type_of_fn(Kwargs::from_iter(vec![("value", Value::from("hello"))])).unwrap();
    assert_eq!(result.as_str().unwrap(), "string");
}

#[test]
fn test_type_of_number() {
    let result = debug::type_of_fn(Kwargs::from_iter(vec![("value", Value::from(123))])).unwrap();
    assert_eq!(result.as_str().unwrap(), "number");
}

#[test]
fn test_type_of_bool() {
    let result = debug::type_of_fn(Kwargs::from_iter(vec![("value", Value::from(true))])).unwrap();
    assert_eq!(result.as_str().unwrap(), "bool");
}

#[test]
fn test_type_of_array() {
    let arr = vec![1, 2, 3];
    let result = debug::type_of_fn(Kwargs::from_iter(vec![("value", Value::from(arr))])).unwrap();
    assert_eq!(result.as_str().unwrap(), "array");
}

#[test]
fn test_type_of_object() {
    let obj = serde_json::json!({"key": "value"});
    let result = debug::type_of_fn(Kwargs::from_iter(vec![(
        "value",
        Value::from_serialize(&obj),
    )]))
    .unwrap();
    assert_eq!(result.as_str().unwrap(), "object");
}

#[test]
fn test_type_of_undefined() {
    let result = debug::type_of_fn(Kwargs::from_iter(vec![("value", Value::UNDEFINED)])).unwrap();
    assert_eq!(result.as_str().unwrap(), "undefined");
}

#[test]
fn test_inspect_returns_value() {
    let result =
        debug::inspect_fn(Kwargs::from_iter(vec![("value", Value::from("test"))])).unwrap();
    assert_eq!(result.as_str().unwrap(), "test");
}

#[test]
fn test_inspect_with_complex_object() {
    let obj = serde_json::json!({"name": "test", "count": 42, "items": [1, 2, 3]});
    let result = debug::inspect_fn(Kwargs::from_iter(vec![(
        "value",
        Value::from_serialize(&obj),
    )]))
    .unwrap();

    // Just verify it returns the value
    assert!(result.is_true());
}

#[test]
fn test_assert_passes() {
    let result = debug::assert_fn(Kwargs::from_iter(vec![
        ("condition", Value::from(true)),
        ("message", Value::from("Should not fail")),
    ]))
    .unwrap();

    assert!(result.is_true());
}

#[test]
fn test_assert_fails() {
    let result = debug::assert_fn(Kwargs::from_iter(vec![
        ("condition", Value::from(false)),
        ("message", Value::from("Custom error message")),
    ]));

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Custom error message")
    );
}

#[test]
fn test_assert_fails_with_default_message() {
    let result = debug::assert_fn(Kwargs::from_iter(vec![("condition", Value::from(false))]));

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Assertion failed"));
}

#[test]
fn test_warn_returns_empty_string() {
    let result = debug::warn_fn(Kwargs::from_iter(vec![(
        "message",
        Value::from("Test warning"),
    )]))
    .unwrap();

    assert_eq!(result.as_str().unwrap(), "");
}

#[test]
fn test_warn_with_long_message() {
    let long_message = "This is a very long warning message that contains important information about something that might be wrong or needs attention";
    let result = debug::warn_fn(Kwargs::from_iter(vec![(
        "message",
        Value::from(long_message),
    )]))
    .unwrap();

    assert_eq!(result.as_str().unwrap(), "");
}

#[test]
fn test_abort_always_fails() {
    let result = debug::abort_fn(Kwargs::from_iter(vec![("message", Value::from("Aborted"))]));

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Aborted"));
}

#[test]
fn test_abort_with_detailed_message() {
    let message = "Critical error: configuration file not found at /etc/app/config.yaml";
    let result = debug::abort_fn(Kwargs::from_iter(vec![("message", Value::from(message))]));

    assert!(result.is_err());
    let error_message = result.unwrap_err().to_string();
    assert!(error_message.contains("Critical error"));
    assert!(error_message.contains("config.yaml"));
}

#[test]
fn test_assert_missing_condition() {
    let result = debug::assert_fn(Kwargs::from_iter(Vec::<(&str, Value)>::new()));
    assert!(result.is_err());
}

#[test]
fn test_type_of_missing_value() {
    let result = debug::type_of_fn(Kwargs::from_iter(Vec::<(&str, Value)>::new()));
    assert!(result.is_err());
}

#[test]
fn test_debug_missing_value() {
    let result = debug::debug_fn(Kwargs::from_iter(Vec::<(&str, Value)>::new()));
    assert!(result.is_err());
}

#[test]
fn test_warn_missing_message() {
    let result = debug::warn_fn(Kwargs::from_iter(Vec::<(&str, Value)>::new()));
    assert!(result.is_err());
}

#[test]
fn test_abort_missing_message() {
    let result = debug::abort_fn(Kwargs::from_iter(Vec::<(&str, Value)>::new()));
    assert!(result.is_err());
}
