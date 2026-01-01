/// UUID generation function
///
/// Generates UUIDs (Universally Unique Identifiers)
use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};
use uuid::Uuid;

/// Generate UUID with configurable version
///
/// # Arguments
///
/// * `version` - UUID version to generate: "v4" (default) or "v7"
///
/// # Example
///
/// ```jinja
/// {{ uuid() }}
/// {{ uuid(version="v4") }}
/// {{ uuid(version="v7") }}
/// ```
pub fn uuid_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let version: Option<String> = kwargs.get("version")?;
    let version = version.as_deref().unwrap_or("v4");

    let uuid = match version {
        "v4" => Uuid::new_v4(),
        "v7" => Uuid::now_v7(),
        _ => {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                format!(
                    "Invalid UUID version '{}'. Supported versions: v4, v7",
                    version
                ),
            ));
        }
    };

    Ok(Value::from(uuid.to_string()))
}
