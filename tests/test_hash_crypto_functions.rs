mod common;

use common::{cleanup_test_file, get_test_file_path};
use regex::Regex;
use std::fs;
use tmpltool::render_template;

#[test]
fn test_md5_function() {
    let template_content = r#"{{ md5(string="hello") }}"#;
    let template_path = get_test_file_path("template_md5.txt");
    let output_path = get_test_file_path("output_md5.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
    );

    assert!(
        result.is_ok(),
        "MD5 template rendering failed: {:?}",
        result.err()
    );

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "5d41402abc4b2a76b9719d911017c592");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_sha1_function() {
    let template_content = r#"{{ sha1(string="test") }}"#;
    let template_path = get_test_file_path("template_sha1.txt");
    let output_path = get_test_file_path("output_sha1.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
    );

    assert!(
        result.is_ok(),
        "SHA1 template rendering failed: {:?}",
        result.err()
    );

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output, "a94a8fe5ccb19ba61c4c0873d391e987982fbbd3");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_sha256_function() {
    let template_content = r#"{{ sha256(string="tmpltool") }}"#;
    let template_path = get_test_file_path("template_sha256.txt");
    let output_path = get_test_file_path("output_sha256.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
    );

    assert!(
        result.is_ok(),
        "SHA256 template rendering failed: {:?}",
        result.err()
    );

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(
        output,
        "5eb389e31748154d04ff7be14bec47d2a72d26c8f36ec7feb6236cc860b9fbe2"
    );

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_sha512_function() {
    let template_content = r#"{{ sha512(string="secure") }}"#;
    let template_path = get_test_file_path("template_sha512.txt");
    let output_path = get_test_file_path("output_sha512.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
    );

    assert!(
        result.is_ok(),
        "SHA512 template rendering failed: {:?}",
        result.err()
    );

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(
        output,
        "66a2d78a8cd30f00d0f8e43434731ce3c9351ce9c7f66bc1cd2e105edc994be0a9106c85bb7eed09a421de36f4af0dc2f24bdc64f8645ce7efd3fd909b93785e"
    );

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_hash_with_env_variable() {
    let template_content =
        r#"{{ sha256(string=get_env(name="TEST_HASH_VAR", default="default")) }}"#;
    let template_path = get_test_file_path("template_hash_env.txt");
    let output_path = get_test_file_path("output_hash_env.txt");

    fs::write(&template_path, template_content).unwrap();

    // Set environment variable
    unsafe {
        std::env::set_var("TEST_HASH_VAR", "mypassword");
    }

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
    );

    assert!(
        result.is_ok(),
        "Hash with env template rendering failed: {:?}",
        result.err()
    );

    let output = fs::read_to_string(&output_path).unwrap();
    // SHA256 of "mypassword"
    assert_eq!(
        output,
        "89e01536ac207279409d4de1e5253e01f4a1769e696db0d6062ca9b8f56767c8"
    );

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);

    unsafe {
        std::env::remove_var("TEST_HASH_VAR");
    }
}

