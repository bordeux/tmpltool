use minijinja::Value;
use minijinja::value::Kwargs;
use tmpltool::functions::datetime::now_fn;

fn empty_kwargs() -> Kwargs {
    Kwargs::from_iter(Vec::<(&str, Value)>::new())
}

fn kwargs_with_format(format: &str) -> Kwargs {
    Kwargs::from_iter(vec![("format", Value::from(format))])
}

// ==================== now() returns Unix timestamp ====================

#[test]
fn test_now_fn_returns_timestamp() {
    let result = now_fn(empty_kwargs()).unwrap();
    let timestamp = result.as_i64().unwrap();

    // Should be a reasonable Unix timestamp (after 2020-01-01)
    assert!(
        timestamp > 1577836800,
        "Timestamp should be after 2020-01-01"
    );
    // Should be before 2100-01-01
    assert!(
        timestamp < 4102444800,
        "Timestamp should be before 2100-01-01"
    );
}

#[test]
fn test_now_fn_returns_integer() {
    let result = now_fn(empty_kwargs()).unwrap();

    // Should be an integer, not a string
    assert!(result.as_i64().is_some(), "now() should return an integer");
    assert!(
        result.as_str().is_none(),
        "now() should not return a string"
    );
}

#[test]
fn test_now_fn_monotonic() {
    let result1 = now_fn(empty_kwargs()).unwrap();
    let ts1 = result1.as_i64().unwrap();

    std::thread::sleep(std::time::Duration::from_secs(1));

    let result2 = now_fn(empty_kwargs()).unwrap();
    let ts2 = result2.as_i64().unwrap();

    assert!(
        ts2 >= ts1,
        "Timestamps should be monotonic: {} >= {}",
        ts2,
        ts1
    );
}

// ==================== now() with format parameter ====================

#[test]
fn test_now_fn_with_format_date_only() {
    let result = now_fn(kwargs_with_format("%Y-%m-%d")).unwrap();
    let formatted = result.as_str().unwrap();

    // Should match YYYY-MM-DD pattern
    assert_eq!(formatted.len(), 10, "Date format should be 10 chars");
    assert!(formatted.contains('-'), "Should contain date separators");
    assert!(
        !formatted.contains('T'),
        "Should not contain time separator"
    );
    assert!(
        !formatted.contains(':'),
        "Should not contain time separators"
    );
}

#[test]
fn test_now_fn_with_format_datetime() {
    let result = now_fn(kwargs_with_format("%Y-%m-%d %H:%M:%S")).unwrap();
    let formatted = result.as_str().unwrap();

    // Should match YYYY-MM-DD HH:MM:SS pattern (19 chars)
    assert_eq!(formatted.len(), 19, "DateTime format should be 19 chars");
    assert!(
        formatted.contains(' '),
        "Should contain space between date and time"
    );
    assert!(formatted.contains(':'), "Should contain time separators");
}

#[test]
fn test_now_fn_with_format_time_only() {
    let result = now_fn(kwargs_with_format("%H:%M:%S")).unwrap();
    let formatted = result.as_str().unwrap();

    // Should match HH:MM:SS pattern (8 chars)
    assert_eq!(formatted.len(), 8, "Time format should be 8 chars");
    assert!(formatted.contains(':'), "Should contain time separators");
}

#[test]
fn test_now_fn_with_format_year_only() {
    let result = now_fn(kwargs_with_format("%Y")).unwrap();
    let formatted = result.as_str().unwrap();

    assert_eq!(formatted.len(), 4, "Year should be 4 digits");
    let year: i32 = formatted.parse().unwrap();
    assert!(
        (2020..=2100).contains(&year),
        "Year should be reasonable: {}",
        year
    );
}

#[test]
fn test_now_fn_with_format_returns_string() {
    let result = now_fn(kwargs_with_format("%Y-%m-%d")).unwrap();

    // With format, should return a string
    assert!(
        result.as_str().is_some(),
        "now(format=...) should return a string"
    );
}

#[test]
fn test_now_fn_with_custom_format() {
    let result = now_fn(kwargs_with_format("%d/%m/%Y")).unwrap();
    let formatted = result.as_str().unwrap();

    // Should match DD/MM/YYYY pattern
    assert_eq!(formatted.len(), 10, "Custom date format should be 10 chars");
    assert!(formatted.contains('/'), "Should contain custom separators");
}
