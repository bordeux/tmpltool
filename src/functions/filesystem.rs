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

/// Get the filename from a path
///
/// # Arguments
///
/// * `path` (required) - File path
///
/// # Returns
///
/// Returns the filename component of the path
///
/// # Example
///
/// ```jinja
/// {{ basename(path="/path/to/file.txt") }}  => file.txt
/// {{ basename(path="folder/document.pdf") }}  => document.pdf
/// ```
pub fn basename_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let path: String = kwargs.get("path")?;

    let path_obj = std::path::Path::new(&path);
    let filename = path_obj.file_name().and_then(|n| n.to_str()).unwrap_or("");

    Ok(Value::from(filename))
}

/// Get the directory component from a path
///
/// # Arguments
///
/// * `path` (required) - File path
///
/// # Returns
///
/// Returns the directory component of the path
///
/// # Example
///
/// ```jinja
/// {{ dirname(path="/path/to/file.txt") }}  => /path/to
/// {{ dirname(path="folder/document.pdf") }}  => folder
/// ```
pub fn dirname_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let path: String = kwargs.get("path")?;

    let path_obj = std::path::Path::new(&path);
    let dir = path_obj.parent().and_then(|p| p.to_str()).unwrap_or("");

    Ok(Value::from(dir))
}

/// Get the file extension from a path
///
/// # Arguments
///
/// * `path` (required) - File path
///
/// # Returns
///
/// Returns the file extension (without the dot)
///
/// # Example
///
/// ```jinja
/// {{ file_extension(path="document.pdf") }}  => pdf
/// {{ file_extension(path="/path/to/file.tar.gz") }}  => gz
/// {{ file_extension(path="noextension") }}  => (empty string)
/// ```
pub fn file_extension_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let path: String = kwargs.get("path")?;

    let path_obj = std::path::Path::new(&path);
    let extension = path_obj.extension().and_then(|e| e.to_str()).unwrap_or("");

    Ok(Value::from(extension))
}

/// Join path components
///
/// # Arguments
///
/// * `parts` (required) - Array of path components to join
///
/// # Returns
///
/// Returns the joined path
///
/// # Example
///
/// ```jinja
/// {{ join_path(parts=["path", "to", "file.txt"]) }}  => path/to/file.txt
/// {{ join_path(parts=["/home", "user", "documents"]) }}  => /home/user/documents
/// ```
pub fn join_path_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let parts: Vec<String> = kwargs.get("parts")?;

    if parts.is_empty() {
        return Ok(Value::from(""));
    }

    let mut path_buf = std::path::PathBuf::new();
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

/// Normalize a path (resolve .. and . components)
///
/// # Arguments
///
/// * `path` (required) - Path to normalize
///
/// # Returns
///
/// Returns the normalized path
///
/// # Example
///
/// ```jinja
/// {{ normalize_path(path="./foo/../bar") }}  => bar
/// {{ normalize_path(path="/path/to/../file.txt") }}  => /path/file.txt
/// ```
pub fn normalize_path_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let path: String = kwargs.get("path")?;

    let path_obj = std::path::Path::new(&path);

    // Use components to normalize the path
    let mut normalized = std::path::PathBuf::new();
    for component in path_obj.components() {
        match component {
            std::path::Component::ParentDir => {
                normalized.pop();
            }
            std::path::Component::CurDir => {
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
    let normalized_slashes = result.replace('\\', "/");

    Ok(Value::from(normalized_slashes))
}

// Note: is_file, is_dir, and is_symlink have been migrated to src/is_functions/filesystem.rs
// and support both function syntax and "is" test syntax:
// - `{{ is_file(path="config.txt") }}` / `{% if "config.txt" is file %}`
// - `{{ is_dir(path="src") }}` / `{% if "src" is dir %}`
// - `{{ is_symlink(path="link") }}` / `{% if "link" is symlink %}`

/// Read lines from a file
///
/// # Arguments
///
/// * `path` (required) - Path to file
/// * `max_lines` (optional) - Number of lines to read (default: 10)
///   - Positive number: Read first N lines
///   - Negative number: Read last N lines
///   - Zero: Read entire file
///
/// # Returns
///
/// Returns an array of lines (without newline characters)
///
/// # Example
///
/// ```jinja
/// {# Read first 5 lines #}
/// {% set first_lines = read_lines(path="log.txt", max_lines=5) %}
///
/// {# Read last 5 lines #}
/// {% set last_lines = read_lines(path="log.txt", max_lines=-5) %}
///
/// {# Read entire file #}
/// {% set all_lines = read_lines(path="config.txt", max_lines=0) %}
/// ```
pub fn create_read_lines_fn(
    context: Arc<TemplateContext>,
) -> impl Fn(Kwargs) -> Result<Value, Error> + Send + Sync + 'static {
    move |kwargs: Kwargs| {
        let path: String = kwargs.get("path")?;
        let max_lines: i64 = kwargs.get::<i64>("max_lines").ok().unwrap_or(10);

        // Validate max_lines range
        if max_lines.abs() > 10000 {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                format!(
                    "max_lines absolute value must be between 0 and 10000, got {}",
                    max_lines
                ),
            ));
        }

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

        // Read file content
        let content = fs::read_to_string(&resolved_path).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Failed to read file '{}': {}", path, e),
            )
        })?;

        // Collect all lines
        let all_lines: Vec<&str> = content.lines().collect();

        // Select lines based on max_lines
        let lines: Vec<Value> = if max_lines == 0 {
            // Read entire file
            all_lines
                .iter()
                .map(|line| Value::from(line.to_string()))
                .collect()
        } else if max_lines > 0 {
            // Read first N lines
            all_lines
                .iter()
                .take(max_lines as usize)
                .map(|line| Value::from(line.to_string()))
                .collect()
        } else {
            // Read last N lines (max_lines is negative)
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
