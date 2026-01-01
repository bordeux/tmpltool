use minijinja::Value;
use minijinja::value::Kwargs;
use tmpltool::functions::object;

#[test]
fn test_object_merge_simple() {
    let obj1 = serde_json::json!({"a": 1, "b": 2});
    let obj2 = serde_json::json!({"c": 3});

    let result = object::object_merge_fn(Kwargs::from_iter(vec![
        ("obj1", Value::from_serialize(&obj1)),
        ("obj2", Value::from_serialize(&obj2)),
    ]))
    .unwrap();

    let json: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json["a"], 1);
    assert_eq!(json["b"], 2);
    assert_eq!(json["c"], 3);
}

#[test]
fn test_object_merge_override() {
    let obj1 = serde_json::json!({"a": 1, "b": 2});
    let obj2 = serde_json::json!({"b": 3, "c": 4});

    let result = object::object_merge_fn(Kwargs::from_iter(vec![
        ("obj1", Value::from_serialize(&obj1)),
        ("obj2", Value::from_serialize(&obj2)),
    ]))
    .unwrap();

    let json: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json["a"], 1);
    assert_eq!(json["b"], 3); // obj2 overrides obj1
    assert_eq!(json["c"], 4);
}

#[test]
fn test_object_merge_nested() {
    let obj1 = serde_json::json!({"a": 1, "b": {"c": 2, "d": 3}});
    let obj2 = serde_json::json!({"b": {"d": 4, "e": 5}, "f": 6});

    let result = object::object_merge_fn(Kwargs::from_iter(vec![
        ("obj1", Value::from_serialize(&obj1)),
        ("obj2", Value::from_serialize(&obj2)),
    ]))
    .unwrap();

    let json: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json["a"], 1);
    assert_eq!(json["b"]["c"], 2); // Preserved from obj1
    assert_eq!(json["b"]["d"], 4); // Overridden by obj2
    assert_eq!(json["b"]["e"], 5); // Added from obj2
    assert_eq!(json["f"], 6);
}

#[test]
fn test_object_merge_deep_nested() {
    let obj1 = serde_json::json!({"a": {"b": {"c": 1}}});
    let obj2 = serde_json::json!({"a": {"b": {"d": 2}}});

    let result = object::object_merge_fn(Kwargs::from_iter(vec![
        ("obj1", Value::from_serialize(&obj1)),
        ("obj2", Value::from_serialize(&obj2)),
    ]))
    .unwrap();

    let json: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json["a"]["b"]["c"], 1);
    assert_eq!(json["a"]["b"]["d"], 2);
}

#[test]
fn test_object_get_simple() {
    let obj = serde_json::json!({"host": "localhost", "port": 8080});

    let result = object::object_get_fn(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("host")),
    ]))
    .unwrap();

    assert_eq!(result.as_str().unwrap(), "localhost");
}

#[test]
fn test_object_get_nested() {
    let obj = serde_json::json!({"server": {"host": "localhost", "port": 8080}});

    let result = object::object_get_fn(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("server.host")),
    ]))
    .unwrap();

    assert_eq!(result.as_str().unwrap(), "localhost");
}

#[test]
fn test_object_get_deep_nested() {
    let obj = serde_json::json!({"a": {"b": {"c": {"d": "value"}}}});

    let result = object::object_get_fn(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("a.b.c.d")),
    ]))
    .unwrap();

    assert_eq!(result.as_str().unwrap(), "value");
}

#[test]
fn test_object_get_array_index() {
    let obj = serde_json::json!({"items": [10, 20, 30]});

    let result = object::object_get_fn(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("items.1")),
    ]))
    .unwrap();

    assert_eq!(result.as_i64(), Some(20));
}

#[test]
fn test_object_get_not_found() {
    let obj = serde_json::json!({"a": 1});

    let result = object::object_get_fn(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("b")),
    ]))
    .unwrap();

    assert!(result.is_undefined());
}

#[test]
fn test_object_get_nested_not_found() {
    let obj = serde_json::json!({"a": {"b": 1}});

    let result = object::object_get_fn(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("a.c")),
    ]))
    .unwrap();

    assert!(result.is_undefined());
}

#[test]
fn test_object_set_simple() {
    let obj = serde_json::json!({"a": 1});

    let result = object::object_set_fn(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("b")),
        ("value", Value::from(2)),
    ]))
    .unwrap();

    let json: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json["a"], 1);
    assert_eq!(json["b"], 2);
}

#[test]
fn test_object_set_nested() {
    let obj = serde_json::json!({"server": {"host": "localhost"}});

    let result = object::object_set_fn(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("server.port")),
        ("value", Value::from(8080)),
    ]))
    .unwrap();

    let json: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json["server"]["host"], "localhost");
    assert_eq!(json["server"]["port"], 8080);
}

