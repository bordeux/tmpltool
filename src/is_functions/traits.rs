//! Core traits for is-functions that support both function and "is" test syntax.
//!
//! This module provides the `IsFunction` trait which allows a single implementation
//! to be registered as both a MiniJinja function and test (for `{% if x is name %}` syntax).
//!
//! # Example
//!
//! ```rust,ignore
//! use crate::is_functions::traits::IsFunction;
//! use crate::functions::metadata::{FunctionMetadata, ArgumentMetadata, SyntaxVariants};
//!
//! pub struct Email;
//!
//! impl IsFunction for Email {
//!     const FUNCTION_NAME: &'static str = "is_email";
//!     const IS_NAME: &'static str = "email";
//!     const METADATA: FunctionMetadata = FunctionMetadata {
//!         name: "is_email",
//!         category: "validation",
//!         description: "Validate email address format",
//!         arguments: &[ArgumentMetadata {
//!             name: "string",
//!             arg_type: "string",
//!             required: true,
//!             default: None,
//!             description: "The string to validate",
//!         }],
//!         return_type: "boolean",
//!         examples: &[
//!             "{{ is_email(string=\"user@example.com\") }}",
//!             "{% if email is email %}valid{% endif %}",
//!         ],
//!         syntax: SyntaxVariants::FUNCTION_AND_TEST,
//!     };
//!
//!     fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
//!         let input: String = kwargs.get("string")?;
//!         Ok(Value::from(Self::validate(&input)))
//!     }
//!
//!     fn call_as_is(value: &Value) -> bool {
//!         value.as_str().map(|s| Self::validate(s)).unwrap_or(false)
//!     }
//! }
//!
//! // Registration:
//! Email::register(&mut env);
//! ```

use crate::TemplateContext;
use crate::functions::metadata::FunctionMetadata;
use minijinja::value::Kwargs;
use minijinja::{Environment, Error, Value};
use std::sync::Arc;

/// Trait for types that can be registered as both a MiniJinja function and test.
///
/// Implementors define how to handle both calling conventions:
/// - Function: `{{ is_email(string="value") }}` or `{% if is_email(string=x) %}`
/// - Is-test: `{% if value is email %}` - value is passed as first argument
///
/// # Usage
///
/// Both syntaxes are equivalent and produce the same result:
/// ```jinja
/// {% if is_email(string=user_input) %}valid{% endif %}
/// {% if user_input is email %}valid{% endif %}
/// ```
pub trait IsFunction: 'static {
    /// Function name WITH "is_" prefix (e.g., "is_email").
    ///
    /// This name is used for function registration: `is_email(string="...")`
    const FUNCTION_NAME: &'static str;

    /// Test name WITHOUT "is_" prefix (e.g., "email").
    ///
    /// This name is used for test registration: `{% if x is email %}`
    /// The `is` keyword in the template provides the "is_" semantics.
    const IS_NAME: &'static str;

    /// Metadata describing this function (required for IDE integration).
    const METADATA: FunctionMetadata;

    /// Handle function-style calls where all arguments come from kwargs.
    ///
    /// Example: `{{ is_email(string="hello@example.com") }}`
    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error>;

    /// Handle is-style calls where the value is passed directly.
    ///
    /// Example: `{% if "hello@example.com" is email %}`
    ///
    /// Returns `true` if the value passes the test, `false` otherwise.
    /// Should not panic or error - invalid values should return `false`.
    fn call_as_is(value: &Value) -> bool;

    /// Register this is-function with the MiniJinja environment.
    ///
    /// This registers the implementation as both a function and a test,
    /// allowing users to choose their preferred syntax.
    fn register(env: &mut Environment) {
        env.add_function(Self::FUNCTION_NAME, Self::call_as_function);
        env.add_test(Self::IS_NAME, Self::call_as_is);
    }
}

/// Trait for context-aware is-functions that need filesystem access.
///
/// This is similar to `IsFunction` but for functions that need access to
/// `TemplateContext` for path resolution and security checks.
///
/// # Example
///
/// ```rust,ignore
/// pub struct File;
///
/// impl ContextIsFunction for File {
///     const FUNCTION_NAME: &'static str = "is_file";
///     const IS_NAME: &'static str = "file";
///
///     fn call_as_function(context: Arc<TemplateContext>, kwargs: Kwargs) -> Result<Value, Error> {
///         let path: String = kwargs.get("path")?;
///         let resolved = context.validate_and_resolve_path(&path)?;
///         Ok(Value::from(resolved.is_file()))
///     }
///
///     fn call_as_is(context: Arc<TemplateContext>, value: &Value) -> bool {
///         value.as_str()
///             .and_then(|s| context.validate_and_resolve_path(s).ok())
///             .map(|p| p.is_file())
///             .unwrap_or(false)
///     }
/// }
/// ```
pub trait ContextIsFunction: 'static {
    /// Function name WITH "is_" prefix (e.g., "is_file").
    const FUNCTION_NAME: &'static str;

    /// Test name WITHOUT "is_" prefix (e.g., "file").
    const IS_NAME: &'static str;

    /// Metadata describing this function (required for IDE integration).
    const METADATA: FunctionMetadata;

    /// Handle function-style calls with context access.
    ///
    /// Example: `{{ is_file(path="config.json") }}`
    fn call_as_function(context: Arc<TemplateContext>, kwargs: Kwargs) -> Result<Value, Error>;

    /// Handle is-style calls with context access.
    ///
    /// Example: `{% if "config.json" is file %}`
    ///
    /// Returns `true` if the value passes the test, `false` otherwise.
    /// Security violations or invalid paths should return `false`.
    fn call_as_is(context: Arc<TemplateContext>, value: &Value) -> bool;

    /// Register this context-aware is-function with the MiniJinja environment.
    ///
    /// This creates closures that capture the context and registers them
    /// as both a function and a test.
    fn register(env: &mut Environment, context: Arc<TemplateContext>) {
        let ctx = context.clone();
        env.add_function(Self::FUNCTION_NAME, move |kwargs: Kwargs| {
            Self::call_as_function(ctx.clone(), kwargs)
        });

        let ctx = context.clone();
        env.add_test(Self::IS_NAME, move |value: &Value| {
            Self::call_as_is(ctx.clone(), value)
        });
    }
}
