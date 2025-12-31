mod common;

use common::{cleanup_test_file, get_test_file_path};
use std::fs;
use tmpltool::render_template;

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
        false,
        None,
    );

    // Verify it fails (get_env() without default should error on missing var)
    assert!(result.is_err());

    // Cleanup
    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}
