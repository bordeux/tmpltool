use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

fn get_binary_path() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("target");
    path.push("debug");
    path.push("tmpltool");
    path
}

#[test]
fn test_validate_json_valid() {
    let temp_dir = TempDir::new().unwrap();
    let template_path = temp_dir.path().join("template.tmpl");
    let mut template_file = fs::File::create(&template_path).unwrap();
    writeln!(
        template_file,
        r#"{{{{ to_json(object={{"name": "test", "value": 42}}) }}}}"#
    )
    .unwrap();

    let output = Command::new(get_binary_path())
        .arg(&template_path)
        .arg("--validate")
        .arg("json")
        .output()
        .expect("Failed to execute tmpltool");

    assert!(output.status.success());
    // No output on success, only on error
}

#[test]
fn test_validate_json_invalid() {
    let temp_dir = TempDir::new().unwrap();
    let template_path = temp_dir.path().join("template.tmpl");
    let mut template_file = fs::File::create(&template_path).unwrap();
    writeln!(template_file, r#"{{"name": "test", invalid}}"#).unwrap();

    let output = Command::new(get_binary_path())
        .arg(&template_path)
        .arg("--validate")
        .arg("json")
        .output()
        .expect("Failed to execute tmpltool");

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("JSON validation failed"));
}

#[test]
fn test_validate_yaml_valid() {
    let temp_dir = TempDir::new().unwrap();
    let template_path = temp_dir.path().join("template.tmpl");
    let mut template_file = fs::File::create(&template_path).unwrap();
    writeln!(
        template_file,
        r#"{{{{ to_yaml(object={{"name": "test", "value": 42}}) }}}}"#
    )
    .unwrap();

    let output = Command::new(get_binary_path())
        .arg(&template_path)
        .arg("--validate")
        .arg("yaml")
        .output()
        .expect("Failed to execute tmpltool");

    assert!(output.status.success());
    // No output on success, only on error
}

#[test]
fn test_validate_yaml_invalid() {
    let temp_dir = TempDir::new().unwrap();
    let template_path = temp_dir.path().join("template.tmpl");
    let mut template_file = fs::File::create(&template_path).unwrap();
    writeln!(template_file, "name: test\nvalue: : invalid").unwrap();

    let output = Command::new(get_binary_path())
        .arg(&template_path)
        .arg("--validate")
        .arg("yaml")
        .output()
        .expect("Failed to execute tmpltool");

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("YAML validation failed"));
}

#[test]
fn test_validate_toml_valid() {
    let temp_dir = TempDir::new().unwrap();
    let template_path = temp_dir.path().join("template.tmpl");
    let mut template_file = fs::File::create(&template_path).unwrap();
    writeln!(
        template_file,
        r#"{{{{ to_toml(object={{"title": "Test", "version": "1.0.0"}}) }}}}"#
    )
    .unwrap();

    let output = Command::new(get_binary_path())
        .arg(&template_path)
        .arg("--validate")
        .arg("toml")
        .output()
        .expect("Failed to execute tmpltool");

    assert!(output.status.success());
    // No output on success, only on error
}

#[test]
fn test_validate_toml_invalid() {
    let temp_dir = TempDir::new().unwrap();
    let template_path = temp_dir.path().join("template.tmpl");
    let mut template_file = fs::File::create(&template_path).unwrap();
    writeln!(template_file, "name = test without quotes").unwrap();

    let output = Command::new(get_binary_path())
        .arg(&template_path)
        .arg("--validate")
        .arg("toml")
        .output()
        .expect("Failed to execute tmpltool");

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("TOML validation failed"));
}

#[test]
fn test_no_validation_by_default() {
    let temp_dir = TempDir::new().unwrap();
    let template_path = temp_dir.path().join("template.tmpl");
    let mut template_file = fs::File::create(&template_path).unwrap();
    writeln!(template_file, "Hello World").unwrap();

    let output = Command::new(get_binary_path())
        .arg(&template_path)
        .output()
        .expect("Failed to execute tmpltool");

    assert!(output.status.success());
    // Stderr should be empty when no validation
    assert!(output.stderr.is_empty());
}

#[test]
fn test_validate_json_with_output_file() {
    let temp_dir = TempDir::new().unwrap();
    let template_path = temp_dir.path().join("template.tmpl");
    let output_path = temp_dir.path().join("output.json");

    let mut template_file = fs::File::create(&template_path).unwrap();
    writeln!(
        template_file,
        r#"{{{{ to_json(object={{"test": true}}) }}}}"#
    )
    .unwrap();

    let output = Command::new(get_binary_path())
        .arg(&template_path)
        .arg("--output")
        .arg(&output_path)
        .arg("--validate")
        .arg("json")
        .output()
        .expect("Failed to execute tmpltool");

    assert!(output.status.success());
    assert!(output_path.exists());

    let content = fs::read_to_string(&output_path).unwrap();
    assert!(content.contains("test"));
}

#[test]
fn test_validate_preserves_output() {
    let temp_dir = TempDir::new().unwrap();
    let template_path = temp_dir.path().join("template.tmpl");
    let mut template_file = fs::File::create(&template_path).unwrap();
    writeln!(
        template_file,
        r#"{{{{ to_json(object={{"name": "Alice", "age": 30}}) }}}}"#
    )
    .unwrap();

    let output = Command::new(get_binary_path())
        .arg(&template_path)
        .arg("--validate")
        .arg("json")
        .output()
        .expect("Failed to execute tmpltool");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Alice"));
    assert!(stdout.contains("30"));
}
