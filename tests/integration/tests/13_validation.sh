#!/usr/bin/env bash
# Test: Validation option


echo "Test: Validation option"

# Test 1: Valid JSON passes validation
create_template "valid_json.tmpl" '{"valid": "json"}'
set +e
"$BINARY" "$TEST_DIR/valid_json.tmpl" --validate json >/dev/null 2>&1
EXIT_CODE=$?
set -e
assert_exit_code 0 "$EXIT_CODE" "Valid JSON passes validation"

# Test 2: Invalid JSON fails validation
create_template "invalid_json.tmpl" '{invalid json}'
set +e
"$BINARY" "$TEST_DIR/invalid_json.tmpl" --validate json >/dev/null 2>&1
EXIT_CODE=$?
set -e
if [ "$EXIT_CODE" -ne 0 ]; then
    pass "Invalid JSON fails validation"
else
    fail "Invalid JSON fails validation" "Exit code was 0"
fi
