//! Metadata types for function documentation and IDE integration.
//!
//! This module provides types for describing function metadata, including
//! arguments, return types, examples, and supported syntax variants.
//!
//! # Usage
//!
//! All function traits require a `METADATA` constant that describes the function:
//!
//! ```rust,ignore
//! impl Function for MyFunction {
//!     const NAME: &'static str = "my_function";
//!     const METADATA: FunctionMetadata = FunctionMetadata {
//!         name: "my_function",
//!         category: "example",
//!         description: "Does something useful",
//!         arguments: &[...],
//!         return_type: "string",
//!         examples: &["{{ my_function(arg=\"value\") }}"],
//!         syntax: SyntaxVariants::FUNCTION_ONLY,
//!     };
//!     // ...
//! }
//! ```

use serde::Serialize;

/// Metadata for a single function argument.
#[derive(Debug, Clone, Serialize)]
pub struct ArgumentMetadata {
    /// Argument name (e.g., "string", "path", "pattern")
    pub name: &'static str,
    /// Type description (e.g., "string", "integer", "boolean", "array", "object")
    pub arg_type: &'static str,
    /// Whether the argument is required
    pub required: bool,
    /// Default value if optional (as string representation, None if required)
    pub default: Option<&'static str>,
    /// Brief description of the argument
    pub description: &'static str,
}

/// Syntax variants supported by a function.
#[derive(Debug, Clone, Serialize)]
pub struct SyntaxVariants {
    /// Whether it can be called as a function: `{{ func(arg=value) }}`
    pub function: bool,
    /// Whether it can be used as a filter: `{{ value | filter }}`
    pub filter: bool,
    /// Whether it can be used as a test: `{% if value is test %}`
    pub is_test: bool,
}

impl SyntaxVariants {
    /// Function-only syntax (no filter or test).
    pub const FUNCTION_ONLY: Self = Self {
        function: true,
        filter: false,
        is_test: false,
    };

    /// Function and filter syntax.
    pub const FUNCTION_AND_FILTER: Self = Self {
        function: true,
        filter: true,
        is_test: false,
    };

    /// Function and is-test syntax.
    pub const FUNCTION_AND_TEST: Self = Self {
        function: true,
        filter: false,
        is_test: true,
    };
}

/// Complete metadata for a single function.
#[derive(Debug, Clone, Serialize)]
pub struct FunctionMetadata {
    /// Function name (e.g., "sha256", "get_env", "is_email")
    pub name: &'static str,
    /// Category for grouping (e.g., "hash", "environment", "validation")
    pub category: &'static str,
    /// Brief description of what the function does
    pub description: &'static str,
    /// Function arguments
    pub arguments: &'static [ArgumentMetadata],
    /// Return type description
    pub return_type: &'static str,
    /// Example usage snippets
    pub examples: &'static [&'static str],
    /// Supported syntax variants
    pub syntax: SyntaxVariants,
}
