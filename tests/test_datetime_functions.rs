use minijinja::Environment;
use std::path::PathBuf;
use tmpltool::{TemplateContext, functions};

fn create_env() -> Environment<'static> {
    let mut env = Environment::new();
    let context = TemplateContext::new(PathBuf::from("."), false);
    functions::register_all(&mut env, context);
    env
}

fn render_template(env: &Environment, template: &str) -> Result<String, minijinja::Error> {
    let tmpl = env.template_from_str(template)?;
    tmpl.render(())
}

// Tests for format_date
#[test]
fn test_format_date_default() {
    let env = create_env();
    let result = render_template(&env, "{{ format_date(timestamp=1704067200) }}").unwrap();
    assert_eq!(result, "2024-01-01 00:00:00");
}

#[test]
fn test_format_date_iso() {
    let env = create_env();
    let result = render_template(
        &env,
        "{{ format_date(timestamp=1704067200, format=\"%Y-%m-%d\") }}",
    )
    .unwrap();
    assert_eq!(result, "2024-01-01");
}

#[test]
fn test_format_date_us_format() {
    let env = create_env();
    let result = render_template(
        &env,
        "{{ format_date(timestamp=1704067200, format=\"%m/%d/%Y\") }}",
    )
    .unwrap();
    assert_eq!(result, "01/01/2024");
}

#[test]
fn test_format_date_full_month() {
    let env = create_env();
    let result = render_template(
        &env,
        "{{ format_date(timestamp=1704067200, format=\"%B %d, %Y\") }}",
    )
    .unwrap();
    assert_eq!(result, "January 01, 2024");
}

#[test]
fn test_format_date_with_time() {
    let env = create_env();
    let result = render_template(
        &env,
        "{{ format_date(timestamp=1704110400, format=\"%Y-%m-%d %H:%M:%S\") }}",
    )
    .unwrap();
    assert_eq!(result, "2024-01-01 12:00:00");
}

#[test]
fn test_format_date_12hour() {
    let env = create_env();
    let result = render_template(
        &env,
        "{{ format_date(timestamp=1704110400, format=\"%I:%M %p\") }}",
    )
    .unwrap();
    assert_eq!(result, "12:00 PM");
}

#[test]
fn test_format_date_weekday() {
    let env = create_env();
    let result = render_template(
        &env,
        "{{ format_date(timestamp=1704067200, format=\"%A\") }}",
    )
    .unwrap();
    assert_eq!(result, "Monday");
}

// Tests for parse_date
#[test]
fn test_parse_date_datetime() {
    let env = create_env();
    let result = render_template(
        &env,
        "{{ parse_date(string=\"2024-01-01 12:00:00\", format=\"%Y-%m-%d %H:%M:%S\") }}",
    )
    .unwrap();
    assert_eq!(result, "1704110400");
}

#[test]
fn test_parse_date_date_only() {
    let env = create_env();
    let result = render_template(
        &env,
        "{{ parse_date(string=\"2024-01-01\", format=\"%Y-%m-%d\") }}",
    )
    .unwrap();
    assert_eq!(result, "1704067200");
}

#[test]
fn test_parse_date_us_format() {
    let env = create_env();
    let result = render_template(
        &env,
        "{{ parse_date(string=\"12/25/2024\", format=\"%m/%d/%Y\") }}",
    )
    .unwrap();
    assert_eq!(result, "1735084800");
}

#[test]
fn test_parse_date_invalid_format() {
    let env = create_env();
    let result = render_template(
        &env,
        "{{ parse_date(string=\"invalid\", format=\"%Y-%m-%d\") }}",
    );
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Failed to parse date")
    );
}

#[test]
fn test_parse_date_roundtrip() {
    let env = create_env();
    let result = render_template(
        &env,
        "{% set ts = parse_date(string=\"2024-06-15\", format=\"%Y-%m-%d\") %}{{ format_date(timestamp=ts, format=\"%Y-%m-%d\") }}",
    )
    .unwrap();
    assert_eq!(result, "2024-06-15");
}

// Tests for date_add
#[test]
fn test_date_add_positive() {
    let env = create_env();
    let result = render_template(
        &env,
        "{{ format_date(timestamp=date_add(timestamp=1704067200, days=7), format=\"%Y-%m-%d\") }}",
    )
    .unwrap();
    assert_eq!(result, "2024-01-08");
}

