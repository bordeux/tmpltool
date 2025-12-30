mod common;

use common::{cleanup_test_file, get_test_file_path};
use std::env;
use std::fs;
use tmpltool::render_template;

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
