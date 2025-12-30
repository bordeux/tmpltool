use clap::Parser;
use std::process;
use tmpltool::{Cli, render_template};

fn main() {
    let cli = Cli::parse();

    if let Err(e) = render_template(cli.template.as_deref(), cli.output.as_deref()) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
