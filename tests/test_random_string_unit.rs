use minijinja::value::Kwargs;
use tmpltool::functions::Function;
use tmpltool::functions::random::RandomString;

// Helper to create kwargs for testing
fn create_kwargs(args: Vec<(&str, minijinja::Value)>) -> Kwargs {
    Kwargs::from_iter(args)
}

const CHARSET_ALPHANUMERIC: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
const CHARSET_LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const CHARSET_UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const CHARSET_NUMERIC: &str = "0123456789";
const CHARSET_HEX: &str = "0123456789abcdef";

#[test]
fn test_random_string_basic() {
    let kwargs = create_kwargs(vec![("length", minijinja::Value::from(16))]);

    let result = RandomString::call(kwargs).unwrap();
    let random_str = result.as_str().unwrap();

    assert_eq!(random_str.len(), 16);
    for ch in random_str.chars() {
        assert!(CHARSET_ALPHANUMERIC.contains(ch));
    }
}

#[test]
fn test_random_string_alphanumeric() {
    let kwargs = create_kwargs(vec![
        ("length", minijinja::Value::from(20)),
        ("charset", minijinja::Value::from("alphanumeric")),
    ]);

    let result = RandomString::call(kwargs).unwrap();
    let random_str = result.as_str().unwrap();

    assert_eq!(random_str.len(), 20);
    for ch in random_str.chars() {
        assert!(CHARSET_ALPHANUMERIC.contains(ch));
    }
}

#[test]
fn test_random_string_lowercase() {
    let kwargs = create_kwargs(vec![
        ("length", minijinja::Value::from(10)),
        ("charset", minijinja::Value::from("lowercase")),
    ]);

    let result = RandomString::call(kwargs).unwrap();
    let random_str = result.as_str().unwrap();

    assert_eq!(random_str.len(), 10);
    for ch in random_str.chars() {
        assert!(CHARSET_LOWERCASE.contains(ch));
    }
}

#[test]
fn test_random_string_uppercase() {
    let kwargs = create_kwargs(vec![
        ("length", minijinja::Value::from(10)),
        ("charset", minijinja::Value::from("uppercase")),
    ]);

    let result = RandomString::call(kwargs).unwrap();
    let random_str = result.as_str().unwrap();

    assert_eq!(random_str.len(), 10);
    for ch in random_str.chars() {
        assert!(CHARSET_UPPERCASE.contains(ch));
    }
}

#[test]
fn test_random_string_numeric() {
    let kwargs = create_kwargs(vec![
        ("length", minijinja::Value::from(8)),
        ("charset", minijinja::Value::from("numeric")),
    ]);

    let result = RandomString::call(kwargs).unwrap();
    let random_str = result.as_str().unwrap();

    assert_eq!(random_str.len(), 8);
    for ch in random_str.chars() {
        assert!(CHARSET_NUMERIC.contains(ch));
    }
}

#[test]
fn test_random_string_hex() {
    let kwargs = create_kwargs(vec![
        ("length", minijinja::Value::from(12)),
        ("charset", minijinja::Value::from("hex")),
    ]);

    let result = RandomString::call(kwargs).unwrap();
    let random_str = result.as_str().unwrap();

    assert_eq!(random_str.len(), 12);
    for ch in random_str.chars() {
        assert!(CHARSET_HEX.contains(ch));
    }
}

#[test]
fn test_random_string_custom_charset() {
    let kwargs = create_kwargs(vec![
        ("length", minijinja::Value::from(15)),
        ("charset", minijinja::Value::from("abc123")),
    ]);

    let result = RandomString::call(kwargs).unwrap();
    let random_str = result.as_str().unwrap();

    assert_eq!(random_str.len(), 15);
    for ch in random_str.chars() {
        assert!("abc123".contains(ch));
    }
}

#[test]
fn test_random_string_empty_length() {
    let kwargs = create_kwargs(vec![("length", minijinja::Value::from(0))]);

    let result = RandomString::call(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "");
}

#[test]
fn test_random_string_no_length() {
    let kwargs = create_kwargs(vec![]);
    let result = RandomString::call(kwargs);
    assert!(result.is_err());
}

#[test]
fn test_random_string_too_long() {
    let kwargs = create_kwargs(vec![("length", minijinja::Value::from(10001))]);

    let result = RandomString::call(kwargs);
    assert!(result.is_err());
}

#[test]
fn test_random_string_uniqueness() {
    let kwargs1 = create_kwargs(vec![("length", minijinja::Value::from(20))]);
    let kwargs2 = create_kwargs(vec![("length", minijinja::Value::from(20))]);

    let result1 = RandomString::call(kwargs1).unwrap();
    let result2 = RandomString::call(kwargs2).unwrap();

    // Two random strings should be different (with very high probability)
    assert_ne!(result1.as_str().unwrap(), result2.as_str().unwrap());
}
