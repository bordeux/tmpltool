//! Object manipulation functions for MiniJinja templates
//!
//! This module provides functions for:
//! - Merging objects
//! - Getting/setting nested values by path
//! - Extracting keys and values
//! - Checking key existence
//! - JSONPath queries
//! - Object picking/omitting keys
//! - Key renaming
//! - Flattening/unflattening nested objects

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

/// Query object using JSONPath-like syntax
///
/// Supports basic JSONPath operations:
/// - `$.key` or `key` - Access object property
/// - `$.key1.key2` - Nested property access
/// - `$.array[0]` - Array index access
/// - `$.array[*]` - Wildcard (returns all elements)
/// - `$.users[*].name` - Extract property from all array elements
///
/// # Arguments
///
/// * `object` (required) - Object or array to query
/// * `path` (required) - JSONPath expression
///
/// # Returns
///
/// Returns the matched value(s). For wildcard queries, returns an array.
///
/// # Example
///
/// ```jinja
/// {% set data = {"users": [{"name": "Alice", "age": 30}, {"name": "Bob", "age": 25}]} %}
/// {{ json_path(object=data, path="$.users[0].name") }}
/// {# Output: Alice #}
///
/// {{ json_path(object=data, path="$.users[*].name") | tojson }}
/// {# Output: ["Alice", "Bob"] #}
///
/// {% set config = {"server": {"host": "localhost", "port": 8080}} %}
/// {{ json_path(object=config, path="$.server.port") }}
/// {# Output: 8080 #}
/// ```
pub fn json_path_fn(kwargs: Kwargs) -> Result<Value, Error> {
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
                (&remaining_path[..bracket_pos], &remaining_path[bracket_pos..])
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
///
/// # Arguments
///
/// * `object` (required) - Source object
/// * `keys` (required) - Array of keys to keep
///
/// # Returns
///
/// Returns a new object containing only the specified keys
///
/// # Example
///
/// ```jinja
/// {% set user = {"name": "Alice", "email": "alice@example.com", "password": "secret", "age": 30} %}
/// {% set public = object_pick(object=user, keys=["name", "email", "age"]) %}
/// {# Result: {"name": "Alice", "email": "alice@example.com", "age": 30} #}
///
/// {{ object_pick(object=config, keys=["host", "port"]) | tojson }}
/// ```
pub fn object_pick_fn(kwargs: Kwargs) -> Result<Value, Error> {
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
            ))
        }
    };

    if let serde_json::Value::Object(map) = json_object {
        let mut result = Map::new();
        for key in keys_to_keep {
            if let Some(value) = map.get(&key) {
                result.insert(key, value.clone());
            }
        }
        Ok(Value::from_serialize(&serde_json::Value::Object(result)))
    } else {
        Err(Error::new(
            ErrorKind::InvalidOperation,
            "object_pick requires an object".to_string(),
        ))
    }
}

/// Create new object without specified keys
///
/// # Arguments
///
/// * `object` (required) - Source object
/// * `keys` (required) - Array of keys to exclude
///
/// # Returns
///
/// Returns a new object with the specified keys removed
///
/// # Example
///
/// ```jinja
/// {% set user = {"name": "Alice", "email": "alice@example.com", "password": "secret", "age": 30} %}
/// {% set safe = object_omit(object=user, keys=["password"]) %}
/// {# Result: {"name": "Alice", "email": "alice@example.com", "age": 30} #}
///
/// {{ object_omit(object=config, keys=["internal", "debug"]) | tojson }}
/// ```
pub fn object_omit_fn(kwargs: Kwargs) -> Result<Value, Error> {
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
            ))
        }
    };

    if let serde_json::Value::Object(map) = json_object {
        let mut result = Map::new();
        for (key, value) in map {
            if !keys_to_omit.contains(&key) {
                result.insert(key, value);
            }
        }
        Ok(Value::from_serialize(&serde_json::Value::Object(result)))
    } else {
        Err(Error::new(
            ErrorKind::InvalidOperation,
            "object_omit requires an object".to_string(),
        ))
    }
}

