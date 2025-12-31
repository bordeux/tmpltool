#!/usr/bin/env bash
# Test: Predicate functions (array_any, array_all, array_contains, starts_with, ends_with)

echo "Test: Predicate functions"

# ============================================================================
# Array Predicate Tests
# ============================================================================

# Test 1: array_any - element found
create_template "array_any_found.tmpl" '{% set nums = [1, 2, 3, 4, 5] %}{{ array_any(array=nums, predicate=3) }}'
OUTPUT=$(run_binary "array_any_found.tmpl")
assert_equals "true" "$OUTPUT" "array_any returns true when element found"

# Test 2: array_any - element not found
create_template "array_any_not_found.tmpl" '{% set nums = [1, 2, 3] %}{{ array_any(array=nums, predicate=99) }}'
OUTPUT=$(run_binary "array_any_not_found.tmpl")
assert_equals "false" "$OUTPUT" "array_any returns false when element not found"

# Test 3: array_any with strings
create_template "array_any_strings.tmpl" '{% set fruits = ["apple", "banana", "cherry"] %}{{ array_any(array=fruits, predicate="banana") }}'
OUTPUT=$(run_binary "array_any_strings.tmpl")
assert_equals "true" "$OUTPUT" "array_any works with strings"

# Test 4: array_all - all match
create_template "array_all_match.tmpl" '{% set nums = [5, 5, 5, 5] %}{{ array_all(array=nums, predicate=5) }}'
OUTPUT=$(run_binary "array_all_match.tmpl")
assert_equals "true" "$OUTPUT" "array_all returns true when all elements match"

# Test 5: array_all - not all match
create_template "array_all_no_match.tmpl" '{% set nums = [5, 5, 3, 5] %}{{ array_all(array=nums, predicate=5) }}'
OUTPUT=$(run_binary "array_all_no_match.tmpl")
assert_equals "false" "$OUTPUT" "array_all returns false when not all elements match"

# Test 6: array_all - empty array
create_template "array_all_empty.tmpl" '{% set empty = [] %}{{ array_all(array=empty, predicate=5) }}'
OUTPUT=$(run_binary "array_all_empty.tmpl")
assert_equals "true" "$OUTPUT" "array_all returns true for empty array (vacuous truth)"

# Test 7: array_contains - found
create_template "array_contains_found.tmpl" '{% set nums = [10, 20, 30, 40] %}{{ array_contains(array=nums, value=30) }}'
OUTPUT=$(run_binary "array_contains_found.tmpl")
assert_equals "true" "$OUTPUT" "array_contains returns true when value found"

# Test 8: array_contains - not found
create_template "array_contains_not_found.tmpl" '{% set nums = [10, 20, 30] %}{{ array_contains(array=nums, value=99) }}'
OUTPUT=$(run_binary "array_contains_not_found.tmpl")
assert_equals "false" "$OUTPUT" "array_contains returns false when value not found"

# ============================================================================
# String Predicate Tests
# ============================================================================

# Test 9: starts_with - true
create_template "starts_with_true.tmpl" '{{ starts_with(string="Hello World", prefix="Hello") }}'
OUTPUT=$(run_binary "starts_with_true.tmpl")
assert_equals "true" "$OUTPUT" "starts_with returns true for matching prefix"

# Test 10: starts_with - false
create_template "starts_with_false.tmpl" '{{ starts_with(string="Hello World", prefix="World") }}'
OUTPUT=$(run_binary "starts_with_false.tmpl")
assert_equals "false" "$OUTPUT" "starts_with returns false for non-matching prefix"

# Test 11: starts_with - file extension check
create_template "starts_with_file.tmpl" '{% set filename = "config.yaml" %}{{ starts_with(string=filename, prefix="config") }}'
OUTPUT=$(run_binary "starts_with_file.tmpl")
assert_equals "true" "$OUTPUT" "starts_with works for filename prefixes"

# Test 12: ends_with - true
create_template "ends_with_true.tmpl" '{{ ends_with(string="readme.txt", suffix=".txt") }}'
OUTPUT=$(run_binary "ends_with_true.tmpl")
assert_equals "true" "$OUTPUT" "ends_with returns true for matching suffix"

# Test 13: ends_with - false
create_template "ends_with_false.tmpl" '{{ ends_with(string="readme.txt", suffix=".md") }}'
OUTPUT=$(run_binary "ends_with_false.tmpl")
assert_equals "false" "$OUTPUT" "ends_with returns false for non-matching suffix"

# Test 14: ends_with - URL check
create_template "ends_with_url.tmpl" '{% set url = "https://example.com" %}{{ ends_with(string=url, suffix=".com") }}'
OUTPUT=$(run_binary "ends_with_url.tmpl")
assert_equals "true" "$OUTPUT" "ends_with works for URL suffixes"

# ============================================================================
# Conditional Use Cases
# ============================================================================

# Test 15: Using predicates in conditionals
create_template "predicate_conditional.tmpl" '{% set files = ["app.py", "config.yaml", "data.json"] %}
{% if array_any(array=files, predicate="config.yaml") %}
Config found
{% else %}
No config
{% endif %}'
OUTPUT=$(run_binary "predicate_conditional.tmpl")
assert_contains "$OUTPUT" "Config found" "Predicates work in conditional statements"

# Test 16: File type filtering with ends_with
create_template "file_type_filter.tmpl" '{% set filename = "document.pdf" %}
{% if ends_with(string=filename, suffix=".pdf") %}
PDF
{% elif ends_with(string=filename, suffix=".txt") %}
TEXT
{% else %}
UNKNOWN
{% endif %}'
OUTPUT=$(run_binary "file_type_filter.tmpl")
assert_contains "$OUTPUT" "PDF" "ends_with useful for file type detection"

# Test 17: Validation with starts_with
create_template "url_validation.tmpl" '{% set url = "https://secure.example.com" %}
{% if starts_with(string=url, prefix="https://") %}
Secure
{% else %}
Insecure
{% endif %}'
OUTPUT=$(run_binary "url_validation.tmpl")
assert_contains "$OUTPUT" "Secure" "starts_with useful for protocol validation"
