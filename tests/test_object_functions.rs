use minijinja::Value;
use minijinja::value::Kwargs;
use tmpltool::functions::Function;
use tmpltool::functions::object::{
    JsonPath, ObjectGet, ObjectHasKey, ObjectMerge, ObjectOmit, ObjectPick, ObjectRenameKeys,
    ObjectSet, ObjectUnflatten,
};

#[test]
fn test_object_merge_simple() {
    let obj1 = serde_json::json!({"a": 1, "b": 2});
    let obj2 = serde_json::json!({"c": 3});

    let result = ObjectMerge::call(Kwargs::from_iter(vec![
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

    let result = ObjectMerge::call(Kwargs::from_iter(vec![
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

    let result = ObjectMerge::call(Kwargs::from_iter(vec![
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

    let result = ObjectMerge::call(Kwargs::from_iter(vec![
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

    let result = ObjectGet::call(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("host")),
    ]))
    .unwrap();

    assert_eq!(result.as_str().unwrap(), "localhost");
}

#[test]
fn test_object_get_nested() {
    let obj = serde_json::json!({"server": {"host": "localhost", "port": 8080}});

    let result = ObjectGet::call(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("server.host")),
    ]))
    .unwrap();

    assert_eq!(result.as_str().unwrap(), "localhost");
}

#[test]
fn test_object_get_deep_nested() {
    let obj = serde_json::json!({"a": {"b": {"c": {"d": "value"}}}});

    let result = ObjectGet::call(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("a.b.c.d")),
    ]))
    .unwrap();

    assert_eq!(result.as_str().unwrap(), "value");
}

#[test]
fn test_object_get_array_index() {
    let obj = serde_json::json!({"items": [10, 20, 30]});

    let result = ObjectGet::call(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("items.1")),
    ]))
    .unwrap();

    assert_eq!(result.as_i64(), Some(20));
}

#[test]
fn test_object_get_not_found() {
    let obj = serde_json::json!({"a": 1});

    let result = ObjectGet::call(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("b")),
    ]))
    .unwrap();

    assert!(result.is_undefined());
}

#[test]
fn test_object_get_nested_not_found() {
    let obj = serde_json::json!({"a": {"b": 1}});

    let result = ObjectGet::call(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("a.c")),
    ]))
    .unwrap();

    assert!(result.is_undefined());
}

#[test]
fn test_object_set_simple() {
    let obj = serde_json::json!({"a": 1});

    let result = ObjectSet::call(Kwargs::from_iter(vec![
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

    let result = ObjectSet::call(Kwargs::from_iter(vec![
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

    let result = ObjectSet::call(Kwargs::from_iter(vec![
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

    let result = ObjectSet::call(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("a.b")),
        ("value", Value::from(2)),
    ]))
    .unwrap();

    let json: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json["a"]["b"], 2);
}

// Note: object_keys_fn and object_values_fn tests removed - these functions
// are now in filter_functions/object.rs with dual function+filter syntax support.
// See tests/test_filters_integration.rs for integration tests of these filters.

#[test]
fn test_object_has_key_exists() {
    let obj = serde_json::json!({"host": "localhost", "port": 8080});

    let result = ObjectHasKey::call(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("key", Value::from("host")),
    ]))
    .unwrap();

    assert!(result.is_true());
}

#[test]
fn test_object_has_key_not_exists() {
    let obj = serde_json::json!({"host": "localhost", "port": 8080});

    let result = ObjectHasKey::call(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("key", Value::from("database")),
    ]))
    .unwrap();

    assert!(!result.is_true());
}

#[test]
fn test_object_has_key_empty_object() {
    let obj = serde_json::json!({});

    let result = ObjectHasKey::call(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("key", Value::from("any")),
    ]))
    .unwrap();

    assert!(!result.is_true());
}

