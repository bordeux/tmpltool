//! Debugging and development functions for MiniJinja templates
//!
//! This module provides functions for:
//! - Debugging values: `debug`, `inspect`, `type_of`
//! - Assertions and validation: `assert`, `warn`, `abort`

use super::metadata::{ArgumentMetadata, FunctionMetadata, SyntaxVariants};
use super::traits::Function;
use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};

/// Print value to stderr and return it (for debugging)
pub struct Debug;

impl Function for Debug {
    const NAME: &'static str = "debug";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "debug",
        category: "debug",
        description: "Print value to stderr and return it unchanged (for debugging)",
        arguments: &[ArgumentMetadata {
            name: "value",
            arg_type: "any",
            required: true,
            default: None,
            description: "Value to debug print",
        }],
        return_type: "any",
        examples: &["{% set config = debug(value=parse_json(string='{\"port\": 8080}')) %}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let value: Value = kwargs.get("value")?;
        eprintln!("[DEBUG] {}", value);
        Ok(value)
    }
}

/// Get the type of a value
pub struct TypeOf;

impl Function for TypeOf {
    const NAME: &'static str = "type_of";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "type_of",
        category: "debug",
        description: "Get the type name of a value",
        arguments: &[ArgumentMetadata {
            name: "value",
            arg_type: "any",
            required: true,
            default: None,
            description: "Value to get type of",
        }],
        return_type: "string",
        examples: &[
            "{{ type_of(value=\"hello\") }}",
            "{{ type_of(value=123) }}",
            "{{ type_of(value=[1, 2, 3]) }}",
        ],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let value: Value = kwargs.get("value")?;

        let type_name = match value.kind() {
            minijinja::value::ValueKind::Undefined => "undefined",
            minijinja::value::ValueKind::None => "undefined",
            minijinja::value::ValueKind::Bool => "bool",
            minijinja::value::ValueKind::Number => "number",
            minijinja::value::ValueKind::String => "string",
            minijinja::value::ValueKind::Bytes => "bytes",
            minijinja::value::ValueKind::Seq => "array",
            minijinja::value::ValueKind::Map => "object",
            minijinja::value::ValueKind::Iterable => "iterable",
            _ => "unknown",
        };

        Ok(Value::from(type_name))
    }
}

/// Pretty-print value structure to stderr and return it
pub struct Inspect;

impl Function for Inspect {
    const NAME: &'static str = "inspect";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "inspect",
        category: "debug",
        description: "Pretty-print value structure to stderr and return it",
        arguments: &[ArgumentMetadata {
            name: "value",
            arg_type: "any",
            required: true,
            default: None,
            description: "Value to inspect",
        }],
        return_type: "any",
        examples: &["{% set data = inspect(value=filter_env(pattern=\"SERVER_*\")) %}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let value: Value = kwargs.get("value")?;
        eprintln!("[INSPECT] {:#?}", value);
        Ok(value)
    }
}

/// Assert a condition or fail with an error message
pub struct Assert;

impl Function for Assert {
    const NAME: &'static str = "assert";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "assert",
        category: "debug",
        description: "Assert a condition is true, or fail with an error message",
        arguments: &[
            ArgumentMetadata {
                name: "condition",
                arg_type: "boolean",
                required: true,
                default: None,
                description: "Condition to check",
            },
            ArgumentMetadata {
                name: "message",
                arg_type: "string",
                required: false,
                default: Some("Assertion failed"),
                description: "Error message if assertion fails",
            },
        ],
        return_type: "boolean",
        examples: &["{{ assert(condition=port != \"\", message=\"PORT is required\") }}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let condition: bool = kwargs.get("condition")?;
        let message: String = kwargs
            .get("message")
            .unwrap_or_else(|_| "Assertion failed".to_string());

        if !condition {
            return Err(Error::new(ErrorKind::InvalidOperation, message));
        }

        Ok(Value::from(true))
    }
}

/// Print a warning message to stderr and continue
pub struct Warn;

impl Function for Warn {
    const NAME: &'static str = "warn";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "warn",
        category: "debug",
        description: "Print a warning message to stderr and continue",
        arguments: &[ArgumentMetadata {
            name: "message",
            arg_type: "string",
            required: true,
            default: None,
            description: "Warning message to print",
        }],
        return_type: "string",
        examples: &["{{ warn(message=\"custom.conf not found, using defaults\") }}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let message: String = kwargs.get("message")?;
        eprintln!("[WARNING] {}", message);
        Ok(Value::from(""))
    }
}

/// Abort template rendering with an error message
pub struct Abort;

impl Function for Abort {
    const NAME: &'static str = "abort";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "abort",
        category: "debug",
        description: "Abort template rendering with an error message",
        arguments: &[ArgumentMetadata {
            name: "message",
            arg_type: "string",
            required: true,
            default: None,
            description: "Error message",
        }],
        return_type: "never",
        examples: &["{{ abort(message=\"Critical configuration file is missing\") }}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let message: String = kwargs.get("message")?;
        Err(Error::new(ErrorKind::InvalidOperation, message))
    }
}
