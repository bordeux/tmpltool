use std::collections::HashMap;
use tera::Value;
use tmpltool::functions::hash::{Md5, Sha1, Sha256, Sha512};

// Import the Function trait to use call()
use tera::Function;

#[test]
fn test_md5() {
    let mut args = HashMap::new();
    args.insert("string".to_string(), Value::String("hello".to_string()));

    let result = Md5.call(&args).unwrap();
    assert_eq!(result.as_str().unwrap(), "5d41402abc4b2a76b9719d911017c592");
}

#[test]
fn test_md5_empty() {
    let mut args = HashMap::new();
    args.insert("string".to_string(), Value::String("".to_string()));

    let result = Md5.call(&args).unwrap();
    assert_eq!(result.as_str().unwrap(), "d41d8cd98f00b204e9800998ecf8427e");
}

#[test]
fn test_sha1() {
    let mut args = HashMap::new();
    args.insert("string".to_string(), Value::String("hello".to_string()));

    let result = Sha1.call(&args).unwrap();
    assert_eq!(
        result.as_str().unwrap(),
        "aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d"
    );
}

#[test]
fn test_sha256() {
    let mut args = HashMap::new();
    args.insert("string".to_string(), Value::String("hello".to_string()));

    let result = Sha256.call(&args).unwrap();
    assert_eq!(
        result.as_str().unwrap(),
        "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
    );
}

#[test]
fn test_sha512() {
    let mut args = HashMap::new();
    args.insert("string".to_string(), Value::String("hello".to_string()));

    let result = Sha512.call(&args).unwrap();
    assert_eq!(
        result.as_str().unwrap(),
        "9b71d224bd62f3785d96d46ad3ea3d73319bfbc2890caadae2dff72519673ca72323c3d99ba5c11d7c7acc6e14b8c5da0c4663475c2e5c3adef46f73bcdec043"
    );
}

#[test]
fn test_md5_no_argument() {
    let args = HashMap::new();
    let result = Md5.call(&args);
    assert!(result.is_err());
}
