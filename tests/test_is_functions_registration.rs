//! Tests for is_functions registration and metadata
//!
//! These tests verify that is-functions are properly registered and their metadata is correct.

use minijinja::Environment;
use std::path::PathBuf;
use std::sync::Arc;
use tmpltool::TemplateContext;
use tmpltool::is_functions;

#[test]
fn test_is_functions_get_all_metadata_returns_expected_count() {
    let metadata = is_functions::get_all_metadata();
    // We expect: email, url, ip, uuid, leap_year, port_available, file, dir, symlink
    assert_eq!(
        metadata.len(),
        9,
        "Expected 9 is-functions, got {}",
        metadata.len()
    );
}

#[test]
fn test_is_functions_metadata_has_is_test_syntax() {
    let metadata = is_functions::get_all_metadata();

    for func in &metadata {
        assert!(
            func.syntax.is_test,
            "Is-function '{}' should have is_test syntax enabled",
            func.name
        );
    }
}

#[test]
fn test_is_functions_metadata_names() {
    let metadata = is_functions::get_all_metadata();
    let names: Vec<&str> = metadata.iter().map(|m| m.name).collect();

    let expected = [
        "is_email",
        "is_url",
        "is_ip",
        "is_uuid",
        "is_leap_year",
        "is_port_available",
        "is_file",
        "is_dir",
        "is_symlink",
    ];

    for expected_name in expected {
        assert!(
            names.contains(&expected_name),
            "Expected is-function '{}' not found. Found: {:?}",
            expected_name,
            names
        );
    }
}

#[test]
fn test_is_functions_register_all_works() {
    let mut env = Environment::new();
    let context = Arc::new(TemplateContext::new(PathBuf::from("."), false));

    // This should not panic
    is_functions::register_all(&mut env, context);
}

#[test]
fn test_is_functions_work_in_template() {
    let mut env = Environment::new();
    let context = Arc::new(TemplateContext::new(PathBuf::from("."), false));
    is_functions::register_all(&mut env, context);

    // Test is_email function syntax
    env.add_template(
        "test_func",
        "{% if is_email(string='test@example.com') %}yes{% else %}no{% endif %}",
    )
    .unwrap();
    let tmpl = env.get_template("test_func").unwrap();
    let result = tmpl.render(()).unwrap();
    assert_eq!(result, "yes");

    // Test is email test syntax
    env.add_template(
        "test_is",
        "{% if 'test@example.com' is email %}yes{% else %}no{% endif %}",
    )
    .unwrap();
    let tmpl = env.get_template("test_is").unwrap();
    let result = tmpl.render(()).unwrap();
    assert_eq!(result, "yes");
}

#[test]
fn test_is_functions_metadata_categories() {
    let metadata = is_functions::get_all_metadata();
    let categories: std::collections::HashSet<&str> = metadata.iter().map(|m| m.category).collect();

    let expected = ["validation", "datetime", "network", "filesystem"];

    for cat in expected {
        assert!(
            categories.contains(cat),
            "Expected category '{}' not found",
            cat
        );
    }
}
