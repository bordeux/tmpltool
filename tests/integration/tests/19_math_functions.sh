#!/usr/bin/env bash
# Test: Math functions (min, max, abs, round, ceil, floor, percentage)

echo "Test: Math functions"

# ============================================================================
# Min Tests
# ============================================================================

# Test 1: min - integers
create_template "min_integers.tmpl" '{{ min(a=10, b=20) }}'
OUTPUT=$(run_binary "min_integers.tmpl")
assert_equals "10" "$OUTPUT" "min returns smaller integer"

# Test 2: min - floats
create_template "min_floats.tmpl" '{{ min(a=3.14, b=2.71) }}'
OUTPUT=$(run_binary "min_floats.tmpl")
assert_equals "2.71" "$OUTPUT" "min returns smaller float"

# Test 3: min - negative numbers
create_template "min_negative.tmpl" '{{ min(a=-10, b=-5) }}'
OUTPUT=$(run_binary "min_negative.tmpl")
assert_equals "-10" "$OUTPUT" "min handles negative numbers"

# Test 4: min - with variables
create_template "min_variables.tmpl" '{% set cpu1 = 45.2 %}
{% set cpu2 = 38.7 %}
Lowest CPU: {{ min(a=cpu1, b=cpu2) }}%'
OUTPUT=$(run_binary "min_variables.tmpl")
assert_contains "$OUTPUT" "Lowest CPU: 38.7%" "min works with variables"

# ============================================================================
# Max Tests
# ============================================================================

# Test 5: max - integers
create_template "max_integers.tmpl" '{{ max(a=10, b=20) }}'
OUTPUT=$(run_binary "max_integers.tmpl")
assert_equals "20" "$OUTPUT" "max returns larger integer"

# Test 6: max - floats
create_template "max_floats.tmpl" '{{ max(a=3.14, b=2.71) }}'
OUTPUT=$(run_binary "max_floats.tmpl")
assert_equals "3.14" "$OUTPUT" "max returns larger float"

# Test 7: max - with variables
create_template "max_variables.tmpl" '{% set memory1 = 2048 %}
{% set memory2 = 4096 %}
Peak memory: {{ max(a=memory1, b=memory2) }}MB'
OUTPUT=$(run_binary "max_variables.tmpl")
assert_contains "$OUTPUT" "Peak memory: 4096MB" "max works with variables"

# ============================================================================
# Abs Tests
# ============================================================================

# Test 8: abs - positive number
create_template "abs_positive.tmpl" '{{ abs(number=42) }}'
OUTPUT=$(run_binary "abs_positive.tmpl")
assert_equals "42" "$OUTPUT" "abs preserves positive numbers"

# Test 9: abs - negative number
create_template "abs_negative.tmpl" '{{ abs(number=-42) }}'
OUTPUT=$(run_binary "abs_negative.tmpl")
assert_equals "42" "$OUTPUT" "abs converts negative to positive"

# Test 10: abs - zero
create_template "abs_zero.tmpl" '{{ abs(number=0) }}'
OUTPUT=$(run_binary "abs_zero.tmpl")
assert_equals "0" "$OUTPUT" "abs handles zero"

# Test 11: abs - with expression
create_template "abs_expression.tmpl" '{% set temp1 = 25 %}
{% set temp2 = 18 %}
Difference: {{ abs(number=temp1 - temp2) }}°C'
OUTPUT=$(run_binary "abs_expression.tmpl")
assert_contains "$OUTPUT" "Difference: 7°C" "abs works with expressions"

# ============================================================================
# Round Tests
# ============================================================================

# Test 12: round - default (to integer)
create_template "round_default.tmpl" '{{ round(number=3.7) }}'
OUTPUT=$(run_binary "round_default.tmpl")
assert_equals "4" "$OUTPUT" "round defaults to nearest integer"

# Test 13: round - down
create_template "round_down.tmpl" '{{ round(number=3.4) }}'
OUTPUT=$(run_binary "round_down.tmpl")
assert_equals "3" "$OUTPUT" "round rounds down when < .5"

# Test 14: round - to 2 decimals
create_template "round_decimals.tmpl" '{{ round(number=3.14159, decimals=2) }}'
OUTPUT=$(run_binary "round_decimals.tmpl")
assert_equals "3.14" "$OUTPUT" "round to 2 decimal places"