#[test]
fn test_date_add_negative() {
    let env = create_env();
    let result = render_template(
        &env,
        "{{ format_date(timestamp=date_add(timestamp=1704067200, days=-7), format=\"%Y-%m-%d\") }}",
    )
    .unwrap();
    assert_eq!(result, "2023-12-25");
}

#[test]
fn test_date_add_zero() {
    let env = create_env();
    let result = render_template(
        &env,
        "{{ format_date(timestamp=date_add(timestamp=1704067200, days=0), format=\"%Y-%m-%d\") }}",
    )
    .unwrap();
    assert_eq!(result, "2024-01-01");
}

#[test]
fn test_date_add_large() {
    let env = create_env();
    let result = render_template(
        &env,
        "{{ format_date(timestamp=date_add(timestamp=1704067200, days=365), format=\"%Y-%m-%d\") }}",
    )
    .unwrap();
    assert_eq!(result, "2024-12-31"); // 2024 is a leap year
}

// Tests for date_diff
#[test]
fn test_date_diff_same() {
    let env = create_env();
    let result = render_template(
        &env,
        "{{ date_diff(timestamp1=1704067200, timestamp2=1704067200) }}",
    )
    .unwrap();
    assert_eq!(result, "0");
}

#[test]
fn test_date_diff_positive() {
    let env = create_env();
    // Jan 2 - Jan 1 = 1 day
    let result = render_template(
        &env,
        "{{ date_diff(timestamp1=1704153600, timestamp2=1704067200) }}",
    )
    .unwrap();
    assert_eq!(result, "1");
}

#[test]
fn test_date_diff_negative() {
    let env = create_env();
    // Jan 1 - Jan 2 = -1 day
    let result = render_template(
        &env,
        "{{ date_diff(timestamp1=1704067200, timestamp2=1704153600) }}",
    )
    .unwrap();
    assert_eq!(result, "-1");
}

#[test]
fn test_date_diff_week() {
    let env = create_env();
    let result = render_template(
        &env,
        "{{ date_diff(timestamp1=1704672000, timestamp2=1704067200) }}",
    )
    .unwrap();
    assert_eq!(result, "7");
}

// Tests for get_year
#[test]
fn test_get_year() {
    let env = create_env();
    let result = render_template(&env, "{{ get_year(timestamp=1704067200) }}").unwrap();
    assert_eq!(result, "2024");
}

#[test]
fn test_get_year_different() {
    let env = create_env();
    // 2025-01-01
    let result = render_template(&env, "{{ get_year(timestamp=1735689600) }}").unwrap();
    assert_eq!(result, "2025");
}

// Tests for get_month
#[test]
fn test_get_month_january() {
    let env = create_env();
    let result = render_template(&env, "{{ get_month(timestamp=1704067200) }}").unwrap();
    assert_eq!(result, "1");
}

#[test]
fn test_get_month_december() {
    let env = create_env();
    // 2024-12-01
    let result = render_template(&env, "{{ get_month(timestamp=1733011200) }}").unwrap();
    assert_eq!(result, "12");
}

// Tests for get_day
#[test]
fn test_get_day_first() {
    let env = create_env();
    let result = render_template(&env, "{{ get_day(timestamp=1704067200) }}").unwrap();
    assert_eq!(result, "1");
}

#[test]
fn test_get_day_last() {
    let env = create_env();
    // 2024-01-31
    let result = render_template(&env, "{{ get_day(timestamp=1706659200) }}").unwrap();
    assert_eq!(result, "31");
}

// Tests for get_hour
#[test]
fn test_get_hour_midnight() {
    let env = create_env();
    let result = render_template(&env, "{{ get_hour(timestamp=1704067200) }}").unwrap();
    assert_eq!(result, "0");
}

#[test]
fn test_get_hour_noon() {
    let env = create_env();
    // 2024-01-01 12:00:00
    let result = render_template(&env, "{{ get_hour(timestamp=1704110400) }}").unwrap();
    assert_eq!(result, "12");
}

#[test]
fn test_get_hour_evening() {
    let env = create_env();
    // 2024-01-01 18:00:00
    let result = render_template(&env, "{{ get_hour(timestamp=1704132000) }}").unwrap();
    assert_eq!(result, "18");
}

