/// File system functions
///
/// Provides functions for interacting with the file system:
/// - read_file: Read file contents
/// - file_exists: Check if file exists
/// - list_dir: List directory contents
/// - glob: List files by pattern
/// - file_size: Get file size
/// - file_modified: Get file modification timestamp
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

/// Create read_file function with context
pub fn create_read_file_fn(
    context: Arc<TemplateContext>,
) -> impl Fn(Kwargs) -> Result<Value, Error> + Send + Sync + 'static {
    move |kwargs: Kwargs| {
        // Extract path from kwargs
        let path: String = kwargs.get("path")?;
        // Security: Prevent reading absolute paths or paths with parent directory traversal (unless trust mode is enabled)
        if !context.is_trust_mode() && (path.starts_with('/') || path.contains("..")) {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                format!(
                    "Security: Absolute paths and parent directory (..) access are not allowed: {}. Use --trust to bypass this restriction.",
                    path
                ),
            ));
        }

        // Resolve path relative to template's base directory
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

/// Create file_exists function with context
pub fn create_file_exists_fn(
    context: Arc<TemplateContext>,
) -> impl Fn(Kwargs) -> Result<Value, Error> + Send + Sync + 'static {
    move |kwargs: Kwargs| {
        // Extract path from kwargs
        let path: String = kwargs.get("path")?;
        // Security: Prevent checking absolute paths or paths with parent directory traversal (unless trust mode is enabled)
        if !context.is_trust_mode() && (path.starts_with('/') || path.contains("..")) {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                format!(
                    "Security: Absolute paths and parent directory (..) access are not allowed: {}. Use --trust to bypass this restriction.",
                    path
                ),
            ));
        }

        // Resolve path relative to template's base directory
        let resolved_path = context.resolve_path(&path);
        Ok(Value::from(resolved_path.exists()))
    }
}

/// Create list_dir function with context
pub fn create_list_dir_fn(
    context: Arc<TemplateContext>,
) -> impl Fn(Kwargs) -> Result<Value, Error> + Send + Sync + 'static {
    move |kwargs: Kwargs| {
        // Extract path from kwargs
        let path: String = kwargs.get("path")?;
        // Security: Prevent listing absolute paths or paths with parent directory traversal (unless trust mode is enabled)
        if !context.is_trust_mode() && (path.starts_with('/') || path.contains("..")) {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                format!(
                    "Security: Absolute paths and parent directory (..) access are not allowed: {}. Use --trust to bypass this restriction.",
                    path
                ),
            ));
        }

        // Resolve path relative to template's base directory
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

        // Sort for consistent output
        files.sort();

        Ok(Value::from_serialize(&files))
    }
}

/// Create glob function with context
pub fn create_glob_fn(
    context: Arc<TemplateContext>,
) -> impl Fn(Kwargs) -> Result<Value, Error> + Send + Sync + 'static {
    move |kwargs: Kwargs| {
        // Extract pattern from kwargs
        let pattern: String = kwargs.get("pattern")?;
        // Security: Prevent absolute paths or paths with parent directory traversal (unless trust mode is enabled)
        if !context.is_trust_mode() && (pattern.starts_with('/') || pattern.contains("..")) {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                format!(
                    "Security: Absolute paths and parent directory (..) access are not allowed: {}. Use --trust to bypass this restriction.",
                    pattern
                ),
            ));
        }

        // Resolve pattern relative to template's base directory
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

        // Sort for consistent output
        files.sort();

        Ok(Value::from_serialize(&files))
    }
}

/// Create file_size function with context
pub fn create_file_size_fn(
    context: Arc<TemplateContext>,
) -> impl Fn(Kwargs) -> Result<Value, Error> + Send + Sync + 'static {
    move |kwargs: Kwargs| {
        // Extract path from kwargs
        let path: String = kwargs.get("path")?;
        // Security: Prevent accessing absolute paths or paths with parent directory traversal (unless trust mode is enabled)
        if !context.is_trust_mode() && (path.starts_with('/') || path.contains("..")) {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                format!(
                    "Security: Absolute paths and parent directory (..) access are not allowed: {}. Use --trust to bypass this restriction.",
                    path
                ),
            ));
        }

        // Resolve path relative to template's base directory
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

/// Create file_modified function with context
pub fn create_file_modified_fn(
    context: Arc<TemplateContext>,
) -> impl Fn(Kwargs) -> Result<Value, Error> + Send + Sync + 'static {
    move |kwargs: Kwargs| {
        // Extract path from kwargs
        let path: String = kwargs.get("path")?;
        // Security: Prevent accessing absolute paths or paths with parent directory traversal (unless trust mode is enabled)
        if !context.is_trust_mode() && (path.starts_with('/') || path.contains("..")) {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                format!(
                    "Security: Absolute paths and parent directory (..) access are not allowed: {}. Use --trust to bypass this restriction.",
                    path
                ),
            ));
        }

        // Resolve path relative to template's base directory
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

        // Convert to Unix timestamp (seconds since epoch)
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
