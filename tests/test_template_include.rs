/// Integration tests for template include functionality
use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, Ordering};
use tmpltool::render_template;

mod common;

// Global counter for unique test directories
static TEST_COUNTER: AtomicU32 = AtomicU32::new(0);

/// Helper to create a test directory
fn setup_test_env() -> PathBuf {
    let counter = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
    let test_dir = env::temp_dir().join(format!(
        "tmpltool_include_test_{}_{}",
        std::process::id(),
        counter
    ));
    fs::create_dir_all(&test_dir).unwrap();
    test_dir
}

/// Helper to create a file in the test directory
fn create_file(dir: &Path, name: &str, content: &str) -> PathBuf {
    let file_path = dir.join(name);
    let mut file = fs::File::create(&file_path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
    file_path
}

/// Helper to cleanup test directory
fn cleanup_test_env(test_dir: &Path) {
    let _ = fs::remove_dir_all(test_dir);
}

#[test]
fn test_simple_include() {
    let test_dir = setup_test_env();

    // Create partial template
    create_file(&test_dir, "partial.tmpl", "Hello from partial!");

    // Create main template
    let main_template = create_file(
        &test_dir,
        "main.tmpl",
        "Start\n{% include \"./partial.tmpl\" %}\nEnd",
    );

    let output_file = test_dir.join("output.txt");

    // Render
    let result = render_template(
        Some(main_template.to_str().unwrap()),
        Some(output_file.to_str().unwrap()),
        false,
        None,
    );

    assert!(
        result.is_ok(),
        "Template rendering failed: {:?}",
        result.err()
    );

    // Verify output
    let output = fs::read_to_string(&output_file).unwrap();
    assert_eq!(output, "Start\nHello from partial!\nEnd");

    cleanup_test_env(&test_dir);
}

#[test]
fn test_include_with_env_vars() {
    let test_dir = setup_test_env();

    // Create partial template with env vars
    create_file(
        &test_dir,
        "partial.tmpl",
        "User: {{ get_env(name=\"TEST_USER\", default=\"guest\") }}",
    );

    // Create main template
    let main_template = create_file(
        &test_dir,
        "main.tmpl",
        "Header\n{% include \"./partial.tmpl\" %}\nFooter",
    );

    let output_file = test_dir.join("output.txt");

    // Set environment variable
    unsafe {
        env::set_var("TEST_USER", "testuser");
    }

    // Render
    let result = render_template(
        Some(main_template.to_str().unwrap()),
        Some(output_file.to_str().unwrap()),
        false,
        None,
    );

    assert!(
        result.is_ok(),
        "Template rendering failed: {:?}",
        result.err()
    );

    // Verify output
    let output = fs::read_to_string(&output_file).unwrap();
    assert_eq!(output, "Header\nUser: testuser\nFooter");

    // Cleanup env var
    unsafe {
        env::remove_var("TEST_USER");
    }
    cleanup_test_env(&test_dir);
}

#[test]
fn test_nested_includes() {
    let test_dir = setup_test_env();

    // Create level 2 template
    create_file(&test_dir, "level2.tmpl", "Level 2 content");

    // Create level 1 template that includes level 2
    create_file(
        &test_dir,
        "level1.tmpl",
        "Level 1 start\n{% include \"./level2.tmpl\" %}\nLevel 1 end",
    );

    // Create main template that includes level 1
    let main_template = create_file(
        &test_dir,
        "main.tmpl",
        "Main start\n{% include \"./level1.tmpl\" %}\nMain end",
    );

    let output_file = test_dir.join("output.txt");

    // Render
    let result = render_template(
        Some(main_template.to_str().unwrap()),
        Some(output_file.to_str().unwrap()),
        false,
        None,
    );

    assert!(
        result.is_ok(),
        "Template rendering failed: {:?}",
        result.err()
    );

    // Verify output
    let output = fs::read_to_string(&output_file).unwrap();
    assert_eq!(
        output,
        "Main start\nLevel 1 start\nLevel 2 content\nLevel 1 end\nMain end"
    );

    cleanup_test_env(&test_dir);
}

#[test]
fn test_include_with_subdirectory() {
    let test_dir = setup_test_env();

    // Create subdirectory
    let subdir = test_dir.join("partials");
    fs::create_dir_all(&subdir).unwrap();

    // Create partial in subdirectory
    create_file(&subdir, "footer.tmpl", "Footer content");

    // Create main template
    let main_template = create_file(
        &test_dir,
        "main.tmpl",
        "Main content\n{% include \"./partials/footer.tmpl\" %}",
    );

    let output_file = test_dir.join("output.txt");

    // Render
    let result = render_template(
        Some(main_template.to_str().unwrap()),
        Some(output_file.to_str().unwrap()),
        false,
        None,
    );

    assert!(
        result.is_ok(),
        "Template rendering failed: {:?}",
        result.err()
    );

    // Verify output
    let output = fs::read_to_string(&output_file).unwrap();
    assert_eq!(output, "Main content\nFooter content");

    cleanup_test_env(&test_dir);
}

#[test]
fn test_include_nonexistent_template() {
    let test_dir = setup_test_env();

    // Create main template that tries to include nonexistent file
    let main_template = create_file(
        &test_dir,
        "main.tmpl",
        "{% include \"./nonexistent.tmpl\" %}",
    );

    let output_file = test_dir.join("output.txt");

    // Render - should fail
    let result = render_template(
        Some(main_template.to_str().unwrap()),
        Some(output_file.to_str().unwrap()),
        false,
        None,
    );

    assert!(result.is_err(), "Expected error for nonexistent template");
    let error = result.unwrap_err().to_string();
    assert!(
        error.contains("Failed to load template") || error.contains("nonexistent"),
        "Error message should mention failed load or nonexistent file: {}",
        error
    );

    cleanup_test_env(&test_dir);
}

#[test]
fn test_include_parent_directory_blocked() {
    let test_dir = setup_test_env();

    // Create parent directory file
    create_file(&test_dir, "parent.tmpl", "Parent content");

    // Create subdirectory
    let subdir = test_dir.join("subdir");
    fs::create_dir_all(&subdir).unwrap();

    // Create template in subdirectory that tries to include parent
    let main_template = create_file(&subdir, "main.tmpl", "{% include \"../parent.tmpl\" %}");

    let output_file = test_dir.join("output.txt");

    // Render - should fail due to security
    let result = render_template(
        Some(main_template.to_str().unwrap()),
        Some(output_file.to_str().unwrap()),
        false,
        None, // trust_mode = false
    );

    assert!(
        result.is_err(),
        "Expected error for parent directory access"
    );
    let error = result.unwrap_err().to_string();
    assert!(
        error.contains("Parent directory") || error.contains(".."),
        "Error message should mention parent directory restriction: {}",
        error
    );

    cleanup_test_env(&test_dir);
}

#[test]
fn test_include_parent_directory_allowed_with_trust() {
    let test_dir = setup_test_env();

    // Create parent directory file
    create_file(&test_dir, "parent.tmpl", "Parent content");

    // Create subdirectory
    let subdir = test_dir.join("subdir");
    fs::create_dir_all(&subdir).unwrap();

    // Create template in subdirectory that includes parent
    let main_template = create_file(
        &subdir,
        "main.tmpl",
        "Start\n{% include \"../parent.tmpl\" %}\nEnd",
    );

    let output_file = test_dir.join("output.txt");

    // Render with trust mode
    let result = render_template(
        Some(main_template.to_str().unwrap()),
        Some(output_file.to_str().unwrap()),
        true,
        None, // trust_mode = true
    );

    assert!(
        result.is_ok(),
        "Template rendering failed: {:?}",
        result.err()
    );

    // Verify output
    let output = fs::read_to_string(&output_file).unwrap();
    assert_eq!(output, "Start\nParent content\nEnd");

    cleanup_test_env(&test_dir);
}

#[test]
fn test_include_absolute_path_blocked() {
    let test_dir = setup_test_env();

    // Create template that tries to use absolute path
    let main_template = create_file(&test_dir, "main.tmpl", "{% include \"/etc/passwd\" %}");

    let output_file = test_dir.join("output.txt");

    // Render - should fail due to security
    let result = render_template(
        Some(main_template.to_str().unwrap()),
        Some(output_file.to_str().unwrap()),
        false,
        None, // trust_mode = false
    );

    assert!(result.is_err(), "Expected error for absolute path");
    let error = result.unwrap_err().to_string();
    assert!(
        error.contains("Absolute paths are not allowed") || error.contains("Security"),
        "Error message should mention security restriction: {}",
        error
    );

    cleanup_test_env(&test_dir);
}

#[test]
fn test_include_multiple_partials() {
    let test_dir = setup_test_env();

    // Create multiple partials
    create_file(&test_dir, "header.tmpl", "=== Header ===");
    create_file(&test_dir, "content.tmpl", "Main Content");
    create_file(&test_dir, "footer.tmpl", "=== Footer ===");

    // Create main template that includes all
    let main_template = create_file(
        &test_dir,
        "main.tmpl",
        "{% include \"./header.tmpl\" %}\n{% include \"./content.tmpl\" %}\n{% include \"./footer.tmpl\" %}",
    );

    let output_file = test_dir.join("output.txt");

    // Render
    let result = render_template(
        Some(main_template.to_str().unwrap()),
        Some(output_file.to_str().unwrap()),
        false,
        None,
    );

    assert!(
        result.is_ok(),
        "Template rendering failed: {:?}",
        result.err()
    );

    // Verify output
    let output = fs::read_to_string(&output_file).unwrap();
    assert_eq!(output, "=== Header ===\nMain Content\n=== Footer ===");

    cleanup_test_env(&test_dir);
}

#[test]
fn test_include_with_conditionals() {
    let test_dir = setup_test_env();

    // Create partial
    create_file(&test_dir, "optional.tmpl", "Optional content included");

    // Create main template with conditional include
    let main_template = create_file(
        &test_dir,
        "main.tmpl",
        "{% set show = get_env(name=\"SHOW_OPTIONAL\", default=\"false\") %}\
        Start\n\
        {% if show == \"true\" %}\
        {% include \"./optional.tmpl\" %}\n\
        {% endif %}\
        End",
    );

    let output_file = test_dir.join("output.txt");

    // Test without env var (should not include)
    let result = render_template(
        Some(main_template.to_str().unwrap()),
        Some(output_file.to_str().unwrap()),
        false,
        None,
    );
    assert!(result.is_ok());
    let output = fs::read_to_string(&output_file).unwrap();
    assert_eq!(output, "Start\nEnd");

    // Test with env var (should include)
    unsafe {
        env::set_var("SHOW_OPTIONAL", "true");
    }
    let result = render_template(
        Some(main_template.to_str().unwrap()),
        Some(output_file.to_str().unwrap()),
        false,
        None,
    );
    assert!(result.is_ok());
    let output = fs::read_to_string(&output_file).unwrap();
    assert_eq!(output, "Start\nOptional content included\nEnd");

    unsafe {
        env::remove_var("SHOW_OPTIONAL");
    }
    cleanup_test_env(&test_dir);
}

#[test]
fn test_include_fixture_templates() {
    // Test using pre-created fixture templates
    let template_path = common::get_fixture_template("include_base.tmpl");
    let output_file = common::get_test_file_path("include_base_output.txt");

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_file.to_str().unwrap()),
        false,
        None,
    );

    assert!(
        result.is_ok(),
        "Template rendering failed: {:?}",
        result.err()
    );

    // Verify output matches expected
    let output = fs::read_to_string(&output_file).unwrap();
    let expected = common::read_fixture_expected("include_base.txt");
    assert_eq!(output, expected);

    common::cleanup_test_file(&output_file);
}

#[test]
fn test_include_nested_fixture_templates() {
    // Test nested includes using fixture templates
    let template_path = common::get_fixture_template("include_nested_main.tmpl");
    let output_file = common::get_test_file_path("include_nested_output.txt");

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_file.to_str().unwrap()),
        false,
        None,
    );

    assert!(
        result.is_ok(),
        "Template rendering failed: {:?}",
        result.err()
    );

    // Verify output matches expected
    let output = fs::read_to_string(&output_file).unwrap();
    let expected = common::read_fixture_expected("include_nested.txt");
    assert_eq!(output, expected);

    common::cleanup_test_file(&output_file);
}
