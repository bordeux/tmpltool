mod common;

use common::{cleanup_test_file, get_test_file_path, read_fixture_expected, read_fixture_template};
use std::env;
use std::fs;
use tmpltool::render_template;

#[test]
fn test_template_with_conditionals() {
    let output_path = get_test_file_path("output_conditional.txt");

    // Set test variables
    unsafe {
        env::set_var("CONDITION", "yes");
    }

    // Read template from fixtures
    let template_content = read_fixture_template("conditionals.tmpl");
    let template_path = get_test_file_path("template_conditional.txt");
    fs::write(&template_path, template_content).unwrap();

    // Run the function
    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
    );

    // Verify success
    assert!(result.is_ok());

    // Verify output matches expected
    let output = fs::read_to_string(&output_path).unwrap();
    let expected = read_fixture_expected("conditionals_true.txt");
    assert_eq!(output, expected);

    // Cleanup
    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
    unsafe {
        env::remove_var("CONDITION");
    }
}
