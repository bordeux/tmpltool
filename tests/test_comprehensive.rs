mod common;

use common::{cleanup_test_file, get_test_file_path, read_fixture_expected, read_fixture_template};
use regex::Regex;
use std::env;
use std::fs;
use tmpltool::render_template;

#[test]
fn test_comprehensive_template() {
    // Set all test environment variables
    unsafe {
        // Basic variables with defaults
        env::set_var("TEST_PORT", "3000");
        env::set_var("TEST_HOST", "testhost");
        env::set_var("TEST_DEBUG", "true");

        // Required variable without default
        env::set_var("TEST_API_KEY", "secret-key-123");

        // Variables for filter_env pattern TEST_SERVER_*
        env::set_var("TEST_SERVER_HOST", "server1.example.com");
        env::set_var("TEST_SERVER_PORT", "9000");

        // String for testing filters
        env::set_var("TEST_NAME", "jane smith");

        // Items for array operations
        env::set_var("TEST_ITEMS", "mango,orange,pineapple");

        // Feature flag for conditionals
        env::set_var("TEST_FEATURE_FLAG", "true");

        // Database variables for filter_env pattern TEST_DB_*
        env::set_var("TEST_DB_HOST", "db.example.com");
        env::set_var("TEST_DB_PORT", "5432");
        env::set_var("TEST_DB_NAME", "myapp");

        // Tags for complex operations
        env::set_var("TEST_TAGS", "python,docker,k8s");

        // Environment type for nested conditionals
        env::set_var("TEST_ENV", "production");

        // Service name
        env::set_var("TEST_SERVICE", "api");
    }

    let output_path = get_test_file_path("output_comprehensive.txt");
    let template_content = read_fixture_template("comprehensive.tmpl");
    let template_path = get_test_file_path("template_comprehensive.txt");
    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
    );

    assert!(
        result.is_ok(),
        "Template rendering failed: {:?}",
        result.err()
    );

    // Read actual output and expected output
    let actual_output = fs::read_to_string(&output_path).unwrap();
    let expected_output = read_fixture_expected("comprehensive.txt");

    // Replace non-deterministic values with patterns for comparison
    let actual_normalized = normalize_output(&actual_output);
    let expected_normalized = normalize_output(&expected_output);

    // Compare the outputs
    assert_eq!(
        actual_normalized, expected_normalized,
        "Template output does not match expected output"
    );

    // Additional validation for dynamic content
    validate_timestamp(&actual_output);
    validate_random_numbers(&actual_output);

    // Cleanup
    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);

    unsafe {
        env::remove_var("TEST_PORT");
        env::remove_var("TEST_HOST");
        env::remove_var("TEST_DEBUG");
        env::remove_var("TEST_API_KEY");
        env::remove_var("TEST_SERVER_HOST");
        env::remove_var("TEST_SERVER_PORT");
        env::remove_var("TEST_NAME");
        env::remove_var("TEST_ITEMS");
        env::remove_var("TEST_FEATURE_FLAG");
        env::remove_var("TEST_DB_HOST");
        env::remove_var("TEST_DB_PORT");
        env::remove_var("TEST_DB_NAME");
        env::remove_var("TEST_TAGS");
        env::remove_var("TEST_ENV");
        env::remove_var("TEST_SERVICE");
    }
}

