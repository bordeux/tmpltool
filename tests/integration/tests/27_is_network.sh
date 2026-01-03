#!/usr/bin/env bash
# Test: Is-functions network (is port_available)
#
# Note: Port availability tests depend on system state.
# We test the syntax works correctly; actual availability may vary.


echo "Test: Is-functions network"

# ========== is port_available function syntax ==========

# Test 1: Function syntax works (high port unlikely to be in use)
create_template "is_port_fn.tmpl" '{{ is_port_available(port=59123) }}'
OUTPUT=$(run_binary "is_port_fn.tmpl")
# Result could be true or false depending on system state
if [ "$OUTPUT" = "true" ] || [ "$OUTPUT" = "false" ]; then
    echo "  [PASS] is_port_available function syntax returns boolean"
else
    echo "  [FAIL] is_port_available function syntax - expected true/false, got: $OUTPUT"
    exit 1
fi

# Test 2: Function syntax in conditional
create_template "is_port_fn_cond.tmpl" '{% if is_port_available(port=59124) %}free{% else %}busy{% endif %}'
OUTPUT=$(run_binary "is_port_fn_cond.tmpl")
if [ "$OUTPUT" = "free" ] || [ "$OUTPUT" = "busy" ]; then
    echo "  [PASS] is_port_available function syntax in conditional works"
else
    echo "  [FAIL] is_port_available function conditional - expected free/busy, got: $OUTPUT"
    exit 1
fi

# ========== is port_available "is" syntax ==========

# Test 3: Is-test syntax with literal port
create_template "is_port_is.tmpl" '{% if 59125 is port_available %}free{% else %}busy{% endif %}'
OUTPUT=$(run_binary "is_port_is.tmpl")
if [ "$OUTPUT" = "free" ] || [ "$OUTPUT" = "busy" ]; then
    echo "  [PASS] 'is port_available' syntax works with literal"
else
    echo "  [FAIL] 'is port_available' syntax - expected free/busy, got: $OUTPUT"
    exit 1
fi

# Test 4: Is-test syntax with variable
create_template "is_port_var.tmpl" '{% set p = 59126 %}{% if p is port_available %}free{% else %}busy{% endif %}'
OUTPUT=$(run_binary "is_port_var.tmpl")
if [ "$OUTPUT" = "free" ] || [ "$OUTPUT" = "busy" ]; then
    echo "  [PASS] 'is port_available' syntax works with variable"
else
    echo "  [FAIL] 'is port_available' with variable - expected free/busy, got: $OUTPUT"
    exit 1
fi

# Test 5: Is-test syntax with string port
create_template "is_port_str.tmpl" '{% if "59127" is port_available %}free{% else %}busy{% endif %}'
OUTPUT=$(run_binary "is_port_str.tmpl")
if [ "$OUTPUT" = "free" ] || [ "$OUTPUT" = "busy" ]; then
    echo "  [PASS] 'is port_available' syntax works with string"
else
    echo "  [FAIL] 'is port_available' with string - expected free/busy, got: $OUTPUT"
    exit 1
fi

# ========== is not port_available negation ==========

# Test 6: Negation syntax
create_template "is_not_port.tmpl" '{% if 59128 is not port_available %}busy{% else %}free{% endif %}'
OUTPUT=$(run_binary "is_not_port.tmpl")
if [ "$OUTPUT" = "free" ] || [ "$OUTPUT" = "busy" ]; then
    echo "  [PASS] 'is not port_available' negation syntax works"
else
    echo "  [FAIL] 'is not port_available' negation - expected free/busy, got: $OUTPUT"
    exit 1
fi

# ========== invalid port tests ==========

# Test 7: Port 0 is invalid (returns false)
create_template "is_port_zero.tmpl" '{% if 0 is port_available %}yes{% else %}no{% endif %}'
OUTPUT=$(run_binary "is_port_zero.tmpl")
assert_equals "no" "$OUTPUT" "Port 0 returns false"

# Test 8: Negative port is invalid (returns false)
create_template "is_port_neg.tmpl" '{% if -1 is port_available %}yes{% else %}no{% endif %}'
OUTPUT=$(run_binary "is_port_neg.tmpl")
assert_equals "no" "$OUTPUT" "Negative port returns false"

# Test 9: Port > 65535 is invalid (returns false)
create_template "is_port_high.tmpl" '{% if 65536 is port_available %}yes{% else %}no{% endif %}'
OUTPUT=$(run_binary "is_port_high.tmpl")
assert_equals "no" "$OUTPUT" "Port > 65535 returns false"

# Test 10: Non-numeric string returns false
create_template "is_port_str_invalid.tmpl" '{% if "not-a-port" is port_available %}yes{% else %}no{% endif %}'
OUTPUT=$(run_binary "is_port_str_invalid.tmpl")
assert_equals "no" "$OUTPUT" "Non-numeric string returns false"

# Test 11: Empty string returns false
create_template "is_port_empty.tmpl" '{% if "" is port_available %}yes{% else %}no{% endif %}'
OUTPUT=$(run_binary "is_port_empty.tmpl")
assert_equals "no" "$OUTPUT" "Empty string returns false"

# ========== boundary tests ==========

# Test 12: Port 1 is valid (though may need root)
create_template "is_port_1.tmpl" '{% if 1 is port_available %}free{% else %}busy{% endif %}'
OUTPUT=$(run_binary "is_port_1.tmpl")
if [ "$OUTPUT" = "free" ] || [ "$OUTPUT" = "busy" ]; then
    echo "  [PASS] Port 1 boundary test works"
else
    echo "  [FAIL] Port 1 boundary - expected free/busy, got: $OUTPUT"
    exit 1
fi

# Test 13: Port 65535 is valid maximum
create_template "is_port_max.tmpl" '{% if 65535 is port_available %}free{% else %}busy{% endif %}'
OUTPUT=$(run_binary "is_port_max.tmpl")
if [ "$OUTPUT" = "free" ] || [ "$OUTPUT" = "busy" ]; then
    echo "  [PASS] Port 65535 maximum boundary test works"
else
    echo "  [FAIL] Port 65535 boundary - expected free/busy, got: $OUTPUT"
    exit 1
fi
