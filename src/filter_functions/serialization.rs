//! Serialization functions that support both function and filter syntax.
//!
//! # Function Syntax
//! ```jinja
//! {{ to_json(object=config) }}
//! {{ to_yaml(object=config) }}
//! {{ parse_json(string='{"key": "value"}') }}
//! ```
//!
//! # Filter Syntax
//! ```jinja
//! {{ config | to_json }}
//! {{ config | to_yaml }}
//! {{ '{"key": "value"}' | parse_json }}
//! ```
//!
//! # Chaining
//! ```jinja
//! {{ config | to_json | base64_encode }}
//! ```

use super::FilterFunction;
use crate::functions::metadata::{ArgumentMetadata, FunctionMetadata, SyntaxVariants};
use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};

/// Common metadata for object argument (serialization)
const OBJECT_ARG: ArgumentMetadata = ArgumentMetadata {
    name: "object",
    arg_type: "any",
    required: true,
    default: None,
    description: "The value to serialize",
};

/// Common metadata for string argument (parsing)
const STRING_ARG: ArgumentMetadata = ArgumentMetadata {
    name: "string",
    arg_type: "string",
    required: true,
    default: None,
    description: "The string to parse",
};

// ============================================
// Serialization (Object -> String)
// ============================================

/// Convert object to JSON string.
///
/// # Function Syntax
/// ```jinja
/// {{ to_json(object=config) }}
/// {{ to_json(object=config, pretty=true) }}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ config | to_json }}
/// {{ config | to_json(pretty=true) }}
/// ```
pub struct ToJson;

impl ToJson {
    fn serialize(value: &Value, pretty: bool) -> Result<String, Error> {
        // Convert MiniJinja Value to serde_json::Value
        let json_value: serde_json::Value = serde_json::to_value(value).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Failed to convert to JSON: {}", e),
            )
        })?;

        // Serialize to JSON string
        if pretty {
            serde_json::to_string_pretty(&json_value).map_err(|e| {
                Error::new(
                    ErrorKind::InvalidOperation,
                    format!("Failed to serialize to JSON: {}", e),
                )
            })
        } else {
            serde_json::to_string(&json_value).map_err(|e| {
                Error::new(
                    ErrorKind::InvalidOperation,
                    format!("Failed to serialize to JSON: {}", e),
                )
            })
        }
    }
}

impl FilterFunction for ToJson {
    const NAME: &'static str = "to_json";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "to_json",
        category: "serialization",
        description: "Convert object to JSON string",
        arguments: &[
            OBJECT_ARG,
            ArgumentMetadata {
                name: "pretty",
                arg_type: "boolean",
                required: false,
                default: Some("false"),
                description: "Pretty-print the JSON output",
            },
        ],
        return_type: "string",
        examples: &[
            "{{ to_json(object=config) }}",
            "{{ config | to_json(pretty=true) }}",
        ],
        syntax: SyntaxVariants::FUNCTION_AND_FILTER,
    };

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let object: Value = kwargs.get("object")?;
        let pretty: bool = kwargs.get("pretty").unwrap_or(false);
        Ok(Value::from(Self::serialize(&object, pretty)?))
    }

    fn call_as_filter(value: &Value, kwargs: Kwargs) -> Result<Value, Error> {
        let pretty: bool = kwargs.get("pretty").unwrap_or(false);
        Ok(Value::from(Self::serialize(value, pretty)?))
    }
}

/// Convert object to YAML string.
///
/// # Function Syntax
/// ```jinja
/// {{ to_yaml(object=config) }}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ config | to_yaml }}
/// ```
pub struct ToYaml;

impl ToYaml {
    fn serialize(value: &Value) -> Result<String, Error> {
        // Convert MiniJinja Value to serde_yaml::Value
        let yaml_value: serde_yaml::Value = serde_yaml::to_value(value).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Failed to convert to YAML: {}", e),
            )
        })?;

        // Serialize to YAML string
        serde_yaml::to_string(&yaml_value).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Failed to serialize to YAML: {}", e),
            )
        })
    }
}

impl FilterFunction for ToYaml {
    const NAME: &'static str = "to_yaml";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "to_yaml",
        category: "serialization",
        description: "Convert object to YAML string",
        arguments: &[OBJECT_ARG],
        return_type: "string",
        examples: &["{{ to_yaml(object=config) }}", "{{ config | to_yaml }}"],
        syntax: SyntaxVariants::FUNCTION_AND_FILTER,
    };

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let object: Value = kwargs.get("object")?;
        Ok(Value::from(Self::serialize(&object)?))
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        Ok(Value::from(Self::serialize(value)?))
    }
}

