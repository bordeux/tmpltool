//! Encoding and security functions for MiniJinja templates
//!
//! This module provides functions for:
//! - Base64 encoding/decoding
//! - Hexadecimal encoding/decoding
//! - Password hashing (bcrypt)
//! - HMAC generation
//! - Secure random string generation
//! - HTML/XML/Shell escaping

use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};

/// Encode a string to Base64
///
/// # Arguments
///
/// * `string` (required) - String to encode
///
/// # Returns
///
/// Returns the Base64-encoded string
///
/// # Example
///
/// ```jinja
/// {{ base64_encode(string="Hello World") }}  => SGVsbG8gV29ybGQ=
/// {{ base64_encode(string="user:password") }}  => dXNlcjpwYXNzd29yZA==
/// ```
pub fn base64_encode_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let input: String = kwargs.get("string")?;
    let encoded = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, input.as_bytes());
    Ok(Value::from(encoded))
}

/// Decode a Base64-encoded string
///
/// # Arguments
///
/// * `string` (required) - Base64 string to decode
///
/// # Returns
///
/// Returns the decoded string
///
/// # Example
///
/// ```jinja
/// {{ base64_decode(string="SGVsbG8gV29ybGQ=") }}  => Hello World
/// ```
pub fn base64_decode_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let input: String = kwargs.get("string")?;

    let decoded_bytes = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, input.as_bytes())
        .map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Failed to decode base64: {}", e),
            )
        })?;

    let decoded_string = String::from_utf8(decoded_bytes).map_err(|e| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("Decoded base64 is not valid UTF-8: {}", e),
        )
    })?;

    Ok(Value::from(decoded_string))
}

/// Encode a string to hexadecimal
///
/// # Arguments
///
/// * `string` (required) - String to encode
///
/// # Returns
///
/// Returns the hexadecimal-encoded string (lowercase)
///
/// # Example
///
/// ```jinja
/// {{ hex_encode(string="Hello") }}  => 48656c6c6f
/// {{ hex_encode(string="ABC") }}  => 414243
/// ```
pub fn hex_encode_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let input: String = kwargs.get("string")?;
    let encoded = hex::encode(input.as_bytes());
    Ok(Value::from(encoded))
}

/// Decode a hexadecimal-encoded string
///
/// # Arguments
///
/// * `string` (required) - Hexadecimal string to decode
///
/// # Returns
///
/// Returns the decoded string
///
/// # Example
///
/// ```jinja
/// {{ hex_decode(string="48656c6c6f") }}  => Hello
/// {{ hex_decode(string="414243") }}  => ABC
/// ```
pub fn hex_decode_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let input: String = kwargs.get("string")?;

    let decoded_bytes = hex::decode(&input).map_err(|e| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("Failed to decode hex: {}", e),
        )
    })?;

    let decoded_string = String::from_utf8(decoded_bytes).map_err(|e| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("Decoded hex is not valid UTF-8: {}", e),
        )
    })?;

    Ok(Value::from(decoded_string))
}

/// Generate a bcrypt hash for password storage
///
/// # Arguments
///
/// * `password` (required) - Password to hash
/// * `rounds` (optional) - Cost factor (4-31, default: 12)
///
/// # Returns
///
/// Returns the bcrypt hash string
///
/// # Example
///
/// ```jinja
/// {{ bcrypt(password="mypassword") }}
/// {{ bcrypt(password="mypassword", rounds=10) }}
/// ```
pub fn bcrypt_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let password: String = kwargs.get("password")?;
    let rounds: u32 = kwargs.get("rounds").unwrap_or(12);

    // Validate rounds (bcrypt supports 4-31)
    if !(4..=31).contains(&rounds) {
        return Err(Error::new(
            ErrorKind::InvalidOperation,
            format!("Bcrypt rounds must be between 4 and 31, got {}", rounds),
        ));
    }

    let hash = bcrypt::hash(password.as_bytes(), rounds).map_err(|e| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("Failed to generate bcrypt hash: {}", e),
        )
    })?;

    Ok(Value::from(hash))
}

