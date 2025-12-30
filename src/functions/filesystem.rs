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
use std::collections::HashMap;
use std::fs;
use tera::{Function, Result, Value, to_value};

/// Validate path security (prevent absolute paths and parent directory traversal)
///
/// This is a public helper function that can be reused by other modules that need
/// to perform file path security validation.
pub fn validate_path_security(path: &str) -> Result<()> {
    if path.starts_with('/') {
        return Err(tera::Error::msg(format!(
            "Security: Absolute paths are not allowed: {}. Use --trust to bypass this restriction.",
            path
        )));
    }

    if path.contains("..") {
        return Err(tera::Error::msg(format!(
            "Security: Parent directory (..) traversal is not allowed: {}. Use --trust to bypass this restriction.",
            path
        )));
    }

    Ok(())
}

/// Read file content function
pub struct ReadFile {
    context: TemplateContext,
}

impl ReadFile {
    pub fn new(context: TemplateContext) -> Self {
        ReadFile { context }
    }
}

impl Function for ReadFile {
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value> {
        let path = args.get("path").and_then(|v| v.as_str()).ok_or_else(|| {
            tera::Error::msg("read_file requires a 'path' argument (e.g., path=\"config.txt\")")
        })?;

        // Security: Prevent reading absolute paths or paths with parent directory traversal (unless trust mode is enabled)
        if !self.context.is_trust_mode() && (path.starts_with('/') || path.contains("..")) {
            return Err(tera::Error::msg(format!(
                "Security: Absolute paths and parent directory (..) access are not allowed: {}. Use --trust to bypass this restriction.",
                path
            )));
        }

        // Resolve path relative to template's base directory
        let resolved_path = self.context.resolve_path(path);

        let content = fs::read_to_string(&resolved_path).map_err(|e| {
            tera::Error::msg(format!(
                "Failed to read file '{}': {}",
                resolved_path.display(),
                e
            ))
        })?;

        to_value(&content)
            .map_err(|e| tera::Error::msg(format!("Failed to convert content: {}", e)))
    }
}

/// Check if file exists function
pub struct FileExists {
    context: TemplateContext,
}

impl FileExists {
    pub fn new(context: TemplateContext) -> Self {
        FileExists { context }
    }
}

impl Function for FileExists {
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value> {
        let path = args.get("path").and_then(|v| v.as_str()).ok_or_else(|| {
            tera::Error::msg("file_exists requires a 'path' argument (e.g., path=\"file.txt\")")
        })?;

        // Security: Prevent checking absolute paths or paths with parent directory traversal (unless trust mode is enabled)
        if !self.context.is_trust_mode() && (path.starts_with('/') || path.contains("..")) {
            return Err(tera::Error::msg(format!(
                "Security: Absolute paths and parent directory (..) access are not allowed: {}. Use --trust to bypass this restriction.",
                path
            )));
        }

        // Resolve path relative to template's base directory
        let resolved_path = self.context.resolve_path(path);
        let exists = resolved_path.exists();

        to_value(exists).map_err(|e| tera::Error::msg(format!("Failed to convert result: {}", e)))
    }
}

/// List directory contents function
pub struct ListDir {
    context: TemplateContext,
}

impl ListDir {
    pub fn new(context: TemplateContext) -> Self {
        ListDir { context }
    }
}

impl Function for ListDir {
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value> {
        let path = args.get("path").and_then(|v| v.as_str()).ok_or_else(|| {
            tera::Error::msg("list_dir requires a 'path' argument (e.g., path=\"./data\")")
        })?;

        // Security: Prevent listing absolute paths or paths with parent directory traversal (unless trust mode is enabled)
        if !self.context.is_trust_mode() && (path.starts_with('/') || path.contains("..")) {
            return Err(tera::Error::msg(format!(
                "Security: Absolute paths and parent directory (..) access are not allowed: {}. Use --trust to bypass this restriction.",
                path
            )));
        }

        // Resolve path relative to template's base directory
        let resolved_path = self.context.resolve_path(path);

        let entries = fs::read_dir(&resolved_path).map_err(|e| {
            tera::Error::msg(format!(
                "Failed to read directory '{}': {}",
                resolved_path.display(),
                e
            ))
        })?;

        let mut files: Vec<String> = Vec::new();
        for entry in entries {
            let entry = entry
                .map_err(|e| tera::Error::msg(format!("Failed to read directory entry: {}", e)))?;
            let file_name = entry
                .file_name()
                .into_string()
                .unwrap_or_else(|_| String::from("?"));
            files.push(file_name);
        }

        // Sort for consistent output
        files.sort();

        to_value(&files).map_err(|e| tera::Error::msg(format!("Failed to convert result: {}", e)))
    }
}

