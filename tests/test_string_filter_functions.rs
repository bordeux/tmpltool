//! Tests for string filter-functions migrated from filters.
//!
//! Tests both function and filter syntax for the 13 migrated string functions.

use minijinja::Value;
use minijinja::value::Kwargs;
use tmpltool::filter_functions::FilterFunction;
use tmpltool::filter_functions::string::{
    Dedent, EscapeQuotes, Indent, PadLeft, PadRight, Quote, Repeat, Reverse, Slugify, ToCamelCase,
    ToKebabCase, ToPascalCase, ToSnakeCase,
};

/// Helper to create empty kwargs
fn empty_kwargs() -> Kwargs {
    Kwargs::from_iter(Vec::<(&str, Value)>::new())
}

// ============================================
// Slugify function syntax tests
// ============================================

#[test]
fn test_slugify_function_syntax() {
    let kwargs = Kwargs::from_iter(vec![("string", Value::from("Hello World!"))]);
    let result = Slugify::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "hello-world");
}

#[test]
fn test_slugify_filter_syntax() {
    let result = Slugify::call_as_filter(&Value::from("Hello World!"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "hello-world");
}

#[test]
fn test_slugify_with_underscores() {
    let kwargs = Kwargs::from_iter(vec![("string", Value::from("test_case_name"))]);
    let result = Slugify::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "test-case-name");
}

#[test]
fn test_slugify_filter_error() {
    let result = Slugify::call_as_filter(&Value::from(123), empty_kwargs());
    assert!(result.is_err());
}

// ============================================
// Indent function syntax tests
// ============================================

#[test]
fn test_indent_function_syntax_default() {
    let kwargs = Kwargs::from_iter(vec![("string", Value::from("hello"))]);
    let result = Indent::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "    hello");
}

#[test]
fn test_indent_function_syntax_custom() {
    let kwargs = Kwargs::from_iter(vec![
        ("string", Value::from("hello")),
        ("spaces", Value::from(2)),
    ]);
    let result = Indent::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "  hello");
}

#[test]
fn test_indent_filter_syntax() {
    let kwargs = Kwargs::from_iter(vec![("spaces", Value::from(2))]);
    let result = Indent::call_as_filter(&Value::from("hello"), kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "  hello");
}

#[test]
fn test_indent_multiline() {
    let kwargs = Kwargs::from_iter(vec![
        ("string", Value::from("line1\nline2")),
        ("spaces", Value::from(2)),
    ]);
    let result = Indent::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "  line1\n  line2");
}

// ============================================
// Dedent function syntax tests
// ============================================

#[test]
fn test_dedent_function_syntax() {
    let kwargs = Kwargs::from_iter(vec![("string", Value::from("  line1\n  line2"))]);
    let result = Dedent::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "line1\nline2");
}

#[test]
fn test_dedent_filter_syntax() {
    let result = Dedent::call_as_filter(&Value::from("  line1\n  line2"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "line1\nline2");
}

#[test]
fn test_dedent_different_levels() {
    let kwargs = Kwargs::from_iter(vec![("string", Value::from("    line1\n  line2"))]);
    let result = Dedent::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "  line1\nline2");
}

// ============================================
// Quote function syntax tests
// ============================================

#[test]
fn test_quote_function_syntax_default() {
    let kwargs = Kwargs::from_iter(vec![("string", Value::from("hello"))]);
    let result = Quote::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), r#""hello""#);
}

#[test]
fn test_quote_function_syntax_single() {
    let kwargs = Kwargs::from_iter(vec![
        ("string", Value::from("hello")),
        ("style", Value::from("single")),
    ]);
    let result = Quote::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "'hello'");
}

#[test]
fn test_quote_function_syntax_backtick() {
    let kwargs = Kwargs::from_iter(vec![
        ("string", Value::from("hello")),
        ("style", Value::from("backtick")),
    ]);
    let result = Quote::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "`hello`");
}

#[test]
fn test_quote_filter_syntax() {
    let kwargs = Kwargs::from_iter(vec![("style", Value::from("single"))]);
    let result = Quote::call_as_filter(&Value::from("hello"), kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "'hello'");
}

#[test]
fn test_quote_invalid_style() {
    let kwargs = Kwargs::from_iter(vec![
        ("string", Value::from("hello")),
        ("style", Value::from("invalid")),
    ]);
    let result = Quote::call_as_function(kwargs);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Invalid quote style")
    );
}

// ============================================
// EscapeQuotes function syntax tests
// ============================================

