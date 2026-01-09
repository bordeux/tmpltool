#!/usr/bin/env bash
# Test: JSON serialization


echo "Test: JSON serialization"

# Test: to_json() serializes correctly
create_template "serialize.tmpltool" '{% set obj = {"test": "value"} %}{{ to_json(object=obj) }}'
OUTPUT=$(run_binary "serialize.tmpltool")
assert_equals '{"test":"value"}' "$OUTPUT" "to_json() serializes correctly"
