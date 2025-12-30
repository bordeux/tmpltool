use std::collections::HashMap;
use tera::Value;
use tmpltool::functions::filter_env::FilterEnv;

// Import the Function trait to use call()
use tera::Function;

#[test]
fn test_filter_env_basic() {
    // Set test environment variables
    unsafe {
        std::env::set_var("TEST_VAR_ONE", "value1");
        std::env::set_var("TEST_VAR_TWO", "value2");
        std::env::set_var("OTHER_VAR", "other");
    }

    let mut args = HashMap::new();
    args.insert(
        "pattern".to_string(),
        Value::String("TEST_VAR_*".to_string()),
    );

    let result = FilterEnv.call(&args).unwrap();
    let array = result.as_array().unwrap();

    assert_eq!(array.len(), 2);

    // Verify both TEST_VAR_* variables are present
    let keys: Vec<String> = array
        .iter()
        .map(|item| item.get("key").unwrap().as_str().unwrap().to_string())
        .collect();

    assert!(keys.contains(&"TEST_VAR_ONE".to_string()));
    assert!(keys.contains(&"TEST_VAR_TWO".to_string()));

    // Cleanup
    unsafe {
        std::env::remove_var("TEST_VAR_ONE");
        std::env::remove_var("TEST_VAR_TWO");
        std::env::remove_var("OTHER_VAR");
    }
}

#[test]
fn test_filter_env_wildcard_middle() {
    unsafe {
        std::env::set_var("PREFIX_MIDDLE_SUFFIX", "value1");
        std::env::set_var("PREFIX_OTHER_SUFFIX", "value2");
    }

    let mut args = HashMap::new();
    args.insert(
        "pattern".to_string(),
        Value::String("PREFIX_*_SUFFIX".to_string()),
    );

    let result = FilterEnv.call(&args).unwrap();
    let array = result.as_array().unwrap();

    assert_eq!(array.len(), 2);

    unsafe {
        std::env::remove_var("PREFIX_MIDDLE_SUFFIX");
        std::env::remove_var("PREFIX_OTHER_SUFFIX");
    }
}

#[test]
fn test_filter_env_question_mark() {
    unsafe {
        std::env::set_var("VAR_A", "value_a");
        std::env::set_var("VAR_B", "value_b");
        std::env::set_var("VAR_AB", "value_ab");
    }

    let mut args = HashMap::new();
    args.insert("pattern".to_string(), Value::String("VAR_?".to_string()));

    let result = FilterEnv.call(&args).unwrap();
    let array = result.as_array().unwrap();

    // Should match VAR_A and VAR_B, but not VAR_AB (two characters)
    assert_eq!(array.len(), 2);

    unsafe {
        std::env::remove_var("VAR_A");
        std::env::remove_var("VAR_B");
        std::env::remove_var("VAR_AB");
    }
}

#[test]
fn test_filter_env_no_matches() {
    let mut args = HashMap::new();
    args.insert(
        "pattern".to_string(),
        Value::String("NONEXISTENT_PATTERN_*".to_string()),
    );

    let result = FilterEnv.call(&args).unwrap();
    let array = result.as_array().unwrap();

    assert_eq!(array.len(), 0);
}

#[test]
fn test_filter_env_no_pattern() {
    let args = HashMap::new();
    let result = FilterEnv.call(&args);

    assert!(result.is_err());
    assert!(result.err().unwrap().to_string().contains("pattern"));
}

#[test]
fn test_glob_to_regex() {
    // This is an internal function test - we test it indirectly through filter_env
    unsafe {
        std::env::set_var("SERVER_HOST", "localhost");
        std::env::set_var("SERVER_PORT", "8080");
        std::env::set_var("CLIENT_HOST", "example.com");
    }

    let mut args = HashMap::new();
    args.insert("pattern".to_string(), Value::String("SERVER_*".to_string()));

    let result = FilterEnv.call(&args).unwrap();
    let array = result.as_array().unwrap();

    assert_eq!(array.len(), 2);

    let keys: Vec<String> = array
        .iter()
        .map(|item| item.get("key").unwrap().as_str().unwrap().to_string())
        .collect();

    assert!(keys.contains(&"SERVER_HOST".to_string()));
    assert!(keys.contains(&"SERVER_PORT".to_string()));
    assert!(!keys.contains(&"CLIENT_HOST".to_string()));

    unsafe {
        std::env::remove_var("SERVER_HOST");
        std::env::remove_var("SERVER_PORT");
        std::env::remove_var("CLIENT_HOST");
    }
}
