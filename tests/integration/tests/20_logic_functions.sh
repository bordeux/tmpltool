#!/usr/bin/env bash
# Test: Logic functions (default, coalesce, ternary, in_range)

echo "Test: Logic functions"

# ============================================================================
# Default Tests
# ============================================================================

# Test 1: default - truthy value
create_template "default_truthy.tmpltool" '{{ default(value="Hello", default="N/A") }}'
OUTPUT=$(run_binary "default_truthy.tmpltool")
assert_equals "Hello" "$OUTPUT" "default returns truthy value"

# Test 2: default - empty string
create_template "default_empty_string.tmpltool" '{{ default(value="", default="N/A") }}'
OUTPUT=$(run_binary "default_empty_string.tmpltool")
assert_equals "N/A" "$OUTPUT" "default returns default for empty string"

# Test 3: default - with variables
create_template "default_variables.tmpltool" '{% set config = {"port": 8080} %}
Host: {{ default(value=config.host, default="localhost") }}
Port: {{ default(value=config.port, default=3000) }}'
OUTPUT=$(run_binary "default_variables.tmpltool")
assert_contains "$OUTPUT" "Host: localhost" "default uses default for missing key"
assert_contains "$OUTPUT" "Port: 8080" "default uses actual value when present"

# Test 4: default - false value
create_template "default_false.tmpltool" '{{ default(value=false, default="Default") }}'
OUTPUT=$(run_binary "default_false.tmpltool")
assert_equals "Default" "$OUTPUT" "default treats false as falsy"

# Test 5: default - true value
create_template "default_true.tmpltool" '{{ default(value=true, default="Default") }}'
OUTPUT=$(run_binary "default_true.tmpltool")
assert_equals "true" "$OUTPUT" "default returns true value"

# Test 6: default - number value
create_template "default_number.tmpltool" '{% set count = 42 %}
Count: {{ default(value=count, default=0) }}'
OUTPUT=$(run_binary "default_number.tmpltool")
assert_contains "$OUTPUT" "Count: 42" "default returns number value"

# ============================================================================
# Coalesce Tests
# ============================================================================

# Test 7: coalesce - first non-null
create_template "coalesce_first.tmpltool" '{%- set a = none -%}
{%- set b = "found" -%}
{%- set c = "other" -%}
{{- coalesce(values=[a, b, c]) -}}'
OUTPUT=$(run_binary "coalesce_first.tmpltool")
assert_equals "found" "$OUTPUT" "coalesce returns first non-null value"

# Test 8: coalesce - configuration precedence
create_template "coalesce_config.tmpltool" '{% set env_host = none %}
{% set config_host = "prod.example.com" %}
{% set default_host = "localhost" %}
Host: {{ coalesce(values=[env_host, config_host, default_host]) }}'
OUTPUT=$(run_binary "coalesce_config.tmpltool")
assert_contains "$OUTPUT" "Host: prod.example.com" "coalesce respects precedence"

# Test 9: coalesce - all values present
create_template "coalesce_all_present.tmpltool" '{{ coalesce(values=["first", "second", "third"]) }}'
OUTPUT=$(run_binary "coalesce_all_present.tmpltool")
assert_equals "first" "$OUTPUT" "coalesce returns first when all present"

# Test 10: coalesce - with zero
create_template "coalesce_zero.tmpltool" '{%- set a = none -%}
{%- set b = 0 -%}
{%- set c = 42 -%}
{{- coalesce(values=[a, b, c]) -}}'
OUTPUT=$(run_binary "coalesce_zero.tmpltool")
assert_equals "0" "$OUTPUT" "coalesce treats zero as valid value"

# Test 11: coalesce - with false
create_template "coalesce_false.tmpltool" '{%- set a = none -%}
{%- set b = false -%}
{%- set c = true -%}
{{- coalesce(values=[a, b, c]) -}}'
OUTPUT=$(run_binary "coalesce_false.tmpltool")
assert_equals "false" "$OUTPUT" "coalesce treats false as valid value"

# ============================================================================
# Ternary Tests
# ============================================================================

# Test 12: ternary - true condition
create_template "ternary_true.tmpltool" '{{ ternary(condition=true, true_val="Yes", false_val="No") }}'
OUTPUT=$(run_binary "ternary_true.tmpltool")
assert_equals "Yes" "$OUTPUT" "ternary returns true_val for true"

# Test 13: ternary - false condition
create_template "ternary_false.tmpltool" '{{ ternary(condition=false, true_val="Yes", false_val="No") }}'
OUTPUT=$(run_binary "ternary_false.tmpltool")
assert_equals "No" "$OUTPUT" "ternary returns false_val for false"

# Test 14: ternary - with comparison
create_template "ternary_comparison.tmpltool" '{% set score = 85 %}
Result: {{ ternary(condition=score >= 60, true_val="Pass", false_val="Fail") }}'
OUTPUT=$(run_binary "ternary_comparison.tmpltool")
assert_contains "$OUTPUT" "Result: Pass" "ternary works with comparison"

