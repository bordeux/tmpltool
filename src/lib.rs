//! # tmpltool
//!
//! A template rendering tool that uses Tera templates with environment variables.
//!
//! This library provides functionality to render Tera templates with all environment
//! variables available as context. It can output to either a file or stdout.
//!
//! # Custom Functions
//!
//! tmpltool provides custom Tera functions that can be used in templates:
//!
//! - `env()` - Get environment variables with optional default values
//!
//! See the [`functions`] module for more details on available functions.

pub mod cli;
pub mod context;
pub mod filter_functions;
pub mod functions;
pub mod is_functions;
pub mod renderer;
pub mod validator;

pub use cli::Cli;
pub use context::TemplateContext;
pub use functions::metadata::FunctionMetadata;
pub use renderer::render_template;

/// Get all function metadata for IDE integration
///
/// Returns a flat list of all available functions, filters, and is-tests
/// with their descriptions, arguments, return types, and examples.
pub fn get_all_metadata() -> Vec<&'static FunctionMetadata> {
    let mut all = Vec::new();

    // Collect from filter_functions (functions + filters)
    all.extend(filter_functions::get_all_metadata());

    // Collect from is_functions (functions + is-tests)
    all.extend(is_functions::get_all_metadata());

    // Collect from functions (function-only, no filter/is-test syntax)
    all.extend(functions::get_all_metadata());

    all
}
