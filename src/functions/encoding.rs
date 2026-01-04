//! Encoding and security functions for MiniJinja templates
//!
//! This module provides functions for:
//! - Password hashing: `bcrypt`
//! - HMAC generation: `hmac_sha256`
//! - Secure random string generation: `generate_secret`
//!
//! Note: Base64, hex, and escape functions are now in filter_functions/encoding.rs
//! with dual function+filter syntax support.

use super::metadata::{ArgumentMetadata, FunctionMetadata, SyntaxVariants};
use super::traits::Function;
use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};

/// Generate a bcrypt hash for password storage
pub struct Bcrypt;

impl Function for Bcrypt {
    const NAME: &'static str = "bcrypt";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "bcrypt",
        category: "encoding",
        description: "Generate a bcrypt hash for password storage",
        arguments: &[
            ArgumentMetadata {
                name: "password",
                arg_type: "string",
                required: true,
                default: None,
                description: "Password to hash",
            },
            ArgumentMetadata {
                name: "rounds",
                arg_type: "integer",
                required: false,
                default: Some("12"),
                description: "Cost factor (4-31)",
            },
        ],
        return_type: "string",
        examples: &[
            "{{ bcrypt(password=\"mypassword\") }}",
            "{{ bcrypt(password=\"mypassword\", rounds=10) }}",
        ],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let password: String = kwargs.get("password")?;
        let rounds: u32 = kwargs.get("rounds").unwrap_or(12);

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
}

/// Generate a cryptographically secure random string
pub struct GenerateSecret;

impl Function for GenerateSecret {
    const NAME: &'static str = "generate_secret";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "generate_secret",
        category: "encoding",
        description: "Generate a cryptographically secure random string",
        arguments: &[
            ArgumentMetadata {
                name: "length",
                arg_type: "integer",
                required: true,
                default: None,
                description: "Length of the string (1-1024)",
            },
            ArgumentMetadata {
                name: "charset",
                arg_type: "string",
                required: false,
                default: Some("alphanumeric"),
                description: "Character set: alphanumeric, hex, or base64",
            },
        ],
        return_type: "string",
        examples: &[
            "{{ generate_secret(length=32) }}",
            "{{ generate_secret(length=16, charset=\"hex\") }}",
        ],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
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

        let charset: String = kwargs
            .get("charset")
            .unwrap_or_else(|_| "alphanumeric".to_string());

        use rand::Rng;
        let mut rng = rand::rng();

        let result = match charset.as_str() {
            "hex" => {
                let byte_count = length.div_ceil(2);
                let bytes: Vec<u8> = (0..byte_count).map(|_| rng.random()).collect();
                let hex_string = hex::encode(bytes);
                hex_string[..length].to_string()
            }
            "base64" => {
                let byte_count = (length * 3).div_ceil(4);
                let bytes: Vec<u8> = (0..byte_count).map(|_| rng.random()).collect();
                let b64_string =
                    base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &bytes);
                b64_string[..length].to_string()
            }
            "alphanumeric" => {
                const CHARSET: &[u8] =
                    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
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
                    format!(
                        "Invalid charset: '{}'. Must be 'alphanumeric', 'hex', or 'base64'",
                        charset
                    ),
                ));
            }
        };

        Ok(Value::from(result))
    }
}

/// Generate HMAC-SHA256 signature
pub struct HmacSha256;

impl Function for HmacSha256 {
    const NAME: &'static str = "hmac_sha256";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "hmac_sha256",
        category: "encoding",
        description: "Generate HMAC-SHA256 signature",
        arguments: &[
            ArgumentMetadata {
                name: "key",
                arg_type: "string",
                required: true,
                default: None,
                description: "Secret key",
            },
            ArgumentMetadata {
                name: "message",
                arg_type: "string",
                required: true,
                default: None,
                description: "Message to sign",
            },
        ],
        return_type: "string",
        examples: &["{{ hmac_sha256(key=\"secret\", message=\"hello\") }}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let key: String = kwargs.get("key")?;
        let message: String = kwargs.get("message")?;

        use hmac::{Hmac, Mac};
        use sha2::Sha256;

        type HmacSha256Inner = Hmac<Sha256>;

        let mut mac = HmacSha256Inner::new_from_slice(key.as_bytes()).map_err(|e| {
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
}
