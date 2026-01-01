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
