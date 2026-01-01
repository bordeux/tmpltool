#!/usr/bin/env bash
# Test: Extended array functions and set operations

echo "Test: Extended array functions and set operations"

# Test 1: array_take
create_template "array_take.tmpl" '{{ array_take(array=[1, 2, 3, 4, 5], n=3) | tojson }}'
OUTPUT=$(run_binary "array_take.tmpl")
assert_equals "[1,2,3]" "$OUTPUT" "array_take takes first N elements"

# Test 2: array_take more than available
create_template "array_take_more.tmpl" '{{ array_take(array=[1, 2], n=5) | tojson }}'
OUTPUT=$(run_binary "array_take_more.tmpl")
assert_equals "[1,2]" "$OUTPUT" "array_take with n > length returns all elements"

# Test 3: array_drop
create_template "array_drop.tmpl" '{{ array_drop(array=[1, 2, 3, 4, 5], n=2) | tojson }}'
OUTPUT=$(run_binary "array_drop.tmpl")
assert_equals "[3,4,5]" "$OUTPUT" "array_drop skips first N elements"

# Test 4: array_drop more than available
create_template "array_drop_more.tmpl" '{{ array_drop(array=[1, 2], n=5) | tojson }}'
OUTPUT=$(run_binary "array_drop_more.tmpl")
assert_equals "[]" "$OUTPUT" "array_drop with n > length returns empty array"

# Test 5: array_index_of found
create_template "array_index_of.tmpl" '{{ array_index_of(array=["a", "b", "c"], value="b") }}'
OUTPUT=$(run_binary "array_index_of.tmpl")
assert_equals "1" "$OUTPUT" "array_index_of returns correct index"

# Test 6: array_index_of not found
create_template "array_index_of_not.tmpl" '{{ array_index_of(array=[1, 2, 3], value=5) }}'
OUTPUT=$(run_binary "array_index_of_not.tmpl")
assert_equals "-1" "$OUTPUT" "array_index_of returns -1 when not found"

# Test 7: array_find
create_template "array_find.tmpl" '{% set users = [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}] %}{{ array_find(array=users, key="id", value=2).name }}'
OUTPUT=$(run_binary "array_find.tmpl")
assert_equals "Bob" "$OUTPUT" "array_find returns matching object"

# Test 8: array_filter_by greater than
create_template "array_filter_gt.tmpl" '{% set items = [{"price": 10}, {"price": 20}, {"price": 30}] %}{{ array_filter_by(array=items, key="price", op="gt", value=15) | length }}'
OUTPUT=$(run_binary "array_filter_gt.tmpl")
assert_equals "2" "$OUTPUT" "array_filter_by with gt operator"

# Test 9: array_filter_by equals
create_template "array_filter_eq.tmpl" '{% set items = [{"status": "active"}, {"status": "inactive"}, {"status": "active"}] %}{{ array_filter_by(array=items, key="status", op="eq", value="active") | length }}'
OUTPUT=$(run_binary "array_filter_eq.tmpl")
assert_equals "2" "$OUTPUT" "array_filter_by with eq operator"

# Test 10: array_filter_by contains
create_template "array_filter_contains.tmpl" '{% set items = [{"name": "Alice"}, {"name": "Bob"}, {"name": "Charlie"}] %}{{ array_filter_by(array=items, key="name", op="contains", value="li") | length }}'
OUTPUT=$(run_binary "array_filter_contains.tmpl")
assert_equals "2" "$OUTPUT" "array_filter_by with contains operator"

# Test 11: array_pluck simple
create_template "array_pluck.tmpl" '{% set users = [{"name": "Alice"}, {"name": "Bob"}] %}{{ array_pluck(array=users, key="name") | tojson }}'
OUTPUT=$(run_binary "array_pluck.tmpl")
assert_equals '["Alice","Bob"]' "$OUTPUT" "array_pluck extracts values"

# Test 12: array_pluck nested
create_template "array_pluck_nested.tmpl" '{% set data = [{"user": {"name": "Alice"}}, {"user": {"name": "Bob"}}] %}{{ array_pluck(array=data, key="user.name") | tojson }}'
OUTPUT=$(run_binary "array_pluck_nested.tmpl")
assert_equals '["Alice","Bob"]' "$OUTPUT" "array_pluck with dot notation"

# Test 13: array_intersection
create_template "array_intersection.tmpl" '{{ array_intersection(array1=[1, 2, 3, 4], array2=[3, 4, 5, 6]) | tojson }}'
OUTPUT=$(run_binary "array_intersection.tmpl")
assert_equals "[3,4]" "$OUTPUT" "array_intersection returns common elements"

# Test 14: array_intersection strings
create_template "array_intersection_str.tmpl" '{{ array_intersection(array1=["a", "b", "c"], array2=["b", "c", "d"]) | tojson }}'
OUTPUT=$(run_binary "array_intersection_str.tmpl")
assert_equals '["b","c"]' "$OUTPUT" "array_intersection works with strings"

# Test 15: array_difference
create_template "array_difference.tmpl" '{{ array_difference(array1=[1, 2, 3, 4], array2=[3, 4, 5, 6]) | tojson }}'
OUTPUT=$(run_binary "array_difference.tmpl")
assert_equals "[1,2]" "$OUTPUT" "array_difference returns elements in first but not second"

# Test 16: array_difference strings
create_template "array_difference_str.tmpl" '{{ array_difference(array1=["a", "b", "c"], array2=["b"]) | tojson }}'
OUTPUT=$(run_binary "array_difference_str.tmpl")
assert_equals '["a","c"]' "$OUTPUT" "array_difference works with strings"

# Test 17: array_union
create_template "array_union.tmpl" '{{ array_union(array1=[1, 2, 3], array2=[3, 4, 5]) | tojson }}'
OUTPUT=$(run_binary "array_union.tmpl")
assert_equals "[1,2,3,4,5]" "$OUTPUT" "array_union returns all unique elements"

# Test 18: array_union strings
create_template "array_union_str.tmpl" '{{ array_union(array1=["a", "b"], array2=["b", "c"]) | tojson }}'
OUTPUT=$(run_binary "array_union_str.tmpl")
assert_equals '["a","b","c"]' "$OUTPUT" "array_union works with strings"

# Test 19: array_symmetric_difference
create_template "array_sym_diff.tmpl" '{{ array_symmetric_difference(array1=[1, 2, 3, 4], array2=[3, 4, 5, 6]) | tojson }}'
OUTPUT=$(run_binary "array_sym_diff.tmpl")
assert_equals "[1,2,5,6]" "$OUTPUT" "array_symmetric_difference returns elements in either but not both"

# Test 20: array_symmetric_difference strings
create_template "array_sym_diff_str.tmpl" '{{ array_symmetric_difference(array1=["a", "b", "c"], array2=["b", "c", "d"]) | tojson }}'
OUTPUT=$(run_binary "array_sym_diff_str.tmpl")
assert_equals '["a","d"]' "$OUTPUT" "array_symmetric_difference works with strings"
