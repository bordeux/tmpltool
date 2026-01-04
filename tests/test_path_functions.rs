use minijinja::Value;
use minijinja::value::Kwargs;
use std::path::PathBuf;
use std::sync::Arc;
use tmpltool::TemplateContext;
use tmpltool::functions::ContextFunction;
use tmpltool::functions::filesystem::ReadLines;

// Helper to create a trusted context
fn create_trusted_context() -> Arc<TemplateContext> {
    Arc::new(TemplateContext::new(PathBuf::from("."), true))
}

// Note: basename_fn, dirname_fn, file_extension_fn, join_path_fn, normalize_path_fn
// tests removed - these functions are now in filter_functions/path.rs with dual syntax support.
// See tests/test_filters_integration.rs for integration tests of these filters.

// Note: is_file, is_dir, and is_symlink tests have been moved to
// tests/test_is_filesystem.rs as part of the is-functions refactoring.

// read_lines tests
#[test]
fn test_read_lines_basic() {
    let context = create_trusted_context();

    let result = ReadLines::call(
        context.clone(),
        Kwargs::from_iter(vec![("path", Value::from("Cargo.toml"))]),
    )
    .unwrap();

    // Should return an array with lines
    let lines: Vec<_> = result.try_iter().unwrap().collect();

    // Default max_lines is 10, so we should get at most 10 lines
    assert!(lines.len() <= 10);

    // Should have at least some content
    assert!(!lines.is_empty());
}

#[test]
fn test_read_lines_with_max() {
    let context = create_trusted_context();

    let result = ReadLines::call(
        context.clone(),
        Kwargs::from_iter(vec![
            ("path", Value::from("Cargo.toml")),
            ("max_lines", Value::from(3)),
        ]),
    )
    .unwrap();

    let lines: Vec<_> = result.try_iter().unwrap().collect();
    assert_eq!(lines.len(), 3);
}

#[test]
fn test_read_lines_entire_file() {
    let context = create_trusted_context();

    let result = ReadLines::call(
        context.clone(),
        Kwargs::from_iter(vec![
            ("path", Value::from("Cargo.toml")),
            ("max_lines", Value::from(0)),
        ]),
    )
    .unwrap();

    let lines: Vec<_> = result.try_iter().unwrap().collect();
    // Cargo.toml should have more than 3 lines
    assert!(lines.len() > 3);
}

#[test]
fn test_read_lines_invalid_max_large() {
    let context = create_trusted_context();

    let result = ReadLines::call(
        context.clone(),
        Kwargs::from_iter(vec![
            ("path", Value::from("Cargo.toml")),
            ("max_lines", Value::from(20000)),
        ]),
    );

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("between 0 and 10000")
    );
}

#[test]
fn test_read_lines_last_lines() {
    let context = create_trusted_context();

    let result = ReadLines::call(
        context.clone(),
        Kwargs::from_iter(vec![
            ("path", Value::from("Cargo.toml")),
            ("max_lines", Value::from(-3)),
        ]),
    )
    .unwrap();

    let lines: Vec<_> = result.try_iter().unwrap().collect();
    assert_eq!(lines.len(), 3);
}

#[test]
fn test_read_lines_negative_more_than_file() {
    let context = create_trusted_context();

    // Request more lines than the file has
    let result = ReadLines::call(
        context.clone(),
        Kwargs::from_iter(vec![
            ("path", Value::from("Cargo.toml")),
            ("max_lines", Value::from(-10000)),
        ]),
    )
    .unwrap();

    let lines: Vec<_> = result.try_iter().unwrap().collect();
    // Should return all lines when requesting more than available
    assert!(!lines.is_empty());
}

#[test]
fn test_read_lines_nonexistent() {
    let context = create_trusted_context();

    let result = ReadLines::call(
        context.clone(),
        Kwargs::from_iter(vec![("path", Value::from("nonexistent.txt"))]),
    );

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Failed to read"));
}
