//! Tests for the run() function in lib.rs
//!
//! These tests exercise the CLI argument parsing and template rendering
//! through the public run() function, providing coverage for the main
//! entry point logic.

use std::fs;
use std::io::Write;
use tempfile::TempDir;
use tmpltool::run;

/// Helper to create a temporary template file
fn create_template(dir: &TempDir, name: &str, content: &str) -> String {
    let path = dir.path().join(name);
    let mut file = fs::File::create(&path).unwrap();
    write!(file, "{}", content).unwrap();
    path.to_string_lossy().to_string()
}

#[test]
fn test_run_with_template_file() {
    let temp_dir = TempDir::new().unwrap();
    let template_path = create_template(&temp_dir, "test.tmpl", "Hello World");

    let result = run(["tmpltool", &template_path]);
    assert!(result.is_ok());
}

#[test]
fn test_run_with_output_file() {
    let temp_dir = TempDir::new().unwrap();
    let template_path = create_template(&temp_dir, "test.tmpl", "Hello Output");
    let output_path = temp_dir.path().join("output.txt");

    let result = run([
        "tmpltool",
        &template_path,
        "-o",
        output_path.to_str().unwrap(),
    ]);

    assert!(result.is_ok());
    assert!(output_path.exists());

    let content = fs::read_to_string(&output_path).unwrap();
    assert_eq!(content, "Hello Output");
}

#[test]
fn test_run_with_long_output_flag() {
    let temp_dir = TempDir::new().unwrap();
    let template_path = create_template(&temp_dir, "test.tmpl", "Long flag test");
    let output_path = temp_dir.path().join("output.txt");

    let result = run([
        "tmpltool",
        &template_path,
        "--output",
        output_path.to_str().unwrap(),
    ]);

    assert!(result.is_ok());
    assert!(output_path.exists());
}

#[test]
fn test_run_missing_template_file() {
    let result = run(["tmpltool", "/nonexistent/path/template.tmpl"]);
    assert!(result.is_err());

    let err = result.unwrap_err().to_string();
    // Error message differs by OS: Unix uses "No such file", Windows uses "cannot find"
    assert!(
        err.contains("No such file") || err.contains("cannot find"),
        "Unexpected error message: {}",
        err
    );
}

#[test]
fn test_run_invalid_template_syntax() {
    let temp_dir = TempDir::new().unwrap();
    let template_path = create_template(&temp_dir, "invalid.tmpl", "{{ invalid syntax }}");

    let result = run(["tmpltool", &template_path]);
    assert!(result.is_err());
}

#[test]
fn test_run_with_trust_flag() {
    let temp_dir = TempDir::new().unwrap();
    let template_path = create_template(&temp_dir, "test.tmpl", "Trust mode enabled");

    let result = run(["tmpltool", "--trust", &template_path]);
    assert!(result.is_ok());
}

#[test]
fn test_run_with_validate_json() {
    let temp_dir = TempDir::new().unwrap();
    let template_path = create_template(
        &temp_dir,
        "test.tmpl",
        r#"{{ to_json(object={"name": "test"}) }}"#,
    );

    let result = run(["tmpltool", &template_path, "--validate", "json"]);
    assert!(result.is_ok());
}

#[test]
fn test_run_with_validate_json_invalid() {
    let temp_dir = TempDir::new().unwrap();
    let template_path = create_template(&temp_dir, "test.tmpl", "not valid json {");

    let result = run(["tmpltool", &template_path, "--validate", "json"]);
    assert!(result.is_err());

    let err = result.unwrap_err();
    assert!(err.to_string().contains("JSON validation failed"));
}

#[test]
fn test_run_with_validate_yaml() {
    let temp_dir = TempDir::new().unwrap();
    let template_path = create_template(&temp_dir, "test.tmpl", "name: test\nvalue: 42");

    let result = run(["tmpltool", &template_path, "--validate", "yaml"]);
    assert!(result.is_ok());
}

