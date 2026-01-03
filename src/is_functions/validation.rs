//! Validation is-functions for tmpltool
//!
//! This module provides validation functions that work with both syntaxes:
//! - Function syntax: `{{ is_email(string="...") }}`
//! - Is-test syntax: `{% if value is email %}`
//!
//! # Available Validation Functions
//!
//! - `is_email` / `email` - Validate email address format
//! - `is_url` / `url` - Validate URL format
//! - `is_ip` / `ip` - Validate IP address (IPv4 or IPv6)
//! - `is_uuid` / `uuid` - Validate UUID format
//!
//! # Example Usage
//!
//! ```jinja
//! {# Function syntax #}
//! {% if is_email(string=user_input) %}valid{% endif %}
//!
//! {# Is-test syntax (preferred for readability) #}
//! {% if user_input is email %}valid{% endif %}
//! ```

use crate::is_functions::IsFunction;
use minijinja::value::Kwargs;
use minijinja::{Environment, Error, Value};
use regex::Regex;

/// Email validation is-function
///
/// Validates email address format using a regex pattern.
///
/// # Function Syntax
/// ```jinja
/// {{ is_email(string="user@example.com") }}
/// {% if is_email(string=email_var) %}...{% endif %}
/// ```
///
/// # Is-Test Syntax
/// ```jinja
/// {% if "user@example.com" is email %}valid{% endif %}
/// {% if email_var is email %}valid{% endif %}
/// ```
pub struct Email;

impl Email {
    /// Regex pattern for email validation
    /// This is not exhaustive but covers most common cases
    const EMAIL_PATTERN: &'static str = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$";

    /// Validate an email string
    pub fn validate(s: &str) -> bool {
        let re = Regex::new(Self::EMAIL_PATTERN).unwrap();
        re.is_match(s)
    }
}

impl IsFunction for Email {
    const FUNCTION_NAME: &'static str = "is_email";
    const IS_NAME: &'static str = "email";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let string: String = kwargs.get("string")?;
        Ok(Value::from(Self::validate(&string)))
    }

    fn call_as_is(value: &Value) -> bool {
        value.as_str().map(Self::validate).unwrap_or(false)
    }
}

/// URL validation is-function
///
/// Validates URL format (supports http, https, ftp, file schemes).
///
/// # Function Syntax
/// ```jinja
/// {{ is_url(string="https://example.com") }}
/// {% if is_url(string=url_var) %}...{% endif %}
/// ```
///
/// # Is-Test Syntax
/// ```jinja
/// {% if "https://example.com" is url %}valid{% endif %}
/// {% if url_var is url %}valid{% endif %}
/// ```
pub struct Url;

impl Url {
    /// Regex pattern for URL validation
    const URL_PATTERN: &'static str =
        r"^(https?|ftp|file)://[-A-Za-z0-9+&@#/%?=~_|!:,.;]*[-A-Za-z0-9+&@#/%=~_|]$";

    /// Validate a URL string
    pub fn validate(s: &str) -> bool {
        let re = Regex::new(Self::URL_PATTERN).unwrap();
        re.is_match(s)
    }
}

impl IsFunction for Url {
    const FUNCTION_NAME: &'static str = "is_url";
    const IS_NAME: &'static str = "url";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let string: String = kwargs.get("string")?;
        Ok(Value::from(Self::validate(&string)))
    }

    fn call_as_is(value: &Value) -> bool {
        value.as_str().map(Self::validate).unwrap_or(false)
    }
}

/// IP address validation is-function
///
/// Validates IPv4 or IPv6 address format.
///
/// # Function Syntax
/// ```jinja
/// {{ is_ip(string="192.168.1.1") }}
/// {{ is_ip(string="::1") }}
/// {% if is_ip(string=ip_var) %}...{% endif %}
/// ```
///
/// # Is-Test Syntax
/// ```jinja
/// {% if "192.168.1.1" is ip %}valid{% endif %}
/// {% if "::1" is ip %}valid{% endif %}
/// {% if ip_var is ip %}valid{% endif %}
/// ```
pub struct Ip;

impl Ip {
    /// Validate an IP address string (IPv4 or IPv6)
    pub fn validate(s: &str) -> bool {
        s.parse::<std::net::IpAddr>().is_ok()
    }
}

impl IsFunction for Ip {
    const FUNCTION_NAME: &'static str = "is_ip";
    const IS_NAME: &'static str = "ip";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let string: String = kwargs.get("string")?;
        Ok(Value::from(Self::validate(&string)))
    }

    fn call_as_is(value: &Value) -> bool {
        value.as_str().map(Self::validate).unwrap_or(false)
    }
}

/// UUID validation is-function
///
/// Validates UUID format (supports all UUID versions).
///
/// # Function Syntax
/// ```jinja
/// {{ is_uuid(string="550e8400-e29b-41d4-a716-446655440000") }}
/// {% if is_uuid(string=uuid_var) %}...{% endif %}
/// ```
///
/// # Is-Test Syntax
/// ```jinja
/// {% if "550e8400-e29b-41d4-a716-446655440000" is uuid %}valid{% endif %}
/// {% if uuid_var is uuid %}valid{% endif %}
/// ```
pub struct Uuid;

impl Uuid {
    /// Regex pattern for UUID validation (all versions)
    const UUID_PATTERN: &'static str =
        r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$";

    /// Validate a UUID string
    pub fn validate(s: &str) -> bool {
        let re = Regex::new(Self::UUID_PATTERN).unwrap();
        re.is_match(s)
    }
}

impl IsFunction for Uuid {
    const FUNCTION_NAME: &'static str = "is_uuid";
    const IS_NAME: &'static str = "uuid";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let string: String = kwargs.get("string")?;
        Ok(Value::from(Self::validate(&string)))
    }

    fn call_as_is(value: &Value) -> bool {
        value.as_str().map(Self::validate).unwrap_or(false)
    }
}

/// Register all validation is-functions with the MiniJinja environment
pub fn register_all(env: &mut Environment) {
    Email::register(env);
    Url::register(env);
    Ip::register(env);
    Uuid::register(env);
}
