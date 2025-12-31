//! Serialization functions for MiniJinja templates
//!
//! This module provides functions for:
//! - Converting objects to JSON strings
//! - Converting objects to YAML strings
//! - Converting objects to TOML strings

use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};

/// Convert object to JSON string
///
/// # Arguments
///
/// * `object` (required) - Object/value to convert to JSON
/// * `pretty` (optional) - Enable pretty-printing with indentation (default: false)
///
/// # Returns
///
/// Returns a JSON string representation of the object
///
/// # Example
///
/// ```jinja
/// {# Simple JSON serialization #}
/// {% set config = {"host": "localhost", "port": 8080, "debug": true} %}
/// {{ to_json(object=config) }}
/// {# Output: {"host":"localhost","port":8080,"debug":true} #}
///
/// {# Pretty-printed JSON #}
/// {{ to_json(object=config, pretty=true) }}
/// {# Output:
/// {
///   "host": "localhost",
///   "port": 8080,
///   "debug": true
/// }
/// #}
///
/// {# Convert array to JSON #}
/// {% set items = [1, 2, 3, 4, 5] %}
/// {{ to_json(object=items) }}
/// {# Output: [1,2,3,4,5] #}
///
/// {# Nested objects #}
/// {% set app_config = {
///   "database": {"host": "db.example.com", "port": 5432},
///   "cache": {"enabled": true, "ttl": 3600}
/// } %}
/// {{ to_json(object=app_config, pretty=true) }}
/// ```
pub fn to_json_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let object: Value = kwargs.get("object")?;
    let pretty: bool = kwargs.get("pretty").unwrap_or(false);

    // Convert MiniJinja Value to serde_json::Value
    let json_value: serde_json::Value = serde_json::to_value(&object).map_err(|e| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("Failed to convert to JSON: {}", e),
        )
    })?;

    // Serialize to JSON string
    let json_string = if pretty {
        serde_json::to_string_pretty(&json_value).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Failed to serialize to JSON: {}", e),
            )
        })?
    } else {
        serde_json::to_string(&json_value).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Failed to serialize to JSON: {}", e),
            )
        })?
    };

    Ok(Value::from(json_string))
}

/// Convert object to YAML string
///
/// # Arguments
///
/// * `object` (required) - Object/value to convert to YAML
///
/// # Returns
///
/// Returns a YAML string representation of the object
///
/// # Example
///
/// ```jinja
/// {# Simple YAML serialization #}
/// {% set config = {"host": "localhost", "port": 8080, "debug": true} %}
/// {{ to_yaml(object=config) }}
/// {# Output:
/// host: localhost
/// port: 8080
/// debug: true
/// #}
///
/// {# Convert array to YAML #}
/// {% set items = ["apple", "banana", "cherry"] %}
/// {{ to_yaml(object=items) }}
/// {# Output:
/// - apple
/// - banana
/// - cherry
/// #}
///
/// {# Nested configuration #}
/// {% set app_config = {
///   "server": {
///     "host": "0.0.0.0",
///     "port": 8080,
///     "workers": 4
///   },
///   "database": {
///     "url": "postgres://localhost/mydb",
///     "pool_size": 10
///   }
/// } %}
/// {{ to_yaml(object=app_config) }}
/// {# Output:
/// server:
///   host: 0.0.0.0
///   port: 8080
///   workers: 4
/// database:
///   url: postgres://localhost/mydb
///   pool_size: 10
/// #}
/// ```
pub fn to_yaml_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let object: Value = kwargs.get("object")?;

    // Convert MiniJinja Value to serde_yaml::Value
    let yaml_value: serde_yaml::Value = serde_yaml::to_value(&object).map_err(|e| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("Failed to convert to YAML: {}", e),
        )
    })?;

    // Serialize to YAML string
    let yaml_string = serde_yaml::to_string(&yaml_value).map_err(|e| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("Failed to serialize to YAML: {}", e),
        )
    })?;

    Ok(Value::from(yaml_string))
}

/// Convert object to TOML string
///
/// # Arguments
///
/// * `object` (required) - Object/value to convert to TOML
///
/// # Returns
///
/// Returns a TOML string representation of the object
///
/// # Note
///
/// TOML has specific requirements:
/// - Root level must be a table (object/map)
/// - Arrays must contain elements of the same type
/// - Some nested structures may not be representable in TOML
///
/// # Example
///
/// ```jinja
/// {# Simple TOML serialization #}
/// {% set config = {"title": "My App", "version": "1.0.0"} %}
/// {{ to_toml(object=config) }}
/// {# Output:
/// title = "My App"
/// version = "1.0.0"
/// #}
///
/// {# Nested configuration #}
/// {% set app_config = {
///   "package": {
///     "name": "myapp",
///     "version": "1.0.0"
///   },
///   "dependencies": {
///     "serde": "1.0",
///     "tokio": "1.0"
///   }
/// } %}
/// {{ to_toml(object=app_config) }}
/// {# Output:
/// [package]
/// name = "myapp"
/// version = "1.0.0"
///
/// [dependencies]
/// serde = "1.0"
/// tokio = "1.0"
/// #}
///
/// {# Array of tables #}
/// {% set config = {
///   "database": [
///     {"name": "primary", "host": "db1.example.com"},
///     {"name": "replica", "host": "db2.example.com"}
///   ]
/// } %}
/// {{ to_toml(object=config) }}
/// {# Output:
/// [[database]]
/// name = "primary"
/// host = "db1.example.com"
///
/// [[database]]
/// name = "replica"
/// host = "db2.example.com"
/// #}
/// ```
pub fn to_toml_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let object: Value = kwargs.get("object")?;

    // Convert MiniJinja Value to serde_json::Value first (as intermediate format)
    let json_value: serde_json::Value = serde_json::to_value(&object).map_err(|e| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("Failed to convert to TOML (intermediate conversion): {}", e),
        )
    })?;

    // Serialize JSON value directly to TOML string
    let toml_string = toml::to_string(&json_value).map_err(|e| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("Failed to serialize to TOML: {}", e),
        )
    })?;

    Ok(Value::from(toml_string))
}
