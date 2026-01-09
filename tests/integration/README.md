# Binary Integration Tests

This directory contains integration tests that test the compiled binary itself, not the code directly.

## Structure

```
tests/integration/
├── test_binary.sh          # Main test runner - executes all tests
├── common.sh               # Shared helper functions and utilities
├── tests/                  # Individual test files
│   ├── 01_binary_execution.sh
│   ├── 02_simple_rendering.sh
│   ├── 03_environment_variables.sh
│   ├── 04_conditionals_loops.sh
│   ├── 05_hash_functions.sh
│   ├── 06_output_and_stdin.sh
│   ├── 07_uuid_timestamp_random.sh
│   ├── 08_error_handling.sh
│   ├── 09_filesystem_functions.sh
│   ├── 10_json_and_filters.sh
│   ├── 11_object_functions.sh
│   ├── 12_serialization.sh
│   ├── 13_validation.sh
│   └── 14_complex_scenarios.sh
└── README.md               # This file
```

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

# Run all tests with the release binary
bash tests/integration/test_binary.sh

# Or specify a custom binary path
bash tests/integration/test_binary.sh /path/to/tmpltool

# Run a specific test file
source tests/integration/common.sh
export BINARY=./target/release/tmpltool
export TEST_DIR=$(mktemp -d)
bash tests/integration/tests/01_binary_execution.sh
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
cat > "$TEST_DIR/test.tmpltool" << 'EOF'
{{ template content }}
EOF

# Run binary and capture output
OUTPUT=$("$BINARY" "$TEST_DIR/test.tmpltool" 2>&1)

# Assert expected result
assert_equals "expected" "$OUTPUT" "test description"
```

## Adding New Tests

To add a new integration test:

1. Create a new file in `tests/integration/tests/` with a descriptive name (use number prefix for ordering):
   ```bash
   # Example: tests/integration/tests/15_my_new_feature.sh
   #!/usr/bin/env bash
   # Test: My new feature description

   echo "Test: My new feature"

   # Test 1: Description
   create_template "mytest.tmpltool" 'Template content here'
   OUTPUT=$(run_binary "mytest.tmpltool")
   assert_equals "expected output" "$OUTPUT" "Test passes when..."
   ```

2. Make it executable:
   ```bash
   chmod +x tests/integration/tests/15_my_new_feature.sh
   ```

3. Test locally:
   ```bash
   bash tests/integration/test_binary.sh
   ```

4. CI will automatically discover and run the new test

## Helper Functions (from common.sh)

### Assertion Functions

- `assert_equals expected actual description` - Check exact match
- `assert_contains haystack needle description` - Check substring
- `assert_matches text pattern description` - Check regex pattern match
- `assert_exit_code expected actual description` - Check exit code
- `assert_file_exists file description` - Check file exists
- `assert_in_range value min max description` - Check value in range
- `pass description` - Mark test as passed
- `fail description details` - Mark test as failed

### Template Helper Functions

- `create_template filename content` - Create a template file in TEST_DIR
- `run_binary template [args...]` - Run binary with template from TEST_DIR
- `run_binary_in_dir dir template [args...]` - Run binary from specific directory
- `run_binary_stdin input [args...]` - Run binary with stdin input
- `run_binary_exit_code template [args...]` - Run binary and return exit code

### Environment Variables

All test files have access to:
- `$BINARY` - Path to the binary being tested
- `$TEST_DIR` - Temporary directory for test files
- `$TESTS_RUN` - Number of tests executed
- `$TESTS_PASSED` - Number of tests passed
- `$TESTS_FAILED` - Number of tests failed

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
   ./target/release/tmpltool examples/greeting.tmpltool
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