#[test]
fn test_object_has_key_not_object() {
    let arr = serde_json::json!([1, 2, 3]);

    let result = ObjectHasKey::call(Kwargs::from_iter(vec![
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

    let result = ObjectMerge::call(Kwargs::from_iter(vec![
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

    let result = ObjectSet::call(Kwargs::from_iter(vec![
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

    let result = ObjectGet::call(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("count")),
    ]))
    .unwrap();

    assert_eq!(result.as_i64(), Some(42));
}

#[test]
fn test_object_get_boolean_value() {
    let obj = serde_json::json!({"active": true});

    let result = ObjectGet::call(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("active")),
    ]))
    .unwrap();

    assert!(result.is_true());
}

#[test]
fn test_object_set_string_value() {
    let obj = serde_json::json!({"name": "old"});

    let result = ObjectSet::call(Kwargs::from_iter(vec![
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

    let result = JsonPath::call(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("$.name")),
    ]))
    .unwrap();

    assert_eq!(result.as_str().unwrap(), "John");
}

#[test]
fn test_json_path_nested() {
    let obj = serde_json::json!({"user": {"name": "John", "email": "john@example.com"}});

    let result = JsonPath::call(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("$.user.email")),
    ]))
    .unwrap();

    assert_eq!(result.as_str().unwrap(), "john@example.com");
}

#[test]
fn test_json_path_array_index() {
    let obj = serde_json::json!({"users": ["Alice", "Bob", "Charlie"]});

    let result = JsonPath::call(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("$.users[1]")),
    ]))
    .unwrap();

    assert_eq!(result.as_str().unwrap(), "Bob");
}

#[test]
fn test_json_path_wildcard() {
    let obj = serde_json::json!({"users": [{"name": "Alice"}, {"name": "Bob"}]});

    let result = JsonPath::call(Kwargs::from_iter(vec![
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

    let result = JsonPath::call(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("name")),
    ]))
    .unwrap();

    assert_eq!(result.as_str().unwrap(), "John");
}