#[test]
fn test_uuid_function() {
    let template_content = r#"{{ uuid() }}"#;
    let template_path = get_test_file_path("template_uuid.txt");
    let output_path = get_test_file_path("output_uuid.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
    );

    assert!(
        result.is_ok(),
        "UUID template rendering failed: {:?}",
        result.err()
    );

    let output = fs::read_to_string(&output_path).unwrap();

    // Validate UUID v4 format: xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx
    let uuid_pattern =
        Regex::new(r"^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$")
            .unwrap();
    assert!(
        uuid_pattern.is_match(&output),
        "Invalid UUID format: {}",
        output
    );

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_uuid_uniqueness() {
    let template_content = r#"{{ uuid() }}
{{ uuid() }}
{{ uuid() }}"#;
    let template_path = get_test_file_path("template_uuid_unique.txt");
    let output_path = get_test_file_path("output_uuid_unique.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
    );

    assert!(
        result.is_ok(),
        "UUID uniqueness template rendering failed: {:?}",
        result.err()
    );

    let output = fs::read_to_string(&output_path).unwrap();
    let uuids: Vec<&str> = output.lines().collect();

    assert_eq!(uuids.len(), 3);
    assert_ne!(uuids[0], uuids[1]);
    assert_ne!(uuids[1], uuids[2]);
    assert_ne!(uuids[0], uuids[2]);

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_random_string_basic() {
    let template_content = r#"{{ random_string(length=16) }}"#;
    let template_path = get_test_file_path("template_random_basic.txt");
    let output_path = get_test_file_path("output_random_basic.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
    );

    assert!(
        result.is_ok(),
        "Random string template rendering failed: {:?}",
        result.err()
    );

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output.len(), 16);

    // Default charset is alphanumeric
    for ch in output.chars() {
        assert!(
            ch.is_ascii_alphanumeric(),
            "Invalid character in random string: {}",
            ch
        );
    }

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_random_string_lowercase() {
    let template_content = r#"{{ random_string(length=10, charset="lowercase") }}"#;
    let template_path = get_test_file_path("template_random_lower.txt");
    let output_path = get_test_file_path("output_random_lower.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
    );

    assert!(
        result.is_ok(),
        "Random string lowercase template rendering failed: {:?}",
        result.err()
    );

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output.len(), 10);

    for ch in output.chars() {
        assert!(
            ch.is_ascii_lowercase(),
            "Invalid character in lowercase random string: {}",
            ch
        );
    }

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_random_string_uppercase() {
    let template_content = r#"{{ random_string(length=8, charset="uppercase") }}"#;
    let template_path = get_test_file_path("template_random_upper.txt");
    let output_path = get_test_file_path("output_random_upper.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
    );

    assert!(
        result.is_ok(),
        "Random string uppercase template rendering failed: {:?}",
        result.err()
    );

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output.len(), 8);

    for ch in output.chars() {
        assert!(
            ch.is_ascii_uppercase(),
            "Invalid character in uppercase random string: {}",
            ch
        );
    }

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_random_string_numeric() {
    let template_content = r#"{{ random_string(length=6, charset="numeric") }}"#;
    let template_path = get_test_file_path("template_random_num.txt");
    let output_path = get_test_file_path("output_random_num.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
    );

    assert!(
        result.is_ok(),
        "Random string numeric template rendering failed: {:?}",
        result.err()
    );

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output.len(), 6);

    for ch in output.chars() {
        assert!(
            ch.is_ascii_digit(),
            "Invalid character in numeric random string: {}",
            ch
        );
    }

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_random_string_hex() {
    let template_content = r#"{{ random_string(length=16, charset="hex") }}"#;
    let template_path = get_test_file_path("template_random_hex.txt");
    let output_path = get_test_file_path("output_random_hex.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
    );

    assert!(
        result.is_ok(),
        "Random string hex template rendering failed: {:?}",
        result.err()
    );

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output.len(), 16);

    for ch in output.chars() {
        assert!(
            ch.is_ascii_hexdigit() && !ch.is_ascii_uppercase(),
            "Invalid character in hex random string: {}",
            ch
        );
    }

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_random_string_custom_charset() {
    let template_content = r#"{{ random_string(length=20, charset="abc123") }}"#;
    let template_path = get_test_file_path("template_random_custom.txt");
    let output_path = get_test_file_path("output_random_custom.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
    );

    assert!(
        result.is_ok(),
        "Random string custom charset template rendering failed: {:?}",
        result.err()
    );

    let output = fs::read_to_string(&output_path).unwrap();
    assert_eq!(output.len(), 20);

    for ch in output.chars() {
        assert!(
            "abc123".contains(ch),
            "Invalid character in custom charset random string: {}",
            ch
        );
    }

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_random_string_uniqueness() {
    let template_content = r#"{{ random_string(length=32) }}
{{ random_string(length=32) }}"#;
    let template_path = get_test_file_path("template_random_unique.txt");
    let output_path = get_test_file_path("output_random_unique.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
    );

    assert!(
        result.is_ok(),
        "Random string uniqueness template rendering failed: {:?}",
        result.err()
    );

    let output = fs::read_to_string(&output_path).unwrap();
    let strings: Vec<&str> = output.lines().collect();

    assert_eq!(strings.len(), 2);
    assert_ne!(strings[0], strings[1], "Random strings should be unique");

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_combined_hash_crypto_functions() {
    let template_content = r#"# Security Configuration
secret_key: {{ random_string(length=64) }}
api_token: {{ random_string(length=32, charset="hex") }}
instance_id: {{ uuid() }}
password_hash: {{ sha256(string="admin123") }}
checksum: {{ md5(string="config-v1") }}"#;

    let template_path = get_test_file_path("template_combined.txt");
    let output_path = get_test_file_path("output_combined.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
    );

    assert!(
        result.is_ok(),
        "Combined functions template rendering failed: {:?}",
        result.err()
    );

    let output = fs::read_to_string(&output_path).unwrap();

    // Verify all expected sections are present
    assert!(output.contains("secret_key:"));
    assert!(output.contains("api_token:"));
    assert!(output.contains("instance_id:"));
    assert!(output.contains("password_hash:"));
    assert!(output.contains("checksum:"));

    // Verify password hash is correct
    assert!(output.contains("240be518fabd2724ddb6f04eeb1da5967448d7e831c08c8fa822809f74c720a9"));

    // Verify checksum is correct
    assert!(output.contains("dec7d66f96dddff3a20bf58b62a2ef8f"));

    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}

#[test]
fn test_hash_function_missing_argument() {
    let template_content = r#"{{ md5() }}"#;
    let template_path = get_test_file_path("template_md5_error.txt");
    let output_path = get_test_file_path("output_md5_error.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
    );

    assert!(result.is_err(), "MD5 without argument should fail");

    let error = result.err().unwrap();
    let error_msg = error.to_string();
    assert!(
        error_msg.contains("md5 requires a 'string' argument"),
        "Error message should mention missing argument: {}",
        error_msg
    );

    cleanup_test_file(&template_path);
}

#[test]
fn test_random_string_missing_length() {
    let template_content = r#"{{ random_string() }}"#;
    let template_path = get_test_file_path("template_random_error.txt");
    let output_path = get_test_file_path("output_random_error.txt");

    fs::write(&template_path, template_content).unwrap();

    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
    );

    assert!(result.is_err(), "random_string without length should fail");

    let error = result.err().unwrap();
    let error_msg = error.to_string();
    assert!(
        error_msg.contains("random_string requires a 'length' argument"),
        "Error message should mention missing length: {}",
        error_msg
    );

    cleanup_test_file(&template_path);
}
