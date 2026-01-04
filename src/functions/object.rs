//! Object manipulation functions for MiniJinja templates
//!
//! This module provides functions for:
//! - Merging objects
//! - Getting/setting nested values by path
//! - Checking key existence
//! - JSONPath queries
//! - Object picking/omitting keys
//! - Key renaming
//! - Unflattening nested objects
//!
//! Note: object_keys, object_values, object_flatten are now in
//! filter_functions/object.rs with dual function+filter syntax support.

use super::metadata::{ArgumentMetadata, FunctionMetadata, SyntaxVariants};
use super::traits::Function;
use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};
use serde_json::Map;

/// Deep merge two objects
pub struct ObjectMerge;

impl Function for ObjectMerge {
    const NAME: &'static str = "object_merge";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "object_merge",
        category: "object",
        description: "Deep merge two objects (obj2 values override obj1 values)",
        arguments: &[
            ArgumentMetadata {
                name: "obj1",
                arg_type: "object",
                required: true,
                default: None,
                description: "First object (base)",
            },
            ArgumentMetadata {
                name: "obj2",
                arg_type: "object",
                required: true,
                default: None,
                description: "Second object (overlay, takes precedence)",
            },
        ],
        return_type: "object",
        examples: &["{{ object_merge(obj1={\"a\": 1}, obj2={\"b\": 2}) }}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let obj1: Value = kwargs.get("obj1")?;
        let obj2: Value = kwargs.get("obj2")?;

        // Convert to serde_json::Value for easier manipulation
        let json1: serde_json::Value = serde_json::to_value(&obj1).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Failed to convert obj1: {}", e),
            )
        })?;

        let json2: serde_json::Value = serde_json::to_value(&obj2).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Failed to convert obj2: {}", e),
            )
        })?;

        let merged = merge_json_values(json1, json2);

        Ok(Value::from_serialize(&merged))
    }
}

/// Recursively merge two JSON values
fn merge_json_values(mut base: serde_json::Value, overlay: serde_json::Value) -> serde_json::Value {
    if let (serde_json::Value::Object(base_map), serde_json::Value::Object(overlay_map)) =
        (&mut base, &overlay)
    {
        for (key, value) in overlay_map {
            if let Some(base_value) = base_map.get_mut(key) {
                *base_value = merge_json_values(base_value.clone(), value.clone());
            } else {
                base_map.insert(key.clone(), value.clone());
            }
        }
        base
    } else {
        overlay
    }
}

/// Get nested value by path
pub struct ObjectGet;

impl Function for ObjectGet {
    const NAME: &'static str = "object_get";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "object_get",
        category: "object",
        description: "Get nested value by dot-separated path",
        arguments: &[
            ArgumentMetadata {
                name: "object",
                arg_type: "object",
                required: true,
                default: None,
                description: "Object to query",
            },
            ArgumentMetadata {
                name: "path",
                arg_type: "string",
                required: true,
                default: None,
                description: "Dot-separated path (e.g., \"a.b.c\")",
            },
        ],
        return_type: "any",
        examples: &["{{ object_get(object=config, path=\"server.host\") }}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let object: Value = kwargs.get("object")?;
        let path: String = kwargs.get("path")?;

        let json_value: serde_json::Value = serde_json::to_value(&object).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Failed to convert object: {}", e),
            )
        })?;

        let parts: Vec<&str> = path.split('.').collect();
        let mut current = &json_value;

        for part in parts {
            match current {
                serde_json::Value::Object(map) => {
                    if let Some(value) = map.get(part) {
                        current = value;
                    } else {
                        return Ok(Value::UNDEFINED);
                    }
                }
                serde_json::Value::Array(arr) => {
                    if let Ok(index) = part.parse::<usize>() {
                        if let Some(value) = arr.get(index) {
                            current = value;
                        } else {
                            return Ok(Value::UNDEFINED);
                        }
                    } else {
                        return Ok(Value::UNDEFINED);
                    }
                }
                _ => return Ok(Value::UNDEFINED),
            }
        }

        Ok(Value::from_serialize(current))
    }
}

/// Set nested value by path
pub struct ObjectSet;

