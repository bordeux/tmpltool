mod common;

use common::{cleanup_test_file, get_test_file_path, read_fixture_expected, read_fixture_template};
use std::env;
use std::fs;
use tmpltool::render_template;

#[test]
fn test_multiline_template() {
    let output_path = get_test_file_path("output_multiline.txt");

    unsafe {
        env::set_var("LINE1", "First");
        env::set_var("LINE2", "Second");
    }

    // Read template from fixtures
    let template_content = read_fixture_template("multiline.tmpl");
    let template_path = get_test_file_path("template_multiline.txt");
    fs::write(&template_path, template_content).unwrap();

    // Run the function
    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
    );

    // Verify success
    assert!(result.is_ok());

    // Verify output matches expected
    let output = fs::read_to_string(&output_path).unwrap();
    let expected = read_fixture_expected("multiline.txt");
    assert_eq!(output, expected);

    // Cleanup
    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
    unsafe {
        env::remove_var("LINE1");
        env::remove_var("LINE2");
    }
}
