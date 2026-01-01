use minijinja::Value;
use minijinja::value::Kwargs;
use tmpltool::functions::array;

// ============================================================================
// Array Count Tests
// ============================================================================

#[test]
fn test_array_count_basic() {
    let result = array::array_count_fn(Kwargs::from_iter(vec![(
        "array",
        Value::from(vec!["apple", "banana", "cherry"]),
    )]))
    .unwrap();

    assert_eq!(result, Value::from(3));
}

#[test]
fn test_array_count_empty() {
    let empty: Vec<i32> = vec![];
    let result =
        array::array_count_fn(Kwargs::from_iter(vec![("array", Value::from(empty))])).unwrap();

    assert_eq!(result, Value::from(0));
}

#[test]
fn test_array_count_single() {
    let result =
        array::array_count_fn(Kwargs::from_iter(vec![("array", Value::from(vec![42]))])).unwrap();

    assert_eq!(result, Value::from(1));
}

#[test]
fn test_array_count_large() {
    let large: Vec<i32> = (1..=100).collect();
    let result =
        array::array_count_fn(Kwargs::from_iter(vec![("array", Value::from(large))])).unwrap();

    assert_eq!(result, Value::from(100));
}

#[test]
fn test_array_count_error_not_array() {
    let result = array::array_count_fn(Kwargs::from_iter(vec![("array", Value::from("test"))]));

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires an array")
    );
}

#[test]
fn test_array_count_missing_array() {
    let result = array::array_count_fn(Kwargs::from_iter(vec![("dummy", Value::from(0))]));

    assert!(result.is_err());
}

// ============================================================================
// Array Chunk Tests
// ============================================================================

#[test]
fn test_array_chunk_even_division() {
    let result = array::array_chunk_fn(Kwargs::from_iter(vec![
        ("array", Value::from(vec![1, 2, 3, 4, 5, 6])),
        ("size", Value::from(2)),
    ]))
    .unwrap();

    let expected = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    assert_eq!(
        result.to_string(),
        Value::from_serialize(&expected).to_string()
    );
}

#[test]
fn test_array_chunk_uneven_division() {
    let result = array::array_chunk_fn(Kwargs::from_iter(vec![
        ("array", Value::from(vec![1, 2, 3, 4, 5])),
        ("size", Value::from(2)),
    ]))
    .unwrap();

    let expected = vec![vec![1, 2], vec![3, 4], vec![5]];
    assert_eq!(
        result.to_string(),
        Value::from_serialize(&expected).to_string()
    );
}

#[test]
fn test_array_chunk_size_one() {
    let result = array::array_chunk_fn(Kwargs::from_iter(vec![
        ("array", Value::from(vec![1, 2, 3])),
        ("size", Value::from(1)),
    ]))
    .unwrap();

    let expected = vec![vec![1], vec![2], vec![3]];
    assert_eq!(
        result.to_string(),
        Value::from_serialize(&expected).to_string()
    );
}

#[test]
fn test_array_chunk_size_larger_than_array() {
    let result = array::array_chunk_fn(Kwargs::from_iter(vec![
        ("array", Value::from(vec![1, 2, 3])),
        ("size", Value::from(10)),
    ]))
    .unwrap();

    let expected = vec![vec![1, 2, 3]];
    assert_eq!(
        result.to_string(),
        Value::from_serialize(&expected).to_string()
    );
}

#[test]
fn test_array_chunk_empty_array() {
    let empty: Vec<i32> = vec![];
    let result = array::array_chunk_fn(Kwargs::from_iter(vec![
        ("array", Value::from(empty)),
        ("size", Value::from(2)),
    ]))
    .unwrap();

    let expected: Vec<Vec<i32>> = vec![];
    assert_eq!(
        result.to_string(),
        Value::from_serialize(&expected).to_string()
    );
}

#[test]
fn test_array_chunk_strings() {
    let result = array::array_chunk_fn(Kwargs::from_iter(vec![
        ("array", Value::from(vec!["a", "b", "c", "d", "e", "f"])),
        ("size", Value::from(3)),
    ]))
    .unwrap();

    let expected = vec![vec!["a", "b", "c"], vec!["d", "e", "f"]];
    assert_eq!(
        result.to_string(),
        Value::from_serialize(&expected).to_string()
    );
}

#[test]
fn test_array_chunk_error_zero_size() {
    let result = array::array_chunk_fn(Kwargs::from_iter(vec![
        ("array", Value::from(vec![1, 2, 3])),
        ("size", Value::from(0)),
    ]));

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("greater than 0"));
}