impl Function for ObjectSet {
    const NAME: &'static str = "object_set";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "object_set",
        category: "object",
        description: "Set nested value by dot-separated path",
        arguments: &[
            ArgumentMetadata {
                name: "object",
                arg_type: "object",
                required: true,
                default: None,
                description: "Object to modify",
            },
            ArgumentMetadata {
                name: "path",
                arg_type: "string",
                required: true,
                default: None,
                description: "Dot-separated path (e.g., \"a.b.c\")",
            },
            ArgumentMetadata {
                name: "value",
                arg_type: "any",
                required: true,
                default: None,
                description: "Value to set",
            },
        ],
        return_type: "object",
        examples: &["{{ object_set(object=config, path=\"server.port\", value=8080) }}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let object: Value = kwargs.get("object")?;
        let path: String = kwargs.get("path")?;
        let value: Value = kwargs.get("value")?;

        let mut json_value: serde_json::Value = serde_json::to_value(&object).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Failed to convert object: {}", e),
            )
        })?;

        let new_value: serde_json::Value = serde_json::to_value(&value).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Failed to convert value: {}", e),
            )
        })?;

        let parts: Vec<&str> = path.split('.').collect();
        set_nested_value(&mut json_value, &parts, new_value)?;

        Ok(Value::from_serialize(&json_value))
    }
}

/// Recursively set nested value
fn set_nested_value(
    current: &mut serde_json::Value,
    parts: &[&str],
    value: serde_json::Value,
) -> Result<(), Error> {
    if parts.is_empty() {
        return Ok(());
    }

    if parts.len() == 1 {
        if let serde_json::Value::Object(map) = current {
            map.insert(parts[0].to_string(), value);
            return Ok(());
        } else {
            // Convert to object if not already
            let mut map = Map::new();
            map.insert(parts[0].to_string(), value);
            *current = serde_json::Value::Object(map);
            return Ok(());
        }
    }

    // More than one part remaining
    if !current.is_object() {
        *current = serde_json::Value::Object(Map::new());
    }

    if let serde_json::Value::Object(map) = current {
        let next_part = parts[0];
        let entry = map
            .entry(next_part.to_string())
            .or_insert_with(|| serde_json::Value::Object(Map::new()));
        set_nested_value(entry, &parts[1..], value)?;
    }

    Ok(())
}

/// Check if object has key
pub struct ObjectHasKey;

impl Function for ObjectHasKey {
    const NAME: &'static str = "object_has_key";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "object_has_key",
        category: "object",
        description: "Check if object has a key",
        arguments: &[
            ArgumentMetadata {
                name: "object",
                arg_type: "object",
                required: true,
                default: None,
                description: "Object to check",
            },
            ArgumentMetadata {
                name: "key",
                arg_type: "string",
                required: true,
                default: None,
                description: "Key to check for",
            },
        ],
        return_type: "boolean",
        examples: &["{{ object_has_key(object=config, key=\"host\") }}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let object: Value = kwargs.get("object")?;
        let key: String = kwargs.get("key")?;

        let json_value: serde_json::Value = serde_json::to_value(&object).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Failed to convert object: {}", e),
            )
        })?;

        if let serde_json::Value::Object(map) = json_value {
            Ok(Value::from(map.contains_key(&key)))
        } else {
            Ok(Value::from(false))
        }
    }
}

/// Query object using JSONPath-like syntax
pub struct JsonPath;

impl Function for JsonPath {
    const NAME: &'static str = "json_path";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "json_path",
        category: "object",
        description: "Query object using JSONPath-like syntax",
        arguments: &[
            ArgumentMetadata {
                name: "object",
                arg_type: "object",
                required: true,
                default: None,
                description: "Object or array to query",
            },
            ArgumentMetadata {
                name: "path",
                arg_type: "string",
                required: true,
                default: None,
                description: "JSONPath expression (e.g., \"$.users[*].name\")",
            },
        ],
        return_type: "any",
        examples: &[
            "{{ json_path(object=data, path=\"$.users[0].name\") }}",
            "{{ json_path(object=data, path=\"$.users[*].name\") }}",
        ],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let object: Value = kwargs.get("object")?;
        let path: String = kwargs.get("path")?;

        let json_value: serde_json::Value = serde_json::to_value(&object).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Failed to convert object: {}", e),
            )
        })?;

        // Remove leading $. if present
        let path = path.strip_prefix("$.").unwrap_or(&path);
        let path = path.strip_prefix('$').unwrap_or(path);

        if path.is_empty() {
            return Ok(Value::from_serialize(&json_value));
        }

        let result = query_json_path(&json_value, path)?;
        Ok(Value::from_serialize(&result))
    }
}

