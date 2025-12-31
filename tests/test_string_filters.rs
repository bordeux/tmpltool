use minijinja::Value;
use tmpltool::filters::string::*;

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

// ============================================================================
// Indent Filter Tests
// ============================================================================

#[test]
fn test_indent_default() {
    let value = Value::from("hello");
    assert_eq!(indent_filter(&value, None).unwrap(), "    hello");
}

#[test]
fn test_indent_custom() {
    let value = Value::from("hello");
    assert_eq!(indent_filter(&value, Some(2)).unwrap(), "  hello");
}

#[test]
fn test_indent_multiline() {
    let value = Value::from("line1\nline2\nline3");
    assert_eq!(
        indent_filter(&value, Some(2)).unwrap(),
        "  line1\n  line2\n  line3"
    );
}

#[test]
fn test_indent_with_empty_lines() {
    let value = Value::from("line1\n\nline3");
    assert_eq!(
        indent_filter(&value, Some(2)).unwrap(),
        "  line1\n\n  line3"
    );
}

// ============================================================================
// Dedent Filter Tests
// ============================================================================

#[test]
fn test_dedent_basic() {
    let value = Value::from("  line1\n  line2");
    assert_eq!(dedent_filter(&value).unwrap(), "line1\nline2");
}

#[test]
fn test_dedent_different_levels() {
    let value = Value::from("    line1\n  line2");
    assert_eq!(dedent_filter(&value).unwrap(), "  line1\nline2");
}

#[test]
fn test_dedent_no_indent() {
    let value = Value::from("line1\nline2");
    assert_eq!(dedent_filter(&value).unwrap(), "line1\nline2");
}

#[test]
fn test_dedent_empty_string() {
    let value = Value::from("");
    assert_eq!(dedent_filter(&value).unwrap(), "");
}

// ============================================================================
// Quote Filter Tests
// ============================================================================

#[test]
fn test_quote_default_double() {
    let value = Value::from("hello");
    assert_eq!(quote_filter(&value, None).unwrap(), r#""hello""#);
}

#[test]
fn test_quote_single() {
    let value = Value::from("hello");
    assert_eq!(
        quote_filter(&value, Some("single".to_string())).unwrap(),
        "'hello'"
    );
}

#[test]
fn test_quote_backtick() {
    let value = Value::from("hello");
    assert_eq!(
        quote_filter(&value, Some("backtick".to_string())).unwrap(),
        "`hello`"
    );
}

#[test]
fn test_quote_invalid_style() {
    let value = Value::from("hello");
    let result = quote_filter(&value, Some("invalid".to_string()));
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Invalid quote style")
    );
}

// ============================================================================
// Escape Quotes Filter Tests
// ============================================================================

