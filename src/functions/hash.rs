use minijinja::value::Kwargs;
/// Hash functions for cryptographic operations
///
/// Provides MD5, SHA1, SHA256, and SHA512 hashing functions.
use minijinja::{Error, Value};

/// MD5 hash function
///
/// # Example
///
/// ```jinja
/// {{ md5(string="hello") }}
/// ```
pub fn md5_fn(kwargs: Kwargs) -> Result<Value, Error> {
    use md5::{Digest, Md5 as Md5Hasher};

    let string: String = kwargs.get("string")?;

    let mut hasher = Md5Hasher::new();
    hasher.update(string.as_bytes());
    let result = hasher.finalize();
    let hash = format!("{:x}", result);

    Ok(Value::from(hash))
}

/// SHA1 hash function
///
/// # Example
///
/// ```jinja
/// {{ sha1(string="hello") }}
/// ```
pub fn sha1_fn(kwargs: Kwargs) -> Result<Value, Error> {
    use sha1::{Digest, Sha1 as Sha1Hasher};

    let string: String = kwargs.get("string")?;

    let mut hasher = Sha1Hasher::new();
    hasher.update(string.as_bytes());
    let result = hasher.finalize();
    let hash = format!("{:x}", result);

    Ok(Value::from(hash))
}

/// SHA256 hash function
///
/// # Example
///
/// ```jinja
/// {{ sha256(string="hello") }}
/// ```
pub fn sha256_fn(kwargs: Kwargs) -> Result<Value, Error> {
    use sha2::{Digest, Sha256 as Sha256Hasher};

    let string: String = kwargs.get("string")?;

    let mut hasher = Sha256Hasher::new();
    hasher.update(string.as_bytes());
    let result = hasher.finalize();
    let hash = format!("{:x}", result);

    Ok(Value::from(hash))
}

/// SHA512 hash function
///
/// # Example
///
/// ```jinja
/// {{ sha512(string="hello") }}
/// ```
pub fn sha512_fn(kwargs: Kwargs) -> Result<Value, Error> {
    use sha2::{Digest, Sha512 as Sha512Hasher};

    let string: String = kwargs.get("string")?;

    let mut hasher = Sha512Hasher::new();
    hasher.update(string.as_bytes());
    let result = hasher.finalize();
    let hash = format!("{:x}", result);

    Ok(Value::from(hash))
}