#[test]
fn test_json_path_not_found() {
    let obj = serde_json::json!({"name": "John"});

    let result = JsonPath::call(Kwargs::from_iter(vec![
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

    let result = ObjectPick::call(Kwargs::from_iter(vec![
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

    let result = ObjectPick::call(Kwargs::from_iter(vec![
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

    let result = ObjectPick::call(Kwargs::from_iter(vec![
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

    let result = ObjectOmit::call(Kwargs::from_iter(vec![
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

    let result = ObjectOmit::call(Kwargs::from_iter(vec![
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

    let result = ObjectOmit::call(Kwargs::from_iter(vec![
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

    let result = ObjectRenameKeys::call(Kwargs::from_iter(vec![
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

    let result = ObjectRenameKeys::call(Kwargs::from_iter(vec![
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

    let result = ObjectRenameKeys::call(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("mapping", Value::from_serialize(&mapping)),
    ]))
    .unwrap();

    let json: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json["a"], 1);
    assert_eq!(json["b"], 2);
}

// Note: object_flatten_fn tests removed - function now in filter_functions/object.rs
// with dual function+filter syntax support.
// See tests/test_filters_integration.rs for integration tests.

// ==================== object_unflatten Tests ====================

#[test]
fn test_object_unflatten_basic() {
    let obj = serde_json::json!({"a.b.c": "value"});

    let result = ObjectUnflatten::call(Kwargs::from_iter(vec![(
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

    let result = ObjectUnflatten::call(Kwargs::from_iter(vec![
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

    let result = ObjectUnflatten::call(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]))
    .unwrap();

    let json: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json["a"]["b"], 1);
    assert_eq!(json["a"]["c"], 2);
    assert_eq!(json["d"], 3);
}

// Note: test_object_flatten_unflatten_roundtrip removed - object_flatten_fn was moved

// ============================================================================
// Additional Edge Case Tests for Coverage Improvement
// ============================================================================

// --- object_merge edge cases ---

#[test]
fn test_object_merge_non_object_overlay() {
    // When overlay is not an object, it completely replaces the base
    let obj1 = serde_json::json!({"a": 1});
    let obj2 = serde_json::json!("string value");

    let result = ObjectMerge::call(Kwargs::from_iter(vec![
        ("obj1", Value::from_serialize(&obj1)),
        ("obj2", Value::from_serialize(&obj2)),
    ]))
    .unwrap();

    assert_eq!(result.as_str().unwrap(), "string value");
}

#[test]
fn test_object_merge_array_overlay() {
    // When overlay is an array, it replaces the base
    let obj1 = serde_json::json!({"a": 1});
    let obj2 = serde_json::json!([1, 2, 3]);

    let result = ObjectMerge::call(Kwargs::from_iter(vec![
        ("obj1", Value::from_serialize(&obj1)),
        ("obj2", Value::from_serialize(&obj2)),
    ]))
    .unwrap();

    let json: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert!(json.is_array());
}

#[test]
fn test_object_merge_nested_array_replacement() {
    // When nested value is an array in overlay, it replaces the base array
    let obj1 = serde_json::json!({"items": [1, 2, 3]});
    let obj2 = serde_json::json!({"items": [4, 5]});

    let result = ObjectMerge::call(Kwargs::from_iter(vec![
        ("obj1", Value::from_serialize(&obj1)),
        ("obj2", Value::from_serialize(&obj2)),
    ]))
    .unwrap();

    let json: serde_json::Value = serde_json::to_value(&result).unwrap();
    let items = json["items"].as_array().unwrap();
    assert_eq!(items.len(), 2);
    assert_eq!(items[0], 4);
    assert_eq!(items[1], 5);
}

// --- object_get edge cases ---

#[test]
fn test_object_get_array_out_of_bounds() {
    let obj = serde_json::json!({"items": [1, 2, 3]});

    let result = ObjectGet::call(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("items.10")),
    ]))
    .unwrap();

    assert!(result.is_undefined());
}

#[test]
fn test_object_get_non_integer_array_index() {
    let obj = serde_json::json!({"items": [1, 2, 3]});

    let result = ObjectGet::call(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("items.abc")),
    ]))
    .unwrap();

    assert!(result.is_undefined());
}

#[test]
fn test_object_get_path_through_primitive() {
    let obj = serde_json::json!({"value": 42});

    let result = ObjectGet::call(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("value.nested")),
    ]))
    .unwrap();

    assert!(result.is_undefined());
}

#[test]
fn test_object_get_nested_array() {
    let obj = serde_json::json!({"matrix": [[1, 2], [3, 4]]});

    let result = ObjectGet::call(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("matrix.1.0")),
    ]))
    .unwrap();

    assert_eq!(result.as_i64(), Some(3));
}

#[test]
fn test_object_get_array_of_objects() {
    let obj = serde_json::json!({"users": [{"name": "Alice"}, {"name": "Bob"}]});

    let result = ObjectGet::call(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("users.1.name")),
    ]))
    .unwrap();

    assert_eq!(result.as_str().unwrap(), "Bob");
}

// --- object_set edge cases ---

#[test]
fn test_object_set_on_primitive() {
    // Setting nested path on a primitive value - should convert to object
    let obj = serde_json::json!(42);

    let result = ObjectSet::call(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("a.b")),
        ("value", Value::from("new")),
    ]))
    .unwrap();

    let json: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json["a"]["b"], "new");
}

#[test]
fn test_object_set_replace_primitive_with_nested() {
    // Setting a nested path where an intermediate is a primitive
    let obj = serde_json::json!({"a": 1});

    let result = ObjectSet::call(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("a.b.c")),
        ("value", Value::from("deep")),
    ]))
    .unwrap();

    let json: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json["a"]["b"]["c"], "deep");
}

#[test]
fn test_object_set_array_value() {
    let obj = serde_json::json!({"config": {}});

    let result = ObjectSet::call(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("config.items")),
        ("value", Value::from_serialize(serde_json::json!([1, 2, 3]))),
    ]))
    .unwrap();

    let json: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert!(json["config"]["items"].is_array());
}

#[test]
fn test_object_set_null_value() {
    let obj = serde_json::json!({"a": 1});

    let result = ObjectSet::call(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("b")),
        ("value", Value::from_serialize(&serde_json::Value::Null)),
    ]))
    .unwrap();

    let json: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert!(json["b"].is_null());
}

