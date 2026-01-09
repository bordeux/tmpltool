#!/usr/bin/env bash
# Test: Output to file and stdin input


echo "Test: Output to file and stdin input"

# Test 1: Output to file
create_template "output.tmpltool" "File content"
"$BINARY" "$TEST_DIR/output.tmpltool" -o "$TEST_DIR/output.txt" 2>&1
if [ -f "$TEST_DIR/output.txt" ]; then
    OUTPUT=$(cat "$TEST_DIR/output.txt")
    assert_equals "File content" "$OUTPUT" "Output file is created with correct content"
else
    fail "Output file is created with correct content" "File was not created"
fi

# Test 2: Stdin input
OUTPUT=$(run_binary_stdin '{{ md5(string="hello") }}')
assert_equals "5d41402abc4b2a76b9719d911017c592" "$OUTPUT" "Stdin input works"
