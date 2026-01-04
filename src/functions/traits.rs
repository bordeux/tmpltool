//! Core traits for function implementations.
//!
//! This module provides traits for implementing template functions with required metadata.
//! All functions must implement one of these traits, ensuring metadata is always defined.
//!
//! # Trait Hierarchy
//!
//! - `Function` - Simple functions that don't need context (e.g., `get_env`, `uuid`)
//! - `ContextFunction` - Functions that need `TemplateContext` for filesystem access
//!
//! # Example
//!
//! ```rust,ignore
//! use crate::functions::traits::Function;
//! use crate::functions::metadata::{FunctionMetadata, ArgumentMetadata, SyntaxVariants};
//!
//! pub struct MyFunction;
//!
//! impl Function for MyFunction {
//!     const NAME: &'static str = "my_function";
//!     const METADATA: FunctionMetadata = FunctionMetadata {
//!         name: "my_function",
//!         category: "example",
//!         description: "Does something useful",
//!         arguments: &[],
//!         return_type: "string",
//!         examples: &["{{ my_function() }}"],
//!         syntax: SyntaxVariants::FUNCTION_ONLY,
//!     };
//!
//!     fn call(kwargs: Kwargs) -> Result<Value, Error> {
//!         Ok(Value::from("result"))
//!     }
//! }
//!
//! // Registration:
//! MyFunction::register(&mut env);
//! ```

use super::metadata::FunctionMetadata;
use crate::TemplateContext;
use minijinja::value::Kwargs;
use minijinja::{Environment, Error, Value};
use std::sync::Arc;

/// Trait for simple functions that don't require context.
///
/// Use this for functions that:
/// - Don't need filesystem access
/// - Don't need trust mode checks
/// - Work purely with their input arguments
///
/// # Example
///
/// ```rust,ignore
/// pub struct GetEnv;
///
/// impl Function for GetEnv {
///     const NAME: &'static str = "get_env";
///     const METADATA: FunctionMetadata = FunctionMetadata { ... };
///
///     fn call(kwargs: Kwargs) -> Result<Value, Error> {
///         let name: String = kwargs.get("name")?;
///         // ...
///     }
/// }
/// ```
pub trait Function: 'static {
    /// Function name used for registration.
    const NAME: &'static str;

    /// Metadata describing this function (required for IDE integration).
    const METADATA: FunctionMetadata;

    /// Handle function calls.
    ///
    /// All arguments come from kwargs (named arguments).
    fn call(kwargs: Kwargs) -> Result<Value, Error>;

    /// Register this function with the MiniJinja environment.
    fn register(env: &mut Environment) {
        env.add_function(Self::NAME, Self::call);
    }
}

/// Trait for functions that require `TemplateContext` for filesystem or security operations.
///
/// Use this for functions that:
/// - Need to access the filesystem
/// - Need to check trust mode
/// - Need path resolution
///
/// # Example
///
/// ```rust,ignore
/// pub struct ReadFile;
///
/// impl ContextFunction for ReadFile {
///     const NAME: &'static str = "read_file";
///     const METADATA: FunctionMetadata = FunctionMetadata { ... };
///
///     fn call(context: Arc<TemplateContext>, kwargs: Kwargs) -> Result<Value, Error> {
///         let path: String = kwargs.get("path")?;
///         let resolved = context.validate_and_resolve_path(&path)?;
///         // ...
///     }
/// }
/// ```
pub trait ContextFunction: 'static {
    /// Function name used for registration.
    const NAME: &'static str;

    /// Metadata describing this function (required for IDE integration).
    const METADATA: FunctionMetadata;

    /// Handle function calls with context access.
    fn call(context: Arc<TemplateContext>, kwargs: Kwargs) -> Result<Value, Error>;

    /// Register this function with the MiniJinja environment.
    ///
    /// Creates a closure that captures the context for use in the function.
    fn register(env: &mut Environment, context: Arc<TemplateContext>) {
        let ctx = context.clone();
        env.add_function(Self::NAME, move |kwargs: Kwargs| {
            Self::call(ctx.clone(), kwargs)
        });
    }
}
