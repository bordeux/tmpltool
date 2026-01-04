//! Object functions that support both function and filter syntax.
//!
//! # Function Syntax
//! ```jinja
//! {{ object_keys(object=config) }}
//! {{ object_values(object=config) }}
//! {{ object_flatten(object=nested) }}
//! ```
//!
//! # Filter Syntax
//! ```jinja
//! {{ config | object_keys }}
//! {{ config | object_values }}
//! {{ nested | object_flatten }}
//! ```
//!
//! # Chaining
//! ```jinja
//! {{ config | object_keys | first }}
//! {{ nested | object_flatten | to_json }}
//! ```

use super::FilterFunction;
use crate::functions::metadata::{ArgumentMetadata, FunctionMetadata, SyntaxVariants};
use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};
use serde_json::Map;

/// Common metadata for object argument
const OBJECT_ARG: ArgumentMetadata = ArgumentMetadata {
    name: "object",
    arg_type: "object",
    required: true,
    default: None,
    description: "The object to process",
};

/// Helper to convert Value to serde_json::Value
fn value_to_json(value: &Value, fn_name: &str) -> Result<serde_json::Value, Error> {
    serde_json::to_value(value).map_err(|e| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("{}: failed to convert value: {}", fn_name, e),
        )
    })
}

/// Helper to ensure value is an object
fn ensure_object(
    json_value: serde_json::Value,
    fn_name: &str,
) -> Result<Map<String, serde_json::Value>, Error> {
    match json_value {
        serde_json::Value::Object(map) => Ok(map),
        _ => Err(Error::new(
            ErrorKind::InvalidOperation,
            format!("{} requires an object, not an array or primitive", fn_name),
        )),
    }
}

// ============================================
// ObjectKeys
// ============================================

/// Get object keys as an array.
///
/// # Function Syntax
/// ```jinja
/// {% set config = {"host": "localhost", "port": 8080, "debug": true} %}
/// {% set keys = object_keys(object=config) %}
/// {# Result: ["host", "port", "debug"] #}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {% set keys = config | object_keys %}
/// {% for key in config | object_keys %}
///   {{ key }}: {{ config[key] }}
/// {% endfor %}
/// ```
pub struct ObjectKeys;

impl ObjectKeys {
    fn compute(json_value: serde_json::Value) -> Result<Value, Error> {
        let map = ensure_object(json_value, "object_keys")?;
        let keys: Vec<String> = map.keys().cloned().collect();
        Ok(Value::from_serialize(&keys))
    }
}

impl FilterFunction for ObjectKeys {
    const NAME: &'static str = "object_keys";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "object_keys",
        category: "object",
        description: "Get object keys as an array",
        arguments: &[OBJECT_ARG],
        return_type: "array",
        examples: &[
            "{{ object_keys(object=config) }}",
            "{{ config | object_keys }}",
        ],
        syntax: SyntaxVariants::FUNCTION_AND_FILTER,
    };

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let object: Value = kwargs.get("object")?;
        let json_value = value_to_json(&object, "object_keys")?;
        Self::compute(json_value)
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        let json_value = value_to_json(value, "object_keys")?;
        Self::compute(json_value)
    }
}

// ============================================
// ObjectValues
// ============================================

/// Get object values as an array.
///
/// # Function Syntax
/// ```jinja
/// {% set config = {"host": "localhost", "port": 8080, "debug": true} %}
/// {% set values = object_values(object=config) %}
/// {# Result: ["localhost", 8080, true] #}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {% set values = config | object_values %}
/// {% for value in config | object_values %}
///   - {{ value }}
/// {% endfor %}
/// ```
pub struct ObjectValues;

impl ObjectValues {
    fn compute(json_value: serde_json::Value) -> Result<Value, Error> {
        let map = ensure_object(json_value, "object_values")?;
        let values: Vec<&serde_json::Value> = map.values().collect();
        Ok(Value::from_serialize(&values))
    }
}

