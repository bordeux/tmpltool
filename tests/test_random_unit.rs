use minijinja::value::Kwargs;
use tmpltool::functions::random::get_random_fn;

// Helper to create kwargs for testing
fn create_kwargs_i64(args: Vec<(&str, i64)>) -> Kwargs {
    Kwargs::from_iter(args.into_iter().map(|(k, v)| (k, minijinja::Value::from(v))))
}

#[test]
fn test_get_random_default_range() {
    let kwargs: Kwargs = Kwargs::from_iter(Vec::<(&str, minijinja::Value)>::new());
    let result = get_random_fn(kwargs).unwrap();
    let value = result.as_i64().unwrap();

    // Default range is 0-100 (exclusive)
    assert!(value >= 0 && value < 100, "Value should be in [0, 100): {}", value);
}

#[test]
fn test_get_random_custom_range() {
    let kwargs = create_kwargs_i64(vec![("start", 10), ("end", 20)]);
    let result = get_random_fn(kwargs).unwrap();
    let value = result.as_i64().unwrap();

    assert!(value >= 10 && value < 20, "Value should be in [10, 20): {}", value);
}

#[test]
fn test_get_random_single_value_range() {
    // Range [5, 6) should only return 5
    let kwargs = create_kwargs_i64(vec![("start", 5), ("end", 6)]);
    let result = get_random_fn(kwargs).unwrap();
    let value = result.as_i64().unwrap();

    assert_eq!(value, 5, "Single value range should return start value");
}

#[test]
fn test_get_random_negative_range() {
    let kwargs = create_kwargs_i64(vec![("start", -10), ("end", 0)]);
    let result = get_random_fn(kwargs).unwrap();
    let value = result.as_i64().unwrap();

    assert!(value >= -10 && value < 0, "Value should be in [-10, 0): {}", value);
}

#[test]
fn test_get_random_crossing_zero() {
    let kwargs = create_kwargs_i64(vec![("start", -5), ("end", 5)]);
    let result = get_random_fn(kwargs).unwrap();
    let value = result.as_i64().unwrap();

    assert!(value >= -5 && value < 5, "Value should be in [-5, 5): {}", value);
}

#[test]
fn test_get_random_large_range() {
    let kwargs = create_kwargs_i64(vec![("start", 0), ("end", 1000000)]);
    let result = get_random_fn(kwargs).unwrap();
    let value = result.as_i64().unwrap();

    assert!(
        value >= 0 && value < 1000000,
        "Value should be in [0, 1000000): {}",
        value
    );
}

#[test]
fn test_get_random_invalid_range_equal() {
    let kwargs = create_kwargs_i64(vec![("start", 10), ("end", 10)]);
    let result = get_random_fn(kwargs);

    assert!(result.is_err());
    assert!(result
        .err()
        .unwrap()
        .to_string()
        .contains("must be less than end"));
}

#[test]
fn test_get_random_invalid_range_reversed() {
    let kwargs = create_kwargs_i64(vec![("start", 20), ("end", 10)]);
    let result = get_random_fn(kwargs);

    assert!(result.is_err());
    assert!(result
        .err()
        .unwrap()
        .to_string()
        .contains("must be less than end"));
}

#[test]
fn test_get_random_only_start() {
    let kwargs = create_kwargs_i64(vec![("start", 50)]);
    let result = get_random_fn(kwargs).unwrap();
    let value = result.as_i64().unwrap();

    // Should use default end of 100
    assert!(value >= 50 && value < 100, "Value should be in [50, 100): {}", value);
}

#[test]
fn test_get_random_only_end() {
    let kwargs = create_kwargs_i64(vec![("end", 50)]);
    let result = get_random_fn(kwargs).unwrap();
    let value = result.as_i64().unwrap();

    // Should use default start of 0
    assert!(value >= 0 && value < 50, "Value should be in [0, 50): {}", value);
}

#[test]
fn test_get_random_distribution() {
    // Test that random values are somewhat distributed (not all the same)
    let kwargs = create_kwargs_i64(vec![("start", 0), ("end", 10)]);
    let mut values = std::collections::HashSet::new();

    for _ in 0..50 {
        let result = get_random_fn(kwargs.clone()).unwrap();
        values.insert(result.as_i64().unwrap());
    }

    // With 50 random values in [0, 10), we should get at least 5 unique values
    assert!(
        values.len() >= 5,
        "Should have reasonable distribution, got {} unique values out of 50",
        values.len()
    );
}

#[test]
fn test_get_random_always_in_range() {
    // Run many times to ensure it always stays in range
    let kwargs = create_kwargs_i64(vec![("start", 1), ("end", 10)]);

    for _ in 0..100 {
        let result = get_random_fn(kwargs.clone()).unwrap();
        let value = result.as_i64().unwrap();
        assert!(
            value >= 1 && value < 10,
            "Value should always be in [1, 10): {}",
            value
        );
    }
}