// Tests for get_minute
#[test]
fn test_get_minute_zero() {
    let env = create_env();
    let result = render_template(&env, "{{ get_minute(timestamp=1704067200) }}").unwrap();
    assert_eq!(result, "0");
}

#[test]
fn test_get_minute_thirty() {
    let env = create_env();
    // 2024-01-01 12:30:00
    let result = render_template(&env, "{{ get_minute(timestamp=1704112200) }}").unwrap();
    assert_eq!(result, "30");
}

#[test]
fn test_get_minute_fiftynine() {
    let env = create_env();
    // 2024-01-01 12:59:00
    let result = render_template(&env, "{{ get_minute(timestamp=1704113940) }}").unwrap();
    assert_eq!(result, "59");
}

// Tests for timezone_convert
#[test]
fn test_timezone_convert_utc_to_utc() {
    let env = create_env();
    let result = render_template(
        &env,
        "{{ timezone_convert(timestamp=1704067200, from_tz=\"UTC\", to_tz=\"UTC\") }}",
    )
    .unwrap();
    assert_eq!(result, "1704067200");
}

#[test]
fn test_timezone_convert_utc_to_eastern() {
    let env = create_env();
    let result = render_template(
        &env,
        "{{ timezone_convert(timestamp=1704067200, from_tz=\"UTC\", to_tz=\"America/New_York\") }}",
    )
    .unwrap();
    // Unix timestamp should remain the same (it's always UTC)
    assert_eq!(result, "1704067200");
}

#[test]
fn test_timezone_convert_invalid_tz() {
    let env = create_env();
    let result = render_template(
        &env,
        "{{ timezone_convert(timestamp=1704067200, from_tz=\"Invalid/Zone\", to_tz=\"UTC\") }}",
    );
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Invalid timezone"));
}

// Tests for is_leap_year
#[test]
fn test_is_leap_year_2024() {
    let env = create_env();
    let result = render_template(&env, "{{ is_leap_year(year=2024) }}").unwrap();
    assert_eq!(result, "true");
}

#[test]
fn test_is_leap_year_2023() {
    let env = create_env();
    let result = render_template(&env, "{{ is_leap_year(year=2023) }}").unwrap();
    assert_eq!(result, "false");
}

#[test]
fn test_is_leap_year_2000() {
    let env = create_env();
    // Divisible by 400: leap year
    let result = render_template(&env, "{{ is_leap_year(year=2000) }}").unwrap();
    assert_eq!(result, "true");
}

#[test]
fn test_is_leap_year_1900() {
    let env = create_env();
    // Divisible by 100 but not 400: not a leap year
    let result = render_template(&env, "{{ is_leap_year(year=1900) }}").unwrap();
    assert_eq!(result, "false");
}

#[test]
fn test_is_leap_year_2020() {
    let env = create_env();
    let result = render_template(&env, "{{ is_leap_year(year=2020) }}").unwrap();
    assert_eq!(result, "true");
}

#[test]
fn test_is_leap_year_2100() {
    let env = create_env();
    // Divisible by 100 but not 400: not a leap year
    let result = render_template(&env, "{{ is_leap_year(year=2100) }}").unwrap();
    assert_eq!(result, "false");
}

// Integration tests combining multiple functions
#[test]
fn test_date_parsing_and_formatting() {
    let env = create_env();
    let result = render_template(
        &env,
        "{% set ts = parse_date(string=\"2024-06-15\", format=\"%Y-%m-%d\") %}{{ format_date(timestamp=ts, format=\"%B %d, %Y\") }}",
    )
    .unwrap();
    assert_eq!(result, "June 15, 2024");
}

#[test]
fn test_date_arithmetic_chain() {
    let env = create_env();
    let result = render_template(
        &env,
        "{% set ts = parse_date(string=\"2024-01-01\", format=\"%Y-%m-%d\") %}{% set ts2 = date_add(timestamp=ts, days=30) %}{{ format_date(timestamp=ts2, format=\"%Y-%m-%d\") }}",
    )
    .unwrap();
    assert_eq!(result, "2024-01-31");
}

