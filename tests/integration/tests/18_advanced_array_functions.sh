#!/usr/bin/env bash
# Test: Advanced array functions (array_sort_by, array_group_by, array_unique, array_flatten)

echo "Test: Advanced array functions"

# ============================================================================
# Array Sort By Tests
# ============================================================================

# Test 1: array_sort_by - numeric sorting
create_template "array_sort_by_numeric.tmpl" '{% set users = [
  {"name": "Alice", "age": 30},
  {"name": "Bob", "age": 25},
  {"name": "Charlie", "age": 35}
] %}
{% for user in array_sort_by(array=users, key="age") %}
{{ user.name }}: {{ user.age }}
{% endfor %}'
OUTPUT=$(run_binary "array_sort_by_numeric.tmpl")
assert_contains "$OUTPUT" "Bob: 25" "array_sort_by sorts by numeric key"
assert_contains "$OUTPUT" "Alice: 30" "array_sort_by sorts by numeric key"
assert_contains "$OUTPUT" "Charlie: 35" "array_sort_by sorts by numeric key"

# Test 2: array_sort_by - string sorting
create_template "array_sort_by_string.tmpl" '{%- set items = [
  {"name": "Zebra"},
  {"name": "Apple"},
  {"name": "Mango"}
] -%}
{% for item in array_sort_by(array=items, key="name") -%}
{{ item.name }}
{% endfor -%}'
OUTPUT=$(run_binary "array_sort_by_string.tmpl")
# Check order by extracting lines
FIRST=$(echo "$OUTPUT" | sed -n '1p' | xargs)
SECOND=$(echo "$OUTPUT" | sed -n '2p' | xargs)
THIRD=$(echo "$OUTPUT" | sed -n '3p' | xargs)
assert_equals "Apple" "$FIRST" "array_sort_by sorts strings alphabetically"
assert_equals "Mango" "$SECOND" "array_sort_by sorts strings alphabetically"
assert_equals "Zebra" "$THIRD" "array_sort_by sorts strings alphabetically"

# ============================================================================
# Array Group By Tests
# ============================================================================

# Test 3: array_group_by - basic grouping
create_template "array_group_by_basic.tmpl" '{%- set users = [
  {"name": "Alice", "dept": "Engineering"},
  {"name": "Bob", "dept": "Sales"},
  {"name": "Charlie", "dept": "Engineering"}
] -%}
{%- set grouped = array_group_by(array=users, key="dept") -%}
{% for dept, members in grouped | items -%}
{{ dept }}: {{ members | length }}
{% endfor -%}'
OUTPUT=$(run_binary "array_group_by_basic.tmpl")
assert_contains "$OUTPUT" "Engineering: 2" "array_group_by groups by key"
assert_contains "$OUTPUT" "Sales: 1" "array_group_by groups by key"

# Test 4: array_group_by - numeric grouping
create_template "array_group_by_numeric.tmpl" '{% set items = [
  {"name": "Item1", "priority": 1},
  {"name": "Item2", "priority": 2},
  {"name": "Item3", "priority": 1}
] %}
{% set grouped = array_group_by(array=items, key="priority") %}
Priority 1: {{ grouped["1"] | length }} items
Priority 2: {{ grouped["2"] | length }} items'
OUTPUT=$(run_binary "array_group_by_numeric.tmpl")
assert_contains "$OUTPUT" "Priority 1: 2 items" "array_group_by handles numeric keys"
assert_contains "$OUTPUT" "Priority 2: 1 items" "array_group_by handles numeric keys"

# Test 5: array_group_by - iteration over groups
create_template "array_group_by_iterate.tmpl" '{% set tasks = [
  {"name": "Task1", "status": "done"},
  {"name": "Task2", "status": "pending"},
  {"name": "Task3", "status": "done"}
] %}
{% set by_status = array_group_by(array=tasks, key="status") %}
{% for task in by_status.done %}
{{ task.name }}
{% endfor %}'
OUTPUT=$(run_binary "array_group_by_iterate.tmpl")
assert_contains "$OUTPUT" "Task1" "array_group_by allows iteration over groups"
assert_contains "$OUTPUT" "Task3" "array_group_by allows iteration over groups"

# ============================================================================
# Array Unique Tests
# ============================================================================

# Test 6: array_unique - numbers
create_template "array_unique_numbers.tmpl" '{%- set nums = [1, 2, 2, 3, 1, 4, 3, 5] -%}
{{- array_unique(array=nums) | length -}}'
OUTPUT=$(run_binary "array_unique_numbers.tmpl")
assert_equals "5" "$OUTPUT" "array_unique removes duplicate numbers"

# Test 7: array_unique - strings
create_template "array_unique_strings.tmpl" '{% set tags = ["docker", "kubernetes", "docker", "helm"] %}
{% for tag in array_unique(array=tags) %}
{{ tag }}
{% endfor %}'
OUTPUT=$(run_binary "array_unique_strings.tmpl")
assert_contains "$OUTPUT" "docker" "array_unique removes duplicate strings"
assert_contains "$OUTPUT" "kubernetes" "array_unique removes duplicate strings"
assert_contains "$OUTPUT" "helm" "array_unique removes duplicate strings"
# Count occurrences - docker should appear only once
DOCKER_COUNT=$(echo "$OUTPUT" | grep -c "docker" || true)
assert_equals "1" "$DOCKER_COUNT" "array_unique removes duplicates"

