use std::env;
use std::fs;
use std::path::PathBuf;
use tmpltool::render_template;

fn get_test_file_path(name: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!("tmpltool_integration_test_{}", name));
    path
}

fn cleanup_test_file(path: &PathBuf) {
    let _ = fs::remove_file(path);
}

#[test]
fn test_successful_rendering() {
    let template_path = get_test_file_path("template_success.txt");
    let output_path = get_test_file_path("output_success.txt");

    unsafe {
        env::set_var("TEST_USER_VAR", "TestUser");
    }

    // Create test template using get_env() function
    let template_content = r#"Hello {{ get_env(name="TEST_USER_VAR") }}!"#;
    fs::write(&template_path, template_content).unwrap();

    // Run the function
    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
    );

    // Verify success
    assert!(result.is_ok());

    // Verify output file exists and contains expected content
    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "Hello TestUser!");

    // Cleanup
    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
    unsafe {
        env::remove_var("TEST_USER_VAR");
    }
}

#[test]
fn test_missing_template_file() {
    let template_path = get_test_file_path("nonexistent_template.txt");
    let output_path = get_test_file_path("output_missing.txt");

    // Ensure template doesn't exist
    cleanup_test_file(&template_path);

    // Run the function
    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
    );

    // Verify error
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("Failed to read template file"));

    // Cleanup
    cleanup_test_file(&output_path);
}

#[test]
fn test_invalid_template_syntax() {
    let template_path = get_test_file_path("template_invalid.txt");
    let output_path = get_test_file_path("output_invalid.txt");

    // Create invalid template with unclosed tag
    let template_content = "Hello {{ USER ";
    fs::write(&template_path, template_content).unwrap();

    // Run the function
    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
    );

    // Verify error
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("Failed to parse template")
            || error_msg.contains("Failed to render template")
    );

    // Cleanup
    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_environment_variable_substitution() {
    let template_path = get_test_file_path("template_env.txt");
    let output_path = get_test_file_path("output_env.txt");

    // Set a test environment variable
    unsafe {
        env::set_var("TMPLTOOL_TEST_VAR", "test_value_123");
    }

    // Create template using get_env() function
    let template_content = r#"Test: {{ get_env(name="TMPLTOOL_TEST_VAR") }}"#;
    fs::write(&template_path, template_content).unwrap();

    // Run the function
    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
    );

    // Verify success
    assert!(result.is_ok());

    // Verify output contains the environment variable value
    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "Test: test_value_123");

    // Cleanup
    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
    unsafe {
        env::remove_var("TMPLTOOL_TEST_VAR");
    }
}

#[test]
fn test_template_with_conditionals() {
    let template_path = get_test_file_path("template_conditional.txt");
    let output_path = get_test_file_path("output_conditional.txt");

    // Set test variables
    unsafe {
        env::set_var("TMPLTOOL_COND_TRUE", "yes");
    }

    // Create template with conditional using get_env()
    let template_content =
        r#"{% if get_env(name="TMPLTOOL_COND_TRUE", default="no") == "yes" %}Variable is set{% else %}Not set{% endif %}"#;
    fs::write(&template_path, template_content).unwrap();

    // Run the function
    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
    );

    // Verify success
    assert!(result.is_ok());

    // Verify conditional worked
    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "Variable is set");

    // Cleanup
    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
    unsafe {
        env::remove_var("TMPLTOOL_COND_TRUE");
    }
}

#[test]
fn test_template_with_missing_variable() {
    let template_path = get_test_file_path("template_missing_var.txt");
    let output_path = get_test_file_path("output_missing_var.txt");

    // Create template with get_env() without default - should error
    let template_content = r#"Value: {{ get_env(name="TMPLTOOL_NONEXISTENT_VAR_XYZ123") }}"#;
    fs::write(&template_path, template_content).unwrap();

    // Run the function
    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
    );

    // Verify it fails (get_env() without default should error on missing var)
    assert!(result.is_err());

    // Cleanup
    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_multiline_template() {
    let template_path = get_test_file_path("template_multiline.txt");
    let output_path = get_test_file_path("output_multiline.txt");

    unsafe {
        env::set_var("TMPLTOOL_LINE1", "First");
        env::set_var("TMPLTOOL_LINE2", "Second");
    }

    // Create multiline template using get_env()
    let template_content =
        r#"Line 1: {{ get_env(name="TMPLTOOL_LINE1") }}
Line 2: {{ get_env(name="TMPLTOOL_LINE2") }}"#;
    fs::write(&template_path, template_content).unwrap();

    // Run the function
    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
    );

    // Verify success
    assert!(result.is_ok());

    // Verify multiline output
    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "Line 1: First\nLine 2: Second");

    // Cleanup
    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
    unsafe {
        env::remove_var("TMPLTOOL_LINE1");
        env::remove_var("TMPLTOOL_LINE2");
    }
}

#[test]
fn test_stdout_output() {
    let template_path = get_test_file_path("template_stdout.txt");

    unsafe {
        env::set_var("TMPLTOOL_STDOUT_TEST", "stdout_value");
    }

    // Create template using get_env()
    let template_content = r#"Output: {{ get_env(name="TMPLTOOL_STDOUT_TEST") }}"#;
    fs::write(&template_path, template_content).unwrap();

    // Run the function with no output file (should print to stdout)
    let result = render_template(Some(template_path.to_str().unwrap()), None);

    // Verify success
    assert!(result.is_ok());

    // Cleanup
    cleanup_test_file(&template_path);
    unsafe {
        env::remove_var("TMPLTOOL_STDOUT_TEST");
    }
}

#[test]
fn test_direct_var_access_fails() {
    let template_path = get_test_file_path("template_direct_var.txt");
    let output_path = get_test_file_path("output_direct_var.txt");

    unsafe {
        env::set_var("TEST_DIRECT_VAR", "value");
    }

    // Try to use variable directly without get_env() - should fail
    let template_content = "{{ TEST_DIRECT_VAR }}";
    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
    );

    // Should fail because env vars not auto-added to context
    assert!(result.is_err());

    // Cleanup
    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
    unsafe {
        env::remove_var("TEST_DIRECT_VAR");
    }
}
