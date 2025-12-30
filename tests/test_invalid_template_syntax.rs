mod common;

use common::{cleanup_test_file, get_test_file_path, read_fixture_template};
use std::fs;
use tmpltool::render_template;

#[test]
fn test_invalid_template_syntax() {
    let output_path = get_test_file_path("output_invalid.txt");

    // Read invalid template from fixtures
    let template_content = read_fixture_template("invalid.tmpl");
    let template_path = get_test_file_path("template_invalid.txt");
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
