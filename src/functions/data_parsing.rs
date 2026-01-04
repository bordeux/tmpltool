//! Data parsing functions
//!
//! Provides functions for reading and parsing structured data files:
//! - read_json_file: Read and parse JSON file
//! - read_yaml_file: Read and parse YAML file
//! - read_toml_file: Read and parse TOML file
//!
//! Note: parse_json, parse_yaml, parse_toml (string parsing) are now in
//! filter_functions/serialization.rs with dual function+filter syntax support.

use super::metadata::{ArgumentMetadata, FunctionMetadata, SyntaxVariants};
use super::traits::ContextFunction;
use crate::TemplateContext;
use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};
use std::fs;
use std::sync::Arc;

/// Read and parse a JSON file
pub struct ReadJsonFile;

impl ContextFunction for ReadJsonFile {
    const NAME: &'static str = "read_json_file";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "read_json_file",
        category: "data_parsing",
        description: "Read and parse a JSON file",
        arguments: &[ArgumentMetadata {
            name: "path",
            arg_type: "string",
            required: true,
            default: None,
            description: "Path to the JSON file",
        }],
        return_type: "object|array",
        examples: &[
            "{% set config = read_json_file(path=\"config.json\") %}",
            "{{ read_json_file(path=\"data.json\").items | length }}",
        ],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(context: Arc<TemplateContext>, kwargs: Kwargs) -> Result<Value, Error> {
        let path: String = kwargs.get("path")?;

        // Security checks
        if !context.is_trust_mode() {
            crate::functions::filesystem::validate_path_security(&path)?;
        }

        let resolved_path = context.resolve_path(&path);

        let content = fs::read_to_string(&resolved_path).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Failed to read file '{}': {}", resolved_path.display(), e),
            )
        })?;

        let json_value: serde_json::Value = serde_json::from_str(&content).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!(
                    "Failed to parse JSON from file '{}': {}",
                    resolved_path.display(),
                    e
                ),
            )
        })?;

        Ok(Value::from_serialize(&json_value))
    }
}

/// Read and parse a YAML file
pub struct ReadYamlFile;

impl ContextFunction for ReadYamlFile {
    const NAME: &'static str = "read_yaml_file";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "read_yaml_file",
        category: "data_parsing",
        description: "Read and parse a YAML file",
        arguments: &[ArgumentMetadata {
            name: "path",
            arg_type: "string",
            required: true,
            default: None,
            description: "Path to the YAML file",
        }],
        return_type: "object|array",
        examples: &[
            "{% set config = read_yaml_file(path=\"config.yaml\") %}",
            "{{ read_yaml_file(path=\"values.yml\").database.host }}",
        ],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(context: Arc<TemplateContext>, kwargs: Kwargs) -> Result<Value, Error> {
        let path: String = kwargs.get("path")?;

        if !context.is_trust_mode() {
            crate::functions::filesystem::validate_path_security(&path)?;
        }

        let resolved_path = context.resolve_path(&path);

        let content = fs::read_to_string(&resolved_path).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Failed to read file '{}': {}", resolved_path.display(), e),
            )
        })?;

        let yaml_value: serde_yaml::Value = serde_yaml::from_str(&content).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!(
                    "Failed to parse YAML from file '{}': {}",
                    resolved_path.display(),
                    e
                ),
            )
        })?;

        let json_value = serde_yaml_to_json(yaml_value).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Failed to convert YAML to JSON: {}", e),
            )
        })?;

        Ok(Value::from_serialize(&json_value))
    }
}

/// Read and parse a TOML file
pub struct ReadTomlFile;

impl ContextFunction for ReadTomlFile {
    const NAME: &'static str = "read_toml_file";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "read_toml_file",
        category: "data_parsing",
        description: "Read and parse a TOML file",
        arguments: &[ArgumentMetadata {
            name: "path",
            arg_type: "string",
            required: true,
            default: None,
            description: "Path to the TOML file",
        }],
        return_type: "object",
        examples: &[
            "{% set config = read_toml_file(path=\"Cargo.toml\") %}",
            "{{ read_toml_file(path=\"config.toml\").package.version }}",
        ],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(context: Arc<TemplateContext>, kwargs: Kwargs) -> Result<Value, Error> {
        let path: String = kwargs.get("path")?;

        if !context.is_trust_mode() {
            crate::functions::filesystem::validate_path_security(&path)?;
        }

        let resolved_path = context.resolve_path(&path);

        let content = fs::read_to_string(&resolved_path).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Failed to read file '{}': {}", resolved_path.display(), e),
            )
        })?;

        let toml_value: toml::Value = toml::from_str(&content).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!(
                    "Failed to parse TOML from file '{}': {}",
                    resolved_path.display(),
                    e
                ),
            )
        })?;

        let json_value = toml_to_json(toml_value).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Failed to convert TOML to JSON: {}", e),
            )
        })?;

        Ok(Value::from_serialize(&json_value))
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
        serde_yaml::Value::Tagged(tagged) => serde_yaml_to_json(tagged.value),
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
