use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tera::{Function, Value};
use tmpltool::TemplateContext;
use tmpltool::functions::data_parsing::{
    ParseJson, ParseToml, ParseYaml, ReadJsonFile, ReadTomlFile, ReadYamlFile,
};

// ========== parse_json tests ==========

#[test]
fn test_parse_json_simple_object() {
    let mut args = HashMap::new();
    args.insert(
        "string".to_string(),
        Value::String(r#"{"name": "John", "age": 30}"#.to_string()),
    );

    let result = ParseJson.call(&args).unwrap();
    let obj = result.as_object().unwrap();
    assert_eq!(obj.get("name").unwrap().as_str().unwrap(), "John");
    assert_eq!(obj.get("age").unwrap().as_u64().unwrap(), 30);
}

#[test]
fn test_parse_json_array() {
    let mut args = HashMap::new();
    args.insert(
        "string".to_string(),
        Value::String(r#"[1, 2, 3, 4, 5]"#.to_string()),
    );

    let result = ParseJson.call(&args).unwrap();
    let arr = result.as_array().unwrap();
    assert_eq!(arr.len(), 5);
    assert_eq!(arr[0].as_u64().unwrap(), 1);
    assert_eq!(arr[4].as_u64().unwrap(), 5);
}

#[test]
fn test_parse_json_nested() {
    let mut args = HashMap::new();
    args.insert(
        "string".to_string(),
        Value::String(r#"{"user": {"name": "Alice", "settings": {"theme": "dark"}}}"#.to_string()),
    );

    let result = ParseJson.call(&args).unwrap();
    let obj = result.as_object().unwrap();
    let user = obj.get("user").unwrap().as_object().unwrap();
    let settings = user.get("settings").unwrap().as_object().unwrap();
    assert_eq!(settings.get("theme").unwrap().as_str().unwrap(), "dark");
}

#[test]
fn test_parse_json_invalid() {
    let mut args = HashMap::new();
    args.insert(
        "string".to_string(),
        Value::String(r#"{"invalid": json"#.to_string()),
    );

    let result = ParseJson.call(&args);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Failed to parse JSON")
    );
}

#[test]
fn test_parse_json_missing_argument() {
    let args = HashMap::new();
    let result = ParseJson.call(&args);
    assert!(result.is_err());
}

// ========== parse_yaml tests ==========

#[test]
fn test_parse_yaml_simple_object() {
    let mut args = HashMap::new();
    args.insert(
        "string".to_string(),
        Value::String("name: John\nage: 30".to_string()),
    );

    let result = ParseYaml.call(&args).unwrap();
    let obj = result.as_object().unwrap();
    assert_eq!(obj.get("name").unwrap().as_str().unwrap(), "John");
    assert_eq!(obj.get("age").unwrap().as_u64().unwrap(), 30);
}

#[test]
fn test_parse_yaml_array() {
    let mut args = HashMap::new();
    args.insert(
        "string".to_string(),
        Value::String("- apple\n- banana\n- cherry".to_string()),
    );

    let result = ParseYaml.call(&args).unwrap();
    let arr = result.as_array().unwrap();
    assert_eq!(arr.len(), 3);
    assert_eq!(arr[0].as_str().unwrap(), "apple");
    assert_eq!(arr[2].as_str().unwrap(), "cherry");
}

#[test]
fn test_parse_yaml_nested() {
    let mut args = HashMap::new();
    args.insert(
        "string".to_string(),
        Value::String(
            "user:\n  name: Alice\n  settings:\n    theme: dark\n    notifications: true"
                .to_string(),
        ),
    );

    let result = ParseYaml.call(&args).unwrap();
    let obj = result.as_object().unwrap();
    let user = obj.get("user").unwrap().as_object().unwrap();
    let settings = user.get("settings").unwrap().as_object().unwrap();
    assert_eq!(settings.get("theme").unwrap().as_str().unwrap(), "dark");
    assert!(settings.get("notifications").unwrap().as_bool().unwrap());
}

#[test]
fn test_parse_yaml_invalid() {
    let mut args = HashMap::new();
    args.insert(
        "string".to_string(),
        Value::String("invalid:\n  - yaml\n  missing: indentation".to_string()),
    );

    let result = ParseYaml.call(&args);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Failed to parse YAML")
    );
}

#[test]
fn test_parse_yaml_missing_argument() {
    let args = HashMap::new();
    let result = ParseYaml.call(&args);
    assert!(result.is_err());
}

// ========== parse_toml tests ==========

#[test]
fn test_parse_toml_simple() {
    let mut args = HashMap::new();
    args.insert(
        "string".to_string(),
        Value::String("name = \"John\"\nage = 30".to_string()),
    );

    let result = ParseToml.call(&args).unwrap();
    let obj = result.as_object().unwrap();
    assert_eq!(obj.get("name").unwrap().as_str().unwrap(), "John");
    assert_eq!(obj.get("age").unwrap().as_u64().unwrap(), 30);
}

#[test]
fn test_parse_toml_array() {
    let mut args = HashMap::new();
    args.insert(
        "string".to_string(),
        Value::String("colors = [\"red\", \"green\", \"blue\"]".to_string()),
    );

    let result = ParseToml.call(&args).unwrap();
    let obj = result.as_object().unwrap();
    let colors = obj.get("colors").unwrap().as_array().unwrap();
    assert_eq!(colors.len(), 3);
    assert_eq!(colors[0].as_str().unwrap(), "red");
    assert_eq!(colors[2].as_str().unwrap(), "blue");
}

#[test]
fn test_parse_toml_table() {
    let mut args = HashMap::new();
    args.insert(
        "string".to_string(),
        Value::String("[database]\nhost = \"localhost\"\nport = 5432\nenabled = true".to_string()),
    );

    let result = ParseToml.call(&args).unwrap();
    let obj = result.as_object().unwrap();
    let db = obj.get("database").unwrap().as_object().unwrap();
    assert_eq!(db.get("host").unwrap().as_str().unwrap(), "localhost");
    assert_eq!(db.get("port").unwrap().as_u64().unwrap(), 5432);
    assert!(db.get("enabled").unwrap().as_bool().unwrap());
}

#[test]
fn test_parse_toml_invalid() {
    let mut args = HashMap::new();
    args.insert(
        "string".to_string(),
        Value::String("invalid toml = [missing quote".to_string()),
    );

    let result = ParseToml.call(&args);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Failed to parse TOML")
    );
}

#[test]
fn test_parse_toml_missing_argument() {
    let args = HashMap::new();
    let result = ParseToml.call(&args);
    assert!(result.is_err());
}

// ========== read_json_file tests ==========

#[test]
fn test_read_json_file_success() {
    // Create a temporary JSON file
    let temp_dir = std::env::temp_dir();
    let json_file = temp_dir.join("test_data.json");
    fs::write(&json_file, r#"{"name": "Test", "value": 42}"#).unwrap();

    let context = TemplateContext::new(temp_dir.clone(), false);
    let mut args = HashMap::new();
    args.insert(
        "path".to_string(),
        Value::String("test_data.json".to_string()),
    );

    let result = ReadJsonFile::new(context).call(&args).unwrap();
    let obj = result.as_object().unwrap();
    assert_eq!(obj.get("name").unwrap().as_str().unwrap(), "Test");
    assert_eq!(obj.get("value").unwrap().as_u64().unwrap(), 42);

    // Cleanup
    fs::remove_file(&json_file).unwrap();
}

#[test]
fn test_read_json_file_not_found() {
    let temp_dir = std::env::temp_dir();
    let context = TemplateContext::new(temp_dir, false);
    let mut args = HashMap::new();
    args.insert(
        "path".to_string(),
        Value::String("nonexistent.json".to_string()),
    );

    let result = ReadJsonFile::new(context).call(&args);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Failed to read file")
    );
}

#[test]
fn test_read_json_file_invalid_json() {
    // Create a temporary file with invalid JSON
    let temp_dir = std::env::temp_dir();
    let json_file = temp_dir.join("invalid.json");
    fs::write(&json_file, "not valid json").unwrap();

    let context = TemplateContext::new(temp_dir.clone(), false);
    let mut args = HashMap::new();
    args.insert(
        "path".to_string(),
        Value::String("invalid.json".to_string()),
    );

    let result = ReadJsonFile::new(context).call(&args);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Failed to parse JSON")
    );

    // Cleanup
    fs::remove_file(&json_file).unwrap();
}

