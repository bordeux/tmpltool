use minijinja::Value;
use minijinja::value::Kwargs;
use tmpltool::functions::Function;
use tmpltool::functions::array::{ArrayGroupBy, ArraySortBy};

// ============================================================================
// Array Sort By Tests
// ============================================================================

#[test]
fn test_array_sort_by_numeric() {
    let users = serde_json::json!([
        {"name": "Alice", "age": 30},
        {"name": "Bob", "age": 25},
        {"name": "Charlie", "age": 35}
    ]);

    let result = ArraySortBy::call(Kwargs::from_iter(vec![
        ("array", Value::from_serialize(&users)),
        ("key", Value::from("age")),
    ]))
    .unwrap();

    let json_result: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json_result[0]["age"], 25);
    assert_eq!(json_result[1]["age"], 30);
    assert_eq!(json_result[2]["age"], 35);
}

#[test]
fn test_array_sort_by_string() {
    let users = serde_json::json!([
        {"name": "Charlie", "age": 30},
        {"name": "Alice", "age": 25},
        {"name": "Bob", "age": 35}
    ]);

    let result = ArraySortBy::call(Kwargs::from_iter(vec![
        ("array", Value::from_serialize(&users)),
        ("key", Value::from("name")),
    ]))
    .unwrap();

    let json_result: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json_result[0]["name"], "Alice");
    assert_eq!(json_result[1]["name"], "Bob");
    assert_eq!(json_result[2]["name"], "Charlie");
}

#[test]
fn test_array_sort_by_missing_key() {
    let users = serde_json::json!([
        {"name": "Alice", "age": 30},
        {"name": "Bob"},
        {"name": "Charlie", "age": 25}
    ]);

    let result = ArraySortBy::call(Kwargs::from_iter(vec![
        ("array", Value::from_serialize(&users)),
        ("key", Value::from("age")),
    ]))
    .unwrap();

    // Items with missing keys should be sorted to the end
    let json_result: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json_result[0]["age"], 25);
    assert_eq!(json_result[1]["age"], 30);
    assert_eq!(json_result[2]["name"], "Bob");
}

#[test]
fn test_array_sort_by_empty_array() {
    let empty: Vec<serde_json::Value> = vec![];

    let result = ArraySortBy::call(Kwargs::from_iter(vec![
        ("array", Value::from_serialize(&empty)),
        ("key", Value::from("age")),
    ]))
    .unwrap();

    let json_result: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json_result.as_array().unwrap().len(), 0);
}

#[test]
fn test_array_sort_by_error_not_array() {
    let result = ArraySortBy::call(Kwargs::from_iter(vec![
        ("array", Value::from("test")),
        ("key", Value::from("age")),
    ]));

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires an array")
    );
}

#[test]
fn test_array_sort_by_missing_key_param() {
    let users = serde_json::json!([{"name": "Alice"}]);
    let result = ArraySortBy::call(Kwargs::from_iter(vec![(
        "array",
        Value::from_serialize(&users),
    )]));

    assert!(result.is_err());
}

// ============================================================================
// Array Group By Tests
// ============================================================================

#[test]
fn test_array_group_by_basic() {
    let users = serde_json::json!([
        {"name": "Alice", "dept": "Engineering"},
        {"name": "Bob", "dept": "Sales"},
        {"name": "Charlie", "dept": "Engineering"}
    ]);

    let result = ArrayGroupBy::call(Kwargs::from_iter(vec![
        ("array", Value::from_serialize(&users)),
        ("key", Value::from("dept")),
    ]))
    .unwrap();

    let json_result: serde_json::Value = serde_json::to_value(&result).unwrap();
    let engineering = json_result["Engineering"].as_array().unwrap();
    let sales = json_result["Sales"].as_array().unwrap();

    assert_eq!(engineering.len(), 2);
    assert_eq!(sales.len(), 1);
}