#[test]
fn test_array_chunk_error_not_array() {
    let result = array::array_chunk_fn(Kwargs::from_iter(vec![
        ("array", Value::from("test")),
        ("size", Value::from(2)),
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
fn test_array_chunk_missing_array() {
    let result = array::array_chunk_fn(Kwargs::from_iter(vec![("size", Value::from(2))]));

    assert!(result.is_err());
}

#[test]
fn test_array_chunk_missing_size() {
    let result = array::array_chunk_fn(Kwargs::from_iter(vec![(
        "array",
        Value::from(vec![1, 2, 3]),
    )]));

    assert!(result.is_err());
}

// ============================================================================
// Array Zip Tests
// ============================================================================

#[test]
fn test_array_zip_equal_length() {
    let result = array::array_zip_fn(Kwargs::from_iter(vec![
        ("array1", Value::from(vec![1, 2, 3])),
        ("array2", Value::from(vec!["a", "b", "c"])),
    ]))
    .unwrap();

    let expected = vec![
        vec![Value::from(1), Value::from("a")],
        vec![Value::from(2), Value::from("b")],
        vec![Value::from(3), Value::from("c")],
    ];
    assert_eq!(
        result.to_string(),
        Value::from_serialize(&expected).to_string()
    );
}

#[test]
fn test_array_zip_first_longer() {
    let result = array::array_zip_fn(Kwargs::from_iter(vec![
        ("array1", Value::from(vec![1, 2, 3, 4])),
        ("array2", Value::from(vec!["a", "b"])),
    ]))
    .unwrap();

    let expected = vec![
        vec![Value::from(1), Value::from("a")],
        vec![Value::from(2), Value::from("b")],
    ];
    assert_eq!(
        result.to_string(),
        Value::from_serialize(&expected).to_string()
    );
}

#[test]
fn test_array_zip_second_longer() {
    let result = array::array_zip_fn(Kwargs::from_iter(vec![
        ("array1", Value::from(vec![1, 2])),
        ("array2", Value::from(vec!["a", "b", "c", "d"])),
    ]))
    .unwrap();

    let expected = vec![
        vec![Value::from(1), Value::from("a")],
        vec![Value::from(2), Value::from("b")],
    ];
    assert_eq!(
        result.to_string(),
        Value::from_serialize(&expected).to_string()
    );
}

#[test]
fn test_array_zip_empty_arrays() {
    let empty1: Vec<i32> = vec![];
    let empty2: Vec<String> = vec![];
    let result = array::array_zip_fn(Kwargs::from_iter(vec![
        ("array1", Value::from(empty1)),
        ("array2", Value::from(empty2)),
    ]))
    .unwrap();

    let expected: Vec<Vec<Value>> = vec![];
    assert_eq!(
        result.to_string(),
        Value::from_serialize(&expected).to_string()
    );
}

#[test]
fn test_array_zip_first_empty() {
    let empty: Vec<i32> = vec![];
    let result = array::array_zip_fn(Kwargs::from_iter(vec![
        ("array1", Value::from(empty)),
        ("array2", Value::from(vec!["a", "b", "c"])),
    ]))
    .unwrap();

    let expected: Vec<Vec<Value>> = vec![];
    assert_eq!(
        result.to_string(),
        Value::from_serialize(&expected).to_string()
    );
}

#[test]
fn test_array_zip_second_empty() {
    let empty: Vec<String> = vec![];
    let result = array::array_zip_fn(Kwargs::from_iter(vec![
        ("array1", Value::from(vec![1, 2, 3])),
        ("array2", Value::from(empty)),
    ]))
    .unwrap();

    let expected: Vec<Vec<Value>> = vec![];
    assert_eq!(
        result.to_string(),
        Value::from_serialize(&expected).to_string()
    );
}

#[test]
fn test_array_zip_single_elements() {
    let result = array::array_zip_fn(Kwargs::from_iter(vec![
        ("array1", Value::from(vec![42])),
        ("array2", Value::from(vec!["test"])),
    ]))
    .unwrap();

    let expected = vec![vec![Value::from(42), Value::from("test")]];
    assert_eq!(
        result.to_string(),
        Value::from_serialize(&expected).to_string()
    );
}

#[test]
fn test_array_zip_error_first_not_array() {
    let result = array::array_zip_fn(Kwargs::from_iter(vec![
        ("array1", Value::from("test")),
        ("array2", Value::from(vec![1, 2, 3])),
    ]));

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("array1 to be an array")
    );
}

#[test]
fn test_array_zip_error_second_not_array() {
    let result = array::array_zip_fn(Kwargs::from_iter(vec![
        ("array1", Value::from(vec![1, 2, 3])),
        ("array2", Value::from(42)),
    ]));

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("array2 to be an array")
    );
}

#[test]
fn test_array_zip_missing_array1() {
    let result = array::array_zip_fn(Kwargs::from_iter(vec![(
        "array2",
        Value::from(vec![1, 2, 3]),
    )]));

    assert!(result.is_err());
}

#[test]
fn test_array_zip_missing_array2() {
    let result = array::array_zip_fn(Kwargs::from_iter(vec![(
        "array1",
        Value::from(vec![1, 2, 3]),
    )]));

    assert!(result.is_err());
}
