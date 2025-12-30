use clap::Parser;

/// A template rendering tool that uses Tera templates with environment variables
#[derive(Parser, Debug)]
#[command(name = "tmpltool")]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Path to the template file
    pub template: String,

    /// Output file (if not specified, prints to stdout)
    #[arg(short, long)]
    pub output: Option<String>,
}
