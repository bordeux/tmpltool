#!/usr/bin/env bash
# Test: Error handling


echo "Test: Error handling"

# Test 1: Invalid template syntax
create_template "invalid.tmpltool" '{{ unclosed'
EXIT_CODE=$(run_binary_exit_code "invalid.tmpltool")
if [ "$EXIT_CODE" -ne 0 ]; then
    pass "Invalid template returns non-zero exit code"
else
    fail "Invalid template returns non-zero exit code" "Exit code was 0"
fi

# Test 2: Missing template file
set +e
"$BINARY" "$TEST_DIR/nonexistent.tmpltool" >/dev/null 2>&1
EXIT_CODE=$?
set -e
if [ "$EXIT_CODE" -ne 0 ]; then
    pass "Missing file returns non-zero exit code"
else
    fail "Missing file returns non-zero exit code" "Exit code was 0"
fi
