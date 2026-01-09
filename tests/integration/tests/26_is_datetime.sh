#!/usr/bin/env bash
# Test: Is-functions datetime (is leap_year)


echo "Test: Is-functions datetime"

# ========== is leap_year tests with "is" syntax ==========

# Test 1: Leap year with "is" syntax
create_template "is_leap_2024.tmpltool" '{% if 2024 is leap_year %}leap{% else %}regular{% endif %}'
OUTPUT=$(run_binary "is_leap_2024.tmpltool")
assert_equals "leap" "$OUTPUT" "2024 is a leap year"

# Test 2: Non-leap year with "is" syntax
create_template "is_leap_2023.tmpltool" '{% if 2023 is leap_year %}leap{% else %}regular{% endif %}'
OUTPUT=$(run_binary "is_leap_2023.tmpltool")
assert_equals "regular" "$OUTPUT" "2023 is not a leap year"

# Test 3: Century year (not leap - divisible by 100 but not 400)
create_template "is_leap_1900.tmpltool" '{% if 1900 is leap_year %}leap{% else %}regular{% endif %}'
OUTPUT=$(run_binary "is_leap_1900.tmpltool")
assert_equals "regular" "$OUTPUT" "1900 is not a leap year (century rule)"

# Test 4: Century year (leap - divisible by 400)
create_template "is_leap_2000.tmpltool" '{% if 2000 is leap_year %}leap{% else %}regular{% endif %}'
OUTPUT=$(run_binary "is_leap_2000.tmpltool")
assert_equals "leap" "$OUTPUT" "2000 is a leap year (divisible by 400)"

# Test 5: Year 2100 (not leap - divisible by 100 but not 400)
create_template "is_leap_2100.tmpltool" '{% if 2100 is leap_year %}leap{% else %}regular{% endif %}'
OUTPUT=$(run_binary "is_leap_2100.tmpltool")
assert_equals "regular" "$OUTPUT" "2100 is not a leap year (century rule)"

# ========== is_leap_year function syntax still works ==========

# Test 6: Function syntax with leap year
create_template "is_leap_fn_2024.tmpltool" '{{ is_leap_year(year=2024) }}'
OUTPUT=$(run_binary "is_leap_fn_2024.tmpltool")
assert_equals "true" "$OUTPUT" "is_leap_year function syntax works for leap year"

# Test 7: Function syntax with non-leap year
create_template "is_leap_fn_2023.tmpltool" '{{ is_leap_year(year=2023) }}'
OUTPUT=$(run_binary "is_leap_fn_2023.tmpltool")
assert_equals "false" "$OUTPUT" "is_leap_year function syntax works for non-leap year"

# Test 8: Function syntax with century year
create_template "is_leap_fn_1900.tmpltool" '{{ is_leap_year(year=1900) }}'
OUTPUT=$(run_binary "is_leap_fn_1900.tmpltool")
assert_equals "false" "$OUTPUT" "is_leap_year function syntax works for century year"

# ========== is leap_year with variables ==========

# Test 9: Variable with is syntax
create_template "is_leap_var.tmpltool" '{% set yr = 2020 %}{% if yr is leap_year %}yes{% else %}no{% endif %}'
OUTPUT=$(run_binary "is_leap_var.tmpltool")
assert_equals "yes" "$OUTPUT" "Variable with is leap_year syntax works"

# Test 10: Loop with is leap_year
create_template "is_leap_loop.tmpltool" '{% for y in [2019, 2020, 2021, 2024] %}{% if y is leap_year %}{{ y }} {% endif %}{% endfor %}'
OUTPUT=$(run_binary "is_leap_loop.tmpltool")
assert_equals "2020 2024 " "$OUTPUT" "Loop with is leap_year works"

# ========== negation tests ==========

# Test 11: "is not" negation syntax
create_template "is_not_leap.tmpltool" '{% if 2023 is not leap_year %}regular{% else %}leap{% endif %}'
OUTPUT=$(run_binary "is_not_leap.tmpltool")
assert_equals "regular" "$OUTPUT" "'is not leap_year' negation works"

# Test 12: "is not" with leap year
create_template "is_not_leap_2024.tmpltool" '{% if 2024 is not leap_year %}regular{% else %}leap{% endif %}'
OUTPUT=$(run_binary "is_not_leap_2024.tmpltool")
assert_equals "leap" "$OUTPUT" "'is not leap_year' returns false for leap years"

# ========== string year tests ==========

# Test 13: String year with is syntax
create_template "is_leap_string.tmpltool" '{% if "2024" is leap_year %}yes{% else %}no{% endif %}'
OUTPUT=$(run_binary "is_leap_string.tmpltool")
assert_equals "yes" "$OUTPUT" "String year works with is leap_year"

# Test 14: Invalid string returns false
create_template "is_leap_invalid.tmpltool" '{% if "not-a-year" is leap_year %}yes{% else %}no{% endif %}'
OUTPUT=$(run_binary "is_leap_invalid.tmpltool")
assert_equals "no" "$OUTPUT" "Invalid string returns false for is leap_year"