/// Convert object to TOML string.
///
/// # Function Syntax
/// ```jinja
/// {{ to_toml(object=config) }}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ config | to_toml }}
/// ```
///
/// # Note
///
/// TOML has specific requirements:
/// - Root level must be a table (object/map)
/// - Arrays must contain elements of the same type
/// - Some nested structures may not be representable in TOML
pub struct ToToml;

impl ToToml {
    fn serialize(value: &Value) -> Result<String, Error> {
        // Convert MiniJinja Value to serde_json::Value first (as intermediate format)
        let json_value: serde_json::Value = serde_json::to_value(value).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Failed to convert to TOML (intermediate conversion): {}", e),
            )
        })?;

        // Serialize JSON value directly to TOML string
        toml::to_string(&json_value).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Failed to serialize to TOML: {}", e),
            )
        })
    }
}

impl FilterFunction for ToToml {
    const NAME: &'static str = "to_toml";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "to_toml",
        category: "serialization",
        description: "Convert object to TOML string",
        arguments: &[OBJECT_ARG],
        return_type: "string",
        examples: &["{{ to_toml(object=config) }}", "{{ config | to_toml }}"],
        syntax: SyntaxVariants::FUNCTION_AND_FILTER,
    };

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let object: Value = kwargs.get("object")?;
        Ok(Value::from(Self::serialize(&object)?))
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        Ok(Value::from(Self::serialize(value)?))
    }
}

// ============================================
// Parsing (String -> Object)
// ============================================

/// Parse JSON string into object.
///
/// # Function Syntax
/// ```jinja
/// {{ parse_json(string='{"key": "value"}') }}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ '{"key": "value"}' | parse_json }}
/// ```
pub struct ParseJson;

impl ParseJson {
    fn parse(input: &str) -> Result<Value, Error> {
        let json_value: serde_json::Value = serde_json::from_str(input).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Failed to parse JSON: {}", e),
            )
        })?;

        Ok(Value::from_serialize(&json_value))
    }
}

impl FilterFunction for ParseJson {
    const NAME: &'static str = "parse_json";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "parse_json",
        category: "serialization",
        description: "Parse JSON string into object",
        arguments: &[STRING_ARG],
        return_type: "any",
        examples: &[
            "{{ parse_json(string='{\"key\": \"value\"}') }}",
            "{{ json_string | parse_json }}",
        ],
        syntax: SyntaxVariants::FUNCTION_AND_FILTER,
    };

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let input: String = kwargs.get("string")?;
        Self::parse(&input)
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        let input = value.as_str().ok_or_else(|| {
            Error::new(ErrorKind::InvalidOperation, "parse_json requires a string")
        })?;
        Self::parse(input)
    }
}

/// Parse YAML string into object.
///
/// # Function Syntax
/// ```jinja
/// {{ parse_yaml(string='key: value') }}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ 'key: value' | parse_yaml }}
/// ```
pub struct ParseYaml;

impl ParseYaml {
    fn parse(input: &str) -> Result<Value, Error> {
        // Parse YAML string to serde_yaml::Value
        let yaml_value: serde_yaml::Value = serde_yaml::from_str(input).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Failed to parse YAML: {}", e),
            )
        })?;

        // Convert serde_yaml::Value to serde_json::Value
        let json_value = serde_yaml_to_json(yaml_value).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Failed to convert YAML to JSON: {}", e),
            )
        })?;

        Ok(Value::from_serialize(&json_value))
    }
}

impl FilterFunction for ParseYaml {
    const NAME: &'static str = "parse_yaml";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "parse_yaml",
        category: "serialization",
        description: "Parse YAML string into object",
        arguments: &[STRING_ARG],
        return_type: "any",
        examples: &[
            "{{ parse_yaml(string='key: value') }}",
            "{{ yaml_string | parse_yaml }}",
        ],
        syntax: SyntaxVariants::FUNCTION_AND_FILTER,
    };

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let input: String = kwargs.get("string")?;
        Self::parse(&input)
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        let input = value.as_str().ok_or_else(|| {
            Error::new(ErrorKind::InvalidOperation, "parse_yaml requires a string")
        })?;
        Self::parse(input)
    }
}

/// Parse TOML string into object.
///
/// # Function Syntax
/// ```jinja
/// {{ parse_toml(string='key = "value"') }}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ 'key = "value"' | parse_toml }}
/// ```
pub struct ParseToml;

