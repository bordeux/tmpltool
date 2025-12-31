use minijinja::value::Kwargs;
use minijinja::Value;
use tmpltool::functions::encoding;

// Base64 tests
#[test]
fn test_base64_encode_basic() {
    let result = encoding::base64_encode_fn(Kwargs::from_iter(vec![(
        "string",
        Value::from("Hello World"),
    )]))
    .unwrap();
    assert_eq!(result.as_str().unwrap(), "SGVsbG8gV29ybGQ=");
}

#[test]
fn test_base64_encode_empty() {
    let result =
        encoding::base64_encode_fn(Kwargs::from_iter(vec![("string", Value::from(""))]))
            .unwrap();
    assert_eq!(result.as_str().unwrap(), "");
}

#[test]
fn test_base64_encode_special_chars() {
    let result = encoding::base64_encode_fn(Kwargs::from_iter(vec![(
        "string",
        Value::from("user:password"),
    )]))
    .unwrap();
    assert_eq!(result.as_str().unwrap(), "dXNlcjpwYXNzd29yZA==");
}

#[test]
fn test_base64_decode_basic() {
    let result = encoding::base64_decode_fn(Kwargs::from_iter(vec![(
        "string",
        Value::from("SGVsbG8gV29ybGQ="),
    )]))
    .unwrap();
    assert_eq!(result.as_str().unwrap(), "Hello World");
}

#[test]
fn test_base64_decode_empty() {
    let result =
        encoding::base64_decode_fn(Kwargs::from_iter(vec![("string", Value::from(""))]))
            .unwrap();
    assert_eq!(result.as_str().unwrap(), "");
}

#[test]
fn test_base64_decode_invalid() {
    let result = encoding::base64_decode_fn(Kwargs::from_iter(vec![(
        "string",
        Value::from("invalid!@#$"),
    )]));
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Failed to decode"));
}

#[test]
fn test_base64_roundtrip() {
    let original = "Test string with special chars: !@#$%^&*()";
    let encoded = encoding::base64_encode_fn(Kwargs::from_iter(vec![(
        "string",
        Value::from(original),
    )]))
    .unwrap();
    let decoded = encoding::base64_decode_fn(Kwargs::from_iter(vec![(
        "string",
        Value::from(encoded.as_str().unwrap()),
    )]))
    .unwrap();
    assert_eq!(decoded.as_str().unwrap(), original);
}

// Hex encoding tests
#[test]
fn test_hex_encode_basic() {
    let result =
        encoding::hex_encode_fn(Kwargs::from_iter(vec![("string", Value::from("Hello"))]))
            .unwrap();
    assert_eq!(result.as_str().unwrap(), "48656c6c6f");
}

#[test]
fn test_hex_encode_empty() {
    let result =
        encoding::hex_encode_fn(Kwargs::from_iter(vec![("string", Value::from(""))]))
            .unwrap();
    assert_eq!(result.as_str().unwrap(), "");
}

#[test]
fn test_hex_encode_numbers() {
    let result =
        encoding::hex_encode_fn(Kwargs::from_iter(vec![("string", Value::from("123"))]))
            .unwrap();
    assert_eq!(result.as_str().unwrap(), "313233");
}

#[test]
fn test_hex_decode_basic() {
    let result = encoding::hex_decode_fn(Kwargs::from_iter(vec![(
        "string",
        Value::from("48656c6c6f"),
    )]))
    .unwrap();
    assert_eq!(result.as_str().unwrap(), "Hello");
}

#[test]
fn test_hex_decode_uppercase() {
    let result = encoding::hex_decode_fn(Kwargs::from_iter(vec![(
        "string",
        Value::from("48656C6C6F"),
    )]))
    .unwrap();
    assert_eq!(result.as_str().unwrap(), "Hello");
}

#[test]
fn test_hex_decode_invalid() {
    let result =
        encoding::hex_decode_fn(Kwargs::from_iter(vec![("string", Value::from("xyz"))]));
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Failed to decode"));
}

#[test]
fn test_hex_decode_odd_length() {
    let result =
        encoding::hex_decode_fn(Kwargs::from_iter(vec![("string", Value::from("123"))]));
    assert!(result.is_err());
}

#[test]
fn test_hex_roundtrip() {
    let original = "Test 123";
    let encoded =
        encoding::hex_encode_fn(Kwargs::from_iter(vec![("string", Value::from(original))]))
            .unwrap();
    let decoded = encoding::hex_decode_fn(Kwargs::from_iter(vec![(
        "string",
        Value::from(encoded.as_str().unwrap()),
    )]))
    .unwrap();
    assert_eq!(decoded.as_str().unwrap(), original);
}

// Bcrypt tests
#[test]
fn test_bcrypt_basic() {
    let result = encoding::bcrypt_fn(Kwargs::from_iter(vec![(
        "password",
        Value::from("mypassword"),
    )]))
    .unwrap();
    let hash = result.as_str().unwrap();

    // Bcrypt hashes start with $2b$ or $2a$
    assert!(hash.starts_with("$2"));
    // Bcrypt hashes are 60 characters long
    assert_eq!(hash.len(), 60);
}

