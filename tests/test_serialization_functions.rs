use minijinja::Value;
use minijinja::value::Kwargs;
use tmpltool::filter_functions::FilterFunction;
use tmpltool::filter_functions::serialization::{ToJson, ToToml, ToYaml};

#[test]
fn test_to_json_simple_object() {
    let obj = serde_json::json!({
        "name": "test",
        "value": 42,
        "active": true
    });

    let result = ToJson::call_as_function(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]))
    .unwrap();

    let json_str = result.as_str().unwrap();
    assert!(json_str.contains("\"name\":\"test\"") || json_str.contains("\"name\": \"test\""));
    assert!(json_str.contains("\"value\":42") || json_str.contains("\"value\": 42"));
    assert!(json_str.contains("\"active\":true") || json_str.contains("\"active\": true"));
}

#[test]
fn test_to_json_simple_object_pretty() {
    let obj = serde_json::json!({
        "name": "test",
        "value": 42
    });

    let result = ToJson::call_as_function(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("pretty", Value::from(true)),
    ]))
    .unwrap();

    let json_str = result.as_str().unwrap();
    // Pretty JSON should contain newlines and indentation
    assert!(json_str.contains('\n'));
    assert!(json_str.contains("  ")); // Indentation
}

#[test]
fn test_to_json_array() {
    let arr = vec![1, 2, 3, 4, 5];

    let result =
        ToJson::call_as_function(Kwargs::from_iter(vec![("object", Value::from(arr))])).unwrap();

    let json_str = result.as_str().unwrap();
    assert_eq!(json_str, "[1,2,3,4,5]");
}

#[test]
fn test_to_json_nested_object() {
    let obj = serde_json::json!({
        "database": {
            "host": "localhost",
            "port": 5432
        },
        "cache": {
            "enabled": true,
            "ttl": 3600
        }
    });

    let result = ToJson::call_as_function(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]))
    .unwrap();

    let json_str = result.as_str().unwrap();
    assert!(json_str.contains("database"));
    assert!(json_str.contains("cache"));
    assert!(json_str.contains("localhost"));
}

#[test]
fn test_to_json_string() {
    let result = ToJson::call_as_function(Kwargs::from_iter(vec![(
        "object",
        Value::from("hello world"),
    )]))
    .unwrap();

    assert_eq!(result.as_str().unwrap(), "\"hello world\"");
}

#[test]
fn test_to_json_number() {
    let result =
        ToJson::call_as_function(Kwargs::from_iter(vec![("object", Value::from(42))])).unwrap();

    assert_eq!(result.as_str().unwrap(), "42");
}

#[test]
fn test_to_json_boolean() {
    let result =
        ToJson::call_as_function(Kwargs::from_iter(vec![("object", Value::from(true))])).unwrap();

    assert_eq!(result.as_str().unwrap(), "true");
}

#[test]
fn test_to_json_null() {
    let result = ToJson::call_as_function(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&serde_json::Value::Null),
    )]))
    .unwrap();

    assert_eq!(result.as_str().unwrap(), "null");
}

#[test]
fn test_to_json_missing_object() {
    let result = ToJson::call_as_function(Kwargs::from_iter(Vec::<(&str, Value)>::new()));
    assert!(result.is_err());
}

#[test]
fn test_to_yaml_simple_object() {
    let obj = serde_json::json!({
        "host": "localhost",
        "port": 8080,
        "debug": true
    });

    let result = ToYaml::call_as_function(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]))
    .unwrap();

    let yaml_str = result.as_str().unwrap();
    assert!(yaml_str.contains("host: localhost"));
    assert!(yaml_str.contains("port: 8080"));
    assert!(yaml_str.contains("debug: true"));
}

#[test]
fn test_to_yaml_array() {
    let arr = vec!["apple", "banana", "cherry"];

    let result =
        ToYaml::call_as_function(Kwargs::from_iter(vec![("object", Value::from(arr))])).unwrap();

    let yaml_str = result.as_str().unwrap();
    assert!(yaml_str.contains("- apple"));
    assert!(yaml_str.contains("- banana"));
    assert!(yaml_str.contains("- cherry"));
}

