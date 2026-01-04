//! Predicate functions for MiniJinja templates
//!
//! This module provides predicate functions for checking conditions:
//! - Array predicates: `array_any`, `array_all`, `array_contains`
//! - String predicates: `starts_with`, `ends_with`

use super::metadata::{ArgumentMetadata, FunctionMetadata, SyntaxVariants};
use super::traits::Function;
use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};

/// Check if any element in array matches a value
pub struct ArrayAny;

impl Function for ArrayAny {
    const NAME: &'static str = "array_any";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "array_any",
        category: "predicate",
        description: "Check if any element in array equals the predicate value",
        arguments: &[
            ArgumentMetadata {
                name: "array",
                arg_type: "array",
                required: true,
                default: None,
                description: "The array to check",
            },
            ArgumentMetadata {
                name: "predicate",
                arg_type: "any",
                required: true,
                default: None,
                description: "Value to compare against",
            },
        ],
        return_type: "boolean",
        examples: &["{% if array_any(array=[1, 2, 5, 8], predicate=5) %}Found 5!{% endif %}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let array: Value = kwargs.get("array")?;
        let predicate: Value = kwargs.get("predicate")?;

        if !matches!(array.kind(), minijinja::value::ValueKind::Seq) {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                "array_any requires an array",
            ));
        }

        if let Ok(seq) = array.try_iter() {
            for item in seq {
                if item == predicate {
                    return Ok(Value::from(true));
                }
            }
        }

        Ok(Value::from(false))
    }
}

/// Check if all elements in array match a value
pub struct ArrayAll;

impl Function for ArrayAll {
    const NAME: &'static str = "array_all";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "array_all",
        category: "predicate",
        description: "Check if all elements in array equal the predicate value",
        arguments: &[
            ArgumentMetadata {
                name: "array",
                arg_type: "array",
                required: true,
                default: None,
                description: "The array to check",
            },
            ArgumentMetadata {
                name: "predicate",
                arg_type: "any",
                required: true,
                default: None,
                description: "Value to compare against",
            },
        ],
        return_type: "boolean",
        examples: &["{% if array_all(array=[5, 5, 5], predicate=5) %}All are 5!{% endif %}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let array: Value = kwargs.get("array")?;
        let predicate: Value = kwargs.get("predicate")?;

        if !matches!(array.kind(), minijinja::value::ValueKind::Seq) {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                "array_all requires an array",
            ));
        }

        if let Ok(seq) = array.try_iter() {
            let items: Vec<_> = seq.collect();
            if items.is_empty() {
                return Ok(Value::from(true));
            }

            for item in items {
                if item != predicate {
                    return Ok(Value::from(false));
                }
            }
        }

        Ok(Value::from(true))
    }
}

/// Check if array contains a specific value
pub struct ArrayContains;

impl Function for ArrayContains {
    const NAME: &'static str = "array_contains";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "array_contains",
        category: "predicate",
        description: "Check if array contains a specific value",
        arguments: &[
            ArgumentMetadata {
                name: "array",
                arg_type: "array",
                required: true,
                default: None,
                description: "The array to search",
            },
            ArgumentMetadata {
                name: "value",
                arg_type: "any",
                required: true,
                default: None,
                description: "The value to find",
            },
        ],
        return_type: "boolean",
        examples: &["{% if array_contains(array=[1, 2, 42, 3], value=42) %}Found it!{% endif %}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let array: Value = kwargs.get("array")?;
        let value: Value = kwargs.get("value")?;

        if !matches!(array.kind(), minijinja::value::ValueKind::Seq) {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                "array_contains requires an array",
            ));
        }

        if let Ok(seq) = array.try_iter() {
            for item in seq {
                if item == value {
                    return Ok(Value::from(true));
                }
            }
        }

        Ok(Value::from(false))
    }
}

/// Check if string starts with a prefix
pub struct StartsWith;

impl Function for StartsWith {
    const NAME: &'static str = "starts_with";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "starts_with",
        category: "predicate",
        description: "Check if string starts with a prefix",
        arguments: &[
            ArgumentMetadata {
                name: "string",
                arg_type: "string",
                required: true,
                default: None,
                description: "The string to check",
            },
            ArgumentMetadata {
                name: "prefix",
                arg_type: "string",
                required: true,
                default: None,
                description: "The prefix to look for",
            },
        ],
        return_type: "boolean",
        examples: &[
            "{% if starts_with(string=\"Hello World\", prefix=\"Hello\") %}Starts with Hello!{% endif %}",
        ],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let string: String = kwargs.get("string")?;
        let prefix: String = kwargs.get("prefix")?;

        Ok(Value::from(string.starts_with(&prefix)))
    }
}

/// Check if string ends with a suffix
pub struct EndsWith;

impl Function for EndsWith {
    const NAME: &'static str = "ends_with";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "ends_with",
        category: "predicate",
        description: "Check if string ends with a suffix",
        arguments: &[
            ArgumentMetadata {
                name: "string",
                arg_type: "string",
                required: true,
                default: None,
                description: "The string to check",
            },
            ArgumentMetadata {
                name: "suffix",
                arg_type: "string",
                required: true,
                default: None,
                description: "The suffix to look for",
            },
        ],
        return_type: "boolean",
        examples: &[
            "{% if ends_with(string=\"readme.txt\", suffix=\".txt\") %}Text file!{% endif %}",
        ],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let string: String = kwargs.get("string")?;
        let suffix: String = kwargs.get("suffix")?;

        Ok(Value::from(string.ends_with(&suffix)))
    }
}
