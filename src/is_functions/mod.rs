//! Unified is-functions module for tmpltool
//!
//! This module provides functions that work with both syntaxes:
//! - Function syntax: `{{ is_email(string="...") }}` or `{% if is_email(string=x) %}`
//! - Is-test syntax: `{% if x is email %}`
//!
//! The is-test syntax uses MiniJinja's native test feature, providing a more
//! readable way to write conditionals in templates.
//!
//! # Available Is-Functions
//!
//! ## Validation
//! - `is_email` / `{% if x is email %}` - Validate email address format
//! - `is_url` / `{% if x is url %}` - Validate URL format
//! - `is_ip` / `{% if x is ip %}` - Validate IP address (IPv4 or IPv6)
//! - `is_uuid` / `{% if x is uuid %}` - Validate UUID format
//!
//! ## DateTime
//! - `is_leap_year` / `{% if year is leap_year %}` - Check if year is a leap year
//!
//! ## Network
//! - `is_port_available` / `{% if port is port_available %}` - Check if port is available
//!
//! ## Filesystem (context-aware)
//! - `is_file` / `{% if path is file %}` - Check if path is a file
//! - `is_dir` / `{% if path is dir %}` - Check if path is a directory
//! - `is_symlink` / `{% if path is symlink %}` - Check if path is a symlink
//!
//! # Example Usage
//!
//! ```jinja
//! {# Function syntax #}
//! {% if is_email(string=user_input) %}
//!   Valid email!
//! {% endif %}
//!
//! {# Is-test syntax (preferred for readability) #}
//! {% if user_input is email %}
//!   Valid email!
//! {% endif %}
//!
//! {# Filesystem checks #}
//! {% if "config.json" is file %}
//!   {% set config = read_json_file(path="config.json") %}
//! {% endif %}
//! ```

pub mod datetime;
pub mod filesystem;
pub mod network;
pub mod traits;
pub mod validation;

pub use traits::{ContextIsFunction, IsFunction};

use crate::TemplateContext;
use minijinja::Environment;
use std::sync::Arc;

/// Register all is-functions with the MiniJinja environment.
///
/// This registers each is-function as both:
/// - A function: `is_name(arg="value")`
/// - A test: `{% if value is name %}`
///
/// # Arguments
///
/// * `env` - Mutable reference to a MiniJinja Environment
/// * `context` - Template context for filesystem operations (wrapped in Arc)
///
/// # Example
///
/// ```rust,ignore
/// use minijinja::Environment;
/// use tmpltool::TemplateContext;
/// use tmpltool::is_functions::register_all;
/// use std::sync::Arc;
/// use std::path::PathBuf;
///
/// let mut env = Environment::new();
/// let ctx = Arc::new(TemplateContext::new(PathBuf::from("."), false));
/// register_all(&mut env, ctx);
/// ```
pub fn register_all(env: &mut Environment, context: Arc<TemplateContext>) {
    // Phase 2: Validation functions (email, url, ip, uuid)
    validation::register_all(env);

    // Phase 3: DateTime functions (leap_year)
    datetime::register_all(env);

    // Phase 4: Network functions (port_available)
    network::register_all(env);

    // Phase 5: Filesystem functions (file, dir, symlink) - context-aware
    filesystem::register_all(env, context);
}
