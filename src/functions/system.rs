//! System information functions for MiniJinja templates
//!
//! This module provides functions to access system information:
//! - `get_hostname`: System hostname
//! - `get_username`: Current username
//! - `get_home_dir`: User's home directory
//! - `get_temp_dir`: System temporary directory
//! - `get_os`: Operating system name
//! - `get_arch`: CPU architecture
//! - `get_cwd`: Current working directory

use super::metadata::{FunctionMetadata, SyntaxVariants};
use super::traits::Function;
use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};
use std::env;

/// Get the system hostname
pub struct GetHostname;

impl Function for GetHostname {
    const NAME: &'static str = "get_hostname";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "get_hostname",
        category: "system",
        description: "Get the system hostname",
        arguments: &[],
        return_type: "string",
        examples: &["{{ get_hostname() }}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(_kwargs: Kwargs) -> Result<Value, Error> {
        let hostname = hostname::get()
            .map_err(|e| {
                Error::new(
                    ErrorKind::InvalidOperation,
                    format!("Failed to get hostname: {}", e),
                )
            })?
            .to_string_lossy()
            .to_string();

        Ok(Value::from(hostname))
    }
}

/// Get the current system username
pub struct GetUsername;

impl Function for GetUsername {
    const NAME: &'static str = "get_username";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "get_username",
        category: "system",
        description: "Get the current system username",
        arguments: &[],
        return_type: "string",
        examples: &["{{ get_username() }}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(_kwargs: Kwargs) -> Result<Value, Error> {
        let username = whoami::username();
        Ok(Value::from(username))
    }
}

/// Get the user's home directory
pub struct GetHomeDir;

impl Function for GetHomeDir {
    const NAME: &'static str = "get_home_dir";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "get_home_dir",
        category: "system",
        description: "Get the user's home directory path",
        arguments: &[],
        return_type: "string",
        examples: &["{{ get_home_dir() }}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(_kwargs: Kwargs) -> Result<Value, Error> {
        let home_dir = dirs::home_dir().ok_or_else(|| {
            Error::new(ErrorKind::InvalidOperation, "Failed to get home directory")
        })?;

        Ok(Value::from(home_dir.to_string_lossy().to_string()))
    }
}

/// Get the system temporary directory
pub struct GetTempDir;

impl Function for GetTempDir {
    const NAME: &'static str = "get_temp_dir";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "get_temp_dir",
        category: "system",
        description: "Get the system temporary directory path",
        arguments: &[],
        return_type: "string",
        examples: &["{{ get_temp_dir() }}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(_kwargs: Kwargs) -> Result<Value, Error> {
        let temp_dir = env::temp_dir();
        Ok(Value::from(temp_dir.to_string_lossy().to_string()))
    }
}

/// Get the operating system name
pub struct GetOs;

impl Function for GetOs {
    const NAME: &'static str = "get_os";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "get_os",
        category: "system",
        description: "Get the operating system name (e.g., linux, macos, windows)",
        arguments: &[],
        return_type: "string",
        examples: &[
            "{{ get_os() }}",
            "{% if get_os() == \"linux\" %}Linux{% endif %}",
        ],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(_kwargs: Kwargs) -> Result<Value, Error> {
        let os = env::consts::OS;
        Ok(Value::from(os))
    }
}

/// Get the CPU architecture
pub struct GetArch;

impl Function for GetArch {
    const NAME: &'static str = "get_arch";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "get_arch",
        category: "system",
        description: "Get the CPU architecture (e.g., x86_64, aarch64, arm)",
        arguments: &[],
        return_type: "string",
        examples: &[
            "{{ get_arch() }}",
            "{% if get_arch() == \"aarch64\" %}ARM64{% endif %}",
        ],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(_kwargs: Kwargs) -> Result<Value, Error> {
        let arch = env::consts::ARCH;
        Ok(Value::from(arch))
    }
}

/// Get the current working directory
pub struct GetCwd;

impl Function for GetCwd {
    const NAME: &'static str = "get_cwd";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "get_cwd",
        category: "system",
        description: "Get the current working directory path",
        arguments: &[],
        return_type: "string",
        examples: &["{{ get_cwd() }}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(_kwargs: Kwargs) -> Result<Value, Error> {
        let cwd = env::current_dir().map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Failed to get current working directory: {}", e),
            )
        })?;
        Ok(Value::from(cwd.to_string_lossy().to_string()))
    }
}
