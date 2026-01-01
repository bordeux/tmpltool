#!/usr/bin/env bash
# Test: Binary execution and version


echo "Test: Binary execution"

# Test 1: Binary executes without error
if "$BINARY" --version >/dev/null 2>&1; then
    pass "Binary executes without error"
else
    fail "Binary executes without error" "Version command failed"
fi

# Test 2: Version output format
VERSION_OUTPUT=$("$BINARY" --version 2>&1)
assert_contains "$VERSION_OUTPUT" "tmpltool" "Version contains program name"

# Test 3: Help output
HELP_OUTPUT=$("$BINARY" --help 2>&1)
assert_contains "$HELP_OUTPUT" "Usage" "Help contains usage information"
assert_contains "$HELP_OUTPUT" "Options" "Help contains options"