#[test]
fn test_bcrypt_with_rounds() {
    let result = encoding::bcrypt_fn(Kwargs::from_iter(vec![
        ("password", Value::from("test")),
        ("rounds", Value::from(10)),
    ]))
    .unwrap();
    let hash = result.as_str().unwrap();
    assert!(hash.starts_with("$2"));
}

#[test]
fn test_bcrypt_invalid_rounds_low() {
    let result = encoding::bcrypt_fn(Kwargs::from_iter(vec![
        ("password", Value::from("test")),
        ("rounds", Value::from(3)),
    ]));
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("between 4 and 31"));
}

#[test]
fn test_bcrypt_invalid_rounds_high() {
    let result = encoding::bcrypt_fn(Kwargs::from_iter(vec![
        ("password", Value::from("test")),
        ("rounds", Value::from(32)),
    ]));
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("between 4 and 31"));
}

#[test]
fn test_bcrypt_uniqueness() {
    // Same password should generate different hashes (due to random salt)
    let hash1 = encoding::bcrypt_fn(Kwargs::from_iter(vec![(
        "password",
        Value::from("test"),
    )]))
    .unwrap();
    let hash2 = encoding::bcrypt_fn(Kwargs::from_iter(vec![(
        "password",
        Value::from("test"),
    )]))
    .unwrap();

    assert_ne!(hash1.as_str().unwrap(), hash2.as_str().unwrap());
}

// Generate secret tests
#[test]
fn test_generate_secret_alphanumeric() {
    let result = encoding::generate_secret_fn(Kwargs::from_iter(vec![(
        "length",
        Value::from(32),
    )]))
    .unwrap();
    let secret = result.as_str().unwrap();

    assert_eq!(secret.len(), 32);
    // Check that it only contains alphanumeric characters
    assert!(secret.chars().all(|c| c.is_ascii_alphanumeric()));
}

#[test]
fn test_generate_secret_hex() {
    let result = encoding::generate_secret_fn(Kwargs::from_iter(vec![
        ("length", Value::from(16)),
        ("charset", Value::from("hex")),
    ]))
    .unwrap();
    let secret = result.as_str().unwrap();

    assert_eq!(secret.len(), 16);
    // Check that it only contains hex characters
    assert!(secret.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_generate_secret_base64() {
    let result = encoding::generate_secret_fn(Kwargs::from_iter(vec![
        ("length", Value::from(24)),
        ("charset", Value::from("base64")),
    ]))
    .unwrap();
    let secret = result.as_str().unwrap();

    assert_eq!(secret.len(), 24);
}

#[test]
fn test_generate_secret_invalid_length_zero() {
    let result =
        encoding::generate_secret_fn(Kwargs::from_iter(vec![("length", Value::from(0))]));
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("between 1 and 1024"));
}

#[test]
fn test_generate_secret_invalid_length_large() {
    let result =
        encoding::generate_secret_fn(Kwargs::from_iter(vec![("length", Value::from(2000))]));
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("between 1 and 1024"));
}

#[test]
fn test_generate_secret_invalid_charset() {
    let result = encoding::generate_secret_fn(Kwargs::from_iter(vec![
        ("length", Value::from(16)),
        ("charset", Value::from("invalid")),
    ]));
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Invalid charset"));
}

#[test]
fn test_generate_secret_uniqueness() {
    let secret1 = encoding::generate_secret_fn(Kwargs::from_iter(vec![(
        "length",
        Value::from(32),
    )]))
    .unwrap();
    let secret2 = encoding::generate_secret_fn(Kwargs::from_iter(vec![(
        "length",
        Value::from(32),
    )]))
    .unwrap();

    assert_ne!(secret1.as_str().unwrap(), secret2.as_str().unwrap());
}

// HMAC-SHA256 tests
#[test]
fn test_hmac_sha256_basic() {
    let result = encoding::hmac_sha256_fn(Kwargs::from_iter(vec![
        ("key", Value::from("secret")),
        ("message", Value::from("hello")),
    ]))
    .unwrap();

    // HMAC-SHA256 produces a 64-character hex string (32 bytes)
    assert_eq!(result.as_str().unwrap().len(), 64);
}

#[test]
fn test_hmac_sha256_deterministic() {
    let result1 = encoding::hmac_sha256_fn(Kwargs::from_iter(vec![
        ("key", Value::from("secret")),
        ("message", Value::from("hello")),
    ]))
    .unwrap();

    let result2 = encoding::hmac_sha256_fn(Kwargs::from_iter(vec![
        ("key", Value::from("secret")),
        ("message", Value::from("hello")),
    ]))
    .unwrap();

    // Same key and message should produce same HMAC
    assert_eq!(result1.as_str().unwrap(), result2.as_str().unwrap());
}

#[test]
fn test_hmac_sha256_different_keys() {
    let result1 = encoding::hmac_sha256_fn(Kwargs::from_iter(vec![
        ("key", Value::from("secret1")),
        ("message", Value::from("hello")),
    ]))
    .unwrap();

    let result2 = encoding::hmac_sha256_fn(Kwargs::from_iter(vec![
        ("key", Value::from("secret2")),
        ("message", Value::from("hello")),
    ]))
    .unwrap();

    // Different keys should produce different HMACs
    assert_ne!(result1.as_str().unwrap(), result2.as_str().unwrap());
}

