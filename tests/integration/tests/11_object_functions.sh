#!/usr/bin/env bash
# Test: Object manipulation functions


echo "Test: Object manipulation functions"

# Test: object_keys() returns correct number of keys
create_template "object.tmpltool" '{% set obj = {"a": 1, "b": 2} %}{% set keys = object_keys(object=obj) %}{{ keys | length }}'
OUTPUT=$(run_binary "object.tmpltool")
assert_equals "2" "$OUTPUT" "object_keys() returns correct number of keys"

# Test: json_path() simple path
create_template "json_path1.tmpltool" '{% set obj = {"name": "John", "age": 30} %}{{ json_path(object=obj, path="$.name") }}'
OUTPUT=$(run_binary "json_path1.tmpltool")
assert_equals "John" "$OUTPUT" "json_path() with simple path"

# Test: json_path() nested path
create_template "json_path2.tmpltool" '{% set obj = {"user": {"email": "john@example.com"}} %}{{ json_path(object=obj, path="$.user.email") }}'
OUTPUT=$(run_binary "json_path2.tmpltool")
assert_equals "john@example.com" "$OUTPUT" "json_path() with nested path"

# Test: json_path() array index
create_template "json_path3.tmpltool" '{% set obj = {"items": ["a", "b", "c"]} %}{{ json_path(object=obj, path="$.items[1]") }}'
OUTPUT=$(run_binary "json_path3.tmpltool")
assert_equals "b" "$OUTPUT" "json_path() with array index"

# Test: json_path() wildcard
create_template "json_path4.tmpltool" '{% set obj = {"users": [{"name": "Alice"}, {"name": "Bob"}]} %}{{ json_path(object=obj, path="$.users[*].name") | tojson }}'
OUTPUT=$(run_binary "json_path4.tmpltool")
assert_equals '["Alice","Bob"]' "$OUTPUT" "json_path() with wildcard"

# Test: object_pick() basic
create_template "pick1.tmpltool" '{% set obj = {"a": 1, "b": 2, "c": 3} %}{% set result = object_pick(object=obj, keys=["a", "c"]) %}{{ result | tojson }}'
OUTPUT=$(run_binary "pick1.tmpltool")
assert_contains "$OUTPUT" '"a":1' "object_pick() includes selected key a"
assert_contains "$OUTPUT" '"c":3' "object_pick() includes selected key c"

# Test: object_omit() basic
create_template "omit1.tmpltool" '{% set obj = {"a": 1, "b": 2, "c": 3} %}{% set result = object_omit(object=obj, keys=["b"]) %}{{ result | tojson }}'
OUTPUT=$(run_binary "omit1.tmpltool")
assert_contains "$OUTPUT" '"a":1' "object_omit() keeps key a"
assert_contains "$OUTPUT" '"c":3' "object_omit() keeps key c"
assert_not_contains "$OUTPUT" '"b"' "object_omit() removes key b"

# Test: object_rename_keys() basic
create_template "rename1.tmpltool" '{% set obj = {"old": "value"} %}{% set result = object_rename_keys(object=obj, mapping={"old": "new"}) %}{{ result | tojson }}'
OUTPUT=$(run_binary "rename1.tmpltool")
assert_contains "$OUTPUT" '"new":"value"' "object_rename_keys() renames key"
assert_not_contains "$OUTPUT" '"old"' "object_rename_keys() removes old key"

# Test: object_flatten() basic
create_template "flatten1.tmpltool" '{% set obj = {"a": {"b": 1}} %}{% set result = object_flatten(object=obj) %}{{ result | tojson }}'
OUTPUT=$(run_binary "flatten1.tmpltool")
assert_contains "$OUTPUT" '"a.b":1' "object_flatten() creates dot notation key"

# Test: object_flatten() custom delimiter
create_template "flatten2.tmpltool" '{% set obj = {"a": {"b": 1}} %}{% set result = object_flatten(object=obj, delimiter="_") %}{{ result | tojson }}'
OUTPUT=$(run_binary "flatten2.tmpltool")
assert_contains "$OUTPUT" '"a_b":1' "object_flatten() uses custom delimiter"

# Test: object_unflatten() basic
create_template "unflatten1.tmpltool" '{% set obj = {"a.b.c": "value"} %}{% set result = object_unflatten(object=obj) %}{{ result.a.b.c }}'
OUTPUT=$(run_binary "unflatten1.tmpltool")
assert_equals "value" "$OUTPUT" "object_unflatten() creates nested object"

# Test: object_unflatten() custom delimiter
create_template "unflatten2.tmpltool" '{% set obj = {"a_b": 1} %}{% set result = object_unflatten(object=obj, delimiter="_") %}{{ result.a.b }}'
OUTPUT=$(run_binary "unflatten2.tmpltool")
assert_equals "1" "$OUTPUT" "object_unflatten() uses custom delimiter"

# Test: flatten/unflatten roundtrip
create_template "roundtrip1.tmpltool" '{% set obj = {"user": {"name": "John"}} %}{% set flat = object_flatten(object=obj) %}{% set unflat = object_unflatten(object=flat) %}{{ unflat.user.name }}'
OUTPUT=$(run_binary "roundtrip1.tmpltool")
assert_equals "John" "$OUTPUT" "flatten/unflatten roundtrip preserves data"