#[test]
fn test_escape_quotes_basic() {
    let value = Value::from(r#"It's a "test""#);
    assert_eq!(escape_quotes_filter(&value).unwrap(), r#"It\'s a \"test\""#);
}

#[test]
fn test_escape_quotes_with_backslash() {
    let value = Value::from(r#"path\to\file"#);
    assert_eq!(escape_quotes_filter(&value).unwrap(), r#"path\\to\\file"#);
}

#[test]
fn test_escape_quotes_simple() {
    let value = Value::from("Simple");
    assert_eq!(escape_quotes_filter(&value).unwrap(), "Simple");
}

// ============================================================================
// Case Conversion Filter Tests
// ============================================================================

#[test]
fn test_to_snake_case_from_pascal() {
    let value = Value::from("HelloWorld");
    assert_eq!(to_snake_case_filter(&value).unwrap(), "hello_world");
}

#[test]
fn test_to_snake_case_from_kebab() {
    let value = Value::from("hello-world");
    assert_eq!(to_snake_case_filter(&value).unwrap(), "hello_world");
}

#[test]
fn test_to_snake_case_from_spaces() {
    let value = Value::from("hello world");
    assert_eq!(to_snake_case_filter(&value).unwrap(), "hello_world");
}

#[test]
fn test_to_snake_case_from_camel() {
    let value = Value::from("helloWorld");
    assert_eq!(to_snake_case_filter(&value).unwrap(), "hello_world");
}

#[test]
fn test_to_camel_case_from_snake() {
    let value = Value::from("hello_world");
    assert_eq!(to_camel_case_filter(&value).unwrap(), "helloWorld");
}

#[test]
fn test_to_camel_case_from_kebab() {
    let value = Value::from("hello-world");
    assert_eq!(to_camel_case_filter(&value).unwrap(), "helloWorld");
}

#[test]
fn test_to_camel_case_from_spaces() {
    let value = Value::from("hello world");
    assert_eq!(to_camel_case_filter(&value).unwrap(), "helloWorld");
}

#[test]
fn test_to_camel_case_from_pascal() {
    let value = Value::from("HelloWorld");
    assert_eq!(to_camel_case_filter(&value).unwrap(), "helloWorld");
}

#[test]
fn test_to_pascal_case_from_snake() {
    let value = Value::from("hello_world");
    assert_eq!(to_pascal_case_filter(&value).unwrap(), "HelloWorld");
}

#[test]
fn test_to_pascal_case_from_kebab() {
    let value = Value::from("hello-world");
    assert_eq!(to_pascal_case_filter(&value).unwrap(), "HelloWorld");
}

#[test]
fn test_to_pascal_case_from_spaces() {
    let value = Value::from("hello world");
    assert_eq!(to_pascal_case_filter(&value).unwrap(), "HelloWorld");
}

#[test]
fn test_to_pascal_case_from_camel() {
    let value = Value::from("helloWorld");
    assert_eq!(to_pascal_case_filter(&value).unwrap(), "HelloWorld");
}

#[test]
fn test_to_kebab_case_from_pascal() {
    let value = Value::from("HelloWorld");
    assert_eq!(to_kebab_case_filter(&value).unwrap(), "hello-world");
}

#[test]
fn test_to_kebab_case_from_snake() {
    let value = Value::from("hello_world");
    assert_eq!(to_kebab_case_filter(&value).unwrap(), "hello-world");
}

#[test]
fn test_to_kebab_case_from_spaces() {
    let value = Value::from("hello world");
    assert_eq!(to_kebab_case_filter(&value).unwrap(), "hello-world");
}

#[test]
fn test_to_kebab_case_from_camel() {
    let value = Value::from("helloWorld");
    assert_eq!(to_kebab_case_filter(&value).unwrap(), "hello-world");
}

// ============================================================================
// Padding Filter Tests
// ============================================================================

#[test]
fn test_pad_left_default() {
    let value = Value::from("hi");
    assert_eq!(pad_left_filter(&value, 5, None).unwrap(), "   hi");
}

#[test]
fn test_pad_left_custom_char() {
    let value = Value::from("5");
    assert_eq!(
        pad_left_filter(&value, 3, Some("0".to_string())).unwrap(),
        "005"
    );
}

#[test]
fn test_pad_left_no_padding_needed() {
    let value = Value::from("hello");
    assert_eq!(pad_left_filter(&value, 3, None).unwrap(), "hello");
}

#[test]
fn test_pad_left_exact_length() {
    let value = Value::from("abc");
    assert_eq!(pad_left_filter(&value, 3, None).unwrap(), "abc");
}

#[test]
fn test_pad_right_default() {
    let value = Value::from("hi");
    assert_eq!(pad_right_filter(&value, 5, None).unwrap(), "hi   ");
}

#[test]
fn test_pad_right_custom_char() {
    let value = Value::from("5");
    assert_eq!(
        pad_right_filter(&value, 3, Some("0".to_string())).unwrap(),
        "500"
    );
}

#[test]
fn test_pad_right_no_padding_needed() {
    let value = Value::from("hello");
    assert_eq!(pad_right_filter(&value, 3, None).unwrap(), "hello");
}

#[test]
fn test_pad_right_exact_length() {
    let value = Value::from("abc");
    assert_eq!(pad_right_filter(&value, 3, None).unwrap(), "abc");
}

// ============================================================================
// Repeat Filter Tests
// ============================================================================

#[test]
fn test_repeat_basic() {
    let value = Value::from("ab");
    assert_eq!(repeat_filter(&value, 3).unwrap(), "ababab");
}

#[test]
fn test_repeat_single_char() {
    let value = Value::from("-");
    assert_eq!(repeat_filter(&value, 5).unwrap(), "-----");
}

#[test]
fn test_repeat_zero_times() {
    let value = Value::from("x");
    assert_eq!(repeat_filter(&value, 0).unwrap(), "");
}

#[test]
fn test_repeat_once() {
    let value = Value::from("test");
    assert_eq!(repeat_filter(&value, 1).unwrap(), "test");
}

// ============================================================================
// Reverse Filter Tests
// ============================================================================

#[test]
fn test_reverse_basic() {
    let value = Value::from("hello");
    assert_eq!(reverse_filter(&value).unwrap(), "olleh");
}

#[test]
fn test_reverse_numbers() {
    let value = Value::from("12345");
    assert_eq!(reverse_filter(&value).unwrap(), "54321");
}

#[test]
fn test_reverse_single_char() {
    let value = Value::from("a");
    assert_eq!(reverse_filter(&value).unwrap(), "a");
}

#[test]
fn test_reverse_empty() {
    let value = Value::from("");
    assert_eq!(reverse_filter(&value).unwrap(), "");
}

#[test]
fn test_reverse_unicode() {
    let value = Value::from("hello世界");
    assert_eq!(reverse_filter(&value).unwrap(), "界世olleh");
}

// ============================================================================
// Unicode Handling Tests
// ============================================================================

#[test]
fn test_unicode_cafe() {
    let value = Value::from("café");
    assert_eq!(to_snake_case_filter(&value).unwrap(), "café");
}

#[test]
fn test_unicode_emoji_repeat() {
    let value = Value::from("🚀");
    assert_eq!(repeat_filter(&value, 3).unwrap(), "🚀🚀🚀");
}
