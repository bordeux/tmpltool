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
        None,
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
        None,
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
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "hello%20world%20%26%20foo%3Dbar");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_filesizeformat_function_syntax_in_template() {
    let template_content = r#"{{ filesizeformat(bytes=1048576) }}"#;
    let template_path = get_test_file_path("template_filesize_fn.txt");
    let output_path = get_test_file_path("output_filesize_fn.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "1 MB");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_urlencode_function_syntax_in_template() {
    let template_content = r#"{{ urlencode(string="hello world & foo=bar") }}"#;
    let template_path = get_test_file_path("template_urlencode_fn.txt");
    let output_path = get_test_file_path("output_urlencode_fn.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "hello%20world%20%26%20foo%3Dbar");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_filesizeformat_both_syntaxes() {
    // Test that both function and filter syntax produce the same result
    let template_content = r#"Filter: {{ 1073741824 | filesizeformat }}
Function: {{ filesizeformat(bytes=1073741824) }}"#;
    let template_path = get_test_file_path("template_filesize_both.txt");
    let output_path = get_test_file_path("output_filesize_both.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert!(output.contains("Filter: 1 GB"));
    assert!(output.contains("Function: 1 GB"));

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_urlencode_both_syntaxes() {
    // Test that both function and filter syntax produce the same result
    let template_content = r#"Filter: {{ "test value" | urlencode }}
Function: {{ urlencode(string="test value") }}"#;
    let template_path = get_test_file_path("template_urlencode_both.txt");
    let output_path = get_test_file_path("output_urlencode_both.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert!(output.contains("Filter: test%20value"));
    assert!(output.contains("Function: test%20value"));

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
        None,
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
        None,
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
        None,
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
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert!(output.contains("hello-world"));
    assert!(output.contains("foo-bar"));
    assert!(output.contains("test-case"));

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

// ============ Hash Filter Integration Tests (Phase 2) ============

#[test]
fn test_sha256_filter_in_template() {
    let template_content = r#"{{ "hello" | sha256 }}"#;
    let template_path = get_test_file_path("template_sha256_filter.txt");
    let output_path = get_test_file_path("output_sha256_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(
        output,
        "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
    );

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_md5_filter_in_template() {
    let template_content = r#"{{ "hello" | md5 }}"#;
    let template_path = get_test_file_path("template_md5_filter.txt");
    let output_path = get_test_file_path("output_md5_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "5d41402abc4b2a76b9719d911017c592");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_hash_filters_chained() {
    // Chain: sha256 then md5 (hash of a hash)
    let template_content = r#"{{ "hello" | sha256 | md5 }}"#;
    let template_path = get_test_file_path("template_hash_chain.txt");
    let output_path = get_test_file_path("output_hash_chain.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    // sha256("hello") = "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
    // md5 of that hash string
    assert_eq!(output, "ebde1b934fa81da163dcf4b7d7cfe18e");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_all_hash_filters() {
    let template_content = r#"MD5: {{ "test" | md5 }}
SHA1: {{ "test" | sha1 }}
SHA256: {{ "test" | sha256 }}
SHA512: {{ "test" | sha512 }}"#;
    let template_path = get_test_file_path("template_all_hashes.txt");
    let output_path = get_test_file_path("output_all_hashes.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert!(output.contains("MD5: 098f6bcd4621d373cade4e832627b4f6"));
    assert!(output.contains("SHA1: a94a8fe5ccb19ba61c4c0873d391e987982fbbd3"));
    assert!(
        output.contains("SHA256: 9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08")
    );
    assert!(output.contains("SHA512: ee26b0dd4af7e749aa1a8ee3c10ae9923f618980772e473f8819a5d4940e0db27ac185f8a0e1d5f84f88bc887fd67b143732c304cc5fa9ad8e6f57f50028a8ff"));

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

// ============ Encoding Filter Integration Tests (Phase 3) ============

#[test]
fn test_base64_encode_filter_in_template() {
    let template_content = r#"{{ "Hello World" | base64_encode }}"#;
    let template_path = get_test_file_path("template_base64_encode_filter.txt");
    let output_path = get_test_file_path("output_base64_encode_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "SGVsbG8gV29ybGQ=");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_base64_decode_filter_in_template() {
    let template_content = r#"{{ "SGVsbG8gV29ybGQ=" | base64_decode }}"#;
    let template_path = get_test_file_path("template_base64_decode_filter.txt");
    let output_path = get_test_file_path("output_base64_decode_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "Hello World");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_hex_encode_filter_in_template() {
    let template_content = r#"{{ "Hello" | hex_encode }}"#;
    let template_path = get_test_file_path("template_hex_encode_filter.txt");
    let output_path = get_test_file_path("output_hex_encode_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "48656c6c6f");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_hex_decode_filter_in_template() {
    let template_content = r#"{{ "48656c6c6f" | hex_decode }}"#;
    let template_path = get_test_file_path("template_hex_decode_filter.txt");
    let output_path = get_test_file_path("output_hex_decode_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "Hello");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_escape_html_filter_in_template() {
    let template_content = r#"{{ "<script>alert('XSS')</script>" | escape_html }}"#;
    let template_path = get_test_file_path("template_escape_html_filter.txt");
    let output_path = get_test_file_path("output_escape_html_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(
        output,
        "&lt;script&gt;alert(&#x27;XSS&#x27;)&lt;/script&gt;"
    );

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_escape_shell_filter_in_template() {
    let template_content = r#"{{ "hello world" | escape_shell }}"#;
    let template_path = get_test_file_path("template_escape_shell_filter.txt");
    let output_path = get_test_file_path("output_escape_shell_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "'hello world'");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_encoding_filters_chained() {
    // Chain: base64_encode then sha256
    let template_content = r#"{{ "hello" | base64_encode | sha256 }}"#;
    let template_path = get_test_file_path("template_encoding_chain.txt");
    let output_path = get_test_file_path("output_encoding_chain.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    // "hello" -> base64 "aGVsbG8=" -> sha256
    assert_eq!(
        output,
        "333d6b3a3c1f5db6c9bdda5939b136986d170f4649172a68368d54ecb44c2ff2"
    );

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_hash_and_encoding_filters_chained() {
    // Chain: sha256 then base64_encode
    let template_content = r#"{{ "hello" | sha256 | base64_encode }}"#;
    let template_path = get_test_file_path("template_hash_encoding_chain.txt");
    let output_path = get_test_file_path("output_hash_encoding_chain.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    // "hello" -> sha256 -> base64
    assert_eq!(
        output,
        "MmNmMjRkYmE1ZmIwYTMwZTI2ZTgzYjJhYzViOWUyOWUxYjE2MWU1YzFmYTc0MjVlNzMwNDMzNjI5MzhiOTgyNA=="
    );

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

// ============ Serialization Filter Integration Tests (Phase 4) ============

#[test]
fn test_to_json_filter_in_template() {
    // Test with inline object
    let template_content =
        r#"{% set config = {"host": "localhost", "port": 8080} %}{{ config | to_json }}"#;
    let template_path = get_test_file_path("template_to_json_filter.txt");
    let output_path = get_test_file_path("output_to_json_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    // Parse to verify it's valid JSON
    let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
    assert_eq!(parsed["host"], "localhost");
    assert_eq!(parsed["port"], 8080);

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_to_json_filter_pretty() {
    let template_content = r#"{% set data = {"key": "value"} %}{{ data | to_json(pretty=true) }}"#;
    let template_path = get_test_file_path("template_to_json_pretty.txt");
    let output_path = get_test_file_path("output_to_json_pretty.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    // Pretty JSON should contain newlines
    assert!(output.contains('\n'));
    assert!(output.contains("key"));

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_to_yaml_filter_in_template() {
    let template_content =
        r#"{% set config = {"host": "localhost", "port": 8080} %}{{ config | to_yaml }}"#;
    let template_path = get_test_file_path("template_to_yaml_filter.txt");
    let output_path = get_test_file_path("output_to_yaml_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert!(output.contains("host: localhost"));
    assert!(output.contains("port: 8080"));

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_to_toml_filter_in_template() {
    let template_content =
        r#"{% set config = {"title": "MyApp", "version": "1.0.0"} %}{{ config | to_toml }}"#;
    let template_path = get_test_file_path("template_to_toml_filter.txt");
    let output_path = get_test_file_path("output_to_toml_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert!(output.contains("title = \"MyApp\""));
    assert!(output.contains("version = \"1.0.0\""));

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_parse_json_filter_in_template() {
    let template_content = r#"{% set json_str = '{"name": "Alice", "age": 30}' %}{% set obj = json_str | parse_json %}Name: {{ obj.name }}, Age: {{ obj.age }}"#;
    let template_path = get_test_file_path("template_parse_json_filter.txt");
    let output_path = get_test_file_path("output_parse_json_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "Name: Alice, Age: 30");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_parse_yaml_filter_in_template() {
    let template_content = r#"{% set yaml_str = "name: Bob
age: 25" %}{% set obj = yaml_str | parse_yaml %}Name: {{ obj.name }}, Age: {{ obj.age }}"#;
    let template_path = get_test_file_path("template_parse_yaml_filter.txt");
    let output_path = get_test_file_path("output_parse_yaml_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "Name: Bob, Age: 25");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_parse_toml_filter_in_template() {
    let template_content = r#"{% set toml_str = 'name = "Charlie"
age = 35' %}{% set obj = toml_str | parse_toml %}Name: {{ obj.name }}, Age: {{ obj.age }}"#;
    let template_path = get_test_file_path("template_parse_toml_filter.txt");
    let output_path = get_test_file_path("output_parse_toml_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "Name: Charlie, Age: 35");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_serialization_roundtrip_json_yaml() {
    // Convert object to JSON, then parse it and convert to YAML
    let template_content = r#"{% set obj = {"key": "value"} %}{% set json_str = obj | to_json %}{% set parsed = json_str | parse_json %}{{ parsed | to_yaml }}"#;
    let template_path = get_test_file_path("template_roundtrip_json_yaml.txt");
    let output_path = get_test_file_path("output_roundtrip_json_yaml.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert!(output.contains("key: value"));

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_serialization_with_array() {
    let template_content = r#"{% set items = [1, 2, 3, 4, 5] %}{{ items | to_json }}"#;
    let template_path = get_test_file_path("template_array_to_json.txt");
    let output_path = get_test_file_path("output_array_to_json.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "[1,2,3,4,5]");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_parse_json_with_nested_access() {
    let template_content = r#"{% set json_str = '{"user": {"profile": {"name": "Dave"}}}' %}{% set data = json_str | parse_json %}{{ data.user.profile.name }}"#;
    let template_path = get_test_file_path("template_parse_json_nested.txt");
    let output_path = get_test_file_path("output_parse_json_nested.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "Dave");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_all_serialization_filters() {
    let template_content = r#"{% set data = {"key": "value"} %}JSON: {{ data | to_json }}
YAML: {{ data | to_yaml | trim }}
TOML: {{ data | to_toml | trim }}"#;
    let template_path = get_test_file_path("template_all_serialization.txt");
    let output_path = get_test_file_path("output_all_serialization.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert!(output.contains("JSON: {\"key\":\"value\"}"));
    assert!(output.contains("YAML: key: value"));
    assert!(output.contains("TOML: key = \"value\""));

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

// ============ Math Filter Integration Tests (Phase 5) ============

#[test]
fn test_abs_filter_in_template() {
    let template_content = r#"{{ -42 | abs }}"#;
    let template_path = get_test_file_path("template_abs_filter.txt");
    let output_path = get_test_file_path("output_abs_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "42");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_abs_filter_with_float() {
    let template_content = r#"{{ -3.14 | abs }}"#;
    let template_path = get_test_file_path("template_abs_float_filter.txt");
    let output_path = get_test_file_path("output_abs_float_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "3.14");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_round_filter_default() {
    let template_content = r#"{{ 3.7 | round }}"#;
    let template_path = get_test_file_path("template_round_default_filter.txt");
    let output_path = get_test_file_path("output_round_default_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "4");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_round_filter_with_decimals() {
    let template_content = r#"{{ 3.14159 | round(decimals=2) }}"#;
    let template_path = get_test_file_path("template_round_decimals_filter.txt");
    let output_path = get_test_file_path("output_round_decimals_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "3.14");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_ceil_filter_in_template() {
    let template_content = r#"{{ 3.1 | ceil }}"#;
    let template_path = get_test_file_path("template_ceil_filter.txt");
    let output_path = get_test_file_path("output_ceil_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "4");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_floor_filter_in_template() {
    let template_content = r#"{{ 3.9 | floor }}"#;
    let template_path = get_test_file_path("template_floor_filter.txt");
    let output_path = get_test_file_path("output_floor_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "3");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_math_filters_chained() {
    // Chain: abs then ceil
    let template_content = r#"{{ -3.7 | abs | ceil }}"#;
    let template_path = get_test_file_path("template_math_chain.txt");
    let output_path = get_test_file_path("output_math_chain.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    // -3.7 -> 3.7 -> 4
    assert_eq!(output, "4");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_all_math_filters() {
    let template_content = r#"Abs: {{ -42 | abs }}
Round: {{ 3.567 | round(decimals=1) }}
Ceil: {{ 2.1 | ceil }}
Floor: {{ 2.9 | floor }}"#;
    let template_path = get_test_file_path("template_all_math.txt");
    let output_path = get_test_file_path("output_all_math.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert!(output.contains("Abs: 42"));
    assert!(output.contains("Round: 3.6"));
    assert!(output.contains("Ceil: 3"));
    assert!(output.contains("Floor: 2"));

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_math_in_set_statement() {
    // Using math filters in set statement
    let template_content =
        r#"{% set val = -3.7 | abs | round %}Calculated: {{ val }}, Plus 10: {{ val + 10 }}"#;
    let template_path = get_test_file_path("template_math_set.txt");
    let output_path = get_test_file_path("output_math_set.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert!(output.contains("Calculated: 4"));
    assert!(output.contains("Plus 10: 14"));

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

// ============ String Filter Integration Tests (Phase 6) ============

#[test]
fn test_regex_replace_filter_in_template() {
    let template_content =
        r#"{{ "hello123world" | regex_replace(pattern="[0-9]+", replacement="-") }}"#;
    let template_path = get_test_file_path("template_regex_replace_filter.txt");
    let output_path = get_test_file_path("output_regex_replace_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "hello-world");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_substring_filter_in_template() {
    let template_content = r#"{{ "hello world" | substring(start=0, length=5) }}"#;
    let template_path = get_test_file_path("template_substring_filter.txt");
    let output_path = get_test_file_path("output_substring_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "hello");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_truncate_filter_in_template() {
    let template_content = r#"{{ "Hello World Example" | truncate(length=10) }}"#;
    let template_path = get_test_file_path("template_truncate_filter.txt");
    let output_path = get_test_file_path("output_truncate_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "Hello W...");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_word_count_filter_in_template() {
    let template_content = r#"{{ "The quick brown fox" | word_count }}"#;
    let template_path = get_test_file_path("template_word_count_filter.txt");
    let output_path = get_test_file_path("output_word_count_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "4");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_strip_html_filter_in_template() {
    let template_content = r#"{{ "<p>Hello <b>World</b></p>" | strip_html }}"#;
    let template_path = get_test_file_path("template_strip_html_filter.txt");
    let output_path = get_test_file_path("output_strip_html_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "Hello World");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_normalize_whitespace_filter_in_template() {
    let template_content = r#"{{ "  hello   world  " | normalize_whitespace }}"#;
    let template_path = get_test_file_path("template_normalize_ws_filter.txt");
    let output_path = get_test_file_path("output_normalize_ws_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "hello world");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_center_filter_in_template() {
    let template_content = r#"{{ "hi" | center(width=10, char="-") }}"#;
    let template_path = get_test_file_path("template_center_filter.txt");
    let output_path = get_test_file_path("output_center_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "----hi----");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_string_filters_chained() {
    // Chain: strip_html then normalize_whitespace then truncate
    let template_content = r#"{{ "<p>Hello   World   Example</p>" | strip_html | normalize_whitespace | truncate(length=15) }}"#;
    let template_path = get_test_file_path("template_string_chain.txt");
    let output_path = get_test_file_path("output_string_chain.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    // strip_html: "Hello   World   Example"
    // normalize_whitespace: "Hello World Example"
    // truncate(15): "Hello World ..."
    assert_eq!(output, "Hello World ...");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_all_string_filters() {
    let template_content = r#"Truncate: {{ "Hello World" | truncate(length=8) }}
WordCount: {{ "one two three" | word_count }}
Center: {{ "hi" | center(width=6) }}
StripHtml: {{ "<b>bold</b>" | strip_html }}"#;
    let template_path = get_test_file_path("template_all_string.txt");
    let output_path = get_test_file_path("output_all_string.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert!(output.contains("Truncate: Hello..."));
    assert!(output.contains("WordCount: 3"));
    assert!(output.contains("Center:   hi  "));
    assert!(output.contains("StripHtml: bold"));

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

// ===========================================
// Phase 7: Array filter tests
// ===========================================

#[test]
fn test_array_sum_filter_in_template() {
    let template_content = r#"{% set nums = [1, 2, 3, 4, 5] %}{{ nums | array_sum }}"#;
    let template_path = get_test_file_path("template_array_sum_filter.txt");
    let output_path = get_test_file_path("output_array_sum_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "15");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_array_avg_filter_in_template() {
    let template_content = r#"{% set nums = [10, 20, 30] %}{{ nums | array_avg }}"#;
    let template_path = get_test_file_path("template_array_avg_filter.txt");
    let output_path = get_test_file_path("output_array_avg_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "20");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_array_median_filter_in_template() {
    let template_content = r#"{% set nums = [1, 3, 5, 7, 9] %}{{ nums | array_median }}"#;
    let template_path = get_test_file_path("template_array_median_filter.txt");
    let output_path = get_test_file_path("output_array_median_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "5");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_array_min_filter_in_template() {
    let template_content = r#"{% set nums = [42, 17, 99, 8, 55] %}{{ nums | array_min }}"#;
    let template_path = get_test_file_path("template_array_min_filter.txt");
    let output_path = get_test_file_path("output_array_min_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "8");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_array_max_filter_in_template() {
    let template_content = r#"{% set nums = [42, 17, 99, 8, 55] %}{{ nums | array_max }}"#;
    let template_path = get_test_file_path("template_array_max_filter.txt");
    let output_path = get_test_file_path("output_array_max_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "99");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_array_unique_filter_in_template() {
    let template_content =
        r#"{% set nums = [1, 2, 2, 3, 3, 3] %}{{ nums | array_unique | join(", ") }}"#;
    let template_path = get_test_file_path("template_array_unique_filter.txt");
    let output_path = get_test_file_path("output_array_unique_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "1, 2, 3");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_array_flatten_filter_in_template() {
    let template_content =
        r#"{% set nested = [[1, 2], [3, 4], [5]] %}{{ nested | array_flatten | join(", ") }}"#;
    let template_path = get_test_file_path("template_array_flatten_filter.txt");
    let output_path = get_test_file_path("output_array_flatten_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "1, 2, 3, 4, 5");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_array_filters_chained() {
    // Chain: unique then sum
    let template_content =
        r#"{% set nums = [1, 2, 2, 3, 3, 3] %}{{ nums | array_unique | array_sum }}"#;
    let template_path = get_test_file_path("template_array_chain.txt");
    let output_path = get_test_file_path("output_array_chain.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    // unique: [1, 2, 3], sum: 6
    assert_eq!(output, "6");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_all_array_filters() {
    let template_content = r#"{% set nums = [5, 2, 8, 2, 1] %}
Sum: {{ nums | array_sum }}
Avg: {{ nums | array_avg }}
Min: {{ nums | array_min }}
Max: {{ nums | array_max }}
Unique: {{ nums | array_unique | join(",") }}"#;
    let template_path = get_test_file_path("template_all_array.txt");
    let output_path = get_test_file_path("output_all_array.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert!(output.contains("Sum: 18"));
    assert!(output.contains("Min: 1"));
    assert!(output.contains("Max: 8"));
    assert!(output.contains("Unique: 5,2,8,1"));

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

// ============================================
// Phase 8: DateTime Filter Tests
// ============================================

#[test]
fn test_format_date_filter_in_template() {
    // 1704067200 = 2024-01-01 00:00:00 UTC
    let template_content = r#"{{ 1704067200 | format_date(format="%Y-%m-%d") }}"#;
    let template_path = get_test_file_path("template_format_date_filter.txt");
    let output_path = get_test_file_path("output_format_date_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "2024-01-01");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_get_year_filter_in_template() {
    // 1704067200 = 2024-01-01 00:00:00 UTC
    let template_content = r#"{{ 1704067200 | get_year }}"#;
    let template_path = get_test_file_path("template_get_year_filter.txt");
    let output_path = get_test_file_path("output_get_year_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "2024");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_get_month_filter_in_template() {
    // 1704067200 = 2024-01-01 00:00:00 UTC
    let template_content = r#"{{ 1704067200 | get_month }}"#;
    let template_path = get_test_file_path("template_get_month_filter.txt");
    let output_path = get_test_file_path("output_get_month_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "1");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_get_day_filter_in_template() {
    // 1704067200 = 2024-01-01 00:00:00 UTC
    let template_content = r#"{{ 1704067200 | get_day }}"#;
    let template_path = get_test_file_path("template_get_day_filter.txt");
    let output_path = get_test_file_path("output_get_day_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "1");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_get_hour_filter_in_template() {
    // 1704110400 = 2024-01-01 12:00:00 UTC
    let template_content = r#"{{ 1704110400 | get_hour }}"#;
    let template_path = get_test_file_path("template_get_hour_filter.txt");
    let output_path = get_test_file_path("output_get_hour_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "12");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_get_minute_filter_in_template() {
    // 1704068700 = 2024-01-01 00:25:00 UTC
    let template_content = r#"{{ 1704068700 | get_minute }}"#;
    let template_path = get_test_file_path("template_get_minute_filter.txt");
    let output_path = get_test_file_path("output_get_minute_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "25");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_get_second_filter_in_template() {
    // 1704067245 = 2024-01-01 00:00:45 UTC
    let template_content = r#"{{ 1704067245 | get_second }}"#;
    let template_path = get_test_file_path("template_get_second_filter.txt");
    let output_path = get_test_file_path("output_get_second_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "45");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_datetime_filters_chained_with_now() {
    // Test chaining now() with format_date filter
    let template_content = r#"{{ now() | format_date(format="%Y") }}"#;
    let template_path = get_test_file_path("template_datetime_chain.txt");
    let output_path = get_test_file_path("output_datetime_chain.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    // Should be a 4-digit year
    assert!(output.len() == 4);
    assert!(output.parse::<i32>().is_ok());

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_all_datetime_filters() {
    // 1704110445 = 2024-01-01 12:00:45 UTC
    let template_content = r#"{% set ts = 1704110445 %}
Year: {{ ts | get_year }}
Month: {{ ts | get_month }}
Day: {{ ts | get_day }}
Hour: {{ ts | get_hour }}
Minute: {{ ts | get_minute }}
Second: {{ ts | get_second }}
Formatted: {{ ts | format_date(format="%Y-%m-%d %H:%M:%S") }}"#;
    let template_path = get_test_file_path("template_all_datetime.txt");
    let output_path = get_test_file_path("output_all_datetime.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert!(output.contains("Year: 2024"));
    assert!(output.contains("Month: 1"));
    assert!(output.contains("Day: 1"));
    assert!(output.contains("Hour: 12"));
    assert!(output.contains("Minute: 0"));
    assert!(output.contains("Second: 45"));
    assert!(output.contains("Formatted: 2024-01-01 12:00:45"));

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

// ============================================
// Phase 9: Path Filter Tests
// ============================================

#[test]
fn test_basename_filter_in_template() {
    let template_content = r#"{{ "/path/to/file.txt" | basename }}"#;
    let template_path = get_test_file_path("template_basename_filter.txt");
    let output_path = get_test_file_path("output_basename_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "file.txt");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_dirname_filter_in_template() {
    let template_content = r#"{{ "/path/to/file.txt" | dirname }}"#;
    let template_path = get_test_file_path("template_dirname_filter.txt");
    let output_path = get_test_file_path("output_dirname_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "/path/to");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_file_extension_filter_in_template() {
    let template_content = r#"{{ "document.pdf" | file_extension }}"#;
    let template_path = get_test_file_path("template_file_ext_filter.txt");
    let output_path = get_test_file_path("output_file_ext_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "pdf");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_join_path_filter_in_template() {
    let template_content = r#"{{ ["path", "to", "file.txt"] | join_path }}"#;
    let template_path = get_test_file_path("template_join_path_filter.txt");
    let output_path = get_test_file_path("output_join_path_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "path/to/file.txt");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_normalize_path_filter_in_template() {
    let template_content = r#"{{ "./foo/../bar/file.txt" | normalize_path }}"#;
    let template_path = get_test_file_path("template_normalize_filter.txt");
    let output_path = get_test_file_path("output_normalize_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "bar/file.txt");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_path_filters_chained() {
    // Chain: basename then file_extension
    let template_content = r#"{{ "/path/to/archive.tar.gz" | basename | file_extension }}"#;
    let template_path = get_test_file_path("template_path_chain.txt");
    let output_path = get_test_file_path("output_path_chain.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    // archive.tar.gz -> extension is "gz"
    assert_eq!(output, "gz");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_all_path_filters() {
    let template_content = r#"{% set p = "/home/user/docs/file.txt" %}
Basename: {{ p | basename }}
Dirname: {{ p | dirname }}
Extension: {{ p | file_extension }}
Joined: {{ ["a", "b", "c"] | join_path }}
Normalized: {{ "./x/../y/z" | normalize_path }}"#;
    let template_path = get_test_file_path("template_all_path.txt");
    let output_path = get_test_file_path("output_all_path.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert!(output.contains("Basename: file.txt"));
    assert!(output.contains("Dirname: /home/user/docs"));
    assert!(output.contains("Extension: txt"));
    assert!(output.contains("Joined: a/b/c"));
    assert!(output.contains("Normalized: y/z"));

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

// ============================================
// Phase 10: URL Filter Tests
// ============================================

#[test]
fn test_url_encode_filter_in_template() {
    let template_content = r#"{{ "hello world" | url_encode }}"#;
    let template_path = get_test_file_path("template_url_encode_filter.txt");
    let output_path = get_test_file_path("output_url_encode_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "hello%20world");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_url_decode_filter_in_template() {
    let template_content = r#"{{ "hello%20world" | url_decode }}"#;
    let template_path = get_test_file_path("template_url_decode_filter.txt");
    let output_path = get_test_file_path("output_url_decode_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "hello world");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_parse_url_filter_in_template() {
    let template_content =
        r#"{% set parts = "https://example.com:8080/path?q=1" | parse_url %}{{ parts.host }}"#;
    let template_path = get_test_file_path("template_parse_url_filter.txt");
    let output_path = get_test_file_path("output_parse_url_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "example.com");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_url_encode_special_chars() {
    let template_content = r#"{{ "foo=bar&baz=qux" | url_encode }}"#;
    let template_path = get_test_file_path("template_url_encode_special.txt");
    let output_path = get_test_file_path("output_url_encode_special.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "foo%3Dbar%26baz%3Dqux");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_url_roundtrip() {
    let template_content = r#"{{ "hello world & foo=bar" | url_encode | url_decode }}"#;
    let template_path = get_test_file_path("template_url_roundtrip.txt");
    let output_path = get_test_file_path("output_url_roundtrip.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "hello world & foo=bar");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_all_url_filters() {
    let template_content = r#"{% set url = "https://user:pass@example.com:8080/path?query=value#section" %}
{% set parts = url | parse_url %}
Scheme: {{ parts.scheme }}
Host: {{ parts.host }}
Port: {{ parts.port }}
Path: {{ parts.path }}
Query: {{ parts.query }}
Encoded: {{ "hello world" | url_encode }}
Decoded: {{ "hello%20world" | url_decode }}"#;
    let template_path = get_test_file_path("template_all_url.txt");
    let output_path = get_test_file_path("output_all_url.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert!(output.contains("Scheme: https"));
    assert!(output.contains("Host: example.com"));
    assert!(output.contains("Port: 8080"));
    assert!(output.contains("Path: /path"));
    assert!(output.contains("Query: query=value"));
    assert!(output.contains("Encoded: hello%20world"));
    assert!(output.contains("Decoded: hello world"));

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

// =============================================================================
// Phase 11: Object filter tests
// =============================================================================

#[test]
fn test_object_keys_filter_in_template() {
    let template_content = r#"{% set config = {"host": "localhost", "port": 8080} %}{{ config | object_keys | join(",") }}"#;
    let template_path = get_test_file_path("template_object_keys_filter.txt");
    let output_path = get_test_file_path("output_object_keys_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    // Keys may be in any order, so check both are present
    assert!(output.contains("host"));
    assert!(output.contains("port"));

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_object_values_filter_in_template() {
    let template_content = r#"{% set config = {"host": "localhost", "port": 8080} %}{{ config | object_values | join(",") }}"#;
    let template_path = get_test_file_path("template_object_values_filter.txt");
    let output_path = get_test_file_path("output_object_values_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    // Values may be in any order
    assert!(output.contains("localhost"));
    assert!(output.contains("8080"));

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_object_flatten_filter_in_template() {
    let template_content = r#"{% set nested = {"server": {"host": "localhost", "port": 8080}} %}{% set flat = nested | object_flatten %}{{ flat["server.host"] }}"#;
    let template_path = get_test_file_path("template_object_flatten_filter.txt");
    let output_path = get_test_file_path("output_object_flatten_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "localhost");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_object_flatten_with_delimiter() {
    let template_content = r#"{% set nested = {"server": {"host": "localhost"}} %}{% set flat = nested | object_flatten(delimiter="_") %}{{ flat["server_host"] }}"#;
    let template_path = get_test_file_path("template_object_flatten_delim.txt");
    let output_path = get_test_file_path("output_object_flatten_delim.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "localhost");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_object_filters_chained() {
    let template_content =
        r#"{% set config = {"a": 1, "b": 2, "c": 3} %}{{ config | object_keys | length }}"#;
    let template_path = get_test_file_path("template_object_chained.txt");
    let output_path = get_test_file_path("output_object_chained.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "3");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_all_object_filters() {
    let template_content = r#"{% set config = {"server": {"host": "localhost", "port": 8080}, "debug": true} %}
Keys: {{ config | object_keys | length }}
Values: {{ config | object_values | length }}
Flat keys: {{ config | object_flatten | object_keys | join(",") }}"#;
    let template_path = get_test_file_path("template_all_object.txt");
    let output_path = get_test_file_path("output_all_object.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert!(output.contains("Keys: 2"));
    assert!(output.contains("Values: 2"));
    assert!(output.contains("server.host"));
    assert!(output.contains("server.port"));
    assert!(output.contains("debug"));

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

// =============================================================================
// Phase 12: Kubernetes filter tests
// =============================================================================

#[test]
fn test_k8s_label_safe_filter_in_template() {
    let template_content = r#"{{ "My App (v2.0)" | k8s_label_safe }}"#;
    let template_path = get_test_file_path("template_k8s_label_safe_filter.txt");
    let output_path = get_test_file_path("output_k8s_label_safe_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "my-app-v2.0");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_k8s_dns_label_safe_filter_in_template() {
    let template_content = r#"{{ "My Service Name" | k8s_dns_label_safe }}"#;
    let template_path = get_test_file_path("template_k8s_dns_label_safe_filter.txt");
    let output_path = get_test_file_path("output_k8s_dns_label_safe_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "my-service-name");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_k8s_annotation_safe_filter_in_template() {
    let template_content = r#"{{ "Description with	tabs and
newlines" | k8s_annotation_safe }}"#;
    let template_path = get_test_file_path("template_k8s_annotation_safe_filter.txt");
    let output_path = get_test_file_path("output_k8s_annotation_safe_filter.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "Description with tabs and newlines");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_k8s_label_safe_long_string() {
    let template_content = r#"{{ "this-is-a-very-long-label-name-that-exceeds-the-kubernetes-maximum-label-length-limit" | k8s_label_safe }}"#;
    let template_path = get_test_file_path("template_k8s_label_long.txt");
    let output_path = get_test_file_path("output_k8s_label_long.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    // Should be truncated to 63 chars max
    assert!(output.len() <= 63);
    assert!(output.starts_with("this-is-a-very-long"));

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_k8s_label_safe_empty_result() {
    let template_content = r#"{{ "!!@@##$$" | k8s_label_safe }}"#;
    let template_path = get_test_file_path("template_k8s_label_empty.txt");
    let output_path = get_test_file_path("output_k8s_label_empty.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    // Should return "default" when all chars are invalid
    assert_eq!(output, "default");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_all_k8s_filters() {
    let template_content = r#"{% set app_name = "My App (v2.0)" %}
{% set service = "My Service Name" %}
{% set desc = "Line 1
Line 2" %}
Label: {{ app_name | k8s_label_safe }}
DNS: {{ service | k8s_dns_label_safe }}
Annotation: {{ desc | k8s_annotation_safe }}"#;
    let template_path = get_test_file_path("template_all_k8s.txt");
    let output_path = get_test_file_path("output_all_k8s.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert!(output.contains("Label: my-app-v2.0"));
    assert!(output.contains("DNS: my-service-name"));
    assert!(output.contains("Annotation: Line 1 Line 2"));

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

// ============================================
// Phase 2: String filter-function tests (function syntax)
// ============================================

#[test]
fn test_slugify_function_syntax_in_template() {
    let template_content = r#"{{ slugify(string="Hello World!") }}"#;
    let template_path = get_test_file_path("template_slugify_fn.txt");
    let output_path = get_test_file_path("output_slugify_fn.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "hello-world");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_indent_function_syntax_in_template() {
    let template_content = r#"{{ indent(string="hello", spaces=2) }}"#;
    let template_path = get_test_file_path("template_indent_fn.txt");
    let output_path = get_test_file_path("output_indent_fn.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "  hello");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_dedent_function_syntax_in_template() {
    let template_content = r#"{{ dedent(string="  line1") }}"#;
    let template_path = get_test_file_path("template_dedent_fn.txt");
    let output_path = get_test_file_path("output_dedent_fn.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "line1");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_quote_function_syntax_in_template() {
    let template_content = r#"{{ quote(string="hello", style="single") }}"#;
    let template_path = get_test_file_path("template_quote_fn.txt");
    let output_path = get_test_file_path("output_quote_fn.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "'hello'");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_escape_quotes_function_syntax_in_template() {
    let template_content = r#"{{ escape_quotes(string="It's test") }}"#;
    let template_path = get_test_file_path("template_escape_fn.txt");
    let output_path = get_test_file_path("output_escape_fn.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, r#"It\'s test"#);

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_to_snake_case_function_syntax_in_template() {
    let template_content = r#"{{ to_snake_case(string="HelloWorld") }}"#;
    let template_path = get_test_file_path("template_snake_fn.txt");
    let output_path = get_test_file_path("output_snake_fn.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "hello_world");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_to_camel_case_function_syntax_in_template() {
    let template_content = r#"{{ to_camel_case(string="hello_world") }}"#;
    let template_path = get_test_file_path("template_camel_fn.txt");
    let output_path = get_test_file_path("output_camel_fn.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "helloWorld");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_to_pascal_case_function_syntax_in_template() {
    let template_content = r#"{{ to_pascal_case(string="hello_world") }}"#;
    let template_path = get_test_file_path("template_pascal_fn.txt");
    let output_path = get_test_file_path("output_pascal_fn.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "HelloWorld");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_to_kebab_case_function_syntax_in_template() {
    let template_content = r#"{{ to_kebab_case(string="HelloWorld") }}"#;
    let template_path = get_test_file_path("template_kebab_fn.txt");
    let output_path = get_test_file_path("output_kebab_fn.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "hello-world");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_pad_left_function_syntax_in_template() {
    let template_content = r#"{{ pad_left(string="5", length=3, char="0") }}"#;
    let template_path = get_test_file_path("template_padleft_fn.txt");
    let output_path = get_test_file_path("output_padleft_fn.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "005");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_pad_right_function_syntax_in_template() {
    let template_content = r#"{{ pad_right(string="5", length=3, char="0") }}"#;
    let template_path = get_test_file_path("template_padright_fn.txt");
    let output_path = get_test_file_path("output_padright_fn.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "500");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_repeat_function_syntax_in_template() {
    let template_content = r#"{{ repeat(string="ab", count=3) }}"#;
    let template_path = get_test_file_path("template_repeat_fn.txt");
    let output_path = get_test_file_path("output_repeat_fn.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "ababab");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_reverse_function_syntax_in_template() {
    let template_content = r#"{{ reverse(string="hello") }}"#;
    let template_path = get_test_file_path("template_reverse_fn.txt");
    let output_path = get_test_file_path("output_reverse_fn.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "olleh");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}
