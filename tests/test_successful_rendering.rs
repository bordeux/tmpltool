mod common;

use common::{cleanup_test_file, get_test_file_path};
use std::env;
use std::fs;
use tmpltool::render_template;

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
