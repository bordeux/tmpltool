//! Tests for filesystem is-functions
//!
//! Tests both function syntax and "is" test syntax for:
//! - is_file / file
//! - is_dir / dir
//! - is_symlink / symlink

use minijinja::Environment;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;
use tempfile::TempDir;
use tmpltool::TemplateContext;

/// Helper to create a test environment with is-functions registered
fn create_test_env(base_dir: PathBuf) -> Environment<'static> {
    let mut env = Environment::new();
    let context = Arc::new(TemplateContext::new(base_dir, false));
    tmpltool::is_functions::register_all(&mut env, context);
    env
}

/// Helper to render a template and check the result
fn render(env: &Environment, template: &str) -> String {
    env.render_str(template, ()).unwrap()
}

/// Create a temporary directory with test files
fn setup_test_dir() -> TempDir {
    let temp_dir = TempDir::new().unwrap();

    // Create a test file
    let file_path = temp_dir.path().join("test_file.txt");
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "test content").unwrap();

    // Create a subdirectory
    let sub_dir = temp_dir.path().join("test_dir");
    fs::create_dir(&sub_dir).unwrap();

    // Create a file in subdirectory
    let sub_file = sub_dir.join("nested_file.txt");
    File::create(&sub_file).unwrap();

    temp_dir
}

// ========== is_file Tests ==========