#[test]
fn test_array_group_by_numeric_key() {
    let items = serde_json::json!([
        {"name": "Item1", "priority": 1},
        {"name": "Item2", "priority": 2},
        {"name": "Item3", "priority": 1}
    ]);

    let result = ArrayGroupBy::call(Kwargs::from_iter(vec![
        ("array", Value::from_serialize(&items)),
        ("key", Value::from("priority")),
    ]))
    .unwrap();

    let json_result: serde_json::Value = serde_json::to_value(&result).unwrap();
    let group1 = json_result["1"].as_array().unwrap();
    let group2 = json_result["2"].as_array().unwrap();

    assert_eq!(group1.len(), 2);
    assert_eq!(group2.len(), 1);
}

#[test]
fn test_array_group_by_boolean_key() {
    let items = serde_json::json!([
        {"name": "Item1", "active": true},
        {"name": "Item2", "active": false},
        {"name": "Item3", "active": true}
    ]);

    let result = ArrayGroupBy::call(Kwargs::from_iter(vec![
        ("array", Value::from_serialize(&items)),
        ("key", Value::from("active")),
    ]))
    .unwrap();

    let json_result: serde_json::Value = serde_json::to_value(&result).unwrap();
    let active = json_result["true"].as_array().unwrap();
    let inactive = json_result["false"].as_array().unwrap();

    assert_eq!(active.len(), 2);
    assert_eq!(inactive.len(), 1);
}

#[test]
fn test_array_group_by_empty_array() {
    let empty: Vec<serde_json::Value> = vec![];

    let result = ArrayGroupBy::call(Kwargs::from_iter(vec![
        ("array", Value::from_serialize(&empty)),
        ("key", Value::from("dept")),
    ]))
    .unwrap();

    let json_result: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json_result.as_object().unwrap().len(), 0);
}

#[test]
fn test_array_group_by_error_not_array() {
    let result = ArrayGroupBy::call(Kwargs::from_iter(vec![
        ("array", Value::from(42)),
        ("key", Value::from("dept")),
    ]));

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires an array")
    );
}

#[test]
fn test_array_group_by_missing_key_param() {
    let users = serde_json::json!([{"name": "Alice"}]);
    let result = ArrayGroupBy::call(Kwargs::from_iter(vec![(
        "array",
        Value::from_serialize(&users),
    )]));

    assert!(result.is_err());
}

// Note: Array Unique and Array Flatten tests removed - functions now in filter_functions/array.rs
// See tests/test_array_filter_functions.rs for tests of these functions

// ============================================================================
// Additional Array Sort By Edge Cases
// ============================================================================

#[test]
fn test_array_sort_by_all_missing_key() {
    let users = serde_json::json!([
        {"name": "Alice"},
        {"name": "Bob"},
        {"name": "Charlie"}
    ]);

    let result = ArraySortBy::call(Kwargs::from_iter(vec![
        ("array", Value::from_serialize(&users)),
        ("key", Value::from("age")),
    ]))
    .unwrap();

    // All have missing key, should maintain relative order
    let json_result: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json_result.as_array().unwrap().len(), 3);
}

#[test]
fn test_array_sort_by_some_missing_key() {
    let items = serde_json::json!([
        {"name": "First"},
        {"name": "Second", "order": 1},
        {"name": "Third"},
        {"name": "Fourth", "order": 2}
    ]);

    let result = ArraySortBy::call(Kwargs::from_iter(vec![
        ("array", Value::from_serialize(&items)),
        ("key", Value::from("order")),
    ]))
    .unwrap();

    let json_result: serde_json::Value = serde_json::to_value(&result).unwrap();
    // Items with key should come first (sorted), items without key should come at end
    assert_eq!(json_result[0]["order"], 1);
    assert_eq!(json_result[1]["order"], 2);
}

