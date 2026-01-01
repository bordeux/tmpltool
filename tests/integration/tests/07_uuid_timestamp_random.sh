#!/usr/bin/env bash
# Test: UUID generation, timestamps, and random numbers


echo "Test: UUID generation, timestamps, and random numbers"

# Test 1: UUID v4 (default) has correct format
create_template "uuid.tmpl" '{{ uuid() }}'
OUTPUT=$(run_binary "uuid.tmpl")
# UUID v4 format: xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx
assert_matches "$OUTPUT" '^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$' "UUID v4 (default) has correct format"

# Test 2: UUID v4 explicit version has correct format
create_template "uuid_v4.tmpl" '{{ uuid(version="v4") }}'
OUTPUT=$(run_binary "uuid_v4.tmpl")
assert_matches "$OUTPUT" '^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$' "UUID v4 explicit has correct format"

# Test 3: UUID v7 has correct format (version digit is 7)
create_template "uuid_v7.tmpl" '{{ uuid(version="v7") }}'
OUTPUT=$(run_binary "uuid_v7.tmpl")
# UUID v7 format: xxxxxxxx-xxxx-7xxx-yxxx-xxxxxxxxxxxx
assert_matches "$OUTPUT" '^[0-9a-f]{8}-[0-9a-f]{4}-7[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$' "UUID v7 has correct format"

# Test 4: UUID invalid version returns error
create_template "uuid_invalid.tmpl" '{{ uuid(version="v99") }}'
OUTPUT=$(run_binary "uuid_invalid.tmpl" 2>&1) || true
assert_contains "$OUTPUT" "Invalid UUID version" "UUID invalid version returns error"

# Test 5: now() returns valid ISO8601 timestamp
create_template "timestamp.tmpl" '{{ now() }}'
OUTPUT=$(run_binary "timestamp.tmpl")
assert_matches "$OUTPUT" '^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}' "now() returns valid ISO8601 timestamp"

# Test 6: get_random() returns number in range
create_template "random.tmpl" '{{ get_random(start=1, end=100) }}'
OUTPUT=$(run_binary "random.tmpl")
if echo "$OUTPUT" | grep -qE '^[0-9]+$' && [ "$OUTPUT" -ge 1 ] && [ "$OUTPUT" -le 100 ]; then
    pass "get_random() returns number in range"
else
    fail "get_random() returns number in range" "Output out of range or invalid: $OUTPUT"
fi
