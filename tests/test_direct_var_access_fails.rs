mod common;

use common::{cleanup_test_file, get_test_file_path, read_fixture_template};
use std::env;
use std::fs;
use tmpltool::render_template;

#[test]
fn test_direct_var_access_fails() {
    let output_path = get_test_file_path("output_direct_var.txt");

    unsafe {
        env::set_var("DIRECT_VAR", "value");
    }

    // Read template from fixtures - tries direct var access without get_env()
    let template_content = read_fixture_template("direct_var.tmpltool");
    let template_path = get_test_file_path("template_direct_var.txt");
    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    // Should fail because env vars not auto-added to context
    assert!(result.is_err());

    // Cleanup
    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
    unsafe {
        env::remove_var("DIRECT_VAR");
    }
}
