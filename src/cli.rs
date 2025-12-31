use clap::Parser;

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
}
