/// Hash functions for cryptographic operations
///
/// Provides MD5, SHA1, SHA256, and SHA512 hashing functions.
use std::collections::HashMap;
use tera::{Function, Result, Value, to_value};

/// MD5 hash function
pub struct Md5;

impl Function for Md5 {
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value> {
        use md5::{Digest, Md5 as Md5Hasher};

        let input = args.get("string").and_then(|v| v.as_str()).ok_or_else(|| {
            tera::Error::msg("md5 requires a 'string' argument (e.g., string=\"hello\")")
        })?;

        let mut hasher = Md5Hasher::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        let hash = format!("{:x}", result);

        to_value(&hash).map_err(|e| tera::Error::msg(format!("Failed to convert hash: {}", e)))
    }
}

/// SHA1 hash function
pub struct Sha1;

impl Function for Sha1 {
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value> {
        use sha1::{Digest, Sha1 as Sha1Hasher};

        let input = args.get("string").and_then(|v| v.as_str()).ok_or_else(|| {
            tera::Error::msg("sha1 requires a 'string' argument (e.g., string=\"hello\")")
        })?;

        let mut hasher = Sha1Hasher::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        let hash = format!("{:x}", result);

        to_value(&hash).map_err(|e| tera::Error::msg(format!("Failed to convert hash: {}", e)))
    }
}

/// SHA256 hash function
pub struct Sha256;

impl Function for Sha256 {
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value> {
        use sha2::{Digest, Sha256 as Sha256Hasher};

        let input = args.get("string").and_then(|v| v.as_str()).ok_or_else(|| {
            tera::Error::msg("sha256 requires a 'string' argument (e.g., string=\"hello\")")
        })?;

        let mut hasher = Sha256Hasher::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        let hash = format!("{:x}", result);

        to_value(&hash).map_err(|e| tera::Error::msg(format!("Failed to convert hash: {}", e)))
    }
}

/// SHA512 hash function
pub struct Sha512;

impl Function for Sha512 {
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value> {
        use sha2::{Digest, Sha512 as Sha512Hasher};

        let input = args.get("string").and_then(|v| v.as_str()).ok_or_else(|| {
            tera::Error::msg("sha512 requires a 'string' argument (e.g., string=\"hello\")")
        })?;

        let mut hasher = Sha512Hasher::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        let hash = format!("{:x}", result);

        to_value(&hash).map_err(|e| tera::Error::msg(format!("Failed to convert hash: {}", e)))
    }
}
