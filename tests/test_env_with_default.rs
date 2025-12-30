mod common;

use common::{cleanup_test_file, get_test_file_path, read_fixture_expected, read_fixture_template};
use std::fs;
use tmpltool::render_template;

#[test]
fn test_env_with_default() {
    let output_path = get_test_file_path("output_with_default.txt");

    // Read template from fixtures - uses default value
    let template_content = read_fixture_template("with_default.tmpl");
    let template_path = get_test_file_path("template_with_default.txt");
    fs::write(&template_path, template_content).unwrap();

    // Run the function (no env var set, should use default)
    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
    );

    // Verify success
    assert!(result.is_ok());

    // Verify output matches expected
    let output = fs::read_to_string(&output_path).unwrap();
    let expected = read_fixture_expected("with_default.txt");
    assert_eq!(output, expected);

    // Cleanup
    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}