# Test 15: round - price calculation
create_template "round_price.tmpl" '{% set price = 19.999 %}
Price: ${{ round(number=price, decimals=2) }}'
OUTPUT=$(run_binary "round_price.tmpl")
assert_contains "$OUTPUT" "Price: \$20" "round works for price calculations"

# Test 16: round - explicit zero decimals
create_template "round_zero_decimals.tmpl" '{{ round(number=19.999, decimals=0) }}'
OUTPUT=$(run_binary "round_zero_decimals.tmpl")
assert_equals "20" "$OUTPUT" "round with decimals=0 rounds to integer"

# ============================================================================
# Ceil Tests
# ============================================================================

# Test 17: ceil - basic
create_template "ceil_basic.tmpl" '{{ ceil(number=3.1) }}'
OUTPUT=$(run_binary "ceil_basic.tmpl")
assert_equals "4" "$OUTPUT" "ceil rounds up"

# Test 18: ceil - exact integer
create_template "ceil_exact.tmpl" '{{ ceil(number=3.0) }}'
OUTPUT=$(run_binary "ceil_exact.tmpl")
assert_equals "3" "$OUTPUT" "ceil preserves exact integers"

# Test 19: ceil - small fraction
create_template "ceil_small.tmpl" '{{ ceil(number=3.001) }}'
OUTPUT=$(run_binary "ceil_small.tmpl")
assert_equals "4" "$OUTPUT" "ceil rounds up even tiny fractions"

# Test 20: ceil - servers calculation
create_template "ceil_servers.tmpl" '{% set users = 150 %}
{% set users_per_server = 50 %}
Servers needed: {{ ceil(number=users / users_per_server) }}'
OUTPUT=$(run_binary "ceil_servers.tmpl")
assert_contains "$OUTPUT" "Servers needed: 3" "ceil calculates required servers"

# ============================================================================
# Floor Tests
# ============================================================================

# Test 21: floor - basic
create_template "floor_basic.tmpl" '{{ floor(number=3.9) }}'
OUTPUT=$(run_binary "floor_basic.tmpl")
assert_equals "3" "$OUTPUT" "floor rounds down"

# Test 22: floor - exact integer
create_template "floor_exact.tmpl" '{{ floor(number=3.0) }}'
OUTPUT=$(run_binary "floor_exact.tmpl")
assert_equals "3" "$OUTPUT" "floor preserves exact integers"

# Test 23: floor - large fraction
create_template "floor_large.tmpl" '{{ floor(number=3.999) }}'
OUTPUT=$(run_binary "floor_large.tmpl")
assert_equals "3" "$OUTPUT" "floor rounds down even large fractions"

# Test 24: floor - pages calculation
create_template "floor_pages.tmpl" '{% set items = 47 %}
{% set items_per_page = 10 %}
Full pages: {{ floor(number=items / items_per_page) }}'
OUTPUT=$(run_binary "floor_pages.tmpl")
assert_contains "$OUTPUT" "Full pages: 4" "floor calculates full pages"

# ============================================================================
# Percentage Tests
# ============================================================================

# Test 25: percentage - basic
create_template "percentage_basic.tmpl" '{{ percentage(value=25, total=100) }}'
OUTPUT=$(run_binary "percentage_basic.tmpl")
assert_equals "25" "$OUTPUT" "percentage calculates basic percentage"

# Test 26: percentage - with rounding
create_template "percentage_round.tmpl" '{% set completed = 7 %}
{% set total_tasks = 10 %}
Progress: {{ round(number=percentage(value=completed, total=total_tasks), decimals=1) }}%'
OUTPUT=$(run_binary "percentage_round.tmpl")
assert_contains "$OUTPUT" "Progress: 70%" "percentage with rounding"

# Test 27: percentage - disk usage
create_template "percentage_disk.tmpl" '{% set used = 450 %}
{% set capacity = 500 %}
Disk usage: {{ round(number=percentage(value=used, total=capacity), decimals=2) }}%'
OUTPUT=$(run_binary "percentage_disk.tmpl")
assert_contains "$OUTPUT" "Disk usage: 90%" "percentage calculates disk usage"

# Test 28: percentage - over 100%
create_template "percentage_over100.tmpl" '{{ percentage(value=150, total=100) }}'
OUTPUT=$(run_binary "percentage_over100.tmpl")
assert_equals "150" "$OUTPUT" "percentage can exceed 100%"

