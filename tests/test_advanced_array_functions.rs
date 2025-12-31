use minijinja::Value;
use minijinja::value::Kwargs;
use tmpltool::functions::array;

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

    let result = array::array_sort_by_fn(Kwargs::from_iter(vec![
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

    let result = array::array_sort_by_fn(Kwargs::from_iter(vec![
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

    let result = array::array_sort_by_fn(Kwargs::from_iter(vec![
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

    let result = array::array_sort_by_fn(Kwargs::from_iter(vec![
        ("array", Value::from_serialize(&empty)),
        ("key", Value::from("age")),
    ]))
    .unwrap();

    let json_result: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json_result.as_array().unwrap().len(), 0);
}

#[test]
fn test_array_sort_by_error_not_array() {
    let result = array::array_sort_by_fn(Kwargs::from_iter(vec![
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
    let result = array::array_sort_by_fn(Kwargs::from_iter(vec![(
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

    let result = array::array_group_by_fn(Kwargs::from_iter(vec![
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

    let result = array::array_group_by_fn(Kwargs::from_iter(vec![
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

    let result = array::array_group_by_fn(Kwargs::from_iter(vec![
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

    let result = array::array_group_by_fn(Kwargs::from_iter(vec![
        ("array", Value::from_serialize(&empty)),
        ("key", Value::from("dept")),
    ]))
    .unwrap();

    let json_result: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json_result.as_object().unwrap().len(), 0);
}

#[test]
fn test_array_group_by_error_not_array() {
    let result = array::array_group_by_fn(Kwargs::from_iter(vec![
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
    let result = array::array_group_by_fn(Kwargs::from_iter(vec![(
        "array",
        Value::from_serialize(&users),
    )]));

    assert!(result.is_err());
}

// ============================================================================
// Array Unique Tests
// ============================================================================

#[test]
fn test_array_unique_numbers() {
    let result = array::array_unique_fn(Kwargs::from_iter(vec![(
        "array",
        Value::from(vec![1, 2, 2, 3, 1, 4, 3, 5]),
    )]))
    .unwrap();

    let json_result: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json_result.as_array().unwrap().len(), 5);
    assert_eq!(json_result[0], 1);
    assert_eq!(json_result[1], 2);
    assert_eq!(json_result[2], 3);
    assert_eq!(json_result[3], 4);
    assert_eq!(json_result[4], 5);
}

#[test]
fn test_array_unique_strings() {
    let result = array::array_unique_fn(Kwargs::from_iter(vec![(
        "array",
        Value::from(vec!["docker", "kubernetes", "docker", "helm"]),
    )]))
    .unwrap();

    let json_result: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json_result.as_array().unwrap().len(), 3);
    assert_eq!(json_result[0], "docker");
    assert_eq!(json_result[1], "kubernetes");
    assert_eq!(json_result[2], "helm");
}

#[test]
fn test_array_unique_empty_array() {
    let empty: Vec<i32> = vec![];
    let result =
        array::array_unique_fn(Kwargs::from_iter(vec![("array", Value::from(empty))])).unwrap();

    let json_result: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json_result.as_array().unwrap().len(), 0);
}

#[test]
fn test_array_unique_all_unique() {
    let result = array::array_unique_fn(Kwargs::from_iter(vec![(
        "array",
        Value::from(vec![1, 2, 3, 4, 5]),
    )]))
    .unwrap();

    let json_result: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json_result.as_array().unwrap().len(), 5);
}

#[test]
fn test_array_unique_all_duplicates() {
    let result = array::array_unique_fn(Kwargs::from_iter(vec![(
        "array",
        Value::from(vec![5, 5, 5, 5]),
    )]))
    .unwrap();

    let json_result: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json_result.as_array().unwrap().len(), 1);
    assert_eq!(json_result[0], 5);
}

#[test]
fn test_array_unique_mixed_types() {
    let mixed = serde_json::json!([1, "test", 1, "test", 2]);

    let result = array::array_unique_fn(Kwargs::from_iter(vec![(
        "array",
        Value::from_serialize(&mixed),
    )]))
    .unwrap();

    let json_result: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json_result.as_array().unwrap().len(), 3);
}

#[test]
fn test_array_unique_error_not_array() {
    let result = array::array_unique_fn(Kwargs::from_iter(vec![("array", Value::from("test"))]));

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires an array")
    );
}

#[test]
fn test_array_unique_missing_array() {
    let result = array::array_unique_fn(Kwargs::from_iter(vec![("dummy", Value::from(0))]));

    assert!(result.is_err());
}

// ============================================================================
// Array Flatten Tests
// ============================================================================

#[test]
fn test_array_flatten_basic() {
    let nested = serde_json::json!([[1, 2], [3, 4], [5]]);

    let result = array::array_flatten_fn(Kwargs::from_iter(vec![(
        "array",
        Value::from_serialize(&nested),
    )]))
    .unwrap();

    let json_result: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json_result.as_array().unwrap().len(), 5);
    assert_eq!(json_result[0], 1);
    assert_eq!(json_result[1], 2);
    assert_eq!(json_result[2], 3);
    assert_eq!(json_result[3], 4);
    assert_eq!(json_result[4], 5);
}

#[test]
fn test_array_flatten_strings() {
    let nested = serde_json::json!([["a", "b"], ["c"], ["d", "e"]]);

    let result = array::array_flatten_fn(Kwargs::from_iter(vec![(
        "array",
        Value::from_serialize(&nested),
    )]))
    .unwrap();

    let json_result: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json_result.as_array().unwrap().len(), 5);
    assert_eq!(json_result[0], "a");
    assert_eq!(json_result[1], "b");
    assert_eq!(json_result[2], "c");
    assert_eq!(json_result[3], "d");
    assert_eq!(json_result[4], "e");
}

#[test]
fn test_array_flatten_one_level_only() {
    let deep = serde_json::json!([[1, [2, 3]], [4]]);

    let result = array::array_flatten_fn(Kwargs::from_iter(vec![(
        "array",
        Value::from_serialize(&deep),
    )]))
    .unwrap();

    let json_result: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json_result.as_array().unwrap().len(), 3);
    assert_eq!(json_result[0], 1);
    assert!(json_result[1].is_array());
    assert_eq!(json_result[2], 4);
}

#[test]
fn test_array_flatten_mixed() {
    let mixed = serde_json::json!([[1, 2], 3, [4, 5]]);

    let result = array::array_flatten_fn(Kwargs::from_iter(vec![(
        "array",
        Value::from_serialize(&mixed),
    )]))
    .unwrap();

    let json_result: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json_result.as_array().unwrap().len(), 5);
    assert_eq!(json_result[0], 1);
    assert_eq!(json_result[1], 2);
    assert_eq!(json_result[2], 3);
    assert_eq!(json_result[3], 4);
    assert_eq!(json_result[4], 5);
}

#[test]
fn test_array_flatten_empty_array() {
    let empty: Vec<Vec<i32>> = vec![];

    let result = array::array_flatten_fn(Kwargs::from_iter(vec![(
        "array",
        Value::from_serialize(&empty),
    )]))
    .unwrap();

    let json_result: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json_result.as_array().unwrap().len(), 0);
}

#[test]
fn test_array_flatten_empty_nested() {
    let nested = serde_json::json!([[], [1, 2], []]);

    let result = array::array_flatten_fn(Kwargs::from_iter(vec![(
        "array",
        Value::from_serialize(&nested),
    )]))
    .unwrap();

    let json_result: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json_result.as_array().unwrap().len(), 2);
    assert_eq!(json_result[0], 1);
    assert_eq!(json_result[1], 2);
}

#[test]
fn test_array_flatten_single_nested() {
    let nested = serde_json::json!([[1, 2, 3]]);

    let result = array::array_flatten_fn(Kwargs::from_iter(vec![(
        "array",
        Value::from_serialize(&nested),
    )]))
    .unwrap();

    let json_result: serde_json::Value = serde_json::to_value(&result).unwrap();
    assert_eq!(json_result.as_array().unwrap().len(), 3);
}

#[test]
fn test_array_flatten_error_not_array() {
    let result = array::array_flatten_fn(Kwargs::from_iter(vec![("array", Value::from(123))]));

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires an array")
    );
}

#[test]
fn test_array_flatten_missing_array() {
    let result = array::array_flatten_fn(Kwargs::from_iter(vec![("dummy", Value::from(0))]));

    assert!(result.is_err());
}
