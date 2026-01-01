use minijinja::Environment;
use std::path::PathBuf;
use tmpltool::{TemplateContext, functions::register_all};

fn render_template(template: &str) -> String {
    let mut env = Environment::new();
    let ctx = TemplateContext::new(PathBuf::from("."), false);
    register_all(&mut env, ctx);

    let tmpl = env.template_from_str(template).unwrap();
    tmpl.render(()).unwrap()
}

// Helper to validate UUID format
fn is_valid_uuid_format(uuid_str: &str) -> bool {
    if uuid_str.len() != 36 {
        return false;
    }
    let chars: Vec<char> = uuid_str.chars().collect();
    chars[8] == '-' && chars[13] == '-' && chars[18] == '-' && chars[23] == '-'
}

// ==================== UUID v4 Tests ====================

#[test]
fn test_uuid_v4_default() {
    let result = render_template("{{ uuid() }}");
    assert!(is_valid_uuid_format(&result));
    // Version should be 4 (14th character)
    assert_eq!(result.chars().nth(14).unwrap(), '4');
}

#[test]
fn test_uuid_v4_explicit() {
    let result = render_template(r#"{{ uuid(version="v4") }}"#);
    assert!(is_valid_uuid_format(&result));
    // Version should be 4
    assert_eq!(result.chars().nth(14).unwrap(), '4');
}

#[test]
fn test_uuid_v4_uniqueness() {
    let result1 = render_template("{{ uuid() }}");
    let result2 = render_template("{{ uuid() }}");
    assert_ne!(result1, result2);
}

#[test]
fn test_uuid_v4_valid_hex() {
    let result = render_template("{{ uuid() }}");
    let hex_part: String = result.chars().filter(|c| *c != '-').collect();
    assert_eq!(hex_part.len(), 32);
    for ch in hex_part.chars() {
        assert!(ch.is_ascii_hexdigit());
    }
}

// ==================== UUID v7 Tests ====================

#[test]
fn test_uuid_v7_explicit() {
    let result = render_template(r#"{{ uuid(version="v7") }}"#);
    assert!(is_valid_uuid_format(&result));
    // Version should be 7 (14th character)
    assert_eq!(result.chars().nth(14).unwrap(), '7');
}

#[test]
fn test_uuid_v7_uniqueness() {
    let result1 = render_template(r#"{{ uuid(version="v7") }}"#);
    let result2 = render_template(r#"{{ uuid(version="v7") }}"#);
    assert_ne!(result1, result2);
}

#[test]
fn test_uuid_v7_valid_hex() {
    let result = render_template(r#"{{ uuid(version="v7") }}"#);
    let hex_part: String = result.chars().filter(|c| *c != '-').collect();
    assert_eq!(hex_part.len(), 32);
    for ch in hex_part.chars() {
        assert!(ch.is_ascii_hexdigit());
    }
}

#[test]
fn test_uuid_v7_is_time_ordered() {
    // UUIDv7 should be roughly time-ordered (lexicographically sortable)
    let result1 = render_template(r#"{{ uuid(version="v7") }}"#);
    std::thread::sleep(std::time::Duration::from_millis(2));
    let result2 = render_template(r#"{{ uuid(version="v7") }}"#);
    // The first part of v7 UUID is timestamp-based, so result2 should be >= result1
    assert!(result2 >= result1);
}

// ==================== Error Cases ====================

#[test]
fn test_uuid_invalid_version() {
    let mut env = Environment::new();
    let ctx = TemplateContext::new(PathBuf::from("."), false);
    register_all(&mut env, ctx);

    let tmpl = env
        .template_from_str(r#"{{ uuid(version="v99") }}"#)
        .unwrap();
    let result = tmpl.render(());
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("Invalid UUID version"));
    assert!(err.contains("v99"));
}

#[test]
fn test_uuid_invalid_version_v1() {
    let mut env = Environment::new();
    let ctx = TemplateContext::new(PathBuf::from("."), false);
    register_all(&mut env, ctx);

    let tmpl = env
        .template_from_str(r#"{{ uuid(version="v1") }}"#)
        .unwrap();
    let result = tmpl.render(());
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("Invalid UUID version"));
}
