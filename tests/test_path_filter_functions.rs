//! Tests for path filter-functions.
//!
//! Tests both function and filter syntax for:
//! - basename, dirname, file_extension, join_path, normalize_path

use minijinja::Value;
use minijinja::value::Kwargs;
use tmpltool::filter_functions::FilterFunction;
use tmpltool::filter_functions::path::{Basename, Dirname, FileExtension, JoinPath, NormalizePath};

/// Helper to create empty kwargs
fn empty_kwargs() -> Kwargs {
    Kwargs::from_iter(Vec::<(&str, Value)>::new())
}

// ============================================
// Basename tests
// ============================================

#[test]
fn test_basename_filter_syntax() {
    let result =
        Basename::call_as_filter(&Value::from("/path/to/file.txt"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "file.txt");
}

#[test]
fn test_basename_function_syntax() {
    let kwargs = Kwargs::from_iter(vec![("path", Value::from("/path/to/file.txt"))]);
    let result = Basename::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "file.txt");
}

#[test]
fn test_basename_no_extension() {
    let result = Basename::call_as_filter(&Value::from("/path/to/file"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "file");
}

#[test]
fn test_basename_just_file() {
    let result = Basename::call_as_filter(&Value::from("file.txt"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "file.txt");
}

#[test]
fn test_basename_error_not_string() {
    let result = Basename::call_as_filter(&Value::from(123), empty_kwargs());
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires a string")
    );
}

// ============================================
// Dirname tests
// ============================================

#[test]
fn test_dirname_filter_syntax() {
    let result =
        Dirname::call_as_filter(&Value::from("/path/to/file.txt"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "/path/to");
}

#[test]
fn test_dirname_function_syntax() {
    let kwargs = Kwargs::from_iter(vec![("path", Value::from("/path/to/file.txt"))]);
    let result = Dirname::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "/path/to");
}

#[test]
fn test_dirname_nested() {
    let result = Dirname::call_as_filter(&Value::from("/a/b/c/d"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "/a/b/c");
}

#[test]
fn test_dirname_just_file() {
    let result = Dirname::call_as_filter(&Value::from("file.txt"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "");
}

#[test]
fn test_dirname_error_not_string() {
    let result = Dirname::call_as_filter(&Value::from(123), empty_kwargs());
    assert!(result.is_err());
}

// ============================================
// FileExtension tests
// ============================================

#[test]
fn test_file_extension_filter_syntax() {
    let result =
        FileExtension::call_as_filter(&Value::from("document.pdf"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "pdf");
}

#[test]
fn test_file_extension_function_syntax() {
    let kwargs = Kwargs::from_iter(vec![("path", Value::from("document.pdf"))]);
    let result = FileExtension::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "pdf");
}

#[test]
fn test_file_extension_double() {
    let result =
        FileExtension::call_as_filter(&Value::from("archive.tar.gz"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "gz");
}

#[test]
fn test_file_extension_none() {
    let result = FileExtension::call_as_filter(&Value::from("README"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "");
}

#[test]
fn test_file_extension_with_path() {
    let result =
        FileExtension::call_as_filter(&Value::from("/path/to/file.txt"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "txt");
}

#[test]
fn test_file_extension_error_not_string() {
    let result = FileExtension::call_as_filter(&Value::from(123), empty_kwargs());
    assert!(result.is_err());
}

// ============================================
// JoinPath tests
// ============================================

#[test]
fn test_join_path_filter_syntax() {
    let parts = Value::from_iter(vec![
        Value::from("path"),
        Value::from("to"),
        Value::from("file.txt"),
    ]);
    let result = JoinPath::call_as_filter(&parts, empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "path/to/file.txt");
}

#[test]
fn test_join_path_function_syntax() {
    let parts = vec!["path".to_string(), "to".to_string(), "file.txt".to_string()];
    let kwargs = Kwargs::from_iter(vec![("parts", Value::from_serialize(&parts))]);
    let result = JoinPath::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "path/to/file.txt");
}

#[test]
fn test_join_path_empty() {
    let parts = Value::from_iter(Vec::<Value>::new());
    let result = JoinPath::call_as_filter(&parts, empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "");
}

#[test]
fn test_join_path_single() {
    let parts = Value::from_iter(vec![Value::from("file.txt")]);
    let result = JoinPath::call_as_filter(&parts, empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "file.txt");
}

#[test]
fn test_join_path_error_not_array() {
    // Numbers are not iterable, so this should error
    let result = JoinPath::call_as_filter(&Value::from(123), empty_kwargs());
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires an array")
    );
}

// ============================================
// NormalizePath tests
// ============================================

#[test]
fn test_normalize_path_filter_syntax() {
    let result =
        NormalizePath::call_as_filter(&Value::from("./foo/../bar"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "bar");
}

#[test]
fn test_normalize_path_function_syntax() {
    let kwargs = Kwargs::from_iter(vec![("path", Value::from("./foo/../bar"))]);
    let result = NormalizePath::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "bar");
}

#[test]
fn test_normalize_path_double_dot() {
    let result =
        NormalizePath::call_as_filter(&Value::from("a/b/c/../../d"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "a/d");
}

#[test]
fn test_normalize_path_current() {
    let result =
        NormalizePath::call_as_filter(&Value::from("./foo/./bar"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "foo/bar");
}

#[test]
fn test_normalize_path_clean() {
    let result =
        NormalizePath::call_as_filter(&Value::from("/path/to/file"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "/path/to/file");
}

#[test]
fn test_normalize_path_error_not_string() {
    let result = NormalizePath::call_as_filter(&Value::from(123), empty_kwargs());
    assert!(result.is_err());
}
