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
pub fn register_all(_tera: &mut Tera) {
    // Add custom function registrations here as you create them
    // Example:
    // tera.register_function("my_function", my_function::my_function);
}
