use clap::Parser;
use serde::Serialize;
use std::process;
use tmpltool::cli::IdeFormat;
use tmpltool::{Cli, FunctionMetadata, get_all_metadata, render_template};

/// Wrapper for TOML output (TOML requires a table at the root)
#[derive(Serialize)]
struct IdeMetadata<'a> {
    functions: Vec<&'a FunctionMetadata>,
}

fn main() {
    let cli = Cli::parse();

    // Handle --ide <format> early exit
    if let Some(format) = cli.ide {
        let metadata = get_all_metadata();
        let result = match format {
            IdeFormat::Json => serde_json::to_string_pretty(&metadata).map_err(|e| e.to_string()),
            IdeFormat::Yaml => serde_yaml::to_string(&metadata).map_err(|e| e.to_string()),
            IdeFormat::Toml => {
                let wrapped = IdeMetadata {
                    functions: metadata,
                };
                toml::to_string_pretty(&wrapped).map_err(|e| e.to_string())
            }
        };
        match result {
            Ok(data) => {
                println!("{}", data);
                process::exit(0);
            }
            Err(e) => {
                eprintln!("Error serializing metadata: {}", e);
                process::exit(1);
            }
        }
    }

    if let Err(e) = render_template(
        cli.template.as_deref(),
        cli.output.as_deref(),
        cli.trust,
        cli.validate,
    ) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