// --- json_path edge cases ---

#[test]
fn test_json_path_empty_path_returns_object() {
    let obj = serde_json::json!({"a": 1, "b": 2});

    let result = JsonPath::call(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("$")),
    ]))
    .unwrap();

    let json: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json["a"], 1);
    assert_eq!(json["b"], 2);
}

#[test]
fn test_json_path_dollar_dot_empty() {
    let obj = serde_json::json!({"a": 1});

    let result = JsonPath::call(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("$.")),
    ]))
    .unwrap();

    let json: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json["a"], 1);
}

#[test]
fn test_json_path_wildcard_on_non_array() {
    let obj = serde_json::json!({"key": "value"});

    let result = JsonPath::call(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("$[*]")),
    ]))
    .unwrap();

    assert!(result.is_none());
}

#[test]
fn test_json_path_index_on_non_array() {
    let obj = serde_json::json!({"key": "value"});

    let result = JsonPath::call(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("$[0]")),
    ]))
    .unwrap();

    assert!(result.is_none());
}

#[test]
fn test_json_path_invalid_array_index() {
    let obj = serde_json::json!({"items": [1, 2, 3]});

    let result = JsonPath::call(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("$.items[abc]")),
    ]));

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Invalid array index")
    );
}

#[test]
fn test_json_path_unclosed_bracket() {
    let obj = serde_json::json!({"items": [1, 2, 3]});

    let result = JsonPath::call(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("$.items[0")),
    ]));

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Unclosed bracket"));
}

#[test]
fn test_json_path_property_on_primitive() {
    let obj = serde_json::json!({"value": 42});

    let result = JsonPath::call(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("$.value.nested")),
    ]))
    .unwrap();

    assert!(result.is_none());
}

#[test]
fn test_json_path_array_then_property() {
    let obj =
        serde_json::json!({"users": [{"name": "Alice", "age": 30}, {"name": "Bob", "age": 25}]});

    let result = JsonPath::call(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("$.users[0].age")),
    ]))
    .unwrap();

    assert_eq!(result.as_i64(), Some(30));
}

#[test]
fn test_json_path_wildcard_returns_all() {
    let obj = serde_json::json!({"items": [1, 2, 3]});

    let result = JsonPath::call(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("$.items[*]")),
    ]))
    .unwrap();

    let json: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert!(json.is_array());
    let arr = json.as_array().unwrap();
    assert_eq!(arr.len(), 3);
}

#[test]
fn test_json_path_array_out_of_bounds() {
    let obj = serde_json::json!({"items": [1, 2]});

    let result = JsonPath::call(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("$.items[10]")),
    ]))
    .unwrap();

    assert!(result.is_none());
}

#[test]
fn test_json_path_complex_nested() {
    let obj = serde_json::json!({
        "data": {
            "users": [
                {"profile": {"name": "Alice"}},
                {"profile": {"name": "Bob"}}
            ]
        }
    });

    let result = JsonPath::call(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("$.data.users[*].profile.name")),
    ]))
    .unwrap();

    let json: serde_json::Value = serde_json::to_value(&result).unwrap();
    let arr = json.as_array().unwrap();
    assert_eq!(arr[0], "Alice");
    assert_eq!(arr[1], "Bob");
}

