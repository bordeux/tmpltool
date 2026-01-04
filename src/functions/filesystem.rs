//! File system functions
//!
//! Provides functions for interacting with the file system:
//! - read_file: Read file contents
//! - file_exists: Check if file exists
//! - list_dir: List directory contents
//! - glob: List files by pattern
//! - file_size: Get file size
//! - file_modified: Get file modification timestamp
//! - read_lines: Read lines from a file
//!
//! Note: basename, dirname, file_extension, join_path, normalize_path are now in
//! filter_functions/path.rs with dual function+filter syntax support.
//!
//! Note: is_file, is_dir, is_symlink are now in is_functions/filesystem.rs with
//! dual function+is-test syntax support.

use super::metadata::{ArgumentMetadata, FunctionMetadata, SyntaxVariants};
use super::traits::ContextFunction;
use crate::TemplateContext;
use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};
use std::fs;
use std::sync::Arc;

/// Validate path security (prevent absolute paths and parent directory traversal)
///
/// This is a public helper function that can be reused by other modules that need
/// to perform file path security validation.
pub fn validate_path_security(path: &str) -> Result<(), Error> {
    if path.starts_with('/') {
        return Err(Error::new(
            ErrorKind::InvalidOperation,
            format!(
                "Security: Absolute paths are not allowed: {}. Use --trust to bypass this restriction.",
                path
            ),
        ));
    }

    if path.contains("..") {
        return Err(Error::new(
            ErrorKind::InvalidOperation,
            format!(
                "Security: Parent directory (..) traversal is not allowed: {}. Use --trust to bypass this restriction.",
                path
            ),
        ));
    }

    Ok(())
}

/// Read file contents
pub struct ReadFile;

impl ContextFunction for ReadFile {
    const NAME: &'static str = "read_file";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "read_file",
        category: "filesystem",
        description: "Read file contents as string",
        arguments: &[ArgumentMetadata {
            name: "path",
            arg_type: "string",
            required: true,
            default: None,
            description: "Path to the file to read",
        }],
        return_type: "string",
        examples: &[
            "{{ read_file(path=\"config.txt\") }}",
            "{% set content = read_file(path=\"data.json\") %}",
        ],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(context: Arc<TemplateContext>, kwargs: Kwargs) -> Result<Value, Error> {
        let path: String = kwargs.get("path")?;

        // Security check
        if !context.is_trust_mode() && (path.starts_with('/') || path.contains("..")) {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                format!(
                    "Security: Absolute paths and parent directory (..) access are not allowed: {}. Use --trust to bypass this restriction.",
                    path
                ),
            ));
        }

        let resolved_path = context.resolve_path(&path);

        let content = fs::read_to_string(&resolved_path).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Failed to read file '{}': {}", resolved_path.display(), e),
            )
        })?;

        Ok(Value::from(content))
    }
}

/// Check if file exists
pub struct FileExists;

impl ContextFunction for FileExists {
    const NAME: &'static str = "file_exists";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "file_exists",
        category: "filesystem",
        description: "Check if a file or directory exists",
        arguments: &[ArgumentMetadata {
            name: "path",
            arg_type: "string",
            required: true,
            default: None,
            description: "Path to check",
        }],
        return_type: "boolean",
        examples: &["{% if file_exists(path=\"config.json\") %}Config found{% endif %}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(context: Arc<TemplateContext>, kwargs: Kwargs) -> Result<Value, Error> {
        let path: String = kwargs.get("path")?;

        if !context.is_trust_mode() && (path.starts_with('/') || path.contains("..")) {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                format!(
                    "Security: Absolute paths and parent directory (..) access are not allowed: {}. Use --trust to bypass this restriction.",
                    path
                ),
            ));
        }

        let resolved_path = context.resolve_path(&path);
        Ok(Value::from(resolved_path.exists()))
    }
}

/// List directory contents
pub struct ListDir;

