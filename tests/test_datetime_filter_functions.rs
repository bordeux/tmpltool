//! Tests for datetime filter-functions.
//!
//! Tests both function and filter syntax for:
//! - format_date, get_year, get_month, get_day, get_hour, get_minute, get_second

use minijinja::Value;
use minijinja::value::Kwargs;
use tmpltool::filter_functions::FilterFunction;
use tmpltool::filter_functions::datetime::{
    FormatDate, GetDay, GetHour, GetMinute, GetMonth, GetSecond, GetYear,
};

/// Helper to create empty kwargs
fn empty_kwargs() -> Kwargs {
    Kwargs::from_iter(Vec::<(&str, Value)>::new())
}

// Known timestamp: 2024-01-01 00:00:00 UTC
const TEST_TIMESTAMP: i64 = 1704067200;

// ============================================
// FormatDate tests
// ============================================

#[test]
fn test_format_date_filter_syntax() {
    let kwargs = Kwargs::from_iter(vec![("format", Value::from("%Y-%m-%d"))]);
    let result = FormatDate::call_as_filter(&Value::from(TEST_TIMESTAMP), kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "2024-01-01");
}

#[test]
fn test_format_date_function_syntax() {
    let kwargs = Kwargs::from_iter(vec![
        ("timestamp", Value::from(TEST_TIMESTAMP)),
        ("format", Value::from("%Y-%m-%d")),
    ]);
    let result = FormatDate::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "2024-01-01");
}

#[test]
fn test_format_date_default_format() {
    let result = FormatDate::call_as_filter(&Value::from(TEST_TIMESTAMP), empty_kwargs()).unwrap();
    // Default format: %Y-%m-%d %H:%M:%S
    assert!(result.as_str().unwrap().contains("2024-01-01"));
}

#[test]
fn test_format_date_custom_format() {
    let kwargs = Kwargs::from_iter(vec![("format", Value::from("%B %d, %Y"))]);
    let result = FormatDate::call_as_filter(&Value::from(TEST_TIMESTAMP), kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "January 01, 2024");
}

#[test]
fn test_format_date_error_not_number() {
    let result = FormatDate::call_as_filter(&Value::from("not a number"), empty_kwargs());
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires a numeric timestamp")
    );
}

// ============================================
// GetYear tests
// ============================================

#[test]
fn test_get_year_filter_syntax() {
    let result = GetYear::call_as_filter(&Value::from(TEST_TIMESTAMP), empty_kwargs()).unwrap();
    assert_eq!(result.as_i64().unwrap(), 2024);
}

#[test]
fn test_get_year_function_syntax() {
    let kwargs = Kwargs::from_iter(vec![("timestamp", Value::from(TEST_TIMESTAMP))]);
    let result = GetYear::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_i64().unwrap(), 2024);
}

#[test]
fn test_get_year_different_year() {
    // 2020-06-15 12:00:00 UTC
    let ts_2020 = 1592222400;
    let result = GetYear::call_as_filter(&Value::from(ts_2020), empty_kwargs()).unwrap();
    assert_eq!(result.as_i64().unwrap(), 2020);
}

#[test]
fn test_get_year_error_not_number() {
    let result = GetYear::call_as_filter(&Value::from("not a number"), empty_kwargs());
    assert!(result.is_err());
}

// ============================================
// GetMonth tests
// ============================================

#[test]
fn test_get_month_filter_syntax() {
    let result = GetMonth::call_as_filter(&Value::from(TEST_TIMESTAMP), empty_kwargs()).unwrap();
    assert_eq!(result.as_i64().unwrap(), 1); // January
}

#[test]
fn test_get_month_function_syntax() {
    let kwargs = Kwargs::from_iter(vec![("timestamp", Value::from(TEST_TIMESTAMP))]);
    let result = GetMonth::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_i64().unwrap(), 1);
}

#[test]
fn test_get_month_december() {
    // 2023-12-15 12:00:00 UTC
    let ts_dec = 1702641600;
    let result = GetMonth::call_as_filter(&Value::from(ts_dec), empty_kwargs()).unwrap();
    assert_eq!(result.as_i64().unwrap(), 12);
}

#[test]
fn test_get_month_error_not_number() {
    let result = GetMonth::call_as_filter(&Value::from("not a number"), empty_kwargs());
    assert!(result.is_err());
}

