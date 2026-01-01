use minijinja::Environment;
use std::path::PathBuf;
use tmpltool::{TemplateContext, functions::register_all};

fn render_template(template: &str) -> String {
    let mut env = Environment::new();
    let ctx = TemplateContext::new(PathBuf::from("."), false);
    register_all(&mut env, ctx);

    let tmpl = env.template_from_str(template).unwrap();
    tmpl.render(()).unwrap()
}

// ==================== regex_replace Tests ====================

#[test]
fn test_regex_replace_basic() {
    let result = render_template(
        r#"{{ regex_replace(string="hello123world", pattern="[0-9]+", replacement="-") }}"#,
    );
    assert_eq!(result, "hello-world");
}

#[test]
fn test_regex_replace_whitespace() {
    let result = render_template(
        r#"{{ regex_replace(string="foo bar baz", pattern="\\s+", replacement="_") }}"#,
    );
    assert_eq!(result, "foo_bar_baz");
}

#[test]
fn test_regex_replace_no_match() {
    let result = render_template(
        r#"{{ regex_replace(string="hello", pattern="[0-9]+", replacement="-") }}"#,
    );
    assert_eq!(result, "hello");
}

#[test]
fn test_regex_replace_capture_groups() {
    let result = render_template(
        r#"{{ regex_replace(string="hello world", pattern="(\\w+) (\\w+)", replacement="$2 $1") }}"#,
    );
    assert_eq!(result, "world hello");
}

// ==================== regex_match Tests ====================

