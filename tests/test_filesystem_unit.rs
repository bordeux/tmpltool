use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::sync::atomic::{AtomicU32, Ordering};
use tera::{Function, Value};
use tmpltool::functions::filesystem::{
    FileExists, FileModified, FileSize, GlobFiles, ListDir, ReadFile,
};

// Global counter for unique test directories
static TEST_COUNTER: AtomicU32 = AtomicU32::new(0);

// Helper to create a unique test directory
fn get_test_dir() -> String {
    let counter = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
    format!("test_data_{}", counter)
}

// Helper to create a temporary test file
fn create_test_file(test_dir: &str, name: &str, content: &str) -> String {
    let path = format!("{}/{}", test_dir, name);
    fs::create_dir_all(test_dir).unwrap();
    let mut file = fs::File::create(&path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
    path
}

// Helper to cleanup test directory
fn cleanup_test_dir(test_dir: &str) {
    let _ = fs::remove_dir_all(test_dir);
}

#[test]
fn test_read_file_basic() {
    let test_dir = get_test_dir();
    let path = create_test_file(&test_dir, "test.txt", "Hello, World!");

    let mut args = HashMap::new();
    args.insert("path".to_string(), Value::String(path.clone()));

    let result = ReadFile::new(false).call(&args).unwrap();
    assert_eq!(result.as_str().unwrap(), "Hello, World!");

    cleanup_test_dir(&test_dir);
}

#[test]
fn test_read_file_multiline() {
    let test_dir = get_test_dir();
    let content = "Line 1\nLine 2\nLine 3";
    let path = create_test_file(&test_dir, "multiline.txt", content);

    let mut args = HashMap::new();
    args.insert("path".to_string(), Value::String(path.clone()));

    let result = ReadFile::new(false).call(&args).unwrap();
    assert_eq!(result.as_str().unwrap(), content);

    cleanup_test_dir(&test_dir);
}

#[test]
fn test_read_file_missing_argument() {
    let args = HashMap::new();
    let result = ReadFile::new(false).call(&args);
    assert!(result.is_err());
    assert!(
        result
            .err()
            .unwrap()
            .to_string()
            .contains("requires a 'path' argument")
    );
}

#[test]
fn test_read_file_nonexistent() {
    let mut args = HashMap::new();
    args.insert(
        "path".to_string(),
        Value::String("test_data/nonexistent.txt".to_string()),
    );

    let result = ReadFile::new(false).call(&args);
    assert!(result.is_err());
}

#[test]
fn test_read_file_security_absolute_path() {
    let mut args = HashMap::new();
    args.insert("path".to_string(), Value::String("/etc/passwd".to_string()));

    let result = ReadFile::new(false).call(&args);
    assert!(result.is_err());
    assert!(result.err().unwrap().to_string().contains("Security"));
}

#[test]
fn test_read_file_security_parent_directory() {
    let mut args = HashMap::new();
    args.insert(
        "path".to_string(),
        Value::String("../../../etc/passwd".to_string()),
    );

    let result = ReadFile::new(false).call(&args);
    assert!(result.is_err());
    assert!(result.err().unwrap().to_string().contains("Security"));
}

#[test]
fn test_file_exists_true() {
    let test_dir = get_test_dir();
    let path = create_test_file(&test_dir, "exists.txt", "content");

    let mut args = HashMap::new();
    args.insert("path".to_string(), Value::String(path.clone()));

    let result = FileExists::new(false).call(&args).unwrap();
    assert!(result.as_bool().unwrap());

    cleanup_test_dir(&test_dir);
}

#[test]
fn test_file_exists_false() {
    let test_dir = get_test_dir();

    let mut args = HashMap::new();
    args.insert(
        "path".to_string(),
        Value::String(format!("{}/nonexistent.txt", test_dir)),
    );

    let result = FileExists::new(false).call(&args).unwrap();
    assert!(!result.as_bool().unwrap());
}

#[test]
fn test_file_exists_security() {
    let mut args = HashMap::new();
    args.insert("path".to_string(), Value::String("/etc/passwd".to_string()));

    let result = FileExists::new(false).call(&args);
    assert!(result.is_err());
    assert!(result.err().unwrap().to_string().contains("Security"));
}

#[test]
fn test_list_dir_basic() {
    let test_dir = get_test_dir();
    fs::create_dir_all(&test_dir).unwrap();
    create_test_file(&test_dir, "file1.txt", "content1");
    create_test_file(&test_dir, "file2.txt", "content2");
    create_test_file(&test_dir, "file3.txt", "content3");

    let mut args = HashMap::new();
    args.insert("path".to_string(), Value::String(test_dir.clone()));

    let result = ListDir::new(false).call(&args).unwrap();
    let files = result.as_array().unwrap();

    assert_eq!(files.len(), 3);
    assert_eq!(files[0].as_str().unwrap(), "file1.txt");
    assert_eq!(files[1].as_str().unwrap(), "file2.txt");
    assert_eq!(files[2].as_str().unwrap(), "file3.txt");

    cleanup_test_dir(&test_dir);
}

#[test]
fn test_list_dir_empty() {
    let test_dir = get_test_dir();
    fs::create_dir_all(&test_dir).unwrap();

    let mut args = HashMap::new();
    args.insert("path".to_string(), Value::String(test_dir.clone()));

    let result = ListDir::new(false).call(&args).unwrap();
    let files = result.as_array().unwrap();

    assert_eq!(files.len(), 0);

    cleanup_test_dir(&test_dir);
}

#[test]
fn test_list_dir_nonexistent() {
    let test_dir = get_test_dir();

    let mut args = HashMap::new();
    args.insert("path".to_string(), Value::String(test_dir.clone()));

    let result = ListDir::new(false).call(&args);
    assert!(result.is_err());
}

#[test]
fn test_list_dir_security() {
    let mut args = HashMap::new();
    args.insert("path".to_string(), Value::String("/etc".to_string()));

    let result = ListDir::new(false).call(&args);
    assert!(result.is_err());
    assert!(result.err().unwrap().to_string().contains("Security"));
}

#[test]
fn test_glob_basic() {
    let test_dir = get_test_dir();
    fs::create_dir_all(&test_dir).unwrap();
    create_test_file(&test_dir, "file1.txt", "content1");
    create_test_file(&test_dir, "file2.txt", "content2");
    create_test_file(&test_dir, "file1.md", "content3");

    let mut args = HashMap::new();
    args.insert(
        "pattern".to_string(),
        Value::String(format!("{}/*.txt", test_dir)),
    );

    let result = GlobFiles::new(false).call(&args).unwrap();
    let files = result.as_array().unwrap();

    assert_eq!(files.len(), 2);
    assert!(files[0].as_str().unwrap().contains("file1.txt"));
    assert!(files[1].as_str().unwrap().contains("file2.txt"));

    cleanup_test_dir(&test_dir);
}

#[test]
fn test_glob_no_matches() {
    let test_dir = get_test_dir();

    let mut args = HashMap::new();
    args.insert(
        "pattern".to_string(),
        Value::String(format!("{}/*.xyz", test_dir)),
    );

    let result = GlobFiles::new(false).call(&args).unwrap();
    let files = result.as_array().unwrap();

    assert_eq!(files.len(), 0);
}

#[test]
fn test_glob_security() {
    let mut args = HashMap::new();
    args.insert("pattern".to_string(), Value::String("/etc/*".to_string()));

    let result = GlobFiles::new(false).call(&args);
    assert!(result.is_err());
    assert!(result.err().unwrap().to_string().contains("Security"));
}

#[test]
fn test_file_size_basic() {
    let test_dir = get_test_dir();
    let content = "Hello, World!"; // 13 bytes
    let path = create_test_file(&test_dir, "size_test.txt", content);

    let mut args = HashMap::new();
    args.insert("path".to_string(), Value::String(path.clone()));

    let result = FileSize::new(false).call(&args).unwrap();
    assert_eq!(result.as_u64().unwrap(), 13);

    cleanup_test_dir(&test_dir);
}

#[test]
fn test_file_size_empty() {
    let test_dir = get_test_dir();
    let path = create_test_file(&test_dir, "empty.txt", "");

    let mut args = HashMap::new();
    args.insert("path".to_string(), Value::String(path.clone()));

    let result = FileSize::new(false).call(&args).unwrap();
    assert_eq!(result.as_u64().unwrap(), 0);

    cleanup_test_dir(&test_dir);
}

#[test]
fn test_file_size_nonexistent() {
    let mut args = HashMap::new();
    args.insert(
        "path".to_string(),
        Value::String("test_data/nonexistent.txt".to_string()),
    );

    let result = FileSize::new(false).call(&args);
    assert!(result.is_err());
}

#[test]
fn test_file_size_security() {
    let mut args = HashMap::new();
    args.insert("path".to_string(), Value::String("/etc/passwd".to_string()));

    let result = FileSize::new(false).call(&args);
    assert!(result.is_err());
    assert!(result.err().unwrap().to_string().contains("Security"));
}

#[test]
fn test_file_modified_basic() {
    let test_dir = get_test_dir();
    let path = create_test_file(&test_dir, "modified_test.txt", "content");

    // Small delay to ensure file is created
    std::thread::sleep(std::time::Duration::from_millis(10));

    let mut args = HashMap::new();
    args.insert("path".to_string(), Value::String(path.clone()));

    let result = FileModified::new(false).call(&args).unwrap();
    let timestamp = result.as_u64().unwrap();

    // Timestamp should be recent (within last minute)
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    assert!(timestamp > 0);
    assert!(timestamp <= now);
    assert!(now - timestamp < 60); // Created within last 60 seconds

    cleanup_test_dir(&test_dir);
}

#[test]
fn test_file_modified_nonexistent() {
    let mut args = HashMap::new();
    args.insert(
        "path".to_string(),
        Value::String("test_data/nonexistent.txt".to_string()),
    );

    let result = FileModified::new(false).call(&args);
    assert!(result.is_err());
}

#[test]
fn test_file_modified_security() {
    let mut args = HashMap::new();
    args.insert("path".to_string(), Value::String("/etc/passwd".to_string()));

    let result = FileModified::new(false).call(&args);
    assert!(result.is_err());
    assert!(result.err().unwrap().to_string().contains("Security"));
}

// Trust mode tests

#[test]
fn test_read_file_trust_mode_allows_absolute_path() {
    let mut args = HashMap::new();
    // Use a file that should exist on most systems
    args.insert("path".to_string(), Value::String("/etc/hosts".to_string()));

    // Without trust mode, should fail
    let result_no_trust = ReadFile::new(false).call(&args);
    assert!(result_no_trust.is_err());
    assert!(
        result_no_trust
            .err()
            .unwrap()
            .to_string()
            .contains("Security")
    );

    // With trust mode, should succeed (or fail with file not found, but not security error)
    let result_trust = ReadFile::new(true).call(&args);
    // Result might succeed or fail depending on file existence/permissions, but should not be a security error
    if let Err(e) = result_trust {
        assert!(!e.to_string().contains("Security"));
    }
}

#[test]
fn test_file_exists_trust_mode_allows_absolute_path() {
    let mut args = HashMap::new();
    args.insert("path".to_string(), Value::String("/etc".to_string()));

    // Without trust mode, should fail
    let result_no_trust = FileExists::new(false).call(&args);
    assert!(result_no_trust.is_err());
    assert!(
        result_no_trust
            .err()
            .unwrap()
            .to_string()
            .contains("Security")
    );

    // With trust mode, should succeed
    let result_trust = FileExists::new(true).call(&args);
    assert!(result_trust.is_ok());
    // /etc should exist on Unix systems
    #[cfg(unix)]
    assert!(result_trust.unwrap().as_bool().unwrap());
}

#[test]
fn test_list_dir_trust_mode_allows_parent_directory() {
    let test_dir = get_test_dir();
    fs::create_dir_all(&test_dir).unwrap();

    let mut args = HashMap::new();
    // Try to access parent directory
    args.insert(
        "path".to_string(),
        Value::String(format!("{}/..", test_dir)),
    );

    // Without trust mode, should fail
    let result_no_trust = ListDir::new(false).call(&args);
    assert!(result_no_trust.is_err());
    assert!(
        result_no_trust
            .err()
            .unwrap()
            .to_string()
            .contains("Security")
    );

    // With trust mode, should succeed
    let result_trust = ListDir::new(true).call(&args);
    assert!(result_trust.is_ok());

    cleanup_test_dir(&test_dir);
}

#[test]
fn test_glob_trust_mode_allows_absolute_path() {
    let mut args = HashMap::new();
    args.insert(
        "pattern".to_string(),
        Value::String("/etc/host*".to_string()),
    );

    // Without trust mode, should fail
    let result_no_trust = GlobFiles::new(false).call(&args);
    assert!(result_no_trust.is_err());
    assert!(
        result_no_trust
            .err()
            .unwrap()
            .to_string()
            .contains("Security")
    );

    // With trust mode, should succeed
    let result_trust = GlobFiles::new(true).call(&args);
    assert!(result_trust.is_ok());
}

#[test]
fn test_file_size_trust_mode_allows_absolute_path() {
    let mut args = HashMap::new();
    args.insert("path".to_string(), Value::String("/etc/hosts".to_string()));

    // Without trust mode, should fail
    let result_no_trust = FileSize::new(false).call(&args);
    assert!(result_no_trust.is_err());
    assert!(
        result_no_trust
            .err()
            .unwrap()
            .to_string()
            .contains("Security")
    );

    // With trust mode, should succeed (or fail with file not found, but not security error)
    let result_trust = FileSize::new(true).call(&args);
    if let Err(e) = result_trust {
        assert!(!e.to_string().contains("Security"));
    }
}

#[test]
fn test_file_modified_trust_mode_allows_absolute_path() {
    let mut args = HashMap::new();
    args.insert("path".to_string(), Value::String("/etc/hosts".to_string()));

    // Without trust mode, should fail
    let result_no_trust = FileModified::new(false).call(&args);
    assert!(result_no_trust.is_err());
    assert!(
        result_no_trust
            .err()
            .unwrap()
            .to_string()
            .contains("Security")
    );

    // With trust mode, should succeed (or fail with file not found, but not security error)
    let result_trust = FileModified::new(true).call(&args);
    if let Err(e) = result_trust {
        assert!(!e.to_string().contains("Security"));
    }
}