#[test]
fn test_to_yaml_nested_object() {
    let obj = serde_json::json!({
        "server": {
            "host": "0.0.0.0",
            "port": 8080,
            "workers": 4
        },
        "database": {
            "url": "postgres://localhost/mydb",
            "pool_size": 10
        }
    });

    let result = ToYaml::call_as_function(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]))
    .unwrap();

    let yaml_str = result.as_str().unwrap();
    assert!(yaml_str.contains("server:"));
    assert!(yaml_str.contains("database:"));
    assert!(yaml_str.contains("host: 0.0.0.0"));
    assert!(yaml_str.contains("pool_size: 10"));
}

#[test]
fn test_to_yaml_string() {
    let result = ToYaml::call_as_function(Kwargs::from_iter(vec![(
        "object",
        Value::from("hello world"),
    )]))
    .unwrap();

    let yaml_str = result.as_str().unwrap();
    assert!(yaml_str.contains("hello world"));
}

#[test]
fn test_to_yaml_number() {
    let result =
        ToYaml::call_as_function(Kwargs::from_iter(vec![("object", Value::from(42))])).unwrap();

    let yaml_str = result.as_str().unwrap().trim();
    assert_eq!(yaml_str, "42");
}

#[test]
fn test_to_yaml_missing_object() {
    let result = ToYaml::call_as_function(Kwargs::from_iter(Vec::<(&str, Value)>::new()));
    assert!(result.is_err());
}

#[test]
fn test_to_toml_simple_object() {
    let obj = serde_json::json!({
        "title": "My App",
        "version": "1.0.0"
    });

    let result = ToToml::call_as_function(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]))
    .unwrap();

    let toml_str = result.as_str().unwrap();
    assert!(toml_str.contains("title = \"My App\""));
    assert!(toml_str.contains("version = \"1.0.0\""));
}

#[test]
fn test_to_toml_nested_object() {
    let obj = serde_json::json!({
        "package": {
            "name": "myapp",
            "version": "1.0.0"
        },
        "dependencies": {
            "serde": "1.0",
            "tokio": "1.0"
        }
    });

    let result = ToToml::call_as_function(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]))
    .unwrap();

    let toml_str = result.as_str().unwrap();
    assert!(toml_str.contains("[package]"));
    assert!(toml_str.contains("name = \"myapp\""));
    assert!(toml_str.contains("[dependencies]"));
    assert!(toml_str.contains("serde = \"1.0\""));
}

#[test]
fn test_to_toml_with_numbers() {
    let obj = serde_json::json!({
        "server": {
            "port": 8080,
            "workers": 4,
            "timeout": 30.5
        }
    });

    let result = ToToml::call_as_function(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]))
    .unwrap();

    let toml_str = result.as_str().unwrap();
    assert!(toml_str.contains("port = 8080"));
    assert!(toml_str.contains("workers = 4"));
    assert!(toml_str.contains("timeout = 30.5"));
}

#[test]
fn test_to_toml_with_boolean() {
    let obj = serde_json::json!({
        "features": {
            "debug": true,
            "logging": false
        }
    });

    let result = ToToml::call_as_function(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]))
    .unwrap();

    let toml_str = result.as_str().unwrap();
    assert!(toml_str.contains("debug = true"));
    assert!(toml_str.contains("logging = false"));
}

#[test]
fn test_to_toml_array_of_tables() {
    let obj = serde_json::json!({
        "database": [
            {"name": "primary", "host": "db1.example.com"},
            {"name": "replica", "host": "db2.example.com"}
        ]
    });

    let result = ToToml::call_as_function(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]))
    .unwrap();

    let toml_str = result.as_str().unwrap();
    assert!(toml_str.contains("[[database]]"));
    assert!(toml_str.contains("name = \"primary\""));
    assert!(toml_str.contains("db1.example.com"));
    assert!(toml_str.contains("db2.example.com"));
}

#[test]
fn test_to_toml_simple_array() {
    let obj = serde_json::json!({
        "ports": [8080, 8081, 8082]
    });

    let result = ToToml::call_as_function(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]))
    .unwrap();

    let toml_str = result.as_str().unwrap();
    assert!(toml_str.contains("ports = [8080, 8081, 8082]"));
}

#[test]
fn test_to_toml_missing_object() {
    let result = ToToml::call_as_function(Kwargs::from_iter(Vec::<(&str, Value)>::new()));
    assert!(result.is_err());
}

