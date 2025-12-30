//! Custom Tera functions for tmpltool
//!
//! This module contains all custom functions that can be used in Tera templates.
//! Each function is defined in its own file for better organization and maintainability.
//!
//! # Available Functions
//!
//! - `env()` - Get environment variables with optional default values
//!
//! # Adding New Functions
//!
//! To add a new custom function:
//!
//! 1. Create a new file in `src/functions/` (e.g., `my_function.rs`)
//! 2. Implement your function with signature: `fn my_function(args: &HashMap<String, Value>) -> tera::Result<Value>`
//! 3. Add `mod my_function;` to this file
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

pub mod env;

use tera::Tera;

/// Register all custom functions with the Tera instance
///
/// This function should be called when setting up a Tera instance to make
/// all custom functions available in templates.
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
    // Register the env() function
    tera.register_function("env", env::env_function);

    // Add more function registrations here as you create them
    // Example:
    // tera.register_function("uppercase", uppercase::uppercase_function);
    // tera.register_function("date", date::date_function);
}
