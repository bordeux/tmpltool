/// UUID generation function
///
/// Generates UUIDs (Universally Unique Identifiers)
use minijinja::Value;
use uuid::Uuid;

/// Generate UUID v4 (random)
///
/// # Example
///
/// ```jinja
/// {{ uuid() }}
/// ```
pub fn uuid_fn() -> Value {
    let uuid = Uuid::new_v4();
    Value::from(uuid.to_string())
}