#[test]
fn test_read_json_file_security_absolute_path() {
    let context = TemplateContext::new(PathBuf::from("."), false);
    let mut args = HashMap::new();
    args.insert("path".to_string(), Value::String("/etc/passwd".to_string()));

    let result = ReadJsonFile::new(context).call(&args);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Absolute paths are not allowed")
    );
}

#[test]
fn test_read_json_file_security_parent_traversal() {
    let context = TemplateContext::new(PathBuf::from("."), false);
    let mut args = HashMap::new();
    args.insert(
        "path".to_string(),
        Value::String("../../../etc/passwd".to_string()),
    );

    let result = ReadJsonFile::new(context).call(&args);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Parent directory (..) traversal")
    );
}

#[test]
fn test_read_json_file_trust_mode_allows_absolute_path() {
    // Create a temporary JSON file
    let temp_dir = std::env::temp_dir();
    let json_file = temp_dir.join("trust_test.json");
    fs::write(&json_file, r#"{"trusted": true}"#).unwrap();

    let context = TemplateContext::new(PathBuf::from("."), true);
    let mut args = HashMap::new();
    args.insert(
        "path".to_string(),
        Value::String(json_file.to_string_lossy().to_string()),
    );

    let result = ReadJsonFile::new(context).call(&args).unwrap();
    let obj = result.as_object().unwrap();
    assert!(obj.get("trusted").unwrap().as_bool().unwrap());

    // Cleanup
    fs::remove_file(&json_file).unwrap();
}

#[test]
fn test_read_json_file_missing_argument() {
    let context = TemplateContext::new(PathBuf::from("."), false);
    let args = HashMap::new();
    let result = ReadJsonFile::new(context).call(&args);
    assert!(result.is_err());
}

// ========== read_yaml_file tests ==========

#[test]
fn test_read_yaml_file_success() {
    // Create a temporary YAML file
    let temp_dir = std::env::temp_dir();
    let yaml_file = temp_dir.join("test_data.yaml");
    fs::write(&yaml_file, "name: Test\nvalue: 42").unwrap();

    let context = TemplateContext::new(temp_dir.clone(), false);
    let mut args = HashMap::new();
    args.insert(
        "path".to_string(),
        Value::String("test_data.yaml".to_string()),
    );

    let result = ReadYamlFile::new(context).call(&args).unwrap();
    let obj = result.as_object().unwrap();
    assert_eq!(obj.get("name").unwrap().as_str().unwrap(), "Test");
    assert_eq!(obj.get("value").unwrap().as_u64().unwrap(), 42);

    // Cleanup
    fs::remove_file(&yaml_file).unwrap();
}

#[test]
fn test_read_yaml_file_not_found() {
    let temp_dir = std::env::temp_dir();
    let context = TemplateContext::new(temp_dir, false);
    let mut args = HashMap::new();
    args.insert(
        "path".to_string(),
        Value::String("nonexistent.yaml".to_string()),
    );

    let result = ReadYamlFile::new(context).call(&args);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Failed to read file")
    );
}