#[test]
fn test_object_set_create_nested() {
    let obj = serde_json::json!({"a": 1});

    let result = object::object_set_fn(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("b.c.d")),
        ("value", Value::from("nested")),
    ]))
    .unwrap();

    let json: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json["a"], 1);
    assert_eq!(json["b"]["c"]["d"], "nested");
}

#[test]
fn test_object_set_override() {
    let obj = serde_json::json!({"a": {"b": 1}});

    let result = object::object_set_fn(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("a.b")),
        ("value", Value::from(2)),
    ]))
    .unwrap();

    let json: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json["a"]["b"], 2);
}

#[test]
fn test_object_keys_simple() {
    let obj = serde_json::json!({"host": "localhost", "port": 8080, "debug": true});

    let result = object::object_keys_fn(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]))
    .unwrap();

    let keys: Vec<_> = result.try_iter().unwrap().collect();
    assert_eq!(keys.len(), 3);

    let key_strings: Vec<String> = keys
        .iter()
        .map(|v| v.as_str().unwrap().to_string())
        .collect();
    assert!(key_strings.contains(&"host".to_string()));
    assert!(key_strings.contains(&"port".to_string()));
    assert!(key_strings.contains(&"debug".to_string()));
}

#[test]
fn test_object_keys_empty() {
    let obj = serde_json::json!({});

    let result = object::object_keys_fn(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]))
    .unwrap();

    let keys: Vec<_> = result.try_iter().unwrap().collect();
    assert_eq!(keys.len(), 0);
}

#[test]
fn test_object_keys_not_object() {
    let arr = serde_json::json!([1, 2, 3]);

    let result = object::object_keys_fn(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&arr),
    )]));

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires an object")
    );
}

#[test]
fn test_object_values_simple() {
    let obj = serde_json::json!({"a": 1, "b": 2, "c": 3});

    let result = object::object_values_fn(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]))
    .unwrap();

    let values: Vec<_> = result.try_iter().unwrap().collect();
    assert_eq!(values.len(), 3);

    let value_numbers: Vec<i64> = values.iter().map(|v| v.as_i64().unwrap()).collect();
    assert!(value_numbers.contains(&1));
    assert!(value_numbers.contains(&2));
    assert!(value_numbers.contains(&3));
}

#[test]
fn test_object_values_mixed_types() {
    let obj = serde_json::json!({"str": "hello", "num": 42, "bool": true});

    let result = object::object_values_fn(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]))
    .unwrap();

    let values: Vec<_> = result.try_iter().unwrap().collect();
    assert_eq!(values.len(), 3);
}

#[test]
fn test_object_values_empty() {
    let obj = serde_json::json!({});

    let result = object::object_values_fn(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]))
    .unwrap();

    let values: Vec<_> = result.try_iter().unwrap().collect();
    assert_eq!(values.len(), 0);
}

#[test]
fn test_object_values_not_object() {
    let arr = serde_json::json!([1, 2, 3]);

    let result = object::object_values_fn(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&arr),
    )]));

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires an object")
    );
}

#[test]
fn test_object_has_key_exists() {
    let obj = serde_json::json!({"host": "localhost", "port": 8080});

    let result = object::object_has_key_fn(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("key", Value::from("host")),
    ]))
    .unwrap();

    assert!(result.is_true());
}

#[test]
fn test_object_has_key_not_exists() {
    let obj = serde_json::json!({"host": "localhost", "port": 8080});

    let result = object::object_has_key_fn(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("key", Value::from("database")),
    ]))
    .unwrap();

    assert!(!result.is_true());
}

#[test]
fn test_object_has_key_empty_object() {
    let obj = serde_json::json!({});

    let result = object::object_has_key_fn(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("key", Value::from("any")),
    ]))
    .unwrap();

    assert!(!result.is_true());
}

#[test]
fn test_object_has_key_not_object() {
    let arr = serde_json::json!([1, 2, 3]);

    let result = object::object_has_key_fn(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&arr)),
        ("key", Value::from("any")),
    ]))
    .unwrap();

    assert!(!result.is_true());
}

#[test]
fn test_object_merge_empty() {
    let obj1 = serde_json::json!({});
    let obj2 = serde_json::json!({"a": 1});

    let result = object::object_merge_fn(Kwargs::from_iter(vec![
        ("obj1", Value::from_serialize(&obj1)),
        ("obj2", Value::from_serialize(&obj2)),
    ]))
    .unwrap();

    let json: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json["a"], 1);
}

