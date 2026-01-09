mod common;

use common::{cleanup_test_file, get_test_file_path, read_fixture_expected, read_fixture_template};
use std::env;
use std::fs;
use tmpltool::render_template;

#[test]
fn test_filter_env() {
    // Set test environment variables
    unsafe {
        env::set_var("SERVER_HOST", "localhost");
        env::set_var("SERVER_PORT", "8080");
        env::set_var("SERVER_NAME", "myapp");
        env::set_var("OTHER_VAR", "should_not_appear");
    }

    let output_path = get_test_file_path("output_filter_env.txt");
    let template_content = read_fixture_template("filter_env.tmpltool");
    let template_path = get_test_file_path("template_filter_env.txt");
    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_ok());
    let output = fs::read_to_string(&output_path)
        .unwrap()
        .trim_end()
        .to_string();
    let expected = read_fixture_expected("filter_env.txt")
        .trim_end()
        .to_string();
    assert_eq!(output, expected);

    // Cleanup
    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);

    unsafe {
        env::remove_var("SERVER_HOST");
        env::remove_var("SERVER_PORT");
        env::remove_var("SERVER_NAME");
        env::remove_var("OTHER_VAR");
    }
}