// ============================================
// GetDay tests
// ============================================

#[test]
fn test_get_day_filter_syntax() {
    let result = GetDay::call_as_filter(&Value::from(TEST_TIMESTAMP), empty_kwargs()).unwrap();
    assert_eq!(result.as_i64().unwrap(), 1);
}

#[test]
fn test_get_day_function_syntax() {
    let kwargs = Kwargs::from_iter(vec![("timestamp", Value::from(TEST_TIMESTAMP))]);
    let result = GetDay::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_i64().unwrap(), 1);
}

#[test]
fn test_get_day_15th() {
    // 2024-01-15 12:00:00 UTC
    let ts_15 = 1705320000;
    let result = GetDay::call_as_filter(&Value::from(ts_15), empty_kwargs()).unwrap();
    assert_eq!(result.as_i64().unwrap(), 15);
}

#[test]
fn test_get_day_error_not_number() {
    let result = GetDay::call_as_filter(&Value::from("not a number"), empty_kwargs());
    assert!(result.is_err());
}

// ============================================
// GetHour tests
// ============================================

#[test]
fn test_get_hour_filter_syntax() {
    let result = GetHour::call_as_filter(&Value::from(TEST_TIMESTAMP), empty_kwargs()).unwrap();
    assert_eq!(result.as_i64().unwrap(), 0); // Midnight UTC
}

#[test]
fn test_get_hour_function_syntax() {
    let kwargs = Kwargs::from_iter(vec![("timestamp", Value::from(TEST_TIMESTAMP))]);
    let result = GetHour::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_i64().unwrap(), 0);
}

#[test]
fn test_get_hour_noon() {
    // 2024-01-01 12:00:00 UTC
    let ts_noon = 1704110400;
    let result = GetHour::call_as_filter(&Value::from(ts_noon), empty_kwargs()).unwrap();
    assert_eq!(result.as_i64().unwrap(), 12);
}

#[test]
fn test_get_hour_error_not_number() {
    let result = GetHour::call_as_filter(&Value::from("not a number"), empty_kwargs());
    assert!(result.is_err());
}

// ============================================
// GetMinute tests
// ============================================

#[test]
fn test_get_minute_filter_syntax() {
    let result = GetMinute::call_as_filter(&Value::from(TEST_TIMESTAMP), empty_kwargs()).unwrap();
    assert_eq!(result.as_i64().unwrap(), 0);
}

#[test]
fn test_get_minute_function_syntax() {
    let kwargs = Kwargs::from_iter(vec![("timestamp", Value::from(TEST_TIMESTAMP))]);
    let result = GetMinute::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_i64().unwrap(), 0);
}

#[test]
fn test_get_minute_30() {
    // 2024-01-01 00:30:00 UTC
    let ts_30 = TEST_TIMESTAMP + 30 * 60;
    let result = GetMinute::call_as_filter(&Value::from(ts_30), empty_kwargs()).unwrap();
    assert_eq!(result.as_i64().unwrap(), 30);
}

#[test]
fn test_get_minute_error_not_number() {
    let result = GetMinute::call_as_filter(&Value::from("not a number"), empty_kwargs());
    assert!(result.is_err());
}

// ============================================
// GetSecond tests
// ============================================

#[test]
fn test_get_second_filter_syntax() {
    let result = GetSecond::call_as_filter(&Value::from(TEST_TIMESTAMP), empty_kwargs()).unwrap();
    assert_eq!(result.as_i64().unwrap(), 0);
}

#[test]
fn test_get_second_function_syntax() {
    let kwargs = Kwargs::from_iter(vec![("timestamp", Value::from(TEST_TIMESTAMP))]);
    let result = GetSecond::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_i64().unwrap(), 0);
}

#[test]
fn test_get_second_45() {
    // 2024-01-01 00:00:45 UTC
    let ts_45 = TEST_TIMESTAMP + 45;
    let result = GetSecond::call_as_filter(&Value::from(ts_45), empty_kwargs()).unwrap();
    assert_eq!(result.as_i64().unwrap(), 45);
}

#[test]
fn test_get_second_error_not_number() {
    let result = GetSecond::call_as_filter(&Value::from("not a number"), empty_kwargs());
    assert!(result.is_err());
}
