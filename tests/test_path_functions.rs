use minijinja::Value;
use minijinja::value::Kwargs;
use std::path::PathBuf;
use std::sync::Arc;
use tmpltool::TemplateContext;
use tmpltool::functions::filesystem;

// Helper to create a trusted context
fn create_trusted_context() -> Arc<TemplateContext> {
    Arc::new(TemplateContext::new(PathBuf::from("."), true))
}

// basename tests
#[test]
fn test_basename_simple() {
    let result =
        filesystem::basename_fn(Kwargs::from_iter(vec![("path", Value::from("file.txt"))]))
            .unwrap();
    assert_eq!(result.as_str().unwrap(), "file.txt");
}

#[test]
fn test_basename_with_directory() {
    let result = filesystem::basename_fn(Kwargs::from_iter(vec![(
        "path",
        Value::from("/path/to/file.txt"),
    )]))
    .unwrap();
    assert_eq!(result.as_str().unwrap(), "file.txt");
}

#[test]
fn test_basename_nested_path() {
    let result = filesystem::basename_fn(Kwargs::from_iter(vec![(
        "path",
        Value::from("folder/subfolder/document.pdf"),
    )]))
    .unwrap();
    assert_eq!(result.as_str().unwrap(), "document.pdf");
}

#[test]
fn test_basename_directory_only() {
    let result = filesystem::basename_fn(Kwargs::from_iter(vec![(
        "path",
        Value::from("/path/to/directory/"),
    )]))
    .unwrap();
    assert_eq!(result.as_str().unwrap(), "directory");
}

#[test]
fn test_basename_no_extension() {
    let result =
        filesystem::basename_fn(Kwargs::from_iter(vec![("path", Value::from("README"))])).unwrap();
    assert_eq!(result.as_str().unwrap(), "README");
}

// dirname tests
#[test]
fn test_dirname_simple() {
    let result = filesystem::dirname_fn(Kwargs::from_iter(vec![(
        "path",
        Value::from("/path/to/file.txt"),
    )]))
    .unwrap();
    assert_eq!(result.as_str().unwrap(), "/path/to");
}

#[test]
fn test_dirname_relative() {
    let result = filesystem::dirname_fn(Kwargs::from_iter(vec![(
        "path",
        Value::from("folder/file.txt"),
    )]))
    .unwrap();
    assert_eq!(result.as_str().unwrap(), "folder");
}

#[test]
fn test_dirname_nested() {
    let result = filesystem::dirname_fn(Kwargs::from_iter(vec![(
        "path",
        Value::from("a/b/c/d/file.txt"),
    )]))
    .unwrap();
    assert_eq!(result.as_str().unwrap(), "a/b/c/d");
}

#[test]
fn test_dirname_root() {
    let result =
        filesystem::dirname_fn(Kwargs::from_iter(vec![("path", Value::from("/file.txt"))]))
            .unwrap();
    assert_eq!(result.as_str().unwrap(), "/");
}

#[test]
fn test_dirname_no_directory() {
    let result =
        filesystem::dirname_fn(Kwargs::from_iter(vec![("path", Value::from("file.txt"))])).unwrap();
    assert_eq!(result.as_str().unwrap(), "");
}

// file_extension tests
#[test]
fn test_file_extension_simple() {
    let result =
        filesystem::file_extension_fn(Kwargs::from_iter(vec![("path", Value::from("file.txt"))]))
            .unwrap();
    assert_eq!(result.as_str().unwrap(), "txt");
}

#[test]
fn test_file_extension_multiple_dots() {
    let result = filesystem::file_extension_fn(Kwargs::from_iter(vec![(
        "path",
        Value::from("archive.tar.gz"),
    )]))
    .unwrap();
    assert_eq!(result.as_str().unwrap(), "gz");
}

#[test]
fn test_file_extension_with_path() {
    let result = filesystem::file_extension_fn(Kwargs::from_iter(vec![(
        "path",
        Value::from("/path/to/document.pdf"),
    )]))
    .unwrap();
    assert_eq!(result.as_str().unwrap(), "pdf");
}

#[test]
fn test_file_extension_no_extension() {
    let result =
        filesystem::file_extension_fn(Kwargs::from_iter(vec![("path", Value::from("README"))]))
            .unwrap();
    assert_eq!(result.as_str().unwrap(), "");
}

#[test]
fn test_file_extension_hidden_file() {
    let result =
        filesystem::file_extension_fn(Kwargs::from_iter(vec![("path", Value::from(".gitignore"))]))
            .unwrap();
    assert_eq!(result.as_str().unwrap(), "");
}

#[test]
fn test_file_extension_hidden_with_ext() {
    let result = filesystem::file_extension_fn(Kwargs::from_iter(vec![(
        "path",
        Value::from(".config.json"),
    )]))
    .unwrap();
    assert_eq!(result.as_str().unwrap(), "json");
}

// join_path tests
#[test]
fn test_join_path_simple() {
    let parts = vec!["path", "to", "file.txt"];
    let result =
        filesystem::join_path_fn(Kwargs::from_iter(vec![("parts", Value::from(parts))])).unwrap();
    assert_eq!(result.as_str().unwrap(), "path/to/file.txt");
}

