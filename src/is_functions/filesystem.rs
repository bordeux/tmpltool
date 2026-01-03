//! Filesystem is-functions for tmpltool
//!
//! This module provides filesystem check functions that work with both syntaxes:
//! - Function syntax: `{{ is_file(path="config.json") }}`
//! - Is-test syntax: `{% if "config.json" is file %}`
//!
//! These functions are context-aware and resolve paths relative to the template's
//! base directory.
//!
//! # Available Filesystem Functions
//!
//! - `is_file` / `file` - Check if path is a file
//! - `is_dir` / `dir` - Check if path is a directory
//! - `is_symlink` / `symlink` - Check if path is a symbolic link
//!
//! # Example Usage
//!
//! ```jinja
//! {# Function syntax #}
//! {% if is_file(path="config.json") %}config exists{% endif %}
//!
//! {# Is-test syntax (preferred for readability) #}
//! {% if "config.json" is file %}config exists{% endif %}
//! {% if "src" is dir %}source directory exists{% endif %}
//! ```

use crate::TemplateContext;
use crate::is_functions::ContextIsFunction;
use minijinja::value::Kwargs;
use minijinja::{Environment, Error, Value};
use std::sync::Arc;

/// File existence check is-function
///
/// Checks if a path exists and is a regular file.
///
/// # Function Syntax
/// ```jinja
/// {{ is_file(path="config.json") }}
/// {% if is_file(path=file_var) %}...{% endif %}
/// ```
///
/// # Is-Test Syntax
/// ```jinja
/// {% if "config.json" is file %}file exists{% endif %}
/// {% if file_var is file %}file exists{% endif %}
/// ```
pub struct File;

impl ContextIsFunction for File {
    const FUNCTION_NAME: &'static str = "is_file";
    const IS_NAME: &'static str = "file";

    fn call_as_function(context: Arc<TemplateContext>, kwargs: Kwargs) -> Result<Value, Error> {
        let path: String = kwargs.get("path")?;
        let resolved = context.resolve_path(&path);
        Ok(Value::from(resolved.is_file()))
    }

    fn call_as_is(context: Arc<TemplateContext>, value: &Value) -> bool {
        value
            .as_str()
            .map(|s| context.resolve_path(s).is_file())
            .unwrap_or(false)
    }
}

/// Directory existence check is-function
///
/// Checks if a path exists and is a directory.
///
/// # Function Syntax
/// ```jinja
/// {{ is_dir(path="src") }}
/// {% if is_dir(path=dir_var) %}...{% endif %}
/// ```
///
/// # Is-Test Syntax
/// ```jinja
/// {% if "src" is dir %}directory exists{% endif %}
/// {% if dir_var is dir %}directory exists{% endif %}
/// ```
pub struct Dir;

impl ContextIsFunction for Dir {
    const FUNCTION_NAME: &'static str = "is_dir";
    const IS_NAME: &'static str = "dir";

    fn call_as_function(context: Arc<TemplateContext>, kwargs: Kwargs) -> Result<Value, Error> {
        let path: String = kwargs.get("path")?;
        let resolved = context.resolve_path(&path);
        Ok(Value::from(resolved.is_dir()))
    }

    fn call_as_is(context: Arc<TemplateContext>, value: &Value) -> bool {
        value
            .as_str()
            .map(|s| context.resolve_path(s).is_dir())
            .unwrap_or(false)
    }
}

/// Symlink check is-function
///
/// Checks if a path exists and is a symbolic link.
///
/// # Function Syntax
/// ```jinja
/// {{ is_symlink(path="link") }}
/// {% if is_symlink(path=link_var) %}...{% endif %}
/// ```
///
/// # Is-Test Syntax
/// ```jinja
/// {% if "link" is symlink %}it's a symlink{% endif %}
/// {% if link_var is symlink %}it's a symlink{% endif %}
/// ```
pub struct Symlink;

impl Symlink {
    /// Check if a path is a symlink
    fn is_symlink(path: &std::path::Path) -> bool {
        path.symlink_metadata()
            .map(|m| m.file_type().is_symlink())
            .unwrap_or(false)
    }
}

impl ContextIsFunction for Symlink {
    const FUNCTION_NAME: &'static str = "is_symlink";
    const IS_NAME: &'static str = "symlink";

    fn call_as_function(context: Arc<TemplateContext>, kwargs: Kwargs) -> Result<Value, Error> {
        let path: String = kwargs.get("path")?;
        let resolved = context.resolve_path(&path);
        Ok(Value::from(Self::is_symlink(&resolved)))
    }

    fn call_as_is(context: Arc<TemplateContext>, value: &Value) -> bool {
        value
            .as_str()
            .map(|s| Self::is_symlink(&context.resolve_path(s)))
            .unwrap_or(false)
    }
}

/// Register all filesystem is-functions with the MiniJinja environment
pub fn register_all(env: &mut Environment, context: Arc<TemplateContext>) {
    File::register(env, context.clone());
    Dir::register(env, context.clone());
    Symlink::register(env, context);
}
