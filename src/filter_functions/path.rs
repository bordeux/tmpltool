//! Path functions that support both function and filter syntax.
//!
//! # Function Syntax
//! ```jinja
//! {{ basename(path="/path/to/file.txt") }}
//! {{ dirname(path="/path/to/file.txt") }}
//! ```
//!
//! # Filter Syntax
//! ```jinja
//! {{ "/path/to/file.txt" | basename }}
//! {{ "/path/to/file.txt" | dirname }}
//! ```
//!
//! # Chaining
//! ```jinja
//! {{ "/path/to/file.tar.gz" | basename | file_extension }}
//! ```

use super::FilterFunction;
use crate::functions::metadata::{ArgumentMetadata, FunctionMetadata, SyntaxVariants};
use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};
use std::path::{Component, Path, PathBuf};

/// Common metadata for path argument
const PATH_ARG: ArgumentMetadata = ArgumentMetadata {
    name: "path",
    arg_type: "string",
    required: true,
    default: None,
    description: "The file path to process",
};

/// Helper to extract path string from Value
fn extract_path(value: &Value, fn_name: &str) -> Result<String, Error> {
    value.as_str().map(|s| s.to_string()).ok_or_else(|| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("{} requires a string path, found: {}", fn_name, value),
        )
    })
}

// ============================================
// Basename
// ============================================

/// Get the filename component from a path.
pub struct Basename;

impl Basename {
    fn compute(path: &str) -> Value {
        let path_obj = Path::new(path);
        let filename = path_obj.file_name().and_then(|n| n.to_str()).unwrap_or("");
        Value::from(filename)
    }
}

impl FilterFunction for Basename {
    const NAME: &'static str = "basename";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "basename",
        category: "path",
        description: "Get the filename component from a path",
        arguments: &[PATH_ARG],
        return_type: "string",
        examples: &[
            "{{ basename(path=\"/path/to/file.txt\") }}",
            "{{ \"/path/to/file.txt\" | basename }}",
        ],
        syntax: SyntaxVariants::FUNCTION_AND_FILTER,
    };

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let path: String = kwargs.get("path")?;
        Ok(Self::compute(&path))
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        let path = extract_path(value, "basename")?;
        Ok(Self::compute(&path))
    }
}

// ============================================
// Dirname
// ============================================

/// Get the directory component from a path.
pub struct Dirname;

impl Dirname {
    fn compute(path: &str) -> Value {
        let path_obj = Path::new(path);
        let dir = path_obj.parent().and_then(|p| p.to_str()).unwrap_or("");
        Value::from(dir)
    }
}

impl FilterFunction for Dirname {
    const NAME: &'static str = "dirname";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "dirname",
        category: "path",
        description: "Get the directory component from a path",
        arguments: &[PATH_ARG],
        return_type: "string",
        examples: &[
            "{{ dirname(path=\"/path/to/file.txt\") }}",
            "{{ \"/path/to/file.txt\" | dirname }}",
        ],
        syntax: SyntaxVariants::FUNCTION_AND_FILTER,
    };

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let path: String = kwargs.get("path")?;
        Ok(Self::compute(&path))
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        let path = extract_path(value, "dirname")?;
        Ok(Self::compute(&path))
    }
}

// ============================================
// FileExtension
// ============================================

/// Get the file extension from a path.
pub struct FileExtension;

impl FileExtension {
    fn compute(path: &str) -> Value {
        let path_obj = Path::new(path);
        let extension = path_obj.extension().and_then(|e| e.to_str()).unwrap_or("");
        Value::from(extension)
    }
}

impl FilterFunction for FileExtension {
    const NAME: &'static str = "file_extension";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "file_extension",
        category: "path",
        description: "Get the file extension from a path",
        arguments: &[PATH_ARG],
        return_type: "string",
        examples: &[
            "{{ file_extension(path=\"document.pdf\") }}",
            "{{ \"document.pdf\" | file_extension }}",
        ],
        syntax: SyntaxVariants::FUNCTION_AND_FILTER,
    };

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let path: String = kwargs.get("path")?;
        Ok(Self::compute(&path))
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        let path = extract_path(value, "file_extension")?;
        Ok(Self::compute(&path))
    }
}

// ============================================
// JoinPath
// ============================================

/// Join path components into a single path.
pub struct JoinPath;

impl JoinPath {
    fn compute(parts: Vec<String>) -> Result<Value, Error> {
        if parts.is_empty() {
            return Ok(Value::from(""));
        }

        let mut path_buf = PathBuf::new();
        for part in parts {
            path_buf.push(part);
        }

        let joined = path_buf.to_str().ok_or_else(|| {
            Error::new(
                ErrorKind::InvalidOperation,
                "Failed to convert path to string".to_string(),
            )
        })?;

        // Normalize to forward slashes for cross-platform consistency
        let normalized = joined.replace('\\', "/");

        Ok(Value::from(normalized))
    }
}

impl FilterFunction for JoinPath {
    const NAME: &'static str = "join_path";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "join_path",
        category: "path",
        description: "Join path components into a single path",
        arguments: &[ArgumentMetadata {
            name: "parts",
            arg_type: "array",
            required: true,
            default: None,
            description: "Array of path components to join",
        }],
        return_type: "string",
        examples: &[
            "{{ join_path(parts=[\"path\", \"to\", \"file.txt\"]) }}",
            "{{ [\"path\", \"to\", \"file.txt\"] | join_path }}",
        ],
        syntax: SyntaxVariants::FUNCTION_AND_FILTER,
    };

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let parts: Vec<String> = kwargs.get("parts")?;
        Self::compute(parts)
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        // Extract array of strings from value
        let mut parts: Vec<String> = Vec::new();

        if let Ok(seq) = value.try_iter() {
            for item in seq {
                let part = item.as_str().ok_or_else(|| {
                    Error::new(
                        ErrorKind::InvalidOperation,
                        format!("join_path requires an array of strings, found: {}", item),
                    )
                })?;
                parts.push(part.to_string());
            }
        } else {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                format!("join_path requires an array, found: {}", value),
            ));
        }

        Self::compute(parts)
    }
}

// ============================================
// NormalizePath
// ============================================

/// Normalize a path (resolve .. and . components).
pub struct NormalizePath;

impl NormalizePath {
    fn compute(path: &str) -> Result<Value, Error> {
        let path_obj = Path::new(path);

        // Use components to normalize the path
        let mut normalized = PathBuf::new();
        for component in path_obj.components() {
            match component {
                Component::ParentDir => {
                    normalized.pop();
                }
                Component::CurDir => {
                    // Skip current directory
                }
                _ => {
                    normalized.push(component);
                }
            }
        }

        let result = normalized.to_str().ok_or_else(|| {
            Error::new(
                ErrorKind::InvalidOperation,
                "Failed to convert normalized path to string".to_string(),
            )
        })?;

        // Normalize to forward slashes for cross-platform consistency
        let normalized_str = result.replace('\\', "/");

        Ok(Value::from(normalized_str))
    }
}

impl FilterFunction for NormalizePath {
    const NAME: &'static str = "normalize_path";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "normalize_path",
        category: "path",
        description: "Normalize a path (resolve .. and . components)",
        arguments: &[PATH_ARG],
        return_type: "string",
        examples: &[
            "{{ normalize_path(path=\"./foo/../bar\") }}",
            "{{ \"./foo/../bar\" | normalize_path }}",
        ],
        syntax: SyntaxVariants::FUNCTION_AND_FILTER,
    };

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let path: String = kwargs.get("path")?;
        Self::compute(&path)
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        let path = extract_path(value, "normalize_path")?;
        Self::compute(&path)
    }
}