#[test]
fn test_roundtrip_json_object() {
    // Test that we can convert to JSON and parse it back
    let original = serde_json::json!({
        "name": "test",
        "count": 42,
        "active": true,
        "items": [1, 2, 3]
    });

    let json_result = ToJson::call_as_function(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&original),
    )]))
    .unwrap();

    let json_str = json_result.as_str().unwrap();

    // Parse it back
    let parsed: serde_json::Value = serde_json::from_str(json_str).unwrap();

    assert_eq!(parsed["name"], "test");
    assert_eq!(parsed["count"], 42);
    assert_eq!(parsed["active"], true);
    assert_eq!(parsed["items"], serde_json::json!([1, 2, 3]));
}

#[test]
fn test_roundtrip_yaml_object() {
    // Test that we can convert to YAML and parse it back
    let original = serde_json::json!({
        "host": "localhost",
        "port": 8080
    });

    let yaml_result = ToYaml::call_as_function(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&original),
    )]))
    .unwrap();

    let yaml_str = yaml_result.as_str().unwrap();

    // Parse it back
    let parsed: serde_yaml::Value = serde_yaml::from_str(yaml_str).unwrap();

    assert_eq!(parsed["host"], "localhost");
    assert_eq!(parsed["port"], 8080);
}

#[test]
fn test_roundtrip_toml_object() {
    // Test that we can convert to TOML and parse it back
    let original = serde_json::json!({
        "title": "Test",
        "version": "1.0.0"
    });

    let toml_result = ToToml::call_as_function(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&original),
    )]))
    .unwrap();

    let toml_str = toml_result.as_str().unwrap();

    // Parse it back
    let parsed: toml::Value = toml::from_str(toml_str).unwrap();

    assert_eq!(parsed["title"], toml::Value::String("Test".to_string()));
    assert_eq!(parsed["version"], toml::Value::String("1.0.0".to_string()));
}

#[test]
fn test_to_json_empty_object() {
    let obj = serde_json::json!({});

    let result = ToJson::call_as_function(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]))
    .unwrap();

    assert_eq!(result.as_str().unwrap(), "{}");
}

#[test]
fn test_to_json_empty_array() {
    let arr: Vec<i32> = vec![];

    let result =
        ToJson::call_as_function(Kwargs::from_iter(vec![("object", Value::from(arr))])).unwrap();

    assert_eq!(result.as_str().unwrap(), "[]");
}

#[test]
fn test_to_yaml_empty_object() {
    let obj = serde_json::json!({});

    let result = ToYaml::call_as_function(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]))
    .unwrap();

    assert_eq!(result.as_str().unwrap().trim(), "{}");
}

#[test]
fn test_to_toml_empty_object() {
    let obj = serde_json::json!({});

    let result = ToToml::call_as_function(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]))
    .unwrap();

    // Empty TOML should be empty or just whitespace
    assert!(result.as_str().unwrap().trim().is_empty());
}

// ============================================================================
// Error Handling Tests
// ============================================================================

#[test]
fn test_to_json_error_missing_argument() {
    let result = ToJson::call_as_function(Kwargs::from_iter(Vec::<(&str, Value)>::new()));
    assert!(result.is_err());
    // Error should be about missing argument from Kwargs::get()
}

#[test]
fn test_to_yaml_error_missing_argument() {
    let result = ToYaml::call_as_function(Kwargs::from_iter(Vec::<(&str, Value)>::new()));
    assert!(result.is_err());
    // Error should be about missing argument from Kwargs::get()
}

#[test]
fn test_to_toml_error_missing_argument() {
    let result = ToToml::call_as_function(Kwargs::from_iter(Vec::<(&str, Value)>::new()));
    assert!(result.is_err());
    // Error should be about missing argument from Kwargs::get()
}

#[test]
fn test_to_toml_error_array_root() {
    // TOML does not support arrays at the root level
    let arr = vec![1, 2, 3];

    let result = ToToml::call_as_function(Kwargs::from_iter(vec![("object", Value::from(arr))]));

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Failed to serialize to TOML"));
}

#[test]
fn test_to_toml_error_string_root() {
    // TOML does not support strings at the root level
    let result =
        ToToml::call_as_function(Kwargs::from_iter(vec![("object", Value::from("hello"))]));

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Failed to serialize to TOML"));
}