#[test]
fn test_object_set_on_empty() {
    let obj = serde_json::json!({});

    let result = object::object_set_fn(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("a.b.c")),
        ("value", Value::from(123)),
    ]))
    .unwrap();

    let json: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json["a"]["b"]["c"], 123);
}

#[test]
fn test_object_get_number_value() {
    let obj = serde_json::json!({"count": 42});

    let result = object::object_get_fn(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("count")),
    ]))
    .unwrap();

    assert_eq!(result.as_i64(), Some(42));
}

#[test]
fn test_object_get_boolean_value() {
    let obj = serde_json::json!({"active": true});

    let result = object::object_get_fn(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("active")),
    ]))
    .unwrap();

    assert!(result.is_true());
}

#[test]
fn test_object_set_string_value() {
    let obj = serde_json::json!({"name": "old"});

    let result = object::object_set_fn(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("name")),
        ("value", Value::from("new")),
    ]))
    .unwrap();

    let json: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json["name"], "new");
}

// ==================== json_path Tests ====================

#[test]
fn test_json_path_simple() {
    let obj = serde_json::json!({"name": "John", "age": 30});

    let result = object::json_path_fn(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("$.name")),
    ]))
    .unwrap();

    assert_eq!(result.as_str().unwrap(), "John");
}

#[test]
fn test_json_path_nested() {
    let obj = serde_json::json!({"user": {"name": "John", "email": "john@example.com"}});

    let result = object::json_path_fn(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("$.user.email")),
    ]))
    .unwrap();

    assert_eq!(result.as_str().unwrap(), "john@example.com");
}

#[test]
fn test_json_path_array_index() {
    let obj = serde_json::json!({"users": ["Alice", "Bob", "Charlie"]});

    let result = object::json_path_fn(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("$.users[1]")),
    ]))
    .unwrap();

    assert_eq!(result.as_str().unwrap(), "Bob");
}

#[test]
fn test_json_path_wildcard() {
    let obj = serde_json::json!({"users": [{"name": "Alice"}, {"name": "Bob"}]});

    let result = object::json_path_fn(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("$.users[*].name")),
    ]))
    .unwrap();

    let json: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert!(json.is_array());
    let arr = json.as_array().unwrap();
    assert_eq!(arr.len(), 2);
    assert_eq!(arr[0], "Alice");
    assert_eq!(arr[1], "Bob");
}

#[test]
fn test_json_path_without_dollar() {
    let obj = serde_json::json!({"name": "John"});

    let result = object::json_path_fn(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("name")),
    ]))
    .unwrap();

    assert_eq!(result.as_str().unwrap(), "John");
}

#[test]
fn test_json_path_not_found() {
    let obj = serde_json::json!({"name": "John"});

    let result = object::json_path_fn(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("$.nonexistent")),
    ]))
    .unwrap();

    assert!(result.is_none());
}

// ==================== object_pick Tests ====================

#[test]
fn test_object_pick_basic() {
    let obj = serde_json::json!({"a": 1, "b": 2, "c": 3, "d": 4});
    let keys = serde_json::json!(["a", "c"]);

    let result = object::object_pick_fn(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("keys", Value::from_serialize(&keys)),
    ]))
    .unwrap();

    let json: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json["a"], 1);
    assert_eq!(json["c"], 3);
    assert!(json.get("b").is_none());
    assert!(json.get("d").is_none());
}

#[test]
fn test_object_pick_missing_keys() {
    let obj = serde_json::json!({"a": 1, "b": 2});
    let keys = serde_json::json!(["a", "x", "y"]);

    let result = object::object_pick_fn(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("keys", Value::from_serialize(&keys)),
    ]))
    .unwrap();

    let json: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json["a"], 1);
    assert!(json.get("x").is_none());
}

#[test]
fn test_object_pick_empty_keys() {
    let obj = serde_json::json!({"a": 1, "b": 2});
    let keys = serde_json::json!([]);

    let result = object::object_pick_fn(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("keys", Value::from_serialize(&keys)),
    ]))
    .unwrap();

    let json: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert!(json.as_object().unwrap().is_empty());
}

// ==================== object_omit Tests ====================

#[test]
fn test_object_omit_basic() {
    let obj = serde_json::json!({"a": 1, "b": 2, "c": 3, "d": 4});
    let keys = serde_json::json!(["b", "d"]);

    let result = object::object_omit_fn(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("keys", Value::from_serialize(&keys)),
    ]))
    .unwrap();

    let json: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json["a"], 1);
    assert_eq!(json["c"], 3);
    assert!(json.get("b").is_none());
    assert!(json.get("d").is_none());
}

