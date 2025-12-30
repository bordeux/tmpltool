use std::collections::HashMap;
use std::env;
use tera::Value;

/// Custom Tera function to get environment variables with optional default values
///
/// # Usage in templates
///
/// Get environment variable with default:
/// ```tera
/// {{ env(name="VAR_NAME", default="fallback") }}
/// ```
///
/// Get environment variable (error if not found):
/// ```tera
/// {{ env(name="VAR_NAME") }}
/// ```
///
/// # Arguments
///
/// * `args` - HashMap containing function arguments
///   - `name` (required): The environment variable name to look up
///   - `default` (optional): Default value if the variable is not found
///
/// # Returns
///
/// Returns the environment variable value as a Tera Value::String, or the default if provided.
/// Returns an error if the variable is not found and no default is provided.
///
/// # Examples
///
/// ```tera
/// # Simple default value
/// port = {{ env(name="PORT", default="8080") }}
///
/// # Numeric default
/// max_connections = {{ env(name="MAX_CONN", default="100") }}
///
/// # Use in conditionals
/// {% if env(name="DEBUG", default="false") == "true" %}
/// debug_mode = enabled
/// {% endif %}
/// ```
pub fn env_function(args: &HashMap<String, Value>) -> tera::Result<Value> {
    let name = args
        .get("name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| tera::Error::msg("env() function requires a 'name' argument"))?;

    match env::var(name) {
        Ok(value) => Ok(Value::String(value)),
        Err(_) => {
            if let Some(default) = args.get("default") {
                Ok(default.clone())
            } else {
                Err(tera::Error::msg(format!(
                    "Environment variable '{}' not found and no default provided",
                    name
                )))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env_function_with_existing_var() {
        unsafe {
            env::set_var("TEST_ENV_FUNC", "test_value");
        }

        let mut args = HashMap::new();
        args.insert("name".to_string(), Value::String("TEST_ENV_FUNC".to_string()));

        let result = env_function(&args);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str().unwrap(), "test_value");

        unsafe {
            env::remove_var("TEST_ENV_FUNC");
        }
    }

    #[test]
    fn test_env_function_with_default() {
        let mut args = HashMap::new();
        args.insert(
            "name".to_string(),
            Value::String("TMPLTOOL_NONEXISTENT_VAR_12345".to_string()),
        );
        args.insert("default".to_string(), Value::String("default_value".to_string()));

        let result = env_function(&args);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str().unwrap(), "default_value");
    }

    #[test]
    fn test_env_function_missing_without_default() {
        let mut args = HashMap::new();
        args.insert(
            "name".to_string(),
            Value::String("TMPLTOOL_NONEXISTENT_VAR_12345".to_string()),
        );

        let result = env_function(&args);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Environment variable"));
    }

    #[test]
    fn test_env_function_missing_name_argument() {
        let args = HashMap::new();

        let result = env_function(&args);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("requires a 'name' argument"));
    }

    #[test]
    fn test_env_function_numeric_default() {
        let mut args = HashMap::new();
        args.insert(
            "name".to_string(),
            Value::String("TMPLTOOL_NONEXISTENT_VAR_NUM".to_string()),
        );
        args.insert("default".to_string(), Value::Number(42.into()));

        let result = env_function(&args);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_i64().unwrap(), 42);
    }
}