impl ParseToml {
    fn parse(input: &str) -> Result<Value, Error> {
        // Parse TOML string to toml::Value
        let toml_value: toml::Value = toml::from_str(input).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Failed to parse TOML: {}", e),
            )
        })?;

        // Convert toml::Value to serde_json::Value
        let json_value = toml_to_json(toml_value).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Failed to convert TOML to JSON: {}", e),
            )
        })?;

        Ok(Value::from_serialize(&json_value))
    }
}

impl FilterFunction for ParseToml {
    const NAME: &'static str = "parse_toml";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "parse_toml",
        category: "serialization",
        description: "Parse TOML string into object",
        arguments: &[STRING_ARG],
        return_type: "any",
        examples: &[
            "{{ parse_toml(string='key = \"value\"') }}",
            "{{ toml_string | parse_toml }}",
        ],
        syntax: SyntaxVariants::FUNCTION_AND_FILTER,
    };

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let input: String = kwargs.get("string")?;
        Self::parse(&input)
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        let input = value.as_str().ok_or_else(|| {
            Error::new(ErrorKind::InvalidOperation, "parse_toml requires a string")
        })?;
        Self::parse(input)
    }
}

// ============================================
// Helper Functions
// ============================================

/// Helper function to convert serde_yaml::Value to serde_json::Value
fn serde_yaml_to_json(yaml: serde_yaml::Value) -> std::result::Result<serde_json::Value, String> {
    match yaml {
        serde_yaml::Value::Null => Ok(serde_json::Value::Null),
        serde_yaml::Value::Bool(b) => Ok(serde_json::Value::Bool(b)),
        serde_yaml::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(serde_json::Value::Number(i.into()))
            } else if let Some(u) = n.as_u64() {
                Ok(serde_json::Value::Number(u.into()))
            } else if let Some(f) = n.as_f64() {
                serde_json::Number::from_f64(f)
                    .map(serde_json::Value::Number)
                    .ok_or_else(|| format!("Invalid float value: {}", f))
            } else {
                Err("Unsupported number type".to_string())
            }
        }
        serde_yaml::Value::String(s) => Ok(serde_json::Value::String(s)),
        serde_yaml::Value::Sequence(seq) => {
            let json_array: std::result::Result<Vec<serde_json::Value>, String> =
                seq.into_iter().map(serde_yaml_to_json).collect();
            json_array.map(serde_json::Value::Array)
        }
        serde_yaml::Value::Mapping(map) => {
            let mut json_map = serde_json::Map::new();
            for (k, v) in map {
                let key = match k {
                    serde_yaml::Value::String(s) => s,
                    serde_yaml::Value::Number(n) => n.to_string(),
                    serde_yaml::Value::Bool(b) => b.to_string(),
                    _ => {
                        return Err(
                            "YAML map keys must be strings, numbers, or booleans".to_string()
                        );
                    }
                };
                json_map.insert(key, serde_yaml_to_json(v)?);
            }
            Ok(serde_json::Value::Object(json_map))
        }
        serde_yaml::Value::Tagged(tagged) => {
            // For tagged values, just convert the inner value
            serde_yaml_to_json(tagged.value)
        }
    }
}

/// Helper function to convert toml::Value to serde_json::Value
fn toml_to_json(toml: toml::Value) -> std::result::Result<serde_json::Value, String> {
    match toml {
        toml::Value::String(s) => Ok(serde_json::Value::String(s)),
        toml::Value::Integer(i) => Ok(serde_json::Value::Number(i.into())),
        toml::Value::Float(f) => serde_json::Number::from_f64(f)
            .map(serde_json::Value::Number)
            .ok_or_else(|| format!("Invalid float value: {}", f)),
        toml::Value::Boolean(b) => Ok(serde_json::Value::Bool(b)),
        toml::Value::Array(arr) => {
            let json_array: std::result::Result<Vec<serde_json::Value>, String> =
                arr.into_iter().map(toml_to_json).collect();
            json_array.map(serde_json::Value::Array)
        }
        toml::Value::Table(table) => {
            let mut json_map = serde_json::Map::new();
            for (k, v) in table {
                json_map.insert(k, toml_to_json(v)?);
            }
            Ok(serde_json::Value::Object(json_map))
        }
        toml::Value::Datetime(dt) => Ok(serde_json::Value::String(dt.to_string())),
    }
}
