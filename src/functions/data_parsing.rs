/// Data parsing functions
///
/// Provides functions for parsing structured data formats:
/// - parse_json: Parse JSON string into object
/// - parse_yaml: Parse YAML string into object
/// - parse_toml: Parse TOML string into object
/// - read_json_file: Read and parse JSON file
/// - read_yaml_file: Read and parse YAML file
/// - read_toml_file: Read and parse TOML file
use std::collections::HashMap;
use std::fs;
use tera::{Function, Result, Value};

use crate::TemplateContext;

/// Parse JSON string into object
pub struct ParseJson;

impl Function for ParseJson {
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value> {
        let string = args.get("string").and_then(|v| v.as_str()).ok_or_else(|| {
            tera::Error::msg(
                "parse_json requires a 'string' argument (e.g., string='{\"key\": \"value\"}')",
            )
        })?;

        // Parse JSON string
        let json_value: serde_json::Value = serde_json::from_str(string)
            .map_err(|e| tera::Error::msg(format!("Failed to parse JSON: {}", e)))?;

        // Convert serde_json::Value to tera::Value
        // Tera's Value is based on serde_json::Value, so we can convert directly
        let tera_value = serde_json::from_value(json_value).map_err(|e| {
            tera::Error::msg(format!("Failed to convert JSON to Tera value: {}", e))
        })?;

        Ok(tera_value)
    }
}

/// Parse YAML string into object
pub struct ParseYaml;

impl Function for ParseYaml {
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value> {
        let string = args.get("string").and_then(|v| v.as_str()).ok_or_else(|| {
            tera::Error::msg("parse_yaml requires a 'string' argument (e.g., string='key: value')")
        })?;

        // Parse YAML string to serde_yaml::Value
        let yaml_value: serde_yaml::Value = serde_yaml::from_str(string)
            .map_err(|e| tera::Error::msg(format!("Failed to parse YAML: {}", e)))?;

        // Convert serde_yaml::Value to serde_json::Value (Tera uses serde_json::Value)
        let json_value = serde_yaml_to_json(yaml_value)
            .map_err(|e| tera::Error::msg(format!("Failed to convert YAML to JSON: {}", e)))?;

        // Convert to Tera Value
        let tera_value = serde_json::from_value(json_value)
            .map_err(|e| tera::Error::msg(format!("Failed to convert to Tera value: {}", e)))?;

        Ok(tera_value)
    }
}

/// Parse TOML string into object
pub struct ParseToml;

impl Function for ParseToml {
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value> {
        let string = args.get("string").and_then(|v| v.as_str()).ok_or_else(|| {
            tera::Error::msg(
                "parse_toml requires a 'string' argument (e.g., string='key = \"value\"')",
            )
        })?;

        // Parse TOML string to toml::Value
        let toml_value: toml::Value = toml::from_str(string)
            .map_err(|e| tera::Error::msg(format!("Failed to parse TOML: {}", e)))?;

        // Convert toml::Value to serde_json::Value
        let json_value = toml_to_json(toml_value)
            .map_err(|e| tera::Error::msg(format!("Failed to convert TOML to JSON: {}", e)))?;

        // Convert to Tera Value
        let tera_value = serde_json::from_value(json_value)
            .map_err(|e| tera::Error::msg(format!("Failed to convert to Tera value: {}", e)))?;

        Ok(tera_value)
    }
}

/// Read and parse JSON file
pub struct ReadJsonFile {
    context: TemplateContext,
}

impl ReadJsonFile {
    pub fn new(context: TemplateContext) -> Self {
        Self { context }
    }
}

