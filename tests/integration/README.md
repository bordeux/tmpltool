# Binary Integration Tests

This directory contains integration tests that test the compiled binary itself, not the code directly.

## Purpose

While unit tests (`cargo test`) validate the code logic, these integration tests ensure that:
- The binary builds correctly across platforms
- The binary executes without errors
- All features work as expected in the compiled artifact
- CLI arguments and options function properly
- Real-world usage scenarios work end-to-end

## Running Tests

### Locally

```bash
# Build the release binary first
cargo build --release

# Run tests with the release binary
bash tests/integration/test_binary.sh

# Or specify a custom binary path
bash tests/integration/test_binary.sh /path/to/tmpltool
```

### In CI/CD

The tests run automatically in GitHub Actions for every PR and push:
- Builds binaries for Linux, macOS, and Windows
- Runs 28 integration tests on each platform
- Uploads binaries as artifacts (available for 7 days)

## Test Coverage

The integration test suite (`test_binary.sh`) includes 28 tests covering:

### Core Functionality
1. Binary execution and version info
2. Help output
3. Simple template rendering
4. Environment variable substitution
5. Default values for missing env vars
6. Conditional logic
7. Loop iteration

### Functions
8. Hash functions (MD5, SHA1, SHA256, SHA512)
9. UUID generation (format validation)
10. Timestamp functions (ISO8601 format)
11. Random number generation
12. Object manipulation (object_keys, object_values, etc.)
13. JSON serialization (to_json)
14. Filesystem functions (read_file)
15. JSON parsing (parse_json)

### Features
16. Output to file (`-o` flag)
17. Stdin input
18. Filters (upper, lower, etc.)
19. Multiple environment variables
20. Validation option (`--validate json/yaml/toml`)

### Error Handling
21. Invalid template syntax
22. Missing template files
23. Invalid JSON validation

### Real-World Scenarios
24. Complex configuration templates with conditionals and env vars

## Test Structure

Each test follows this pattern:

```bash
# Create test template
cat > "$TEST_DIR/test.tmpl" << 'EOF'
{{ template content }}
EOF

# Run binary and capture output
OUTPUT=$("$BINARY" "$TEST_DIR/test.tmpl" 2>&1)

# Assert expected result
assert_equals "expected" "$OUTPUT" "test description"
```

## Adding New Tests

To add a new integration test:

1. Add a new test section in `test_binary.sh`
2. Follow the existing pattern:
   ```bash
   echo ""
   echo "Test N: Description"
   cat > "$TEST_DIR/mytest.tmpl" << 'EOF'
   Template content here
   EOF
   OUTPUT=$("$BINARY" "$TEST_DIR/mytest.tmpl" 2>&1)
   assert_equals "expected output" "$OUTPUT" "Test passes when..."
   ```
3. Test locally before committing
4. CI will automatically run the new test

## Helper Functions

Available assertion functions:

- `assert_equals expected actual description` - Check exact match
- `assert_contains haystack needle description` - Check substring
- `assert_exit_code expected actual description` - Check exit code
- `pass description` - Mark test as passed
- `fail description details` - Mark test as failed

## Platform Differences

The tests are designed to work cross-platform (Linux, macOS, Windows):

- Use `bash` for Windows compatibility (Git Bash or WSL)
- Avoid platform-specific commands
- Use portable patterns for temp files (`mktemp -d`)
- Handle path separators appropriately

## Debugging Failed Tests

If a test fails:

1. Run the test locally with the same binary:
   ```bash
   bash tests/integration/test_binary.sh
   ```

2. Run with debug output:
   ```bash
   bash -x tests/integration/test_binary.sh 2>&1 | grep "FAIL" -A 5
   ```

3. Test individual commands manually:
   ```bash
   ./target/release/tmpltool examples/greeting.tmpl
   ```

4. Check the test's expected vs actual output in the failure message

## CI Artifacts

GitHub Actions uploads the built binaries as artifacts:
- Retention: 7 days
- Available for download from the Actions run
- Useful for testing PR changes manually
- Platform-specific naming: `tmpltool-linux-x86_64`, `tmpltool-macos`, `tmpltool-windows.exe`

## Test Philosophy

These integration tests complement unit tests by:

- **Unit tests**: Test individual functions and code paths
- **Integration tests**: Test the complete user experience with the actual binary

Both are necessary for comprehensive quality assurance.
