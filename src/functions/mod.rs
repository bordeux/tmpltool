//! Custom MiniJinja functions for tmpltool
//!
//! This module contains all custom functions that can be used in MiniJinja templates.
//! Each function is defined in its own file for better organization and maintainability.
//!
//! # Built-in Functions
//!
//! MiniJinja provides these functions (implemented in the builtins module):
//! - `env(name, default)` - Get environment variables with optional default values
//! - `now()` - Get current timestamp
//! - `get_random(start, end)` - Generate random integers
//!
//! # Custom Functions
//!
//! tmpltool provides additional custom functions:
//! - `filter_env(pattern)` - Filter environment variables by glob pattern (e.g., "SERVER_*")
//! - `md5(string)` - Calculate MD5 hash of a string
//! - `sha1(string)` - Calculate SHA1 hash of a string
//! - `sha256(string)` - Calculate SHA256 hash of a string
//! - `sha512(string)` - Calculate SHA512 hash of a string
//! - `uuid()` - Generate a random UUID v4
//! - `random_string(length, charset)` - Generate a random string with custom length and character set
//! - `read_file(path)` - Read content from a file
//! - `file_exists(path)` - Check if a file exists
//! - `list_dir(path)` - List files in a directory
//! - `glob(pattern)` - List files matching a glob pattern
//! - `file_size(path)` - Get file size in bytes
//! - `file_modified(path)` - Get file modification timestamp
//! - `is_email(string)` - Validate email address format
//! - `is_url(string)` - Validate URL format
//! - `is_ip(string)` - Validate IP address (IPv4 or IPv6)
//! - `is_uuid(string)` - Validate UUID format
//! - `matches_regex(pattern, string)` - Check if string matches regex pattern
//! - `parse_json(string)` - Parse JSON string into object
//! - `parse_yaml(string)` - Parse YAML string into object
//! - `parse_toml(string)` - Parse TOML string into object
//! - `read_json_file(path)` - Read and parse JSON file
//! - `read_yaml_file(path)` - Read and parse YAML file
//! - `read_toml_file(path)` - Read and parse TOML file
//!
//! # Adding Custom Functions
//!
//! To add a new custom function:
//!
//! 1. Create a new file in `src/functions/` (e.g., `my_function.rs`)
//! 2. Implement your function with signature: `fn my_function(args: &HashMap<String, Value>) -> tera::Result<Value>`
//! 3. Add `pub mod my_function;` to this file
//! 4. Add your function to the `register_all()` function below
//!
//! # Example
//!
//! ```rust
//! // In src/functions/my_function.rs
//! use std::collections::HashMap;
//! use tera::Value;
//!
//! pub fn my_function(args: &HashMap<String, Value>) -> tera::Result<Value> {
//!     // Your implementation here
//!     Ok(Value::String("result".to_string()))
//! }
//! ```

pub mod builtins;
pub mod data_parsing;
pub mod filesystem;
pub mod filter_env;
pub mod hash;
pub mod random_string;
pub mod uuid_gen;
pub mod validation;

use crate::TemplateContext;
use minijinja::Environment;

/// Register all custom functions with the MiniJinja environment
///
/// This function is called when setting up a MiniJinja environment to register
/// all custom functions, including built-in function replacements.
///
/// # Arguments
///
/// * `env` - Mutable reference to a MiniJinja Environment
/// * `context` - Template context with base directory and trust mode settings
///
/// # Example
///
/// ```
/// use minijinja::Environment;
/// use tmpltool::{TemplateContext, functions::register_all};
/// use std::path::PathBuf;
///
/// let mut env = Environment::new();
/// let ctx = TemplateContext::new(PathBuf::from("."), false);
/// register_all(&mut env, ctx);
/// ```
pub fn register_all(env: &mut Environment, context: TemplateContext) {
    use std::sync::Arc;

    // Register built-in function replacements (were built-in in Tera)
    env.add_function("get_env", builtins::env_fn);
    env.add_function("now", builtins::now_fn);
    env.add_function("get_random", builtins::get_random_fn);

    // Register custom functions (simple, no context needed)
    env.add_function("filter_env", filter_env::filter_env_fn);
    env.add_function("md5", hash::md5_fn);
    env.add_function("sha1", hash::sha1_fn);
    env.add_function("sha256", hash::sha256_fn);
    env.add_function("sha512", hash::sha512_fn);
    env.add_function("uuid", uuid_gen::uuid_fn);
    env.add_function("random_string", random_string::random_string_fn);

    // Validation functions
    env.add_function("is_email", validation::is_email_fn);
    env.add_function("is_url", validation::is_url_fn);
    env.add_function("is_ip", validation::is_ip_fn);
    env.add_function("is_uuid", validation::is_uuid_fn);
    env.add_function("matches_regex", validation::matches_regex_fn);

    // Data parsing functions (simple, no context)
    env.add_function("parse_json", data_parsing::parse_json_fn);
    env.add_function("parse_yaml", data_parsing::parse_yaml_fn);
    env.add_function("parse_toml", data_parsing::parse_toml_fn);

    // File system functions (need context)
    let context_arc = Arc::new(context);
    env.add_function("read_file", filesystem::create_read_file_fn(context_arc.clone()));
    env.add_function("file_exists", filesystem::create_file_exists_fn(context_arc.clone()));
    env.add_function("list_dir", filesystem::create_list_dir_fn(context_arc.clone()));
    env.add_function("glob", filesystem::create_glob_fn(context_arc.clone()));
    env.add_function("file_size", filesystem::create_file_size_fn(context_arc.clone()));
    env.add_function("file_modified", filesystem::create_file_modified_fn(context_arc.clone()));

    // Data parsing file functions (need context)
    env.add_function("read_json_file", data_parsing::create_read_json_file_fn(context_arc.clone()));
    env.add_function("read_yaml_file", data_parsing::create_read_yaml_file_fn(context_arc.clone()));
    env.add_function("read_toml_file", data_parsing::create_read_toml_file_fn(context_arc));
}