#[test]
fn test_regex_match_found() {
    let result = render_template(r#"{{ regex_match(string="hello123", pattern="[0-9]+") }}"#);
    assert_eq!(result, "true");
}

#[test]
fn test_regex_match_not_found() {
    let result = render_template(r#"{{ regex_match(string="hello", pattern="[0-9]+") }}"#);
    assert_eq!(result, "false");
}

#[test]
fn test_regex_match_email() {
    let result = render_template(
        r#"{{ regex_match(string="test@example.com", pattern="^[\\w.-]+@[\\w.-]+\\.\\w+$") }}"#,
    );
    assert_eq!(result, "true");
}

// ==================== regex_find_all Tests ====================

#[test]
fn test_regex_find_all_numbers() {
    let result =
        render_template(r#"{{ regex_find_all(string="a1b2c3", pattern="[0-9]+") | tojson }}"#);
    assert_eq!(result, r#"["1","2","3"]"#);
}

#[test]
fn test_regex_find_all_words() {
    let result =
        render_template(r#"{{ regex_find_all(string="hello world", pattern="\\w+") | tojson }}"#);
    assert_eq!(result, r#"["hello","world"]"#);
}

#[test]
fn test_regex_find_all_no_match() {
    let result =
        render_template(r#"{{ regex_find_all(string="hello", pattern="[0-9]+") | tojson }}"#);
    assert_eq!(result, "[]");
}

// ==================== substring Tests ====================

#[test]
fn test_substring_with_length() {
    let result = render_template(r#"{{ substring(string="hello world", start=0, length=5) }}"#);
    assert_eq!(result, "hello");
}

#[test]
fn test_substring_without_length() {
    let result = render_template(r#"{{ substring(string="hello world", start=6) }}"#);
    assert_eq!(result, "world");
}

#[test]
fn test_substring_negative_start() {
    let result = render_template(r#"{{ substring(string="hello world", start=-5) }}"#);
    assert_eq!(result, "world");
}

#[test]
fn test_substring_out_of_bounds() {
    let result = render_template(r#"{{ substring(string="hello", start=10) }}"#);
    assert_eq!(result, "");
}

// ==================== contains Tests ====================

#[test]
fn test_contains_found() {
    let result = render_template(r#"{{ contains(string="hello world", substring="world") }}"#);
    assert_eq!(result, "true");
}

#[test]
fn test_contains_not_found() {
    let result = render_template(r#"{{ contains(string="hello world", substring="foo") }}"#);
    assert_eq!(result, "false");
}

#[test]
fn test_contains_empty_substring() {
    let result = render_template(r#"{{ contains(string="hello", substring="") }}"#);
    assert_eq!(result, "true");
}

// ==================== index_of Tests ====================

#[test]
fn test_index_of_found() {
    let result = render_template(r#"{{ index_of(string="hello world", substring="world") }}"#);
    assert_eq!(result, "6");
}

#[test]
fn test_index_of_not_found() {
    let result = render_template(r#"{{ index_of(string="hello world", substring="foo") }}"#);
    assert_eq!(result, "-1");
}

#[test]
fn test_index_of_at_start() {
    let result = render_template(r#"{{ index_of(string="hello", substring="hello") }}"#);
    assert_eq!(result, "0");
}

// ==================== count_occurrences Tests ====================

#[test]
fn test_count_occurrences_multiple() {
    let result = render_template(
        r#"{{ count_occurrences(string="hello hello hello", substring="hello") }}"#,
    );
    assert_eq!(result, "3");
}

#[test]
fn test_count_occurrences_none() {
    let result = render_template(r#"{{ count_occurrences(string="hello", substring="world") }}"#);
    assert_eq!(result, "0");
}

#[test]
fn test_count_occurrences_single_char() {
    let result = render_template(r#"{{ count_occurrences(string="aaa", substring="a") }}"#);
    assert_eq!(result, "3");
}

// ==================== truncate Tests ====================

#[test]
fn test_truncate_with_ellipsis() {
    let result = render_template(r#"{{ truncate(string="Hello World", length=8) }}"#);
    assert_eq!(result, "Hello...");
}

#[test]
fn test_truncate_custom_suffix() {
    let result = render_template(r#"{{ truncate(string="Hello World", length=8, suffix=">>") }}"#);
    assert_eq!(result, "Hello >>");
}

#[test]
fn test_truncate_no_truncation_needed() {
    let result = render_template(r#"{{ truncate(string="Hi", length=10) }}"#);
    assert_eq!(result, "Hi");
}

#[test]
fn test_truncate_exact_length() {
    let result = render_template(r#"{{ truncate(string="Hello", length=5) }}"#);
    assert_eq!(result, "Hello");
}

// ==================== word_count Tests ====================

#[test]
fn test_word_count_simple() {
    let result = render_template(r#"{{ word_count(string="Hello World") }}"#);
    assert_eq!(result, "2");
}

#[test]
fn test_word_count_multiple_spaces() {
    let result = render_template(r#"{{ word_count(string="  one   two   three  ") }}"#);
    assert_eq!(result, "3");
}

#[test]
fn test_word_count_empty() {
    let result = render_template(r#"{{ word_count(string="") }}"#);
    assert_eq!(result, "0");
}

#[test]
fn test_word_count_single() {
    let result = render_template(r#"{{ word_count(string="hello") }}"#);
    assert_eq!(result, "1");
}

// ==================== split_lines Tests ====================

#[test]
fn test_split_lines_basic() {
    let result = render_template("{{ split_lines(string=\"line1\\nline2\\nline3\") | tojson }}");
    assert_eq!(result, r#"["line1","line2","line3"]"#);
}

#[test]
fn test_split_lines_single() {
    let result = render_template(r#"{{ split_lines(string="single line") | tojson }}"#);
    assert_eq!(result, r#"["single line"]"#);
}

#[test]
fn test_split_lines_empty() {
    let result = render_template(r#"{{ split_lines(string="") | tojson }}"#);
    assert_eq!(result, "[]");
}

// ==================== Error Cases ====================

#[test]
fn test_regex_replace_invalid_pattern() {
    let mut env = Environment::new();
    let ctx = TemplateContext::new(PathBuf::from("."), false);
    register_all(&mut env, ctx);

    let tmpl = env
        .template_from_str(
            r#"{{ regex_replace(string="test", pattern="[invalid", replacement="x") }}"#,
        )
        .unwrap();
    let result = tmpl.render(());
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Invalid regex"));
}

#[test]
fn test_count_occurrences_empty_substring() {
    let mut env = Environment::new();
    let ctx = TemplateContext::new(PathBuf::from("."), false);
    register_all(&mut env, ctx);

    let tmpl = env
        .template_from_str(r#"{{ count_occurrences(string="test", substring="") }}"#)
        .unwrap();
    let result = tmpl.render(());
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("cannot be empty"));
}

// ==================== wrap Tests ====================

#[test]
fn test_wrap_basic() {
    let result = render_template(
        r#"{{ wrap(string="The quick brown fox jumps over the lazy dog", width=20) }}"#,
    );
    assert!(result.contains("The quick brown fox"));
    assert!(result.contains("\n"));
}

#[test]
fn test_wrap_with_indent() {
    let result =
        render_template(r#"{{ wrap(string="Hello World Example Test", width=10, indent="  ") }}"#);
    assert!(result.contains("  "));
}

#[test]
fn test_wrap_single_word() {
    let result = render_template(r#"{{ wrap(string="Hello", width=20) }}"#);
    assert_eq!(result, "Hello");
}

#[test]
fn test_wrap_zero_width_error() {
    let mut env = Environment::new();
    let ctx = TemplateContext::new(PathBuf::from("."), false);
    register_all(&mut env, ctx);

    let tmpl = env
        .template_from_str(r#"{{ wrap(string="test", width=0) }}"#)
        .unwrap();
    let result = tmpl.render(());
    assert!(result.is_err());
}

// ==================== center Tests ====================

#[test]
fn test_center_basic() {
    let result = render_template(r#"{{ center(string="hello", width=11) }}"#);
    assert_eq!(result, "   hello   ");
}

#[test]
fn test_center_custom_char() {
    let result = render_template(r#"{{ center(string="hi", width=10, char="-") }}"#);
    assert_eq!(result, "----hi----");
}

#[test]
fn test_center_string_longer_than_width() {
    let result = render_template(r#"{{ center(string="hello world", width=5) }}"#);
    assert_eq!(result, "hello world");
}

#[test]
fn test_center_odd_padding() {
    let result = render_template(r#"{{ center(string="hi", width=7, char="*") }}"#);
    assert_eq!(result, "**hi***");
}

// ==================== sentence_case Tests ====================

#[test]
fn test_sentence_case_lowercase() {
    let result = render_template(r#"{{ sentence_case(string="hello world") }}"#);
    assert_eq!(result, "Hello world");
}

#[test]
fn test_sentence_case_uppercase() {
    let result = render_template(r#"{{ sentence_case(string="HELLO WORLD") }}"#);
    assert_eq!(result, "Hello world");
}

#[test]
fn test_sentence_case_mixed() {
    let result = render_template(r#"{{ sentence_case(string="hELLO wORLD") }}"#);
    assert_eq!(result, "Hello world");
}

#[test]
fn test_sentence_case_empty() {
    let result = render_template(r#"{{ sentence_case(string="") }}"#);
    assert_eq!(result, "");
}

// ==================== strip_html Tests ====================

#[test]
fn test_strip_html_basic() {
    let result = render_template(r#"{{ strip_html(string="<p>Hello World</p>") }}"#);
    assert_eq!(result, "Hello World");
}

#[test]
fn test_strip_html_nested() {
    let result = render_template(r#"{{ strip_html(string="<p>Hello <b>World</b></p>") }}"#);
    assert_eq!(result, "Hello World");
}

#[test]
fn test_strip_html_with_attributes() {
    let result = render_template(r#"{{ strip_html(string="<div class='test'>Content</div>") }}"#);
    assert_eq!(result, "Content");
}

#[test]
fn test_strip_html_no_tags() {
    let result = render_template(r#"{{ strip_html(string="No tags here") }}"#);
    assert_eq!(result, "No tags here");
}

// ==================== strip_ansi Tests ====================

#[test]
fn test_strip_ansi_color() {
    let result = render_template(r#"{{ strip_ansi(string="\x1b[31mRed\x1b[0m") }}"#);
    assert_eq!(result, "Red");
}

#[test]
fn test_strip_ansi_bold() {
    let result = render_template(r#"{{ strip_ansi(string="\x1b[1mBold\x1b[0m") }}"#);
    assert_eq!(result, "Bold");
}

#[test]
fn test_strip_ansi_no_codes() {
    let result = render_template(r#"{{ strip_ansi(string="Plain text") }}"#);
    assert_eq!(result, "Plain text");
}

// ==================== normalize_whitespace Tests ====================

#[test]
fn test_normalize_whitespace_spaces() {
    let result = render_template(r#"{{ normalize_whitespace(string="  hello   world  ") }}"#);
    assert_eq!(result, "hello world");
}

#[test]
fn test_normalize_whitespace_tabs() {
    let result = render_template("{{ normalize_whitespace(string=\"hello\\t\\tworld\") }}");
    assert_eq!(result, "hello world");
}

#[test]
fn test_normalize_whitespace_newlines() {
    let result = render_template("{{ normalize_whitespace(string=\"line1\\n\\nline2\") }}");
    assert_eq!(result, "line1 line2");
}

#[test]
fn test_normalize_whitespace_mixed() {
    let result = render_template("{{ normalize_whitespace(string=\"  a  \\t b \\n c  \") }}");
    assert_eq!(result, "a b c");
}

// ==================== to_constant_case Tests ====================

#[test]
fn test_to_constant_case_spaces() {
    let result = render_template(r#"{{ to_constant_case(string="hello world") }}"#);
    assert_eq!(result, "HELLO_WORLD");
}

#[test]
fn test_to_constant_case_camel() {
    let result = render_template(r#"{{ to_constant_case(string="helloWorld") }}"#);
    assert_eq!(result, "HELLO_WORLD");
}

#[test]
fn test_to_constant_case_kebab() {
    let result = render_template(r#"{{ to_constant_case(string="hello-world-test") }}"#);
    assert_eq!(result, "HELLO_WORLD_TEST");
}

#[test]
fn test_to_constant_case_snake() {
    let result = render_template(r#"{{ to_constant_case(string="hello_world") }}"#);
    assert_eq!(result, "HELLO_WORLD");
}

#[test]
fn test_to_constant_case_empty() {
    let result = render_template(r#"{{ to_constant_case(string="") }}"#);
    assert_eq!(result, "");
}

// ==================== pluralize Tests ====================

#[test]
fn test_pluralize_singular() {
    let result = render_template(r#"{{ pluralize(count=1, singular="item") }}"#);
    assert_eq!(result, "item");
}

#[test]
fn test_pluralize_plural_default() {
    let result = render_template(r#"{{ pluralize(count=5, singular="item") }}"#);
    assert_eq!(result, "items");
}

#[test]
fn test_pluralize_zero() {
    let result = render_template(r#"{{ pluralize(count=0, singular="item") }}"#);
    assert_eq!(result, "items");
}

#[test]
fn test_pluralize_custom_plural() {
    let result =
        render_template(r#"{{ pluralize(count=2, singular="child", plural="children") }}"#);
    assert_eq!(result, "children");
}

#[test]
fn test_pluralize_custom_singular() {
    let result = render_template(r#"{{ pluralize(count=1, singular="person", plural="people") }}"#);
    assert_eq!(result, "person");
}
