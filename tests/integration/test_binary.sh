#!/usr/bin/env bash
#
# Binary Integration Tests
#
# This script tests the compiled binary itself (not the code).
# It validates that the binary works correctly with real-world scenarios.
#
# Usage:
#   ./test_binary.sh [path/to/tmpltool]
#
# If no path is provided, it will look for the binary in:
#   - ./target/release/tmpltool
#   - ./target/debug/tmpltool
#

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test counters
TESTS_RUN=0
TESTS_PASSED=0
TESTS_FAILED=0

# Find the binary
BINARY="${1:-}"
if [ -z "$BINARY" ]; then
    if [ -f "./target/release/tmpltool" ]; then
        BINARY="./target/release/tmpltool"
    elif [ -f "./target/debug/tmpltool" ]; then
        BINARY="./target/debug/tmpltool"
    elif [ -f "./target/release/tmpltool.exe" ]; then
        BINARY="./target/release/tmpltool.exe"
    elif [ -f "./target/debug/tmpltool.exe" ]; then
        BINARY="./target/debug/tmpltool.exe"
    else
        echo -e "${RED}Error: Could not find tmpltool binary${NC}"
        echo "Please build the binary first with: cargo build --release"
        exit 1
    fi
fi

# Verify binary exists and is executable
if [ ! -f "$BINARY" ]; then
    echo -e "${RED}Error: Binary not found at: $BINARY${NC}"
    exit 1
fi

# Make executable if not already (for Unix-like systems)
if [[ "$OSTYPE" != "msys" && "$OSTYPE" != "win32" ]]; then
    chmod +x "$BINARY" 2>/dev/null || true
fi

echo "Testing binary: $BINARY"
echo "=================================================================================="

# Create temporary directory for test files
TEST_DIR=$(mktemp -d)
trap "rm -rf $TEST_DIR" EXIT

# Helper functions
pass() {
    echo -e "${GREEN}✓ PASS${NC}: $1"
    TESTS_PASSED=$((TESTS_PASSED + 1))
    TESTS_RUN=$((TESTS_RUN + 1))
}

fail() {
    echo -e "${RED}✗ FAIL${NC}: $1"
    echo -e "  ${YELLOW}Details:${NC} $2"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    TESTS_RUN=$((TESTS_RUN + 1))
}

assert_equals() {
    local expected="$1"
    local actual="$2"
    local test_name="$3"

    if [ "$expected" = "$actual" ]; then
        pass "$test_name"
    else
        fail "$test_name" "Expected '$expected', got '$actual'"
    fi
}

assert_contains() {
    local haystack="$1"
    local needle="$2"
    local test_name="$3"

    if echo "$haystack" | grep -q "$needle"; then
        pass "$test_name"
    else
        fail "$test_name" "Output does not contain '$needle'"
    fi
}

assert_exit_code() {
    local expected="$1"
    local actual="$2"
    local test_name="$3"

    if [ "$expected" -eq "$actual" ]; then
        pass "$test_name"
    else
        fail "$test_name" "Expected exit code $expected, got $actual"
    fi
}

# Test 1: Binary exists and runs
echo "Test 1: Binary execution"
if "$BINARY" --version >/dev/null 2>&1; then
    pass "Binary executes without error"
else
    fail "Binary executes without error" "Version command failed"
fi

# Test 2: Version output format
echo ""
echo "Test 2: Version information"
VERSION_OUTPUT=$("$BINARY" --version 2>&1 || true)
assert_contains "$VERSION_OUTPUT" "tmpltool" "Version contains program name"

# Test 3: Help output
echo ""
echo "Test 3: Help information"
HELP_OUTPUT=$("$BINARY" --help 2>&1 || true)
assert_contains "$HELP_OUTPUT" "Usage" "Help contains usage information"
assert_contains "$HELP_OUTPUT" "Options" "Help contains options"

