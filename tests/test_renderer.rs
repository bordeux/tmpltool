use std::fs;
use std::io::Write;
use tempfile::{NamedTempFile, TempDir};
use tmpltool::cli::ValidateFormat;
use tmpltool::render_template;

// ============================================================================
// render_template() Tests - Core Functionality
// ============================================================================

#[test]
fn test_render_template_from_file_to_stdout() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "Hello World").unwrap();
    let path = temp_file.path().to_str().unwrap();

    // This will output to stdout, we just verify it doesn't error
    let result = render_template(Some(path), None, false, None);
    assert!(result.is_ok());
}

#[test]
fn test_render_template_from_file_to_file() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("input.tmpl");
    let output_path = temp_dir.path().join("output.txt");

    fs::write(&input_path, "Test output").unwrap();

    let result = render_template(
        Some(input_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());
    assert!(output_path.exists());
    let content = fs::read_to_string(&output_path).unwrap();
    assert_eq!(content, "Test output");
}

#[test]
fn test_render_template_with_env_var() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("input.tmpl");
    let output_path = temp_dir.path().join("output.txt");

    fs::write(
        &input_path,
        "{{ get_env(name=\"TEST_RENDERER_VAR\", default=\"default\") }}",
    )
    .unwrap();

    unsafe {
        std::env::set_var("TEST_RENDERER_VAR", "test_value");
    }

    let result = render_template(
        Some(input_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());
    let content = fs::read_to_string(&output_path).unwrap();
    assert_eq!(content, "test_value");

    unsafe {
        std::env::remove_var("TEST_RENDERER_VAR");
    }
}

#[test]
fn test_render_template_with_trust_mode() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("input.tmpl");
    let output_path = temp_dir.path().join("output.txt");
    let data_file = temp_dir.path().join("data.txt");

    fs::write(&data_file, "trusted data").unwrap();

    // Use relative path with parent directory traversal (requires trust mode)
    fs::write(&input_path, "{{ read_file(path=\"../data.txt\") }}").unwrap();

    // Create a subdirectory and move the template there
    let subdir = temp_dir.path().join("subdir");
    fs::create_dir(&subdir).unwrap();
    let nested_input = subdir.join("input.tmpl");
    fs::write(&nested_input, "{{ read_file(path=\"../data.txt\") }}").unwrap();

    // Should work with trust mode (accessing parent directory)
    let result = render_template(
        Some(nested_input.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        true, // trust mode enabled
        None,
    );

    assert!(result.is_ok());
    let content = fs::read_to_string(&output_path).unwrap();
    assert_eq!(content, "trusted data");
}

// ============================================================================
// Error Handling Tests
// ============================================================================

#[test]
fn test_render_template_missing_file() {
    let result = render_template(Some("/nonexistent/file.tmpl"), None, false, None);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Failed to read template file"));
}

#[test]
fn test_render_template_invalid_template_syntax() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "{{{{ unclosed_variable").unwrap();
    let path = temp_file.path().to_str().unwrap();

    let result = render_template(Some(path), None, false, None);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Failed to parse template"));
}

#[test]
fn test_render_template_undefined_variable() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "{{{{ undefined_var }}}}").unwrap();
    let path = temp_file.path().to_str().unwrap();

    let result = render_template(Some(path), None, false, None);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Failed to render template"));
}

#[test]
fn test_render_template_invalid_output_path() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "Hello").unwrap();
    let path = temp_file.path().to_str().unwrap();

    // Try to write to a directory that doesn't exist
    let result = render_template(Some(path), Some("/nonexistent/dir/output.txt"), false, None);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Failed to write output file"));
}

#[test]
#[cfg(unix)]
fn test_render_template_security_absolute_path() {
    // This test only works on Unix where absolute paths start with /
    // On Windows, the security check for absolute paths works differently
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("input.tmpl");

    // Try to read with Unix absolute path without trust mode
    fs::write(&input_path, "{{ read_file(path=\"/etc/passwd\") }}").unwrap();

    let result = render_template(Some(input_path.to_str().unwrap()), None, false, None);

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Security") || err.to_string().contains("absolute"));
}

#[test]
fn test_render_template_security_parent_directory() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("input.tmpl");

    // Try to read with parent directory traversal
    fs::write(&input_path, "{{ read_file(path=\"../secret.txt\") }}").unwrap();

    let result = render_template(Some(input_path.to_str().unwrap()), None, false, None);

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Security") || err.to_string().contains("parent"));
}

// ============================================================================
// Validation Tests
// ============================================================================

#[test]
fn test_render_template_validate_json_success() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("input.tmpl");
    let output_path = temp_dir.path().join("output.json");

    fs::write(&input_path, r#"{"valid": "json", "number": 42}"#).unwrap();

    let result = render_template(
        Some(input_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        Some(ValidateFormat::Json),
    );

    assert!(result.is_ok());
}

#[test]
fn test_render_template_validate_json_failure() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "{{invalid json}}").unwrap();
    let path = temp_file.path().to_str().unwrap();

    let result = render_template(Some(path), None, false, Some(ValidateFormat::Json));

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("JSON") || err.to_string().contains("validation"));
}

