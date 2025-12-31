//! System information functions for MiniJinja templates
//!
//! This module provides functions to access system information like:
//! - Hostname
//! - Username
//! - Home directory
//! - Temporary directory

use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};
use std::env;

/// Get the system hostname
///
/// # Arguments
///
/// This function takes no arguments (but MiniJinja requires Kwargs parameter)
///
/// # Returns
///
/// Returns the system hostname as a string
///
/// # Example
///
/// ```jinja
/// Hostname: {{ get_hostname() }}
/// ```
pub fn get_hostname_fn(_kwargs: Kwargs) -> Result<Value, Error> {
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

/// Get the current system username
///
/// # Arguments
///
/// This function takes no arguments (but MiniJinja requires Kwargs parameter)
///
/// # Returns
///
/// Returns the current username as a string
///
/// # Example
///
/// ```jinja
/// User: {{ get_username() }}
/// ```
pub fn get_username_fn(_kwargs: Kwargs) -> Result<Value, Error> {
    let username = whoami::username();
    Ok(Value::from(username))
}

/// Get the user's home directory
///
/// # Arguments
///
/// This function takes no arguments (but MiniJinja requires Kwargs parameter)
///
/// # Returns
///
/// Returns the home directory path as a string
///
/// # Example
///
/// ```jinja
/// Home: {{ get_home_dir() }}
/// ```
pub fn get_home_dir_fn(_kwargs: Kwargs) -> Result<Value, Error> {
    let home_dir = dirs::home_dir()
        .ok_or_else(|| Error::new(ErrorKind::InvalidOperation, "Failed to get home directory"))?;

    Ok(Value::from(home_dir.to_string_lossy().to_string()))
}

/// Get the system temporary directory
///
/// # Arguments
///
/// This function takes no arguments (but MiniJinja requires Kwargs parameter)
///
/// # Returns
///
/// Returns the temporary directory path as a string
///
/// # Example
///
/// ```jinja
/// Temp dir: {{ get_temp_dir() }}
/// ```
pub fn get_temp_dir_fn(_kwargs: Kwargs) -> Result<Value, Error> {
    let temp_dir = env::temp_dir();
    Ok(Value::from(temp_dir.to_string_lossy().to_string()))
}