impl ContextFunction for ListDir {
    const NAME: &'static str = "list_dir";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "list_dir",
        category: "filesystem",
        description: "List files and directories in a directory",
        arguments: &[ArgumentMetadata {
            name: "path",
            arg_type: "string",
            required: true,
            default: None,
            description: "Directory path to list",
        }],
        return_type: "array",
        examples: &["{% for file in list_dir(path=\".\") %}{{ file }}{% endfor %}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(context: Arc<TemplateContext>, kwargs: Kwargs) -> Result<Value, Error> {
        let path: String = kwargs.get("path")?;

        if !context.is_trust_mode() && (path.starts_with('/') || path.contains("..")) {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                format!(
                    "Security: Absolute paths and parent directory (..) access are not allowed: {}. Use --trust to bypass this restriction.",
                    path
                ),
            ));
        }

        let resolved_path = context.resolve_path(&path);

        let entries = fs::read_dir(&resolved_path).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!(
                    "Failed to read directory '{}': {}",
                    resolved_path.display(),
                    e
                ),
            )
        })?;

        let mut files: Vec<String> = Vec::new();
        for entry in entries {
            let entry = entry.map_err(|e| {
                Error::new(
                    ErrorKind::InvalidOperation,
                    format!("Failed to read directory entry: {}", e),
                )
            })?;
            let file_name = entry
                .file_name()
                .into_string()
                .unwrap_or_else(|_| String::from("?"));
            files.push(file_name);
        }

        files.sort();
        Ok(Value::from_serialize(&files))
    }
}

/// List files matching a glob pattern
pub struct Glob;

impl ContextFunction for Glob {
    const NAME: &'static str = "glob";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "glob",
        category: "filesystem",
        description: "List files matching a glob pattern",
        arguments: &[ArgumentMetadata {
            name: "pattern",
            arg_type: "string",
            required: true,
            default: None,
            description: "Glob pattern (e.g., \"*.txt\", \"**/*.json\")",
        }],
        return_type: "array",
        examples: &[
            "{% for f in glob(pattern=\"*.txt\") %}{{ f }}{% endfor %}",
            "{{ glob(pattern=\"src/**/*.rs\") | length }}",
        ],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(context: Arc<TemplateContext>, kwargs: Kwargs) -> Result<Value, Error> {
        let pattern: String = kwargs.get("pattern")?;

        if !context.is_trust_mode() && (pattern.starts_with('/') || pattern.contains("..")) {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                format!(
                    "Security: Absolute paths and parent directory (..) access are not allowed: {}. Use --trust to bypass this restriction.",
                    pattern
                ),
            ));
        }

        let resolved_pattern = context.resolve_path(&pattern);
        let pattern_str = resolved_pattern.to_str().ok_or_else(|| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Invalid path encoding in pattern: {:?}", resolved_pattern),
            )
        })?;

        let glob_result = glob::glob(pattern_str).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Invalid glob pattern '{}': {}", pattern_str, e),
            )
        })?;

        let mut files: Vec<String> = Vec::new();
        for entry in glob_result {
            match entry {
                Ok(path) => {
                    if let Some(path_str) = path.to_str() {
                        files.push(path_str.to_string());
                    }
                }
                Err(e) => {
                    return Err(Error::new(
                        ErrorKind::InvalidOperation,
                        format!("Glob error: {}", e),
                    ));
                }
            }
        }

        files.sort();
        Ok(Value::from_serialize(&files))
    }
}

/// Get file size in bytes
pub struct FileSize;

impl ContextFunction for FileSize {
    const NAME: &'static str = "file_size";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "file_size",
        category: "filesystem",
        description: "Get file size in bytes",
        arguments: &[ArgumentMetadata {
            name: "path",
            arg_type: "string",
            required: true,
            default: None,
            description: "Path to the file",
        }],
        return_type: "integer",
        examples: &[
            "{{ file_size(path=\"data.bin\") }}",
            "{{ file_size(path=\"large.zip\") | filesizeformat }}",
        ],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(context: Arc<TemplateContext>, kwargs: Kwargs) -> Result<Value, Error> {
        let path: String = kwargs.get("path")?;

        if !context.is_trust_mode() && (path.starts_with('/') || path.contains("..")) {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                format!(
                    "Security: Absolute paths and parent directory (..) access are not allowed: {}. Use --trust to bypass this restriction.",
                    path
                ),
            ));
        }

        let resolved_path = context.resolve_path(&path);

        let metadata = fs::metadata(&resolved_path).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!(
                    "Failed to get file metadata for '{}': {}",
                    resolved_path.display(),
                    e
                ),
            )
        })?;

        Ok(Value::from(metadata.len()))
    }
}