impl Function for ReadJsonFile {
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value> {
        let path = args.get("path").and_then(|v| v.as_str()).ok_or_else(|| {
            tera::Error::msg("read_json_file requires a 'path' argument (e.g., path=\"data.json\")")
        })?;

        // Security checks (unless trust mode is enabled)
        if !self.context.is_trust_mode() {
            crate::functions::filesystem::validate_path_security(path)?;
        }

        // Resolve path relative to template's directory
        let resolved_path = self.context.resolve_path(path);

        // Read file content
        let content = fs::read_to_string(&resolved_path).map_err(|e| {
            tera::Error::msg(format!(
                "Failed to read file '{}': {}",
                resolved_path.display(),
                e
            ))
        })?;

        // Parse JSON
        let json_value: serde_json::Value = serde_json::from_str(&content).map_err(|e| {
            tera::Error::msg(format!(
                "Failed to parse JSON from file '{}': {}",
                resolved_path.display(),
                e
            ))
        })?;

        // Convert to Tera Value
        let tera_value = serde_json::from_value(json_value).map_err(|e| {
            tera::Error::msg(format!("Failed to convert JSON to Tera value: {}", e))
        })?;

        Ok(tera_value)
    }
}

/// Read and parse YAML file
pub struct ReadYamlFile {
    context: TemplateContext,
}

impl ReadYamlFile {
    pub fn new(context: TemplateContext) -> Self {
        Self { context }
    }
}

impl Function for ReadYamlFile {
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value> {
        let path = args.get("path").and_then(|v| v.as_str()).ok_or_else(|| {
            tera::Error::msg("read_yaml_file requires a 'path' argument (e.g., path=\"data.yaml\")")
        })?;

        // Security checks (unless trust mode is enabled)
        if !self.context.is_trust_mode() {
            crate::functions::filesystem::validate_path_security(path)?;
        }

        // Resolve path relative to template's directory
        let resolved_path = self.context.resolve_path(path);

        // Read file content
        let content = fs::read_to_string(&resolved_path).map_err(|e| {
            tera::Error::msg(format!(
                "Failed to read file '{}': {}",
                resolved_path.display(),
                e
            ))
        })?;

        // Parse YAML
        let yaml_value: serde_yaml::Value = serde_yaml::from_str(&content).map_err(|e| {
            tera::Error::msg(format!(
                "Failed to parse YAML from file '{}': {}",
                resolved_path.display(),
                e
            ))
        })?;

        // Convert to JSON Value
        let json_value = serde_yaml_to_json(yaml_value)
            .map_err(|e| tera::Error::msg(format!("Failed to convert YAML to JSON: {}", e)))?;

        // Convert to Tera Value
        let tera_value = serde_json::from_value(json_value)
            .map_err(|e| tera::Error::msg(format!("Failed to convert to Tera value: {}", e)))?;

        Ok(tera_value)
    }
}

/// Read and parse TOML file
pub struct ReadTomlFile {
    context: TemplateContext,
}

impl ReadTomlFile {
    pub fn new(context: TemplateContext) -> Self {
        Self { context }
    }
}

impl Function for ReadTomlFile {
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value> {
        let path = args.get("path").and_then(|v| v.as_str()).ok_or_else(|| {
            tera::Error::msg("read_toml_file requires a 'path' argument (e.g., path=\"data.toml\")")
        })?;

        // Security checks (unless trust mode is enabled)
        if !self.context.is_trust_mode() {
            crate::functions::filesystem::validate_path_security(path)?;
        }

        // Resolve path relative to template's directory
        let resolved_path = self.context.resolve_path(path);

        // Read file content
        let content = fs::read_to_string(&resolved_path).map_err(|e| {
            tera::Error::msg(format!(
                "Failed to read file '{}': {}",
                resolved_path.display(),
                e
            ))
        })?;

        // Parse TOML
        let toml_value: toml::Value = toml::from_str(&content).map_err(|e| {
            tera::Error::msg(format!(
                "Failed to parse TOML from file '{}': {}",
                resolved_path.display(),
                e
            ))
        })?;

        // Convert to JSON Value
        let json_value = toml_to_json(toml_value)
            .map_err(|e| tera::Error::msg(format!("Failed to convert TOML to JSON: {}", e)))?;

        // Convert to Tera Value
        let tera_value = serde_json::from_value(json_value)
            .map_err(|e| tera::Error::msg(format!("Failed to convert to Tera value: {}", e)))?;

        Ok(tera_value)
    }
}

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
