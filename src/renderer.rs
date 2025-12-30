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
            fs::read_to_string(file_path).map_err(|e| {
                format!("Failed to read template file '{}': {}", file_path, e).into()
            })
        }
        None => {
            // Read from stdin
            let mut buffer = String::new();
            io::stdin()
                .read_to_string(&mut buffer)
                .map_err(|e| format!("Failed to read from stdin: {}", e))?;

            if buffer.is_empty() {
                return Err("No input provided. Either specify a template file or pipe content to stdin.".into());
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
fn write_output(rendered: &str, output_file: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::path::PathBuf;

    fn get_test_file_path(name: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        path.push(format!("tmpltool_test_{}", name));
        path
    }

    fn cleanup_test_file(path: &PathBuf) {
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_render() {
        let context = Context::new();
        let template = "Value: test_value";
        let result = render(template, &context);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Value: test_value");
    }

    #[test]
    fn test_render_invalid_template() {
        let context = Context::new();
        let template = "Invalid {{ TEST_VAR";
        let result = render(template, &context);

        assert!(result.is_err());
    }

    #[test]
    fn test_read_template_from_file() {
        let template_path = get_test_file_path("read_template.txt");
        fs::write(&template_path, "Test content").unwrap();

        let result = read_template(Some(template_path.to_str().unwrap()));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Test content");

        cleanup_test_file(&template_path);
    }

    #[test]
    fn test_read_missing_template() {
        let template_path = get_test_file_path("missing_template.txt");
        cleanup_test_file(&template_path);

        let result = read_template(Some(template_path.to_str().unwrap()));
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Failed to read template file"));
    }

    #[test]
    fn test_get_env_function_in_template() {
        unsafe {
            env::set_var("TEST_TEMPLATE_VAR", "from_env");
        }

        let context = Context::new();
        let template = r#"Value: {{ get_env(name="TEST_TEMPLATE_VAR") }}"#;
        let result = render(template, &context);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Value: from_env");

        unsafe {
            env::remove_var("TEST_TEMPLATE_VAR");
        }
    }

    #[test]
    fn test_get_env_function_with_default_in_template() {
        let context = Context::new();
        let template = r#"Value: {{ get_env(name="TMPLTOOL_NONEXISTENT_12345", default="fallback") }}"#;
        let result = render(template, &context);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Value: fallback");
    }

    #[test]
    fn test_no_auto_env_vars_in_context() {
        unsafe {
            env::set_var("TEST_NO_AUTO_ENV", "should_not_be_accessible");
        }

        let context = Context::new();
        // Try to use env var directly without get_env() function - should fail
        let template = "{{ TEST_NO_AUTO_ENV }}";
        let result = render(template, &context);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Failed to render template"));

        unsafe {
            env::remove_var("TEST_NO_AUTO_ENV");
        }
    }

    #[test]
    fn test_render_template_from_file() {
        let template_path = get_test_file_path("template_file.txt");
        let output_path = get_test_file_path("output_file.txt");

        fs::write(&template_path, "Static content").unwrap();

        let result = render_template(
            Some(template_path.to_str().unwrap()),
            Some(output_path.to_str().unwrap()),
        );

        assert!(result.is_ok());
        let output = fs::read_to_string(&output_path).unwrap();
        assert_eq!(output, "Static content");

        cleanup_test_file(&template_path);
        cleanup_test_file(&output_path);
    }
}