/// Get file modification timestamp
pub struct FileModified;

impl ContextFunction for FileModified {
    const NAME: &'static str = "file_modified";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "file_modified",
        category: "filesystem",
        description: "Get file modification timestamp (Unix epoch seconds)",
        arguments: &[ArgumentMetadata {
            name: "path",
            arg_type: "string",
            required: true,
            default: None,
            description: "Path to the file",
        }],
        return_type: "integer",
        examples: &["{{ file_modified(path=\"data.txt\") }}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(context: Arc<TemplateContext>, kwargs: Kwargs) -> Result<Value, Error> {
        let path: String = kwargs.get("path")?;

        if !context.is_trust_mode() && (path.starts_with('/') || path.contains("..")) {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                format!(
                    "Security: Absolute paths and parent directory (..) access are not allowed: {}. Use --trust to bypass this restriction.",
                    path
                ),
            ));
        }

        let resolved_path = context.resolve_path(&path);

        let metadata = fs::metadata(&resolved_path).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!(
                    "Failed to get file metadata for '{}': {}",
                    resolved_path.display(),
                    e
                ),
            )
        })?;

        let modified = metadata.modified().map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Failed to get modification time: {}", e),
            )
        })?;

        let duration = modified
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| {
                Error::new(
                    ErrorKind::InvalidOperation,
                    format!("Failed to convert timestamp: {}", e),
                )
            })?;

        Ok(Value::from(duration.as_secs()))
    }
}

/// Read lines from a file
pub struct ReadLines;

impl ContextFunction for ReadLines {
    const NAME: &'static str = "read_lines";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "read_lines",
        category: "filesystem",
        description: "Read lines from a file",
        arguments: &[
            ArgumentMetadata {
                name: "path",
                arg_type: "string",
                required: true,
                default: None,
                description: "Path to the file",
            },
            ArgumentMetadata {
                name: "max_lines",
                arg_type: "integer",
                required: false,
                default: Some("10"),
                description: "Number of lines to read (positive=first N, negative=last N, 0=all)",
            },
        ],
        return_type: "array",
        examples: &[
            "{{ read_lines(path=\"log.txt\", max_lines=5) }}",
            "{{ read_lines(path=\"log.txt\", max_lines=-5) }}",
        ],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(context: Arc<TemplateContext>, kwargs: Kwargs) -> Result<Value, Error> {
        let path: String = kwargs.get("path")?;
        let max_lines: i64 = kwargs.get::<i64>("max_lines").ok().unwrap_or(10);

        if max_lines.abs() > 10000 {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                format!(
                    "max_lines absolute value must be between 0 and 10000, got {}",
                    max_lines
                ),
            ));
        }

        if !context.is_trust_mode() && (path.starts_with('/') || path.contains("..")) {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                format!(
                    "Security: Absolute paths and parent directory (..) access are not allowed: {}. Use --trust to bypass this restriction.",
                    path
                ),
            ));
        }

        let resolved_path = context.resolve_path(&path);

        let content = fs::read_to_string(&resolved_path).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Failed to read file '{}': {}", path, e),
            )
        })?;

        let all_lines: Vec<&str> = content.lines().collect();

        let lines: Vec<Value> = if max_lines == 0 {
            all_lines
                .iter()
                .map(|line| Value::from(line.to_string()))
                .collect()
        } else if max_lines > 0 {
            all_lines
                .iter()
                .take(max_lines as usize)
                .map(|line| Value::from(line.to_string()))
                .collect()
        } else {
            let n = (-max_lines) as usize;
            let start_index = all_lines.len().saturating_sub(n);
            all_lines
                .iter()
                .skip(start_index)
                .map(|line| Value::from(line.to_string()))
                .collect()
        };

        Ok(Value::from(lines))
    }
}
