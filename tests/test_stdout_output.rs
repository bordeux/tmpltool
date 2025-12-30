mod common;

use common::{cleanup_test_file, get_test_file_path};
use std::env;
use std::fs;
use tmpltool::render_template;

#[test]
fn test_stdout_output() {
    let template_path = get_test_file_path("template_stdout.txt");

    unsafe {
        env::set_var("TMPLTOOL_STDOUT_TEST", "stdout_value");
    }

    // Create template using get_env()
    let template_content = r#"Output: {{ get_env(name="TMPLTOOL_STDOUT_TEST") }}"#;
    fs::write(&template_path, template_content).unwrap();

    // Run the function with no output file (should print to stdout)
    let result = render_template(Some(template_path.to_str().unwrap()), None, false);

    // Verify success
    assert!(result.is_ok());

    // Cleanup
    cleanup_test_file(&template_path);
    unsafe {
        env::remove_var("TMPLTOOL_STDOUT_TEST");
    }
}