#[test]
fn test_json_path_bracket_before_dot() {
    let obj = serde_json::json!({"items": [{"a": 1}, {"a": 2}]});

    let result = JsonPath::call(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("items[0].a")),
    ]))
    .unwrap();

    assert_eq!(result.as_i64(), Some(1));
}

// --- object_pick error cases ---

#[test]
fn test_object_pick_keys_not_array() {
    let obj = serde_json::json!({"a": 1, "b": 2});

    let result = ObjectPick::call(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("keys", Value::from("not an array")),
    ]));

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("keys must be an array")
    );
}

#[test]
fn test_object_pick_not_object() {
    let arr = serde_json::json!([1, 2, 3]);
    let keys = serde_json::json!(["a"]);

    let result = ObjectPick::call(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&arr)),
        ("keys", Value::from_serialize(&keys)),
    ]));

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires an object")
    );
}

#[test]
fn test_object_pick_all_keys() {
    let obj = serde_json::json!({"a": 1, "b": 2, "c": 3});
    let keys = serde_json::json!(["a", "b", "c"]);

    let result = ObjectPick::call(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("keys", Value::from_serialize(&keys)),
    ]))
    .unwrap();

    let json: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json["a"], 1);
    assert_eq!(json["b"], 2);
    assert_eq!(json["c"], 3);
}

// --- object_omit error cases ---

#[test]
fn test_object_omit_keys_not_array() {
    let obj = serde_json::json!({"a": 1, "b": 2});

    let result = ObjectOmit::call(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("keys", Value::from("not an array")),
    ]));

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("keys must be an array")
    );
}

#[test]
fn test_object_omit_not_object() {
    let arr = serde_json::json!([1, 2, 3]);
    let keys = serde_json::json!(["a"]);

    let result = ObjectOmit::call(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&arr)),
        ("keys", Value::from_serialize(&keys)),
    ]));

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires an object")
    );
}

#[test]
fn test_object_omit_all_keys() {
    let obj = serde_json::json!({"a": 1, "b": 2});
    let keys = serde_json::json!(["a", "b"]);

    let result = ObjectOmit::call(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("keys", Value::from_serialize(&keys)),
    ]))
    .unwrap();

    let json: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert!(json.as_object().unwrap().is_empty());
}

// --- object_rename_keys error cases ---

#[test]
fn test_object_rename_keys_mapping_not_object() {
    let obj = serde_json::json!({"a": 1});

    let result = ObjectRenameKeys::call(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("mapping", Value::from("not an object")),
    ]));

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("mapping must be an object")
    );
}

#[test]
fn test_object_rename_keys_not_object() {
    let arr = serde_json::json!([1, 2, 3]);
    let mapping = serde_json::json!({"a": "b"});

    let result = ObjectRenameKeys::call(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&arr)),
        ("mapping", Value::from_serialize(&mapping)),
    ]));

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires an object")
    );
}

#[test]
fn test_object_rename_keys_empty_mapping() {
    let obj = serde_json::json!({"a": 1, "b": 2});
    let mapping = serde_json::json!({});

    let result = ObjectRenameKeys::call(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("mapping", Value::from_serialize(&mapping)),
    ]))
    .unwrap();

    let json: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json["a"], 1);
    assert_eq!(json["b"], 2);
}

#[test]
fn test_object_rename_keys_non_string_values() {
    // Mapping values that aren't strings should be ignored
    let obj = serde_json::json!({"a": 1, "b": 2});
    let mapping = serde_json::json!({"a": 123, "b": "new_b"});

    let result = ObjectRenameKeys::call(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("mapping", Value::from_serialize(&mapping)),
    ]))
    .unwrap();

    let json: serde_json::Value = serde_json::to_value(&result).unwrap();
    // "a" mapping is ignored (123 is not a string)
    assert_eq!(json["a"], 1);
    // "b" is renamed to "new_b"
    assert_eq!(json["new_b"], 2);
}

// --- object_unflatten error and edge cases ---

