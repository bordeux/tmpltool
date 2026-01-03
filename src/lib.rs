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
pub use renderer::render_template;