# Test 29: percentage - decimal result
create_template "percentage_decimal.tmpl" '{{ percentage(value=1, total=3) }}'
OUTPUT=$(run_binary "percentage_decimal.tmpl")
# Should be 33.333...
OUTPUT_NUM=$(echo "$OUTPUT" | xargs)
# Check it starts with 33.33
assert_contains "$OUTPUT_NUM" "33.33" "percentage handles decimal results"

# ============================================================================
# Combined Use Cases
# ============================================================================

# Test 30: Min/Max together
create_template "min_max_combined.tmpl" '{% set values = [10, 25, 15, 30, 20] %}
Min: {{ min(a=min(a=min(a=10, b=25), b=15), b=min(a=30, b=20)) }}
Max: {{ max(a=max(a=max(a=10, b=25), b=15), b=max(a=30, b=20)) }}'
OUTPUT=$(run_binary "min_max_combined.tmpl")
assert_contains "$OUTPUT" "Min: 10" "min finds minimum"
assert_contains "$OUTPUT" "Max: 30" "max finds maximum"

# Test 31: Round with abs
create_template "round_abs.tmpl" '{% set diff = -3.14159 %}
Absolute rounded: {{ round(number=abs(number=diff), decimals=2) }}'
OUTPUT=$(run_binary "round_abs.tmpl")
assert_contains "$OUTPUT" "Absolute rounded: 3.14" "round and abs work together"

# Test 32: Percentage with ceil
create_template "percentage_ceil.tmpl" '{% set partial = 7 %}
{% set total = 10 %}
At least {{ ceil(number=percentage(value=partial, total=total)) }}% complete'
OUTPUT=$(run_binary "percentage_ceil.tmpl")
assert_contains "$OUTPUT" "At least 70% complete" "percentage with ceil"

# Test 33: Real-world resource calculation
create_template "resource_calc.tmpl" '{% set current_memory = 7.5 %}
{% set max_memory = 8.0 %}
{% set usage_pct = percentage(value=current_memory, total=max_memory) %}
Memory Usage: {{ round(number=usage_pct, decimals=1) }}%
{% if usage_pct > 90 %}
WARNING: High memory usage!
{% endif %}'
OUTPUT=$(run_binary "resource_calc.tmpl")
assert_contains "$OUTPUT" "Memory Usage: 93.8%" "resource calculation works"

# Test 34: Temperature conversion with rounding
create_template "temperature.tmpl" '{% set celsius = 22.7 %}
{% set fahrenheit = (celsius * 9 / 5) + 32 %}
{{ celsius }}°C = {{ round(number=fahrenheit, decimals=1) }}°F'
OUTPUT=$(run_binary "temperature.tmpl")
assert_contains "$OUTPUT" "22.7°C = 72.9°F" "temperature conversion with rounding"

# Test 35: Server capacity planning
create_template "capacity_planning.tmpl" '{% set current_users = 1234 %}
{% set capacity_per_server = 500 %}
{% set servers_needed = current_users / capacity_per_server %}
Current servers needed: {{ ceil(number=servers_needed) }}
Current utilization: {{ round(number=percentage(value=current_users, total=capacity_per_server * ceil(number=servers_needed)), decimals=1) }}%'
OUTPUT=$(run_binary "capacity_planning.tmpl")
assert_contains "$OUTPUT" "Current servers needed: 3" "capacity planning calculation"
assert_contains "$OUTPUT" "Current utilization: 82.3%" "utilization percentage"

# ============================================================================
# Error Cases
# ============================================================================

# Test 36: Error - min with non-numeric
create_template "error_min_non_numeric.tmpl" '{{ min(a="test", b=10) }}'
OUTPUT=$(run_binary_expect_error "error_min_non_numeric.tmpl")
assert_contains "$OUTPUT" "error" "min rejects non-numeric values"

# Test 37: Error - round with negative decimals
create_template "error_round_negative_decimals.tmpl" '{{ round(number=3.14, decimals=-1) }}'
OUTPUT=$(run_binary_expect_error "error_round_negative_decimals.tmpl")
assert_contains "$OUTPUT" "error" "round rejects negative decimals"

# Test 38: Error - percentage with zero total
create_template "error_percentage_zero.tmpl" '{{ percentage(value=25, total=0) }}'
OUTPUT=$(run_binary_expect_error "error_percentage_zero.tmpl")
assert_contains "$OUTPUT" "error" "percentage rejects zero total"