/// Glob pattern matching function
pub struct GlobFiles {
    context: TemplateContext,
}

impl GlobFiles {
    pub fn new(context: TemplateContext) -> Self {
        GlobFiles { context }
    }
}

impl Function for GlobFiles {
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value> {
        let pattern = args
            .get("pattern")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                tera::Error::msg("glob requires a 'pattern' argument (e.g., pattern=\"*.txt\")")
            })?;

        // Security: Prevent absolute paths or paths with parent directory traversal (unless trust mode is enabled)
        if !self.context.is_trust_mode() && (pattern.starts_with('/') || pattern.contains("..")) {
            return Err(tera::Error::msg(format!(
                "Security: Absolute paths and parent directory (..) access are not allowed: {}. Use --trust to bypass this restriction.",
                pattern
            )));
        }

        // Resolve pattern relative to template's base directory
        let resolved_pattern = self.context.resolve_path(pattern);
        let pattern_str = resolved_pattern.to_str().ok_or_else(|| {
            tera::Error::msg(format!(
                "Invalid path encoding in pattern: {:?}",
                resolved_pattern
            ))
        })?;

        let glob_result = glob::glob(pattern_str).map_err(|e| {
            tera::Error::msg(format!("Invalid glob pattern '{}': {}", pattern_str, e))
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
                    return Err(tera::Error::msg(format!("Glob error: {}", e)));
                }
            }
        }

        // Sort for consistent output
        files.sort();

        to_value(&files).map_err(|e| tera::Error::msg(format!("Failed to convert result: {}", e)))
    }
}

/// Get file size function
pub struct FileSize {
    context: TemplateContext,
}

impl FileSize {
    pub fn new(context: TemplateContext) -> Self {
        FileSize { context }
    }
}

impl Function for FileSize {
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value> {
        let path = args.get("path").and_then(|v| v.as_str()).ok_or_else(|| {
            tera::Error::msg("file_size requires a 'path' argument (e.g., path=\"data.bin\")")
        })?;

        // Security: Prevent accessing absolute paths or paths with parent directory traversal (unless trust mode is enabled)
        if !self.context.is_trust_mode() && (path.starts_with('/') || path.contains("..")) {
            return Err(tera::Error::msg(format!(
                "Security: Absolute paths and parent directory (..) access are not allowed: {}. Use --trust to bypass this restriction.",
                path
            )));
        }

        // Resolve path relative to template's base directory
        let resolved_path = self.context.resolve_path(path);

        let metadata = fs::metadata(&resolved_path).map_err(|e| {
            tera::Error::msg(format!(
                "Failed to get file metadata for '{}': {}",
                resolved_path.display(),
                e
            ))
        })?;

        let size = metadata.len();

        to_value(size).map_err(|e| tera::Error::msg(format!("Failed to convert result: {}", e)))
    }
}

/// Get file modification time function
pub struct FileModified {
    context: TemplateContext,
}

impl FileModified {
    pub fn new(context: TemplateContext) -> Self {
        FileModified { context }
    }
}

impl Function for FileModified {
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value> {
        let path = args.get("path").and_then(|v| v.as_str()).ok_or_else(|| {
            tera::Error::msg("file_modified requires a 'path' argument (e.g., path=\"file.txt\")")
        })?;

        // Security: Prevent accessing absolute paths or paths with parent directory traversal (unless trust mode is enabled)
        if !self.context.is_trust_mode() && (path.starts_with('/') || path.contains("..")) {
            return Err(tera::Error::msg(format!(
                "Security: Absolute paths and parent directory (..) access are not allowed: {}. Use --trust to bypass this restriction.",
                path
            )));
        }

        // Resolve path relative to template's base directory
        let resolved_path = self.context.resolve_path(path);

        let metadata = fs::metadata(&resolved_path).map_err(|e| {
            tera::Error::msg(format!(
                "Failed to get file metadata for '{}': {}",
                resolved_path.display(),
                e
            ))
        })?;

        let modified = metadata
            .modified()
            .map_err(|e| tera::Error::msg(format!("Failed to get modification time: {}", e)))?;

        // Convert to Unix timestamp (seconds since epoch)
        let duration = modified
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| tera::Error::msg(format!("Failed to convert timestamp: {}", e)))?;

        let timestamp = duration.as_secs();

        to_value(timestamp)
            .map_err(|e| tera::Error::msg(format!("Failed to convert result: {}", e)))
    }
}
