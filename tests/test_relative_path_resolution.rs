use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, Ordering};
use tmpltool::render_template;

// Global counter for unique test directories
static TEST_COUNTER: AtomicU32 = AtomicU32::new(0);

/// Helper to create a test directory with files
fn setup_test_env() -> (PathBuf, PathBuf) {
    let counter = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
    let test_dir = env::temp_dir().join(format!(
        "tmpltool_rel_test_{}_{}",
        std::process::id(),
        counter
    ));
    fs::create_dir_all(&test_dir).unwrap();

    // Create a data file
    let data_file = test_dir.join("data.txt");
    let mut file = fs::File::create(&data_file).unwrap();
    file.write_all(b"Hello from data file!").unwrap();

    (test_dir, data_file)
}

/// Helper to create a template file
fn create_template_file(dir: &Path, name: &str, content: &str) -> PathBuf {
    let template_file = dir.join(name);
    let mut file = fs::File::create(&template_file).unwrap();
    file.write_all(content.as_bytes()).unwrap();
    template_file
}

/// Helper to cleanup test directory
fn cleanup_test_env(test_dir: &Path) {
    let _ = fs::remove_dir_all(test_dir);
}

#[test]
fn test_read_file_relative_to_template_file() {
    let (test_dir, _) = setup_test_env();

    // Create template that reads data.txt with relative path
    let template_content = "{{ read_file(path=\"./data.txt\") }}";
    let template_file = create_template_file(&test_dir, "template.tmpl", template_content);

    // Create output file
    let output_file = test_dir.join("output.txt");

    // Render template
    render_template(
        Some(template_file.to_str().unwrap()),
        Some(output_file.to_str().unwrap()),
        false,
    )
    .unwrap();

    // Verify output
    let output = fs::read_to_string(&output_file).unwrap();
    assert_eq!(output, "Hello from data file!");

    cleanup_test_env(&test_dir);
}

#[test]
fn test_read_file_relative_to_template_in_subdirectory() {
    let (test_dir, _) = setup_test_env();

    // Create subdirectory
    let subdir = test_dir.join("templates");
    fs::create_dir_all(&subdir).unwrap();

    // Create data file in subdirectory
    let data_file = subdir.join("config.txt");
    let mut file = fs::File::create(&data_file).unwrap();
    file.write_all(b"Config data").unwrap();

    // Create template in subdirectory that reads config.txt with relative path
    let template_content = "{{ read_file(path=\"./config.txt\") }}";
    let template_file = create_template_file(&subdir, "template.tmpl", template_content);

    // Create output file
    let output_file = test_dir.join("output.txt");

    // Render template
    render_template(
        Some(template_file.to_str().unwrap()),
        Some(output_file.to_str().unwrap()),
        false,
    )
    .unwrap();

    // Verify output
    let output = fs::read_to_string(&output_file).unwrap();
    assert_eq!(output, "Config data");

    cleanup_test_env(&test_dir);
}

#[test]
fn test_file_exists_relative_to_template() {
    let (test_dir, _) = setup_test_env();

    // Create template that checks if data.txt exists
    let template_content = "exists: {{ file_exists(path=\"./data.txt\") }}";
    let template_file = create_template_file(&test_dir, "template.tmpl", template_content);

    // Create output file
    let output_file = test_dir.join("output.txt");

    // Render template
    render_template(
        Some(template_file.to_str().unwrap()),
        Some(output_file.to_str().unwrap()),
        false,
    )
    .unwrap();

    // Verify output
    let output = fs::read_to_string(&output_file).unwrap();
    assert_eq!(output, "exists: true");

    cleanup_test_env(&test_dir);
}

#[test]
fn test_file_size_relative_to_template() {
    let (test_dir, _) = setup_test_env();

    // Create template that gets size of data.txt
    let template_content = "size: {{ file_size(path=\"./data.txt\") }}";
    let template_file = create_template_file(&test_dir, "template.tmpl", template_content);

    // Create output file
    let output_file = test_dir.join("output.txt");

    // Render template
    render_template(
        Some(template_file.to_str().unwrap()),
        Some(output_file.to_str().unwrap()),
        false,
    )
    .unwrap();

    // Verify output
    let output = fs::read_to_string(&output_file).unwrap();
    assert_eq!(output, "size: 21"); // "Hello from data file!" is 21 bytes

    cleanup_test_env(&test_dir);
}

#[test]
fn test_list_dir_relative_to_template() {
    let (test_dir, _) = setup_test_env();

    // Create some files in test directory
    create_template_file(&test_dir, "file1.txt", "content1");
    create_template_file(&test_dir, "file2.txt", "content2");

    // Create template that lists current directory
    let template_content = "{% for file in list_dir(path=\".\") | sort %}{{ file }}\n{% endfor %}";
    let template_file = create_template_file(&test_dir, "template.tmpl", template_content);

    // Create output file
    let output_file = test_dir.join("output.txt");

    // Render template
    render_template(
        Some(template_file.to_str().unwrap()),
        Some(output_file.to_str().unwrap()),
        false,
    )
    .unwrap();

    // Verify output contains the files
    let output = fs::read_to_string(&output_file).unwrap();
    assert!(output.contains("data.txt"));
    assert!(output.contains("file1.txt"));
    assert!(output.contains("file2.txt"));
    assert!(output.contains("template.tmpl"));

    cleanup_test_env(&test_dir);
}