#[test]
fn test_component_extraction() {
    let env = create_env();
    let result = render_template(
        &env,
        "{% set ts = parse_date(string=\"2024-12-25 15:30:00\", format=\"%Y-%m-%d %H:%M:%S\") %}{{ get_year(timestamp=ts) }}-{{ get_month(timestamp=ts) }}-{{ get_day(timestamp=ts) }} {{ get_hour(timestamp=ts) }}:{{ get_minute(timestamp=ts) }}",
    )
    .unwrap();
    assert_eq!(result, "2024-12-25 15:30");
}

#[test]
fn test_leap_year_with_date() {
    let env = create_env();
    let result = render_template(
        &env,
        "{% set ts = parse_date(string=\"2024-02-29\", format=\"%Y-%m-%d\") %}{% set year = get_year(timestamp=ts) %}{{ is_leap_year(year=year) }}",
    )
    .unwrap();
    assert_eq!(result, "true");
}

#[test]
fn test_date_diff_with_parsed_dates() {
    let env = create_env();
    let result = render_template(
        &env,
        "{% set start = parse_date(string=\"2024-01-01\", format=\"%Y-%m-%d\") %}{% set end = parse_date(string=\"2024-01-31\", format=\"%Y-%m-%d\") %}{{ date_diff(timestamp1=end, timestamp2=start) }}",
    )
    .unwrap();
    assert_eq!(result, "30");
}

// Edge cases
#[test]
fn test_format_date_invalid_timestamp() {
    let env = create_env();
    // Very large invalid timestamp
    let result = render_template(&env, "{{ format_date(timestamp=99999999999999) }}");
    assert!(result.is_err());
}

#[test]
fn test_date_add_across_year_boundary() {
    let env = create_env();
    // 2023-12-31 + 1 day
    let result = render_template(
        &env,
        "{{ format_date(timestamp=date_add(timestamp=1704067200, days=-1), format=\"%Y-%m-%d\") }}",
    )
    .unwrap();
    assert_eq!(result, "2023-12-31");
}

#[test]
fn test_date_add_leap_day() {
    let env = create_env();
    // 2024-02-28 + 1 day = 2024-02-29 (leap year)
    let feb28_2024 = 1709078400; // 2024-02-28 00:00:00 UTC
    let result = render_template(
        &env,
        &format!(
            "{{{{ format_date(timestamp=date_add(timestamp={}, days=1), format=\"%Y-%m-%d\") }}}}",
            feb28_2024
        ),
    )
    .unwrap();
    assert_eq!(result, "2024-02-29");
}

#[test]
fn test_component_boundary_values() {
    let env = create_env();
    // 2024-12-31 23:59:00
    let result = render_template(
        &env,
        "{% set ts = parse_date(string=\"2024-12-31 23:59:00\", format=\"%Y-%m-%d %H:%M:%S\") %}{{ get_year(timestamp=ts) }}/{{ get_month(timestamp=ts) }}/{{ get_day(timestamp=ts) }} {{ get_hour(timestamp=ts) }}:{{ get_minute(timestamp=ts) }}",
    )
    .unwrap();
    assert_eq!(result, "2024/12/31 23:59");
}

// Error case tests - testing invalid inputs

#[test]
fn test_date_add_invalid_timestamp() {
    let env = create_env();
    // Timestamp too large to be valid
    let result = render_template(&env, "{{ date_add(timestamp=99999999999999, days=1) }}");
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Invalid timestamp")
    );
}

#[test]
fn test_date_add_negative_invalid_timestamp() {
    let env = create_env();
    // Negative timestamp that's out of range
    let result = render_template(&env, "{{ date_add(timestamp=-99999999999999, days=1) }}");
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Invalid timestamp")
    );
}

#[test]
fn test_date_diff_invalid_timestamp1() {
    let env = create_env();
    let result = render_template(
        &env,
        "{{ date_diff(timestamp1=99999999999999, timestamp2=1704067200) }}",
    );
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Invalid timestamp1")
    );
}

#[test]
fn test_date_diff_invalid_timestamp2() {
    let env = create_env();
    let result = render_template(
        &env,
        "{{ date_diff(timestamp1=1704067200, timestamp2=99999999999999) }}",
    );
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Invalid timestamp2")
    );
}

#[test]
fn test_date_diff_both_invalid_timestamps() {
    let env = create_env();
    let result = render_template(
        &env,
        "{{ date_diff(timestamp1=99999999999999, timestamp2=-99999999999999) }}",
    );
    assert!(result.is_err());
    // Should fail on timestamp1 first
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Invalid timestamp")
    );
}

