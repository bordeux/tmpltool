use std::collections::HashMap;
use tera::Value;
use tmpltool::functions::random_string::RandomString;

// Import the Function trait to use call()
use tera::Function;

const CHARSET_ALPHANUMERIC: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
const CHARSET_LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const CHARSET_UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const CHARSET_NUMERIC: &str = "0123456789";
const CHARSET_HEX: &str = "0123456789abcdef";

#[test]
fn test_random_string_basic() {
    let mut args = HashMap::new();
    args.insert("length".to_string(), Value::Number(16.into()));

    let result = RandomString.call(&args).unwrap();
    let random_str = result.as_str().unwrap();

    assert_eq!(random_str.len(), 16);
    for ch in random_str.chars() {
        assert!(CHARSET_ALPHANUMERIC.contains(ch));
    }
}

#[test]
fn test_random_string_alphanumeric() {
    let mut args = HashMap::new();
    args.insert("length".to_string(), Value::Number(20.into()));
    args.insert(
        "charset".to_string(),
        Value::String("alphanumeric".to_string()),
    );

    let result = RandomString.call(&args).unwrap();
    let random_str = result.as_str().unwrap();

    assert_eq!(random_str.len(), 20);
    for ch in random_str.chars() {
        assert!(CHARSET_ALPHANUMERIC.contains(ch));
    }
}

#[test]
fn test_random_string_lowercase() {
    let mut args = HashMap::new();
    args.insert("length".to_string(), Value::Number(10.into()));
    args.insert(
        "charset".to_string(),
        Value::String("lowercase".to_string()),
    );

    let result = RandomString.call(&args).unwrap();
    let random_str = result.as_str().unwrap();

    assert_eq!(random_str.len(), 10);
    for ch in random_str.chars() {
        assert!(CHARSET_LOWERCASE.contains(ch));
    }
}

#[test]
fn test_random_string_uppercase() {
    let mut args = HashMap::new();
    args.insert("length".to_string(), Value::Number(10.into()));
    args.insert(
        "charset".to_string(),
        Value::String("uppercase".to_string()),
    );

    let result = RandomString.call(&args).unwrap();
    let random_str = result.as_str().unwrap();

    assert_eq!(random_str.len(), 10);
    for ch in random_str.chars() {
        assert!(CHARSET_UPPERCASE.contains(ch));
    }
}

#[test]
fn test_random_string_numeric() {
    let mut args = HashMap::new();
    args.insert("length".to_string(), Value::Number(8.into()));
    args.insert("charset".to_string(), Value::String("numeric".to_string()));

    let result = RandomString.call(&args).unwrap();
    let random_str = result.as_str().unwrap();

    assert_eq!(random_str.len(), 8);
    for ch in random_str.chars() {
        assert!(CHARSET_NUMERIC.contains(ch));
    }
}

#[test]
fn test_random_string_hex() {
    let mut args = HashMap::new();
    args.insert("length".to_string(), Value::Number(12.into()));
    args.insert("charset".to_string(), Value::String("hex".to_string()));

    let result = RandomString.call(&args).unwrap();
    let random_str = result.as_str().unwrap();

    assert_eq!(random_str.len(), 12);
    for ch in random_str.chars() {
        assert!(CHARSET_HEX.contains(ch));
    }
}

#[test]
fn test_random_string_custom_charset() {
    let mut args = HashMap::new();
    args.insert("length".to_string(), Value::Number(15.into()));
    args.insert("charset".to_string(), Value::String("abc123".to_string()));

    let result = RandomString.call(&args).unwrap();
    let random_str = result.as_str().unwrap();

    assert_eq!(random_str.len(), 15);
    for ch in random_str.chars() {
        assert!("abc123".contains(ch));
    }
}

#[test]
fn test_random_string_empty_length() {
    let mut args = HashMap::new();
    args.insert("length".to_string(), Value::Number(0.into()));

    let result = RandomString.call(&args).unwrap();
    assert_eq!(result.as_str().unwrap(), "");
}

#[test]
fn test_random_string_no_length() {
    let args = HashMap::new();
    let result = RandomString.call(&args);
    assert!(result.is_err());
}

#[test]
fn test_random_string_too_long() {
    let mut args = HashMap::new();
    args.insert("length".to_string(), Value::Number(10001.into()));

    let result = RandomString.call(&args);
    assert!(result.is_err());
}

#[test]
fn test_random_string_uniqueness() {
    let mut args = HashMap::new();
    args.insert("length".to_string(), Value::Number(20.into()));

    let result1 = RandomString.call(&args).unwrap();
    let result2 = RandomString.call(&args).unwrap();

    // Two random strings should be different (with very high probability)
    assert_ne!(result1.as_str().unwrap(), result2.as_str().unwrap());
}
