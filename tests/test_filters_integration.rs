mod common;

use common::{cleanup_test_file, get_test_file_path};
use std::fs;
use tmpltool::render_template;

#[test]
fn test_slugify_filter_in_template() {
    let template_content = r#"{{ "Hello World!" | slugify }}"#;
    let template_path = get_test_file_path("template_slugify.txt");
    let output_path = get_test_file_path("output_slugify.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "hello-world");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_filesizeformat_filter_in_template() {
    let template_content = r#"{{ 1048576 | filesizeformat }}"#;
    let template_path = get_test_file_path("template_filesize.txt");
    let output_path = get_test_file_path("output_filesize.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "1 MB");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_urlencode_filter_in_template() {
    let template_content = r#"{{ "hello world & foo=bar" | urlencode }}"#;
    let template_path = get_test_file_path("template_urlencode.txt");
    let output_path = get_test_file_path("output_urlencode.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "hello%20world%20%26%20foo%3Dbar");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_multiple_filters_chained() {
    let template_content = r#"{{ "Hello World" | slugify | upper }}"#;
    let template_path = get_test_file_path("template_chained.txt");
    let output_path = get_test_file_path("output_chained.txt");

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
fn test_filter_with_variable() {
    unsafe {
        std::env::set_var("TEST_FILTER_VAR", "test value");
    }

    let template_content = r#"{{ get_env(name="TEST_FILTER_VAR") | slugify }}"#;
    let template_path = get_test_file_path("template_filter_var.txt");
    let output_path = get_test_file_path("output_filter_var.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "test-value");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);

    unsafe {
        std::env::remove_var("TEST_FILTER_VAR");
    }
}

#[test]
fn test_filesizeformat_with_different_sizes() {
    let template_content = r#"Small: {{ 500 | filesizeformat }}
KB: {{ 2048 | filesizeformat }}
MB: {{ 5242880 | filesizeformat }}
GB: {{ 1073741824 | filesizeformat }}"#;
    let template_path = get_test_file_path("template_filesizes.txt");
    let output_path = get_test_file_path("output_filesizes.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert!(output.contains("Small: 500 bytes"));
    assert!(output.contains("KB: 2 KB"));
    assert!(output.contains("MB: 5 MB"));
    assert!(output.contains("GB: 1 GB"));

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_filter_in_loop() {
    let template_content = r#"{% for item in ["hello world", "foo bar", "test case"] %}
{{ item | slugify }}
{% endfor %}"#;
    let template_path = get_test_file_path("template_filter_loop.txt");
    let output_path = get_test_file_path("output_filter_loop.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert!(output.contains("hello-world"));
    assert!(output.contains("foo-bar"));
    assert!(output.contains("test-case"));

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}
