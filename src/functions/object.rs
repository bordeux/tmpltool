//! Object manipulation functions for MiniJinja templates
//!
//! This module provides functions for:
//! - Merging objects
//! - Getting/setting nested values by path
//! - Extracting keys and values
//! - Checking key existence

use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};
use serde_json::Map;

/// Deep merge two objects
///
/// # Arguments
///
/// * `obj1` (required) - First object (base)
/// * `obj2` (required) - Second object (overlay, takes precedence)
///
/// # Returns
///
/// Returns a new object with merged values. obj2 values override obj1 values.
/// Nested objects are merged recursively.
///
/// # Example
///
/// ```jinja
/// {% set base = {"a": 1, "b": {"c": 2}} %}
/// {% set overlay = {"b": {"d": 3}, "e": 4} %}
/// {% set merged = object_merge(obj1=base, obj2=overlay) %}
/// {# Result: {"a": 1, "b": {"c": 2, "d": 3}, "e": 4} #}
/// ```
pub fn object_merge_fn(kwargs: Kwargs) -> Result<Value, Error> {
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
///
/// # Arguments
///
/// * `object` (required) - Object to query
/// * `path` (required) - Dot-separated path (e.g., "a.b.c")
///
/// # Returns
///
/// Returns the value at the specified path, or undefined if not found
///
/// # Example
///
/// ```jinja
/// {% set config = {"server": {"host": "localhost", "port": 8080}} %}
/// {{ object_get(object=config, path="server.host") }}
/// {# Output: localhost #}
///
/// {{ object_get(object=config, path="server.port") }}
/// {# Output: 8080 #}
/// ```
pub fn object_get_fn(kwargs: Kwargs) -> Result<Value, Error> {
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

/// Set nested value by path
///
/// # Arguments
///
/// * `object` (required) - Object to modify
/// * `path` (required) - Dot-separated path (e.g., "a.b.c")
/// * `value` (required) - Value to set
///
/// # Returns
///
/// Returns a new object with the value set at the specified path.
/// Creates intermediate objects as needed.
///
/// # Example
///
/// ```jinja
/// {% set config = {"server": {"host": "localhost"}} %}
/// {% set updated = object_set(object=config, path="server.port", value=8080) %}
/// {# Result: {"server": {"host": "localhost", "port": 8080}} #}
///
/// {% set updated = object_set(object=config, path="database.host", value="db.local") %}
/// {# Creates: {"server": {...}, "database": {"host": "db.local"}} #}
/// ```
pub fn object_set_fn(kwargs: Kwargs) -> Result<Value, Error> {
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

/// Get object keys as array
///
/// # Arguments
///
/// * `object` (required) - Object to get keys from
///
/// # Returns
///
/// Returns an array of string keys
///
/// # Example
///
/// ```jinja
/// {% set config = {"host": "localhost", "port": 8080, "debug": true} %}
/// {% set keys = object_keys(object=config) %}
/// {# Result: ["host", "port", "debug"] #}
///
/// {% for key in object_keys(object=config) %}
///   {{ key }}: {{ config[key] }}
/// {% endfor %}
/// ```
pub fn object_keys_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let object: Value = kwargs.get("object")?;

    let json_value: serde_json::Value = serde_json::to_value(&object).map_err(|e| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("Failed to convert object: {}", e),
        )
    })?;

    if let serde_json::Value::Object(map) = json_value {
        let keys: Vec<String> = map.keys().cloned().collect();
        Ok(Value::from_serialize(&keys))
    } else {
        Err(Error::new(
            ErrorKind::InvalidOperation,
            "object_keys requires an object, not an array or primitive".to_string(),
        ))
    }
}

/// Get object values as array
///
/// # Arguments
///
/// * `object` (required) - Object to get values from
///
/// # Returns
///
/// Returns an array of values
///
/// # Example
///
/// ```jinja
/// {% set config = {"host": "localhost", "port": 8080, "debug": true} %}
/// {% set values = object_values(object=config) %}
/// {# Result: ["localhost", 8080, true] #}
///
/// {% for value in object_values(object=config) %}
///   - {{ value }}
/// {% endfor %}
/// ```
pub fn object_values_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let object: Value = kwargs.get("object")?;

    let json_value: serde_json::Value = serde_json::to_value(&object).map_err(|e| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("Failed to convert object: {}", e),
        )
    })?;

    if let serde_json::Value::Object(map) = json_value {
        let values: Vec<&serde_json::Value> = map.values().collect();
        Ok(Value::from_serialize(&values))
    } else {
        Err(Error::new(
            ErrorKind::InvalidOperation,
            "object_values requires an object, not an array or primitive".to_string(),
        ))
    }
}

/// Check if object has key
///
/// # Arguments
///
/// * `object` (required) - Object to check
/// * `key` (required) - Key to check for
///
/// # Returns
///
/// Returns true if the key exists, false otherwise
///
/// # Example
///
/// ```jinja
/// {% set config = {"host": "localhost", "port": 8080} %}
/// {{ object_has_key(object=config, key="host") }}
/// {# Output: true #}
///
/// {{ object_has_key(object=config, key="database") }}
/// {# Output: false #}
///
/// {% if object_has_key(object=config, key="debug") %}
///   Debug mode: {{ config.debug }}
/// {% else %}
///   Debug mode not configured
/// {% endif %}
/// ```
pub fn object_has_key_fn(kwargs: Kwargs) -> Result<Value, Error> {
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
