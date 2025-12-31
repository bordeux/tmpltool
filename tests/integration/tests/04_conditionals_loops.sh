#!/usr/bin/env bash
# Test: Conditional logic and loops


echo "Test: Conditional logic and loops"

# Test 1: Conditional evaluates to true
create_template "conditional.tmpl" '{% if get_env(name="ENABLE_FEATURE") == "true" %}enabled{% else %}disabled{% endif %}'
OUTPUT=$(ENABLE_FEATURE="true" run_binary "conditional.tmpl")
assert_equals "enabled" "$OUTPUT" "Conditional evaluates to true"

# Test 2: Conditional evaluates to false
OUTPUT=$(ENABLE_FEATURE="false" run_binary "conditional.tmpl")
assert_equals "disabled" "$OUTPUT" "Conditional evaluates to false"

# Test 3: Loop iteration
create_template "loop.tmpl" '{% for i in [1, 2, 3] %}{{ i }}{% endfor %}'
OUTPUT=$(run_binary "loop.tmpl")
assert_equals "123" "$OUTPUT" "Loop iterates correctly"
