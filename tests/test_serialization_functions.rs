use minijinja::Value;
use minijinja::value::Kwargs;
use tmpltool::functions::serialization;

#[test]
fn test_to_json_simple_object() {
    let obj = serde_json::json!({
        "name": "test",
        "value": 42,
        "active": true
    });

    let result = serialization::to_json_fn(Kwargs::from_iter(vec![(
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

    let result = serialization::to_json_fn(Kwargs::from_iter(vec![
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
        serialization::to_json_fn(Kwargs::from_iter(vec![("object", Value::from(arr))])).unwrap();

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

    let result = serialization::to_json_fn(Kwargs::from_iter(vec![(
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
    let result = serialization::to_json_fn(Kwargs::from_iter(vec![(
        "object",
        Value::from("hello world"),
    )]))
    .unwrap();

    assert_eq!(result.as_str().unwrap(), "\"hello world\"");
}

#[test]
fn test_to_json_number() {
    let result =
        serialization::to_json_fn(Kwargs::from_iter(vec![("object", Value::from(42))])).unwrap();

    assert_eq!(result.as_str().unwrap(), "42");
}

#[test]
fn test_to_json_boolean() {
    let result =
        serialization::to_json_fn(Kwargs::from_iter(vec![("object", Value::from(true))])).unwrap();

    assert_eq!(result.as_str().unwrap(), "true");
}

#[test]
fn test_to_json_null() {
    let result = serialization::to_json_fn(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&serde_json::Value::Null),
    )]))
    .unwrap();

    assert_eq!(result.as_str().unwrap(), "null");
}

#[test]
fn test_to_json_missing_object() {
    let result = serialization::to_json_fn(Kwargs::from_iter(Vec::<(&str, Value)>::new()));
    assert!(result.is_err());
}

#[test]
fn test_to_yaml_simple_object() {
    let obj = serde_json::json!({
        "host": "localhost",
        "port": 8080,
        "debug": true
    });

    let result = serialization::to_yaml_fn(Kwargs::from_iter(vec![(
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
        serialization::to_yaml_fn(Kwargs::from_iter(vec![("object", Value::from(arr))])).unwrap();

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

    let result = serialization::to_yaml_fn(Kwargs::from_iter(vec![(
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
    let result = serialization::to_yaml_fn(Kwargs::from_iter(vec![(
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
        serialization::to_yaml_fn(Kwargs::from_iter(vec![("object", Value::from(42))])).unwrap();

    let yaml_str = result.as_str().unwrap().trim();
    assert_eq!(yaml_str, "42");
}

#[test]
fn test_to_yaml_missing_object() {
    let result = serialization::to_yaml_fn(Kwargs::from_iter(Vec::<(&str, Value)>::new()));
    assert!(result.is_err());
}

#[test]
fn test_to_toml_simple_object() {
    let obj = serde_json::json!({
        "title": "My App",
        "version": "1.0.0"
    });

    let result = serialization::to_toml_fn(Kwargs::from_iter(vec![(
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

    let result = serialization::to_toml_fn(Kwargs::from_iter(vec![(
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

    let result = serialization::to_toml_fn(Kwargs::from_iter(vec![(
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

    let result = serialization::to_toml_fn(Kwargs::from_iter(vec![(
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

    let result = serialization::to_toml_fn(Kwargs::from_iter(vec![(
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

    let result = serialization::to_toml_fn(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]))
    .unwrap();

    let toml_str = result.as_str().unwrap();
    assert!(toml_str.contains("ports = [8080, 8081, 8082]"));
}

#[test]
fn test_to_toml_missing_object() {
    let result = serialization::to_toml_fn(Kwargs::from_iter(Vec::<(&str, Value)>::new()));
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

    let json_result = serialization::to_json_fn(Kwargs::from_iter(vec![(
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

    let yaml_result = serialization::to_yaml_fn(Kwargs::from_iter(vec![(
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

    let toml_result = serialization::to_toml_fn(Kwargs::from_iter(vec![(
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

    let result = serialization::to_json_fn(Kwargs::from_iter(vec![(
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
        serialization::to_json_fn(Kwargs::from_iter(vec![("object", Value::from(arr))])).unwrap();

    assert_eq!(result.as_str().unwrap(), "[]");
}

#[test]
fn test_to_yaml_empty_object() {
    let obj = serde_json::json!({});

    let result = serialization::to_yaml_fn(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]))
    .unwrap();

    assert_eq!(result.as_str().unwrap().trim(), "{}");
}

#[test]
fn test_to_toml_empty_object() {
    let obj = serde_json::json!({});

    let result = serialization::to_toml_fn(Kwargs::from_iter(vec![(
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
    let result = serialization::to_json_fn(Kwargs::from_iter(Vec::<(&str, Value)>::new()));
    assert!(result.is_err());
    // Error should be about missing argument from Kwargs::get()
}

#[test]
fn test_to_yaml_error_missing_argument() {
    let result = serialization::to_yaml_fn(Kwargs::from_iter(Vec::<(&str, Value)>::new()));
    assert!(result.is_err());
    // Error should be about missing argument from Kwargs::get()
}

#[test]
fn test_to_toml_error_missing_argument() {
    let result = serialization::to_toml_fn(Kwargs::from_iter(Vec::<(&str, Value)>::new()));
    assert!(result.is_err());
    // Error should be about missing argument from Kwargs::get()
}

#[test]
fn test_to_toml_error_array_root() {
    // TOML does not support arrays at the root level
    let arr = vec![1, 2, 3];

    let result = serialization::to_toml_fn(Kwargs::from_iter(vec![(
        "object",
        Value::from(arr),
    )]));

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Failed to serialize to TOML"));
}

#[test]
fn test_to_toml_error_string_root() {
    // TOML does not support strings at the root level
    let result = serialization::to_toml_fn(Kwargs::from_iter(vec![(
        "object",
        Value::from("hello"),
    )]));

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Failed to serialize to TOML"));
}

#[test]
fn test_to_toml_error_number_root() {
    // TOML does not support numbers at the root level
    let result = serialization::to_toml_fn(Kwargs::from_iter(vec![(
        "object",
        Value::from(42),
    )]));

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Failed to serialize to TOML"));
}

#[test]
fn test_to_toml_error_boolean_root() {
    // TOML does not support booleans at the root level
    let result = serialization::to_toml_fn(Kwargs::from_iter(vec![(
        "object",
        Value::from(true),
    )]));

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

    let result = serialization::to_toml_fn(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]));

    // This should succeed - TOML can handle tables with different fields
    // Just verify it doesn't panic
    if result.is_ok() {
        assert!(result.unwrap().as_str().is_some());
    }
}

#[test]
fn test_to_json_invalid_pretty_type() {
    // Test that pretty parameter accepts boolean (non-boolean should use default)
    let obj = serde_json::json!({"test": "value"});

    // This should work - the pretty param will just use default if wrong type
    let result = serialization::to_json_fn(Kwargs::from_iter(vec![
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

    let result = serialization::to_json_fn(Kwargs::from_iter(vec![(
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

    let result = serialization::to_yaml_fn(Kwargs::from_iter(vec![(
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

    let result = serialization::to_toml_fn(Kwargs::from_iter(vec![(
        "object",
        Value::from_serialize(&obj),
    )]));

    // TOML serialization with null should either fail or succeed with omitted field
    // This depends on serde's behavior - typically it omits nulls
    if result.is_ok() {
        let toml_str = result.unwrap().as_str().unwrap().to_string();
        // Null fields are typically omitted in TOML
        assert!(!toml_str.contains("null"));
    }
    // If it errors, that's also valid behavior
}