#[test]
fn test_is_file_function_syntax_exists() {
    let temp_dir = setup_test_dir();
    let env = create_test_env(temp_dir.path().to_path_buf());

    let result = render(&env, r#"{{ is_file(path="test_file.txt") }}"#);
    assert_eq!(result, "true");
}

#[test]
fn test_is_file_function_syntax_not_exists() {
    let temp_dir = setup_test_dir();
    let env = create_test_env(temp_dir.path().to_path_buf());

    let result = render(&env, r#"{{ is_file(path="nonexistent.txt") }}"#);
    assert_eq!(result, "false");
}

#[test]
fn test_is_file_function_syntax_is_dir() {
    let temp_dir = setup_test_dir();
    let env = create_test_env(temp_dir.path().to_path_buf());

    // A directory should return false for is_file
    let result = render(&env, r#"{{ is_file(path="test_dir") }}"#);
    assert_eq!(result, "false");
}

#[test]
fn test_is_file_is_syntax_exists() {
    let temp_dir = setup_test_dir();
    let env = create_test_env(temp_dir.path().to_path_buf());

    let result = render(
        &env,
        r#"{% if "test_file.txt" is file %}yes{% else %}no{% endif %}"#,
    );
    assert_eq!(result, "yes");
}

#[test]
fn test_is_file_is_syntax_not_exists() {
    let temp_dir = setup_test_dir();
    let env = create_test_env(temp_dir.path().to_path_buf());

    let result = render(
        &env,
        r#"{% if "nonexistent.txt" is file %}yes{% else %}no{% endif %}"#,
    );
    assert_eq!(result, "no");
}

#[test]
fn test_is_file_is_syntax_with_variable() {
    let temp_dir = setup_test_dir();
    let mut env = create_test_env(temp_dir.path().to_path_buf());
    env.add_template("test", r#"{% if path is file %}yes{% else %}no{% endif %}"#)
        .unwrap();

    let tmpl = env.get_template("test").unwrap();

    // Existing file
    let result = tmpl
        .render(minijinja::context! { path => "test_file.txt" })
        .unwrap();
    assert_eq!(result, "yes");

    // Non-existing file
    let result = tmpl
        .render(minijinja::context! { path => "nonexistent.txt" })
        .unwrap();
    assert_eq!(result, "no");
}

#[test]
fn test_is_file_nested_path() {
    let temp_dir = setup_test_dir();
    let env = create_test_env(temp_dir.path().to_path_buf());

    let result = render(
        &env,
        r#"{% if "test_dir/nested_file.txt" is file %}yes{% else %}no{% endif %}"#,
    );
    assert_eq!(result, "yes");
}

#[test]
fn test_is_file_non_string_value() {
    let temp_dir = setup_test_dir();
    let env = create_test_env(temp_dir.path().to_path_buf());

    // Non-string values should return false
    let result = render(&env, r#"{% if 123 is file %}yes{% else %}no{% endif %}"#);
    assert_eq!(result, "no");
}

// ========== is_dir Tests ==========

#[test]
fn test_is_dir_function_syntax_exists() {
    let temp_dir = setup_test_dir();
    let env = create_test_env(temp_dir.path().to_path_buf());

    let result = render(&env, r#"{{ is_dir(path="test_dir") }}"#);
    assert_eq!(result, "true");
}

#[test]
fn test_is_dir_function_syntax_not_exists() {
    let temp_dir = setup_test_dir();
    let env = create_test_env(temp_dir.path().to_path_buf());

    let result = render(&env, r#"{{ is_dir(path="nonexistent_dir") }}"#);
    assert_eq!(result, "false");
}

#[test]
fn test_is_dir_function_syntax_is_file() {
    let temp_dir = setup_test_dir();
    let env = create_test_env(temp_dir.path().to_path_buf());

    // A file should return false for is_dir
    let result = render(&env, r#"{{ is_dir(path="test_file.txt") }}"#);
    assert_eq!(result, "false");
}

#[test]
fn test_is_dir_is_syntax_exists() {
    let temp_dir = setup_test_dir();
    let env = create_test_env(temp_dir.path().to_path_buf());

    let result = render(
        &env,
        r#"{% if "test_dir" is dir %}yes{% else %}no{% endif %}"#,
    );
    assert_eq!(result, "yes");
}

#[test]
fn test_is_dir_is_syntax_not_exists() {
    let temp_dir = setup_test_dir();
    let env = create_test_env(temp_dir.path().to_path_buf());

    let result = render(
        &env,
        r#"{% if "nonexistent_dir" is dir %}yes{% else %}no{% endif %}"#,
    );
    assert_eq!(result, "no");
}

#[test]
fn test_is_dir_is_syntax_with_variable() {
    let temp_dir = setup_test_dir();
    let mut env = create_test_env(temp_dir.path().to_path_buf());
    env.add_template("test", r#"{% if path is dir %}yes{% else %}no{% endif %}"#)
        .unwrap();

    let tmpl = env.get_template("test").unwrap();

    // Existing directory
    let result = tmpl
        .render(minijinja::context! { path => "test_dir" })
        .unwrap();
    assert_eq!(result, "yes");

    // File (not a directory)
    let result = tmpl
        .render(minijinja::context! { path => "test_file.txt" })
        .unwrap();
    assert_eq!(result, "no");
}

#[test]
fn test_is_dir_non_string_value() {
    let temp_dir = setup_test_dir();
    let env = create_test_env(temp_dir.path().to_path_buf());

    // Non-string values should return false
    let result = render(&env, r#"{% if 123 is dir %}yes{% else %}no{% endif %}"#);
    assert_eq!(result, "no");
}

// ========== is_symlink Tests ==========

#[cfg(unix)]
#[test]
fn test_is_symlink_function_syntax() {
    let temp_dir = setup_test_dir();

    // Create a symlink
    let target = temp_dir.path().join("test_file.txt");
    let link = temp_dir.path().join("test_link");
    std::os::unix::fs::symlink(&target, &link).unwrap();

    let env = create_test_env(temp_dir.path().to_path_buf());

    let result = render(&env, r#"{{ is_symlink(path="test_link") }}"#);
    assert_eq!(result, "true");
}

#[cfg(unix)]
#[test]
fn test_is_symlink_function_syntax_regular_file() {
    let temp_dir = setup_test_dir();
    let env = create_test_env(temp_dir.path().to_path_buf());

    // Regular file should return false
    let result = render(&env, r#"{{ is_symlink(path="test_file.txt") }}"#);
    assert_eq!(result, "false");
}

#[cfg(unix)]
#[test]
fn test_is_symlink_is_syntax() {
    let temp_dir = setup_test_dir();

    // Create a symlink
    let target = temp_dir.path().join("test_file.txt");
    let link = temp_dir.path().join("test_link");
    std::os::unix::fs::symlink(&target, &link).unwrap();

    let env = create_test_env(temp_dir.path().to_path_buf());

    let result = render(
        &env,
        r#"{% if "test_link" is symlink %}yes{% else %}no{% endif %}"#,
    );
    assert_eq!(result, "yes");
}

#[cfg(unix)]
#[test]
fn test_is_symlink_is_syntax_regular_file() {
    let temp_dir = setup_test_dir();
    let env = create_test_env(temp_dir.path().to_path_buf());

    let result = render(
        &env,
        r#"{% if "test_file.txt" is symlink %}yes{% else %}no{% endif %}"#,
    );
    assert_eq!(result, "no");
}

#[test]
fn test_is_symlink_nonexistent() {
    let temp_dir = setup_test_dir();
    let env = create_test_env(temp_dir.path().to_path_buf());

    let result = render(
        &env,
        r#"{% if "nonexistent" is symlink %}yes{% else %}no{% endif %}"#,
    );
    assert_eq!(result, "no");
}

#[test]
fn test_is_symlink_non_string_value() {
    let temp_dir = setup_test_dir();
    let env = create_test_env(temp_dir.path().to_path_buf());

    // Non-string values should return false
    let result = render(&env, r#"{% if 123 is symlink %}yes{% else %}no{% endif %}"#);
    assert_eq!(result, "no");
}

// ========== Negation Tests ==========

#[test]
fn test_is_file_negation() {
    let temp_dir = setup_test_dir();
    let env = create_test_env(temp_dir.path().to_path_buf());

    let result = render(
        &env,
        r#"{% if "nonexistent.txt" is not file %}missing{% else %}found{% endif %}"#,
    );
    assert_eq!(result, "missing");
}

#[test]
fn test_is_dir_negation() {
    let temp_dir = setup_test_dir();
    let env = create_test_env(temp_dir.path().to_path_buf());

    let result = render(
        &env,
        r#"{% if "test_file.txt" is not dir %}not a dir{% else %}is a dir{% endif %}"#,
    );
    assert_eq!(result, "not a dir");
}

// ========== Combined Tests ==========

#[test]
fn test_multiple_filesystem_checks() {
    let temp_dir = setup_test_dir();
    let env = create_test_env(temp_dir.path().to_path_buf());

    let result = render(
        &env,
        r#"{% if "test_file.txt" is file %}F{% endif %}{% if "test_dir" is dir %}D{% endif %}"#,
    );
    assert_eq!(result, "FD");
}

#[test]
fn test_filesystem_check_in_loop() {
    let temp_dir = setup_test_dir();
    let mut env = create_test_env(temp_dir.path().to_path_buf());
    env.add_template(
        "test",
        r#"{% for p in paths %}{% if p is file %}F{% elif p is dir %}D{% else %}X{% endif %}{% endfor %}"#,
    )
    .unwrap();

    let tmpl = env.get_template("test").unwrap();
    let result = tmpl
        .render(minijinja::context! {
            paths => vec!["test_file.txt", "test_dir", "nonexistent"]
        })
        .unwrap();

    assert_eq!(result, "FDX");
}
