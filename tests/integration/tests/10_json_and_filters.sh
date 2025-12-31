#!/usr/bin/env bash
# Test: JSON parsing and filters


echo "Test: JSON parsing and filters"

# Test 1: JSON parsing works
create_template "json.tmpl" "{% set data = parse_json(string='{\"name\": \"test\", \"value\": 42}') %}{{ data.name }}"
OUTPUT=$(run_binary "json.tmpl")
assert_equals "test" "$OUTPUT" "JSON parsing works"

# Test 2: Filters work correctly
create_template "filter.tmpl" '{{ "Hello World" | upper }}'
OUTPUT=$(run_binary "filter.tmpl")
assert_equals "HELLO WORLD" "$OUTPUT" "Filters work correctly"