# Test 8: array_unique - all unique
create_template "array_unique_all_unique.tmpl" '{%- set nums = [1, 2, 3, 4, 5] -%}
{{- array_unique(array=nums) | length -}}'
OUTPUT=$(run_binary "array_unique_all_unique.tmpl")
assert_equals "5" "$OUTPUT" "array_unique preserves already unique array"

# Test 9: array_unique - all duplicates
create_template "array_unique_all_dup.tmpl" '{%- set nums = [5, 5, 5, 5] -%}
{{- array_unique(array=nums) | length -}}'
OUTPUT=$(run_binary "array_unique_all_dup.tmpl")
assert_equals "1" "$OUTPUT" "array_unique handles all duplicates"

# ============================================================================
# Array Flatten Tests
# ============================================================================

# Test 10: array_flatten - basic
create_template "array_flatten_basic.tmpl" '{%- set nested = [[1, 2], [3, 4], [5]] -%}
{{- array_flatten(array=nested) | length -}}'
OUTPUT=$(run_binary "array_flatten_basic.tmpl")
assert_equals "5" "$OUTPUT" "array_flatten flattens nested arrays"

# Test 11: array_flatten - strings
create_template "array_flatten_strings.tmpl" '{% set nested = [["a", "b"], ["c"], ["d", "e"]] %}
{% for item in array_flatten(array=nested) %}
{{ item }}
{% endfor %}'
OUTPUT=$(run_binary "array_flatten_strings.tmpl")
assert_contains "$OUTPUT" "a" "array_flatten handles string arrays"
assert_contains "$OUTPUT" "b" "array_flatten handles string arrays"
assert_contains "$OUTPUT" "c" "array_flatten handles string arrays"
assert_contains "$OUTPUT" "d" "array_flatten handles string arrays"
assert_contains "$OUTPUT" "e" "array_flatten handles string arrays"

# Test 12: array_flatten - mixed with non-arrays
create_template "array_flatten_mixed.tmpl" '{%- set mixed = [[1, 2], 3, [4, 5]] -%}
{{- array_flatten(array=mixed) | length -}}'
OUTPUT=$(run_binary "array_flatten_mixed.tmpl")
assert_equals "5" "$OUTPUT" "array_flatten handles mixed arrays and scalars"

# Test 13: array_flatten - empty nested
create_template "array_flatten_empty_nested.tmpl" '{%- set nested = [[], [1, 2], []] -%}
{{- array_flatten(array=nested) | length -}}'
OUTPUT=$(run_binary "array_flatten_empty_nested.tmpl")
assert_equals "2" "$OUTPUT" "array_flatten handles empty nested arrays"

# ============================================================================
# Combined Use Cases
# ============================================================================

# Test 14: Sort + Unique
create_template "sort_and_unique.tmpl" '{% set nums = [3, 1, 2, 1, 3, 2] %}
{% set unique_nums = array_unique(array=nums) %}
Unique count: {{ unique_nums | length }}'
OUTPUT=$(run_binary "sort_and_unique.tmpl")
assert_contains "$OUTPUT" "Unique count: 3" "Unique and sort work together"

# Test 15: Group + Count
create_template "group_and_count.tmpl" '{% set events = [
  {"type": "error", "msg": "E1"},
  {"type": "warning", "msg": "W1"},
  {"type": "error", "msg": "E2"}
] %}
{% set by_type = array_group_by(array=events, key="type") %}
Errors: {{ by_type.error | length }}
Warnings: {{ by_type.warning | length }}'
OUTPUT=$(run_binary "group_and_count.tmpl")
assert_contains "$OUTPUT" "Errors: 2" "Group by enables counting"
assert_contains "$OUTPUT" "Warnings: 1" "Group by enables counting"

# Test 16: Flatten + Unique
create_template "flatten_and_unique.tmpl" '{% set nested = [[1, 2], [2, 3], [3, 4]] %}
{% set flat = array_flatten(array=nested) %}
{% set unique = array_unique(array=flat) %}
Total unique: {{ unique | length }}'
OUTPUT=$(run_binary "flatten_and_unique.tmpl")
assert_contains "$OUTPUT" "Total unique: 4" "Flatten and unique combine well"

# Test 17: Real-world - Group tasks by status and count
create_template "realworld_tasks.tmpl" '{%- set tasks = [
  {"name": "T1", "status": "done", "priority": 1},
  {"name": "T2", "status": "pending", "priority": 2},
  {"name": "T3", "status": "done", "priority": 1},
  {"name": "T4", "status": "in_progress", "priority": 3}
] -%}
{%- set by_status = array_group_by(array=tasks, key="status") %}
Status Report:
{% for status, items in by_status | items -%}
  {{ status }}: {{ items | length }} tasks
{% endfor -%}'
OUTPUT=$(run_binary "realworld_tasks.tmpl")
assert_contains "$OUTPUT" "done: 2 tasks" "Real-world grouping works"
assert_contains "$OUTPUT" "pending: 1 tasks" "Real-world grouping works"
assert_contains "$OUTPUT" "in_progress: 1 tasks" "Real-world grouping works"
