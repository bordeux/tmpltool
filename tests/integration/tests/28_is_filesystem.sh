#!/usr/bin/env bash
# Test: Is-functions filesystem (is file, is dir, is symlink)


echo "Test: Is-functions filesystem"

# ========== Setup test files in TEST_DIR ==========
# Create test files and directories for filesystem checks
echo "test content" > "$TEST_DIR/test_file.txt"
mkdir -p "$TEST_DIR/test_subdir"

# ========== is file tests ==========

# Test 1: is file with "is" syntax - existing file
create_template "is_file_exists.tmpl" '{% if "test_file.txt" is file %}yes{% else %}no{% endif %}'
OUTPUT=$(run_binary "is_file_exists.tmpl")
assert_equals "yes" "$OUTPUT" "Existing file passes 'is file' test"

# Test 2: is file with "is" syntax - non-existing file
create_template "is_file_missing.tmpl" '{% if "nonexistent_file_12345.txt" is file %}yes{% else %}no{% endif %}'
OUTPUT=$(run_binary "is_file_missing.tmpl")
assert_equals "no" "$OUTPUT" "Non-existing file fails 'is file' test"

# Test 3: is file with "is" syntax - directory (should be false)
create_template "is_file_dir.tmpl" '{% if "test_subdir" is file %}yes{% else %}no{% endif %}'
OUTPUT=$(run_binary "is_file_dir.tmpl")
assert_equals "no" "$OUTPUT" "Directory fails 'is file' test"

# Test 4: is_file function syntax still works
create_template "is_file_fn.tmpl" '{{ is_file(path="test_file.txt") }}'
OUTPUT=$(run_binary "is_file_fn.tmpl")
assert_equals "true" "$OUTPUT" "is_file function syntax works"

# Test 5: is file with variable
create_template "is_file_var.tmpl" '{% set f = "test_file.txt" %}{% if f is file %}yes{% else %}no{% endif %}'
OUTPUT=$(run_binary "is_file_var.tmpl")
assert_equals "yes" "$OUTPUT" "Variable with 'is file' works"

# ========== is dir tests ==========

# Test 6: is dir with "is" syntax - existing directory
create_template "is_dir_exists.tmpl" '{% if "test_subdir" is dir %}yes{% else %}no{% endif %}'
OUTPUT=$(run_binary "is_dir_exists.tmpl")
assert_equals "yes" "$OUTPUT" "Existing directory passes 'is dir' test"

# Test 7: is dir with "is" syntax - non-existing directory
create_template "is_dir_missing.tmpl" '{% if "nonexistent_dir_12345" is dir %}yes{% else %}no{% endif %}'
OUTPUT=$(run_binary "is_dir_missing.tmpl")
assert_equals "no" "$OUTPUT" "Non-existing directory fails 'is dir' test"

# Test 8: is dir with "is" syntax - file (should be false)
create_template "is_dir_file.tmpl" '{% if "test_file.txt" is dir %}yes{% else %}no{% endif %}'
OUTPUT=$(run_binary "is_dir_file.tmpl")
assert_equals "no" "$OUTPUT" "File fails 'is dir' test"

# Test 9: is_dir function syntax still works
create_template "is_dir_fn.tmpl" '{{ is_dir(path="test_subdir") }}'
OUTPUT=$(run_binary "is_dir_fn.tmpl")
assert_equals "true" "$OUTPUT" "is_dir function syntax works"

# Test 10: is dir with variable
create_template "is_dir_var.tmpl" '{% set d = "test_subdir" %}{% if d is dir %}yes{% else %}no{% endif %}'
OUTPUT=$(run_binary "is_dir_var.tmpl")
assert_equals "yes" "$OUTPUT" "Variable with 'is dir' works"

# ========== is symlink tests ==========

# Test 11: is_symlink function syntax - regular file (not a symlink)
create_template "is_symlink_fn.tmpl" '{{ is_symlink(path="test_file.txt") }}'
OUTPUT=$(run_binary "is_symlink_fn.tmpl")
assert_equals "false" "$OUTPUT" "Regular file is not a symlink"

# Test 12: is symlink with "is" syntax - regular file
create_template "is_symlink_file.tmpl" '{% if "test_file.txt" is symlink %}yes{% else %}no{% endif %}'
OUTPUT=$(run_binary "is_symlink_file.tmpl")
assert_equals "no" "$OUTPUT" "Regular file fails 'is symlink' test"

# Test 13: is symlink - non-existing path
create_template "is_symlink_missing.tmpl" '{% if "nonexistent_12345" is symlink %}yes{% else %}no{% endif %}'
OUTPUT=$(run_binary "is_symlink_missing.tmpl")
assert_equals "no" "$OUTPUT" "Non-existing path fails 'is symlink' test"

# ========== negation tests ==========

# Test 14: is not file
create_template "is_not_file.tmpl" '{% if "nonexistent.txt" is not file %}missing{% else %}found{% endif %}'
OUTPUT=$(run_binary "is_not_file.tmpl")
assert_equals "missing" "$OUTPUT" "'is not file' negation works"

# Test 15: is not dir
create_template "is_not_dir.tmpl" '{% if "test_file.txt" is not dir %}not a dir{% else %}is a dir{% endif %}'
OUTPUT=$(run_binary "is_not_dir.tmpl")
assert_equals "not a dir" "$OUTPUT" "'is not dir' negation works"

# ========== combined tests ==========

# Test 16: Multiple checks in one template
create_template "is_fs_multi.tmpl" '{% if "test_file.txt" is file %}F{% endif %}{% if "test_subdir" is dir %}D{% endif %}'
OUTPUT=$(run_binary "is_fs_multi.tmpl")
assert_equals "FD" "$OUTPUT" "Multiple filesystem checks work"

# Test 17: Non-string value returns false
create_template "is_file_nonstring.tmpl" '{% if 123 is file %}yes{% else %}no{% endif %}'
OUTPUT=$(run_binary "is_file_nonstring.tmpl")
assert_equals "no" "$OUTPUT" "Non-string value returns false for is file"

# Test 18: Nested path check (create nested file for this test)
mkdir -p "$TEST_DIR/test_subdir/nested"
echo "nested content" > "$TEST_DIR/test_subdir/nested/file.txt"
create_template "is_file_nested.tmpl" '{% if "test_subdir/nested/file.txt" is file %}yes{% else %}no{% endif %}'
OUTPUT=$(run_binary "is_file_nested.tmpl")
assert_equals "yes" "$OUTPUT" "Nested path 'is file' works"
