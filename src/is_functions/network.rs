//! Network is-functions for tmpltool
//!
//! This module provides network validation functions that work with both syntaxes:
//! - Function syntax: `{{ is_port_available(port=8080) }}`
//! - Is-test syntax: `{% if 8080 is port_available %}`
//!
//! # Available Network Functions
//!
//! - `is_port_available` / `port_available` - Check if a TCP port is available
//!
//! # Example Usage
//!
//! ```jinja
//! {# Function syntax #}
//! {% if is_port_available(port=8080) %}port is free{% endif %}
//!
//! {# Is-test syntax (preferred for readability) #}
//! {% if 8080 is port_available %}port is free{% endif %}
//! ```

use crate::is_functions::IsFunction;
use minijinja::value::Kwargs;
use minijinja::{Environment, Error, ErrorKind, Value};
use std::net::TcpListener;

/// Port availability check is-function
///
/// Checks if a TCP port is available for binding by attempting to bind to it.
///
/// # Function Syntax
/// ```jinja
/// {{ is_port_available(port=8080) }}
/// {% if is_port_available(port=port_var) %}...{% endif %}
/// ```
///
/// # Is-Test Syntax
/// ```jinja
/// {% if 8080 is port_available %}port is free{% endif %}
/// {% if port_var is port_available %}port is free{% endif %}
/// ```
///
/// # Valid Port Range
/// Port must be between 1 and 65535.
pub struct PortAvailable;

impl PortAvailable {
    /// Check if a port is available for binding
    ///
    /// Attempts to bind to the port on all interfaces (0.0.0.0).
    /// Returns true if binding succeeds (port is available).
    pub fn is_available(port: u16) -> bool {
        TcpListener::bind(("0.0.0.0", port)).is_ok()
    }

    /// Validate and convert a port number
    fn validate_port(port: i64) -> Result<u16, Error> {
        if (1..=65535).contains(&port) {
            Ok(port as u16)
        } else {
            Err(Error::new(
                ErrorKind::InvalidOperation,
                format!("Port must be between 1 and 65535, got {}", port),
            ))
        }
    }
}

impl IsFunction for PortAvailable {
    const FUNCTION_NAME: &'static str = "is_port_available";
    const IS_NAME: &'static str = "port_available";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let port_i64: i64 = kwargs.get("port")?;
        let port = Self::validate_port(port_i64)?;
        Ok(Value::from(Self::is_available(port)))
    }

    fn call_as_is(value: &Value) -> bool {
        // Try to extract a valid port number from the value
        let port = if let Some(n) = value.as_i64() {
            n
        } else if let Some(s) = value.as_str() {
            // Also support string representation of ports
            s.parse::<i64>().unwrap_or(-1)
        } else {
            return false;
        };

        if (1..=65535).contains(&port) {
            Self::is_available(port as u16)
        } else {
            false
        }
    }
}

/// Register all network is-functions with the MiniJinja environment
pub fn register_all(env: &mut Environment) {
    PortAvailable::register(env);
}
