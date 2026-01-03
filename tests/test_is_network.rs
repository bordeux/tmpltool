//! Tests for network is-functions
//!
//! Tests both function syntax and "is" test syntax for:
//! - is_port_available / port_available
//!
//! Note: Port availability tests may be flaky depending on system state.
//! We use high-numbered ports that are unlikely to be in use.

use minijinja::Environment;
use std::net::TcpListener;
use std::path::PathBuf;
use std::sync::Arc;
use tmpltool::TemplateContext;
use tmpltool::is_functions::network::PortAvailable;

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

/// Find an available port for testing
fn find_available_port() -> u16 {
    // Bind to port 0 to let OS assign an available port
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener); // Release the port
    port
}

/// Bind to a port and return the listener (keeping it bound)
fn bind_port(port: u16) -> Option<TcpListener> {
    TcpListener::bind(("0.0.0.0", port)).ok()
}

// ========== PortAvailable Unit Tests ==========

#[test]
fn test_port_available_finds_free_port() {
    let port = find_available_port();
    // Port should be available right after we release it
    assert!(PortAvailable::is_available(port));
}

#[test]
fn test_port_available_detects_bound_port() {
    let port = find_available_port();
    // Bind to the port
    let _listener = bind_port(port);
    if _listener.is_some() {
        // Port should now be unavailable
        assert!(!PortAvailable::is_available(port));
    }
    // When _listener is dropped, port becomes available again
}

// ========== Function Syntax Tests ==========

#[test]
fn test_is_port_available_function_syntax() {
    let env = create_test_env();
    let port = find_available_port();

    let template = format!(r#"{{{{ is_port_available(port={}) }}}}"#, port);
    let result = render(&env, &template);
    assert_eq!(result, "true");
}

#[test]
fn test_is_port_available_function_syntax_bound_port() {
    let env = create_test_env();
    let port = find_available_port();

    // Bind to the port
    let _listener = bind_port(port);
    if _listener.is_some() {
        let template = format!(r#"{{{{ is_port_available(port={}) }}}}"#, port);
        let result = render(&env, &template);
        assert_eq!(result, "false");
    }
}

#[test]
fn test_is_port_available_function_in_conditional() {
    let env = create_test_env();
    let port = find_available_port();

    let template = format!(
        r#"{{% if is_port_available(port={}) %}}free{{% else %}}busy{{% endif %}}"#,
        port
    );
    let result = render(&env, &template);
    assert_eq!(result, "free");
}

// ========== Is-Test Syntax Tests ==========

#[test]
fn test_is_port_available_is_syntax() {
    let env = create_test_env();
    let port = find_available_port();

    let template = format!(
        r#"{{% if {} is port_available %}}yes{{% else %}}no{{% endif %}}"#,
        port
    );
    let result = render(&env, &template);
    assert_eq!(result, "yes");
}

#[test]
fn test_is_port_available_is_syntax_bound_port() {
    let env = create_test_env();
    let port = find_available_port();

    // Bind to the port
    let _listener = bind_port(port);
    if _listener.is_some() {
        let template = format!(
            r#"{{% if {} is port_available %}}yes{{% else %}}no{{% endif %}}"#,
            port
        );
        let result = render(&env, &template);
        assert_eq!(result, "no");
    }
}

#[test]
fn test_is_port_available_is_syntax_with_variable() {
    let mut env = create_test_env();
    env.add_template(
        "test",
        r#"{% if port is port_available %}free{% else %}busy{% endif %}"#,
    )
    .unwrap();

    let port = find_available_port();
    let tmpl = env.get_template("test").unwrap();
    let result = tmpl.render(minijinja::context! { port => port }).unwrap();
    assert_eq!(result, "free");
}

#[test]
fn test_is_port_available_is_syntax_with_string() {
    let env = create_test_env();
    let port = find_available_port();

    // String representation of port should also work
    let template = format!(
        r#"{{% if "{}" is port_available %}}yes{{% else %}}no{{% endif %}}"#,
        port
    );
    let result = render(&env, &template);
    assert_eq!(result, "yes");
}

#[test]
fn test_is_port_available_is_syntax_negation() {
    let env = create_test_env();
    let port = find_available_port();

    // Bind to the port
    let _listener = bind_port(port);
    if _listener.is_some() {
        let template = format!(
            r#"{{% if {} is not port_available %}}busy{{% else %}}free{{% endif %}}"#,
            port
        );
        let result = render(&env, &template);
        assert_eq!(result, "busy");
    }
}

// ========== Invalid Input Tests ==========

#[test]
fn test_is_port_available_invalid_port_zero() {
    let env = create_test_env();

    // Port 0 is invalid
    assert_eq!(
        render(
            &env,
            r#"{% if 0 is port_available %}yes{% else %}no{% endif %}"#
        ),
        "no"
    );
}

#[test]
fn test_is_port_available_invalid_port_negative() {
    let env = create_test_env();

    // Negative port is invalid (will be false since it's out of range)
    assert_eq!(
        render(
            &env,
            r#"{% if -1 is port_available %}yes{% else %}no{% endif %}"#
        ),
        "no"
    );
}

#[test]
fn test_is_port_available_invalid_port_too_high() {
    let env = create_test_env();

    // Port > 65535 is invalid
    assert_eq!(
        render(
            &env,
            r#"{% if 65536 is port_available %}yes{% else %}no{% endif %}"#
        ),
        "no"
    );
}

#[test]
fn test_is_port_available_non_numeric_string() {
    let env = create_test_env();

    // Non-numeric string should return false
    assert_eq!(
        render(
            &env,
            r#"{% if "not-a-port" is port_available %}yes{% else %}no{% endif %}"#
        ),
        "no"
    );
}

#[test]
fn test_is_port_available_empty_string() {
    let env = create_test_env();

    // Empty string should return false
    assert_eq!(
        render(
            &env,
            r#"{% if "" is port_available %}yes{% else %}no{% endif %}"#
        ),
        "no"
    );
}

// ========== Edge Case Tests ==========

#[test]
fn test_is_port_available_boundary_port_1() {
    let env = create_test_env();

    // Port 1 is valid but typically requires root privileges
    // Just verify it doesn't crash - result depends on privileges
    let result = render(
        &env,
        r#"{% if 1 is port_available %}yes{% else %}no{% endif %}"#,
    );
    assert!(result == "yes" || result == "no");
}

#[test]
fn test_is_port_available_boundary_port_65535() {
    let env = create_test_env();

    // Port 65535 is the maximum valid port
    // Just verify it doesn't crash - result depends on system state
    let result = render(
        &env,
        r#"{% if 65535 is port_available %}yes{% else %}no{% endif %}"#,
    );
    assert!(result == "yes" || result == "no");
}

// ========== Function Syntax Error Tests ==========

#[test]
fn test_is_port_available_function_invalid_port_error() {
    let env = create_test_env();

    // Function syntax with invalid port should return an error
    let result = env.render_str(r#"{{ is_port_available(port=0) }}"#, ());
    assert!(result.is_err());

    let result = env.render_str(r#"{{ is_port_available(port=70000) }}"#, ());
    assert!(result.is_err());
}
