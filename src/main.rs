use clap::Parser;
use std::process;
use tmpltool::{render_template, Cli};

fn main() {
    let cli = Cli::parse();

    if let Err(e) = render_template(&cli.template, cli.output.as_deref()) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
