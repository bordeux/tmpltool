#!/usr/bin/env bash
# Test: Hash functions


echo "Test: Hash functions"

# Test: MD5 hash is correct
create_template "hash.tmpl" '{{ md5(string="test") }}'
OUTPUT=$(run_binary "hash.tmpl")
assert_equals "098f6bcd4621d373cade4e832627b4f6" "$OUTPUT" "MD5 hash is correct"
