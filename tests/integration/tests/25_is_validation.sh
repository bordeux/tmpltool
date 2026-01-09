#!/usr/bin/env bash
# Test: Is-functions validation (is email, is url, is ip, is uuid)


echo "Test: Is-functions validation"

# ========== is email tests ==========

# Test 1: Valid email with "is" syntax
create_template "is_email_valid.tmpltool" '{% if "test@example.com" is email %}valid{% else %}invalid{% endif %}'
OUTPUT=$(run_binary "is_email_valid.tmpltool")
assert_equals "valid" "$OUTPUT" "Valid email passes 'is email' test"

# Test 2: Invalid email with "is" syntax
create_template "is_email_invalid.tmpltool" '{% if "not-an-email" is email %}valid{% else %}invalid{% endif %}'
OUTPUT=$(run_binary "is_email_invalid.tmpltool")
assert_equals "invalid" "$OUTPUT" "Invalid email fails 'is email' test"

# Test 3: Email function syntax still works
create_template "is_email_fn.tmpltool" '{{ is_email(string="user@domain.org") }}'
OUTPUT=$(run_binary "is_email_fn.tmpltool")
assert_equals "true" "$OUTPUT" "is_email function syntax works"

# Test 4: Email with variable
create_template "is_email_var.tmpltool" '{% set addr = "test@example.com" %}{% if addr is email %}yes{% else %}no{% endif %}'
OUTPUT=$(run_binary "is_email_var.tmpltool")
assert_equals "yes" "$OUTPUT" "Email variable passes 'is email' test"

# ========== is url tests ==========

# Test 5: Valid URL with "is" syntax
create_template "is_url_valid.tmpltool" '{% if "https://example.com" is url %}valid{% else %}invalid{% endif %}'
OUTPUT=$(run_binary "is_url_valid.tmpltool")
assert_equals "valid" "$OUTPUT" "Valid URL passes 'is url' test"

# Test 6: Invalid URL with "is" syntax
create_template "is_url_invalid.tmpltool" '{% if "example.com" is url %}valid{% else %}invalid{% endif %}'
OUTPUT=$(run_binary "is_url_invalid.tmpltool")
assert_equals "invalid" "$OUTPUT" "URL without scheme fails 'is url' test"

# Test 7: URL function syntax still works
create_template "is_url_fn.tmpltool" '{{ is_url(string="http://example.com/path") }}'
OUTPUT=$(run_binary "is_url_fn.tmpltool")
assert_equals "true" "$OUTPUT" "is_url function syntax works"

# ========== is ip tests ==========

# Test 8: Valid IPv4 with "is" syntax
create_template "is_ip_v4.tmpltool" '{% if "192.168.1.1" is ip %}valid{% else %}invalid{% endif %}'
OUTPUT=$(run_binary "is_ip_v4.tmpltool")
assert_equals "valid" "$OUTPUT" "Valid IPv4 passes 'is ip' test"

# Test 9: Valid IPv6 with "is" syntax
create_template "is_ip_v6.tmpltool" '{% if "::1" is ip %}valid{% else %}invalid{% endif %}'
OUTPUT=$(run_binary "is_ip_v6.tmpltool")
assert_equals "valid" "$OUTPUT" "Valid IPv6 passes 'is ip' test"

# Test 10: Invalid IP with "is" syntax
create_template "is_ip_invalid.tmpltool" '{% if "256.1.1.1" is ip %}valid{% else %}invalid{% endif %}'
OUTPUT=$(run_binary "is_ip_invalid.tmpltool")
assert_equals "invalid" "$OUTPUT" "Invalid IP fails 'is ip' test"

# Test 11: IP function syntax still works
create_template "is_ip_fn.tmpltool" '{{ is_ip(string="127.0.0.1") }}'
OUTPUT=$(run_binary "is_ip_fn.tmpltool")
assert_equals "true" "$OUTPUT" "is_ip function syntax works"

# ========== is uuid tests ==========

# Test 12: Valid UUID with "is" syntax
create_template "is_uuid_valid.tmpltool" '{% if "550e8400-e29b-41d4-a716-446655440000" is uuid %}valid{% else %}invalid{% endif %}'
OUTPUT=$(run_binary "is_uuid_valid.tmpltool")
assert_equals "valid" "$OUTPUT" "Valid UUID passes 'is uuid' test"

# Test 13: Invalid UUID with "is" syntax
create_template "is_uuid_invalid.tmpltool" '{% if "not-a-uuid" is uuid %}valid{% else %}invalid{% endif %}'
OUTPUT=$(run_binary "is_uuid_invalid.tmpltool")
assert_equals "invalid" "$OUTPUT" "Invalid UUID fails 'is uuid' test"

# Test 14: UUID function syntax still works
create_template "is_uuid_fn.tmpltool" '{{ is_uuid(string="f47ac10b-58cc-4372-a567-0e02b2c3d479") }}'
OUTPUT=$(run_binary "is_uuid_fn.tmpltool")
assert_equals "true" "$OUTPUT" "is_uuid function syntax works"

# ========== combined and negation tests ==========

# Test 15: Multiple is checks in one template
create_template "is_multi.tmpltool" '{% if "test@example.com" is email %}e{% endif %}{% if "https://x.com" is url %}u{% endif %}{% if "1.2.3.4" is ip %}i{% endif %}'
OUTPUT=$(run_binary "is_multi.tmpltool")
assert_equals "eui" "$OUTPUT" "Multiple is checks work in one template"

# Test 16: Negated is check (is not)
create_template "is_not.tmpltool" '{% if "not-email" is not email %}invalid{% else %}valid{% endif %}'
OUTPUT=$(run_binary "is_not.tmpltool")
assert_equals "invalid" "$OUTPUT" "'is not' negation works"

# Test 17: Non-string value returns false
create_template "is_nonstring.tmpltool" '{% if 123 is email %}yes{% else %}no{% endif %}'
OUTPUT=$(run_binary "is_nonstring.tmpltool")
assert_equals "no" "$OUTPUT" "Non-string value returns false for is email"
