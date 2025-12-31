use minijinja::value::Kwargs;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tmpltool::TemplateContext;
use tmpltool::functions::data_parsing::{
    create_read_json_file_fn, create_read_toml_file_fn, create_read_yaml_file_fn, parse_json_fn,
    parse_toml_fn, parse_yaml_fn,
};

// Helper to create kwargs for testing
fn create_kwargs(args: Vec<(&str, &str)>) -> Kwargs {
    Kwargs::from_iter(args.iter().map(|(k, v)| (*k, minijinja::Value::from(*v))))
}

// ========== parse_json tests ==========

#[test]
fn test_parse_json_simple_object() {
    let kwargs = create_kwargs(vec![("string", r#"{"name": "John", "age": 30}"#)]);

    let result = parse_json_fn(kwargs).unwrap();
    assert_eq!(result.get_attr("name").unwrap().as_str().unwrap(), "John");
    assert_eq!(result.get_attr("age").unwrap().as_usize(), Some(30));
}

#[test]
fn test_parse_json_array() {
    let kwargs = create_kwargs(vec![("string", r#"[1, 2, 3, 4, 5]"#)]);

    let result = parse_json_fn(kwargs).unwrap();
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

    let result = parse_json_fn(kwargs).unwrap();
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

    let result = parse_json_fn(kwargs);
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
    let result = parse_json_fn(kwargs);
    assert!(result.is_err());
}

// ========== parse_yaml tests ==========

#[test]
fn test_parse_yaml_simple_object() {
    let kwargs = create_kwargs(vec![("string", "name: John\nage: 30")]);

    let result = parse_yaml_fn(kwargs).unwrap();
    assert_eq!(result.get_attr("name").unwrap().as_str().unwrap(), "John");
    assert_eq!(result.get_attr("age").unwrap().as_usize(), Some(30));
}

#[test]
fn test_parse_yaml_array() {
    let kwargs = create_kwargs(vec![("string", "- apple\n- banana\n- cherry")]);

    let result = parse_yaml_fn(kwargs).unwrap();
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

    let result = parse_yaml_fn(kwargs).unwrap();
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

    let result = parse_yaml_fn(kwargs);
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
    let result = parse_yaml_fn(kwargs);
    assert!(result.is_err());
}

// ========== parse_toml tests ==========

#[test]
fn test_parse_toml_simple() {
    let kwargs = create_kwargs(vec![("string", "name = \"John\"\nage = 30")]);

    let result = parse_toml_fn(kwargs).unwrap();
    assert_eq!(result.get_attr("name").unwrap().as_str().unwrap(), "John");
    assert_eq!(result.get_attr("age").unwrap().as_usize(), Some(30));
}

#[test]
fn test_parse_toml_array() {
    let kwargs = create_kwargs(vec![("string", "colors = [\"red\", \"green\", \"blue\"]")]);

    let result = parse_toml_fn(kwargs).unwrap();
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

    let result = parse_toml_fn(kwargs).unwrap();
    let db = result.get_attr("database").unwrap();
    assert_eq!(db.get_attr("host").unwrap().as_str().unwrap(), "localhost");
    assert_eq!(db.get_attr("port").unwrap().as_usize(), Some(5432));
    assert!(db.get_attr("enabled").unwrap().is_true());
}

#[test]
fn test_parse_toml_invalid() {
    let kwargs = create_kwargs(vec![("string", "invalid toml = [missing quote")]);

    let result = parse_toml_fn(kwargs);
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
    let result = parse_toml_fn(kwargs);
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
    let read_json_file = create_read_json_file_fn(context);

    let result = read_json_file("test_data.json".to_string()).unwrap();
    assert_eq!(result.get_attr("name").unwrap().as_str().unwrap(), "Test");
    assert_eq!(result.get_attr("value").unwrap().as_usize(), Some(42));

    // Cleanup
    fs::remove_file(&json_file).unwrap();
}

#[test]
fn test_read_json_file_not_found() {
    let temp_dir = std::env::temp_dir();
    let context = Arc::new(TemplateContext::new(temp_dir, false));
    let read_json_file = create_read_json_file_fn(context);

    let result = read_json_file("nonexistent.json".to_string());
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
    let read_json_file = create_read_json_file_fn(context);

    let result = read_json_file("invalid.json".to_string());
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
    let read_json_file = create_read_json_file_fn(context);

    let result = read_json_file("/etc/passwd".to_string());
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
    let read_json_file = create_read_json_file_fn(context);

    let result = read_json_file("../../../etc/passwd".to_string());
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
    let read_json_file = create_read_json_file_fn(context);

    let result = read_json_file(json_file.to_string_lossy().to_string()).unwrap();
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
    let read_yaml_file = create_read_yaml_file_fn(context);

    let result = read_yaml_file("test_data.yaml".to_string()).unwrap();
    assert_eq!(result.get_attr("name").unwrap().as_str().unwrap(), "Test");
    assert_eq!(result.get_attr("value").unwrap().as_usize(), Some(42));

    // Cleanup
    fs::remove_file(&yaml_file).unwrap();
}

#[test]
fn test_read_yaml_file_not_found() {
    let temp_dir = std::env::temp_dir();
    let context = Arc::new(TemplateContext::new(temp_dir, false));
    let read_yaml_file = create_read_yaml_file_fn(context);

    let result = read_yaml_file("nonexistent.yaml".to_string());
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
    let read_yaml_file = create_read_yaml_file_fn(context);

    let result = read_yaml_file("array.yaml".to_string()).unwrap();
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
    let read_yaml_file = create_read_yaml_file_fn(context);

    let result = read_yaml_file("/etc/config.yaml".to_string());
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
    let read_toml_file = create_read_toml_file_fn(context);

    let result = read_toml_file("test_data.toml".to_string()).unwrap();
    assert_eq!(result.get_attr("name").unwrap().as_str().unwrap(), "Test");
    assert_eq!(result.get_attr("value").unwrap().as_usize(), Some(42));

    // Cleanup
    fs::remove_file(&toml_file).unwrap();
}

#[test]
fn test_read_toml_file_not_found() {
    let temp_dir = std::env::temp_dir();
    let context = Arc::new(TemplateContext::new(temp_dir, false));
    let read_toml_file = create_read_toml_file_fn(context);

    let result = read_toml_file("nonexistent.toml".to_string());
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
    let read_toml_file = create_read_toml_file_fn(context);

    let result = read_toml_file("config.toml".to_string()).unwrap();
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
    let read_toml_file = create_read_toml_file_fn(context);

    let result = read_toml_file("invalid.toml".to_string());
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
    let read_toml_file = create_read_toml_file_fn(context);

    let result = read_toml_file("../../config.toml".to_string());
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Parent directory (..) traversal")
    );
}
