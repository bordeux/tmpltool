use minijinja::value::Kwargs;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use tmpltool::TemplateContext;
use tmpltool::functions::ContextFunction;
use tmpltool::functions::filesystem::{
    FileExists, FileModified, FileSize, Glob, ListDir, ReadFile,
};

// Global counter for unique test directories
static TEST_COUNTER: AtomicU32 = AtomicU32::new(0);

// Helper to create kwargs for testing
fn create_kwargs(args: Vec<(&str, &str)>) -> Kwargs {
    Kwargs::from_iter(args.iter().map(|(k, v)| (*k, minijinja::Value::from(*v))))
}

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

    let context = Arc::new(TemplateContext::new(PathBuf::from("."), false));

    let kwargs = create_kwargs(vec![("path", &path)]);

    let result = ReadFile::call(context.clone(), kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "Hello, World!");

    cleanup_test_dir(&test_dir);
}

#[test]
fn test_read_file_multiline() {
    let test_dir = get_test_dir();
    let content = "Line 1\nLine 2\nLine 3";
    let path = create_test_file(&test_dir, "multiline.txt", content);

    let context = Arc::new(TemplateContext::new(PathBuf::from("."), false));

    let kwargs = create_kwargs(vec![("path", &path)]);

    let result = ReadFile::call(context.clone(), kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), content);

    cleanup_test_dir(&test_dir);
}

#[test]
fn test_read_file_missing_argument() {
    let context = Arc::new(TemplateContext::new(PathBuf::from("."), false));

    let kwargs = create_kwargs(vec![]);

    let result = ReadFile::call(context.clone(), kwargs);
    assert!(result.is_err());
}

#[test]
fn test_read_file_nonexistent() {
    let context = Arc::new(TemplateContext::new(PathBuf::from("."), false));

    let kwargs = create_kwargs(vec![("path", "test_data/nonexistent.txt")]);

    let result = ReadFile::call(context.clone(), kwargs);
    assert!(result.is_err());
}

#[test]
fn test_read_file_security_absolute_path() {
    let context = Arc::new(TemplateContext::new(PathBuf::from("."), false));

    let kwargs = create_kwargs(vec![("path", "/etc/passwd")]);

    let result = ReadFile::call(context.clone(), kwargs);
    assert!(result.is_err());
    assert!(result.err().unwrap().to_string().contains("Security"));
}

#[test]
fn test_read_file_security_parent_directory() {
    let context = Arc::new(TemplateContext::new(PathBuf::from("."), false));

    let kwargs = create_kwargs(vec![("path", "../../../etc/passwd")]);

    let result = ReadFile::call(context.clone(), kwargs);
    assert!(result.is_err());
    assert!(result.err().unwrap().to_string().contains("Security"));
}

#[test]
fn test_file_exists_true() {
    let test_dir = get_test_dir();
    let path = create_test_file(&test_dir, "exists.txt", "content");

    let context = Arc::new(TemplateContext::new(PathBuf::from("."), false));

    let kwargs = create_kwargs(vec![("path", &path)]);

    let result = FileExists::call(context.clone(), kwargs).unwrap();
    assert!(result.is_true());

    cleanup_test_dir(&test_dir);
}

#[test]
fn test_file_exists_false() {
    let test_dir = get_test_dir();

    let context = Arc::new(TemplateContext::new(PathBuf::from("."), false));

    let path = format!("{}/nonexistent.txt", test_dir);
    let kwargs = create_kwargs(vec![("path", &path)]);

    let result = FileExists::call(context.clone(), kwargs).unwrap();
    assert!(!result.is_true());
}