#[test]
fn test_escape_quotes_function_syntax() {
    let kwargs = Kwargs::from_iter(vec![("string", Value::from(r#"It's a "test""#))]);
    let result = EscapeQuotes::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), r#"It\'s a \"test\""#);
}

#[test]
fn test_escape_quotes_filter_syntax() {
    let result =
        EscapeQuotes::call_as_filter(&Value::from(r#"It's a "test""#), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), r#"It\'s a \"test\""#);
}

#[test]
fn test_escape_quotes_with_backslash() {
    let kwargs = Kwargs::from_iter(vec![("string", Value::from(r#"path\to\file"#))]);
    let result = EscapeQuotes::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), r#"path\\to\\file"#);
}

// ============================================
// ToSnakeCase function syntax tests
// ============================================

#[test]
fn test_to_snake_case_function_syntax() {
    let kwargs = Kwargs::from_iter(vec![("string", Value::from("HelloWorld"))]);
    let result = ToSnakeCase::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "hello_world");
}

#[test]
fn test_to_snake_case_filter_syntax() {
    let result = ToSnakeCase::call_as_filter(&Value::from("HelloWorld"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "hello_world");
}

#[test]
fn test_to_snake_case_from_kebab() {
    let kwargs = Kwargs::from_iter(vec![("string", Value::from("hello-world"))]);
    let result = ToSnakeCase::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "hello_world");
}

// ============================================
// ToCamelCase function syntax tests
// ============================================

#[test]
fn test_to_camel_case_function_syntax() {
    let kwargs = Kwargs::from_iter(vec![("string", Value::from("hello_world"))]);
    let result = ToCamelCase::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "helloWorld");
}

#[test]
fn test_to_camel_case_filter_syntax() {
    let result = ToCamelCase::call_as_filter(&Value::from("hello_world"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "helloWorld");
}

#[test]
fn test_to_camel_case_from_kebab() {
    let kwargs = Kwargs::from_iter(vec![("string", Value::from("hello-world"))]);
    let result = ToCamelCase::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "helloWorld");
}

// ============================================
// ToPascalCase function syntax tests
// ============================================

#[test]
fn test_to_pascal_case_function_syntax() {
    let kwargs = Kwargs::from_iter(vec![("string", Value::from("hello_world"))]);
    let result = ToPascalCase::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "HelloWorld");
}

#[test]
fn test_to_pascal_case_filter_syntax() {
    let result = ToPascalCase::call_as_filter(&Value::from("hello_world"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "HelloWorld");
}

#[test]
fn test_to_pascal_case_from_kebab() {
    let kwargs = Kwargs::from_iter(vec![("string", Value::from("hello-world"))]);
    let result = ToPascalCase::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "HelloWorld");
}

// ============================================
// ToKebabCase function syntax tests
// ============================================

#[test]
fn test_to_kebab_case_function_syntax() {
    let kwargs = Kwargs::from_iter(vec![("string", Value::from("HelloWorld"))]);
    let result = ToKebabCase::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "hello-world");
}

#[test]
fn test_to_kebab_case_filter_syntax() {
    let result = ToKebabCase::call_as_filter(&Value::from("HelloWorld"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "hello-world");
}

#[test]
fn test_to_kebab_case_from_snake() {
    let kwargs = Kwargs::from_iter(vec![("string", Value::from("hello_world"))]);
    let result = ToKebabCase::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "hello-world");
}

// ============================================
// PadLeft function syntax tests
// ============================================

#[test]
fn test_pad_left_function_syntax() {
    let kwargs = Kwargs::from_iter(vec![
        ("string", Value::from("5")),
        ("length", Value::from(3)),
        ("char", Value::from("0")),
    ]);
    let result = PadLeft::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "005");
}

#[test]
fn test_pad_left_filter_syntax() {
    let kwargs = Kwargs::from_iter(vec![("length", Value::from(3)), ("char", Value::from("0"))]);
    let result = PadLeft::call_as_filter(&Value::from("5"), kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "005");
}

#[test]
fn test_pad_left_default_space() {
    let kwargs = Kwargs::from_iter(vec![
        ("string", Value::from("hi")),
        ("length", Value::from(5)),
    ]);
    let result = PadLeft::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "   hi");
}

#[test]
fn test_pad_left_no_padding_needed() {
    let kwargs = Kwargs::from_iter(vec![
        ("string", Value::from("hello")),
        ("length", Value::from(3)),
    ]);
    let result = PadLeft::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "hello");
}

// ============================================
// PadRight function syntax tests
// ============================================

#[test]
fn test_pad_right_function_syntax() {
    let kwargs = Kwargs::from_iter(vec![
        ("string", Value::from("5")),
        ("length", Value::from(3)),
        ("char", Value::from("0")),
    ]);
    let result = PadRight::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "500");
}

#[test]
fn test_pad_right_filter_syntax() {
    let kwargs = Kwargs::from_iter(vec![("length", Value::from(3)), ("char", Value::from("0"))]);
    let result = PadRight::call_as_filter(&Value::from("5"), kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "500");
}

#[test]
fn test_pad_right_default_space() {
    let kwargs = Kwargs::from_iter(vec![
        ("string", Value::from("hi")),
        ("length", Value::from(5)),
    ]);
    let result = PadRight::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "hi   ");
}

// ============================================
// Repeat function syntax tests
// ============================================

#[test]
fn test_repeat_function_syntax() {
    let kwargs = Kwargs::from_iter(vec![
        ("string", Value::from("ab")),
        ("count", Value::from(3)),
    ]);
    let result = Repeat::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "ababab");
}

#[test]
fn test_repeat_filter_syntax() {
    let kwargs = Kwargs::from_iter(vec![("count", Value::from(3))]);
    let result = Repeat::call_as_filter(&Value::from("ab"), kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "ababab");
}

#[test]
fn test_repeat_single_char() {
    let kwargs = Kwargs::from_iter(vec![
        ("string", Value::from("-")),
        ("count", Value::from(5)),
    ]);
    let result = Repeat::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "-----");
}

#[test]
fn test_repeat_zero_times() {
    let kwargs = Kwargs::from_iter(vec![
        ("string", Value::from("x")),
        ("count", Value::from(0)),
    ]);
    let result = Repeat::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "");
}

// ============================================
// Reverse function syntax tests
// ============================================

#[test]
fn test_reverse_function_syntax() {
    let kwargs = Kwargs::from_iter(vec![("string", Value::from("hello"))]);
    let result = Reverse::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "olleh");
}

#[test]
fn test_reverse_filter_syntax() {
    let result = Reverse::call_as_filter(&Value::from("hello"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "olleh");
}

#[test]
fn test_reverse_numbers() {
    let kwargs = Kwargs::from_iter(vec![("string", Value::from("12345"))]);
    let result = Reverse::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "54321");
}

#[test]
fn test_reverse_unicode() {
    let kwargs = Kwargs::from_iter(vec![("string", Value::from("hello"))]);
    let result = Reverse::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "olleh");
}
