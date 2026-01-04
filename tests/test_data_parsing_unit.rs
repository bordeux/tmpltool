use minijinja::value::Kwargs;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tmpltool::TemplateContext;
use tmpltool::filter_functions::FilterFunction;
use tmpltool::filter_functions::serialization::{ParseJson, ParseToml, ParseYaml};
use tmpltool::functions::ContextFunction;
use tmpltool::functions::data_parsing::{ReadJsonFile, ReadTomlFile, ReadYamlFile};

// Helper to create kwargs for testing
fn create_kwargs(args: Vec<(&str, &str)>) -> Kwargs {
    Kwargs::from_iter(args.iter().map(|(k, v)| (*k, minijinja::Value::from(*v))))
}

// Helper to create path kwargs for file reading functions
fn path_kwargs(path: &str) -> Kwargs {
    create_kwargs(vec![("path", path)])
}

// Helper to create path kwargs from a String (for variable paths)
fn path_kwargs_string(path: String) -> Kwargs {
    Kwargs::from_iter([("path", minijinja::Value::from(path))])
}

// ========== parse_json tests ==========

#[test]
fn test_parse_json_simple_object() {
    let kwargs = create_kwargs(vec![("string", r#"{"name": "John", "age": 30}"#)]);

    let result = ParseJson::call_as_function(kwargs).unwrap();
    assert_eq!(result.get_attr("name").unwrap().as_str().unwrap(), "John");
    assert_eq!(result.get_attr("age").unwrap().as_usize(), Some(30));
}

#[test]
fn test_parse_json_array() {
    let kwargs = create_kwargs(vec![("string", r#"[1, 2, 3, 4, 5]"#)]);

    let result = ParseJson::call_as_function(kwargs).unwrap();
    assert_eq!(result.len(), Some(5));
    assert_eq!(
        result
            .get_item(&minijinja::Value::from(0))
            .unwrap()
            .as_usize(),
        Some(1)
    );
    assert_eq!(
        result
            .get_item(&minijinja::Value::from(4))
            .unwrap()
            .as_usize(),
        Some(5)
    );
}

#[test]
fn test_parse_json_nested() {
    let kwargs = create_kwargs(vec![(
        "string",
        r#"{"user": {"name": "Alice", "settings": {"theme": "dark"}}}"#,
    )]);

    let result = ParseJson::call_as_function(kwargs).unwrap();
    let user = result.get_attr("user").unwrap();
    let settings = user.get_attr("settings").unwrap();
    assert_eq!(
        settings.get_attr("theme").unwrap().as_str().unwrap(),
        "dark"
    );
}

#[test]
fn test_parse_json_invalid() {
    let kwargs = create_kwargs(vec![("string", r#"{"invalid": json"#)]);

    let result = ParseJson::call_as_function(kwargs);
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
    let kwargs = create_kwargs(vec![]);
    let result = ParseJson::call_as_function(kwargs);
    assert!(result.is_err());
}

// ========== parse_yaml tests ==========

#[test]
fn test_parse_yaml_simple_object() {
    let kwargs = create_kwargs(vec![("string", "name: John\nage: 30")]);

    let result = ParseYaml::call_as_function(kwargs).unwrap();
    assert_eq!(result.get_attr("name").unwrap().as_str().unwrap(), "John");
    assert_eq!(result.get_attr("age").unwrap().as_usize(), Some(30));
}

#[test]
fn test_parse_yaml_array() {
    let kwargs = create_kwargs(vec![("string", "- apple\n- banana\n- cherry")]);

    let result = ParseYaml::call_as_function(kwargs).unwrap();
    assert_eq!(result.len(), Some(3));
    assert_eq!(
        result
            .get_item(&minijinja::Value::from(0))
            .unwrap()
            .as_str()
            .unwrap(),
        "apple"
    );
    assert_eq!(
        result
            .get_item(&minijinja::Value::from(2))
            .unwrap()
            .as_str()
            .unwrap(),
        "cherry"
    );
}

#[test]
fn test_parse_yaml_nested() {
    let kwargs = create_kwargs(vec![(
        "string",
        "user:\n  name: Alice\n  settings:\n    theme: dark\n    notifications: true",
    )]);

    let result = ParseYaml::call_as_function(kwargs).unwrap();
    let user = result.get_attr("user").unwrap();
    let settings = user.get_attr("settings").unwrap();
    assert_eq!(
        settings.get_attr("theme").unwrap().as_str().unwrap(),
        "dark"
    );
    assert!(settings.get_attr("notifications").unwrap().is_true());
}

#[test]
fn test_parse_yaml_invalid() {
    let kwargs = create_kwargs(vec![(
        "string",
        "invalid:\n  - yaml\n  missing: indentation",
    )]);

    let result = ParseYaml::call_as_function(kwargs);
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
    let kwargs = create_kwargs(vec![]);
    let result = ParseYaml::call_as_function(kwargs);
    assert!(result.is_err());
}

// ========== parse_toml tests ==========

#[test]
fn test_parse_toml_simple() {
    let kwargs = create_kwargs(vec![("string", "name = \"John\"\nage = 30")]);

    let result = ParseToml::call_as_function(kwargs).unwrap();
    assert_eq!(result.get_attr("name").unwrap().as_str().unwrap(), "John");
    assert_eq!(result.get_attr("age").unwrap().as_usize(), Some(30));
}

#[test]
fn test_parse_toml_array() {
    let kwargs = create_kwargs(vec![("string", "colors = [\"red\", \"green\", \"blue\"]")]);

    let result = ParseToml::call_as_function(kwargs).unwrap();
    let colors = result.get_attr("colors").unwrap();
    assert_eq!(colors.len(), Some(3));
    assert_eq!(
        colors
            .get_item(&minijinja::Value::from(0))
            .unwrap()
            .as_str()
            .unwrap(),
        "red"
    );
    assert_eq!(
        colors
            .get_item(&minijinja::Value::from(2))
            .unwrap()
            .as_str()
            .unwrap(),
        "blue"
    );
}

#[test]
fn test_parse_toml_table() {
    let kwargs = create_kwargs(vec![(
        "string",
        "[database]\nhost = \"localhost\"\nport = 5432\nenabled = true",
    )]);

    let result = ParseToml::call_as_function(kwargs).unwrap();
    let db = result.get_attr("database").unwrap();
    assert_eq!(db.get_attr("host").unwrap().as_str().unwrap(), "localhost");
    assert_eq!(db.get_attr("port").unwrap().as_usize(), Some(5432));
    assert!(db.get_attr("enabled").unwrap().is_true());
}

#[test]
fn test_parse_toml_invalid() {
    let kwargs = create_kwargs(vec![("string", "invalid toml = [missing quote")]);

    let result = ParseToml::call_as_function(kwargs);
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
    let kwargs = create_kwargs(vec![]);
    let result = ParseToml::call_as_function(kwargs);
    assert!(result.is_err());
}

// ========== read_json_file tests ==========

#[test]
fn test_read_json_file_success() {
    // Create a temporary JSON file
    let temp_dir = std::env::temp_dir();
    let json_file = temp_dir.join("test_data.json");
    fs::write(&json_file, r#"{"name": "Test", "value": 42}"#).unwrap();

    let context = Arc::new(TemplateContext::new(temp_dir.clone(), false));

    let result = ReadJsonFile::call(context.clone(), path_kwargs("test_data.json")).unwrap();
    assert_eq!(result.get_attr("name").unwrap().as_str().unwrap(), "Test");
    assert_eq!(result.get_attr("value").unwrap().as_usize(), Some(42));

    // Cleanup
    fs::remove_file(&json_file).unwrap();
}

#[test]
fn test_read_json_file_not_found() {
    let temp_dir = std::env::temp_dir();
    let context = Arc::new(TemplateContext::new(temp_dir, false));

    let result = ReadJsonFile::call(context.clone(), path_kwargs("nonexistent.json"));
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

    let context = Arc::new(TemplateContext::new(temp_dir.clone(), false));

    let result = ReadJsonFile::call(context.clone(), path_kwargs("invalid.json"));
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
    let context = Arc::new(TemplateContext::new(PathBuf::from("."), false));

    let result = ReadJsonFile::call(context.clone(), path_kwargs("/etc/passwd"));
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
    let context = Arc::new(TemplateContext::new(PathBuf::from("."), false));

    let result = ReadJsonFile::call(context.clone(), path_kwargs("../../../etc/passwd"));
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

    let context = Arc::new(TemplateContext::new(PathBuf::from("."), true));

    let result = ReadJsonFile::call(
        context.clone(),
        path_kwargs_string(json_file.to_string_lossy().to_string()),
    )
    .unwrap();
    assert!(result.get_attr("trusted").unwrap().is_true());

    // Cleanup
    fs::remove_file(&json_file).unwrap();
}

// ========== read_yaml_file tests ==========

#[test]
fn test_read_yaml_file_success() {
    // Create a temporary YAML file
    let temp_dir = std::env::temp_dir();
    let yaml_file = temp_dir.join("test_data.yaml");
    fs::write(&yaml_file, "name: Test\nvalue: 42").unwrap();

    let context = Arc::new(TemplateContext::new(temp_dir.clone(), false));

    let result = ReadYamlFile::call(context.clone(), path_kwargs("test_data.yaml")).unwrap();
    assert_eq!(result.get_attr("name").unwrap().as_str().unwrap(), "Test");
    assert_eq!(result.get_attr("value").unwrap().as_usize(), Some(42));

    // Cleanup
    fs::remove_file(&yaml_file).unwrap();
}

#[test]
fn test_read_yaml_file_not_found() {
    let temp_dir = std::env::temp_dir();
    let context = Arc::new(TemplateContext::new(temp_dir, false));

    let result = ReadYamlFile::call(context.clone(), path_kwargs("nonexistent.yaml"));
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

    let context = Arc::new(TemplateContext::new(temp_dir.clone(), false));

    let result = ReadYamlFile::call(context.clone(), path_kwargs("array.yaml")).unwrap();
    assert_eq!(result.len(), Some(3));
    assert_eq!(
        result
            .get_item(&minijinja::Value::from(0))
            .unwrap()
            .as_str()
            .unwrap(),
        "apple"
    );

    // Cleanup
    fs::remove_file(&yaml_file).unwrap();
}

#[test]
fn test_read_yaml_file_security_absolute_path() {
    let context = Arc::new(TemplateContext::new(PathBuf::from("."), false));

    let result = ReadYamlFile::call(context.clone(), path_kwargs("/etc/config.yaml"));
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Absolute paths are not allowed")
    );
}

// ========== read_toml_file tests ==========

#[test]
fn test_read_toml_file_success() {
    // Create a temporary TOML file
    let temp_dir = std::env::temp_dir();
    let toml_file = temp_dir.join("test_data.toml");
    fs::write(&toml_file, "name = \"Test\"\nvalue = 42").unwrap();

    let context = Arc::new(TemplateContext::new(temp_dir.clone(), false));

    let result = ReadTomlFile::call(context.clone(), path_kwargs("test_data.toml")).unwrap();
    assert_eq!(result.get_attr("name").unwrap().as_str().unwrap(), "Test");
    assert_eq!(result.get_attr("value").unwrap().as_usize(), Some(42));

    // Cleanup
    fs::remove_file(&toml_file).unwrap();
}

#[test]
fn test_read_toml_file_not_found() {
    let temp_dir = std::env::temp_dir();
    let context = Arc::new(TemplateContext::new(temp_dir, false));

    let result = ReadTomlFile::call(context.clone(), path_kwargs("nonexistent.toml"));
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

    let context = Arc::new(TemplateContext::new(temp_dir.clone(), false));

    let result = ReadTomlFile::call(context.clone(), path_kwargs("config.toml")).unwrap();
    let db = result.get_attr("database").unwrap();
    assert_eq!(db.get_attr("host").unwrap().as_str().unwrap(), "localhost");
    assert_eq!(db.get_attr("port").unwrap().as_usize(), Some(5432));

    // Cleanup
    fs::remove_file(&toml_file).unwrap();
}

#[test]
fn test_read_toml_file_invalid_toml() {
    // Create a temporary file with invalid TOML
    let temp_dir = std::env::temp_dir();
    let toml_file = temp_dir.join("invalid.toml");
    fs::write(&toml_file, "invalid = [missing quote").unwrap();

    let context = Arc::new(TemplateContext::new(temp_dir.clone(), false));

    let result = ReadTomlFile::call(context.clone(), path_kwargs("invalid.toml"));
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
    let context = Arc::new(TemplateContext::new(PathBuf::from("."), false));

    let result = ReadTomlFile::call(context.clone(), path_kwargs("../../config.toml"));
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Parent directory (..) traversal")
    );
}

// ========== Additional parse_json edge case tests ==========

#[test]
fn test_parse_json_null() {
    let kwargs = create_kwargs(vec![("string", "null")]);
    let result = ParseJson::call_as_function(kwargs).unwrap();
    assert!(result.is_none());
}

#[test]
fn test_parse_json_boolean_true() {
    let kwargs = create_kwargs(vec![("string", "true")]);
    let result = ParseJson::call_as_function(kwargs).unwrap();
    assert!(result.is_true());
}

#[test]
fn test_parse_json_boolean_false() {
    let kwargs = create_kwargs(vec![("string", "false")]);
    let result = ParseJson::call_as_function(kwargs).unwrap();
    assert!(!result.is_true());
}

#[test]
fn test_parse_json_float() {
    let kwargs = create_kwargs(vec![("string", "1.23456")]);
    let result = ParseJson::call_as_function(kwargs).unwrap();
    let json_val: serde_json::Value = serde_json::to_value(&result).unwrap();
    let f = json_val.as_f64().unwrap();
    assert!((f - 1.23456).abs() < 0.0001);
}

#[test]
fn test_parse_json_negative_number() {
    let kwargs = create_kwargs(vec![("string", "-42")]);
    let result = ParseJson::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_i64(), Some(-42));
}

#[test]
fn test_parse_json_large_number() {
    let kwargs = create_kwargs(vec![("string", "9223372036854775807")]);
    let result = ParseJson::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_i64(), Some(9223372036854775807i64));
}

#[test]
fn test_parse_json_empty_object() {
    let kwargs = create_kwargs(vec![("string", "{}")]);
    let result = ParseJson::call_as_function(kwargs).unwrap();
    assert!(result.is_undefined() || result.len() == Some(0));
}

#[test]
fn test_parse_json_empty_array() {
    let kwargs = create_kwargs(vec![("string", "[]")]);
    let result = ParseJson::call_as_function(kwargs).unwrap();
    assert_eq!(result.len(), Some(0));
}

#[test]
fn test_parse_json_unicode() {
    let kwargs = create_kwargs(vec![("string", r#"{"emoji": "🎉", "chinese": "你好"}"#)]);
    let result = ParseJson::call_as_function(kwargs).unwrap();
    assert_eq!(result.get_attr("emoji").unwrap().as_str().unwrap(), "🎉");
    assert_eq!(
        result.get_attr("chinese").unwrap().as_str().unwrap(),
        "你好"
    );
}

#[test]
fn test_parse_json_special_chars() {
    let kwargs = create_kwargs(vec![("string", r#"{"text": "line1\nline2\ttab"}"#)]);
    let result = ParseJson::call_as_function(kwargs).unwrap();
    let text_val = result.get_attr("text").unwrap();
    let text = text_val.as_str().unwrap();
    assert!(text.contains('\n'));
    assert!(text.contains('\t'));
}

#[test]
fn test_parse_json_deeply_nested() {
    let kwargs = create_kwargs(vec![(
        "string",
        r#"{"a": {"b": {"c": {"d": {"e": "deep"}}}}}"#,
    )]);
    let result = ParseJson::call_as_function(kwargs).unwrap();
    let deep = result
        .get_attr("a")
        .unwrap()
        .get_attr("b")
        .unwrap()
        .get_attr("c")
        .unwrap()
        .get_attr("d")
        .unwrap()
        .get_attr("e")
        .unwrap();
    assert_eq!(deep.as_str().unwrap(), "deep");
}

#[test]
fn test_parse_json_mixed_array() {
    let kwargs = create_kwargs(vec![("string", r#"[1, "two", true, null, 3.14]"#)]);
    let result = ParseJson::call_as_function(kwargs).unwrap();
    assert_eq!(result.len(), Some(5));
    assert_eq!(
        result
            .get_item(&minijinja::Value::from(0))
            .unwrap()
            .as_usize(),
        Some(1)
    );
    assert_eq!(
        result
            .get_item(&minijinja::Value::from(1))
            .unwrap()
            .as_str()
            .unwrap(),
        "two"
    );
    assert!(
        result
            .get_item(&minijinja::Value::from(2))
            .unwrap()
            .is_true()
    );
    assert!(
        result
            .get_item(&minijinja::Value::from(3))
            .unwrap()
            .is_none()
    );
}

#[test]
fn test_parse_json_empty_string_value() {
    let kwargs = create_kwargs(vec![("string", r#"{"empty": ""}"#)]);
    let result = ParseJson::call_as_function(kwargs).unwrap();
    assert_eq!(result.get_attr("empty").unwrap().as_str().unwrap(), "");
}

#[test]
fn test_parse_json_string_primitive() {
    let kwargs = create_kwargs(vec![("string", r#""hello world""#)]);
    let result = ParseJson::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "hello world");
}

#[test]
fn test_parse_json_trailing_comma_error() {
    let kwargs = create_kwargs(vec![("string", r#"{"key": "value",}"#)]);
    let result = ParseJson::call_as_function(kwargs);
    assert!(result.is_err());
}

#[test]
fn test_parse_json_unquoted_key_error() {
    let kwargs = create_kwargs(vec![("string", r#"{key: "value"}"#)]);
    let result = ParseJson::call_as_function(kwargs);
    assert!(result.is_err());
}

// ========== Additional parse_yaml edge case tests ==========

#[test]
fn test_parse_yaml_null_value() {
    let kwargs = create_kwargs(vec![("string", "value: null")]);
    let result = ParseYaml::call_as_function(kwargs).unwrap();
    assert!(result.get_attr("value").unwrap().is_none());
}

#[test]
fn test_parse_yaml_tilde_null() {
    let kwargs = create_kwargs(vec![("string", "value: ~")]);
    let result = ParseYaml::call_as_function(kwargs).unwrap();
    assert!(result.get_attr("value").unwrap().is_none());
}

#[test]
fn test_parse_yaml_boolean_variants() {
    // YAML 1.2 only treats true/false as booleans (not yes/no/on/off)
    let kwargs = create_kwargs(vec![("string", "enabled: true\ndisabled: false")]);
    let result = ParseYaml::call_as_function(kwargs).unwrap();
    assert!(result.get_attr("enabled").unwrap().is_true());
    assert!(!result.get_attr("disabled").unwrap().is_true());
}

#[test]
fn test_parse_yaml_float() {
    let kwargs = create_kwargs(vec![("string", "value: 1.23456")]);
    let result = ParseYaml::call_as_function(kwargs).unwrap();
    let val = result.get_attr("value").unwrap();
    let json_val: serde_json::Value = serde_json::to_value(&val).unwrap();
    let f = json_val.as_f64().unwrap();
    assert!((f - 1.23456).abs() < 0.0001);
}

#[test]
fn test_parse_yaml_negative_float() {
    let kwargs = create_kwargs(vec![("string", "temp: -273.15")]);
    let result = ParseYaml::call_as_function(kwargs).unwrap();
    let temp_val = result.get_attr("temp").unwrap();
    let json_val: serde_json::Value = serde_json::to_value(&temp_val).unwrap();
    let f = json_val.as_f64().unwrap();
    assert!((f - (-273.15)).abs() < 0.0001);
}

#[test]
fn test_parse_yaml_scientific_notation() {
    let kwargs = create_kwargs(vec![("string", "large: 1.0e10")]);
    let result = ParseYaml::call_as_function(kwargs).unwrap();
    let large_val = result.get_attr("large").unwrap();
    let json_val: serde_json::Value = serde_json::to_value(&large_val).unwrap();
    let f = json_val.as_f64().unwrap();
    assert!((f - 1.0e10).abs() < 1.0);
}

#[test]
fn test_parse_yaml_number_as_key() {
    let kwargs = create_kwargs(vec![("string", "123: numeric key")]);
    let result = ParseYaml::call_as_function(kwargs).unwrap();
    // Access via string representation of number
    assert_eq!(
        result.get_attr("123").unwrap().as_str().unwrap(),
        "numeric key"
    );
}

#[test]
fn test_parse_yaml_boolean_as_key() {
    let kwargs = create_kwargs(vec![("string", "true: boolean key")]);
    let result = ParseYaml::call_as_function(kwargs).unwrap();
    assert_eq!(
        result.get_attr("true").unwrap().as_str().unwrap(),
        "boolean key"
    );
}

#[test]
fn test_parse_yaml_multiline_string() {
    let kwargs = create_kwargs(vec![("string", "text: |\n  line1\n  line2\n  line3")]);
    let result = ParseYaml::call_as_function(kwargs).unwrap();
    let text_val = result.get_attr("text").unwrap();
    let text = text_val.as_str().unwrap();
    assert!(text.contains("line1"));
    assert!(text.contains("line2"));
}

#[test]
fn test_parse_yaml_folded_string() {
    let kwargs = create_kwargs(vec![("string", "text: >\n  folded\n  text")]);
    let result = ParseYaml::call_as_function(kwargs).unwrap();
    let text_val = result.get_attr("text").unwrap();
    let text = text_val.as_str().unwrap();
    assert!(text.contains("folded"));
}

#[test]
fn test_parse_yaml_unicode() {
    let kwargs = create_kwargs(vec![("string", "emoji: 🚀\njapanese: 日本語")]);
    let result = ParseYaml::call_as_function(kwargs).unwrap();
    assert_eq!(result.get_attr("emoji").unwrap().as_str().unwrap(), "🚀");
    assert_eq!(
        result.get_attr("japanese").unwrap().as_str().unwrap(),
        "日本語"
    );
}

#[test]
fn test_parse_yaml_empty_document() {
    let kwargs = create_kwargs(vec![("string", "")]);
    let result = ParseYaml::call_as_function(kwargs).unwrap();
    assert!(result.is_none());
}

#[test]
fn test_parse_yaml_only_null() {
    let kwargs = create_kwargs(vec![("string", "~")]);
    let result = ParseYaml::call_as_function(kwargs).unwrap();
    assert!(result.is_none());
}

#[test]
fn test_parse_yaml_inline_array() {
    let kwargs = create_kwargs(vec![("string", "items: [1, 2, 3]")]);
    let result = ParseYaml::call_as_function(kwargs).unwrap();
    let items = result.get_attr("items").unwrap();
    assert_eq!(items.len(), Some(3));
}

#[test]
fn test_parse_yaml_inline_object() {
    let kwargs = create_kwargs(vec![("string", "person: {name: John, age: 30}")]);
    let result = ParseYaml::call_as_function(kwargs).unwrap();
    let person = result.get_attr("person").unwrap();
    assert_eq!(person.get_attr("name").unwrap().as_str().unwrap(), "John");
}

#[test]
fn test_parse_yaml_anchor_alias() {
    // Test simple anchor and alias reference
    let kwargs = create_kwargs(vec![(
        "string",
        "default_value: &default 100\nfirst: *default\nsecond: *default",
    )]);
    let result = ParseYaml::call_as_function(kwargs).unwrap();
    assert_eq!(
        result.get_attr("default_value").unwrap().as_usize(),
        Some(100)
    );
    assert_eq!(result.get_attr("first").unwrap().as_usize(), Some(100));
    assert_eq!(result.get_attr("second").unwrap().as_usize(), Some(100));
}

#[test]
fn test_parse_yaml_tagged_value() {
    // Tagged values should be converted to their inner value
    let kwargs = create_kwargs(vec![("string", "date: !custom 2024-01-01")]);
    let result = ParseYaml::call_as_function(kwargs).unwrap();
    assert!(result.get_attr("date").is_ok());
}

#[test]
fn test_parse_yaml_large_integer() {
    let kwargs = create_kwargs(vec![("string", "big: 9223372036854775807")]);
    let result = ParseYaml::call_as_function(kwargs).unwrap();
    assert_eq!(
        result.get_attr("big").unwrap().as_i64(),
        Some(9223372036854775807i64)
    );
}

#[test]
fn test_parse_yaml_unsigned_large() {
    let kwargs = create_kwargs(vec![("string", "unsigned: 18446744073709551615")]);
    let result = ParseYaml::call_as_function(kwargs).unwrap();
    // Large unsigned values are handled
    assert!(result.get_attr("unsigned").is_ok());
}

// ========== Additional parse_toml edge case tests ==========

#[test]
fn test_parse_toml_float() {
    let kwargs = create_kwargs(vec![("string", "value = 1.23456")]);
    let result = ParseToml::call_as_function(kwargs).unwrap();
    let val = result.get_attr("value").unwrap();
    let json_val: serde_json::Value = serde_json::to_value(&val).unwrap();
    let f = json_val.as_f64().unwrap();
    assert!((f - 1.23456).abs() < 0.0001);
}

#[test]
fn test_parse_toml_negative_float() {
    let kwargs = create_kwargs(vec![("string", "temp = -273.15")]);
    let result = ParseToml::call_as_function(kwargs).unwrap();
    let temp_val = result.get_attr("temp").unwrap();
    let json_val: serde_json::Value = serde_json::to_value(&temp_val).unwrap();
    let f = json_val.as_f64().unwrap();
    assert!((f - (-273.15)).abs() < 0.0001);
}

#[test]
fn test_parse_toml_scientific_notation() {
    let kwargs = create_kwargs(vec![("string", "large = 1.0e10")]);
    let result = ParseToml::call_as_function(kwargs).unwrap();
    let large_val = result.get_attr("large").unwrap();
    let json_val: serde_json::Value = serde_json::to_value(&large_val).unwrap();
    let f = json_val.as_f64().unwrap();
    assert!((f - 1.0e10).abs() < 1.0);
}

#[test]
fn test_parse_toml_datetime() {
    let kwargs = create_kwargs(vec![("string", "date = 2024-01-15T10:30:00Z")]);
    let result = ParseToml::call_as_function(kwargs).unwrap();
    let date_val = result.get_attr("date").unwrap();
    let date = date_val.as_str().unwrap();
    assert!(date.contains("2024"));
    assert!(date.contains("01"));
    assert!(date.contains("15"));
}

#[test]
fn test_parse_toml_local_date() {
    let kwargs = create_kwargs(vec![("string", "date = 2024-01-15")]);
    let result = ParseToml::call_as_function(kwargs).unwrap();
    let date_val = result.get_attr("date").unwrap();
    let date = date_val.as_str().unwrap();
    assert!(date.contains("2024-01-15"));
}

#[test]
fn test_parse_toml_local_time() {
    let kwargs = create_kwargs(vec![("string", "time = 10:30:00")]);
    let result = ParseToml::call_as_function(kwargs).unwrap();
    let time_val = result.get_attr("time").unwrap();
    let time = time_val.as_str().unwrap();
    assert!(time.contains("10:30:00"));
}

#[test]
fn test_parse_toml_nested_tables() {
    let kwargs = create_kwargs(vec![(
        "string",
        "[server]\nhost = \"localhost\"\n\n[server.ssl]\nenabled = true\nport = 443",
    )]);
    let result = ParseToml::call_as_function(kwargs).unwrap();
    let server = result.get_attr("server").unwrap();
    assert_eq!(
        server.get_attr("host").unwrap().as_str().unwrap(),
        "localhost"
    );
    let ssl = server.get_attr("ssl").unwrap();
    assert!(ssl.get_attr("enabled").unwrap().is_true());
    assert_eq!(ssl.get_attr("port").unwrap().as_usize(), Some(443));
}

#[test]
fn test_parse_toml_array_of_tables() {
    let kwargs = create_kwargs(vec![(
        "string",
        "[[products]]\nname = \"Apple\"\nprice = 1.0\n\n[[products]]\nname = \"Banana\"\nprice = 0.5",
    )]);
    let result = ParseToml::call_as_function(kwargs).unwrap();
    let products = result.get_attr("products").unwrap();
    assert_eq!(products.len(), Some(2));
    assert_eq!(
        products
            .get_item(&minijinja::Value::from(0))
            .unwrap()
            .get_attr("name")
            .unwrap()
            .as_str()
            .unwrap(),
        "Apple"
    );
}

#[test]
fn test_parse_toml_inline_table() {
    let kwargs = create_kwargs(vec![("string", "point = { x = 1, y = 2 }")]);
    let result = ParseToml::call_as_function(kwargs).unwrap();
    let point = result.get_attr("point").unwrap();
    assert_eq!(point.get_attr("x").unwrap().as_usize(), Some(1));
    assert_eq!(point.get_attr("y").unwrap().as_usize(), Some(2));
}

#[test]
fn test_parse_toml_multiline_string() {
    let kwargs = create_kwargs(vec![(
        "string",
        r#"text = """
line1
line2
line3""""#,
    )]);
    let result = ParseToml::call_as_function(kwargs).unwrap();
    let text_val = result.get_attr("text").unwrap();
    let text = text_val.as_str().unwrap();
    assert!(text.contains("line1"));
    assert!(text.contains("line2"));
}

#[test]
fn test_parse_toml_literal_string() {
    let kwargs = create_kwargs(vec![("string", r"path = 'C:\Users\name'")]);
    let result = ParseToml::call_as_function(kwargs).unwrap();
    let path_val = result.get_attr("path").unwrap();
    let path = path_val.as_str().unwrap();
    assert!(path.contains("C:\\Users"));
}

#[test]
fn test_parse_toml_boolean_true() {
    let kwargs = create_kwargs(vec![("string", "enabled = true")]);
    let result = ParseToml::call_as_function(kwargs).unwrap();
    assert!(result.get_attr("enabled").unwrap().is_true());
}

#[test]
fn test_parse_toml_boolean_false() {
    let kwargs = create_kwargs(vec![("string", "enabled = false")]);
    let result = ParseToml::call_as_function(kwargs).unwrap();
    assert!(!result.get_attr("enabled").unwrap().is_true());
}

#[test]
fn test_parse_toml_integer_negative() {
    let kwargs = create_kwargs(vec![("string", "value = -42")]);
    let result = ParseToml::call_as_function(kwargs).unwrap();
    assert_eq!(result.get_attr("value").unwrap().as_i64(), Some(-42));
}

#[test]
fn test_parse_toml_integer_large() {
    let kwargs = create_kwargs(vec![("string", "big = 9223372036854775807")]);
    let result = ParseToml::call_as_function(kwargs).unwrap();
    assert_eq!(
        result.get_attr("big").unwrap().as_i64(),
        Some(9223372036854775807i64)
    );
}

#[test]
fn test_parse_toml_mixed_array() {
    // TOML requires homogeneous arrays, but can have different tables
    let kwargs = create_kwargs(vec![(
        "string",
        "numbers = [1, 2, 3]\nstrings = [\"a\", \"b\"]",
    )]);
    let result = ParseToml::call_as_function(kwargs).unwrap();
    let numbers = result.get_attr("numbers").unwrap();
    let strings = result.get_attr("strings").unwrap();
    assert_eq!(numbers.len(), Some(3));
    assert_eq!(strings.len(), Some(2));
}

#[test]
fn test_parse_toml_empty_table() {
    let kwargs = create_kwargs(vec![("string", "[empty]")]);
    let result = ParseToml::call_as_function(kwargs).unwrap();
    assert!(result.get_attr("empty").is_ok());
}

#[test]
fn test_parse_toml_unicode() {
    let kwargs = create_kwargs(vec![("string", "emoji = \"🎉\"\nchinese = \"你好\"")]);
    let result = ParseToml::call_as_function(kwargs).unwrap();
    assert_eq!(result.get_attr("emoji").unwrap().as_str().unwrap(), "🎉");
    assert_eq!(
        result.get_attr("chinese").unwrap().as_str().unwrap(),
        "你好"
    );
}

// ========== Additional file reading edge case tests ==========

#[test]
fn test_read_yaml_file_security_parent_traversal() {
    let context = Arc::new(TemplateContext::new(PathBuf::from("."), false));

    let result = ReadYamlFile::call(context.clone(), path_kwargs("../../../config.yaml"));
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Parent directory (..) traversal")
    );
}

#[test]
fn test_read_toml_file_security_absolute_path() {
    let context = Arc::new(TemplateContext::new(PathBuf::from("."), false));

    let result = ReadTomlFile::call(context.clone(), path_kwargs("/etc/config.toml"));
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Absolute paths are not allowed")
    );
}

#[test]
fn test_read_yaml_file_trust_mode_allows_absolute() {
    // Create a temporary YAML file
    let temp_dir = std::env::temp_dir();
    let yaml_file = temp_dir.join("trust_test.yaml");
    fs::write(&yaml_file, "trusted: true").unwrap();

    let context = Arc::new(TemplateContext::new(PathBuf::from("."), true));

    let result = ReadYamlFile::call(
        context.clone(),
        path_kwargs_string(yaml_file.to_string_lossy().to_string()),
    )
    .unwrap();
    assert!(result.get_attr("trusted").unwrap().is_true());

    // Cleanup
    fs::remove_file(&yaml_file).unwrap();
}

#[test]
fn test_read_toml_file_trust_mode_allows_absolute() {
    // Create a temporary TOML file
    let temp_dir = std::env::temp_dir();
    let toml_file = temp_dir.join("trust_test.toml");
    fs::write(&toml_file, "trusted = true").unwrap();

    let context = Arc::new(TemplateContext::new(PathBuf::from("."), true));

    let result = ReadTomlFile::call(
        context.clone(),
        path_kwargs_string(toml_file.to_string_lossy().to_string()),
    )
    .unwrap();
    assert!(result.get_attr("trusted").unwrap().is_true());

    // Cleanup
    fs::remove_file(&toml_file).unwrap();
}

#[test]
fn test_read_yaml_file_invalid_yaml() {
    // Create a temporary file with invalid YAML
    let temp_dir = std::env::temp_dir();
    let yaml_file = temp_dir.join("invalid_yaml.yaml");
    fs::write(&yaml_file, "invalid:\n  - yaml\n  missing: indentation").unwrap();

    let context = Arc::new(TemplateContext::new(temp_dir.clone(), false));

    let result = ReadYamlFile::call(context.clone(), path_kwargs("invalid_yaml.yaml"));
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Failed to parse YAML")
    );

    // Cleanup
    fs::remove_file(&yaml_file).unwrap();
}

#[test]
fn test_read_json_file_complex_nested() {
    let temp_dir = std::env::temp_dir();
    let json_file = temp_dir.join("complex_nested.json");
    fs::write(&json_file, r#"{"a": {"b": {"c": [1, 2, {"d": "deep"}]}}}"#).unwrap();

    let context = Arc::new(TemplateContext::new(temp_dir.clone(), false));

    let result = ReadJsonFile::call(context.clone(), path_kwargs("complex_nested.json")).unwrap();
    let a = result.get_attr("a").unwrap();
    let b = a.get_attr("b").unwrap();
    let c = b.get_attr("c").unwrap();
    assert_eq!(c.len(), Some(3));

    // Cleanup
    fs::remove_file(&json_file).unwrap();
}

#[test]
fn test_read_yaml_file_with_nulls() {
    let temp_dir = std::env::temp_dir();
    let yaml_file = temp_dir.join("with_nulls.yaml");
    fs::write(&yaml_file, "value: null\nempty: ~").unwrap();

    let context = Arc::new(TemplateContext::new(temp_dir.clone(), false));

    let result = ReadYamlFile::call(context.clone(), path_kwargs("with_nulls.yaml")).unwrap();
    assert!(result.get_attr("value").unwrap().is_none());
    assert!(result.get_attr("empty").unwrap().is_none());

    // Cleanup
    fs::remove_file(&yaml_file).unwrap();
}

#[test]
fn test_read_yaml_file_with_floats() {
    let temp_dir = std::env::temp_dir();
    let yaml_file = temp_dir.join("with_floats.yaml");
    fs::write(&yaml_file, "value: 1.23456\nother: 9.87654").unwrap();

    let context = Arc::new(TemplateContext::new(temp_dir.clone(), false));

    let result = ReadYamlFile::call(context.clone(), path_kwargs("with_floats.yaml")).unwrap();
    let val = result.get_attr("value").unwrap();
    let json_val: serde_json::Value = serde_json::to_value(&val).unwrap();
    let f = json_val.as_f64().unwrap();
    assert!((f - 1.23456).abs() < 0.0001);

    // Cleanup
    fs::remove_file(&yaml_file).unwrap();
}

#[test]
fn test_read_toml_file_with_datetime() {
    let temp_dir = std::env::temp_dir();
    let toml_file = temp_dir.join("with_datetime.toml");
    fs::write(&toml_file, "created = 2024-01-15T10:30:00Z").unwrap();

    let context = Arc::new(TemplateContext::new(temp_dir.clone(), false));

    let result = ReadTomlFile::call(context.clone(), path_kwargs("with_datetime.toml")).unwrap();
    let created_val = result.get_attr("created").unwrap();
    let created = created_val.as_str().unwrap();
    assert!(created.contains("2024"));

    // Cleanup
    fs::remove_file(&toml_file).unwrap();
}

#[test]
fn test_read_toml_file_with_array_of_tables() {
    let temp_dir = std::env::temp_dir();
    let toml_file = temp_dir.join("array_of_tables.toml");
    fs::write(
        &toml_file,
        "[[items]]\nname = \"first\"\n\n[[items]]\nname = \"second\"",
    )
    .unwrap();

    let context = Arc::new(TemplateContext::new(temp_dir.clone(), false));

    let result = ReadTomlFile::call(context.clone(), path_kwargs("array_of_tables.toml")).unwrap();
    let items = result.get_attr("items").unwrap();
    assert_eq!(items.len(), Some(2));

    // Cleanup
    fs::remove_file(&toml_file).unwrap();
}

#[test]
fn test_read_yaml_file_with_number_key() {
    let temp_dir = std::env::temp_dir();
    let yaml_file = temp_dir.join("number_key.yaml");
    fs::write(&yaml_file, "123: numeric key value").unwrap();

    let context = Arc::new(TemplateContext::new(temp_dir.clone(), false));

    let result = ReadYamlFile::call(context.clone(), path_kwargs("number_key.yaml")).unwrap();
    assert_eq!(
        result.get_attr("123").unwrap().as_str().unwrap(),
        "numeric key value"
    );

    // Cleanup
    fs::remove_file(&yaml_file).unwrap();
}

#[test]
fn test_read_yaml_file_with_boolean_key() {
    let temp_dir = std::env::temp_dir();
    let yaml_file = temp_dir.join("bool_key.yaml");
    fs::write(&yaml_file, "true: boolean key value").unwrap();

    let context = Arc::new(TemplateContext::new(temp_dir.clone(), false));

    let result = ReadYamlFile::call(context.clone(), path_kwargs("bool_key.yaml")).unwrap();
    assert_eq!(
        result.get_attr("true").unwrap().as_str().unwrap(),
        "boolean key value"
    );

    // Cleanup
    fs::remove_file(&yaml_file).unwrap();
}

#[test]
fn test_read_yaml_file_with_tagged_value() {
    let temp_dir = std::env::temp_dir();
    let yaml_file = temp_dir.join("tagged.yaml");
    fs::write(&yaml_file, "custom: !tag value").unwrap();

    let context = Arc::new(TemplateContext::new(temp_dir.clone(), false));

    let result = ReadYamlFile::call(context.clone(), path_kwargs("tagged.yaml")).unwrap();
    // Tagged value should be converted to its inner value
    assert!(result.get_attr("custom").is_ok());

    // Cleanup
    fs::remove_file(&yaml_file).unwrap();
}

#[test]
fn test_read_yaml_file_with_anchor_alias() {
    let temp_dir = std::env::temp_dir();
    let yaml_file = temp_dir.join("anchor_alias.yaml");
    fs::write(
        &yaml_file,
        "default_value: &default 100\nfirst: *default\nsecond: *default",
    )
    .unwrap();

    let context = Arc::new(TemplateContext::new(temp_dir.clone(), false));

    let result = ReadYamlFile::call(context.clone(), path_kwargs("anchor_alias.yaml")).unwrap();
    assert_eq!(
        result.get_attr("default_value").unwrap().as_usize(),
        Some(100)
    );
    assert_eq!(result.get_attr("first").unwrap().as_usize(), Some(100));
    assert_eq!(result.get_attr("second").unwrap().as_usize(), Some(100));

    // Cleanup
    fs::remove_file(&yaml_file).unwrap();
}

#[test]
fn test_read_toml_file_with_nested_tables() {
    let temp_dir = std::env::temp_dir();
    let toml_file = temp_dir.join("nested_tables.toml");
    fs::write(&toml_file, "[a]\n[a.b]\n[a.b.c]\nvalue = \"deep\"").unwrap();

    let context = Arc::new(TemplateContext::new(temp_dir.clone(), false));

    let result = ReadTomlFile::call(context.clone(), path_kwargs("nested_tables.toml")).unwrap();
    let a = result.get_attr("a").unwrap();
    let b = a.get_attr("b").unwrap();
    let c = b.get_attr("c").unwrap();
    assert_eq!(c.get_attr("value").unwrap().as_str().unwrap(), "deep");

    // Cleanup
    fs::remove_file(&toml_file).unwrap();
}

#[test]
fn test_read_json_file_with_unicode() {
    let temp_dir = std::env::temp_dir();
    let json_file = temp_dir.join("unicode.json");
    fs::write(&json_file, r#"{"emoji": "🚀", "text": "日本語"}"#).unwrap();

    let context = Arc::new(TemplateContext::new(temp_dir.clone(), false));

    let result = ReadJsonFile::call(context.clone(), path_kwargs("unicode.json")).unwrap();
    assert_eq!(result.get_attr("emoji").unwrap().as_str().unwrap(), "🚀");
    assert_eq!(result.get_attr("text").unwrap().as_str().unwrap(), "日本語");

    // Cleanup
    fs::remove_file(&json_file).unwrap();
}

#[test]
fn test_read_yaml_file_with_large_integers() {
    let temp_dir = std::env::temp_dir();
    let yaml_file = temp_dir.join("large_int.yaml");
    fs::write(&yaml_file, "big: 9223372036854775807").unwrap();

    let context = Arc::new(TemplateContext::new(temp_dir.clone(), false));

    let result = ReadYamlFile::call(context.clone(), path_kwargs("large_int.yaml")).unwrap();
    assert_eq!(
        result.get_attr("big").unwrap().as_i64(),
        Some(9223372036854775807i64)
    );

    // Cleanup
    fs::remove_file(&yaml_file).unwrap();
}

#[test]
fn test_read_toml_file_with_inline_table() {
    let temp_dir = std::env::temp_dir();
    let toml_file = temp_dir.join("inline_table.toml");
    fs::write(&toml_file, "point = { x = 10, y = 20 }").unwrap();

    let context = Arc::new(TemplateContext::new(temp_dir.clone(), false));

    let result = ReadTomlFile::call(context.clone(), path_kwargs("inline_table.toml")).unwrap();
    let point = result.get_attr("point").unwrap();
    assert_eq!(point.get_attr("x").unwrap().as_usize(), Some(10));
    assert_eq!(point.get_attr("y").unwrap().as_usize(), Some(20));

    // Cleanup
    fs::remove_file(&toml_file).unwrap();
}
