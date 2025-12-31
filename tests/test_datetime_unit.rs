use tmpltool::functions::datetime::now_fn;

#[test]
fn test_now_fn_returns_timestamp() {
    let result = now_fn().unwrap();
    let timestamp = result.as_str().unwrap();

    // Check it's not empty
    assert!(!timestamp.is_empty());

    // Check it contains basic ISO 8601 components
    assert!(timestamp.contains('T'), "Should contain date-time separator");
    assert!(
        timestamp.contains('-'),
        "Should contain date separators"
    );
    assert!(timestamp.contains(':'), "Should contain time separators");
}

#[test]
fn test_now_fn_format_iso8601() {
    let result = now_fn().unwrap();
    let timestamp = result.as_str().unwrap();

    // Check basic ISO 8601 format: YYYY-MM-DDTHH:MM:SS
    // Format should be like: 2024-12-31T12:34:56.789+00:00 or 2024-12-31T12:34:56.789Z
    let parts: Vec<&str> = timestamp.split('T').collect();
    assert_eq!(parts.len(), 2, "Should have date and time parts");

    let date_part = parts[0];
    let date_components: Vec<&str> = date_part.split('-').collect();
    assert_eq!(date_components.len(), 3, "Date should have year-month-day");
    assert_eq!(
        date_components[0].len(),
        4,
        "Year should be 4 digits"
    );
    assert_eq!(
        date_components[1].len(),
        2,
        "Month should be 2 digits"
    );
    assert_eq!(date_components[2].len(), 2, "Day should be 2 digits");
}

#[test]
fn test_now_fn_valid_year() {
    let result = now_fn().unwrap();
    let timestamp = result.as_str().unwrap();

    let year_str = &timestamp[0..4];
    let year: i32 = year_str.parse().unwrap();

    // Check year is reasonable (between 2020 and 2100)
    assert!(
        year >= 2020 && year <= 2100,
        "Year should be reasonable: {}",
        year
    );
}

#[test]
fn test_now_fn_monotonic() {
    // Get two timestamps and ensure second is >= first
    let result1 = now_fn().unwrap();
    let timestamp1 = result1.as_str().unwrap().to_string();

    // Small delay to ensure different timestamps
    std::thread::sleep(std::time::Duration::from_millis(10));

    let result2 = now_fn().unwrap();
    let timestamp2 = result2.as_str().unwrap().to_string();

    // Second timestamp should be >= first (lexicographic comparison works for ISO 8601)
    assert!(
        timestamp2 >= timestamp1,
        "Timestamps should be monotonic: {} >= {}",
        timestamp2,
        timestamp1
    );
}

#[test]
fn test_now_fn_consistent_format() {
    // Call multiple times and ensure format is consistent
    for _ in 0..5 {
        let result = now_fn().unwrap();
        let timestamp = result.as_str().unwrap();

        // All should have T separator
        assert!(timestamp.contains('T'));

        // All should have timezone info (+ or Z at the end)
        assert!(
            timestamp.contains('+') || timestamp.ends_with('Z'),
            "Should have timezone info"
        );
    }
}

#[test]
fn test_now_fn_valid_month() {
    let result = now_fn().unwrap();
    let timestamp = result.as_str().unwrap();

    let month_str = &timestamp[5..7];
    let month: u32 = month_str.parse().unwrap();

    assert!(month >= 1 && month <= 12, "Month should be 1-12: {}", month);
}

#[test]
fn test_now_fn_valid_day() {
    let result = now_fn().unwrap();
    let timestamp = result.as_str().unwrap();

    let day_str = &timestamp[8..10];
    let day: u32 = day_str.parse().unwrap();

    assert!(day >= 1 && day <= 31, "Day should be 1-31: {}", day);
}

#[test]
fn test_now_fn_valid_hour() {
    let result = now_fn().unwrap();
    let timestamp = result.as_str().unwrap();

    let hour_str = &timestamp[11..13];
    let hour: u32 = hour_str.parse().unwrap();

    assert!(hour <= 23, "Hour should be 0-23: {}", hour);
}

#[test]
fn test_now_fn_valid_minute() {
    let result = now_fn().unwrap();
    let timestamp = result.as_str().unwrap();

    let minute_str = &timestamp[14..16];
    let minute: u32 = minute_str.parse().unwrap();

    assert!(minute <= 59, "Minute should be 0-59: {}", minute);
}

#[test]
fn test_now_fn_valid_second() {
    let result = now_fn().unwrap();
    let timestamp = result.as_str().unwrap();

    let second_str = &timestamp[17..19];
    let second: u32 = second_str.parse().unwrap();

    assert!(second <= 60, "Second should be 0-60 (leap second): {}", second);
}