#[test]
fn test_render_template_validate_yaml_success() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("input.tmpl");
    let output_path = temp_dir.path().join("output.yaml");

    fs::write(&input_path, "server:\n  host: localhost\n  port: 8080").unwrap();

    let result = render_template(
        Some(input_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        Some(ValidateFormat::Yaml),
    );

    assert!(result.is_ok());
}

#[test]
fn test_render_template_validate_yaml_failure() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "  invalid:\nyaml: - badly formatted").unwrap();
    let path = temp_file.path().to_str().unwrap();

    let result = render_template(Some(path), None, false, Some(ValidateFormat::Yaml));

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("YAML") || err.to_string().contains("validation"));
}

#[test]
fn test_render_template_validate_toml_success() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("input.tmpl");
    let output_path = temp_dir.path().join("output.toml");

    fs::write(&input_path, "[server]\nhost = \"localhost\"\nport = 8080").unwrap();

    let result = render_template(
        Some(input_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        Some(ValidateFormat::Toml),
    );

    assert!(result.is_ok());
}

#[test]
fn test_render_template_validate_toml_failure() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "invalid = toml = syntax").unwrap();
    let path = temp_file.path().to_str().unwrap();

    let result = render_template(Some(path), None, false, Some(ValidateFormat::Toml));

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("TOML") || err.to_string().contains("validation"));
}

// ============================================================================
// Complex Scenario Tests
// ============================================================================

#[test]
fn test_render_template_with_includes() {
    let temp_dir = TempDir::new().unwrap();
    let main_path = temp_dir.path().join("main.tmpl");
    let partial_path = temp_dir.path().join("partial.tmpl");
    let output_path = temp_dir.path().join("output.txt");

    fs::write(&partial_path, "included content").unwrap();
    fs::write(&main_path, "Start {% include \"partial.tmpl\" %} End").unwrap();

    let result = render_template(
        Some(main_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());
    let content = fs::read_to_string(&output_path).unwrap();
    assert_eq!(content, "Start included content End");
}

#[test]
fn test_render_template_with_filters() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("input.tmpl");
    let output_path = temp_dir.path().join("output.txt");

    fs::write(&input_path, "{{ \"hello world\" | upper }}").unwrap();

    let result = render_template(
        Some(input_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());
    let content = fs::read_to_string(&output_path).unwrap();
    assert_eq!(content, "HELLO WORLD");
}

#[test]
fn test_render_template_with_conditionals() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("input.tmpl");
    let output_path = temp_dir.path().join("output.txt");

    fs::write(
        &input_path,
        "{% if get_env(name=\"ENABLE_FEATURE\", default=\"false\") == \"true\" %}enabled{% else %}disabled{% endif %}",
    )
    .unwrap();

    unsafe {
        std::env::set_var("ENABLE_FEATURE", "true");
    }

    let result = render_template(
        Some(input_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());
    let content = fs::read_to_string(&output_path).unwrap();
    assert_eq!(content, "enabled");

    unsafe {
        std::env::remove_var("ENABLE_FEATURE");
    }
}

#[test]
fn test_render_template_with_loops() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("input.tmpl");
    let output_path = temp_dir.path().join("output.txt");

    fs::write(&input_path, "{% for i in [1, 2, 3] %}{{ i }}{% endfor %}").unwrap();

    let result = render_template(
        Some(input_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());
    let content = fs::read_to_string(&output_path).unwrap();
    assert_eq!(content, "123");
}

#[test]
fn test_render_template_empty_file() {
    let mut temp_file = NamedTempFile::new().unwrap();
    // Write empty content
    write!(temp_file, "").unwrap();
    let path = temp_file.path().to_str().unwrap();

    let result = render_template(Some(path), None, false, None);
    // Empty templates are valid
    assert!(result.is_ok());
}

#[test]
fn test_render_template_large_template() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("input.tmpl");
    let output_path = temp_dir.path().join("output.txt");

    // Create a large template with lots of repetition
    let large_content = "{% for i in range(1000) %}Line {{ i }}\n{% endfor %}";
    fs::write(&input_path, large_content).unwrap();

    let result = render_template(
        Some(input_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());
    let content = fs::read_to_string(&output_path).unwrap();
    assert!(content.contains("Line 0"));
    assert!(content.contains("Line 999"));
}

#[test]
fn test_render_template_unicode_content() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("input.tmpl");
    let output_path = temp_dir.path().join("output.txt");

    fs::write(&input_path, "Hello 世界 🚀 café").unwrap();

    let result = render_template(
        Some(input_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());
    let content = fs::read_to_string(&output_path).unwrap();
    assert_eq!(content, "Hello 世界 🚀 café");
}
