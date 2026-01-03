//! Tests for validation is-functions
//!
//! Tests both function syntax and "is" test syntax for:
//! - is_email / email
//! - is_url / url
//! - is_ip / ip
//! - is_uuid / uuid

use minijinja::Environment;
use std::path::PathBuf;
use std::sync::Arc;
use tmpltool::TemplateContext;
use tmpltool::is_functions::validation::{Email, Ip, Url, Uuid};

/// Helper to create a test environment with is-functions registered
fn create_test_env() -> Environment<'static> {
    let mut env = Environment::new();
    let context = Arc::new(TemplateContext::new(PathBuf::from("."), false));
    tmpltool::is_functions::register_all(&mut env, context);
    env
}

/// Helper to render a template and check the result
fn render(env: &Environment, template: &str) -> String {
    env.render_str(template, ()).unwrap()
}

// ========== Email Tests ==========

#[test]
fn test_email_validate_valid() {
    assert!(Email::validate("test@example.com"));
    assert!(Email::validate("user+tag@example.com"));
    assert!(Email::validate("user.name@subdomain.example.com"));
}

#[test]
fn test_email_validate_invalid() {
    assert!(!Email::validate("not-an-email"));
    assert!(!Email::validate("@example.com"));
    assert!(!Email::validate("user@"));
    assert!(!Email::validate(""));
}

