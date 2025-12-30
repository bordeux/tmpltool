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

    // Create test template
    let template_content = "Hello {{ USER }}!";
    fs::write(&template_path, template_content).unwrap();

    // Run the function
    let result = render_template(
        template_path.to_str().unwrap(),
        Some(output_path.to_str().unwrap()),
    );

    // Verify success
    assert!(result.is_ok());

    // Verify output file exists and contains expected content
    let output = fs::read_to_string(&output_path).unwrap();
    assert!(output.starts_with("Hello "));
    assert!(output.contains("!"));

    // Cleanup
    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_missing_template_file() {
    let template_path = get_test_file_path("nonexistent_template.txt");
    let output_path = get_test_file_path("output_missing.txt");

    // Ensure template doesn't exist
    cleanup_test_file(&template_path);

    // Run the function
    let result = render_template(
        template_path.to_str().unwrap(),
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
        template_path.to_str().unwrap(),
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

    // Create template using the test env var
    let template_content = "Test: {{ TMPLTOOL_TEST_VAR }}";
    fs::write(&template_path, template_content).unwrap();

    // Run the function
    let result = render_template(
        template_path.to_str().unwrap(),
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

    // Create template with conditional
    let template_content =
        r#"{% if TMPLTOOL_COND_TRUE %}Variable is set{% else %}Not set{% endif %}"#;
    fs::write(&template_path, template_content).unwrap();

    // Run the function
    let result = render_template(
        template_path.to_str().unwrap(),
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

    // Create template with a variable that likely doesn't exist
    let template_content = "Value: {{ TMPLTOOL_NONEXISTENT_VAR_XYZ123 }}";
    fs::write(&template_path, template_content).unwrap();

    // Run the function
    let result = render_template(
        template_path.to_str().unwrap(),
        Some(output_path.to_str().unwrap()),
    );

    // Verify it fails (Tera should error on missing variables by default)
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

    // Create multiline template
    let template_content = "Line 1: {{ TMPLTOOL_LINE1 }}\nLine 2: {{ TMPLTOOL_LINE2 }}";
    fs::write(&template_path, template_content).unwrap();

    // Run the function
    let result = render_template(
        template_path.to_str().unwrap(),
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

    // Create template
    let template_content = "Output: {{ TMPLTOOL_STDOUT_TEST }}";
    fs::write(&template_path, template_content).unwrap();

    // Run the function with no output file (should print to stdout)
    let result = render_template(template_path.to_str().unwrap(), None);

    // Verify success
    assert!(result.is_ok());

    // Cleanup
    cleanup_test_file(&template_path);
    unsafe {
        env::remove_var("TMPLTOOL_STDOUT_TEST");
    }
}
