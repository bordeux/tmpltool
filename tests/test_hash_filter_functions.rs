//! Tests for hash filter-functions (md5, sha1, sha256, sha512).
//!
//! Tests both function and filter syntax.

use minijinja::Value;
use minijinja::value::Kwargs;
use tmpltool::filter_functions::FilterFunction;
use tmpltool::filter_functions::hash::{Md5, Sha1, Sha256, Sha512};

/// Helper to create empty kwargs
fn empty_kwargs() -> Kwargs {
    Kwargs::from_iter(Vec::<(&str, Value)>::new())
}

// ============================================
// MD5 tests
// ============================================

#[test]
fn test_md5_filter_syntax() {
    let result = Md5::call_as_filter(&Value::from("hello"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "5d41402abc4b2a76b9719d911017c592");
}

#[test]
fn test_md5_function_syntax() {
    let kwargs = Kwargs::from_iter(vec![("string", Value::from("hello"))]);
    let result = Md5::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "5d41402abc4b2a76b9719d911017c592");
}

#[test]
fn test_md5_empty_string() {
    let result = Md5::call_as_filter(&Value::from(""), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "d41d8cd98f00b204e9800998ecf8427e");
}

#[test]
fn test_md5_unicode() {
    let result = Md5::call_as_filter(&Value::from("héllo"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap().len(), 32);
}

#[test]
fn test_md5_error_not_string() {
    let result = Md5::call_as_filter(&Value::from(123), empty_kwargs());
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires a string")
    );
}

// ============================================
// SHA1 tests
// ============================================

#[test]
fn test_sha1_filter_syntax() {
    let result = Sha1::call_as_filter(&Value::from("hello"), empty_kwargs()).unwrap();
    assert_eq!(
        result.as_str().unwrap(),
        "aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d"
    );
}

#[test]
fn test_sha1_function_syntax() {
    let kwargs = Kwargs::from_iter(vec![("string", Value::from("hello"))]);
    let result = Sha1::call_as_function(kwargs).unwrap();
    assert_eq!(
        result.as_str().unwrap(),
        "aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d"
    );
}

#[test]
fn test_sha1_empty_string() {
    let result = Sha1::call_as_filter(&Value::from(""), empty_kwargs()).unwrap();
    assert_eq!(
        result.as_str().unwrap(),
        "da39a3ee5e6b4b0d3255bfef95601890afd80709"
    );
}

#[test]
fn test_sha1_error_not_string() {
    let result = Sha1::call_as_filter(&Value::from(123), empty_kwargs());
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires a string")
    );
}

// ============================================
// SHA256 tests
// ============================================

#[test]
fn test_sha256_filter_syntax() {
    let result = Sha256::call_as_filter(&Value::from("hello"), empty_kwargs()).unwrap();
    assert_eq!(
        result.as_str().unwrap(),
        "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
    );
}

#[test]
fn test_sha256_function_syntax() {
    let kwargs = Kwargs::from_iter(vec![("string", Value::from("hello"))]);
    let result = Sha256::call_as_function(kwargs).unwrap();
    assert_eq!(
        result.as_str().unwrap(),
        "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
    );
}

#[test]
fn test_sha256_empty_string() {
    let result = Sha256::call_as_filter(&Value::from(""), empty_kwargs()).unwrap();
    assert_eq!(
        result.as_str().unwrap(),
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
}

#[test]
fn test_sha256_error_not_string() {
    let result = Sha256::call_as_filter(&Value::from(123), empty_kwargs());
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires a string")
    );
}

// ============================================
// SHA512 tests
// ============================================

#[test]
fn test_sha512_filter_syntax() {
    let result = Sha512::call_as_filter(&Value::from("hello"), empty_kwargs()).unwrap();
    assert_eq!(
        result.as_str().unwrap(),
        "9b71d224bd62f3785d96d46ad3ea3d73319bfbc2890caadae2dff72519673ca72323c3d99ba5c11d7c7acc6e14b8c5da0c4663475c2e5c3adef46f73bcdec043"
    );
}

#[test]
fn test_sha512_function_syntax() {
    let kwargs = Kwargs::from_iter(vec![("string", Value::from("hello"))]);
    let result = Sha512::call_as_function(kwargs).unwrap();
    assert_eq!(
        result.as_str().unwrap(),
        "9b71d224bd62f3785d96d46ad3ea3d73319bfbc2890caadae2dff72519673ca72323c3d99ba5c11d7c7acc6e14b8c5da0c4663475c2e5c3adef46f73bcdec043"
    );
}

#[test]
fn test_sha512_empty_string() {
    let result = Sha512::call_as_filter(&Value::from(""), empty_kwargs()).unwrap();
    // SHA512 of empty string
    assert_eq!(result.as_str().unwrap().len(), 128);
}

#[test]
fn test_sha512_error_not_string() {
    let result = Sha512::call_as_filter(&Value::from(123), empty_kwargs());
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires a string")
    );
}
