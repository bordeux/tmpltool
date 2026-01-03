//! Core trait for filter-functions that support both function and filter syntax.
//!
//! This module provides the `FilterFunction` trait which allows a single implementation
//! to be registered as both a MiniJinja function and filter.
//!
//! # Example
//!
//! ```rust,ignore
//! use crate::filter_functions::traits::FilterFunction;
//!
//! pub struct Sha256;
//!
//! impl FilterFunction for Sha256 {
//!     const NAME: &'static str = "sha256";
//!
//!     fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
//!         let input: String = kwargs.get("string")?;
//!         Ok(Value::from(Self::hash(&input)))
//!     }
//!
//!     fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
//!         let input = value.as_str().ok_or_else(|| {
//!             Error::new(ErrorKind::InvalidOperation, "sha256 requires a string")
//!         })?;
//!         Ok(Value::from(Self::hash(input)))
//!     }
//! }
//!
//! // Registration:
//! Sha256::register(&mut env);
//! ```

use minijinja::value::Kwargs;
use minijinja::{Environment, Error, Value};

/// Trait for types that can be registered as both a MiniJinja function and filter.
///
/// Implementors define how to handle both calling conventions:
/// - Function: `{{ name(arg="value") }}` - all arguments come from kwargs
/// - Filter: `{{ value | name(arg="value") }}` - first arg is piped, rest from kwargs
///
/// # Usage
///
/// Both syntaxes are equivalent and produce the same result:
/// ```jinja
/// {{ sha256(string="hello") }}
/// {{ "hello" | sha256 }}
/// ```
///
/// Filters can be chained:
/// ```jinja
/// {{ "hello" | sha256 | base64_encode }}
/// ```
pub trait FilterFunction: 'static {
    /// Name used for registration (e.g., "sha256", "format_date").
    ///
    /// This name is used for both the function and filter registration.
    const NAME: &'static str;

    /// Handle function-style calls where all arguments come from kwargs.
    ///
    /// Example: `{{ sha256(string="hello") }}`
    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error>;

    /// Handle filter-style calls where the primary value is piped in.
    ///
    /// Example: `{{ "hello" | sha256 }}`
    ///
    /// Additional arguments can still be passed via kwargs:
    /// Example: `{{ 3.14159 | round(decimals=2) }}`
    fn call_as_filter(value: &Value, kwargs: Kwargs) -> Result<Value, Error>;

    /// Register this filter-function with the MiniJinja environment.
    ///
    /// This registers the implementation as both a function and a filter,
    /// allowing users to choose their preferred syntax.
    ///
    /// Override this method for filters that need custom positional argument
    /// handling (e.g., `| indent(4)` instead of `| indent(spaces=4)`).
    fn register(env: &mut Environment) {
        env.add_function(Self::NAME, Self::call_as_function);
        env.add_filter(Self::NAME, Self::call_as_filter);
    }
}