#[test]
fn test_join_path_absolute() {
    let parts = vec!["/home", "user", "documents"];
    let result =
        filesystem::join_path_fn(Kwargs::from_iter(vec![("parts", Value::from(parts))])).unwrap();
    assert_eq!(result.as_str().unwrap(), "/home/user/documents");
}

#[test]
fn test_join_path_single() {
    let parts = vec!["file.txt"];
    let result =
        filesystem::join_path_fn(Kwargs::from_iter(vec![("parts", Value::from(parts))])).unwrap();
    assert_eq!(result.as_str().unwrap(), "file.txt");
}

#[test]
fn test_join_path_empty() {
    let parts: Vec<String> = vec![];
    let result =
        filesystem::join_path_fn(Kwargs::from_iter(vec![("parts", Value::from(parts))])).unwrap();
    assert_eq!(result.as_str().unwrap(), "");
}

// normalize_path tests
#[test]
fn test_normalize_path_current_dir() {
    let result =
        filesystem::normalize_path_fn(Kwargs::from_iter(vec![("path", Value::from("./foo/bar"))]))
            .unwrap();
    assert_eq!(result.as_str().unwrap(), "foo/bar");
}

#[test]
fn test_normalize_path_parent_dir() {
    let result =
        filesystem::normalize_path_fn(Kwargs::from_iter(vec![("path", Value::from("foo/../bar"))]))
            .unwrap();
    assert_eq!(result.as_str().unwrap(), "bar");
}

#[test]
fn test_normalize_path_multiple_parents() {
    let result = filesystem::normalize_path_fn(Kwargs::from_iter(vec![(
        "path",
        Value::from("a/b/c/../../d"),
    )]))
    .unwrap();
    assert_eq!(result.as_str().unwrap(), "a/d");
}

#[test]
fn test_normalize_path_absolute() {
    let result = filesystem::normalize_path_fn(Kwargs::from_iter(vec![(
        "path",
        Value::from("/path/to/../file.txt"),
    )]))
    .unwrap();
    assert_eq!(result.as_str().unwrap(), "/path/file.txt");
}

#[test]
fn test_normalize_path_complex() {
    let result = filesystem::normalize_path_fn(Kwargs::from_iter(vec![(
        "path",
        Value::from("./a/./b/../c/./d"),
    )]))
    .unwrap();
    assert_eq!(result.as_str().unwrap(), "a/c/d");
}

// Note: is_file, is_dir, and is_symlink tests have been moved to
// tests/test_is_filesystem.rs as part of the is-functions refactoring.

// read_lines tests
#[test]
fn test_read_lines_basic() {
    let context = create_trusted_context();
    let read_lines_fn = filesystem::create_read_lines_fn(context);

    let result =
        read_lines_fn(Kwargs::from_iter(vec![("path", Value::from("Cargo.toml"))])).unwrap();

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
    let read_lines_fn = filesystem::create_read_lines_fn(context);

    let result = read_lines_fn(Kwargs::from_iter(vec![
        ("path", Value::from("Cargo.toml")),
        ("max_lines", Value::from(3)),
    ]))
    .unwrap();

    let lines: Vec<_> = result.try_iter().unwrap().collect();
    assert_eq!(lines.len(), 3);
}

#[test]
fn test_read_lines_entire_file() {
    let context = create_trusted_context();
    let read_lines_fn = filesystem::create_read_lines_fn(context);

    let result = read_lines_fn(Kwargs::from_iter(vec![
        ("path", Value::from("Cargo.toml")),
        ("max_lines", Value::from(0)),
    ]))
    .unwrap();

    let lines: Vec<_> = result.try_iter().unwrap().collect();
    // Cargo.toml should have more than 3 lines
    assert!(lines.len() > 3);
}

#[test]
fn test_read_lines_invalid_max_large() {
    let context = create_trusted_context();
    let read_lines_fn = filesystem::create_read_lines_fn(context);

    let result = read_lines_fn(Kwargs::from_iter(vec![
        ("path", Value::from("Cargo.toml")),
        ("max_lines", Value::from(20000)),
    ]));

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
    let read_lines_fn = filesystem::create_read_lines_fn(context);

    let result = read_lines_fn(Kwargs::from_iter(vec![
        ("path", Value::from("Cargo.toml")),
        ("max_lines", Value::from(-3)),
    ]))
    .unwrap();

    let lines: Vec<_> = result.try_iter().unwrap().collect();
    assert_eq!(lines.len(), 3);
}

#[test]
fn test_read_lines_negative_more_than_file() {
    let context = create_trusted_context();
    let read_lines_fn = filesystem::create_read_lines_fn(context);

    // Request more lines than the file has
    let result = read_lines_fn(Kwargs::from_iter(vec![
        ("path", Value::from("Cargo.toml")),
        ("max_lines", Value::from(-10000)),
    ]))
    .unwrap();

    let lines: Vec<_> = result.try_iter().unwrap().collect();
    // Should return all lines when requesting more than available
    assert!(!lines.is_empty());
}

#[test]
fn test_read_lines_nonexistent() {
    let context = create_trusted_context();
    let read_lines_fn = filesystem::create_read_lines_fn(context);

    let result = read_lines_fn(Kwargs::from_iter(vec![(
        "path",
        Value::from("nonexistent.txt"),
    )]));

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Failed to read"));
}
