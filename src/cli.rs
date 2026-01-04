use clap::{Parser, ValueEnum};

/// Output format for validation
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ValidateFormat {
    /// Validate as JSON
    Json,
    /// Validate as YAML
    Yaml,
    /// Validate as TOML
    Toml,
}

/// Output format for IDE metadata
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum IdeFormat {
    /// Output as JSON
    Json,
    /// Output as YAML
    Yaml,
    /// Output as TOML
    Toml,
}

/// A template rendering tool that uses Tera templates with environment variables
#[derive(Parser, Debug)]
#[command(name = "tmpltool")]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Path to the template file (omit to read from stdin)
    pub template: Option<String>,

    /// Output file (if not specified, prints to stdout)
    #[arg(short, long)]
    pub output: Option<String>,

    /// Trust mode: Allow filesystem functions to access absolute paths and parent directories
    /// WARNING: This disables security restrictions. Only use with trusted templates.
    #[arg(long)]
    pub trust: bool,

    /// Validate output format (json, yaml, or toml)
    /// If validation fails, the program exits with an error and shows the validation message
    #[arg(long, value_enum)]
    pub validate: Option<ValidateFormat>,

    /// Output function metadata for IDE integration
    /// Prints all available functions with their descriptions,
    /// arguments, return types, and examples, then exits
    #[arg(long, value_enum)]
    pub ide: Option<IdeFormat>,
}
