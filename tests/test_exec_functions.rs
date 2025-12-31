use minijinja::Environment;
use std::path::PathBuf;
use tmpltool::{TemplateContext, functions};

fn create_env(trust_mode: bool) -> Environment<'static> {
    let mut env = Environment::new();
    let context = TemplateContext::new(PathBuf::from("."), trust_mode);
    functions::register_all(&mut env, context);
    env
}

fn render_template(env: &Environment, template: &str) -> Result<String, minijinja::Error> {
    let tmpl = env.template_from_str(template)?;
    tmpl.render(())
}

// Tests for exec() - simple version

#[test]
fn test_exec_requires_trust_mode() {
    let env = create_env(false);
    let result = render_template(&env, "{{ exec(command=\"echo hello\") }}");

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("requires trust mode"));
    assert!(err.contains("--trust"));
}

#[test]
fn test_exec_simple_command() {
    let env = create_env(true);
    let result = render_template(&env, "{{ exec(command=\"echo hello\") }}").unwrap();

    assert!(result.contains("hello"));
}

#[test]
fn test_exec_with_trim_filter() {
    let env = create_env(true);
    let result = render_template(&env, "{{ exec(command=\"echo hello\") | trim }}").unwrap();

    assert_eq!(result, "hello");
}

#[test]
fn test_exec_in_variable() {
    let env = create_env(true);
    let result = render_template(
        &env,
        "{% set output = exec(command=\"echo test\") %}Result: {{ output | trim }}",
    )
    .unwrap();

    assert!(result.contains("Result: test"));
}

#[test]
#[cfg(not(target_os = "windows"))]
fn test_exec_throws_on_failure() {
    let env = create_env(true);
    let result = render_template(&env, "{{ exec(command=\"ls /nonexistent_12345\") }}");

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("Command failed"));
}

#[test]
#[cfg(not(target_os = "windows"))]
fn test_exec_with_pipe() {
    let env = create_env(true);
    let result = render_template(
        &env,
        "{{ exec(command=\"echo 'hello world' | awk '{print $2}'\") }}",
    )
    .unwrap();

    assert!(result.contains("world"));
}

#[test]
fn test_exec_multiple_commands() {
    let env = create_env(true);
    let result = render_template(
        &env,
        "{% set a = exec(command=\"echo first\") %}{% set b = exec(command=\"echo second\") %}{{ a | trim }}-{{ b | trim }}",
    )
    .unwrap();

    assert_eq!(result, "first-second");
}

// Tests for exec_raw() - advanced version

#[test]
fn test_exec_raw_requires_trust_mode() {
    let env = create_env(false);
    let result = render_template(&env, "{{ exec_raw(command=\"echo hello\").stdout }}");

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("requires trust mode"));
}

#[test]
fn test_exec_raw_success_flag() {
    let env = create_env(true);
    let result = render_template(&env, "{{ exec_raw(command=\"echo hello\").success }}").unwrap();

    assert_eq!(result, "true");
}

#[test]
fn test_exec_raw_exit_code_success() {
    let env = create_env(true);
    let result = render_template(&env, "{{ exec_raw(command=\"echo hello\").exit_code }}").unwrap();

    assert_eq!(result, "0");
}

#[test]
fn test_exec_raw_stdout() {
    let env = create_env(true);
    let result =
        render_template(&env, "{{ exec_raw(command=\"echo hello\").stdout | trim }}").unwrap();

    assert_eq!(result, "hello");
}

#[test]
#[cfg(not(target_os = "windows"))]
fn test_exec_raw_stderr() {
    let env = create_env(true);
    let result = render_template(
        &env,
        "{{ exec_raw(command=\"echo error >&2\").stderr | trim }}",
    )
    .unwrap();

    assert!(result.contains("error"));
}

#[test]
#[cfg(not(target_os = "windows"))]
fn test_exec_raw_failing_command_no_error() {
    let env = create_env(true);
    // exec_raw should NOT throw error, just return result
    let result = render_template(
        &env,
        "{% set r = exec_raw(command=\"ls /nonexistent_12345\") %}{{ r.success }}",
    )
    .unwrap();

    assert_eq!(result, "false");
}

#[test]
#[cfg(not(target_os = "windows"))]
fn test_exec_raw_exit_code_nonzero() {
    let env = create_env(true);
    let result = render_template(
        &env,
        "{{ exec_raw(command=\"ls /nonexistent_12345\").exit_code }}",
    )
    .unwrap();

    let exit_code: i32 = result.parse().unwrap();
    assert_ne!(exit_code, 0);
}

#[test]
#[cfg(not(target_os = "windows"))]
fn test_exec_raw_with_conditional() {
    let env = create_env(true);
    let result = render_template(
        &env,
        "{% set r = exec_raw(command=\"which sh\") %}{% if r.success %}found{% else %}not found{% endif %}",
    )
    .unwrap();

    assert_eq!(result, "found");
}

