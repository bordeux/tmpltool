#!/usr/bin/env bash
# Test: Environment file loading (--env flag)

echo "Test: Environment file loading (--env flag)"

# Test 1: Load single env file
cat > "$TEST_DIR/.env" << 'EOF'
TEST_VAR=hello_from_env
ANOTHER_VAR=world
EOF
create_template "env_file.tmpl" '{{ get_env(name="TEST_VAR") }} {{ get_env(name="ANOTHER_VAR") }}'
OUTPUT=$("$BINARY" --env "$TEST_DIR/.env" "$TEST_DIR/env_file.tmpl" 2>&1)
assert_equals "hello_from_env world" "$OUTPUT" "Single env file loading works"

# Test 2: Multiple env files with override
cat > "$TEST_DIR/base.env" << 'EOF'
VAR1=base_value
VAR2=base_value
EOF
cat > "$TEST_DIR/override.env" << 'EOF'
VAR2=override_value
VAR3=new_value
EOF
create_template "multi_env.tmpl" '{{ get_env(name="VAR1") }}-{{ get_env(name="VAR2") }}-{{ get_env(name="VAR3") }}'
OUTPUT=$("$BINARY" --env "$TEST_DIR/base.env" --env "$TEST_DIR/override.env" "$TEST_DIR/multi_env.tmpl" 2>&1)
assert_equals "base_value-override_value-new_value" "$OUTPUT" "Multiple env files with override"

# Test 3: Comments in env file
cat > "$TEST_DIR/comments.env" << 'EOF'
# This is a comment
VAR=value
# Another comment
EOF
create_template "comments.tmpl" '{{ get_env(name="VAR") }}'
OUTPUT=$("$BINARY" --env "$TEST_DIR/comments.env" "$TEST_DIR/comments.tmpl" 2>&1)
assert_equals "value" "$OUTPUT" "Comments in env file are ignored"

# Test 4: Quoted values in env file
cat > "$TEST_DIR/quoted.env" << 'EOF'
DOUBLE="hello world"
SINGLE='single quoted'
EOF
create_template "quoted.tmpl" '{{ get_env(name="DOUBLE") }}|{{ get_env(name="SINGLE") }}'
OUTPUT=$("$BINARY" --env "$TEST_DIR/quoted.env" "$TEST_DIR/quoted.tmpl" 2>&1)
assert_equals "hello world|single quoted" "$OUTPUT" "Quoted values in env file"

# Test 5: Empty values in env file
cat > "$TEST_DIR/empty.env" << 'EOF'
EMPTY=
EOF
create_template "empty.tmpl" '[{{ get_env(name="EMPTY") }}]'
OUTPUT=$("$BINARY" --env "$TEST_DIR/empty.env" "$TEST_DIR/empty.tmpl" 2>&1)
assert_equals "[]" "$OUTPUT" "Empty values in env file"

# Test 6: Special characters in env file
cat > "$TEST_DIR/special.env" << 'EOF'
URL=https://example.com/path?key=value&other=123
EOF
create_template "special.tmpl" '{{ get_env(name="URL") }}'
OUTPUT=$("$BINARY" --env "$TEST_DIR/special.env" "$TEST_DIR/special.tmpl" 2>&1)
assert_equals "https://example.com/path?key=value&other=123" "$OUTPUT" "Special characters in env file"

# Test 7: Missing env file error
create_template "missing_env.tmpl" '{{ get_env(name="X", default="default") }}'
set +e
OUTPUT=$("$BINARY" --env "$TEST_DIR/nonexistent.env" "$TEST_DIR/missing_env.tmpl" 2>&1)
EXIT_CODE=$?
set -e
assert_contains "$OUTPUT" "Environment file not found" "Missing env file shows error"
assert_equals "1" "$EXIT_CODE" "Missing env file returns exit code 1"