#[test]
fn test_read_yaml_file_array() {
    // Create a temporary YAML file with array
    let temp_dir = std::env::temp_dir();
    let yaml_file = temp_dir.join("array.yaml");
    fs::write(&yaml_file, "- apple\n- banana\n- cherry").unwrap();

    let context = TemplateContext::new(temp_dir.clone(), false);
    let mut args = HashMap::new();
    args.insert("path".to_string(), Value::String("array.yaml".to_string()));

    let result = ReadYamlFile::new(context).call(&args).unwrap();
    let arr = result.as_array().unwrap();
    assert_eq!(arr.len(), 3);
    assert_eq!(arr[0].as_str().unwrap(), "apple");

    // Cleanup
    fs::remove_file(&yaml_file).unwrap();
}

#[test]
fn test_read_yaml_file_security_absolute_path() {
    let context = TemplateContext::new(PathBuf::from("."), false);
    let mut args = HashMap::new();
    args.insert(
        "path".to_string(),
        Value::String("/etc/config.yaml".to_string()),
    );

    let result = ReadYamlFile::new(context).call(&args);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Absolute paths are not allowed")
    );
}

#[test]
fn test_read_yaml_file_missing_argument() {
    let context = TemplateContext::new(PathBuf::from("."), false);
    let args = HashMap::new();
    let result = ReadYamlFile::new(context).call(&args);
    assert!(result.is_err());
}