# Test 4: Simple template rendering
echo ""
echo "Test 4: Simple template rendering"
cat > "$TEST_DIR/simple.tmpl" << 'EOF'
Hello World!
EOF
OUTPUT=$("$BINARY" "$TEST_DIR/simple.tmpl" 2>&1)
assert_equals "Hello World!" "$OUTPUT" "Simple template renders correctly"

# Test 5: Environment variable substitution
echo ""
echo "Test 5: Environment variable substitution"
cat > "$TEST_DIR/env.tmpl" << 'EOF'
{{ get_env(name="TEST_VAR", default="default_value") }}
EOF
OUTPUT=$(TEST_VAR="test_value" "$BINARY" "$TEST_DIR/env.tmpl" 2>&1)
assert_equals "test_value" "$OUTPUT" "Environment variable substitution works"

# Test 6: Default value when env var missing
echo ""
echo "Test 6: Default value for missing env var"
OUTPUT=$("$BINARY" "$TEST_DIR/env.tmpl" 2>&1)
assert_equals "default_value" "$OUTPUT" "Default value is used when env var is missing"

# Test 7: Template with conditional
echo ""
echo "Test 7: Conditional logic"
cat > "$TEST_DIR/conditional.tmpl" << 'EOF'
{% if get_env(name="ENABLE_FEATURE") == "true" %}enabled{% else %}disabled{% endif %}
EOF
OUTPUT=$(ENABLE_FEATURE="true" "$BINARY" "$TEST_DIR/conditional.tmpl" 2>&1)
assert_equals "enabled" "$OUTPUT" "Conditional evaluates to true"

OUTPUT=$(ENABLE_FEATURE="false" "$BINARY" "$TEST_DIR/conditional.tmpl" 2>&1)
assert_equals "disabled" "$OUTPUT" "Conditional evaluates to false"

# Test 8: Template with loop
echo ""
echo "Test 8: Loop iteration"
cat > "$TEST_DIR/loop.tmpl" << 'EOF'
{% for i in [1, 2, 3] %}{{ i }}{% endfor %}
EOF
OUTPUT=$("$BINARY" "$TEST_DIR/loop.tmpl" 2>&1)
assert_equals "123" "$OUTPUT" "Loop iterates correctly"

# Test 9: Hash functions
echo ""
echo "Test 9: Hash functions"
cat > "$TEST_DIR/hash.tmpl" << 'EOF'
{{ md5(string="test") }}
EOF
OUTPUT=$("$BINARY" "$TEST_DIR/hash.tmpl" 2>&1)
assert_equals "098f6bcd4621d373cade4e832627b4f6" "$OUTPUT" "MD5 hash is correct"

# Test 10: Output to file
echo ""
echo "Test 10: Output to file"
cat > "$TEST_DIR/output.tmpl" << 'EOF'
File content
EOF
"$BINARY" "$TEST_DIR/output.tmpl" -o "$TEST_DIR/output.txt" 2>&1
if [ -f "$TEST_DIR/output.txt" ]; then
    OUTPUT=$(cat "$TEST_DIR/output.txt")
    assert_equals "File content" "$OUTPUT" "Output file is created with correct content"
else
    fail "Output file is created with correct content" "File was not created"
fi

# Test 11: Stdin input
echo ""
echo "Test 11: Stdin input"
OUTPUT=$(echo "{{ md5(string=\"hello\") }}" | "$BINARY" 2>&1)
assert_equals "5d41402abc4b2a76b9719d911017c592" "$OUTPUT" "Stdin input works"

# Test 12: UUID generation (format check)
echo ""
echo "Test 12: UUID generation"
cat > "$TEST_DIR/uuid.tmpl" << 'EOF'
{{ uuid() }}
EOF
OUTPUT=$("$BINARY" "$TEST_DIR/uuid.tmpl" 2>&1)
# UUID format: xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx
if echo "$OUTPUT" | grep -qE '^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$'; then
    pass "UUID has correct format"
