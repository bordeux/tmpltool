mod common;

use common::{cleanup_test_file, get_test_file_path};
use std::env;
use std::fs;
use tmpltool::render_template;

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
