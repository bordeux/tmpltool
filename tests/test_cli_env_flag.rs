//! Integration tests for the --env CLI flag
//!
//! These tests verify the .env file loading functionality works correctly.

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::io::Write;
use tempfile::TempDir;

#[allow(deprecated)]
fn tmpltool() -> Command {
    Command::cargo_bin("tmpltool").unwrap()
}

/// Helper to create a temporary .env file
fn create_env_file(dir: &TempDir, name: &str, content: &str) -> String {
    let path = dir.path().join(name);
    let mut file = fs::File::create(&path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
    path.to_str().unwrap().to_string()
}

/// Helper to create a temporary template file
fn create_template_file(dir: &TempDir, name: &str, content: &str) -> String {
    let path = dir.path().join(name);
    let mut file = fs::File::create(&path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
    path.to_str().unwrap().to_string()
}

#[test]
fn test_env_flag_loads_variables() {
    let dir = TempDir::new().unwrap();

    let env_file = create_env_file(&dir, ".env", "TEST_VAR=hello_world\n");
    let template = create_template_file(
        &dir,
        "test.tmpltool",
        "Value: {{ get_env(name=\"TEST_VAR\", default=\"not_set\") }}",
    );

    tmpltool()
        .args(["--env", &env_file, &template])
        .assert()
        .success()
        .stdout(predicate::str::contains("Value: hello_world"));
}

#[test]
fn test_env_flag_multiple_variables() {
    let dir = TempDir::new().unwrap();

    let env_file = create_env_file(&dir, ".env", "VAR1=first\nVAR2=second\nVAR3=third\n");
    let template = create_template_file(
        &dir,
        "test.tmpltool",
        "{{ get_env(name=\"VAR1\") }}-{{ get_env(name=\"VAR2\") }}-{{ get_env(name=\"VAR3\") }}",
    );

    tmpltool()
        .args(["--env", &env_file, &template])
        .assert()
        .success()
        .stdout(predicate::str::contains("first-second-third"));
}

#[test]
fn test_env_flag_multiple_files() {
    let dir = TempDir::new().unwrap();

    let env1 = create_env_file(&dir, ".env", "VAR1=from_env1\nVAR2=from_env1\n");
    let env2 = create_env_file(&dir, ".env.local", "VAR2=from_env2\nVAR3=from_env2\n");
    let template = create_template_file(
        &dir,
        "test.tmpltool",
        "VAR1={{ get_env(name=\"VAR1\") }} VAR2={{ get_env(name=\"VAR2\") }} VAR3={{ get_env(name=\"VAR3\") }}",
    );

    tmpltool()
        .args(["--env", &env1, "--env", &env2, &template])
        .assert()
        .success()
        .stdout(predicate::str::contains("VAR1=from_env1"))
        .stdout(predicate::str::contains("VAR2=from_env2")) // Later file overrides
        .stdout(predicate::str::contains("VAR3=from_env2"));
}

#[test]
fn test_env_flag_override_order() {
    let dir = TempDir::new().unwrap();

    let env1 = create_env_file(&dir, "first.env", "VALUE=first\n");
    let env2 = create_env_file(&dir, "second.env", "VALUE=second\n");
    let env3 = create_env_file(&dir, "third.env", "VALUE=third\n");
    let template = create_template_file(&dir, "test.tmpltool", "{{ get_env(name=\"VALUE\") }}");

    // Last file wins
    tmpltool()
        .args(["--env", &env1, "--env", &env2, "--env", &env3, &template])
        .assert()
        .success()
        .stdout(predicate::str::contains("third"));
}

#[test]
fn test_env_flag_missing_file() {
    let dir = TempDir::new().unwrap();

    let template = create_template_file(&dir, "test.tmpltool", "{{ get_env(name=\"X\") }}");

    tmpltool()
        .args(["--env", "/nonexistent/.env", &template])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Environment file not found"));
}

#[test]
fn test_env_flag_with_comments() {
    let dir = TempDir::new().unwrap();

    let env_file = create_env_file(
        &dir,
        ".env",
        "# This is a comment\nVAR=value\n# Another comment\n",
    );
    let template = create_template_file(&dir, "test.tmpltool", "{{ get_env(name=\"VAR\") }}");

    tmpltool()
        .args(["--env", &env_file, &template])
        .assert()
        .success()
        .stdout(predicate::str::contains("value"));
}

#[test]
fn test_env_flag_with_quoted_values() {
    let dir = TempDir::new().unwrap();

    let env_file = create_env_file(
        &dir,
        ".env",
        "QUOTED=\"hello world\"\nSINGLE='single quoted'\n",
    );
    let template = create_template_file(
        &dir,
        "test.tmpltool",
        "{{ get_env(name=\"QUOTED\") }} | {{ get_env(name=\"SINGLE\") }}",
    );

    tmpltool()
        .args(["--env", &env_file, &template])
        .assert()
        .success()
        .stdout(predicate::str::contains("hello world"))
        .stdout(predicate::str::contains("single quoted"));
}

#[test]
fn test_env_flag_with_empty_value() {
    let dir = TempDir::new().unwrap();

    let env_file = create_env_file(&dir, ".env", "EMPTY=\n");
    let template = create_template_file(&dir, "test.tmpltool", "[{{ get_env(name=\"EMPTY\") }}]");

    tmpltool()
        .args(["--env", &env_file, &template])
        .assert()
        .success()
        .stdout(predicate::str::contains("[]"));
}

#[test]
fn test_env_flag_with_special_characters() {
    let dir = TempDir::new().unwrap();

    let env_file = create_env_file(
        &dir,
        ".env",
        "URL=https://example.com/path?key=value&other=123\n",
    );
    let template = create_template_file(&dir, "test.tmpltool", "{{ get_env(name=\"URL\") }}");

    tmpltool()
        .args(["--env", &env_file, &template])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "https://example.com/path?key=value&other=123",
        ));
}

