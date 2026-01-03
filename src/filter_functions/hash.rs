//! Hash functions that support both function and filter syntax.
//!
//! # Function Syntax
//! ```jinja
//! {{ md5(string="hello") }}
//! {{ sha256(string="hello world") }}
//! ```
//!
//! # Filter Syntax
//! ```jinja
//! {{ "hello" | md5 }}
//! {{ "hello world" | sha256 }}
//! ```
//!
//! # Chaining
//! ```jinja
//! {{ "hello" | sha256 | md5 }}
//! ```

use super::FilterFunction;
use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};

/// MD5 hash function.
///
/// # Function Syntax
/// ```jinja
/// {{ md5(string="hello") }}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ "hello" | md5 }}
/// ```
pub struct Md5;

impl Md5 {
    fn hash(input: &str) -> String {
        use md5::{Digest, Md5 as Md5Hasher};
        let mut hasher = Md5Hasher::new();
        hasher.update(input.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

impl FilterFunction for Md5 {
    const NAME: &'static str = "md5";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let input: String = kwargs.get("string")?;
        Ok(Value::from(Self::hash(&input)))
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        let input = value
            .as_str()
            .ok_or_else(|| Error::new(ErrorKind::InvalidOperation, "md5 requires a string"))?;
        Ok(Value::from(Self::hash(input)))
    }
}

/// SHA1 hash function.
///
/// # Function Syntax
/// ```jinja
/// {{ sha1(string="hello") }}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ "hello" | sha1 }}
/// ```
pub struct Sha1;

impl Sha1 {
    fn hash(input: &str) -> String {
        use sha1::{Digest, Sha1 as Sha1Hasher};
        let mut hasher = Sha1Hasher::new();
        hasher.update(input.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

impl FilterFunction for Sha1 {
    const NAME: &'static str = "sha1";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let input: String = kwargs.get("string")?;
        Ok(Value::from(Self::hash(&input)))
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        let input = value
            .as_str()
            .ok_or_else(|| Error::new(ErrorKind::InvalidOperation, "sha1 requires a string"))?;
        Ok(Value::from(Self::hash(input)))
    }
}

/// SHA256 hash function.
///
/// # Function Syntax
/// ```jinja
/// {{ sha256(string="hello") }}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ "hello" | sha256 }}
/// ```
pub struct Sha256;

impl Sha256 {
    fn hash(input: &str) -> String {
        use sha2::{Digest, Sha256 as Sha256Hasher};
        let mut hasher = Sha256Hasher::new();
        hasher.update(input.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

impl FilterFunction for Sha256 {
    const NAME: &'static str = "sha256";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let input: String = kwargs.get("string")?;
        Ok(Value::from(Self::hash(&input)))
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        let input = value
            .as_str()
            .ok_or_else(|| Error::new(ErrorKind::InvalidOperation, "sha256 requires a string"))?;
        Ok(Value::from(Self::hash(input)))
    }
}

/// SHA512 hash function.
///
/// # Function Syntax
/// ```jinja
/// {{ sha512(string="hello") }}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ "hello" | sha512 }}
/// ```
pub struct Sha512;

impl Sha512 {
    fn hash(input: &str) -> String {
        use sha2::{Digest, Sha512 as Sha512Hasher};
        let mut hasher = Sha512Hasher::new();
        hasher.update(input.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

impl FilterFunction for Sha512 {
    const NAME: &'static str = "sha512";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let input: String = kwargs.get("string")?;
        Ok(Value::from(Self::hash(&input)))
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        let input = value
            .as_str()
            .ok_or_else(|| Error::new(ErrorKind::InvalidOperation, "sha512 requires a string"))?;
        Ok(Value::from(Self::hash(input)))
    }
}