#[test]
fn test_object_unflatten_not_object() {
    let arr = serde_json::json!([1, 2, 3]);

    let result = ObjectUnflatten::call(Kwargs::from_iter(vec![(
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
fn test_object_unflatten_empty() {
    let obj = serde_json::json!({});

    let result = ObjectUnflatten::call(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]))
    .unwrap();

    let json: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert!(json.as_object().unwrap().is_empty());
}

#[test]
fn test_object_unflatten_single_key() {
    let obj = serde_json::json!({"key": "value"});

    let result = ObjectUnflatten::call(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]))
    .unwrap();

    let json: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json["key"], "value");
}

#[test]
fn test_object_unflatten_overlapping_paths() {
    // When paths partially overlap, later values should merge
    let obj = serde_json::json!({"a.b": 1, "a.c": 2});

    let result = ObjectUnflatten::call(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]))
    .unwrap();

    let json: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json["a"]["b"], 1);
    assert_eq!(json["a"]["c"], 2);
}

// --- Missing parameter tests ---

#[test]
fn test_object_merge_missing_obj1() {
    let obj2 = serde_json::json!({"a": 1});

    let result = ObjectMerge::call(Kwargs::from_iter(vec![(
        "obj2",
        Value::from_serialize(&obj2),
    )]));

    assert!(result.is_err());
}

#[test]
fn test_object_merge_missing_obj2() {
    let obj1 = serde_json::json!({"a": 1});

    let result = ObjectMerge::call(Kwargs::from_iter(vec![(
        "obj1",
        Value::from_serialize(&obj1),
    )]));

    assert!(result.is_err());
}

#[test]
fn test_object_get_missing_object() {
    let result = ObjectGet::call(Kwargs::from_iter(vec![("path", Value::from("a.b"))]));

    assert!(result.is_err());
}

#[test]
fn test_object_get_missing_path() {
    let obj = serde_json::json!({"a": 1});

    let result = ObjectGet::call(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]));

    assert!(result.is_err());
}

#[test]
fn test_object_set_missing_value() {
    let obj = serde_json::json!({"a": 1});

    let result = ObjectSet::call(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("path", Value::from("b")),
    ]));

    assert!(result.is_err());
}

// Note: test_object_keys_missing_object, test_object_values_missing_object removed

#[test]
fn test_object_has_key_missing_object() {
    let result = ObjectHasKey::call(Kwargs::from_iter(vec![("key", Value::from("test"))]));

    assert!(result.is_err());
}

#[test]
fn test_object_has_key_missing_key() {
    let obj = serde_json::json!({"a": 1});

    let result = ObjectHasKey::call(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]));

    assert!(result.is_err());
}

#[test]
fn test_json_path_missing_object() {
    let result = JsonPath::call(Kwargs::from_iter(vec![("path", Value::from("$.name"))]));

    assert!(result.is_err());
}

#[test]
fn test_json_path_missing_path() {
    let obj = serde_json::json!({"a": 1});

    let result = JsonPath::call(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]));

    assert!(result.is_err());
}

#[test]
fn test_object_pick_missing_keys_param() {
    let obj = serde_json::json!({"a": 1});

    let result = ObjectPick::call(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]));

    assert!(result.is_err());
}

#[test]
fn test_object_omit_missing_keys_param() {
    let obj = serde_json::json!({"a": 1});

    let result = ObjectOmit::call(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]));

    assert!(result.is_err());
}

#[test]
fn test_object_rename_keys_missing_mapping() {
    let obj = serde_json::json!({"a": 1});

    let result = ObjectRenameKeys::call(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]));

    assert!(result.is_err());
}

// Note: test_object_flatten_missing_object removed

#[test]
fn test_object_unflatten_missing_object() {
    let result = ObjectUnflatten::call(Kwargs::from_iter(Vec::<(&str, Value)>::new()));

    assert!(result.is_err());
}
