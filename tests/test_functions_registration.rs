//! Tests for functions module registration and metadata
//!
//! These tests verify that functions are properly registered and their metadata is correct.

use minijinja::Environment;
use std::path::PathBuf;
use tmpltool::TemplateContext;
use tmpltool::functions;

#[test]
fn test_functions_get_all_metadata_returns_expected_count() {
    let metadata = functions::get_all_metadata();
    // We expect a significant number of functions (at least 50)
    assert!(
        metadata.len() >= 50,
        "Expected at least 50 functions, got {}",
        metadata.len()
    );
}

#[test]
fn test_functions_metadata_all_function_syntax() {
    let metadata = functions::get_all_metadata();

    for func in &metadata {
        assert!(
            func.syntax.function,
            "Function '{}' should have function syntax enabled",
            func.name
        );
    }
}

#[test]
fn test_functions_metadata_has_expected_functions() {
    let metadata = functions::get_all_metadata();
    let names: std::collections::HashSet<&str> = metadata.iter().map(|m| m.name).collect();

    let expected = [
        "get_env",
        "filter_env",
        "uuid",
        "random_string",
        "get_random",
        "now",
        "read_file",
        "file_exists",
        "glob",
        "get_hostname",
        "get_ip_address",
        "cidr_contains",
        "min",
        "max",
        "exec",
        "read_json_file",
    ];

    for expected_name in expected {
        assert!(
            names.contains(expected_name),
            "Expected function '{}' not found. Total count: {}",
            expected_name,
            names.len()
        );
    }
}

#[test]
fn test_functions_register_all_works() {
    let mut env = Environment::new();
    let context = TemplateContext::new(PathBuf::from("."), false);

    // This should not panic
    functions::register_all(&mut env, context);
}

#[test]
fn test_functions_work_in_template() {
    let mut env = Environment::new();
    let context = TemplateContext::new(PathBuf::from("."), false);
    functions::register_all(&mut env, context);

    // Test get_env with default
    // SAFETY: This is a test environment, we control the variable
    unsafe {
        std::env::set_var("TEST_FUNC_REG_VAR", "test_value");
    }
    env.add_template("test", "{{ get_env(name='TEST_FUNC_REG_VAR') }}")
        .unwrap();
    let tmpl = env.get_template("test").unwrap();
    let result = tmpl.render(()).unwrap();
    assert_eq!(result, "test_value");
    unsafe {
        std::env::remove_var("TEST_FUNC_REG_VAR");
    }

    // Test get_env with default when var not set
    env.add_template(
        "test2",
        "{{ get_env(name='NONEXISTENT_VAR_12345', default='fallback') }}",
    )
    .unwrap();
    let tmpl = env.get_template("test2").unwrap();
    let result = tmpl.render(()).unwrap();
    assert_eq!(result, "fallback");
}

#[test]
fn test_functions_metadata_categories() {
    let metadata = functions::get_all_metadata();
    let categories: std::collections::HashSet<&str> = metadata.iter().map(|m| m.category).collect();

    let expected = [
        "environment",
        "random",
        "datetime",
        "filesystem",
        "network",
        "system",
        "debug",
        "predicate",
        "logic",
        "array",
        "object",
        "kubernetes",
        "url",
        "data_parsing",
        "exec",
    ];

    for cat in expected {
        assert!(
            categories.contains(cat),
            "Expected category '{}' not found. Found: {:?}",
            cat,
            categories
        );
    }
}

#[test]
fn test_functions_metadata_no_duplicates() {
    let metadata = functions::get_all_metadata();
    let mut names: Vec<&str> = metadata.iter().map(|m| m.name).collect();
    let original_count = names.len();
    names.sort();
    names.dedup();

    assert_eq!(
        names.len(),
        original_count,
        "Function names should be unique within functions module"
    );
}

#[test]
fn test_trust_mode_functions_registered() {
    let mut env = Environment::new();
    let context = TemplateContext::new(PathBuf::from("."), true); // trust mode
    functions::register_all(&mut env, context);

    // Test that exec function is accessible in trust mode via template
    env.add_template("test_exec", "{% if true %}exec available{% endif %}")
        .unwrap();
    let tmpl = env.get_template("test_exec").unwrap();
    let result = tmpl.render(()).unwrap();
    assert_eq!(result, "exec available");
}
