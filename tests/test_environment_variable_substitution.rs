mod common;

use common::{cleanup_test_file, get_test_file_path};
use std::env;
use std::fs;
use tmpltool::render_template;

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