# Test 8: Env file with stdin template
cat > "$TEST_DIR/stdin.env" << 'EOF'
STDIN_VAR=from_stdin
EOF
OUTPUT=$(echo '{{ get_env(name="STDIN_VAR") }}' | "$BINARY" --env "$TEST_DIR/stdin.env" 2>&1)
assert_equals "from_stdin" "$OUTPUT" "Env file works with stdin template"

# Test 9: Env file combined with output flag
cat > "$TEST_DIR/output.env" << 'EOF'
OUTPUT_VAR=test_output
EOF
create_template "output.tmpl" '{{ get_env(name="OUTPUT_VAR") }}'
"$BINARY" --env "$TEST_DIR/output.env" -o "$TEST_DIR/output.txt" "$TEST_DIR/output.tmpl" 2>&1
OUTPUT=$(cat "$TEST_DIR/output.txt")
assert_equals "test_output" "$OUTPUT" "Env file combined with output flag"

# Test 10: Env file combined with validate flag
cat > "$TEST_DIR/validate.env" << 'EOF'
APP_NAME=myapp
PORT=8080
EOF
create_template "validate.tmpl" '{"app": "{{ get_env(name="APP_NAME") }}", "port": {{ get_env(name="PORT") }}}'
OUTPUT=$("$BINARY" --env "$TEST_DIR/validate.env" --validate json "$TEST_DIR/validate.tmpl" 2>&1)
assert_contains "$OUTPUT" '"app": "myapp"' "Env file combined with validate flag"

# Test 11: Override order - last file wins
cat > "$TEST_DIR/first.env" << 'EOF'
VALUE=first
EOF
cat > "$TEST_DIR/second.env" << 'EOF'
VALUE=second
EOF
cat > "$TEST_DIR/third.env" << 'EOF'
VALUE=third
EOF
create_template "order.tmpl" '{{ get_env(name="VALUE") }}'
OUTPUT=$("$BINARY" --env "$TEST_DIR/first.env" --env "$TEST_DIR/second.env" --env "$TEST_DIR/third.env" "$TEST_DIR/order.tmpl" 2>&1)
assert_equals "third" "$OUTPUT" "Override order - last file wins"

# Test 12: Env file with trust mode
cat > "$TEST_DIR/trust.env" << 'EOF'
TRUST_VAR=trusted
EOF
create_template "trust.tmpl" '{{ get_env(name="TRUST_VAR") }}'
OUTPUT=$("$BINARY" --trust --env "$TEST_DIR/trust.env" "$TEST_DIR/trust.tmpl" 2>&1)
assert_equals "trusted" "$OUTPUT" "Env file works with trust mode"

# Test 13: Multiline values in env file
cat > "$TEST_DIR/multiline.env" << 'EOF'
MULTI="line1
line2
line3"
EOF
create_template "multiline.tmpl" '{{ get_env(name="MULTI") }}'
OUTPUT=$("$BINARY" --env "$TEST_DIR/multiline.env" "$TEST_DIR/multiline.tmpl" 2>&1)
assert_contains "$OUTPUT" "line1" "Multiline values - line1"
assert_contains "$OUTPUT" "line2" "Multiline values - line2"
assert_contains "$OUTPUT" "line3" "Multiline values - line3"

# Test 14: Env file doesn't pollute shell environment
cat > "$TEST_DIR/nopollute.env" << 'EOF'
UNIQUE_VAR_12345=secret
EOF
create_template "nopollute.tmpl" '{{ get_env(name="UNIQUE_VAR_12345") }}'
"$BINARY" --env "$TEST_DIR/nopollute.env" "$TEST_DIR/nopollute.tmpl" > /dev/null 2>&1
# Check if env var leaked to shell
if [ -z "${UNIQUE_VAR_12345:-}" ]; then
    pass "Env file variables don't pollute parent shell"
else
    fail "Env file variables don't pollute parent shell" "Variable leaked to shell"
fi

# Test 15: Help shows --env option
OUTPUT=$("$BINARY" --help 2>&1)
if echo "$OUTPUT" | grep -qF -- "--env"; then
    pass "Help shows --env option"
else
    fail "Help shows --env option" "Output does not contain '--env'"
fi