#[test]
fn test_get_year_invalid_timestamp() {
    let env = create_env();
    let result = render_template(&env, "{{ get_year(timestamp=99999999999999) }}");
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Invalid timestamp")
    );
}

#[test]
fn test_get_month_invalid_timestamp() {
    let env = create_env();
    let result = render_template(&env, "{{ get_month(timestamp=99999999999999) }}");
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Invalid timestamp")
    );
}

#[test]
fn test_get_day_invalid_timestamp() {
    let env = create_env();
    let result = render_template(&env, "{{ get_day(timestamp=99999999999999) }}");
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Invalid timestamp")
    );
}

#[test]
fn test_get_hour_invalid_timestamp() {
    let env = create_env();
    let result = render_template(&env, "{{ get_hour(timestamp=99999999999999) }}");
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Invalid timestamp")
    );
}

#[test]
fn test_get_minute_invalid_timestamp() {
    let env = create_env();
    let result = render_template(&env, "{{ get_minute(timestamp=99999999999999) }}");
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Invalid timestamp")
    );
}

#[test]
fn test_timezone_convert_invalid_timestamp() {
    let env = create_env();
    let result = render_template(
        &env,
        "{{ timezone_convert(timestamp=99999999999999, from_tz=\"UTC\", to_tz=\"America/New_York\") }}",
    );
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Invalid timestamp")
    );
}

#[test]
fn test_timezone_convert_invalid_from_tz() {
    let env = create_env();
    let result = render_template(
        &env,
        "{{ timezone_convert(timestamp=1704067200, from_tz=\"Not/A/Timezone\", to_tz=\"UTC\") }}",
    );
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Invalid timezone"));
}

#[test]
fn test_timezone_convert_invalid_to_tz() {
    let env = create_env();
    let result = render_template(
        &env,
        "{{ timezone_convert(timestamp=1704067200, from_tz=\"UTC\", to_tz=\"Not/A/Timezone\") }}",
    );
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Invalid timezone"));
}

// ============ Direct Function Tests ============

mod unit_tests {
    use minijinja::Value;
    use minijinja::value::Kwargs;
    // Note: is_leap_year has been moved to src/is_functions/datetime.rs
    // and is tested in tests/test_is_datetime.rs
    // Note: format_date_fn, get_year_fn, get_month_fn, get_day_fn, get_hour_fn, get_minute_fn
    // have been moved to filter_functions/datetime.rs with dual function+filter syntax.
    use tmpltool::functions::Function;
    use tmpltool::functions::datetime::{DateAdd, DateDiff, Now, ParseDate, TimezoneConvert};

    const TEST_TIMESTAMP: i64 = 1704067200; // 2024-01-01 00:00:00 UTC

    #[test]
    fn test_now_fn_direct() {
        let kwargs = Kwargs::from_iter(Vec::<(&str, Value)>::new());
        let result = Now::call(kwargs).unwrap();
        // Should return a timestamp (integer)
        assert!(result.as_i64().is_some());
    }

    #[test]
    fn test_now_fn_with_format() {
        let kwargs = Kwargs::from_iter(vec![("format", Value::from("%Y"))]);
        let result = Now::call(kwargs).unwrap();
        // Should return a formatted string
        assert!(result.as_str().is_some());
    }

    // Note: format_date_fn tests removed - function now in filter_functions/datetime.rs

    #[test]
    fn test_parse_date_fn_direct() {
        let kwargs = Kwargs::from_iter(vec![
            ("string", Value::from("2024-01-01")),
            ("format", Value::from("%Y-%m-%d")),
        ]);
        let result = ParseDate::call(kwargs).unwrap();
        assert_eq!(result.as_i64().unwrap(), TEST_TIMESTAMP);
    }

    #[test]
    fn test_parse_date_fn_with_time() {
        let kwargs = Kwargs::from_iter(vec![
            ("string", Value::from("2024-01-01 12:00:00")),
            ("format", Value::from("%Y-%m-%d %H:%M:%S")),
        ]);
        let result = ParseDate::call(kwargs).unwrap();
        assert!(result.as_i64().is_some());
    }

    #[test]
    fn test_parse_date_fn_invalid_format() {
        let kwargs = Kwargs::from_iter(vec![
            ("string", Value::from("not-a-date")),
            ("format", Value::from("%Y-%m-%d")),
        ]);
        let result = ParseDate::call(kwargs);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Failed to parse"));
    }

