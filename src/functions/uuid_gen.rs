/// UUID generation function
///
/// Generates UUIDs (Universally Unique Identifiers)
use std::collections::HashMap;
use tera::{Function, Result, Value, to_value};
use uuid::Uuid;

/// Generate UUID v4 (random)
pub struct UuidV4;

impl Function for UuidV4 {
    fn call(&self, _args: &HashMap<String, Value>) -> Result<Value> {
        let uuid = Uuid::new_v4();
        let uuid_string = uuid.to_string();

        to_value(&uuid_string)
            .map_err(|e| tera::Error::msg(format!("Failed to convert UUID: {}", e)))
    }
}