/// Parse and execute JSONPath query
fn query_json_path(value: &serde_json::Value, path: &str) -> Result<serde_json::Value, Error> {
    // Split path into segments, handling array notation
    let mut current = value.clone();
    let mut remaining_path = path;

    while !remaining_path.is_empty() {
        // Check for array notation [n] or [*]
        if remaining_path.starts_with('[') {
            if let Some(end) = remaining_path.find(']') {
                let index_str = &remaining_path[1..end];
                remaining_path = &remaining_path[end + 1..];

                // Skip leading dot if present
                remaining_path = remaining_path.strip_prefix('.').unwrap_or(remaining_path);

                if index_str == "*" {
                    // Wildcard - return all elements with remaining path applied
                    if let serde_json::Value::Array(arr) = &current {
                        if remaining_path.is_empty() {
                            return Ok(serde_json::Value::Array(arr.clone()));
                        } else {
                            let results: Vec<serde_json::Value> = arr
                                .iter()
                                .filter_map(|item| query_json_path(item, remaining_path).ok())
                                .collect();
                            return Ok(serde_json::Value::Array(results));
                        }
                    } else {
                        return Ok(serde_json::Value::Null);
                    }
                } else if let Ok(index) = index_str.parse::<usize>() {
                    if let serde_json::Value::Array(arr) = &current {
                        current = arr.get(index).cloned().unwrap_or(serde_json::Value::Null);
                    } else {
                        return Ok(serde_json::Value::Null);
                    }
                } else {
                    return Err(Error::new(
                        ErrorKind::InvalidOperation,
                        format!("Invalid array index: {}", index_str),
                    ));
                }
            } else {
                return Err(Error::new(
                    ErrorKind::InvalidOperation,
                    "Unclosed bracket in path".to_string(),
                ));
            }
        } else {
            // Regular property access
            let (key, rest) = if let Some(dot_pos) = remaining_path.find('.') {
                let bracket_pos = remaining_path.find('[');
                if let Some(bp) = bracket_pos {
                    if bp < dot_pos {
                        (&remaining_path[..bp], &remaining_path[bp..])
                    } else {
                        (&remaining_path[..dot_pos], &remaining_path[dot_pos + 1..])
                    }
                } else {
                    (&remaining_path[..dot_pos], &remaining_path[dot_pos + 1..])
                }
            } else if let Some(bracket_pos) = remaining_path.find('[') {
                (
                    &remaining_path[..bracket_pos],
                    &remaining_path[bracket_pos..],
                )
            } else {
                (remaining_path, "")
            };

            remaining_path = rest;

            if let serde_json::Value::Object(map) = &current {
                current = map.get(key).cloned().unwrap_or(serde_json::Value::Null);
            } else {
                return Ok(serde_json::Value::Null);
            }
        }
    }

    Ok(current)
}

/// Create new object with only specified keys
pub struct ObjectPick;

impl Function for ObjectPick {
    const NAME: &'static str = "object_pick";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "object_pick",
        category: "object",
        description: "Create new object with only specified keys",
        arguments: &[
            ArgumentMetadata {
                name: "object",
                arg_type: "object",
                required: true,
                default: None,
                description: "Source object",
            },
            ArgumentMetadata {
                name: "keys",
                arg_type: "array",
                required: true,
                default: None,
                description: "Array of keys to keep",
            },
        ],
        return_type: "object",
        examples: &["{{ object_pick(object=user, keys=[\"name\", \"email\"]) }}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let object: Value = kwargs.get("object")?;
        let keys: Value = kwargs.get("keys")?;

        let json_object: serde_json::Value = serde_json::to_value(&object).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Failed to convert object: {}", e),
            )
        })?;

        let keys_array: serde_json::Value = serde_json::to_value(&keys).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Failed to convert keys: {}", e),
            )
        })?;

        let keys_to_keep: Vec<String> = match keys_array {
            serde_json::Value::Array(arr) => arr
                .iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect(),
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidOperation,
                    "keys must be an array of strings".to_string(),
                ));
            }
        };

        if let serde_json::Value::Object(map) = json_object {
            let mut result = Map::new();
            for key in keys_to_keep {
                if let Some(value) = map.get(&key) {
                    result.insert(key, value.clone());
                }
            }
            Ok(Value::from_serialize(serde_json::Value::Object(result)))
        } else {
            Err(Error::new(
                ErrorKind::InvalidOperation,
                "object_pick requires an object".to_string(),
            ))
        }
    }
}

/// Create new object without specified keys
pub struct ObjectOmit;

impl Function for ObjectOmit {
    const NAME: &'static str = "object_omit";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "object_omit",
        category: "object",
        description: "Create new object without specified keys",
        arguments: &[
            ArgumentMetadata {
                name: "object",
                arg_type: "object",
                required: true,
                default: None,
                description: "Source object",
            },
            ArgumentMetadata {
                name: "keys",
                arg_type: "array",
                required: true,
                default: None,
                description: "Array of keys to exclude",
            },
        ],
        return_type: "object",
        examples: &["{{ object_omit(object=user, keys=[\"password\"]) }}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let object: Value = kwargs.get("object")?;
        let keys: Value = kwargs.get("keys")?;

        let json_object: serde_json::Value = serde_json::to_value(&object).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Failed to convert object: {}", e),
            )
        })?;

        let keys_array: serde_json::Value = serde_json::to_value(&keys).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Failed to convert keys: {}", e),
            )
        })?;

        let keys_to_omit: Vec<String> = match keys_array {
            serde_json::Value::Array(arr) => arr
                .iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect(),
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidOperation,
                    "keys must be an array of strings".to_string(),
                ));
            }
        };

        if let serde_json::Value::Object(map) = json_object {
            let mut result = Map::new();
            for (key, value) in map {
                if !keys_to_omit.contains(&key) {
                    result.insert(key, value);
                }
            }
            Ok(Value::from_serialize(serde_json::Value::Object(result)))
        } else {
            Err(Error::new(
                ErrorKind::InvalidOperation,
                "object_omit requires an object".to_string(),
            ))
        }
    }
}

