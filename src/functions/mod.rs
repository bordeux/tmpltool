//! Custom Tera functions for tmpltool
//!
//! This module contains all custom functions that can be used in Tera templates.
//! Each function is defined in its own file for better organization and maintainability.
//!
//! # Built-in Functions
//!
//! Tera provides built-in functions when the "builtins" feature is enabled:
//! - `get_env(name, default)` - Get environment variables with optional default values
//! - `now()` - Get current timestamp
//! - `get_random(start, end)` - Generate random integers
//! - And many built-in filters: slugify, date, filesizeformat, urlencode, etc.
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

pub mod data_parsing;
pub mod filesystem;
pub mod filter_env;
pub mod hash;
pub mod random_string;
pub mod uuid_gen;
pub mod validation;

use crate::TemplateContext;
use tera::Tera;

/// Register all custom functions with the Tera instance
///
/// This function is called when setting up a Tera instance to register
/// any custom functions. Built-in functions like get_env() are automatically
/// available when the "builtins" feature is enabled.
///
/// # Arguments
///
/// * `tera` - Mutable reference to a Tera instance
/// * `context` - Template context with base directory and trust mode settings
///
/// # Example
///
/// ```
/// use tera::Tera;
/// use tmpltool::{TemplateContext, functions::register_all};
/// use std::path::PathBuf;
///
/// let mut tera = Tera::default();
/// let ctx = TemplateContext::new(PathBuf::from("."), false);
/// register_all(&mut tera, ctx);
/// ```
pub fn register_all(tera: &mut Tera, context: TemplateContext) {
    // Register custom functions
    tera.register_function("filter_env", filter_env::FilterEnv);

    // Hash functions
    tera.register_function("md5", hash::Md5);
    tera.register_function("sha1", hash::Sha1);
    tera.register_function("sha256", hash::Sha256);
    tera.register_function("sha512", hash::Sha512);

    // UUID generation
    tera.register_function("uuid", uuid_gen::UuidV4);

    // Random string generation
    tera.register_function("random_string", random_string::RandomString);

    // File system functions (with context containing base path and trust mode)
    tera.register_function("read_file", filesystem::ReadFile::new(context.clone()));
    tera.register_function("file_exists", filesystem::FileExists::new(context.clone()));
    tera.register_function("list_dir", filesystem::ListDir::new(context.clone()));
    tera.register_function("glob", filesystem::GlobFiles::new(context.clone()));
    tera.register_function("file_size", filesystem::FileSize::new(context.clone()));
    tera.register_function(
        "file_modified",
        filesystem::FileModified::new(context.clone()),
    );

    // Validation functions
    tera.register_function("is_email", validation::IsEmail);
    tera.register_function("is_url", validation::IsUrl);
    tera.register_function("is_ip", validation::IsIp);
    tera.register_function("is_uuid", validation::IsUuid);
    tera.register_function("matches_regex", validation::MatchesRegex);

    // Data parsing functions
    tera.register_function("parse_json", data_parsing::ParseJson);
    tera.register_function("parse_yaml", data_parsing::ParseYaml);
    tera.register_function("parse_toml", data_parsing::ParseToml);
    tera.register_function(
        "read_json_file",
        data_parsing::ReadJsonFile::new(context.clone()),
    );
    tera.register_function(
        "read_yaml_file",
        data_parsing::ReadYamlFile::new(context.clone()),
    );
    tera.register_function("read_toml_file", data_parsing::ReadTomlFile::new(context));
}
