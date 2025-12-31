#!/usr/bin/env bash
# Test: Filesystem functions


echo "Test: Filesystem functions"

# Test: read_file() works
echo "test content" > "$TEST_DIR/test_file.txt"
create_template "file_ops.tmpl" '{{ read_file(path="test_file.txt") }}'
OUTPUT=$(run_binary "file_ops.tmpl")
assert_contains "$OUTPUT" "test content" "read_file() works"