#[test]
fn test_object_omit_missing_keys() {
    let obj = serde_json::json!({"a": 1, "b": 2});
    let keys = serde_json::json!(["x", "y"]);

    let result = object::object_omit_fn(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("keys", Value::from_serialize(&keys)),
    ]))
    .unwrap();

    let json: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json["a"], 1);
    assert_eq!(json["b"], 2);
}

#[test]
fn test_object_omit_empty_keys() {
    let obj = serde_json::json!({"a": 1, "b": 2});
    let keys = serde_json::json!([]);

    let result = object::object_omit_fn(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("keys", Value::from_serialize(&keys)),
    ]))
    .unwrap();

    let json: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json["a"], 1);
    assert_eq!(json["b"], 2);
}

// ==================== object_rename_keys Tests ====================

#[test]
fn test_object_rename_keys_basic() {
    let obj = serde_json::json!({"old_name": "value", "keep": 123});
    let mapping = serde_json::json!({"old_name": "new_name"});

    let result = object::object_rename_keys_fn(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("mapping", Value::from_serialize(&mapping)),
    ]))
    .unwrap();

    let json: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json["new_name"], "value");
    assert_eq!(json["keep"], 123);
    assert!(json.get("old_name").is_none());
}

#[test]
fn test_object_rename_keys_multiple() {
    let obj = serde_json::json!({"a": 1, "b": 2, "c": 3});
    let mapping = serde_json::json!({"a": "x", "b": "y"});

    let result = object::object_rename_keys_fn(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("mapping", Value::from_serialize(&mapping)),
    ]))
    .unwrap();

    let json: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json["x"], 1);
    assert_eq!(json["y"], 2);
    assert_eq!(json["c"], 3);
}

#[test]
fn test_object_rename_keys_no_match() {
    let obj = serde_json::json!({"a": 1, "b": 2});
    let mapping = serde_json::json!({"x": "y"});

    let result = object::object_rename_keys_fn(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("mapping", Value::from_serialize(&mapping)),
    ]))
    .unwrap();

    let json: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json["a"], 1);
    assert_eq!(json["b"], 2);
}

// ==================== object_flatten Tests ====================

#[test]
fn test_object_flatten_basic() {
    let obj = serde_json::json!({"a": {"b": {"c": "value"}}});

    let result = object::object_flatten_fn(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]))
    .unwrap();

    let json: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json["a.b.c"], "value");
}

#[test]
fn test_object_flatten_custom_delimiter() {
    let obj = serde_json::json!({"a": {"b": 1}});

    let result = object::object_flatten_fn(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("delimiter", Value::from("_")),
    ]))
    .unwrap();

    let json: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json["a_b"], 1);
}

#[test]
fn test_object_flatten_mixed() {
    let obj = serde_json::json!({"user": {"name": "John", "address": {"city": "NYC"}}});

    let result = object::object_flatten_fn(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]))
    .unwrap();

    let json: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json["user.name"], "John");
    assert_eq!(json["user.address.city"], "NYC");
}

#[test]
fn test_object_flatten_already_flat() {
    let obj = serde_json::json!({"a": 1, "b": 2});

    let result = object::object_flatten_fn(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]))
    .unwrap();

    let json: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json["a"], 1);
    assert_eq!(json["b"], 2);
}

// ==================== object_unflatten Tests ====================

#[test]
fn test_object_unflatten_basic() {
    let obj = serde_json::json!({"a.b.c": "value"});

    let result = object::object_unflatten_fn(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]))
    .unwrap();

    let json: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json["a"]["b"]["c"], "value");
}

#[test]
fn test_object_unflatten_custom_delimiter() {
    let obj = serde_json::json!({"a_b_c": "value"});

    let result = object::object_unflatten_fn(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("delimiter", Value::from("_")),
    ]))
    .unwrap();

    let json: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json["a"]["b"]["c"], "value");
}

#[test]
fn test_object_unflatten_multiple_keys() {
    let obj = serde_json::json!({"a.b": 1, "a.c": 2, "d": 3});

    let result = object::object_unflatten_fn(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]))
    .unwrap();

    let json: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json["a"]["b"], 1);
    assert_eq!(json["a"]["c"], 2);
    assert_eq!(json["d"], 3);
}

#[test]
fn test_object_flatten_unflatten_roundtrip() {
    let obj = serde_json::json!({"user": {"name": "John", "age": 30}});

    // Flatten
    let flattened = object::object_flatten_fn(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]))
    .unwrap();

    // Unflatten
    let result =
        object::object_unflatten_fn(Kwargs::from_iter(vec![("object", flattened)])).unwrap();

    let json: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json["user"]["name"], "John");
    assert_eq!(json["user"]["age"], 30);
}