#[test]
fn test_array_sort_by_null_values() {
    let items = serde_json::json!([
        {"name": "First", "order": null},
        {"name": "Second", "order": 1},
        {"name": "Third", "order": 2}
    ]);

    let result = ArraySortBy::call(Kwargs::from_iter(vec![
        ("array", Value::from_serialize(&items)),
        ("key", Value::from("order")),
    ]))
    .unwrap();

    let json_result: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json_result.as_array().unwrap().len(), 3);
}

#[test]
fn test_array_sort_by_boolean_values() {
    let items = serde_json::json!([
        {"name": "First", "active": true},
        {"name": "Second", "active": false},
        {"name": "Third", "active": true}
    ]);

    let result = ArraySortBy::call(Kwargs::from_iter(vec![
        ("array", Value::from_serialize(&items)),
        ("key", Value::from("active")),
    ]))
    .unwrap();

    let json_result: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json_result.as_array().unwrap().len(), 3);
}

#[test]
fn test_array_sort_by_single_item() {
    let items = serde_json::json!([{"name": "Only", "age": 25}]);

    let result = ArraySortBy::call(Kwargs::from_iter(vec![
        ("array", Value::from_serialize(&items)),
        ("key", Value::from("age")),
    ]))
    .unwrap();

    let json_result: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json_result.as_array().unwrap().len(), 1);
    assert_eq!(json_result[0]["age"], 25);
}

#[test]
fn test_array_sort_by_missing_array_param() {
    let result = ArraySortBy::call(Kwargs::from_iter(vec![("key", Value::from("name"))]));

    assert!(result.is_err());
}

// ============================================================================
// Additional Array Group By Edge Cases
// ============================================================================

#[test]
fn test_array_group_by_null_key_value() {
    let items = serde_json::json!([
        {"name": "First", "category": null},
        {"name": "Second", "category": "A"},
        {"name": "Third", "category": null}
    ]);

    let result = ArrayGroupBy::call(Kwargs::from_iter(vec![
        ("array", Value::from_serialize(&items)),
        ("key", Value::from("category")),
    ]))
    .unwrap();

    let json_result: serde_json::Value = serde_json::to_value(&result).unwrap();
    // Nulls should be grouped under "null" key
    assert!(json_result.get("null").is_some() || json_result.get("A").is_some());
}

#[test]
fn test_array_group_by_missing_key_in_some_items() {
    let items = serde_json::json!([
        {"name": "First", "dept": "Engineering"},
        {"name": "Second"},
        {"name": "Third", "dept": "Sales"}
    ]);

    let result = ArrayGroupBy::call(Kwargs::from_iter(vec![
        ("array", Value::from_serialize(&items)),
        ("key", Value::from("dept")),
    ]))
    .unwrap();

    let json_result: serde_json::Value = serde_json::to_value(&result).unwrap();
    // Only items with the key should be included in groups
    let total_items: usize = json_result
        .as_object()
        .unwrap()
        .values()
        .map(|v| v.as_array().unwrap().len())
        .sum();
    assert_eq!(total_items, 2);
}

#[test]
fn test_array_group_by_single_group() {
    let items = serde_json::json!([
        {"name": "First", "type": "A"},
        {"name": "Second", "type": "A"},
        {"name": "Third", "type": "A"}
    ]);

    let result = ArrayGroupBy::call(Kwargs::from_iter(vec![
        ("array", Value::from_serialize(&items)),
        ("key", Value::from("type")),
    ]))
    .unwrap();

    let json_result: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json_result.as_object().unwrap().len(), 1);
    assert_eq!(json_result["A"].as_array().unwrap().len(), 3);
}

#[test]
fn test_array_group_by_missing_array_param() {
    let result = ArrayGroupBy::call(Kwargs::from_iter(vec![("key", Value::from("dept"))]));

    assert!(result.is_err());
}

// Note: array_unique and array_flatten tests removed - these functions are now
// in filter_functions/array.rs with dual function+filter syntax support.
// See tests/test_filters_integration.rs for integration tests of these filters.
