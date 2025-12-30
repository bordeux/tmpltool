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
}