#[test]
fn test_to_toml_error_number_root() {
    // TOML does not support numbers at the root level
    let result = ToToml::call_as_function(Kwargs::from_iter(vec![("object", Value::from(42))]));

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Failed to serialize to TOML"));
}

#[test]
fn test_to_toml_error_boolean_root() {
    // TOML does not support booleans at the root level
    let result = ToToml::call_as_function(Kwargs::from_iter(vec![("object", Value::from(true))]));

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Failed to serialize to TOML"));
}

#[test]
fn test_to_toml_error_nested_mixed_array() {
    // TOML can be strict about certain nested structures
    // Test with arrays of tables where structure doesn't match
    let obj = serde_json::json!({
        "items": [
            {"type": "a", "value": 1},
            {"type": "b", "extra": "field"}  // Different structure
        ]
    });

    let result = ToToml::call_as_function(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]));

    // This should succeed - TOML can handle tables with different fields
    // Just verify it doesn't panic
    if let Ok(value) = result {
        assert!(value.as_str().is_some());
    }
}

#[test]
fn test_to_json_invalid_pretty_type() {
    // Test that pretty parameter accepts boolean (non-boolean should use default)
    let obj = serde_json::json!({"test": "value"});

    // This should work - the pretty param will just use default if wrong type
    let result = ToJson::call_as_function(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("pretty", Value::from("not a bool")), // Wrong type, should use default (false)
    ]));

    // Should still succeed with default pretty=false behavior
    assert!(result.is_ok());
}

#[test]
fn test_to_json_with_undefined_in_object() {
    // Test JSON serialization with undefined values (should convert to null)
    let obj = serde_json::json!({
        "defined": "value",
        "nullable": null
    });

    let result = ToJson::call_as_function(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]))
    .unwrap();

    let json_str = result.as_str().unwrap();
    assert!(json_str.contains("\"nullable\":null") || json_str.contains("\"nullable\": null"));
}

#[test]
fn test_to_yaml_with_null() {
    // Test YAML serialization with null values
    let obj = serde_json::json!({
        "key": null
    });

    let result = ToYaml::call_as_function(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]))
    .unwrap();

    let yaml_str = result.as_str().unwrap();
    assert!(yaml_str.contains("key:") && (yaml_str.contains("null") || yaml_str.contains("~")));
}

#[test]
fn test_to_toml_with_null_value() {
    // TOML doesn't have a null type, so this should fail or omit the field
    let obj = serde_json::json!({
        "key": null
    });

    let result = ToToml::call_as_function(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]));

    // TOML serialization with null should either fail or succeed with omitted field
    // This depends on serde's behavior - typically it omits nulls
    if let Ok(value) = result {
        let toml_str = value.as_str().unwrap().to_string();
        // Null fields are typically omitted in TOML
        assert!(!toml_str.contains("null"));
    }
    // If it errors, that's also valid behavior
}

// ============================================================================
// Additional Edge Case Tests for Coverage Improvement
// ============================================================================

// --- Float and numeric edge cases ---

#[test]
fn test_to_json_float() {
    let result =
        ToJson::call_as_function(Kwargs::from_iter(vec![("object", Value::from(1.23456))]))
            .unwrap();

    let json_str = result.as_str().unwrap();
    assert!(json_str.contains("1.23"));
}

#[test]
fn test_to_json_negative_number() {
    let result =
        ToJson::call_as_function(Kwargs::from_iter(vec![("object", Value::from(-42))])).unwrap();

    assert_eq!(result.as_str().unwrap(), "-42");
}

#[test]
fn test_to_json_large_number() {
    let large_num: i64 = 9223372036854775807; // i64::MAX
    let result =
        ToJson::call_as_function(Kwargs::from_iter(vec![("object", Value::from(large_num))]))
            .unwrap();

    assert_eq!(result.as_str().unwrap(), "9223372036854775807");
}

#[test]
fn test_to_json_zero() {
    let result =
        ToJson::call_as_function(Kwargs::from_iter(vec![("object", Value::from(0))])).unwrap();

    assert_eq!(result.as_str().unwrap(), "0");
}

#[test]
fn test_to_yaml_float() {
    let result =
        ToYaml::call_as_function(Kwargs::from_iter(vec![("object", Value::from(1.23456))]))
            .unwrap();

    let yaml_str = result.as_str().unwrap().trim();
    assert!(yaml_str.contains("1.23"));
}

