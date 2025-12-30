/// File system functions
///
/// Provides functions for interacting with the file system:
/// - read_file: Read file contents
/// - file_exists: Check if file exists
/// - list_dir: List directory contents
/// - glob: List files by pattern
/// - file_size: Get file size
/// - file_modified: Get file modification timestamp
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tera::{to_value, Function, Result, Value};

/// Read file content function
pub struct ReadFile;

impl Function for ReadFile {
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value> {
        let path = args
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                tera::Error::msg("read_file requires a 'path' argument (e.g., path=\"config.txt\")")
            })?;

        // Security: Prevent reading absolute paths or paths with parent directory traversal
        if path.starts_with('/') || path.contains("..") {
            return Err(tera::Error::msg(format!(
                "Security: Absolute paths and parent directory (..) access are not allowed: {}",
                path
            )));
        }

        let content = fs::read_to_string(path).map_err(|e| {
            tera::Error::msg(format!("Failed to read file '{}': {}", path, e))
        })?;

        to_value(&content).map_err(|e| tera::Error::msg(format!("Failed to convert content: {}", e)))
    }
}

/// Check if file exists function
pub struct FileExists;

impl Function for FileExists {
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value> {
        let path = args
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                tera::Error::msg("file_exists requires a 'path' argument (e.g., path=\"file.txt\")")
            })?;

        // Security: Prevent checking absolute paths or paths with parent directory traversal
        if path.starts_with('/') || path.contains("..") {
            return Err(tera::Error::msg(format!(
                "Security: Absolute paths and parent directory (..) access are not allowed: {}",
                path
            )));
        }

        let exists = Path::new(path).exists();

        to_value(&exists).map_err(|e| tera::Error::msg(format!("Failed to convert result: {}", e)))
    }
}

/// List directory contents function
pub struct ListDir;

impl Function for ListDir {
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value> {
        let path = args
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                tera::Error::msg("list_dir requires a 'path' argument (e.g., path=\"./data\")")
            })?;

        // Security: Prevent listing absolute paths or paths with parent directory traversal
        if path.starts_with('/') || path.contains("..") {
            return Err(tera::Error::msg(format!(
                "Security: Absolute paths and parent directory (..) access are not allowed: {}",
                path
            )));
        }

        let entries = fs::read_dir(path)
            .map_err(|e| tera::Error::msg(format!("Failed to read directory '{}': {}", path, e)))?;

        let mut files: Vec<String> = Vec::new();
        for entry in entries {
            let entry = entry.map_err(|e| {
                tera::Error::msg(format!("Failed to read directory entry: {}", e))
            })?;
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
pub struct GlobFiles;

impl Function for GlobFiles {
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value> {
        let pattern = args
            .get("pattern")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                tera::Error::msg("glob requires a 'pattern' argument (e.g., pattern=\"*.txt\")")
            })?;

        // Security: Prevent absolute paths or paths with parent directory traversal
        if pattern.starts_with('/') || pattern.contains("..") {
            return Err(tera::Error::msg(format!(
                "Security: Absolute paths and parent directory (..) access are not allowed: {}",
                pattern
            )));
        }

        let glob_result = glob::glob(pattern)
            .map_err(|e| tera::Error::msg(format!("Invalid glob pattern '{}': {}", pattern, e)))?;

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
pub struct FileSize;

impl Function for FileSize {
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value> {
        let path = args
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                tera::Error::msg("file_size requires a 'path' argument (e.g., path=\"data.bin\")")
            })?;

        // Security: Prevent accessing absolute paths or paths with parent directory traversal
        if path.starts_with('/') || path.contains("..") {
            return Err(tera::Error::msg(format!(
                "Security: Absolute paths and parent directory (..) access are not allowed: {}",
                path
            )));
        }

        let metadata = fs::metadata(path)
            .map_err(|e| tera::Error::msg(format!("Failed to get file metadata for '{}': {}", path, e)))?;

        let size = metadata.len();

        to_value(&size).map_err(|e| tera::Error::msg(format!("Failed to convert result: {}", e)))
    }
}

/// Get file modification time function
pub struct FileModified;

impl Function for FileModified {
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value> {
        let path = args
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                tera::Error::msg("file_modified requires a 'path' argument (e.g., path=\"file.txt\")")
            })?;

        // Security: Prevent accessing absolute paths or paths with parent directory traversal
        if path.starts_with('/') || path.contains("..") {
            return Err(tera::Error::msg(format!(
                "Security: Absolute paths and parent directory (..) access are not allowed: {}",
                path
            )));
        }

        let metadata = fs::metadata(path)
            .map_err(|e| tera::Error::msg(format!("Failed to get file metadata for '{}': {}", path, e)))?;

        let modified = metadata
            .modified()
            .map_err(|e| tera::Error::msg(format!("Failed to get modification time: {}", e)))?;

        // Convert to Unix timestamp (seconds since epoch)
        let duration = modified
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| tera::Error::msg(format!("Failed to convert timestamp: {}", e)))?;

        let timestamp = duration.as_secs();

        to_value(&timestamp).map_err(|e| tera::Error::msg(format!("Failed to convert result: {}", e)))
    }
}