#[test]
fn test_run_with_validate_toml() {
    let temp_dir = TempDir::new().unwrap();
    let template_path = create_template(&temp_dir, "test.tmpl", "[section]\nkey = \"value\"");

    let result = run(["tmpltool", &template_path, "--validate", "toml"]);
    assert!(result.is_ok());
}

#[test]
fn test_run_with_env_function() {
    let temp_dir = TempDir::new().unwrap();
    let template_path = create_template(
        &temp_dir,
        "test.tmpl",
        r#"{{ get_env(name="PATH", default="no-path") }}"#,
    );

    let result = run(["tmpltool", &template_path]);
    assert!(result.is_ok());
}

#[test]
fn test_run_help_flag() {
    // --help causes clap to print help and return an error
    let result = run(["tmpltool", "--help"]);
    assert!(result.is_err());

    // The error should be from clap (help is treated as early exit)
    let err = result.unwrap_err();
    // Clap help error contains the help text
    assert!(err.to_string().contains("Usage") || err.to_string().contains("tmpltool"));
}

#[test]
fn test_run_version_flag() {
    // --version causes clap to print version and return an error
    let result = run(["tmpltool", "--version"]);
    assert!(result.is_err());
}

#[test]
fn test_run_invalid_flag() {
    let result = run(["tmpltool", "--invalid-flag"]);
    assert!(result.is_err());

    let err = result.unwrap_err();
    assert!(err.to_string().contains("unexpected argument"));
}

#[test]
fn test_run_with_template_using_include() {
    let temp_dir = TempDir::new().unwrap();

    // Create a partial template
    create_template(&temp_dir, "partial.tmpl", "Included content");

    // Create main template that includes the partial
    let template_path = create_template(
        &temp_dir,
        "main.tmpl",
        r#"Before {% include "partial.tmpl" %} After"#,
    );

    let result = run(["tmpltool", &template_path]);
    assert!(result.is_ok());
}

#[test]
fn test_run_combined_flags() {
    let temp_dir = TempDir::new().unwrap();
    let template_path = create_template(
        &temp_dir,
        "test.tmpl",
        r#"{{ to_json(object={"valid": true}) }}"#,
    );
    let output_path = temp_dir.path().join("output.json");

    let result = run([
        "tmpltool",
        "--trust",
        &template_path,
        "--output",
        output_path.to_str().unwrap(),
        "--validate",
        "json",
    ]);

    assert!(result.is_ok());
    assert!(output_path.exists());
}

#[test]
fn test_run_with_multiline_template() {
    let temp_dir = TempDir::new().unwrap();
    let template_content = r#"Line 1
Line 2
{% for i in range(3) %}
Item {{ i }}
{% endfor %}
End"#;
    let template_path = create_template(&temp_dir, "multiline.tmpl", template_content);

    let result = run(["tmpltool", &template_path]);
    assert!(result.is_ok());
}

#[test]
fn test_run_empty_template() {
    let temp_dir = TempDir::new().unwrap();
    let template_path = create_template(&temp_dir, "empty.tmpl", "");

    let result = run(["tmpltool", &template_path]);
    assert!(result.is_ok());
}

#[test]
fn test_run_template_with_conditionals() {
    let temp_dir = TempDir::new().unwrap();
    let template_path = create_template(
        &temp_dir,
        "conditional.tmpl",
        r#"{% if true %}Yes{% else %}No{% endif %}"#,
    );

    let result = run(["tmpltool", &template_path]);
    assert!(result.is_ok());
}

#[test]
fn test_run_output_to_nested_directory() {
    let temp_dir = TempDir::new().unwrap();
    let template_path = create_template(&temp_dir, "test.tmpl", "Nested output");

    // Create nested directory
    let nested_dir = temp_dir.path().join("nested").join("deep");
    fs::create_dir_all(&nested_dir).unwrap();

    let output_path = nested_dir.join("output.txt");

    let result = run([
        "tmpltool",
        &template_path,
        "-o",
        output_path.to_str().unwrap(),
    ]);

    assert!(result.is_ok());
    assert!(output_path.exists());
}
