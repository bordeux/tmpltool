use minijinja::value::Kwargs;
use tmpltool::functions::environment::env_fn;

// Helper to create kwargs for testing
fn create_kwargs(args: Vec<(&str, &str)>) -> Kwargs {
    Kwargs::from_iter(args.iter().map(|(k, v)| (*k, minijinja::Value::from(*v))))
}

#[test]
fn test_env_fn_existing_var() {
    unsafe {
        std::env::set_var("TEST_ENV_VAR", "test_value");
    }

    let kwargs = create_kwargs(vec![("name", "TEST_ENV_VAR")]);
    let result = env_fn(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "test_value");

    unsafe {
        std::env::remove_var("TEST_ENV_VAR");
    }
}

#[test]
fn test_env_fn_with_default() {
    let kwargs = create_kwargs(vec![
        ("name", "NONEXISTENT_VAR_12345"),
        ("default", "default_value"),
    ]);
    let result = env_fn(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "default_value");
}

#[test]
fn test_env_fn_missing_no_default() {
    let kwargs = create_kwargs(vec![("name", "NONEXISTENT_VAR_67890")]);
    let result = env_fn(kwargs);
    assert!(result.is_err());
    assert!(
        result
            .err()
            .unwrap()
            .to_string()
            .contains("is not set and no default provided")
    );
}

#[test]
fn test_env_fn_empty_default() {
    let kwargs = create_kwargs(vec![("name", "NONEXISTENT_VAR_EMPTY"), ("default", "")]);
    let result = env_fn(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "");
}

#[test]
fn test_env_fn_override_with_default() {
    unsafe {
        std::env::set_var("TEST_OVERRIDE_VAR", "actual_value");
    }

    let kwargs = create_kwargs(vec![
        ("name", "TEST_OVERRIDE_VAR"),
        ("default", "default_value"),
    ]);
    let result = env_fn(kwargs).unwrap();
    assert_eq!(
        result.as_str().unwrap(),
        "actual_value",
        "Should use actual value, not default"
    );

    unsafe {
        std::env::remove_var("TEST_OVERRIDE_VAR");
    }
}

#[test]
fn test_env_fn_missing_name_param() {
    let kwargs: Kwargs = Kwargs::from_iter(Vec::<(&str, minijinja::Value)>::new());
    let result = env_fn(kwargs);
    assert!(result.is_err());
}

#[test]
fn test_env_fn_special_characters() {
    unsafe {
        std::env::set_var("TEST_SPECIAL_VAR", "value with spaces & symbols!");
    }

    let kwargs = create_kwargs(vec![("name", "TEST_SPECIAL_VAR")]);
    let result = env_fn(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "value with spaces & symbols!");

    unsafe {
        std::env::remove_var("TEST_SPECIAL_VAR");
    }
}

#[test]
fn test_env_fn_unicode_value() {
    unsafe {
        std::env::set_var("TEST_UNICODE_VAR", "Hello 世界 🌍");
    }

    let kwargs = create_kwargs(vec![("name", "TEST_UNICODE_VAR")]);
    let result = env_fn(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "Hello 世界 🌍");

    unsafe {
        std::env::remove_var("TEST_UNICODE_VAR");
    }
}

#[test]
fn test_env_fn_multiline_value() {
    unsafe {
        std::env::set_var("TEST_MULTILINE_VAR", "line1\nline2\nline3");
    }

    let kwargs = create_kwargs(vec![("name", "TEST_MULTILINE_VAR")]);
    let result = env_fn(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "line1\nline2\nline3");

    unsafe {
        std::env::remove_var("TEST_MULTILINE_VAR");
    }
}

#[test]
fn test_env_fn_empty_string_value() {
    unsafe {
        std::env::set_var("TEST_EMPTY_VAR", "");
    }

    let kwargs = create_kwargs(vec![("name", "TEST_EMPTY_VAR")]);
    let result = env_fn(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "");

    unsafe {
        std::env::remove_var("TEST_EMPTY_VAR");
    }
}
