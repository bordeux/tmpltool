#!/usr/bin/env bash
# Test: Simple template rendering


echo "Test: Simple template rendering"

# Test: Simple template renders correctly
create_template "simple.tmpl" "Hello World!"
OUTPUT=$(run_binary "simple.tmpl")
assert_equals "Hello World!" "$OUTPUT" "Simple template renders correctly"
