use minijinja::value::Kwargs;
use tmpltool::functions::hash::{md5_fn, sha1_fn, sha256_fn, sha512_fn};

// Helper to create kwargs for testing
fn create_kwargs(args: Vec<(&str, &str)>) -> Kwargs {
    Kwargs::from_iter(args.into_iter().map(|(k, v)| (k, minijinja::Value::from(v))))
}

#[test]
fn test_md5() {
    let kwargs = create_kwargs(vec![("string", "hello")]);
    let result = md5_fn(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "5d41402abc4b2a76b9719d911017c592");
}

#[test]
fn test_md5_empty() {
    let kwargs = create_kwargs(vec![("string", "")]);
    let result = md5_fn(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "d41d8cd98f00b204e9800998ecf8427e");
}

#[test]
fn test_sha1() {
    let kwargs = create_kwargs(vec![("string", "hello")]);
    let result = sha1_fn(kwargs).unwrap();
    assert_eq!(
        result.as_str().unwrap(),
        "aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d"
    );
}

#[test]
fn test_sha256() {
    let kwargs = create_kwargs(vec![("string", "hello")]);
    let result = sha256_fn(kwargs).unwrap();
    assert_eq!(
        result.as_str().unwrap(),
        "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
    );
}

#[test]
fn test_sha512() {
    let kwargs = create_kwargs(vec![("string", "hello")]);
    let result = sha512_fn(kwargs).unwrap();
    assert_eq!(
        result.as_str().unwrap(),
        "9b71d224bd62f3785d96d46ad3ea3d73319bfbc2890caadae2dff72519673ca72323c3d99ba5c11d7c7acc6e14b8c5da0c4663475c2e5c3adef46f73bcdec043"
    );
}

#[test]
fn test_md5_no_argument() {
    let kwargs = create_kwargs(vec![]);
    let result = md5_fn(kwargs);
    assert!(result.is_err());
}