/// Rename object keys using a mapping
///
/// # Arguments
///
/// * `object` (required) - Source object
/// * `mapping` (required) - Object mapping old keys to new keys
///
/// # Returns
///
/// Returns a new object with renamed keys
///
/// # Example
///
/// ```jinja
/// {% set data = {"firstName": "Alice", "lastName": "Smith", "emailAddress": "alice@example.com"} %}
/// {% set renamed = object_rename_keys(object=data, mapping={"firstName": "first_name", "lastName": "last_name", "emailAddress": "email"}) %}
/// {# Result: {"first_name": "Alice", "last_name": "Smith", "email": "alice@example.com"} #}
///
/// {# Useful for API response transformation #}
/// {% set api_data = object_rename_keys(object=response, mapping={"userId": "user_id", "createdAt": "created_at"}) %}
/// ```
pub fn object_rename_keys_fn(kwargs: Kwargs) -> Result<Value, Error> {
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
            ))
        }
    };

    if let serde_json::Value::Object(map) = json_object {
        let mut result = Map::new();
        for (key, value) in map {
            let new_key = key_map.get(&key).cloned().unwrap_or(key);
            result.insert(new_key, value);
        }
        Ok(Value::from_serialize(&serde_json::Value::Object(result)))
    } else {
        Err(Error::new(
            ErrorKind::InvalidOperation,
            "object_rename_keys requires an object".to_string(),
        ))
    }
}

/// Flatten nested object to dot notation
///
/// # Arguments
///
/// * `object` (required) - Nested object to flatten
/// * `delimiter` (optional) - Delimiter for keys (default: ".")
///
/// # Returns
///
/// Returns a flat object with dot-notation keys
///
/// # Example
///
/// ```jinja
/// {% set nested = {"server": {"host": "localhost", "port": 8080}, "database": {"name": "mydb"}} %}
/// {% set flat = object_flatten(object=nested) %}
/// {# Result: {"server.host": "localhost", "server.port": 8080, "database.name": "mydb"} #}
///
/// {% set flat_underscore = object_flatten(object=nested, delimiter="_") %}
/// {# Result: {"server_host": "localhost", "server_port": 8080, "database_name": "mydb"} #}
/// ```
pub fn object_flatten_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let object: Value = kwargs.get("object")?;
    let delimiter: Option<String> = kwargs.get("delimiter")?;
    let delimiter = delimiter.unwrap_or_else(|| ".".to_string());

    let json_object: serde_json::Value = serde_json::to_value(&object).map_err(|e| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("Failed to convert object: {}", e),
        )
    })?;

    if !json_object.is_object() {
        return Err(Error::new(
            ErrorKind::InvalidOperation,
            "object_flatten requires an object".to_string(),
        ));
    }

    let mut result = Map::new();
    flatten_recursive(&json_object, "", &delimiter, &mut result);

    Ok(Value::from_serialize(&serde_json::Value::Object(result)))
}

/// Recursively flatten object
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
                flatten_recursive(val, &new_key, delimiter, result);
            }
        }
        serde_json::Value::Array(arr) => {
            for (index, val) in arr.iter().enumerate() {
                let new_key = if prefix.is_empty() {
                    index.to_string()
                } else {
                    format!("{}{}{}",  prefix, delimiter, index)
                };
                flatten_recursive(val, &new_key, delimiter, result);
            }
        }
        _ => {
            result.insert(prefix.to_string(), value.clone());
        }
    }
}

/// Unflatten dot-notation object to nested structure
///
/// # Arguments
///
/// * `object` (required) - Flat object with dot-notation keys
/// * `delimiter` (optional) - Delimiter used in keys (default: ".")
///
/// # Returns
///
/// Returns a nested object structure
///
/// # Example
///
/// ```jinja
/// {% set flat = {"server.host": "localhost", "server.port": 8080, "database.name": "mydb"} %}
/// {% set nested = object_unflatten(object=flat) %}
/// {# Result: {"server": {"host": "localhost", "port": 8080}, "database": {"name": "mydb"}} #}
///
/// {% set flat_underscore = {"server_host": "localhost", "server_port": 8080} %}
/// {% set nested = object_unflatten(object=flat_underscore, delimiter="_") %}
/// {# Result: {"server": {"host": "localhost", "port": 8080}} #}
/// ```
pub fn object_unflatten_fn(kwargs: Kwargs) -> Result<Value, Error> {
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
