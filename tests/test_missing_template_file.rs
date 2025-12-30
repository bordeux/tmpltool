mod common;

use common::{cleanup_test_file, get_test_file_path};
use tmpltool::render_template;

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