// ========== read_toml_file tests ==========

#[test]
fn test_read_toml_file_success() {
    // Create a temporary TOML file
    let temp_dir = std::env::temp_dir();
    let toml_file = temp_dir.join("test_data.toml");
    fs::write(&toml_file, "name = \"Test\"\nvalue = 42").unwrap();

    let context = TemplateContext::new(temp_dir.clone(), false);
    let mut args = HashMap::new();
    args.insert(
        "path".to_string(),
        Value::String("test_data.toml".to_string()),
    );

    let result = ReadTomlFile::new(context).call(&args).unwrap();
    let obj = result.as_object().unwrap();
    assert_eq!(obj.get("name").unwrap().as_str().unwrap(), "Test");
    assert_eq!(obj.get("value").unwrap().as_u64().unwrap(), 42);

    // Cleanup
    fs::remove_file(&toml_file).unwrap();
}

#[test]
fn test_read_toml_file_not_found() {
    let temp_dir = std::env::temp_dir();
    let context = TemplateContext::new(temp_dir, false);
    let mut args = HashMap::new();
    args.insert(
        "path".to_string(),
        Value::String("nonexistent.toml".to_string()),
    );

    let result = ReadTomlFile::new(context).call(&args);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Failed to read file")
    );
}

#[test]
fn test_read_toml_file_table() {
    // Create a temporary TOML file with table
    let temp_dir = std::env::temp_dir();
    let toml_file = temp_dir.join("config.toml");
    fs::write(&toml_file, "[database]\nhost = \"localhost\"\nport = 5432").unwrap();

    let context = TemplateContext::new(temp_dir.clone(), false);
    let mut args = HashMap::new();
    args.insert("path".to_string(), Value::String("config.toml".to_string()));

    let result = ReadTomlFile::new(context).call(&args).unwrap();
    let obj = result.as_object().unwrap();
    let db = obj.get("database").unwrap().as_object().unwrap();
    assert_eq!(db.get("host").unwrap().as_str().unwrap(), "localhost");
    assert_eq!(db.get("port").unwrap().as_u64().unwrap(), 5432);

    // Cleanup
    fs::remove_file(&toml_file).unwrap();
}

#[test]
fn test_read_toml_file_invalid_toml() {
    // Create a temporary file with invalid TOML
    let temp_dir = std::env::temp_dir();
    let toml_file = temp_dir.join("invalid.toml");
    fs::write(&toml_file, "invalid = [missing quote").unwrap();

    let context = TemplateContext::new(temp_dir.clone(), false);
    let mut args = HashMap::new();
    args.insert(
        "path".to_string(),
        Value::String("invalid.toml".to_string()),
    );

    let result = ReadTomlFile::new(context).call(&args);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Failed to parse TOML")
    );

    // Cleanup
    fs::remove_file(&toml_file).unwrap();
}

#[test]
fn test_read_toml_file_security_parent_traversal() {
    let context = TemplateContext::new(PathBuf::from("."), false);
    let mut args = HashMap::new();
    args.insert(
        "path".to_string(),
        Value::String("../../config.toml".to_string()),
    );

    let result = ReadTomlFile::new(context).call(&args);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Parent directory (..) traversal")
    );
}

#[test]
fn test_read_toml_file_missing_argument() {
    let context = TemplateContext::new(PathBuf::from("."), false);
    let args = HashMap::new();
    let result = ReadTomlFile::new(context).call(&args);
    assert!(result.is_err());
}
