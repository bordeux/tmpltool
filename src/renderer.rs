use crate::functions;
use std::env;
use std::fs;
use std::io::{self, Write};
use tera::{Context, Tera};

/// Renders a template file with environment variables
///
/// # Arguments
///
/// * `template_file` - Path to the template file
/// * `output_file` - Optional path to output file. If None, prints to stdout
///
/// # Returns
///
/// Returns Ok(()) on success, or an error message on failure
pub fn render_template(
    template_file: &str,
    output_file: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Read template file
    let template_content = read_template(template_file)?;

    // Build context from environment variables
    let context = build_context();

    // Render the template
    let rendered = render(&template_content, &context)?;

    // Write output to file or stdout
    write_output(&rendered, output_file)?;

    Ok(())
}

/// Reads the template file content
fn read_template(template_file: &str) -> Result<String, Box<dyn std::error::Error>> {
    fs::read_to_string(template_file)
        .map_err(|e| format!("Failed to read template file '{}': {}", template_file, e).into())
}

/// Builds a Tera context from all environment variables
fn build_context() -> Context {
    let mut context = Context::new();
    for (key, value) in env::vars() {
        context.insert(&key, &value);
    }
    context
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
    fn test_build_context() {
        unsafe {
            env::set_var("TEST_BUILD_CONTEXT", "test_value");
        }

        let context = build_context();
        let value = context.get("TEST_BUILD_CONTEXT");
        assert!(value.is_some());

        unsafe {
            env::remove_var("TEST_BUILD_CONTEXT");
        }
    }

    #[test]
    fn test_render() {
        let mut context = Context::new();
        context.insert("TEST_VAR", "test_value");

        let template = "Value: {{ TEST_VAR }}";
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
    fn test_read_template() {
        let template_path = get_test_file_path("read_template.txt");
        fs::write(&template_path, "Test content").unwrap();

        let result = read_template(template_path.to_str().unwrap());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Test content");

        cleanup_test_file(&template_path);
    }

    #[test]
    fn test_read_missing_template() {
        let template_path = get_test_file_path("missing_template.txt");
        cleanup_test_file(&template_path);

        let result = read_template(template_path.to_str().unwrap());
        assert!(result.is_err());
    }

    #[test]
    fn test_env_function_in_template() {
        unsafe {
            env::set_var("TEST_TEMPLATE_VAR", "from_env");
        }

        let context = Context::new();
        let template = r#"Value: {{ env(name="TEST_TEMPLATE_VAR") }}"#;
        let result = render(template, &context);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Value: from_env");

        unsafe {
            env::remove_var("TEST_TEMPLATE_VAR");
        }
    }

    #[test]
    fn test_env_function_with_default_in_template() {
        let context = Context::new();
        let template = r#"Value: {{ env(name="TMPLTOOL_NONEXISTENT_12345", default="fallback") }}"#;
        let result = render(template, &context);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Value: fallback");
    }
}