#[test]
fn test_hmac_sha256_different_messages() {
    let result1 = encoding::hmac_sha256_fn(Kwargs::from_iter(vec![
        ("key", Value::from("secret")),
        ("message", Value::from("hello")),
    ]))
    .unwrap();

    let result2 = encoding::hmac_sha256_fn(Kwargs::from_iter(vec![
        ("key", Value::from("secret")),
        ("message", Value::from("world")),
    ]))
    .unwrap();

    // Different messages should produce different HMACs
    assert_ne!(result1.as_str().unwrap(), result2.as_str().unwrap());
}

// Escape HTML tests
#[test]
fn test_escape_html_basic() {
    let result = encoding::escape_html_fn(Kwargs::from_iter(vec![(
        "string",
        Value::from("<script>alert('XSS')</script>"),
    )]))
    .unwrap();
    assert_eq!(
        result.as_str().unwrap(),
        "&lt;script&gt;alert(&#x27;XSS&#x27;)&lt;/script&gt;"
    );
}

#[test]
fn test_escape_html_ampersand() {
    let result = encoding::escape_html_fn(Kwargs::from_iter(vec![(
        "string",
        Value::from("A & B"),
    )]))
    .unwrap();
    assert_eq!(result.as_str().unwrap(), "A &amp; B");
}

#[test]
fn test_escape_html_quotes() {
    let result = encoding::escape_html_fn(Kwargs::from_iter(vec![(
        "string",
        Value::from(r#"Say "hello""#),
    )]))
    .unwrap();
    assert_eq!(result.as_str().unwrap(), "Say &quot;hello&quot;");
}

#[test]
fn test_escape_html_all_entities() {
    let result = encoding::escape_html_fn(Kwargs::from_iter(vec![(
        "string",
        Value::from(r#"<tag attr="value">'text' & more</tag>"#),
    )]))
    .unwrap();
    assert_eq!(
        result.as_str().unwrap(),
        "&lt;tag attr=&quot;value&quot;&gt;&#x27;text&#x27; &amp; more&lt;/tag&gt;"
    );
}

#[test]
fn test_escape_html_empty() {
    let result =
        encoding::escape_html_fn(Kwargs::from_iter(vec![("string", Value::from(""))]))
            .unwrap();
    assert_eq!(result.as_str().unwrap(), "");
}

// Escape XML tests
#[test]
fn test_escape_xml_basic() {
    let result = encoding::escape_xml_fn(Kwargs::from_iter(vec![(
        "string",
        Value::from("<tag>content</tag>"),
    )]))
    .unwrap();
    assert_eq!(
        result.as_str().unwrap(),
        "&lt;tag&gt;content&lt;/tag&gt;"
    );
}

#[test]
fn test_escape_xml_apostrophe() {
    let result = encoding::escape_xml_fn(Kwargs::from_iter(vec![(
        "string",
        Value::from("it's working"),
    )]))
    .unwrap();
    assert_eq!(result.as_str().unwrap(), "it&apos;s working");
}

#[test]
fn test_escape_xml_all_entities() {
    let result = encoding::escape_xml_fn(Kwargs::from_iter(vec![(
        "string",
        Value::from(r#"<tag attr="value">'text' & more</tag>"#),
    )]))
    .unwrap();
    assert_eq!(
        result.as_str().unwrap(),
        "&lt;tag attr=&quot;value&quot;&gt;&apos;text&apos; &amp; more&lt;/tag&gt;"
    );
}

// Escape shell tests
#[test]
fn test_escape_shell_simple() {
    let result = encoding::escape_shell_fn(Kwargs::from_iter(vec![(
        "string",
        Value::from("hello"),
    )]))
    .unwrap();
    assert_eq!(result.as_str().unwrap(), "'hello'");
}

#[test]
fn test_escape_shell_with_spaces() {
    let result = encoding::escape_shell_fn(Kwargs::from_iter(vec![(
        "string",
        Value::from("hello world"),
    )]))
    .unwrap();
    assert_eq!(result.as_str().unwrap(), "'hello world'");
}

#[test]
fn test_escape_shell_with_quote() {
    let result = encoding::escape_shell_fn(Kwargs::from_iter(vec![(
        "string",
        Value::from("it's working"),
    )]))
    .unwrap();
    assert_eq!(result.as_str().unwrap(), "'it'\\''s working'");
}

#[test]
fn test_escape_shell_special_chars() {
    let result = encoding::escape_shell_fn(Kwargs::from_iter(vec![(
        "string",
        Value::from("$VAR && rm -rf /"),
    )]))
    .unwrap();
    assert_eq!(result.as_str().unwrap(), "'$VAR && rm -rf /'");
}

#[test]
fn test_escape_shell_empty() {
    let result =
        encoding::escape_shell_fn(Kwargs::from_iter(vec![("string", Value::from(""))]))
            .unwrap();
    assert_eq!(result.as_str().unwrap(), "''");
}
