#!/usr/bin/env bash
#
# Binary Integration Test Runner
#
# This script runs all integration tests for the tmpltool binary.
# It executes all .sh files in the tests/ directory.
#
# Usage:
#   ./test_binary.sh [path/to/tmpltool]
#
# If no path is provided, it will look for the binary in:
#   - ./target/release/tmpltool
#   - ./target/debug/tmpltool
#

set -euo pipefail

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Source common functions
source "$SCRIPT_DIR/common.sh"

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

# Convert to absolute path
BINARY="$(cd "$(dirname "$BINARY")" && pwd)/$(basename "$BINARY")"

echo "Testing binary: $BINARY"
echo "=================================================================================="

# Create temporary directory for test files
TEST_DIR=$(mktemp -d)
trap "rm -rf $TEST_DIR" EXIT

# Export variables for test scripts
export BINARY
export TEST_DIR
export TESTS_RUN=0
export TESTS_PASSED=0
export TESTS_FAILED=0

# Find and run all test scripts
TEST_FILES=$(find "$SCRIPT_DIR/tests" -name "*.sh" -type f | sort)

if [ -z "$TEST_FILES" ]; then
    echo -e "${YELLOW}Warning: No test files found in $SCRIPT_DIR/tests${NC}"
    exit 1
fi

# Run each test file
for test_file in $TEST_FILES; do
    echo ""
    # Make test file executable
    chmod +x "$test_file" 2>/dev/null || true

    # Run the test file and capture the updated counter values
    # Use source to run in same shell so counters persist
    if source "$test_file"; then
        # Test file executed successfully
        :
    else
        echo -e "${RED}Error: Test file failed: $test_file${NC}"
        TESTS_FAILED=$((TESTS_FAILED + 1))
        TESTS_RUN=$((TESTS_RUN + 1))
    fi
done

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
