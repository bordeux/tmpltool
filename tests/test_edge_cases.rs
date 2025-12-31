mod common;

use common::{cleanup_test_file, get_test_file_path};
use std::fs;
use tmpltool::render_template;

#[test]
fn test_empty_template() {
    let template_content = "";
    let template_path = get_test_file_path("template_empty.txt");
    let output_path = get_test_file_path("output_empty.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_template_only_comments() {
    let template_content = r#"{# This is a comment #}
{# Another comment #}"#;
    let template_path = get_test_file_path("template_comments.txt");
    let output_path = get_test_file_path("output_comments.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output.trim(), "");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_template_with_unicode() {
    let template_content = "Hello 世界 🌍\n{{ get_env(name='TEST_UNICODE', default='默认值') }}";
    let template_path = get_test_file_path("template_unicode.txt");
    let output_path = get_test_file_path("output_unicode.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert!(output.contains("Hello 世界 🌍"));
    assert!(output.contains("默认值"));

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_very_long_template() {
    // Create a template with 1000 lines
    let mut template_content = String::new();
    for i in 0..1000 {
        template_content.push_str(&format!("Line {}\n", i));
    }

    let template_path = get_test_file_path("template_long.txt");
    let output_path = get_test_file_path("output_long.txt");

    fs::write(&template_path, &template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output.lines().count(), 1000);
    assert!(output.contains("Line 0"));
    assert!(output.contains("Line 999"));

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_nested_loops() {
    let template_content = r#"{% for i in range(3) %}
  {% for j in range(3) %}
    {{ i }}-{{ j }}
  {% endfor %}
{% endfor %}"#;
    let template_path = get_test_file_path("template_nested.txt");
    let output_path = get_test_file_path("output_nested.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert!(output.contains("0-0"));
    assert!(output.contains("0-1"));
    assert!(output.contains("0-2"));
    assert!(output.contains("2-2"));

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_complex_conditionals() {
    unsafe {
        std::env::set_var("TEST_COND_A", "true");
        std::env::set_var("TEST_COND_B", "false");
    }

    let template_content = r#"{% set a = get_env(name="TEST_COND_A") %}
{% set b = get_env(name="TEST_COND_B") %}
{% if a == "true" and b == "false" %}
PASS
{% else %}
FAIL
{% endif %}"#;
    let template_path = get_test_file_path("template_cond.txt");
    let output_path = get_test_file_path("output_cond.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert!(output.contains("PASS"));
    assert!(!output.contains("FAIL"));

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);

    unsafe {
        std::env::remove_var("TEST_COND_A");
        std::env::remove_var("TEST_COND_B");
    }
}

#[test]
fn test_special_characters_in_output() {
    let template_content = r#"Special: < > & " ' \n \t"#;
    let template_path = get_test_file_path("template_special.txt");
    let output_path = get_test_file_path("output_special.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, r#"Special: < > & " ' \n \t"#);

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_function_with_default_and_filter() {
    let template_content =
        r#"{{ get_env(name="NONEXISTENT_VAR", default="hello world") | slugify | upper }}"#;
    let template_path = get_test_file_path("template_default_filter.txt");
    let output_path = get_test_file_path("output_default_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "HELLO-WORLD");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_whitespace_control() {
    let template_content = r#"{% for i in range(3) -%}
{{ i }}
{%- endfor %}"#;
    let template_path = get_test_file_path("template_whitespace.txt");
    let output_path = get_test_file_path("output_whitespace.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    // Whitespace control should make it compact
    assert_eq!(output, "012");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}
