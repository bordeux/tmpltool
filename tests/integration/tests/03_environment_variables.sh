#!/usr/bin/env bash
# Test: Environment variable substitution


echo "Test: Environment variable substitution"

# Test 1: Environment variable substitution works
create_template "env.tmpltool" '{{ get_env(name="TEST_VAR", default="default_value") }}'
OUTPUT=$(TEST_VAR="test_value" run_binary "env.tmpltool")
assert_equals "test_value" "$OUTPUT" "Environment variable substitution works"

# Test 2: Default value when env var missing
OUTPUT=$(run_binary "env.tmpltool")
assert_equals "default_value" "$OUTPUT" "Default value is used when env var is missing"

# Test 3: Multiple environment variables
create_template "multi_env.tmpltool" '{{ get_env(name="VAR1", default="d1") }}-{{ get_env(name="VAR2", default="d2") }}'
OUTPUT=$(VAR1="value1" VAR2="value2" run_binary "multi_env.tmpltool")
assert_equals "value1-value2" "$OUTPUT" "Multiple env vars work"