else
    fail "UUID has correct format" "UUID does not match expected format: $OUTPUT"
fi

# Test 13: Invalid template syntax
echo ""
echo "Test 13: Invalid template syntax handling"
cat > "$TEST_DIR/invalid.tmpl" << 'EOF'
{{ unclosed
EOF
set +e
"$BINARY" "$TEST_DIR/invalid.tmpl" >/dev/null 2>&1
EXIT_CODE=$?
set -e
if [ $EXIT_CODE -ne 0 ]; then
    pass "Invalid template returns non-zero exit code"
else
    fail "Invalid template returns non-zero exit code" "Exit code was 0"
fi

# Test 14: Missing template file
echo ""
echo "Test 14: Missing template file handling"
set +e
"$BINARY" "$TEST_DIR/nonexistent.tmpl" >/dev/null 2>&1
EXIT_CODE=$?
set -e
if [ $EXIT_CODE -ne 0 ]; then
    pass "Missing file returns non-zero exit code"
else
    fail "Missing file returns non-zero exit code" "Exit code was 0"
fi

# Test 15: File operations (filesystem functions)
echo ""
echo "Test 15: Filesystem functions"
echo "test content" > "$TEST_DIR/test_file.txt"
cat > "$TEST_DIR/file_ops.tmpl" << 'EOF'
{{ read_file(path="test_file.txt") }}
EOF
OUTPUT=$("$BINARY" "$TEST_DIR/file_ops.tmpl" 2>&1)
assert_contains "$OUTPUT" "test content" "read_file() works"

# Test 16: JSON parsing
echo ""
echo "Test 16: JSON functions"
cat > "$TEST_DIR/data.json" << 'EOF'
{"name": "test", "value": 42}
EOF
cat > "$TEST_DIR/json.tmpl" << EOF
{% set data = parse_json(string='{"name": "test", "value": 42}') %}{{ data.name }}
EOF
OUTPUT=$("$BINARY" "$TEST_DIR/json.tmpl" 2>&1)
assert_equals "test" "$OUTPUT" "JSON parsing works"

# Test 17: Filter usage
echo ""
echo "Test 17: Filters"
cat > "$TEST_DIR/filter.tmpl" << 'EOF'
{{ "Hello World" | upper }}
EOF
OUTPUT=$("$BINARY" "$TEST_DIR/filter.tmpl" 2>&1)
assert_equals "HELLO WORLD" "$OUTPUT" "Filters work correctly"

# Test 18: Multiple environment variables
echo ""
echo "Test 18: Multiple environment variables"
cat > "$TEST_DIR/multi_env.tmpl" << 'EOF'
{{ get_env(name="VAR1", default="d1") }}-{{ get_env(name="VAR2", default="d2") }}
EOF
OUTPUT=$(VAR1="value1" VAR2="value2" "$BINARY" "$TEST_DIR/multi_env.tmpl" 2>&1)
assert_equals "value1-value2" "$OUTPUT" "Multiple env vars work"

# Test 19: Now function (timestamp validation)
echo ""
echo "Test 19: Timestamp function"
cat > "$TEST_DIR/timestamp.tmpl" << 'EOF'
{{ now() }}
EOF
OUTPUT=$("$BINARY" "$TEST_DIR/timestamp.tmpl" 2>&1)
# Should be an ISO8601 timestamp (e.g., 2025-12-31T16:07:37.422352+00:00)
if echo "$OUTPUT" | grep -qE '^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}'; then
    pass "now() returns valid ISO8601 timestamp"
else
    fail "now() returns valid ISO8601 timestamp" "Output is not a valid timestamp: $OUTPUT"
fi

# Test 20: Random number generation
echo ""
echo "Test 20: Random number generation"
cat > "$TEST_DIR/random.tmpl" << 'EOF'
{{ get_random(start=1, end=100) }}
EOF
OUTPUT=$("$BINARY" "$TEST_DIR/random.tmpl" 2>&1)
if echo "$OUTPUT" | grep -qE '^[0-9]+$' && [ "$OUTPUT" -ge 1 ] && [ "$OUTPUT" -le 100 ]; then
    pass "get_random() returns number in range"
else
    fail "get_random() returns number in range" "Output out of range or invalid: $OUTPUT"
fi

# Test 21: Object manipulation functions
echo ""
echo "Test 21: Object manipulation"
cat > "$TEST_DIR/object.tmpl" << 'EOF'
{% set obj = {"a": 1, "b": 2} %}{% set keys = object_keys(object=obj) %}{{ keys | length }}
EOF
OUTPUT=$("$BINARY" "$TEST_DIR/object.tmpl" 2>&1)
assert_equals "2" "$OUTPUT" "object_keys() returns correct number of keys"

# Test 22: Serialization functions
echo ""
echo "Test 22: JSON serialization"
cat > "$TEST_DIR/serialize.tmpl" << 'EOF'
{% set obj = {"test": "value"} %}{{ to_json(object=obj) }}
EOF
OUTPUT=$("$BINARY" "$TEST_DIR/serialize.tmpl" 2>&1)
assert_equals '{"test":"value"}' "$OUTPUT" "to_json() serializes correctly"

# Test 23: Validation option - valid JSON
echo ""
echo "Test 23: Validation option (valid JSON)"
cat > "$TEST_DIR/valid_json.tmpl" << 'EOF'
{"valid": "json"}
EOF
set +e
"$BINARY" "$TEST_DIR/valid_json.tmpl" --validate json >/dev/null 2>&1
EXIT_CODE=$?
set -e
assert_exit_code 0 $EXIT_CODE "Valid JSON passes validation"

# Test 24: Validation option - invalid JSON
echo ""
echo "Test 24: Validation option (invalid JSON)"
cat > "$TEST_DIR/invalid_json.tmpl" << 'EOF'
{invalid json}
EOF
set +e
"$BINARY" "$TEST_DIR/invalid_json.tmpl" --validate json >/dev/null 2>&1
EXIT_CODE=$?
set -e
if [ $EXIT_CODE -ne 0 ]; then
    pass "Invalid JSON fails validation"
else
    fail "Invalid JSON fails validation" "Exit code was 0"
fi

# Test 25: Complex real-world example
echo ""
echo "Test 25: Complex configuration template"
cat > "$TEST_DIR/complex.tmpl" << 'EOF'
# Server Configuration
server:
  host: {{ get_env(name="SERVER_HOST", default="0.0.0.0") }}
  port: {{ get_env(name="SERVER_PORT", default="8080") }}
  {% if get_env(name="ENABLE_SSL", default="false") == "true" %}
  ssl:
    enabled: true
    cert: {{ get_env(name="SSL_CERT_PATH") }}
  {% endif %}
EOF
OUTPUT=$(SERVER_HOST="localhost" SERVER_PORT="3000" "$BINARY" "$TEST_DIR/complex.tmpl" 2>&1)
assert_contains "$OUTPUT" "host: localhost" "Complex template renders host correctly"
assert_contains "$OUTPUT" "port: 3000" "Complex template renders port correctly"

# Summary
echo ""
echo "=================================================================================="
echo -e "Test Summary:"
echo -e "  Total:  $TESTS_RUN"
echo -e "  ${GREEN}Passed: $TESTS_PASSED${NC}"
if [ $TESTS_FAILED -gt 0 ]; then
    echo -e "  ${RED}Failed: $TESTS_FAILED${NC}"
else
    echo -e "  Failed: $TESTS_FAILED"
fi
echo "=================================================================================="

if [ $TESTS_FAILED -gt 0 ]; then
    exit 1
else
    echo -e "${GREEN}All tests passed!${NC}"
    exit 0
fi
