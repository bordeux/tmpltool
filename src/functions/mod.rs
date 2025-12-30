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

pub mod filter_env;
pub mod filesystem;
pub mod hash;
pub mod random_string;
pub mod uuid_gen;

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
///
/// # Example
///
/// ```
/// use tera::Tera;
/// use tmpltool::functions::register_all;
///
/// let mut tera = Tera::default();
/// register_all(&mut tera);
/// ```
pub fn register_all(tera: &mut Tera) {
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

    // File system functions
    tera.register_function("read_file", filesystem::ReadFile);
    tera.register_function("file_exists", filesystem::FileExists);
    tera.register_function("list_dir", filesystem::ListDir);
    tera.register_function("glob", filesystem::GlobFiles);
    tera.register_function("file_size", filesystem::FileSize);
    tera.register_function("file_modified", filesystem::FileModified);
}