#[test]
fn test_file_exists_security() {
    let context = Arc::new(TemplateContext::new(PathBuf::from("."), false));

    let kwargs = create_kwargs(vec![("path", "/etc/passwd")]);

    let result = FileExists::call(context.clone(), kwargs);
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

    let context = Arc::new(TemplateContext::new(PathBuf::from("."), false));

    let kwargs = create_kwargs(vec![("path", &test_dir)]);

    let result = ListDir::call(context.clone(), kwargs).unwrap();

    assert_eq!(result.len(), Some(3));
    assert_eq!(
        result
            .get_item(&minijinja::Value::from(0))
            .unwrap()
            .as_str()
            .unwrap(),
        "file1.txt"
    );
    assert_eq!(
        result
            .get_item(&minijinja::Value::from(1))
            .unwrap()
            .as_str()
            .unwrap(),
        "file2.txt"
    );
    assert_eq!(
        result
            .get_item(&minijinja::Value::from(2))
            .unwrap()
            .as_str()
            .unwrap(),
        "file3.txt"
    );

    cleanup_test_dir(&test_dir);
}

#[test]
fn test_list_dir_empty() {
    let test_dir = get_test_dir();
    fs::create_dir_all(&test_dir).unwrap();

    let context = Arc::new(TemplateContext::new(PathBuf::from("."), false));

    let kwargs = create_kwargs(vec![("path", &test_dir)]);

    let result = ListDir::call(context.clone(), kwargs).unwrap();

    assert_eq!(result.len(), Some(0));

    cleanup_test_dir(&test_dir);
}

#[test]
fn test_list_dir_nonexistent() {
    let test_dir = get_test_dir();

    let context = Arc::new(TemplateContext::new(PathBuf::from("."), false));

    let kwargs = create_kwargs(vec![("path", &test_dir)]);

    let result = ListDir::call(context.clone(), kwargs);
    assert!(result.is_err());
}

#[test]
fn test_list_dir_security() {
    let context = Arc::new(TemplateContext::new(PathBuf::from("."), false));

    let kwargs = create_kwargs(vec![("path", "/etc")]);

    let result = ListDir::call(context.clone(), kwargs);
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

    let context = Arc::new(TemplateContext::new(PathBuf::from("."), false));
    let pattern = format!("{}/*.txt", test_dir);
    let kwargs = create_kwargs(vec![("pattern", &pattern)]);

    let result = Glob::call(context, kwargs).unwrap();

    assert_eq!(result.len(), Some(2));
    assert!(
        result
            .get_item(&minijinja::Value::from(0))
            .unwrap()
            .as_str()
            .unwrap()
            .contains("file1.txt")
    );
    assert!(
        result
            .get_item(&minijinja::Value::from(1))
            .unwrap()
            .as_str()
            .unwrap()
            .contains("file2.txt")
    );

    cleanup_test_dir(&test_dir);
}

#[test]
fn test_glob_no_matches() {
    let test_dir = get_test_dir();

    let context = Arc::new(TemplateContext::new(PathBuf::from("."), false));
    let pattern = format!("{}/*.xyz", test_dir);
    let kwargs = create_kwargs(vec![("pattern", &pattern)]);

    let result = Glob::call(context, kwargs).unwrap();

    assert_eq!(result.len(), Some(0));
}

#[test]
fn test_glob_security() {
    let context = Arc::new(TemplateContext::new(PathBuf::from("."), false));
    let kwargs = create_kwargs(vec![("pattern", "/etc/*")]);

    let result = Glob::call(context, kwargs);
    assert!(result.is_err());
    assert!(result.err().unwrap().to_string().contains("Security"));
}

#[test]
fn test_file_size_basic() {
    let test_dir = get_test_dir();
    let content = "Hello, World!"; // 13 bytes
    let path = create_test_file(&test_dir, "size_test.txt", content);

    let context = Arc::new(TemplateContext::new(PathBuf::from("."), false));

    let kwargs = create_kwargs(vec![("path", &path)]);

    let result = FileSize::call(context.clone(), kwargs).unwrap();
    assert_eq!(result.as_usize().unwrap(), 13);

    cleanup_test_dir(&test_dir);
}

