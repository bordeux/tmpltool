//! Unit tests for hash filter-functions.
//!
//! Tests both function syntax and filter syntax to ensure both work identically.

use minijinja::value::Kwargs;
use minijinja::{Environment, Value};
use tmpltool::filter_functions::FilterFunction;
use tmpltool::filter_functions::hash::{Md5, Sha1, Sha256, Sha512};

// Helper to create kwargs for testing
fn create_kwargs(args: Vec<(&str, &str)>) -> Kwargs {
    Kwargs::from_iter(args.iter().map(|(k, v)| (*k, minijinja::Value::from(*v))))
}

// Helper to test both function and filter syntax produce the same result
fn assert_both_syntaxes_equal(
    env: &Environment,
    function_template: &str,
    filter_template: &str,
) -> String {
    let fn_result = env
        .render_str(function_template, ())
        .expect("Function syntax should work");
    let filter_result = env
        .render_str(filter_template, ())
        .expect("Filter syntax should work");
    assert_eq!(
        fn_result, filter_result,
        "Function and filter syntax should produce identical results"
    );
    fn_result
}

fn setup_env() -> Environment<'static> {
    let mut env = Environment::new();
    Md5::register(&mut env);
    Sha1::register(&mut env);
    Sha256::register(&mut env);
    Sha512::register(&mut env);
    env
}

// ============ MD5 Tests ============

#[test]
fn test_md5_function_syntax() {
    let kwargs = create_kwargs(vec![("string", "hello")]);
    let result = Md5::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "5d41402abc4b2a76b9719d911017c592");
}

#[test]
fn test_md5_filter_syntax() {
    let value = Value::from("hello");
    let kwargs = Kwargs::from_iter(std::iter::empty::<(&str, Value)>());
    let result = Md5::call_as_filter(&value, kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "5d41402abc4b2a76b9719d911017c592");
}

#[test]
fn test_md5_both_syntaxes_equal() {
    let env = setup_env();
    let result = assert_both_syntaxes_equal(
        &env,
        r#"{{ md5(string="hello") }}"#,
        r#"{{ "hello" | md5 }}"#,
    );
    assert_eq!(result, "5d41402abc4b2a76b9719d911017c592");
}

#[test]
fn test_md5_empty() {
    let kwargs = create_kwargs(vec![("string", "")]);
    let result = Md5::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "d41d8cd98f00b204e9800998ecf8427e");
}

// ============ SHA1 Tests ============

#[test]
fn test_sha1_function_syntax() {
    let kwargs = create_kwargs(vec![("string", "hello")]);
    let result = Sha1::call_as_function(kwargs).unwrap();
    assert_eq!(
        result.as_str().unwrap(),
        "aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d"
    );
}

#[test]
fn test_sha1_filter_syntax() {
    let value = Value::from("hello");
    let kwargs = Kwargs::from_iter(std::iter::empty::<(&str, Value)>());
    let result = Sha1::call_as_filter(&value, kwargs).unwrap();
    assert_eq!(
        result.as_str().unwrap(),
        "aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d"
    );
}

#[test]
fn test_sha1_both_syntaxes_equal() {
    let env = setup_env();
    let result = assert_both_syntaxes_equal(
        &env,
        r#"{{ sha1(string="hello") }}"#,
        r#"{{ "hello" | sha1 }}"#,
    );
    assert_eq!(result, "aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d");
}

// ============ SHA256 Tests ============

#[test]
fn test_sha256_function_syntax() {
    let kwargs = create_kwargs(vec![("string", "hello")]);
    let result = Sha256::call_as_function(kwargs).unwrap();
    assert_eq!(
        result.as_str().unwrap(),
        "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
    );
}

#[test]
fn test_sha256_filter_syntax() {
    let value = Value::from("hello");
    let kwargs = Kwargs::from_iter(std::iter::empty::<(&str, Value)>());
    let result = Sha256::call_as_filter(&value, kwargs).unwrap();
    assert_eq!(
        result.as_str().unwrap(),
        "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
    );
}

#[test]
fn test_sha256_both_syntaxes_equal() {
    let env = setup_env();
    let result = assert_both_syntaxes_equal(
        &env,
        r#"{{ sha256(string="hello") }}"#,
        r#"{{ "hello" | sha256 }}"#,
    );
    assert_eq!(
        result,
        "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
    );
}

// ============ SHA512 Tests ============

#[test]
fn test_sha512_function_syntax() {
    let kwargs = create_kwargs(vec![("string", "hello")]);
    let result = Sha512::call_as_function(kwargs).unwrap();
    assert_eq!(
        result.as_str().unwrap(),
        "9b71d224bd62f3785d96d46ad3ea3d73319bfbc2890caadae2dff72519673ca72323c3d99ba5c11d7c7acc6e14b8c5da0c4663475c2e5c3adef46f73bcdec043"
    );
}

#[test]
fn test_sha512_filter_syntax() {
    let value = Value::from("hello");
    let kwargs = Kwargs::from_iter(std::iter::empty::<(&str, Value)>());
    let result = Sha512::call_as_filter(&value, kwargs).unwrap();
    assert_eq!(
        result.as_str().unwrap(),
        "9b71d224bd62f3785d96d46ad3ea3d73319bfbc2890caadae2dff72519673ca72323c3d99ba5c11d7c7acc6e14b8c5da0c4663475c2e5c3adef46f73bcdec043"
    );
}

#[test]
fn test_sha512_both_syntaxes_equal() {
    let env = setup_env();
    let result = assert_both_syntaxes_equal(
        &env,
        r#"{{ sha512(string="hello") }}"#,
        r#"{{ "hello" | sha512 }}"#,
    );
    assert_eq!(
        result,
        "9b71d224bd62f3785d96d46ad3ea3d73319bfbc2890caadae2dff72519673ca72323c3d99ba5c11d7c7acc6e14b8c5da0c4663475c2e5c3adef46f73bcdec043"
    );
}

// ============ Error Tests ============

#[test]
fn test_md5_function_no_argument() {
    let kwargs = create_kwargs(vec![]);
    let result = Md5::call_as_function(kwargs);
    assert!(result.is_err());
}

#[test]
fn test_md5_filter_non_string() {
    let value = Value::from(123);
    let kwargs = Kwargs::from_iter(std::iter::empty::<(&str, Value)>());
    let result = Md5::call_as_filter(&value, kwargs);
    assert!(result.is_err());
}

// ============ Chaining Tests ============

#[test]
fn test_hash_chaining() {
    let env = setup_env();
    // Chain: "hello" | sha256 | md5
    let result = env
        .render_str(r#"{{ "hello" | sha256 | md5 }}"#, ())
        .expect("Chaining should work");

    // sha256("hello") = 2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824
    // md5 of that sha256 hash (pre-computed)
    assert_eq!(result, "ebde1b934fa81da163dcf4b7d7cfe18e");
}
