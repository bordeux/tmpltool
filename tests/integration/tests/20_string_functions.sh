#!/usr/bin/env bash
# Test: String manipulation functions

echo "Test: String manipulation functions"

# Test 1: regex_replace basic
create_template "regex_replace.tmpl" '{{ regex_replace(string="hello123world", pattern="[0-9]+", replacement="-") }}'
OUTPUT=$(run_binary "regex_replace.tmpl")
assert_equals "hello-world" "$OUTPUT" "regex_replace replaces digits with dash"

# Test 2: regex_replace whitespace
create_template "regex_replace_ws.tmpl" '{{ regex_replace(string="foo bar baz", pattern="\\s+", replacement="_") }}'
OUTPUT=$(run_binary "regex_replace_ws.tmpl")
assert_equals "foo_bar_baz" "$OUTPUT" "regex_replace replaces whitespace with underscore"

# Test 3: regex_match found
create_template "regex_match_found.tmpl" '{{ regex_match(string="hello123", pattern="[0-9]+") }}'
OUTPUT=$(run_binary "regex_match_found.tmpl")
assert_equals "true" "$OUTPUT" "regex_match returns true when pattern found"

# Test 4: regex_match not found
create_template "regex_match_not.tmpl" '{{ regex_match(string="hello", pattern="[0-9]+") }}'
OUTPUT=$(run_binary "regex_match_not.tmpl")
assert_equals "false" "$OUTPUT" "regex_match returns false when pattern not found"

# Test 5: regex_find_all
create_template "regex_find_all.tmpl" '{{ regex_find_all(string="a1b2c3", pattern="[0-9]+") | length }}'
OUTPUT=$(run_binary "regex_find_all.tmpl")
assert_equals "3" "$OUTPUT" "regex_find_all finds all matches"

# Test 6: substring with length
create_template "substring_len.tmpl" '{{ substring(string="hello world", start=0, length=5) }}'
OUTPUT=$(run_binary "substring_len.tmpl")
assert_equals "hello" "$OUTPUT" "substring extracts first 5 characters"

# Test 7: substring from position to end
create_template "substring_end.tmpl" '{{ substring(string="hello world", start=6) }}'
OUTPUT=$(run_binary "substring_end.tmpl")
assert_equals "world" "$OUTPUT" "substring extracts from position to end"

# Test 8: substring negative start
create_template "substring_neg.tmpl" '{{ substring(string="hello world", start=-5) }}'
OUTPUT=$(run_binary "substring_neg.tmpl")
assert_equals "world" "$OUTPUT" "substring with negative start counts from end"

# Test 9: contains found
create_template "contains_found.tmpl" '{{ contains(string="hello world", substring="world") }}'
OUTPUT=$(run_binary "contains_found.tmpl")
assert_equals "true" "$OUTPUT" "contains returns true when substring found"

# Test 10: contains not found
create_template "contains_not.tmpl" '{{ contains(string="hello world", substring="foo") }}'
OUTPUT=$(run_binary "contains_not.tmpl")
assert_equals "false" "$OUTPUT" "contains returns false when substring not found"

# Test 11: index_of found
create_template "index_of_found.tmpl" '{{ index_of(string="hello world", substring="world") }}'
OUTPUT=$(run_binary "index_of_found.tmpl")
assert_equals "6" "$OUTPUT" "index_of returns correct position"

# Test 12: index_of not found
create_template "index_of_not.tmpl" '{{ index_of(string="hello world", substring="foo") }}'
OUTPUT=$(run_binary "index_of_not.tmpl")
assert_equals "-1" "$OUTPUT" "index_of returns -1 when not found"

# Test 13: count_occurrences
create_template "count_occ.tmpl" '{{ count_occurrences(string="hello hello hello", substring="hello") }}'
OUTPUT=$(run_binary "count_occ.tmpl")
assert_equals "3" "$OUTPUT" "count_occurrences counts all occurrences"

# Test 14: truncate with default suffix
create_template "truncate.tmpl" '{{ truncate(string="Hello World", length=8) }}'
OUTPUT=$(run_binary "truncate.tmpl")
assert_equals "Hello..." "$OUTPUT" "truncate adds ellipsis"

# Test 15: truncate with custom suffix
create_template "truncate_custom.tmpl" '{{ truncate(string="Hello World", length=8, suffix=">>") }}'
OUTPUT=$(run_binary "truncate_custom.tmpl")
assert_equals "Hello >>" "$OUTPUT" "truncate with custom suffix"

# Test 16: word_count
create_template "word_count.tmpl" '{{ word_count(string="one two three four") }}'
OUTPUT=$(run_binary "word_count.tmpl")
assert_equals "4" "$OUTPUT" "word_count counts words correctly"

# Test 17: split_lines
create_template "split_lines.tmpl" '{{ split_lines(string="a\nb\nc") | length }}'
OUTPUT=$(run_binary "split_lines.tmpl")
assert_equals "3" "$OUTPUT" "split_lines splits into correct number of lines"
