//! Custom filters for MiniJinja templates
//!
//! This module contains custom filters organized by category.
//!
//! # Filter Categories
//!
//! - **String Filters** (`string` module): Text manipulation filters
//!   - `slugify` - Convert strings to URL-friendly slugs
//!
//! - **Formatting Filters** (`formatting` module): Data formatting filters
//!   - `filesizeformat` - Format bytes as human-readable file sizes
//!   - `urlencode` - URL-encode strings for safe URL usage
//!
//! # Adding Custom Filters
//!
//! To add a new custom filter:
//!
//! 1. Choose or create an appropriate category module in `src/filters/`
//! 2. Implement your filter function with signature: `fn my_filter(value: &Value) -> Result<T, Error>`
//! 3. Add `pub mod category;` to this file if it's a new category
//! 4. Add your filter to the `register_all()` function below
//!
//! # Example
//!
//! ```rust
//! // In src/filters/string.rs
//! use minijinja::Value;
//!
//! pub fn my_filter(value: &Value) -> Result<String, minijinja::Error> {
//!     let s = value.as_str().ok_or_else(|| {
//!         minijinja::Error::new(
//!             minijinja::ErrorKind::InvalidOperation,
//!             "my_filter requires a string",
//!         )
//!     })?;
//!
//!     // Your implementation here
//!     Ok(s.to_uppercase())
//! }
//! ```

pub mod formatting;
pub mod string;

use minijinja::Environment;

/// Register all custom filters with the MiniJinja environment
///
/// This function is called when setting up a MiniJinja environment to register
/// all custom filters.
///
/// # Arguments
///
/// * `env` - Mutable reference to a MiniJinja Environment
///
/// # Example
///
/// ```
/// use minijinja::Environment;
/// use tmpltool::filters;
///
/// let mut env = Environment::new();
/// filters::register_all(&mut env);
/// ```
pub fn register_all(env: &mut Environment) {
    // String filters
    env.add_filter("slugify", string::slugify_filter);

    // Formatting filters
    env.add_filter("filesizeformat", formatting::filesizeformat_filter);
    env.add_filter("urlencode", formatting::urlencode_filter);
}