# Test 15: ternary - status indicator
create_template "ternary_status.tmpltool" '{% set cpu_usage = 75 %}
Status: {{ ternary(
  condition=cpu_usage > 90,
  true_val="Critical",
  false_val="Normal"
) }}'
OUTPUT=$(run_binary "ternary_status.tmpltool")
assert_contains "$OUTPUT" "Status: Normal" "ternary evaluates condition"

# Test 16: ternary - with numbers
create_template "ternary_numbers.tmpltool" '{% set enabled = true %}
Max connections: {{ ternary(condition=enabled, true_val=100, false_val=10) }}'
OUTPUT=$(run_binary "ternary_numbers.tmpltool")
assert_contains "$OUTPUT" "Max connections: 100" "ternary works with numeric values"

# Test 17: ternary - truthy string
create_template "ternary_truthy_string.tmpltool" '{{ ternary(condition="hello", true_val="Yes", false_val="No") }}'
OUTPUT=$(run_binary "ternary_truthy_string.tmpltool")
assert_equals "Yes" "$OUTPUT" "ternary treats non-empty string as truthy"

# Test 18: ternary - empty string
create_template "ternary_empty_string.tmpltool" '{{ ternary(condition="", true_val="Yes", false_val="No") }}'
OUTPUT=$(run_binary "ternary_empty_string.tmpltool")
assert_equals "No" "$OUTPUT" "ternary treats empty string as falsy"

# ============================================================================
# In Range Tests
# ============================================================================

# Test 19: in_range - within range
create_template "in_range_within.tmpltool" '{{ in_range(value=50, min=0, max=100) }}'
OUTPUT=$(run_binary "in_range_within.tmpltool")
assert_equals "true" "$OUTPUT" "in_range returns true for value in range"

# Test 20: in_range - below range
create_template "in_range_below.tmpltool" '{{ in_range(value=-10, min=0, max=100) }}'
OUTPUT=$(run_binary "in_range_below.tmpltool")
assert_equals "false" "$OUTPUT" "in_range returns false for value below range"

# Test 21: in_range - above range
create_template "in_range_above.tmpltool" '{{ in_range(value=150, min=0, max=100) }}'
OUTPUT=$(run_binary "in_range_above.tmpltool")
assert_equals "false" "$OUTPUT" "in_range returns false for value above range"

# Test 22: in_range - at minimum
create_template "in_range_min.tmpltool" '{{ in_range(value=0, min=0, max=100) }}'
OUTPUT=$(run_binary "in_range_min.tmpltool")
assert_equals "true" "$OUTPUT" "in_range includes minimum boundary"

# Test 23: in_range - at maximum
create_template "in_range_max.tmpltool" '{{ in_range(value=100, min=0, max=100) }}'
OUTPUT=$(run_binary "in_range_max.tmpltool")
assert_equals "true" "$OUTPUT" "in_range includes maximum boundary"

# Test 24: in_range - port validation
create_template "in_range_port.tmpltool" '{% set port = 8080 %}
{% if in_range(value=port, min=1024, max=65535) %}
Valid port number
{% else %}
Invalid port number
{% endif %}'
OUTPUT=$(run_binary "in_range_port.tmpltool")
assert_contains "$OUTPUT" "Valid port number" "in_range validates port numbers"

# Test 25: in_range - temperature check
create_template "in_range_temp.tmpltool" '{% set temp = 22 %}
Comfortable: {{ in_range(value=temp, min=18, max=25) }}'
OUTPUT=$(run_binary "in_range_temp.tmpltool")
assert_contains "$OUTPUT" "Comfortable: true" "in_range checks temperature"

# Test 26: in_range - with floats
create_template "in_range_floats.tmpltool" '{% set cpu = 75.5 %}
{% if in_range(value=cpu, min=0, max=80) %}
CPU usage normal
{% else %}
CPU usage high
{% endif %}'
OUTPUT=$(run_binary "in_range_floats.tmpltool")
assert_contains "$OUTPUT" "CPU usage normal" "in_range works with floats"

# Test 27: in_range - negative range
create_template "in_range_negative.tmpltool" '{{ in_range(value=-5, min=-10, max=0) }}'
OUTPUT=$(run_binary "in_range_negative.tmpltool")
assert_equals "true" "$OUTPUT" "in_range handles negative ranges"

# ============================================================================
# Combined Use Cases
# ============================================================================

# Test 28: default with ternary
create_template "default_ternary.tmpltool" '{% set user = {"name": "Alice"} %}
{% set role = default(value=user.role, default="guest") %}
Access: {{ ternary(condition=role == "admin", true_val="Full", false_val="Limited") }}'
OUTPUT=$(run_binary "default_ternary.tmpltool")
assert_contains "$OUTPUT" "Access: Limited" "default and ternary work together"