#[test]
fn test_to_yaml_negative_number() {
    let result =
        ToYaml::call_as_function(Kwargs::from_iter(vec![("object", Value::from(-100))])).unwrap();

    let yaml_str = result.as_str().unwrap().trim();
    assert!(yaml_str.contains("-100"));
}

#[test]
fn test_to_yaml_boolean_false() {
    let result =
        ToYaml::call_as_function(Kwargs::from_iter(vec![("object", Value::from(false))])).unwrap();

    let yaml_str = result.as_str().unwrap().trim();
    assert_eq!(yaml_str, "false");
}

#[test]
fn test_to_toml_float() {
    let obj = serde_json::json!({
        "value": 1.23456
    });

    let result = ToToml::call_as_function(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]))
    .unwrap();

    let toml_str = result.as_str().unwrap();
    assert!(toml_str.contains("1.23"));
}

#[test]
fn test_to_toml_negative_number() {
    let obj = serde_json::json!({
        "value": -42
    });

    let result = ToToml::call_as_function(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]))
    .unwrap();

    let toml_str = result.as_str().unwrap();
    assert!(toml_str.contains("-42"));
}

// --- String edge cases ---

#[test]
fn test_to_json_empty_string() {
    let result =
        ToJson::call_as_function(Kwargs::from_iter(vec![("object", Value::from(""))])).unwrap();

    assert_eq!(result.as_str().unwrap(), "\"\"");
}

#[test]
fn test_to_json_unicode_string() {
    let result = ToJson::call_as_function(Kwargs::from_iter(vec![(
        "object",
        Value::from("Hello 世界 🌍"),
    )]))
    .unwrap();

    let json_str = result.as_str().unwrap();
    assert!(json_str.contains("Hello"));
    // Unicode should be preserved or escaped
}

#[test]
fn test_to_json_special_chars() {
    let result = ToJson::call_as_function(Kwargs::from_iter(vec![(
        "object",
        Value::from("line1\nline2\ttab\"quote"),
    )]))
    .unwrap();

    let json_str = result.as_str().unwrap();
    // Should properly escape special characters
    assert!(json_str.contains("\\n") || json_str.contains("\\t") || json_str.contains("\\\""));
}

#[test]
fn test_to_json_backslash() {
    let result = ToJson::call_as_function(Kwargs::from_iter(vec![(
        "object",
        Value::from("path\\to\\file"),
    )]))
    .unwrap();

    let json_str = result.as_str().unwrap();
    assert!(json_str.contains("\\\\"));
}

#[test]
fn test_to_yaml_empty_string() {
    let obj = serde_json::json!({
        "key": ""
    });

    let result = ToYaml::call_as_function(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]))
    .unwrap();

    let yaml_str = result.as_str().unwrap();
    assert!(yaml_str.contains("key:"));
}

#[test]
fn test_to_yaml_multiline_string() {
    let obj = serde_json::json!({
        "text": "line1\nline2\nline3"
    });

    let result = ToYaml::call_as_function(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]))
    .unwrap();

    let yaml_str = result.as_str().unwrap();
    assert!(yaml_str.contains("text:"));
}

#[test]
fn test_to_yaml_unicode_string() {
    let obj = serde_json::json!({
        "greeting": "你好世界"
    });

    let result = ToYaml::call_as_function(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]))
    .unwrap();

    let yaml_str = result.as_str().unwrap();
    assert!(yaml_str.contains("greeting:"));
}

#[test]
fn test_to_toml_empty_string() {
    let obj = serde_json::json!({
        "key": ""
    });

    let result = ToToml::call_as_function(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]))
    .unwrap();

    let toml_str = result.as_str().unwrap();
    assert!(toml_str.contains("key = \"\""));
}

#[test]
fn test_to_toml_string_with_quotes() {
    let obj = serde_json::json!({
        "text": "He said \"hello\""
    });

    let result = ToToml::call_as_function(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]))
    .unwrap();

    let toml_str = result.as_str().unwrap();
    assert!(toml_str.contains("text = "));
}

// --- Nested structure edge cases ---

#[test]
fn test_to_json_deeply_nested() {
    let obj = serde_json::json!({
        "a": {"b": {"c": {"d": {"e": "deep"}}}}
    });

    let result = ToJson::call_as_function(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]))
    .unwrap();

    let json_str = result.as_str().unwrap();
    assert!(json_str.contains("deep"));
}

