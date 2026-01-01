#!/usr/bin/env bash
# Test: Extended string functions (wrap, center, sentence_case, strip_html, strip_ansi, normalize_whitespace, to_constant_case, pluralize)

echo "Test: Extended string functions"

# Test 1: wrap basic
create_template "wrap_basic.tmpl" '{{ wrap(string="Hello World Example", width=10) }}'
OUTPUT=$(run_binary "wrap_basic.tmpl")
assert_contains "$OUTPUT" "Hello" "wrap basic contains Hello"

# Test 2: wrap with indent
create_template "wrap_indent.tmpl" '{{ wrap(string="One Two Three Four", width=8, indent="  ") }}'
OUTPUT=$(run_binary "wrap_indent.tmpl")
assert_contains "$OUTPUT" "  " "wrap with indent contains indent"

# Test 3: center basic
create_template "center_basic.tmpl" '{{ center(string="hi", width=6) }}'
OUTPUT=$(run_binary "center_basic.tmpl")
assert_equals "  hi  " "$OUTPUT" "center basic"

# Test 4: center with custom char
create_template "center_char.tmpl" '{{ center(string="hi", width=8, char="-") }}'
OUTPUT=$(run_binary "center_char.tmpl")
assert_equals "---hi---" "$OUTPUT" "center with custom char"

# Test 5: center string longer than width
create_template "center_long.tmpl" '{{ center(string="hello", width=3) }}'
OUTPUT=$(run_binary "center_long.tmpl")
assert_equals "hello" "$OUTPUT" "center with string longer than width"

# Test 6: sentence_case lowercase
create_template "sentence_case_lower.tmpl" '{{ sentence_case(string="hello world") }}'
OUTPUT=$(run_binary "sentence_case_lower.tmpl")
assert_equals "Hello world" "$OUTPUT" "sentence_case from lowercase"

# Test 7: sentence_case uppercase
create_template "sentence_case_upper.tmpl" '{{ sentence_case(string="HELLO WORLD") }}'
OUTPUT=$(run_binary "sentence_case_upper.tmpl")
assert_equals "Hello world" "$OUTPUT" "sentence_case from uppercase"

# Test 8: strip_html simple
create_template "strip_html_simple.tmpl" '{{ strip_html(string="<p>Hello</p>") }}'
OUTPUT=$(run_binary "strip_html_simple.tmpl")
assert_equals "Hello" "$OUTPUT" "strip_html simple"

# Test 9: strip_html nested
create_template "strip_html_nested.tmpl" '{{ strip_html(string="<div><p>Hello <b>World</b></p></div>") }}'
OUTPUT=$(run_binary "strip_html_nested.tmpl")
assert_equals "Hello World" "$OUTPUT" "strip_html nested tags"

# Test 10: strip_html with attributes
create_template "strip_html_attr.tmpl" '{{ strip_html(string="<a href=\"link\">Click</a>") }}'
OUTPUT=$(run_binary "strip_html_attr.tmpl")
assert_equals "Click" "$OUTPUT" "strip_html with attributes"

# Test 11: strip_ansi color code
create_template "strip_ansi_color.tmpl" '{{ strip_ansi(string="\x1b[31mRed\x1b[0m") }}'
OUTPUT=$(run_binary "strip_ansi_color.tmpl")
assert_equals "Red" "$OUTPUT" "strip_ansi removes color codes"

# Test 12: strip_ansi plain text
create_template "strip_ansi_plain.tmpl" '{{ strip_ansi(string="Plain text") }}'
OUTPUT=$(run_binary "strip_ansi_plain.tmpl")
assert_equals "Plain text" "$OUTPUT" "strip_ansi preserves plain text"

# Test 13: normalize_whitespace spaces
create_template "normalize_spaces.tmpl" '{{ normalize_whitespace(string="  hello   world  ") }}'
OUTPUT=$(run_binary "normalize_spaces.tmpl")
assert_equals "hello world" "$OUTPUT" "normalize_whitespace collapses spaces"

# Test 14: normalize_whitespace mixed
create_template "normalize_mixed.tmpl" '{{ normalize_whitespace(string="a  b  c") }}'
OUTPUT=$(run_binary "normalize_mixed.tmpl")
assert_equals "a b c" "$OUTPUT" "normalize_whitespace mixed whitespace"

# Test 15: to_constant_case spaces
create_template "constant_case_spaces.tmpl" '{{ to_constant_case(string="hello world") }}'
OUTPUT=$(run_binary "constant_case_spaces.tmpl")
assert_equals "HELLO_WORLD" "$OUTPUT" "to_constant_case from spaces"

# Test 16: to_constant_case camelCase
create_template "constant_case_camel.tmpl" '{{ to_constant_case(string="helloWorld") }}'
OUTPUT=$(run_binary "constant_case_camel.tmpl")
assert_equals "HELLO_WORLD" "$OUTPUT" "to_constant_case from camelCase"

# Test 17: to_constant_case kebab-case
create_template "constant_case_kebab.tmpl" '{{ to_constant_case(string="hello-world") }}'
OUTPUT=$(run_binary "constant_case_kebab.tmpl")
assert_equals "HELLO_WORLD" "$OUTPUT" "to_constant_case from kebab-case"

# Test 18: pluralize singular
create_template "pluralize_singular.tmpl" '{{ pluralize(count=1, singular="item") }}'
OUTPUT=$(run_binary "pluralize_singular.tmpl")
assert_equals "item" "$OUTPUT" "pluralize returns singular for count=1"

# Test 19: pluralize plural default
create_template "pluralize_plural.tmpl" '{{ pluralize(count=5, singular="item") }}'
OUTPUT=$(run_binary "pluralize_plural.tmpl")
assert_equals "items" "$OUTPUT" "pluralize returns plural for count>1"

# Test 20: pluralize zero
create_template "pluralize_zero.tmpl" '{{ pluralize(count=0, singular="item") }}'
OUTPUT=$(run_binary "pluralize_zero.tmpl")
assert_equals "items" "$OUTPUT" "pluralize returns plural for count=0"

# Test 21: pluralize custom plural
create_template "pluralize_custom.tmpl" '{{ pluralize(count=2, singular="child", plural="children") }}'
OUTPUT=$(run_binary "pluralize_custom.tmpl")
assert_equals "children" "$OUTPUT" "pluralize with custom plural form"

# Test 22: pluralize custom singular
create_template "pluralize_custom_sing.tmpl" '{{ pluralize(count=1, singular="person", plural="people") }}'
OUTPUT=$(run_binary "pluralize_custom_sing.tmpl")
assert_equals "person" "$OUTPUT" "pluralize custom returns singular for count=1"
