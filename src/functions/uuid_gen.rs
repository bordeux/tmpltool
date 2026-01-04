//! UUID generation function
//!
//! This module provides UUID generation with configurable versions:
//! - `uuid`: Generate UUID v4 (random) or v7 (time-ordered)

use super::metadata::{ArgumentMetadata, FunctionMetadata, SyntaxVariants};
use super::traits::Function;
use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};
use uuid::Uuid;

/// Generate UUID with configurable version
pub struct UuidGen;

impl Function for UuidGen {
    const NAME: &'static str = "uuid";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "uuid",
        category: "random",
        description: "Generate UUID (Universally Unique Identifier)",
        arguments: &[ArgumentMetadata {
            name: "version",
            arg_type: "string",
            required: false,
            default: Some("v4"),
            description: "UUID version: v4 (random) or v7 (time-ordered)",
        }],
        return_type: "string",
        examples: &[
            "{{ uuid() }}",
            "{{ uuid(version=\"v4\") }}",
            "{{ uuid(version=\"v7\") }}",
        ],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
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
}

// Legacy function export for backward compatibility during migration
