#!/usr/bin/env bash
#
# Common helper functions for binary integration tests
#

# Colors for output
export RED='\033[0;31m'
export GREEN='\033[0;32m'
export YELLOW='\033[1;33m'
export BLUE='\033[0;34m'
export NC='\033[0m' # No Color

# Test counters (global across all test files)
export TESTS_RUN=${TESTS_RUN:-0}
export TESTS_PASSED=${TESTS_PASSED:-0}
export TESTS_FAILED=${TESTS_FAILED:-0}

# Binary path (set by runner)
export BINARY="${BINARY:-}"

# Test directory (set by runner)
export TEST_DIR="${TEST_DIR:-}"

# Helper functions
pass() {
    echo -e "${GREEN}✓ PASS${NC}: $1"
    TESTS_PASSED=$((TESTS_PASSED + 1))
    TESTS_RUN=$((TESTS_RUN + 1))
    export TESTS_PASSED TESTS_RUN
}

fail() {
    echo -e "${RED}✗ FAIL${NC}: $1"
    echo -e "  ${YELLOW}Details:${NC} $2"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    TESTS_RUN=$((TESTS_RUN + 1))
    export TESTS_FAILED TESTS_RUN
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

assert_matches() {
    local text="$1"
    local pattern="$2"
    local test_name="$3"

    if echo "$text" | grep -qE "$pattern"; then
        pass "$test_name"
    else
        fail "$test_name" "Output does not match pattern '$pattern'"
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

assert_file_exists() {
    local file="$1"
    local test_name="$2"

    if [ -f "$file" ]; then
        pass "$test_name"
    else
        fail "$test_name" "File does not exist: $file"
    fi
}

assert_in_range() {
    local value="$1"
    local min="$2"
    local max="$3"
    local test_name="$4"

    if [ "$value" -ge "$min" ] && [ "$value" -le "$max" ]; then
        pass "$test_name"
    else
        fail "$test_name" "Value $value not in range [$min, $max]"
    fi
}

# Create template file helper
create_template() {
    local filename="$1"
    local content="$2"
    echo "$content" > "$TEST_DIR/$filename"
}

# Run binary and capture output
run_binary() {
    local template="$1"
    shift
    "$BINARY" "$TEST_DIR/$template" "$@" 2>&1
}

# Run binary from specific directory
run_binary_in_dir() {
    local dir="$1"
    local template="$2"
    shift 2
    (cd "$dir" && "$BINARY" "$template" "$@" 2>&1)
}

# Run binary with stdin
run_binary_stdin() {
    local input="$1"
    shift
    echo "$input" | "$BINARY" "$@" 2>&1
}

# Run binary and get exit code (disables set -e temporarily)
run_binary_exit_code() {
    local template="$1"
    shift
    set +e
    "$BINARY" "$TEST_DIR/$template" "$@" >/dev/null 2>&1
    local exit_code=$?
    set -e
    echo "$exit_code"
}

# Verify binary is set
check_binary() {
    if [ -z "$BINARY" ]; then
        echo -e "${RED}Error: BINARY environment variable not set${NC}"
        return 1
    fi

    if [ ! -f "$BINARY" ]; then
        echo -e "${RED}Error: Binary not found at: $BINARY${NC}"
        return 1
    fi

    return 0
}

# Verify test directory is set
check_test_dir() {
    if [ -z "$TEST_DIR" ]; then
        echo -e "${RED}Error: TEST_DIR environment variable not set${NC}"
        return 1
    fi

    if [ ! -d "$TEST_DIR" ]; then
        echo -e "${RED}Error: Test directory not found: $TEST_DIR${NC}"
        return 1
    fi

    return 0
}