#[test]
fn test_to_json_deeply_nested_pretty() {
    let obj = serde_json::json!({
        "a": {"b": {"c": "value"}}
    });

    let result = ToJson::call_as_function(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("pretty", Value::from(true)),
    ]))
    .unwrap();

    let json_str = result.as_str().unwrap();
    // Should have multiple levels of indentation
    assert!(json_str.contains("    ")); // At least 4 spaces for 2 levels
}

#[test]
fn test_to_yaml_deeply_nested() {
    let obj = serde_json::json!({
        "level1": {
            "level2": {
                "level3": {
                    "value": "deep"
                }
            }
        }
    });

    let result = ToYaml::call_as_function(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]))
    .unwrap();

    let yaml_str = result.as_str().unwrap();
    assert!(yaml_str.contains("level1:"));
    assert!(yaml_str.contains("value: deep"));
}

#[test]
fn test_to_toml_deeply_nested() {
    let obj = serde_json::json!({
        "section": {
            "subsection": {
                "key": "value"
            }
        }
    });

    let result = ToToml::call_as_function(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]))
    .unwrap();

    let toml_str = result.as_str().unwrap();
    assert!(toml_str.contains("[section.subsection]") || toml_str.contains("section.subsection"));
}

// --- Array edge cases ---

#[test]
fn test_to_json_nested_arrays() {
    let obj = serde_json::json!([[1, 2], [3, 4], [5, 6]]);

    let result = ToJson::call_as_function(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]))
    .unwrap();

    let json_str = result.as_str().unwrap();
    assert_eq!(json_str, "[[1,2],[3,4],[5,6]]");
}

#[test]
fn test_to_json_array_of_objects() {
    let arr = serde_json::json!([
        {"name": "Alice", "age": 30},
        {"name": "Bob", "age": 25}
    ]);

    let result = ToJson::call_as_function(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&arr),
    )]))
    .unwrap();

    let json_str = result.as_str().unwrap();
    assert!(json_str.contains("Alice"));
    assert!(json_str.contains("Bob"));
}

#[test]
fn test_to_yaml_nested_arrays() {
    let obj = serde_json::json!({
        "matrix": [[1, 2], [3, 4]]
    });

    let result = ToYaml::call_as_function(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]))
    .unwrap();

    let yaml_str = result.as_str().unwrap();
    assert!(yaml_str.contains("matrix:"));
}

#[test]
fn test_to_yaml_mixed_array() {
    let arr = serde_json::json!(["string", 42, true, null]);

    let result = ToYaml::call_as_function(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&arr),
    )]))
    .unwrap();

    let yaml_str = result.as_str().unwrap();
    assert!(yaml_str.contains("- string") || yaml_str.contains("string"));
    assert!(yaml_str.contains("42"));
}

#[test]
fn test_to_yaml_empty_array() {
    let arr: Vec<i32> = vec![];

    let result =
        ToYaml::call_as_function(Kwargs::from_iter(vec![("object", Value::from(arr))])).unwrap();

    let yaml_str = result.as_str().unwrap().trim();
    assert_eq!(yaml_str, "[]");
}

#[test]
fn test_to_toml_string_array() {
    let obj = serde_json::json!({
        "names": ["Alice", "Bob", "Charlie"]
    });

    let result = ToToml::call_as_function(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]))
    .unwrap();

    let toml_str = result.as_str().unwrap();
    assert!(toml_str.contains("names = ["));
    assert!(toml_str.contains("Alice"));
}

// --- Pretty printing edge cases ---

#[test]
fn test_to_json_pretty_false_explicit() {
    let obj = serde_json::json!({"key": "value"});

    let result = ToJson::call_as_function(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("pretty", Value::from(false)),
    ]))
    .unwrap();

    let json_str = result.as_str().unwrap();
    // Compact JSON should not have newlines
    assert!(!json_str.contains('\n'));
}

#[test]
fn test_to_json_pretty_with_array() {
    let arr = serde_json::json!([1, 2, 3, 4, 5]);

    let result = ToJson::call_as_function(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&arr)),
        ("pretty", Value::from(true)),
    ]))
    .unwrap();

    let json_str = result.as_str().unwrap();
    // Pretty array should have newlines
    assert!(json_str.contains('\n'));
}

