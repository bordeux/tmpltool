#!/usr/bin/env bash
# Test: Simple template rendering


echo "Test: Simple template rendering"

# Test: Simple template renders correctly
create_template "simple.tmpltool" "Hello World!"
OUTPUT=$(run_binary "simple.tmpltool")
assert_equals "Hello World!" "$OUTPUT" "Simple template renders correctly"