#[test]
#[cfg(not(target_os = "windows"))]
fn test_exec_raw_check_exit_code_grep() {
    let env = create_env(true);
    // grep returns 0 if found, 1 if not found, 2 if error
    let result = render_template(
        &env,
        r#"{% set r = exec_raw(command="echo 'hello' | grep 'hello'") %}{% if r.exit_code == 0 %}found{% elif r.exit_code == 1 %}not found{% else %}error{% endif %}"#,
    )
    .unwrap();

    assert_eq!(result, "found");
}

// Integration tests combining both functions

#[test]
fn test_exec_and_exec_raw_together() {
    let env = create_env(true);
    let result = render_template(
        &env,
        "{% set simple = exec(command=\"echo simple\") %}{% set detailed = exec_raw(command=\"echo detailed\") %}{{ simple | trim }}-{{ detailed.stdout | trim }}",
    )
    .unwrap();

    assert_eq!(result, "simple-detailed");
}

#[test]
#[cfg(not(target_os = "windows"))]
fn test_exec_raw_access_all_fields() {
    let env = create_env(true);
    let result = render_template(
        &env,
        "{% set r = exec_raw(command=\"echo test\") %}exit={{ r.exit_code }},success={{ r.success }},stdout={{ r.stdout | trim }},stderr={{ r.stderr | trim }}",
    )
    .unwrap();

    assert_eq!(result, "exit=0,success=true,stdout=test,stderr=");
}

// Timeout parameter tests

#[test]
fn test_exec_with_valid_timeout() {
    let env = create_env(true);
    let result = render_template(
        &env,
        "{{ exec(command=\"echo hello\", timeout=10) | trim }}",
    )
    .unwrap();

    assert_eq!(result, "hello");
}

#[test]
fn test_exec_raw_with_valid_timeout() {
    let env = create_env(true);
    let result = render_template(
        &env,
        "{{ exec_raw(command=\"echo hello\", timeout=30).stdout | trim }}",
    )
    .unwrap();

    assert_eq!(result, "hello");
}

#[test]
fn test_exec_with_invalid_timeout() {
    let env = create_env(true);
    let result = render_template(&env, "{{ exec(command=\"echo hello\", timeout=500) }}");

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("Timeout must be"));
}

#[test]
fn test_exec_raw_with_invalid_timeout() {
    let env = create_env(true);
    let result = render_template(
        &env,
        "{{ exec_raw(command=\"echo hello\", timeout=500).stdout }}",
    );

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("Timeout must be"));
}

// Real-world use case tests

#[test]
#[cfg(not(target_os = "windows"))]
fn test_use_case_build_info() {
    let env = create_env(true);
    let result = render_template(
        &env,
        r#"commit: {{ exec(command="git rev-parse --short HEAD 2>/dev/null || echo 'unknown'") | trim }}"#,
    )
    .unwrap();

    assert!(result.starts_with("commit: "));
}

#[test]
#[cfg(not(target_os = "windows"))]
fn test_use_case_conditional_config() {
    let env = create_env(true);
    let result = render_template(
        &env,
        "{% set r = exec_raw(command=\"which sh\") %}sh_available: {{ r.success }}",
    )
    .unwrap();

    assert!(result.contains("sh_available: true"));
}

#[test]
#[cfg(not(target_os = "windows"))]
fn test_use_case_fallback_pattern() {
    let env = create_env(true);
    let result = render_template(
        &env,
        "{% set r = exec_raw(command=\"hostname 2>/dev/null\") %}hostname: {{ r.stdout | trim if r.success else 'unknown' }}",
    )
    .unwrap();

    assert!(result.starts_with("hostname: "));
    assert!(!result.contains("unknown"));
}

#[test]
#[cfg(not(target_os = "windows"))]
fn test_use_case_version_detection() {
    let env = create_env(true);
    let result = render_template(
        &env,
        "{% set r = exec_raw(command=\"sh --version 2>&1 || echo 'sh available'\") %}status: {% if r.success %}installed{% else %}not installed{% endif %}",
    )
    .unwrap();

    assert!(result.contains("status: installed"));
}

// Edge cases

#[test]
fn test_exec_empty_command() {
    let env = create_env(true);
    let result = render_template(&env, "{{ exec(command=\"\") }}");

    // Empty command succeeds with no output (shell behavior)
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "");
}

#[test]
#[cfg(not(target_os = "windows"))]
fn test_exec_raw_command_not_found() {
    let env = create_env(true);
    let result = render_template(
        &env,
        "{{ exec_raw(command=\"nonexistent_command_12345\").exit_code }}",
    )
    .unwrap();

    // Command not found should have non-zero exit code
    let exit_code: i32 = result.parse().unwrap();
    assert_ne!(exit_code, 0);
}

#[test]
fn test_exec_with_special_characters_in_output() {
    let env = create_env(true);
    let result = render_template(
        &env,
        "{{ exec(command=\"echo 'test@example.com'\") | trim }}",
    )
    .unwrap();

    assert_eq!(result, "test@example.com");
}

#[test]
#[cfg(not(target_os = "windows"))]
fn test_exec_multiline_output() {
    let env = create_env(true);
    let result = render_template(
        &env,
        "{{ exec(command=\"printf 'line1\\nline2\\nline3'\") }}",
    )
    .unwrap();

    assert!(result.contains("line1"));
    assert!(result.contains("line2"));
    assert!(result.contains("line3"));
}