/// Generate a cryptographically secure random string
///
/// # Arguments
///
/// * `length` (required) - Length of the string to generate
/// * `charset` (optional) - Character set: "alphanumeric" (default), "hex", "base64"
///
/// # Returns
///
/// Returns a cryptographically secure random string
///
/// # Example
///
/// ```jinja
/// {{ generate_secret(length=32) }}
/// {{ generate_secret(length=16, charset="hex") }}
/// {{ generate_secret(length=24, charset="base64") }}
/// ```
pub fn generate_secret_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let length: usize = kwargs.get::<i64>("length").and_then(|l| {
        if l > 0 && l <= 1024 {
            Ok(l as usize)
        } else {
            Err(Error::new(
                ErrorKind::InvalidOperation,
                format!("Length must be between 1 and 1024, got {}", l),
            ))
        }
    })?;

    let charset: String = kwargs.get("charset").unwrap_or_else(|_| "alphanumeric".to_string());

    use rand::Rng;
    let mut rng = rand::rng();

    let result = match charset.as_str() {
        "hex" => {
            // Generate random bytes and convert to hex
            let byte_count = (length + 1) / 2;
            let bytes: Vec<u8> = (0..byte_count).map(|_| rng.random()).collect();
            let hex_string = hex::encode(bytes);
            hex_string[..length].to_string()
        }
        "base64" => {
            // Generate random bytes and convert to base64
            let byte_count = (length * 3 + 3) / 4;
            let bytes: Vec<u8> = (0..byte_count).map(|_| rng.random()).collect();
            let b64_string = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &bytes);
            b64_string[..length].to_string()
        }
        "alphanumeric" => {
            // Generate alphanumeric string
            const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
            (0..length)
                .map(|_| {
                    let idx = rng.random_range(0..CHARSET.len());
                    CHARSET[idx] as char
                })
                .collect()
        }
        _ => {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                format!("Invalid charset: '{}'. Must be 'alphanumeric', 'hex', or 'base64'", charset),
            ));
        }
    };

    Ok(Value::from(result))
}

/// Generate HMAC-SHA256 signature
///
/// # Arguments
///
/// * `key` (required) - Secret key
/// * `message` (required) - Message to sign
///
/// # Returns
///
/// Returns the HMAC-SHA256 signature as a hexadecimal string
///
/// # Example
///
/// ```jinja
/// {{ hmac_sha256(key="secret", message="hello") }}
/// {% set signature = hmac_sha256(key="api_key", message="data") %}
/// ```
pub fn hmac_sha256_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let key: String = kwargs.get("key")?;
    let message: String = kwargs.get("message")?;

    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    type HmacSha256 = Hmac<Sha256>;

    let mut mac = HmacSha256::new_from_slice(key.as_bytes()).map_err(|e| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("Failed to create HMAC: {}", e),
        )
    })?;

    mac.update(message.as_bytes());
    let result = mac.finalize();
    let signature = hex::encode(result.into_bytes());

    Ok(Value::from(signature))
}

/// Escape HTML entities
///
/// # Arguments
///
/// * `string` (required) - String to escape
///
/// # Returns
///
/// Returns the HTML-escaped string
///
/// # Example
///
/// ```jinja
/// {{ escape_html(string='<script>alert("XSS")</script>') }}
/// => &lt;script&gt;alert(&quot;XSS&quot;)&lt;/script&gt;
/// ```
pub fn escape_html_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let input: String = kwargs.get("string")?;

    let escaped = input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;");

    Ok(Value::from(escaped))
}

/// Escape XML entities
///
/// # Arguments
///
/// * `string` (required) - String to escape
///
/// # Returns
///
/// Returns the XML-escaped string
///
/// # Example
///
/// ```jinja
/// {{ escape_xml(string='<tag attr="value">text & more</tag>') }}
/// => &lt;tag attr=&quot;value&quot;&gt;text &amp; more&lt;/tag&gt;
/// ```
pub fn escape_xml_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let input: String = kwargs.get("string")?;

    // XML has the same entities as HTML but only uses a subset
    let escaped = input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;");

    Ok(Value::from(escaped))
}

/// Escape shell command arguments for safe execution
///
/// # Arguments
///
/// * `string` (required) - String to escape
///
/// # Returns
///
/// Returns the shell-escaped string (single-quoted)
///
/// # Example
///
/// ```jinja
/// {{ escape_shell(string="hello world") }}  => 'hello world'
/// {{ escape_shell(string="it's working") }}  => 'it'\''s working'
/// ```
pub fn escape_shell_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let input: String = kwargs.get("string")?;

    // Use single quotes and escape any single quotes in the input
    // The technique is to end the quote, add an escaped quote, and start a new quote
    let escaped = input.replace('\'', "'\\''");

    Ok(Value::from(format!("'{}'", escaped)))
}
