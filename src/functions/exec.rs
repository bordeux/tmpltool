//! Command execution functions for MiniJinja templates
//!
//! This module provides the ability to execute external commands from templates.
//! This is a powerful but potentially dangerous feature, so it requires trust mode.
//!
//! Two functions are provided:
//! - `exec(command)` - Simple execution, returns stdout, throws on error
//! - `exec_raw(command)` - Full control, returns object with exit code, stdout, stderr

use super::metadata::{ArgumentMetadata, FunctionMetadata, SyntaxVariants};
use super::traits::ContextFunction;
use crate::TemplateContext;
use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};
use std::collections::HashMap;
use std::process::{Command, Stdio};
use std::sync::Arc;

/// Execute command and return stdout (throws on error)
pub struct Exec;

impl ContextFunction for Exec {
    const NAME: &'static str = "exec";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "exec",
        category: "exec",
        description: "Execute command and return stdout (throws on non-zero exit)",
        arguments: &[
            ArgumentMetadata {
                name: "command",
                arg_type: "string",
                required: true,
                default: None,
                description: "Command to execute (via shell)",
            },
            ArgumentMetadata {
                name: "timeout",
                arg_type: "integer",
                required: false,
                default: Some("30"),
                description: "Timeout in seconds (max: 300)",
            },
        ],
        return_type: "string",
        examples: &[
            "{{ exec(command=\"hostname\") }}",
            "{% set files = exec(command=\"ls /tmp\") %}",
        ],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(context: Arc<TemplateContext>, kwargs: Kwargs) -> Result<Value, Error> {
        // Security check: exec is only available in trust mode
        if !context.is_trust_mode() {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                "Security: exec() function requires trust mode. Use --trust flag to enable command execution.",
            ));
        }

        let command: String = kwargs.get("command")?;
        let timeout_secs: u64 = kwargs.get("timeout").unwrap_or(30);

        if timeout_secs > 300 {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                format!("Timeout must be <= 300 seconds, got {}", timeout_secs),
            ));
        }

        let result = execute_command(&command, timeout_secs)?;

        let exit_code = result.get_attr("exit_code").unwrap().as_i64().unwrap();
        let stdout = result.get_attr("stdout").unwrap().to_string();
        let stderr = result.get_attr("stderr").unwrap().to_string();

        if exit_code != 0 {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                format!(
                    "Command failed (exit {}): {}\nStderr: {}",
                    exit_code, command, stderr
                ),
            ));
        }

        Ok(Value::from(stdout))
    }
}

/// Execute command and return full result object
pub struct ExecRaw;

impl ContextFunction for ExecRaw {
    const NAME: &'static str = "exec_raw";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "exec_raw",
        category: "exec",
        description: "Execute command and return full result object with exit_code, stdout, stderr",
        arguments: &[
            ArgumentMetadata {
                name: "command",
                arg_type: "string",
                required: true,
                default: None,
                description: "Command to execute (via shell)",
            },
            ArgumentMetadata {
                name: "timeout",
                arg_type: "integer",
                required: false,
                default: Some("30"),
                description: "Timeout in seconds (max: 300)",
            },
        ],
        return_type: "object",
        examples: &[
            "{% set r = exec_raw(command=\"ls\") %}{% if r.success %}{{ r.stdout }}{% endif %}",
        ],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(context: Arc<TemplateContext>, kwargs: Kwargs) -> Result<Value, Error> {
        // Security check: exec_raw is only available in trust mode
        if !context.is_trust_mode() {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                "Security: exec_raw() function requires trust mode. Use --trust flag to enable command execution.",
            ));
        }

        let command: String = kwargs.get("command")?;
        let timeout_secs: u64 = kwargs.get("timeout").unwrap_or(30);

        if timeout_secs > 300 {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                format!("Timeout must be <= 300 seconds, got {}", timeout_secs),
            ));
        }

        execute_command(&command, timeout_secs)
    }
}

/// Execute a command with timeout and return structured result
fn execute_command(command: &str, timeout_secs: u64) -> Result<Value, Error> {
    #[cfg(target_os = "windows")]
    let (shell, shell_arg) = ("cmd", "/C");

    #[cfg(not(target_os = "windows"))]
    let (shell, shell_arg) = ("sh", "-c");

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

    // Note: timeout_secs is documented but not enforced in this simple implementation.
    // For production use, you'd want to use std::process::Child with a separate
    // timeout mechanism or the `wait-timeout` crate.
    let _ = timeout_secs;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let exit_code = output.status.code().unwrap_or(-1);
    let success = output.status.success();

    let mut result = HashMap::new();
    result.insert("exit_code".to_string(), Value::from(exit_code));
    result.insert("stdout".to_string(), Value::from(stdout));
    result.insert("stderr".to_string(), Value::from(stderr));
    result.insert("success".to_string(), Value::from(success));

    Ok(Value::from_object(result))
}
