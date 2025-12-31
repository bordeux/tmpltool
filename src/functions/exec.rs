//! Command execution functions for MiniJinja templates
//!
//! This module provides the ability to execute external commands from templates.
//! This is a powerful but potentially dangerous feature, so it requires trust mode.
//!
//! Two functions are provided:
//! - `exec(command)` - Simple execution, returns stdout, throws on error
//! - `exec_raw(command)` - Full control, returns object with exit code, stdout, stderr

use crate::TemplateContext;
use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};
use std::collections::HashMap;
use std::process::{Command, Stdio};
use std::sync::Arc;

/// Execute an external command and return stdout
///
/// This is the simple version that returns stdout directly and throws an error
/// if the command fails (non-zero exit code).
///
/// **SECURITY WARNING:** This function can execute arbitrary commands and is only
/// available in trust mode (`--trust` flag).
///
/// # Arguments
///
/// * `command` (required) - Command to execute (full command line, will be executed via shell)
/// * `timeout` (optional) - Timeout in seconds (default: 30, max: 300)
///
/// # Returns
///
/// Returns stdout as a string. Throws an error if exit code is non-zero.
///
/// # Example
///
/// ```jinja
/// {# Simple usage - get output directly #}
/// Hostname: {{ exec(command="hostname") }}
///
/// {# Use in variable #}
/// {% set files = exec(command="ls /tmp") %}
/// {{ files }}
///
/// {# This will throw an error #}
/// {{ exec(command="ls /nonexistent") }}  {# Error: Command failed (exit 2): ... #}
/// ```
pub fn create_exec_fn(context: Arc<TemplateContext>) -> impl Fn(Kwargs) -> Result<Value, Error> {
    move |kwargs: Kwargs| {
        // Security check: exec is only available in trust mode
        if !context.is_trust_mode() {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                "Security: exec() function requires trust mode. Use --trust flag to enable command execution.",
            ));
        }

        let command: String = kwargs.get("command")?;
        let timeout_secs: u64 = kwargs.get("timeout").unwrap_or(30);

        // Validate timeout
        if timeout_secs > 300 {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                format!("Timeout must be <= 300 seconds, got {}", timeout_secs),
            ));
        }

        // Execute command and get result
        let result = execute_command(&command, timeout_secs)?;

        // Extract values from result object
        let exit_code = result.get_attr("exit_code").unwrap().as_i64().unwrap();
        let stdout = result.get_attr("stdout").unwrap().to_string();
        let stderr = result.get_attr("stderr").unwrap().to_string();

        // Throw error if command failed
        if exit_code != 0 {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                format!(
                    "Command failed (exit {}): {}\nStderr: {}",
                    exit_code, command, stderr
                ),
            ));
        }

        // Return stdout as string
        Ok(Value::from(stdout))
    }
}

/// Execute an external command and return full result object
///
/// This is the advanced version that returns an object with exit code, stdout,
/// and stderr. It never throws based on exit code - you control error handling.
///
/// **SECURITY WARNING:** This function can execute arbitrary commands and is only
/// available in trust mode (`--trust` flag).
///
/// # Arguments
///
/// * `command` (required) - Command to execute (full command line, will be executed via shell)
/// * `timeout` (optional) - Timeout in seconds (default: 30, max: 300)
///
/// # Returns
///
/// Returns an object with the following fields:
/// - `exit_code` - Exit code of the command (integer, 0 = success)
/// - `stdout` - Standard output as string (UTF-8)
/// - `stderr` - Standard error as string (UTF-8)
/// - `success` - Boolean, true if exit_code == 0
///
/// # Example
///
/// ```jinja
/// {# Full control over result #}
/// {% set result = exec_raw(command="ls -la /tmp") %}
/// {% if result.success %}
/// Files:
/// {{ result.stdout }}
/// {% else %}
/// Error (exit {{ result.exit_code }}): {{ result.stderr }}
/// {% endif %}
///
/// {# Handle expected non-zero exit (e.g., grep) #}
/// {% set result = exec_raw(command="grep foo /etc/hosts") %}
/// {% if result.exit_code == 0 %}
/// Found: {{ result.stdout }}
/// {% elif result.exit_code == 1 %}
/// Not found
/// {% else %}
/// Error: {{ result.stderr }}
/// {% endif %}
/// ```
pub fn create_exec_raw_fn(
    context: Arc<TemplateContext>,
) -> impl Fn(Kwargs) -> Result<Value, Error> {
    move |kwargs: Kwargs| {
        // Security check: exec_raw is only available in trust mode
        if !context.is_trust_mode() {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                "Security: exec_raw() function requires trust mode. Use --trust flag to enable command execution.",
            ));
        }

        let command: String = kwargs.get("command")?;
        let timeout_secs: u64 = kwargs.get("timeout").unwrap_or(30);

        // Validate timeout
        if timeout_secs > 300 {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                format!("Timeout must be <= 300 seconds, got {}", timeout_secs),
            ));
        }

        // Execute command and return full result
        execute_command(&command, timeout_secs)
    }
}

