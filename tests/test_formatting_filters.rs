use minijinja::Value;
use tmpltool::filters::formatting::{filesizeformat_filter, urlencode_filter};

// ========== filesizeformat tests ==========

#[test]
fn test_filesizeformat_bytes() {
    let value = Value::from(500);
    assert_eq!(filesizeformat_filter(&value).unwrap(), "500 bytes");
}

#[test]
fn test_filesizeformat_kb() {
    let value = Value::from(2048);
    assert_eq!(filesizeformat_filter(&value).unwrap(), "2 KB");
}

#[test]
fn test_filesizeformat_mb() {
    let value = Value::from(1048576);
    assert_eq!(filesizeformat_filter(&value).unwrap(), "1 MB");
}

#[test]
fn test_filesizeformat_gb() {
    let value = Value::from(1073741824_i64);
    assert_eq!(filesizeformat_filter(&value).unwrap(), "1 GB");
}

#[test]
fn test_filesizeformat_decimal_kb() {
    let value = Value::from(1536); // 1.5 KB
    let result = filesizeformat_filter(&value).unwrap();
    assert!(result.starts_with("1.5") && result.ends_with("KB"));
}

#[test]
fn test_filesizeformat_decimal_mb() {
    let value = Value::from(2621440); // 2.5 MB
    let result = filesizeformat_filter(&value).unwrap();
    assert!(result.starts_with("2.5") && result.ends_with("MB"));
}

#[test]
fn test_filesizeformat_zero() {
    let value = Value::from(0);
    assert_eq!(filesizeformat_filter(&value).unwrap(), "0 bytes");
}

#[test]
fn test_filesizeformat_one_byte() {
    let value = Value::from(1);
    assert_eq!(filesizeformat_filter(&value).unwrap(), "1 bytes");
}

#[test]
fn test_filesizeformat_large_tb() {
    let value = Value::from(1099511627776_i64); // 1 TB
    assert_eq!(filesizeformat_filter(&value).unwrap(), "1 TB");
}

#[test]
fn test_filesizeformat_error_not_number() {
    let value = Value::from("not a number");
    let result = filesizeformat_filter(&value);
    assert!(result.is_err());
    assert!(
        result
            .err()
            .unwrap()
            .to_string()
            .contains("requires a number")
    );
}

// ========== urlencode tests ==========

#[test]
fn test_urlencode_basic() {
    let value = Value::from("hello world");
    assert_eq!(urlencode_filter(&value).unwrap(), "hello%20world");
}

#[test]
fn test_urlencode_special_chars() {
    let value = Value::from("hello world & foo=bar");
    assert_eq!(
        urlencode_filter(&value).unwrap(),
        "hello%20world%20%26%20foo%3Dbar"
    );
}

#[test]
fn test_urlencode_already_encoded() {
    let value = Value::from("hello%20world");
    assert_eq!(urlencode_filter(&value).unwrap(), "hello%2520world");
}

#[test]
fn test_urlencode_alphanumeric() {
    let value = Value::from("abc123");
    assert_eq!(urlencode_filter(&value).unwrap(), "abc123");
}

#[test]
fn test_urlencode_url() {
    let value = Value::from("https://example.com/path?query=value");
    assert_eq!(
        urlencode_filter(&value).unwrap(),
        "https%3A%2F%2Fexample%2Ecom%2Fpath%3Fquery%3Dvalue"
    );
}

#[test]
fn test_urlencode_empty_string() {
    let value = Value::from("");
    assert_eq!(urlencode_filter(&value).unwrap(), "");
}

#[test]
fn test_urlencode_unicode() {
    let value = Value::from("hello 世界");
    let result = urlencode_filter(&value).unwrap();
    assert!(result.contains("%E4%B8%96%E7%95%8C")); // UTF-8 encoding of 世界
}

#[test]
fn test_urlencode_slash() {
    let value = Value::from("path/to/file");
    assert_eq!(urlencode_filter(&value).unwrap(), "path%2Fto%2Ffile");
}

#[test]
fn test_urlencode_error_not_string() {
    let value = Value::from(123);
    let result = urlencode_filter(&value);
    assert!(result.is_err());
    assert!(
        result
            .err()
            .unwrap()
            .to_string()
            .contains("requires a string")
    );
}
