mod common;

use common::{cleanup_test_file, get_test_file_path};
use std::env;
use std::fs;
use tmpltool::render_template;

#[test]
fn test_multiline_template() {
    let template_path = get_test_file_path("template_multiline.txt");
    let output_path = get_test_file_path("output_multiline.txt");

    unsafe {
        env::set_var("TMPLTOOL_LINE1", "First");
        env::set_var("TMPLTOOL_LINE2", "Second");
    }

    // Create multiline template using get_env()
    let template_content = r#"Line 1: {{ get_env(name="TMPLTOOL_LINE1") }}
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