    #[test]
    fn test_date_add_fn_direct() {
        let kwargs = Kwargs::from_iter(vec![
            ("timestamp", Value::from(TEST_TIMESTAMP)),
            ("days", Value::from(7)),
        ]);
        let result = DateAdd::call(kwargs).unwrap();
        assert_eq!(result.as_i64().unwrap(), TEST_TIMESTAMP + 7 * 86400);
    }

    #[test]
    fn test_date_add_fn_negative() {
        let kwargs = Kwargs::from_iter(vec![
            ("timestamp", Value::from(TEST_TIMESTAMP)),
            ("days", Value::from(-7)),
        ]);
        let result = DateAdd::call(kwargs).unwrap();
        assert_eq!(result.as_i64().unwrap(), TEST_TIMESTAMP - 7 * 86400);
    }

    #[test]
    fn test_date_add_fn_invalid_timestamp() {
        let kwargs = Kwargs::from_iter(vec![
            ("timestamp", Value::from(i64::MAX)),
            ("days", Value::from(1)),
        ]);
        let result = DateAdd::call(kwargs);
        assert!(result.is_err());
    }

    #[test]
    fn test_date_diff_fn_direct() {
        let kwargs = Kwargs::from_iter(vec![
            ("timestamp1", Value::from(TEST_TIMESTAMP + 7 * 86400)),
            ("timestamp2", Value::from(TEST_TIMESTAMP)),
        ]);
        let result = DateDiff::call(kwargs).unwrap();
        assert_eq!(result.as_i64().unwrap(), 7);
    }

    #[test]
    fn test_date_diff_fn_invalid_timestamp1() {
        let kwargs = Kwargs::from_iter(vec![
            ("timestamp1", Value::from(i64::MAX)),
            ("timestamp2", Value::from(TEST_TIMESTAMP)),
        ]);
        let result = DateDiff::call(kwargs);
        assert!(result.is_err());
    }

    #[test]
    fn test_date_diff_fn_invalid_timestamp2() {
        let kwargs = Kwargs::from_iter(vec![
            ("timestamp1", Value::from(TEST_TIMESTAMP)),
            ("timestamp2", Value::from(i64::MAX)),
        ]);
        let result = DateDiff::call(kwargs);
        assert!(result.is_err());
    }

    // Note: get_year_fn, get_month_fn, get_day_fn, get_hour_fn, get_minute_fn tests removed
    // - these functions are now in filter_functions/datetime.rs with dual function+filter syntax.

    #[test]
    fn test_timezone_convert_fn_direct() {
        let kwargs = Kwargs::from_iter(vec![
            ("timestamp", Value::from(TEST_TIMESTAMP)),
            ("from_tz", Value::from("UTC")),
            ("to_tz", Value::from("America/New_York")),
        ]);
        let result = TimezoneConvert::call(kwargs).unwrap();
        assert!(result.as_i64().is_some());
    }

    #[test]
    fn test_timezone_convert_fn_invalid_timestamp() {
        let kwargs = Kwargs::from_iter(vec![
            ("timestamp", Value::from(i64::MAX)),
            ("from_tz", Value::from("UTC")),
            ("to_tz", Value::from("UTC")),
        ]);
        let result = TimezoneConvert::call(kwargs);
        assert!(result.is_err());
    }

    #[test]
    fn test_timezone_convert_fn_invalid_from_tz() {
        let kwargs = Kwargs::from_iter(vec![
            ("timestamp", Value::from(TEST_TIMESTAMP)),
            ("from_tz", Value::from("Invalid/TZ")),
            ("to_tz", Value::from("UTC")),
        ]);
        let result = TimezoneConvert::call(kwargs);
        assert!(result.is_err());
    }

    #[test]
    fn test_timezone_convert_fn_invalid_to_tz() {
        let kwargs = Kwargs::from_iter(vec![
            ("timestamp", Value::from(TEST_TIMESTAMP)),
            ("from_tz", Value::from("UTC")),
            ("to_tz", Value::from("Invalid/TZ")),
        ]);
        let result = TimezoneConvert::call(kwargs);
        assert!(result.is_err());
    }

    // Note: is_leap_year tests have been moved to tests/test_is_datetime.rs
}