#[test]
fn test_env_flag_with_multiline_value() {
    let dir = TempDir::new().unwrap();

    // dotenvy supports multiline values with quotes
    let env_file = create_env_file(&dir, ".env", "MULTI=\"line1\nline2\nline3\"\n");
    let template = create_template_file(&dir, "test.tmpltool", "{{ get_env(name=\"MULTI\") }}");

    tmpltool()
        .args(["--env", &env_file, &template])
        .assert()
        .success()
        .stdout(predicate::str::contains("line1"))
        .stdout(predicate::str::contains("line2"))
        .stdout(predicate::str::contains("line3"));
}

#[test]
fn test_env_flag_with_stdin_template() {
    let dir = TempDir::new().unwrap();

    let env_file = create_env_file(&dir, ".env", "STDIN_VAR=from_stdin_test\n");

    tmpltool()
        .args(["--env", &env_file])
        .write_stdin("Value: {{ get_env(name=\"STDIN_VAR\") }}")
        .assert()
        .success()
        .stdout(predicate::str::contains("Value: from_stdin_test"));
}

#[test]
fn test_env_flag_combined_with_output() {
    let dir = TempDir::new().unwrap();

    let env_file = create_env_file(&dir, ".env", "OUTPUT_VAR=test_output\n");
    let template =
        create_template_file(&dir, "test.tmpltool", "{{ get_env(name=\"OUTPUT_VAR\") }}");
    let output_file = dir.path().join("output.txt");

    tmpltool()
        .args([
            "--env",
            &env_file,
            "-o",
            output_file.to_str().unwrap(),
            &template,
        ])
        .assert()
        .success();

    let output = fs::read_to_string(&output_file).unwrap();
    assert_eq!(output, "test_output");
}

#[test]
fn test_env_flag_combined_with_validate() {
    let dir = TempDir::new().unwrap();

    let env_file = create_env_file(&dir, ".env", "APP_NAME=myapp\nPORT=8080\n");
    let template = create_template_file(
        &dir,
        "test.tmpltool",
        "{\"app\": \"{{ get_env(name=\"APP_NAME\") }}\", \"port\": {{ get_env(name=\"PORT\") }}}",
    );

    tmpltool()
        .args(["--env", &env_file, "--validate", "json", &template])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"app\": \"myapp\""))
        .stdout(predicate::str::contains("\"port\": 8080"));
}

#[test]
fn test_env_flag_no_env_files_works() {
    let dir = TempDir::new().unwrap();

    let template = create_template_file(
        &dir,
        "test.tmpltool",
        "{{ get_env(name=\"MISSING\", default=\"default_value\") }}",
    );

    // Without --env flag, tool should still work
    tmpltool()
        .args([&template])
        .assert()
        .success()
        .stdout(predicate::str::contains("default_value"));
}

#[test]
fn test_env_flag_help_shows_option() {
    tmpltool()
        .args(["--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--env"))
        .stdout(predicate::str::contains("env file"));
}