#[test]
fn test_file_size_empty() {
    let test_dir = get_test_dir();
    let path = create_test_file(&test_dir, "empty.txt", "");

    let context = Arc::new(TemplateContext::new(PathBuf::from("."), false));

    let kwargs = create_kwargs(vec![("path", &path)]);

    let result = FileSize::call(context.clone(), kwargs).unwrap();
    assert_eq!(result.as_usize().unwrap(), 0);

    cleanup_test_dir(&test_dir);
}

#[test]
fn test_file_size_nonexistent() {
    let context = Arc::new(TemplateContext::new(PathBuf::from("."), false));

    let kwargs = create_kwargs(vec![("path", "test_data/nonexistent.txt")]);

    let result = FileSize::call(context.clone(), kwargs);
    assert!(result.is_err());
}

#[test]
fn test_file_size_security() {
    let context = Arc::new(TemplateContext::new(PathBuf::from("."), false));

    let kwargs = create_kwargs(vec![("path", "/etc/passwd")]);

    let result = FileSize::call(context.clone(), kwargs);
    assert!(result.is_err());
    assert!(result.err().unwrap().to_string().contains("Security"));
}

#[test]
fn test_file_modified_basic() {
    let test_dir = get_test_dir();
    let path = create_test_file(&test_dir, "modified_test.txt", "content");

    // Small delay to ensure file is created
    std::thread::sleep(std::time::Duration::from_millis(10));

    let context = Arc::new(TemplateContext::new(PathBuf::from("."), false));

    let kwargs = create_kwargs(vec![("path", &path)]);

    let result = FileModified::call(context.clone(), kwargs).unwrap();
    let timestamp = result.as_usize().unwrap() as u64;

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
    let context = Arc::new(TemplateContext::new(PathBuf::from("."), false));

    let kwargs = create_kwargs(vec![("path", "test_data/nonexistent.txt")]);

    let result = FileModified::call(context.clone(), kwargs);
    assert!(result.is_err());
}

#[test]
fn test_file_modified_security() {
    let context = Arc::new(TemplateContext::new(PathBuf::from("."), false));

    let kwargs = create_kwargs(vec![("path", "/etc/passwd")]);

    let result = FileModified::call(context.clone(), kwargs);
    assert!(result.is_err());
    assert!(result.err().unwrap().to_string().contains("Security"));
}

// Trust mode tests

#[test]
fn test_read_file_trust_mode_allows_absolute_path() {
    let kwargs = create_kwargs(vec![("path", "/etc/hosts")]);

    // Without trust mode, should fail
    let context_no_trust = Arc::new(TemplateContext::new(PathBuf::from("."), false));

    let result_no_trust = ReadFile::call(context_no_trust, kwargs);
    assert!(result_no_trust.is_err());
    assert!(
        result_no_trust
            .err()
            .unwrap()
            .to_string()
            .contains("Security")
    );

    // With trust mode, should succeed (or fail with file not found, but not security error)
    let context_trust = Arc::new(TemplateContext::new(PathBuf::from("."), true));

    let kwargs_trust = create_kwargs(vec![("path", "/etc/hosts")]);
    let result_trust = ReadFile::call(context_trust, kwargs_trust);
    // Result might succeed or fail depending on file existence/permissions, but should not be a security error
    if let Err(e) = result_trust {
        assert!(!e.to_string().contains("Security"));
    }
}

#[test]
fn test_file_exists_trust_mode_allows_absolute_path() {
    let kwargs = create_kwargs(vec![("path", "/etc")]);

    // Without trust mode, should fail
    let context_no_trust = Arc::new(TemplateContext::new(PathBuf::from("."), false));
    let result_no_trust = FileExists::call(context_no_trust, kwargs);
    assert!(result_no_trust.is_err());
    assert!(
        result_no_trust
            .err()
            .unwrap()
            .to_string()
            .contains("Security")
    );

    // With trust mode, should succeed
    let context_trust = Arc::new(TemplateContext::new(PathBuf::from("."), true));
    let kwargs_trust = create_kwargs(vec![("path", "/etc")]);
    let result_trust = FileExists::call(context_trust, kwargs_trust);
    assert!(result_trust.is_ok());
    // /etc should exist on Unix systems
    #[cfg(unix)]
    assert!(result_trust.unwrap().is_true());
}

