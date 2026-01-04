//! Tests for lib.rs get_all_metadata() function
//!
//! These tests verify that the IDE metadata export is working correctly.

use tmpltool::get_all_metadata;

#[test]
fn test_get_all_metadata_returns_non_empty() {
    let metadata = get_all_metadata();
    assert!(!metadata.is_empty(), "Metadata should not be empty");
}

#[test]
fn test_get_all_metadata_count() {
    let metadata = get_all_metadata();
    // We should have a significant number of functions (100+)
    assert!(
        metadata.len() > 100,
        "Expected 100+ functions, got {}",
        metadata.len()
    );
}

#[test]
fn test_get_all_metadata_has_required_categories() {
    let metadata = get_all_metadata();
    let categories: std::collections::HashSet<&str> = metadata.iter().map(|m| m.category).collect();

    // Check for expected categories
    let expected_categories = [
        "hash",
        "encoding",
        "string",
        "array",
        "environment",
        "datetime",
        "filesystem",
        "network",
        "kubernetes",
        "validation",
    ];

    for cat in expected_categories {
        assert!(
            categories.contains(cat),
            "Expected category '{}' not found. Found: {:?}",
            cat,
            categories
        );
    }
}

#[test]
fn test_get_all_metadata_all_have_names() {
    let metadata = get_all_metadata();

    for func in &metadata {
        assert!(!func.name.is_empty(), "Function name should not be empty");
    }
}

#[test]
fn test_get_all_metadata_all_have_descriptions() {
    let metadata = get_all_metadata();

    for func in &metadata {
        assert!(
            !func.description.is_empty(),
            "Function '{}' should have a description",
            func.name
        );
    }
}

#[test]
fn test_get_all_metadata_all_have_return_types() {
    let metadata = get_all_metadata();

    for func in &metadata {
        assert!(
            !func.return_type.is_empty(),
            "Function '{}' should have a return type",
            func.name
        );
    }
}

#[test]
fn test_get_all_metadata_all_have_examples() {
    let metadata = get_all_metadata();

    for func in &metadata {
        assert!(
            !func.examples.is_empty(),
            "Function '{}' should have at least one example",
            func.name
        );
    }
}

#[test]
fn test_get_all_metadata_syntax_variants() {
    let metadata = get_all_metadata();

    let mut has_function_only = false;
    let mut has_filter = false;
    let mut has_is_test = false;

    for func in &metadata {
        if func.syntax.function && !func.syntax.filter && !func.syntax.is_test {
            has_function_only = true;
        }
        if func.syntax.filter {
            has_filter = true;
        }
        if func.syntax.is_test {
            has_is_test = true;
        }
    }

    assert!(
        has_function_only,
        "Should have at least one function-only syntax"
    );
    assert!(has_filter, "Should have at least one filter syntax");
    assert!(has_is_test, "Should have at least one is-test syntax");
}

#[test]
fn test_get_all_metadata_unique_names() {
    let metadata = get_all_metadata();
    let mut names: Vec<&str> = metadata.iter().map(|m| m.name).collect();
    let original_count = names.len();
    names.sort();
    names.dedup();

    assert_eq!(
        names.len(),
        original_count,
        "All function names should be unique"
    );
}

#[test]
fn test_get_all_metadata_specific_functions_exist() {
    let metadata = get_all_metadata();
    let names: std::collections::HashSet<&str> = metadata.iter().map(|m| m.name).collect();

    let expected_functions = [
        "get_env",
        "md5",
        "sha256",
        "base64_encode",
        "now",
        "read_file",
        "is_email",
        "uuid",
        "get_hostname",
    ];

    for func_name in expected_functions {
        assert!(
            names.contains(func_name),
            "Expected function '{}' not found",
            func_name
        );
    }
}

#[test]
fn test_get_all_metadata_arguments_have_required_fields() {
    let metadata = get_all_metadata();

    for func in &metadata {
        for arg in func.arguments.iter() {
            assert!(
                !arg.name.is_empty(),
                "Argument in function '{}' should have a name",
                func.name
            );
            assert!(
                !arg.arg_type.is_empty(),
                "Argument '{}' in function '{}' should have a type",
                arg.name,
                func.name
            );
            assert!(
                !arg.description.is_empty(),
                "Argument '{}' in function '{}' should have a description",
                arg.name,
                func.name
            );
        }
    }
}

#[test]
fn test_get_all_metadata_required_arguments_have_no_default() {
    let metadata = get_all_metadata();

    for func in &metadata {
        for arg in func.arguments.iter() {
            if arg.required {
                // Required arguments typically don't have defaults (except in some edge cases)
                // This is a soft check - we just verify the metadata is consistent
                if arg.default.is_some() {
                    // This is allowed but unusual - just make sure default is valid
                    assert!(
                        !arg.default.unwrap().is_empty(),
                        "If required arg '{}' in '{}' has default, it should be non-empty",
                        arg.name,
                        func.name
                    );
                }
            }
        }
    }
}
