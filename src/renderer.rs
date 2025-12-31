use crate::{TemplateContext, functions};
use minijinja::Environment;
use serde::Serialize;
use std::fs;
use std::io::{self, Read, Write};

/// Renders a template with environment variables
///
/// # Arguments
///
/// * `template_source` - Optional path to template file. If None, reads from stdin
/// * `output_file` - Optional path to output file. If None, prints to stdout
/// * `trust_mode` - If true, disables filesystem security restrictions
///
/// # Returns
///
/// Returns Ok(()) on success, or an error message on failure
pub fn render_template(
    template_source: Option<&str>,
    output_file: Option<&str>,
    trust_mode: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // Read template from file or stdin
    let template_content = read_template(template_source)?;

    // Create template context for resolving file paths
    let template_context = match template_source {
        Some(file_path) => TemplateContext::from_template_file(file_path, trust_mode)?,
        None => TemplateContext::from_stdin(trust_mode)?,
    };

    // Create empty context - env vars only accessible via env() function
    let context = serde_json::json!({});

    // Render the template
    let rendered = render(
        template_source,
        &template_content,
        &context,
        template_context,
    )?;

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
fn render(
    template_source: Option<&str>,
    template_content: &str,
    context: &impl Serialize,
    template_context: TemplateContext,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut env = Environment::new();

    // Set strict undefined behavior - fail on undefined variables (like Tera)
    env.set_undefined_behavior(minijinja::UndefinedBehavior::Strict);

    // Register all custom functions
    functions::register_all(&mut env, template_context);

    // Use full file path as template name if it's a file, otherwise use "template"
    let template_name = template_source.unwrap_or("template");

    env.add_template(template_name, template_content)
        .map_err(|e| format_minijinja_error("Failed to parse template", &e))?;

    let tmpl = env.get_template(template_name)?;
    tmpl.render(context)
        .map_err(|e| format_minijinja_error("Failed to render template", &e).into())
}

/// Formats MiniJinja errors with detailed information
fn format_minijinja_error(prefix: &str, error: &minijinja::Error) -> String {
    use std::fmt::Write;

    let mut msg = String::new();
    writeln!(&mut msg, "{}", prefix).ok();
    writeln!(&mut msg).ok();
    writeln!(&mut msg, "Error: {}", error).ok();

    // MiniJinja has excellent error messages built-in
    if let Some(detail) = error.detail() {
        writeln!(&mut msg).ok();
        writeln!(&mut msg, "{}", detail).ok();
    }

    msg
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
