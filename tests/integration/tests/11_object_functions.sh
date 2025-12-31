#!/usr/bin/env bash
# Test: Object manipulation functions


echo "Test: Object manipulation functions"

# Test: object_keys() returns correct number of keys
create_template "object.tmpl" '{% set obj = {"a": 1, "b": 2} %}{% set keys = object_keys(object=obj) %}{{ keys | length }}'
OUTPUT=$(run_binary "object.tmpl")
assert_equals "2" "$OUTPUT" "object_keys() returns correct number of keys"