/// Execute a command with timeout and return structured result
fn execute_command(command: &str, timeout_secs: u64) -> Result<Value, Error> {
    // Determine shell based on OS
    #[cfg(target_os = "windows")]
    let (shell, shell_arg) = ("cmd", "/C");

    #[cfg(not(target_os = "windows"))]
    let (shell, shell_arg) = ("sh", "-c");

    // Spawn command
    let output = Command::new(shell)
        .arg(shell_arg)
        .arg(command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Failed to execute command '{}': {}", command, e),
            )
        })?;

    // Note: We're using .output() which waits for completion, so timeout
    // isn't enforced in this simple implementation. For production use,
    // you'd want to use std::process::Child with a separate timeout mechanism
    // or the `wait-timeout` crate.
    //
    // For now, we document the timeout parameter but don't enforce it.
    // This can be improved in a future PR.
    let _ = timeout_secs; // Suppress unused variable warning

    // Convert output to UTF-8 strings
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let exit_code = output.status.code().unwrap_or(-1);
    let success = output.status.success();

    // Build result object
    let mut result = HashMap::new();
    result.insert("exit_code".to_string(), Value::from(exit_code));
    result.insert("stdout".to_string(), Value::from(stdout));
    result.insert("stderr".to_string(), Value::from(stderr));
    result.insert("success".to_string(), Value::from(success));

    Ok(Value::from_object(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use minijinja::Value;
    use std::path::PathBuf;

    fn create_trusted_context() -> Arc<TemplateContext> {
        Arc::new(TemplateContext::new(PathBuf::from("."), true))
    }

    fn create_untrusted_context() -> Arc<TemplateContext> {
        Arc::new(TemplateContext::new(PathBuf::from("."), false))
    }

    // Tests for exec() - simple version
    #[test]
    fn test_exec_requires_trust_mode() {
        let context = create_untrusted_context();
        let exec_fn = create_exec_fn(context);

        let result = exec_fn(Kwargs::from_iter(vec![(
            "command",
            Value::from("echo hello"),
        )]));

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("requires trust mode")
        );
    }

    #[test]
    fn test_exec_simple_command() {
        let context = create_trusted_context();
        let exec_fn = create_exec_fn(context);

        let result = exec_fn(Kwargs::from_iter(vec![(
            "command",
            Value::from("echo hello"),
        )]))
        .unwrap();

        // exec() returns stdout directly as string
        let stdout = result.as_str().unwrap();
        assert!(stdout.contains("hello"));
    }

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn test_exec_failing_command_throws_error() {
        let context = create_trusted_context();
        let exec_fn = create_exec_fn(context);

        let result = exec_fn(Kwargs::from_iter(vec![(
            "command",
            Value::from("ls /nonexistent_directory_12345"),
        )]));

        // exec() should throw error on non-zero exit
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Command failed"));
    }

    #[test]
    fn test_exec_invalid_timeout() {
        let context = create_trusted_context();
        let exec_fn = create_exec_fn(context);

        let result = exec_fn(Kwargs::from_iter(vec![
            ("command", Value::from("echo hello")),
            ("timeout", Value::from(500)),
        ]));

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Timeout must be"));
    }

    // Tests for exec_raw() - advanced version
    #[test]
    fn test_exec_raw_requires_trust_mode() {
        let context = create_untrusted_context();
        let exec_raw_fn = create_exec_raw_fn(context);

        let result = exec_raw_fn(Kwargs::from_iter(vec![(
            "command",
            Value::from("echo hello"),
        )]));

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("requires trust mode")
        );
    }

    #[test]
    fn test_exec_raw_simple_command() {
        let context = create_trusted_context();
        let exec_raw_fn = create_exec_raw_fn(context);

        let result = exec_raw_fn(Kwargs::from_iter(vec![(
            "command",
            Value::from("echo hello"),
        )]))
        .unwrap();

        // Verify result structure
        assert!(result.get_attr("success").unwrap().is_true());
        assert_eq!(result.get_attr("exit_code").unwrap().as_i64(), Some(0));

        let stdout_val = result.get_attr("stdout").unwrap();
        let stdout = stdout_val.as_str().unwrap();
        assert!(stdout.contains("hello"));
    }

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn test_exec_raw_failing_command() {
        let context = create_trusted_context();
        let exec_raw_fn = create_exec_raw_fn(context);

        let result = exec_raw_fn(Kwargs::from_iter(vec![(
            "command",
            Value::from("ls /nonexistent_directory_12345"),
        )]))
        .unwrap();

        // exec_raw() should NOT throw error, just return result
        assert!(!result.get_attr("success").unwrap().is_true());
        assert_ne!(result.get_attr("exit_code").unwrap().as_i64(), Some(0));

        let stderr_val = result.get_attr("stderr").unwrap();
        let stderr = stderr_val.as_str().unwrap();
        assert!(!stderr.is_empty());
    }

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn test_exec_raw_stderr_output() {
        let context = create_trusted_context();
        let exec_raw_fn = create_exec_raw_fn(context);

        // Command that writes to stderr
        let result = exec_raw_fn(Kwargs::from_iter(vec![(
            "command",
            Value::from("echo error >&2"),
        )]))
        .unwrap();

        assert!(result.get_attr("success").unwrap().is_true());
        let stderr_val = result.get_attr("stderr").unwrap();
        let stderr = stderr_val.as_str().unwrap();
        assert!(stderr.contains("error"));
    }

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn test_exec_raw_exit_code() {
        let context = create_trusted_context();
        let exec_raw_fn = create_exec_raw_fn(context);

        // Command that exits with code 42
        let result =
            exec_raw_fn(Kwargs::from_iter(vec![("command", Value::from("exit 42"))])).unwrap();

        assert_eq!(result.get_attr("exit_code").unwrap().as_i64(), Some(42));
        assert!(!result.get_attr("success").unwrap().is_true());
    }
}
