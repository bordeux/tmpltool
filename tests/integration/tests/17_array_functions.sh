#!/usr/bin/env bash
# Test: Array manipulation functions (array_count, array_chunk, array_zip)

echo "Test: Array manipulation functions"

# ============================================================================
# Array Count Tests
# ============================================================================

# Test 1: array_count - basic
create_template "array_count_basic.tmpl" '{% set items = ["apple", "banana", "cherry"] %}{{ array_count(array=items) }}'
OUTPUT=$(run_binary "array_count_basic.tmpl")
assert_equals "3" "$OUTPUT" "array_count returns correct count"

# Test 2: array_count - empty array
create_template "array_count_empty.tmpl" '{% set items = [] %}{{ array_count(array=items) }}'
OUTPUT=$(run_binary "array_count_empty.tmpl")
assert_equals "0" "$OUTPUT" "array_count returns 0 for empty array"

# Test 3: array_count - single element
create_template "array_count_single.tmpl" '{% set items = [42] %}{{ array_count(array=items) }}'
OUTPUT=$(run_binary "array_count_single.tmpl")
assert_equals "1" "$OUTPUT" "array_count works with single element"

# ============================================================================
# Array Chunk Tests
# ============================================================================

# Test 4: array_chunk - even division
create_template "array_chunk_even.tmpl" '{% set nums = [1, 2, 3, 4, 5, 6] %}
{% for chunk in array_chunk(array=nums, size=2) %}
{{ chunk }}
{% endfor %}'
OUTPUT=$(run_binary "array_chunk_even.tmpl")
assert_contains "$OUTPUT" "[1, 2]" "array_chunk splits evenly"
assert_contains "$OUTPUT" "[3, 4]" "array_chunk splits evenly"
assert_contains "$OUTPUT" "[5, 6]" "array_chunk splits evenly"

# Test 5: array_chunk - uneven division
create_template "array_chunk_uneven.tmpl" '{% set nums = [1, 2, 3, 4, 5] %}
{% for chunk in array_chunk(array=nums, size=2) %}
{{ chunk }}
{% endfor %}'
OUTPUT=$(run_binary "array_chunk_uneven.tmpl")
assert_contains "$OUTPUT" "[1, 2]" "array_chunk handles remainder"
assert_contains "$OUTPUT" "[3, 4]" "array_chunk handles remainder"
assert_contains "$OUTPUT" "[5]" "array_chunk handles remainder"

# Test 6: array_chunk - size 1
create_template "array_chunk_size_one.tmpl" '{% set nums = [1, 2, 3] -%}
{{- array_chunk(array=nums, size=1) | length -}}'
OUTPUT=$(run_binary "array_chunk_size_one.tmpl")
assert_equals "3" "$OUTPUT" "array_chunk with size 1 creates individual chunks"

# Test 7: array_chunk - larger than array
create_template "array_chunk_large.tmpl" '{% set nums = [1, 2, 3] -%}
{{- array_chunk(array=nums, size=10) | length -}}'
OUTPUT=$(run_binary "array_chunk_large.tmpl")
assert_equals "1" "$OUTPUT" "array_chunk with large size creates single chunk"

# ============================================================================
# Array Zip Tests
# ============================================================================

# Test 8: array_zip - equal length
create_template "array_zip_equal.tmpl" '{% set keys = ["name", "age", "city"] %}
{% set values = ["Alice", 30, "NYC"] %}
{% for pair in array_zip(array1=keys, array2=values) %}
{{ pair[0] }}: {{ pair[1] }}
{% endfor %}'
OUTPUT=$(run_binary "array_zip_equal.tmpl")
assert_contains "$OUTPUT" "name: Alice" "array_zip combines arrays"
assert_contains "$OUTPUT" "age: 30" "array_zip combines arrays"
assert_contains "$OUTPUT" "city: NYC" "array_zip combines arrays"

# Test 9: array_zip - different lengths
create_template "array_zip_different.tmpl" '{% set a = [1, 2, 3, 4] -%}
{%- set b = ["a", "b"] -%}
{{- array_zip(array1=a, array2=b) | length -}}'
OUTPUT=$(run_binary "array_zip_different.tmpl")
assert_equals "2" "$OUTPUT" "array_zip stops at shorter array length"

# Test 10: array_zip - empty arrays
create_template "array_zip_empty.tmpl" '{% set a = [] -%}
{%- set b = [] -%}
{{- array_zip(array1=a, array2=b) | length -}}'
OUTPUT=$(run_binary "array_zip_empty.tmpl")
assert_equals "0" "$OUTPUT" "array_zip handles empty arrays"

# Test 11: array_zip - first empty
create_template "array_zip_first_empty.tmpl" '{% set a = [] -%}
{%- set b = [1, 2, 3] -%}
{{- array_zip(array1=a, array2=b) | length -}}'
OUTPUT=$(run_binary "array_zip_first_empty.tmpl")
assert_equals "0" "$OUTPUT" "array_zip handles first array empty"

# ============================================================================
# Combined Use Cases
# ============================================================================

# Test 12: Pagination with array_chunk
create_template "pagination.tmpl" '{% set items = ["a", "b", "c", "d", "e", "f"] %}
{% for page in array_chunk(array=items, size=3) %}
Page: {{ page | join(", ") }}
{% endfor %}'
OUTPUT=$(run_binary "pagination.tmpl")
assert_contains "$OUTPUT" "Page: a, b, c" "array_chunk useful for pagination"
assert_contains "$OUTPUT" "Page: d, e, f" "array_chunk useful for pagination"

# Test 13: Key-value mapping with array_zip
create_template "key_value_map.tmpl" '{% set k = ["host", "port", "user"] %}
{% set v = ["localhost", 8080, "admin"] %}
{% for pair in array_zip(array1=k, array2=v) %}
{{ pair[0] }}={{ pair[1] }}
{% endfor %}'
OUTPUT=$(run_binary "key_value_map.tmpl")
assert_contains "$OUTPUT" "host=localhost" "array_zip creates key-value pairs"
assert_contains "$OUTPUT" "port=8080" "array_zip creates key-value pairs"
assert_contains "$OUTPUT" "user=admin" "array_zip creates key-value pairs"

# Test 14: Count with conditional
create_template "count_conditional.tmpl" '{% set items = [1, 2, 3, 4, 5] %}
{% set count = array_count(array=items) %}
{% if count > 3 %}
Many items
{% else %}
Few items
{% endif %}'
OUTPUT=$(run_binary "count_conditional.tmpl")
assert_contains "$OUTPUT" "Many items" "array_count works in conditionals"

# Test 15: Chunked iteration
create_template "chunked_iteration.tmpl" '{% set data = [1, 2, 3, 4] %}
{% for chunk in array_chunk(array=data, size=2) %}
Chunk size: {{ array_count(array=chunk) }}
{% endfor %}'
OUTPUT=$(run_binary "chunked_iteration.tmpl")
assert_contains "$OUTPUT" "Chunk size: 2" "array_count and array_chunk work together"