# Test 29: coalesce with in_range
create_template "coalesce_in_range.tmpltool" '{% set env_port = none %}
{% set config_port = 8080 %}
{% set default_port = 3000 %}
{% set port = coalesce(values=[env_port, config_port, default_port]) %}
Valid: {{ in_range(value=port, min=1024, max=65535) }}'
OUTPUT=$(run_binary "coalesce_in_range.tmpltool")
assert_contains "$OUTPUT" "Valid: true" "coalesce and in_range work together"

# Test 30: nested ternary
create_template "nested_ternary.tmpltool" '{% set temp = 25 %}
Weather: {{ ternary(
  condition=temp > 30,
  true_val="Hot",
  false_val=ternary(condition=temp > 20, true_val="Warm", false_val="Cold")
) }}'
OUTPUT=$(run_binary "nested_ternary.tmpltool")
assert_contains "$OUTPUT" "Weather: Warm" "nested ternary works"

# Test 31: configuration with all functions
create_template "config_all_functions.tmpltool" '{% set config = {"max_connections": 50} %}
{% set env_max = none %}
{% set max_conn = coalesce(values=[env_max, config.max_connections, 10]) %}
{% set timeout = default(value=config.timeout, default=30) %}
Max Connections: {{ max_conn }}
Timeout: {{ timeout }}s
Status: {{ ternary(
  condition=in_range(value=max_conn, min=10, max=100),
  true_val="Valid",
  false_val="Invalid"
) }}'
OUTPUT=$(run_binary "config_all_functions.tmpltool")
assert_contains "$OUTPUT" "Max Connections: 50" "configuration uses coalesce"
assert_contains "$OUTPUT" "Timeout: 30s" "configuration uses default"
assert_contains "$OUTPUT" "Status: Valid" "configuration uses ternary and in_range"

# Test 32: resource limits validation
create_template "resource_limits.tmpltool" '{% set cpu = 75 %}
{% set memory = 85 %}
{% set disk = 95 %}
CPU: {{ ternary(condition=in_range(value=cpu, min=0, max=80), true_val="OK", false_val="HIGH") }}
Memory: {{ ternary(condition=in_range(value=memory, min=0, max=80), true_val="OK", false_val="HIGH") }}
Disk: {{ ternary(condition=in_range(value=disk, min=0, max=80), true_val="OK", false_val="HIGH") }}'
OUTPUT=$(run_binary "resource_limits.tmpltool")
assert_contains "$OUTPUT" "CPU: OK" "resource validation for CPU"
assert_contains "$OUTPUT" "Memory: HIGH" "resource validation for Memory"
assert_contains "$OUTPUT" "Disk: HIGH" "resource validation for Disk"

# Test 33: fallback chain
create_template "fallback_chain.tmpltool" '{% set primary = none %}
{% set secondary = none %}
{% set tertiary = "backup.example.com" %}
{% set fallback = "localhost" %}
Server: {{ default(
  value=coalesce(values=[primary, secondary, tertiary]),
  default=fallback
) }}'
OUTPUT=$(run_binary "fallback_chain.tmpltool")
assert_contains "$OUTPUT" "Server: backup.example.com" "fallback chain works"

# Test 34: environment-based configuration
create_template "env_based_config.tmpltool" '{% set env = "production" %}
{% set debug = ternary(condition=env == "development", true_val=true, false_val=false) %}
{% set max_conn = ternary(condition=env == "production", true_val=100, false_val=10) %}
{% set log_level = ternary(
  condition=env == "production",
  true_val="error",
  false_val="debug"
) %}
Environment: {{ env }}
Debug: {{ debug }}
Max Connections: {{ max_conn }}
Log Level: {{ log_level }}'
OUTPUT=$(run_binary "env_based_config.tmpltool")
assert_contains "$OUTPUT" "Environment: production" "environment set"
assert_contains "$OUTPUT" "Debug: false" "debug disabled in production"
assert_contains "$OUTPUT" "Max Connections: 100" "high connections in production"
assert_contains "$OUTPUT" "Log Level: error" "error logging in production"

# ============================================================================
# Error Cases
# ============================================================================

# Test 35: Error - coalesce with non-array
create_template "error_coalesce_non_array.tmpltool" '{{ coalesce(values="test") }}'
OUTPUT=$(run_binary_expect_error "error_coalesce_non_array.tmpltool")
assert_contains "$OUTPUT" "error" "coalesce rejects non-array"

# Test 36: Error - in_range with non-numeric value
create_template "error_in_range_non_numeric.tmpltool" '{{ in_range(value="test", min=0, max=100) }}'
OUTPUT=$(run_binary_expect_error "error_in_range_non_numeric.tmpltool")
assert_contains "$OUTPUT" "error" "in_range rejects non-numeric value"

# Test 37: Error - in_range with non-numeric min
create_template "error_in_range_non_numeric_min.tmpltool" '{{ in_range(value=50, min="test", max=100) }}'
OUTPUT=$(run_binary_expect_error "error_in_range_non_numeric_min.tmpltool")
assert_contains "$OUTPUT" "error" "in_range rejects non-numeric min"