#[test]
fn test_to_json_pretty_empty_object() {
    let obj = serde_json::json!({});

    let result = ToJson::call_as_function(Kwargs::from_iter(vec![
        ("object", Value::from_serialize(&obj)),
        ("pretty", Value::from(true)),
    ]))
    .unwrap();

    assert_eq!(result.as_str().unwrap(), "{}");
}

// --- Complex real-world structures ---

#[test]
fn test_to_json_complex_config() {
    let config = serde_json::json!({
        "server": {
            "host": "0.0.0.0",
            "port": 8080,
            "ssl": {
                "enabled": true,
                "cert": "/path/to/cert.pem",
                "key": "/path/to/key.pem"
            }
        },
        "database": {
            "connections": [
                {"name": "primary", "url": "postgres://localhost/db"},
                {"name": "replica", "url": "postgres://replica/db"}
            ]
        },
        "features": ["auth", "cache", "logging"]
    });

    let result = ToJson::call_as_function(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&config),
    )]))
    .unwrap();

    let json_str = result.as_str().unwrap();
    assert!(json_str.contains("server"));
    assert!(json_str.contains("database"));
    assert!(json_str.contains("features"));
}

#[test]
fn test_to_yaml_complex_config() {
    let config = serde_json::json!({
        "apiVersion": "v1",
        "kind": "ConfigMap",
        "metadata": {
            "name": "my-config",
            "namespace": "default"
        },
        "data": {
            "key1": "value1",
            "key2": "value2"
        }
    });

    let result = ToYaml::call_as_function(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&config),
    )]))
    .unwrap();

    let yaml_str = result.as_str().unwrap();
    assert!(yaml_str.contains("apiVersion:"));
    assert!(yaml_str.contains("kind:"));
    assert!(yaml_str.contains("metadata:"));
}

#[test]
fn test_to_toml_cargo_like() {
    let config = serde_json::json!({
        "package": {
            "name": "myapp",
            "version": "0.1.0",
            "edition": "2021"
        },
        "dependencies": {
            "serde": "1.0",
            "tokio": "1.0"
        },
        "dev-dependencies": {
            "criterion": "0.5"
        }
    });

    let result = ToToml::call_as_function(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&config),
    )]))
    .unwrap();

    let toml_str = result.as_str().unwrap();
    assert!(toml_str.contains("[package]"));
    assert!(toml_str.contains("[dependencies]"));
}

// --- Primitive value handling in objects ---

#[test]
fn test_to_json_all_primitive_types() {
    let obj = serde_json::json!({
        "string": "hello",
        "integer": 42,
        "float": 1.238,
        "boolean_true": true,
        "boolean_false": false,
        "null_value": null,
        "array": [1, 2, 3],
        "object": {"nested": "value"}
    });

    let result = ToJson::call_as_function(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]))
    .unwrap();

    let json_str = result.as_str().unwrap();
    // Parse back to verify structure
    let parsed: serde_json::Value = serde_json::from_str(json_str).unwrap();
    assert_eq!(parsed["string"], "hello");
    assert_eq!(parsed["integer"], 42);
    assert_eq!(parsed["boolean_true"], true);
    assert_eq!(parsed["boolean_false"], false);
    assert!(parsed["null_value"].is_null());
}

#[test]
fn test_to_yaml_all_primitive_types() {
    let obj = serde_json::json!({
        "string": "hello",
        "integer": 42,
        "float": 1.238,
        "boolean": true,
        "array": [1, 2, 3]
    });

    let result = ToYaml::call_as_function(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]))
    .unwrap();

    let yaml_str = result.as_str().unwrap();
    assert!(yaml_str.contains("string: hello"));
    assert!(yaml_str.contains("integer: 42"));
    assert!(yaml_str.contains("boolean: true"));
}

#[test]
fn test_to_toml_all_primitive_types() {
    let obj = serde_json::json!({
        "string": "hello",
        "integer": 42,
        "float": 1.238,
        "boolean": true,
        "array": [1, 2, 3]
    });

    let result = ToToml::call_as_function(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]))
    .unwrap();

    let toml_str = result.as_str().unwrap();
    assert!(toml_str.contains("string = \"hello\""));
    assert!(toml_str.contains("integer = 42"));
    assert!(toml_str.contains("boolean = true"));
}