impl FilterFunction for ObjectValues {
    const NAME: &'static str = "object_values";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "object_values",
        category: "object",
        description: "Get object values as an array",
        arguments: &[OBJECT_ARG],
        return_type: "array",
        examples: &[
            "{{ object_values(object=config) }}",
            "{{ config | object_values }}",
        ],
        syntax: SyntaxVariants::FUNCTION_AND_FILTER,
    };

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let object: Value = kwargs.get("object")?;
        let json_value = value_to_json(&object, "object_values")?;
        Self::compute(json_value)
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        let json_value = value_to_json(value, "object_values")?;
        Self::compute(json_value)
    }
}

// ============================================
// ObjectFlatten
// ============================================

/// Flatten nested object to dot notation.
///
/// # Function Syntax
/// ```jinja
/// {% set nested = {"server": {"host": "localhost", "port": 8080}, "database": {"name": "mydb"}} %}
/// {% set flat = object_flatten(object=nested) %}
/// {# Result: {"server.host": "localhost", "server.port": 8080, "database.name": "mydb"} #}
///
/// {% set flat_underscore = object_flatten(object=nested, delimiter="_") %}
/// {# Result: {"server_host": "localhost", "server_port": 8080, "database_name": "mydb"} #}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {% set flat = nested | object_flatten %}
/// {% set flat_underscore = nested | object_flatten(delimiter="_") %}
/// ```
pub struct ObjectFlatten;

impl ObjectFlatten {
    fn compute(json_value: serde_json::Value, delimiter: &str) -> Result<Value, Error> {
        if !json_value.is_object() {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                "object_flatten requires an object",
            ));
        }

        let mut result = Map::new();
        Self::flatten_recursive(&json_value, "", delimiter, &mut result);

        Ok(Value::from_serialize(serde_json::Value::Object(result)))
    }

    fn flatten_recursive(
        value: &serde_json::Value,
        prefix: &str,
        delimiter: &str,
        result: &mut Map<String, serde_json::Value>,
    ) {
        match value {
            serde_json::Value::Object(map) => {
                for (key, val) in map {
                    let new_key = if prefix.is_empty() {
                        key.clone()
                    } else {
                        format!("{}{}{}", prefix, delimiter, key)
                    };
                    Self::flatten_recursive(val, &new_key, delimiter, result);
                }
            }
            serde_json::Value::Array(arr) => {
                for (index, val) in arr.iter().enumerate() {
                    let new_key = if prefix.is_empty() {
                        index.to_string()
                    } else {
                        format!("{}{}{}", prefix, delimiter, index)
                    };
                    Self::flatten_recursive(val, &new_key, delimiter, result);
                }
            }
            _ => {
                result.insert(prefix.to_string(), value.clone());
            }
        }
    }
}

impl FilterFunction for ObjectFlatten {
    const NAME: &'static str = "object_flatten";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "object_flatten",
        category: "object",
        description: "Flatten nested object to dot notation",
        arguments: &[
            OBJECT_ARG,
            ArgumentMetadata {
                name: "delimiter",
                arg_type: "string",
                required: false,
                default: Some("."),
                description: "Delimiter to use between keys",
            },
        ],
        return_type: "object",
        examples: &[
            "{{ object_flatten(object=nested) }}",
            "{{ nested | object_flatten(delimiter=\"_\") }}",
        ],
        syntax: SyntaxVariants::FUNCTION_AND_FILTER,
    };

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let object: Value = kwargs.get("object")?;
        let delimiter: Option<String> = kwargs.get("delimiter")?;
        let delimiter = delimiter.unwrap_or_else(|| ".".to_string());

        let json_value = value_to_json(&object, "object_flatten")?;
        Self::compute(json_value, &delimiter)
    }

    fn call_as_filter(value: &Value, kwargs: Kwargs) -> Result<Value, Error> {
        let delimiter: Option<String> = kwargs.get("delimiter")?;
        let delimiter = delimiter.unwrap_or_else(|| ".".to_string());

        let json_value = value_to_json(value, "object_flatten")?;
        Self::compute(json_value, &delimiter)
    }
}