#[test]
fn test_list_dir_trust_mode_allows_parent_directory() {
    let test_dir = get_test_dir();
    fs::create_dir_all(&test_dir).unwrap();

    // Try to access parent directory
    let path = format!("{}/..", test_dir);
    let kwargs = create_kwargs(vec![("path", &path)]);

    // Without trust mode, should fail
    let context_no_trust = Arc::new(TemplateContext::new(PathBuf::from("."), false));
    let result_no_trust = ListDir::call(context_no_trust, kwargs);
    assert!(result_no_trust.is_err());
    assert!(
        result_no_trust
            .err()
            .unwrap()
            .to_string()
            .contains("Security")
    );

    // With trust mode, should succeed
    let context_trust = Arc::new(TemplateContext::new(PathBuf::from("."), true));
    let kwargs_trust = create_kwargs(vec![("path", &path)]);
    let result_trust = ListDir::call(context_trust, kwargs_trust);
    assert!(result_trust.is_ok());

    cleanup_test_dir(&test_dir);
}

#[test]
fn test_glob_trust_mode_allows_absolute_path() {
    let kwargs = create_kwargs(vec![("pattern", "/etc/host*")]);

    // Without trust mode, should fail
    let context_no_trust = Arc::new(TemplateContext::new(PathBuf::from("."), false));
    let result_no_trust = Glob::call(context_no_trust, kwargs);
    assert!(result_no_trust.is_err());
    assert!(
        result_no_trust
            .err()
            .unwrap()
            .to_string()
            .contains("Security")
    );

    // With trust mode, should succeed
    let context_trust = Arc::new(TemplateContext::new(PathBuf::from("."), true));
    let kwargs_trust = create_kwargs(vec![("pattern", "/etc/host*")]);
    let result_trust = Glob::call(context_trust, kwargs_trust);
    assert!(result_trust.is_ok());
}

#[test]
fn test_file_size_trust_mode_allows_absolute_path() {
    let kwargs = create_kwargs(vec![("path", "/etc/hosts")]);

    // Without trust mode, should fail
    let context_no_trust = Arc::new(TemplateContext::new(PathBuf::from("."), false));
    let result_no_trust = FileSize::call(context_no_trust, kwargs);
    assert!(result_no_trust.is_err());
    assert!(
        result_no_trust
            .err()
            .unwrap()
            .to_string()
            .contains("Security")
    );

    // With trust mode, should succeed (or fail with file not found, but not security error)
    let context_trust = Arc::new(TemplateContext::new(PathBuf::from("."), true));
    let kwargs_trust = create_kwargs(vec![("path", "/etc/hosts")]);
    let result_trust = FileSize::call(context_trust, kwargs_trust);
    if let Err(e) = result_trust {
        assert!(!e.to_string().contains("Security"));
    }
}

#[test]
fn test_file_modified_trust_mode_allows_absolute_path() {
    let kwargs = create_kwargs(vec![("path", "/etc/hosts")]);

    // Without trust mode, should fail
    let context_no_trust = Arc::new(TemplateContext::new(PathBuf::from("."), false));
    let result_no_trust = FileModified::call(context_no_trust, kwargs);
    assert!(result_no_trust.is_err());
    assert!(
        result_no_trust
            .err()
            .unwrap()
            .to_string()
            .contains("Security")
    );

    // With trust mode, should succeed (or fail with file not found, but not security error)
    let context_trust = Arc::new(TemplateContext::new(PathBuf::from("."), true));
    let kwargs_trust = create_kwargs(vec![("path", "/etc/hosts")]);
    let result_trust = FileModified::call(context_trust, kwargs_trust);
    if let Err(e) = result_trust {
        assert!(!e.to_string().contains("Security"));
    }
}
