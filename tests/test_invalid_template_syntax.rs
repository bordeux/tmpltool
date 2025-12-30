mod common;

use common::{cleanup_test_file, get_test_file_path};
use std::fs;
use tmpltool::render_template;

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