/// Rename object keys using a mapping
pub struct ObjectRenameKeys;

impl Function for ObjectRenameKeys {
    const NAME: &'static str = "object_rename_keys";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "object_rename_keys",
        category: "object",
        description: "Rename object keys using a mapping",
        arguments: &[
            ArgumentMetadata {
                name: "object",
                arg_type: "object",
                required: true,
                default: None,
                description: "Source object",
            },
            ArgumentMetadata {
                name: "mapping",
                arg_type: "object",
                required: true,
                default: None,
                description: "Object mapping old keys to new keys",
            },
        ],
        return_type: "object",
        examples: &[
            "{{ object_rename_keys(object=data, mapping={\"firstName\": \"first_name\"}) }}",
        ],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let object: Value = kwargs.get("object")?;
        let mapping: Value = kwargs.get("mapping")?;

        let json_object: serde_json::Value = serde_json::to_value(&object).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Failed to convert object: {}", e),
            )
        })?;

        let json_mapping: serde_json::Value = serde_json::to_value(&mapping).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Failed to convert mapping: {}", e),
            )
        })?;

        let key_map: std::collections::HashMap<String, String> = match json_mapping {
            serde_json::Value::Object(map) => map
                .iter()
                .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                .collect(),
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidOperation,
                    "mapping must be an object".to_string(),
                ));
            }
        };

        if let serde_json::Value::Object(map) = json_object {
            let mut result = Map::new();
            for (key, value) in map {
                let new_key = key_map.get(&key).cloned().unwrap_or(key);
                result.insert(new_key, value);
            }
            Ok(Value::from_serialize(serde_json::Value::Object(result)))
        } else {
            Err(Error::new(
                ErrorKind::InvalidOperation,
                "object_rename_keys requires an object".to_string(),
            ))
        }
    }
}

/// Unflatten dot-notation object to nested structure
pub struct ObjectUnflatten;

impl Function for ObjectUnflatten {
    const NAME: &'static str = "object_unflatten";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "object_unflatten",
        category: "object",
        description: "Unflatten dot-notation object to nested structure",
        arguments: &[
            ArgumentMetadata {
                name: "object",
                arg_type: "object",
                required: true,
                default: None,
                description: "Flat object with dot-notation keys",
            },
            ArgumentMetadata {
                name: "delimiter",
                arg_type: "string",
                required: false,
                default: Some("\".\""),
                description: "Delimiter used in keys (default: \".\")",
            },
        ],
        return_type: "object",
        examples: &["{{ object_unflatten(object={\"server.host\": \"localhost\"}) }}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let object: Value = kwargs.get("object")?;
        let delimiter: Option<String> = kwargs.get("delimiter")?;
        let delimiter = delimiter.unwrap_or_else(|| ".".to_string());

        let json_object: serde_json::Value = serde_json::to_value(&object).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Failed to convert object: {}", e),
            )
        })?;

        if let serde_json::Value::Object(map) = json_object {
            let mut result = serde_json::Value::Object(Map::new());

            for (key, value) in map {
                let parts: Vec<&str> = key.split(&delimiter).collect();
                set_nested_value_json(&mut result, &parts, value)?;
            }

            Ok(Value::from_serialize(&result))
        } else {
            Err(Error::new(
                ErrorKind::InvalidOperation,
                "object_unflatten requires an object".to_string(),
            ))
        }
    }
}

/// Set nested value in JSON structure for unflatten
fn set_nested_value_json(
    current: &mut serde_json::Value,
    parts: &[&str],
    value: serde_json::Value,
) -> Result<(), Error> {
    if parts.is_empty() {
        return Ok(());
    }

    if parts.len() == 1 {
        if let serde_json::Value::Object(map) = current {
            map.insert(parts[0].to_string(), value);
        }
        return Ok(());
    }

    // Ensure current is an object
    if !current.is_object() {
        *current = serde_json::Value::Object(Map::new());
    }

    if let serde_json::Value::Object(map) = current {
        let next_part = parts[0];
        let entry = map
            .entry(next_part.to_string())
            .or_insert_with(|| serde_json::Value::Object(Map::new()));
        set_nested_value_json(entry, &parts[1..], value)?;
    }

    Ok(())
}
