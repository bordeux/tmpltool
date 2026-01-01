#!/usr/bin/env bash
# Test: Statistical functions (array_sum, array_avg, array_median, array_min, array_max)

echo "Test: Statistical functions"

# ============================================================================
# Array Sum Tests
# ============================================================================

# Test 1: array_sum - integers
create_template "array_sum_integers.tmpl" '{% set nums = [1, 2, 3, 4, 5] %}{{ array_sum(array=nums) }}'
OUTPUT=$(run_binary "array_sum_integers.tmpl")
assert_equals "15" "$OUTPUT" "array_sum calculates sum of integers"

# Test 2: array_sum - floats
create_template "array_sum_floats.tmpl" '{% set nums = [1.5, 2.5, 3.0] %}{{ array_sum(array=nums) }}'
OUTPUT=$(run_binary "array_sum_floats.tmpl")
assert_equals "7" "$OUTPUT" "array_sum calculates sum of floats"

# Test 3: array_sum - empty array
create_template "array_sum_empty.tmpl" '{% set nums = [] %}{{ array_sum(array=nums) }}'
OUTPUT=$(run_binary "array_sum_empty.tmpl")
assert_equals "0" "$OUTPUT" "array_sum returns 0 for empty array"

# Test 4: array_sum - negative numbers
create_template "array_sum_negative.tmpl" '{% set nums = [-5, -10, 15] %}{{ array_sum(array=nums) }}'
OUTPUT=$(run_binary "array_sum_negative.tmpl")
assert_equals "0" "$OUTPUT" "array_sum handles negative numbers"

# ============================================================================
# Array Average Tests
# ============================================================================

# Test 5: array_avg - basic
create_template "array_avg_basic.tmpl" '{% set scores = [10, 20, 30, 40] %}{{ array_avg(array=scores) }}'
OUTPUT=$(run_binary "array_avg_basic.tmpl")
assert_equals "25" "$OUTPUT" "array_avg calculates average"

# Test 6: array_avg - empty array
create_template "array_avg_empty.tmpl" '{% set nums = [] %}{{ array_avg(array=nums) }}'
OUTPUT=$(run_binary "array_avg_empty.tmpl")
assert_equals "0" "$OUTPUT" "array_avg returns 0 for empty array"

# Test 7: array_avg - single element
create_template "array_avg_single.tmpl" '{% set nums = [42] %}{{ array_avg(array=nums) }}'
OUTPUT=$(run_binary "array_avg_single.tmpl")
assert_equals "42" "$OUTPUT" "array_avg works with single element"

# ============================================================================
# Array Median Tests
# ============================================================================

# Test 8: array_median - odd length
create_template "array_median_odd.tmpl" '{% set nums = [1, 3, 5, 7, 9] %}{{ array_median(array=nums) }}'
OUTPUT=$(run_binary "array_median_odd.tmpl")
assert_equals "5" "$OUTPUT" "array_median finds middle value for odd-length array"

# Test 9: array_median - even length
create_template "array_median_even.tmpl" '{% set nums = [1, 2, 3, 4] %}{{ array_median(array=nums) }}'
OUTPUT=$(run_binary "array_median_even.tmpl")
assert_equals "2.5" "$OUTPUT" "array_median averages middle values for even-length array"

# Test 10: array_median - unsorted
create_template "array_median_unsorted.tmpl" '{% set nums = [9, 1, 5, 3, 7] %}{{ array_median(array=nums) }}'
OUTPUT=$(run_binary "array_median_unsorted.tmpl")
assert_equals "5" "$OUTPUT" "array_median handles unsorted arrays"

# ============================================================================
# Array Min Tests
# ============================================================================

# Test 11: array_min - basic
create_template "array_min_basic.tmpl" '{% set nums = [42, 17, 99, 8, 55] %}{{ array_min(array=nums) }}'
OUTPUT=$(run_binary "array_min_basic.tmpl")
assert_equals "8" "$OUTPUT" "array_min finds minimum value"

# Test 12: array_min - negative numbers
create_template "array_min_negative.tmpl" '{% set nums = [-5, -10, 15, 3] %}{{ array_min(array=nums) }}'
OUTPUT=$(run_binary "array_min_negative.tmpl")
assert_equals "-10" "$OUTPUT" "array_min handles negative numbers"

# Test 13: array_min - single element
create_template "array_min_single.tmpl" '{% set nums = [42] %}{{ array_min(array=nums) }}'
OUTPUT=$(run_binary "array_min_single.tmpl")
assert_equals "42" "$OUTPUT" "array_min works with single element"

# ============================================================================
# Array Max Tests
# ============================================================================

# Test 14: array_max - basic
create_template "array_max_basic.tmpl" '{% set nums = [42, 17, 99, 8, 55] %}{{ array_max(array=nums) }}'
OUTPUT=$(run_binary "array_max_basic.tmpl")
assert_equals "99" "$OUTPUT" "array_max finds maximum value"

# Test 15: array_max - negative numbers
create_template "array_max_negative.tmpl" '{% set nums = [-5, -10, -15, -3] %}{{ array_max(array=nums) }}'
OUTPUT=$(run_binary "array_max_negative.tmpl")
assert_equals "-3" "$OUTPUT" "array_max handles negative numbers"

# Test 16: array_max - single element
create_template "array_max_single.tmpl" '{% set nums = [42] %}{{ array_max(array=nums) }}'
OUTPUT=$(run_binary "array_max_single.tmpl")
assert_equals "42" "$OUTPUT" "array_max works with single element"

# ============================================================================
# Combined Use Cases
# ============================================================================

# Test 17: Statistics in conditionals
create_template "stats_conditional.tmpl" '{% set scores = [85, 90, 78, 92, 88] %}
{% set avg = array_avg(array=scores) %}
{% if avg >= 85 %}
Excellent
{% else %}
Good
{% endif %}'
OUTPUT=$(run_binary "stats_conditional.tmpl")
assert_contains "$OUTPUT" "Excellent" "Statistics work in conditional statements"

# Test 18: Range check with min/max
create_template "stats_range.tmpl" '{% set values = [10, 50, 30, 70, 20] %}
Min: {{ array_min(array=values) }}, Max: {{ array_max(array=values) }}'
OUTPUT=$(run_binary "stats_range.tmpl")
assert_contains "$OUTPUT" "Min: 10, Max: 70" "Min and max can be used together"

# Test 19: Summary statistics
create_template "stats_summary.tmpl" '{% set data = [10, 20, 30, 40, 50] %}
Sum: {{ array_sum(array=data) }}
Avg: {{ array_avg(array=data) }}
Med: {{ array_median(array=data) }}'
OUTPUT=$(run_binary "stats_summary.tmpl")
assert_contains "$OUTPUT" "Sum: 150" "Summary statistics work together"
assert_contains "$OUTPUT" "Avg: 30" "Summary statistics work together"
assert_contains "$OUTPUT" "Med: 30" "Summary statistics work together"
