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
pub mod filters;
pub mod functions;
pub mod renderer;
pub mod validator;

pub use cli::Cli;
pub use context::TemplateContext;
pub use renderer::render_template;

use clap::Parser;
use std::ffi::OsString;

/// Run the template tool with the given command line arguments.
///
/// This function parses command line arguments and renders the template.
/// It's designed to be testable by accepting arguments programmatically.
///
/// # Arguments
///
/// * `args` - Iterator of command line arguments (including program name as first element)
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error describing what went wrong.
///
/// # Example
///
/// ```no_run
/// use tmpltool::run;
///
/// // Run with custom arguments
/// let result = run(["tmpltool", "template.tmpl", "-o", "output.txt"]);
/// ```
pub fn run<I, T>(args: I) -> Result<(), Box<dyn std::error::Error>>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let cli = Cli::try_parse_from(args)?;

    render_template(
        cli.template.as_deref(),
        cli.output.as_deref(),
        cli.trust,
        cli.validate,
    )
}