#[test]
fn test_glob_relative_to_template() {
    let (test_dir, _) = setup_test_env();

    // Create some .txt files
    create_template_file(&test_dir, "test1.txt", "content1");
    create_template_file(&test_dir, "test2.txt", "content2");
    create_template_file(&test_dir, "readme.md", "markdown");

    // Create template that globs for .txt files
    let template_content =
        "{% for file in glob(pattern=\"*.txt\") | sort %}{{ file }}\n{% endfor %}";
    let template_file = create_template_file(&test_dir, "template.tmpl", template_content);

    // Create output file
    let output_file = test_dir.join("output.txt");

    // Render template
    render_template(
        Some(template_file.to_str().unwrap()),
        Some(output_file.to_str().unwrap()),
        false,
    )
    .unwrap();

    // Verify output
    let output = fs::read_to_string(&output_file).unwrap();
    assert!(output.contains("data.txt"));
    assert!(output.contains("test1.txt"));
    assert!(output.contains("test2.txt"));
    assert!(!output.contains("readme.md")); // Should not include .md files

    cleanup_test_env(&test_dir);
}

#[test]
fn test_security_restriction_prevents_parent_directory_access() {
    let (test_dir, _) = setup_test_env();

    // Create template that tries to access parent directory
    let template_content = "{{ read_file(path=\"../secret.txt\") }}";
    let template_file = create_template_file(&test_dir, "template.tmpl", template_content);

    // Create output file
    let output_file = test_dir.join("output.txt");

    // Render template - should fail due to security restriction
    let result = render_template(
        Some(template_file.to_str().unwrap()),
        Some(output_file.to_str().unwrap()),
        false,
    );

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Security"));

    cleanup_test_env(&test_dir);
}

#[test]
fn test_trust_mode_allows_parent_directory_access() {
    let (test_dir, _) = setup_test_env();

    // Create a file in parent directory
    let parent_file = test_dir.join("parent_data.txt");
    let mut file = fs::File::create(&parent_file).unwrap();
    file.write_all(b"Parent data").unwrap();

    // Create subdirectory
    let subdir = test_dir.join("subdir");
    fs::create_dir_all(&subdir).unwrap();

    // Create template in subdirectory that accesses parent directory
    let template_content = "{{ read_file(path=\"../parent_data.txt\") }}";
    let template_file = create_template_file(&subdir, "template.tmpl", template_content);

    // Create output file
    let output_file = test_dir.join("output.txt");

    // Render template with trust mode enabled
    render_template(
        Some(template_file.to_str().unwrap()),
        Some(output_file.to_str().unwrap()),
        true, // trust mode enabled
    )
    .unwrap();

    // Verify output
    let output = fs::read_to_string(&output_file).unwrap();
    assert_eq!(output, "Parent data");

    cleanup_test_env(&test_dir);
}

#[test]
fn test_file_modified_relative_to_template() {
    let (test_dir, _) = setup_test_env();

    // Create template that gets modification time of data.txt
    let template_content = "modified: {{ file_modified(path=\"./data.txt\") }}";
    let template_file = create_template_file(&test_dir, "template.tmpl", template_content);

    // Create output file
    let output_file = test_dir.join("output.txt");

    // Render template
    render_template(
        Some(template_file.to_str().unwrap()),
        Some(output_file.to_str().unwrap()),
        false,
    )
    .unwrap();

    // Verify output contains a timestamp (numeric value)
    let output = fs::read_to_string(&output_file).unwrap();
    assert!(output.starts_with("modified: "));
    let timestamp_str = output.strip_prefix("modified: ").unwrap();
    let timestamp: u64 = timestamp_str.parse().unwrap();

    // Timestamp should be recent (within last hour)
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    assert!(timestamp > now - 3600);
    assert!(timestamp <= now);

    cleanup_test_env(&test_dir);
}

#[test]
fn test_multiple_relative_reads_in_same_template() {
    let (test_dir, _) = setup_test_env();

    // Create multiple data files
    let file1 = test_dir.join("file1.txt");
    let mut f = fs::File::create(&file1).unwrap();
    f.write_all(b"Content 1").unwrap();

    let file2 = test_dir.join("file2.txt");
    let mut f = fs::File::create(&file2).unwrap();
    f.write_all(b"Content 2").unwrap();

    // Create template that reads multiple files
    let template_content = "File 1: {{ read_file(path=\"./file1.txt\") }}\nFile 2: {{ read_file(path=\"./file2.txt\") }}";
    let template_file = create_template_file(&test_dir, "template.tmpl", template_content);

    // Create output file
    let output_file = test_dir.join("output.txt");

    // Render template
    render_template(
        Some(template_file.to_str().unwrap()),
        Some(output_file.to_str().unwrap()),
        false,
    )
    .unwrap();

    // Verify output
    let output = fs::read_to_string(&output_file).unwrap();
    assert_eq!(output, "File 1: Content 1\nFile 2: Content 2");

    cleanup_test_env(&test_dir);
}
