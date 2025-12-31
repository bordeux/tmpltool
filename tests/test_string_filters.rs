use minijinja::Value;
use tmpltool::filters::string::slugify_filter;

#[test]
fn test_slugify_basic() {
    let value = Value::from("Hello World");
    assert_eq!(slugify_filter(&value).unwrap(), "hello-world");
}

#[test]
fn test_slugify_with_special_chars() {
    let value = Value::from("jane smith!");
    assert_eq!(slugify_filter(&value).unwrap(), "jane-smith");
}

#[test]
fn test_slugify_multiple_spaces() {
    let value = Value::from("foo   bar");
    assert_eq!(slugify_filter(&value).unwrap(), "foo-bar");
}

#[test]
fn test_slugify_with_numbers() {
    let value = Value::from("Test 123");
    assert_eq!(slugify_filter(&value).unwrap(), "test-123");
}

#[test]
fn test_slugify_with_underscores() {
    let value = Value::from("test_case_name");
    assert_eq!(slugify_filter(&value).unwrap(), "test-case-name");
}

#[test]
fn test_slugify_already_slug() {
    let value = Value::from("already-a-slug");
    assert_eq!(slugify_filter(&value).unwrap(), "already-a-slug");
}

#[test]
fn test_slugify_empty_string() {
    let value = Value::from("");
    assert_eq!(slugify_filter(&value).unwrap(), "");
}

#[test]
fn test_slugify_only_special_chars() {
    let value = Value::from("!@#$%^&*()");
    assert_eq!(slugify_filter(&value).unwrap(), "");
}

#[test]
fn test_slugify_mixed_case() {
    let value = Value::from("CamelCaseString");
    assert_eq!(slugify_filter(&value).unwrap(), "camelcasestring");
}

#[test]
fn test_slugify_error_not_string() {
    let value = Value::from(123);
    let result = slugify_filter(&value);
    assert!(result.is_err());
    assert!(
        result
            .err()
            .unwrap()
            .to_string()
            .contains("requires a string")
    );
}
