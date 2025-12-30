use crate::functions;
use std::fs;
use std::io::{self, Read, Write};
use tera::{Context, Tera};

/// Renders a template with environment variables
///
/// # Arguments
///
/// * `template_source` - Optional path to template file. If None, reads from stdin
/// * `output_file` - Optional path to output file. If None, prints to stdout
///
/// # Returns
///
/// Returns Ok(()) on success, or an error message on failure
pub fn render_template(
    template_source: Option<&str>,
    output_file: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Read template from file or stdin
    let template_content = read_template(template_source)?;

    // Create empty context - env vars only accessible via env() function
    let context = Context::new();

    // Render the template
    let rendered = render(&template_content, &context)?;

    // Write output to file or stdout
    write_output(&rendered, output_file)?;

    Ok(())
}

/// Reads the template content from file or stdin
fn read_template(template_source: Option<&str>) -> Result<String, Box<dyn std::error::Error>> {
    match template_source {
        Some(file_path) => {
            // Read from file
            fs::read_to_string(file_path)
                .map_err(|e| format!("Failed to read template file '{}': {}", file_path, e).into())
        }
        None => {
            // Read from stdin
            let mut buffer = String::new();
            io::stdin()
                .read_to_string(&mut buffer)
                .map_err(|e| format!("Failed to read from stdin: {}", e))?;

            if buffer.is_empty() {
                return Err(
                    "No input provided. Either specify a template file or pipe content to stdin."
                        .into(),
                );
            }

            Ok(buffer)
        }
    }
}

/// Renders the template with the given context
fn render(template_content: &str, context: &Context) -> Result<String, Box<dyn std::error::Error>> {
    let mut tera = Tera::default();

    // Register all custom functions
    functions::register_all(&mut tera);

    tera.add_raw_template("template", template_content)
        .map_err(|e| format!("Failed to parse template: {}", e))?;

    tera.render("template", context)
        .map_err(|e| format!("Failed to render template: {}", e).into())
}

/// Writes the rendered content to file or stdout
fn write_output(
    rendered: &str,
    output_file: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    match output_file {
        Some(path) => {
            fs::write(path, rendered)
                .map_err(|e| format!("Failed to write output file '{}': {}", path, e))?;
            eprintln!("Successfully rendered template to '{}'", path);
        }
        None => {
            print!("{}", rendered);
            io::stdout().flush()?;
        }
    }
    Ok(())
}