/// Normalize output by replacing non-deterministic values with placeholders
fn normalize_output(output: &str) -> String {
    // Replace ISO 8601 timestamp format
    let timestamp_re =
        Regex::new(r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d+[+-]\d{2}:\d{2}").unwrap();
    let normalized = timestamp_re.replace_all(output, "__TIMESTAMP__");

    // Replace random number in "Random (1-10): X" pattern
    let random_line_re = Regex::new(r"Random \(1-10\): \d+").unwrap();
    let normalized = random_line_re.replace_all(&normalized, "Random (1-10): __RANDOM__");

    // Replace container random suffix (100-999)
    let container_re = Regex::new(r"api-container-\d{3}").unwrap();
    let normalized = container_re.replace_all(&normalized, "api-container-__RANDOM__");

    normalized.to_string()
}

/// Validate that timestamp is in correct format (ISO 8601)
fn validate_timestamp(output: &str) {
    let timestamp_re =
        Regex::new(r"Current timestamp: \d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d+[+-]\d{2}:\d{2}")
            .unwrap();
    assert!(
        timestamp_re.is_match(output),
        "Timestamp not found or in wrong format in output"
    );
}

/// Validate that random numbers are within expected ranges
fn validate_random_numbers(output: &str) {
    // Check random number (1-10)
    let random_re = Regex::new(r"Random \(1-10\): (\d+)").unwrap();
    if let Some(caps) = random_re.captures(output) {
        let random_num: i32 = caps[1].parse().unwrap();
        assert!(
            (1..=10).contains(&random_num),
            "Random number {} is not in range 1-10",
            random_num
        );
    }

    // Check container random suffix (100-999)
    let container_re = Regex::new(r"api-container-(\d+)").unwrap();
    if let Some(caps) = container_re.captures(output) {
        let random_num: i32 = caps[1].parse().unwrap();
        assert!(
            (100..=999).contains(&random_num),
            "Container random number {} is not in range 100-999",
            random_num
        );
    }
}

#[test]
fn test_comprehensive_template_validates_all_functions() {
    // This test just ensures all the functions we claim to support actually work
    // Set all environment variables to match the main test to avoid conflicts
    unsafe {
        env::set_var("TEST_PORT", "3000");
        env::set_var("TEST_HOST", "testhost");
        env::set_var("TEST_DEBUG", "true");
        env::set_var("TEST_API_KEY", "secret-key-123");
        env::set_var("TEST_SERVER_HOST", "server1.example.com");
        env::set_var("TEST_SERVER_PORT", "9000");
        env::set_var("TEST_NAME", "jane smith");
        env::set_var("TEST_ITEMS", "mango,orange,pineapple");
        env::set_var("TEST_FEATURE_FLAG", "true");
        env::set_var("TEST_DB_HOST", "db.example.com");
        env::set_var("TEST_DB_PORT", "5432");
        env::set_var("TEST_DB_NAME", "myapp");
        env::set_var("TEST_TAGS", "python,docker,k8s");
        env::set_var("TEST_ENV", "production");
        env::set_var("TEST_SERVICE", "api");
    }

    let output_path = get_test_file_path("output_validation.txt");
    let template_content = read_fixture_template("comprehensive.tmpl");
    let template_path = get_test_file_path("template_validation.txt");
    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
    );

    assert!(
        result.is_ok(),
        "Comprehensive template should render without errors"
    );

    let output = fs::read_to_string(&output_path).unwrap();

    // Verify all sections are present
    assert!(
        output.contains("Built-in get_env() function"),
        "Missing get_env section"
    );
    assert!(
        output.contains("Custom filter_env() function"),
        "Missing filter_env section"
    );
    assert!(
        output.contains("Built-in filters"),
        "Missing filters section"
    );
    assert!(output.contains("String filters:"), "Missing string filters");
    assert!(output.contains("Array filters:"), "Missing array filters");
    assert!(
        output.contains("Conditionals with environment variables"),
        "Missing conditionals"
    );
    assert!(output.contains("Loops with filter_env"), "Missing loops");
    assert!(
        output.contains("Complex operations"),
        "Missing complex operations"
    );
    assert!(output.contains("Current timestamp:"), "Missing timestamp");
    assert!(output.contains("Random (1-10):"), "Missing random number");

    // Verify comments are NOT in output
    assert!(
        !output.contains("This is a comment"),
        "Comments should not appear in output"
    );
    assert!(
        !output.contains("Multi-line comment"),
        "Multi-line comments should not appear"
    );

    // Verify specific function outputs
    assert!(output.contains("Upper: "), "upper filter not working");
    assert!(output.contains("Lower: "), "lower filter not working");
    assert!(output.contains("Title: "), "title filter not working");
    assert!(output.contains("Slugified: "), "slugify filter not working");
    assert!(
        output.contains("Trimmed: spaces"),
        "trim filter not working"
    );
    assert!(output.contains("Items count:"), "length filter not working");
    assert!(
        output.contains("File size:"),
        "filesizeformat filter not working"
    );
    assert!(output.contains("Encoded:"), "urlencode filter not working");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);

    unsafe {
        env::remove_var("TEST_PORT");
        env::remove_var("TEST_HOST");
        env::remove_var("TEST_DEBUG");
        env::remove_var("TEST_API_KEY");
        env::remove_var("TEST_SERVER_HOST");
        env::remove_var("TEST_SERVER_PORT");
        env::remove_var("TEST_NAME");
        env::remove_var("TEST_ITEMS");
        env::remove_var("TEST_FEATURE_FLAG");
        env::remove_var("TEST_DB_HOST");
        env::remove_var("TEST_DB_PORT");
        env::remove_var("TEST_DB_NAME");
        env::remove_var("TEST_TAGS");
        env::remove_var("TEST_ENV");
        env::remove_var("TEST_SERVICE");
    }
}