#[test]
fn test_is_email_function_syntax() {
    let env = create_test_env();

    // Valid emails
    assert_eq!(
        render(&env, r#"{{ is_email(string="test@example.com") }}"#),
        "true"
    );
    assert_eq!(
        render(
            &env,
            r#"{{ is_email(string="user+tag@mail.example.org") }}"#
        ),
        "true"
    );

    // Invalid emails
    assert_eq!(
        render(&env, r#"{{ is_email(string="not-an-email") }}"#),
        "false"
    );
    assert_eq!(render(&env, r#"{{ is_email(string="user@") }}"#), "false");
}

#[test]
fn test_is_email_is_syntax() {
    let env = create_test_env();

    // Valid emails
    assert_eq!(
        render(
            &env,
            r#"{% if "test@example.com" is email %}yes{% else %}no{% endif %}"#
        ),
        "yes"
    );
    assert_eq!(
        render(
            &env,
            r#"{% if "user@mail.example.org" is email %}yes{% else %}no{% endif %}"#
        ),
        "yes"
    );

    // Invalid emails
    assert_eq!(
        render(
            &env,
            r#"{% if "not-an-email" is email %}yes{% else %}no{% endif %}"#
        ),
        "no"
    );
    assert_eq!(
        render(&env, r#"{% if "" is email %}yes{% else %}no{% endif %}"#),
        "no"
    );
}

#[test]
fn test_is_email_is_syntax_with_variable() {
    let mut env = create_test_env();
    env.add_template(
        "test",
        r#"{% if email is email %}valid{% else %}invalid{% endif %}"#,
    )
    .unwrap();

    let tmpl = env.get_template("test").unwrap();

    // Valid email
    let result = tmpl
        .render(minijinja::context! { email => "user@example.com" })
        .unwrap();
    assert_eq!(result, "valid");

    // Invalid email
    let result = tmpl
        .render(minijinja::context! { email => "not-an-email" })
        .unwrap();
    assert_eq!(result, "invalid");
}

#[test]
fn test_is_email_is_syntax_non_string() {
    let env = create_test_env();

    // Non-string values should return false
    assert_eq!(
        render(&env, r#"{% if 123 is email %}yes{% else %}no{% endif %}"#),
        "no"
    );
    assert_eq!(
        render(&env, r#"{% if true is email %}yes{% else %}no{% endif %}"#),
        "no"
    );
}

// ========== URL Tests ==========

#[test]
fn test_url_validate_valid() {
    assert!(Url::validate("https://example.com"));
    assert!(Url::validate("http://example.com/path"));
    assert!(Url::validate("ftp://files.example.com/file.txt"));
    assert!(Url::validate("https://example.com:8080/path?query=value"));
}

#[test]
fn test_url_validate_invalid() {
    assert!(!Url::validate("example.com"));
    assert!(!Url::validate("not-a-url"));
    assert!(!Url::validate("mailto:user@example.com")); // unsupported scheme
    assert!(!Url::validate(""));
}

#[test]
fn test_is_url_function_syntax() {
    let env = create_test_env();

    // Valid URLs
    assert_eq!(
        render(&env, r#"{{ is_url(string="https://example.com") }}"#),
        "true"
    );
    assert_eq!(
        render(
            &env,
            r#"{{ is_url(string="http://example.com/path?q=1") }}"#
        ),
        "true"
    );

    // Invalid URLs
    assert_eq!(
        render(&env, r#"{{ is_url(string="example.com") }}"#),
        "false"
    );
    assert_eq!(render(&env, r#"{{ is_url(string="not-a-url") }}"#), "false");
}

#[test]
fn test_is_url_is_syntax() {
    let env = create_test_env();

    // Valid URLs
    assert_eq!(
        render(
            &env,
            r#"{% if "https://example.com" is url %}yes{% else %}no{% endif %}"#
        ),
        "yes"
    );
    assert_eq!(
        render(
            &env,
            r#"{% if "ftp://files.example.com/file" is url %}yes{% else %}no{% endif %}"#
        ),
        "yes"
    );

    // Invalid URLs
    assert_eq!(
        render(
            &env,
            r#"{% if "example.com" is url %}yes{% else %}no{% endif %}"#
        ),
        "no"
    );
    assert_eq!(
        render(&env, r#"{% if "" is url %}yes{% else %}no{% endif %}"#),
        "no"
    );
}

#[test]
fn test_is_url_is_syntax_non_string() {
    let env = create_test_env();

    // Non-string values should return false
    assert_eq!(
        render(&env, r#"{% if 123 is url %}yes{% else %}no{% endif %}"#),
        "no"
    );
}

// ========== IP Tests ==========

#[test]
fn test_ip_validate_valid() {
    // IPv4
    assert!(Ip::validate("192.168.1.1"));
    assert!(Ip::validate("127.0.0.1"));
    assert!(Ip::validate("0.0.0.0"));
    assert!(Ip::validate("255.255.255.255"));

    // IPv6
    assert!(Ip::validate("::1"));
    assert!(Ip::validate("2001:db8::1"));
    assert!(Ip::validate("fe80::1"));
}

#[test]
fn test_ip_validate_invalid() {
    assert!(!Ip::validate("256.1.1.1"));
    assert!(!Ip::validate("192.168.1"));
    assert!(!Ip::validate("not-an-ip"));
    assert!(!Ip::validate(""));
}

#[test]
fn test_is_ip_function_syntax() {
    let env = create_test_env();

    // Valid IPs
    assert_eq!(render(&env, r#"{{ is_ip(string="192.168.1.1") }}"#), "true");
    assert_eq!(render(&env, r#"{{ is_ip(string="::1") }}"#), "true");

    // Invalid IPs
    assert_eq!(render(&env, r#"{{ is_ip(string="256.1.1.1") }}"#), "false");
    assert_eq!(render(&env, r#"{{ is_ip(string="not-an-ip") }}"#), "false");
}

#[test]
fn test_is_ip_is_syntax() {
    let env = create_test_env();

    // Valid IPs
    assert_eq!(
        render(
            &env,
            r#"{% if "192.168.1.1" is ip %}yes{% else %}no{% endif %}"#
        ),
        "yes"
    );
    assert_eq!(
        render(&env, r#"{% if "::1" is ip %}yes{% else %}no{% endif %}"#),
        "yes"
    );
    assert_eq!(
        render(
            &env,
            r#"{% if "2001:db8::1" is ip %}yes{% else %}no{% endif %}"#
        ),
        "yes"
    );

    // Invalid IPs
    assert_eq!(
        render(
            &env,
            r#"{% if "256.1.1.1" is ip %}yes{% else %}no{% endif %}"#
        ),
        "no"
    );
    assert_eq!(
        render(
            &env,
            r#"{% if "not-an-ip" is ip %}yes{% else %}no{% endif %}"#
        ),
        "no"
    );
}

#[test]
fn test_is_ip_is_syntax_non_string() {
    let env = create_test_env();

    // Non-string values should return false
    assert_eq!(
        render(&env, r#"{% if 192 is ip %}yes{% else %}no{% endif %}"#),
        "no"
    );
}

// ========== UUID Tests ==========

#[test]
fn test_uuid_validate_valid() {
    assert!(Uuid::validate("550e8400-e29b-41d4-a716-446655440000"));
    assert!(Uuid::validate("f47ac10b-58cc-4372-a567-0e02b2c3d479"));
    assert!(Uuid::validate("F47AC10B-58CC-4372-A567-0E02B2C3D479")); // uppercase
}

#[test]
fn test_uuid_validate_invalid() {
    assert!(!Uuid::validate("not-a-uuid"));
    assert!(!Uuid::validate("550e8400e29b41d4a716446655440000")); // no dashes
    assert!(!Uuid::validate("550e8400-e29b-41d4-a716")); // too short
    assert!(!Uuid::validate(""));
}

#[test]
fn test_is_uuid_function_syntax() {
    let env = create_test_env();

    // Valid UUIDs
    assert_eq!(
        render(
            &env,
            r#"{{ is_uuid(string="550e8400-e29b-41d4-a716-446655440000") }}"#
        ),
        "true"
    );

    // Invalid UUIDs
    assert_eq!(
        render(&env, r#"{{ is_uuid(string="not-a-uuid") }}"#),
        "false"
    );
    assert_eq!(
        render(
            &env,
            r#"{{ is_uuid(string="550e8400e29b41d4a716446655440000") }}"#
        ),
        "false"
    );
}

#[test]
fn test_is_uuid_is_syntax() {
    let env = create_test_env();

    // Valid UUIDs
    assert_eq!(
        render(
            &env,
            r#"{% if "550e8400-e29b-41d4-a716-446655440000" is uuid %}yes{% else %}no{% endif %}"#
        ),
        "yes"
    );
    assert_eq!(
        render(
            &env,
            r#"{% if "F47AC10B-58CC-4372-A567-0E02B2C3D479" is uuid %}yes{% else %}no{% endif %}"#
        ),
        "yes"
    );

    // Invalid UUIDs
    assert_eq!(
        render(
            &env,
            r#"{% if "not-a-uuid" is uuid %}yes{% else %}no{% endif %}"#
        ),
        "no"
    );
    assert_eq!(
        render(&env, r#"{% if "" is uuid %}yes{% else %}no{% endif %}"#),
        "no"
    );
}

#[test]
fn test_is_uuid_is_syntax_non_string() {
    let env = create_test_env();

    // Non-string values should return false
    assert_eq!(
        render(&env, r#"{% if 123 is uuid %}yes{% else %}no{% endif %}"#),
        "no"
    );
}

// ========== Combined Tests ==========

#[test]
fn test_multiple_is_checks_in_template() {
    let env = create_test_env();

    let template = r#"
{%- if "test@example.com" is email %}email:yes{% else %}email:no{% endif %}
{%- if "https://example.com" is url %}url:yes{% else %}url:no{% endif %}
{%- if "192.168.1.1" is ip %}ip:yes{% else %}ip:no{% endif %}
{%- if "550e8400-e29b-41d4-a716-446655440000" is uuid %}uuid:yes{% else %}uuid:no{% endif %}
"#;

    let result = render(&env, template);
    assert!(result.contains("email:yes"));
    assert!(result.contains("url:yes"));
    assert!(result.contains("ip:yes"));
    assert!(result.contains("uuid:yes"));
}

#[test]
fn test_is_check_in_for_loop() {
    let mut env = create_test_env();
    env.add_template(
        "test",
        r#"{% for item in items %}{% if item is email %}{{ item }} is valid{% endif %}{% endfor %}"#,
    )
    .unwrap();

    let tmpl = env.get_template("test").unwrap();
    let result = tmpl
        .render(minijinja::context! {
            items => vec!["test@example.com", "not-email", "user@domain.org"]
        })
        .unwrap();

    assert!(result.contains("test@example.com is valid"));
    assert!(result.contains("user@domain.org is valid"));
    assert!(!result.contains("not-email is valid"));
}

#[test]
fn test_negated_is_check() {
    let env = create_test_env();

    // Using "is not" syntax
    assert_eq!(
        render(
            &env,
            r#"{% if "not-email" is not email %}invalid{% else %}valid{% endif %}"#
        ),
        "invalid"
    );
    assert_eq!(
        render(
            &env,
            r#"{% if "test@example.com" is not email %}invalid{% else %}valid{% endif %}"#
        ),
        "valid"
    );
}

// ========== Edge Cases ==========

#[test]
fn test_is_email_edge_cases() {
    let env = create_test_env();

    // Edge cases
    assert_eq!(
        render(
            &env,
            r#"{% if "a@b.co" is email %}yes{% else %}no{% endif %}"#
        ),
        "yes"
    ); // minimal valid
    assert_eq!(
        render(
            &env,
            r#"{% if "user.name+tag@sub.domain.example.com" is email %}yes{% else %}no{% endif %}"#
        ),
        "yes"
    ); // complex valid
}

#[test]
fn test_is_url_edge_cases() {
    let env = create_test_env();

    // With port
    assert_eq!(
        render(
            &env,
            r#"{% if "https://localhost:3000" is url %}yes{% else %}no{% endif %}"#
        ),
        "yes"
    );
    // Note: URL regex currently doesn't support fragments (#section)
    // This is consistent with the original is_url function behavior
}

#[test]
fn test_is_ip_edge_cases() {
    let env = create_test_env();

    // Edge cases for IPv4
    assert_eq!(
        render(
            &env,
            r#"{% if "0.0.0.0" is ip %}yes{% else %}no{% endif %}"#
        ),
        "yes"
    );
    assert_eq!(
        render(
            &env,
            r#"{% if "255.255.255.255" is ip %}yes{% else %}no{% endif %}"#
        ),
        "yes"
    );

    // Edge cases for IPv6
    assert_eq!(
        render(&env, r#"{% if "::" is ip %}yes{% else %}no{% endif %}"#),
        "yes"
    ); // all zeros compressed
}
