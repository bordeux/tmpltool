#!/usr/bin/env bash
# Test: URL and HTTP utility functions (basic_auth, parse_url, build_url, query_string)

echo "Test: URL and HTTP utility functions"

# ============================================================================
# basic_auth Tests
# ============================================================================

# Test 1: Basic auth with simple credentials
create_template "basic_auth_simple.tmpl" '{{ basic_auth(username="admin", password="secret") }}'
OUTPUT=$(run_binary "basic_auth_simple.tmpl")
assert_equals "$OUTPUT" "Basic YWRtaW46c2VjcmV0" "basic_auth generates correct header"

# Test 2: Basic auth with special characters
create_template "basic_auth_special.tmpl" '{{ basic_auth(username="user@example.com", password="p@ss:w0rd") }}'
OUTPUT=$(run_binary "basic_auth_special.tmpl")
assert_equals "$OUTPUT" "Basic dXNlckBleGFtcGxlLmNvbTpwQHNzOncwcmQ=" "basic_auth handles special characters"

# Test 3: Basic auth in Authorization header
create_template "basic_auth_header.tmpl" 'Authorization: {{ basic_auth(username="api", password="key123") }}'
OUTPUT=$(run_binary "basic_auth_header.tmpl")
assert_equals "$OUTPUT" "Authorization: Basic YXBpOmtleTEyMw==" "basic_auth works in header"

# ============================================================================
# parse_url Tests
# ============================================================================

# Test 4: Parse simple URL - scheme
create_template "parse_url_scheme.tmpl" '{% set url = parse_url(url="https://example.com/path") %}{{ url.scheme }}'
OUTPUT=$(run_binary "parse_url_scheme.tmpl")
assert_equals "$OUTPUT" "https" "parse_url extracts scheme"

# Test 5: Parse simple URL - host
create_template "parse_url_host.tmpl" '{% set url = parse_url(url="https://example.com/path") %}{{ url.host }}'
OUTPUT=$(run_binary "parse_url_host.tmpl")
assert_equals "$OUTPUT" "example.com" "parse_url extracts host"

# Test 6: Parse simple URL - path
create_template "parse_url_path.tmpl" '{% set url = parse_url(url="https://example.com/path") %}{{ url.path }}'
OUTPUT=$(run_binary "parse_url_path.tmpl")
assert_equals "$OUTPUT" "/path" "parse_url extracts path"

# Test 7: Parse URL with port
create_template "parse_url_port.tmpl" '{% set url = parse_url(url="https://example.com:8080/api") %}{{ url.port }}'
OUTPUT=$(run_binary "parse_url_port.tmpl")
assert_equals "$OUTPUT" "8080" "parse_url extracts custom port"

# Test 8: Parse URL with default HTTPS port
create_template "parse_url_default_https.tmpl" '{% set url = parse_url(url="https://example.com/path") %}{{ url.port }}'
OUTPUT=$(run_binary "parse_url_default_https.tmpl")
assert_equals "$OUTPUT" "443" "parse_url returns default HTTPS port"

# Test 9: Parse URL with query string
create_template "parse_url_query.tmpl" '{% set url = parse_url(url="https://example.com/search?q=test&limit=10") %}{{ url.query }}'
OUTPUT=$(run_binary "parse_url_query.tmpl")
assert_equals "$OUTPUT" "q=test&limit=10" "parse_url extracts query string"

# Test 10: Parse URL with fragment
create_template "parse_url_fragment.tmpl" '{% set url = parse_url(url="https://example.com/page#section") %}{{ url.fragment }}'
OUTPUT=$(run_binary "parse_url_fragment.tmpl")
assert_equals "$OUTPUT" "section" "parse_url extracts fragment"

# Test 11: Parse URL with credentials - username
create_template "parse_url_username.tmpl" '{% set url = parse_url(url="https://user:pass@example.com/path") %}{{ url.username }}'
OUTPUT=$(run_binary "parse_url_username.tmpl")
assert_equals "$OUTPUT" "user" "parse_url extracts username"

# Test 12: Parse URL with credentials - password
create_template "parse_url_password.tmpl" '{% set url = parse_url(url="https://user:pass@example.com/path") %}{{ url.password }}'
OUTPUT=$(run_binary "parse_url_password.tmpl")
assert_equals "$OUTPUT" "pass" "parse_url extracts password"

# ============================================================================
# build_url Tests
# ============================================================================

# Test 13: Build simple URL with explicit scheme
create_template "build_url_simple.tmpl" '{{ build_url(scheme="https", host="example.com") }}'
OUTPUT=$(run_binary "build_url_simple.tmpl")
assert_equals "$OUTPUT" "https://example.com/" "build_url creates simple URL"

# Test 13b: Build simple URL with default scheme (https)
create_template "build_url_default_scheme.tmpl" '{{ build_url(host="example.com") }}'
OUTPUT=$(run_binary "build_url_default_scheme.tmpl")
assert_equals "$OUTPUT" "https://example.com/" "build_url uses https as default scheme"

# Test 14: Build URL with port
create_template "build_url_port.tmpl" '{{ build_url(scheme="https", host="example.com", port=8080) }}'
OUTPUT=$(run_binary "build_url_port.tmpl")
assert_equals "$OUTPUT" "https://example.com:8080/" "build_url includes port"

# Test 15: Build URL with path
create_template "build_url_path.tmpl" '{{ build_url(scheme="https", host="example.com", path="/api/v1/users") }}'
OUTPUT=$(run_binary "build_url_path.tmpl")
assert_equals "$OUTPUT" "https://example.com/api/v1/users" "build_url includes path"

# Test 16: Build URL with path without leading slash
create_template "build_url_path_noslash.tmpl" '{{ build_url(scheme="https", host="example.com", path="api/users") }}'
OUTPUT=$(run_binary "build_url_path_noslash.tmpl")
assert_equals "$OUTPUT" "https://example.com/api/users" "build_url adds leading slash to path"

# Test 17: Build URL with query string
create_template "build_url_query.tmpl" '{{ build_url(scheme="https", host="example.com", path="/search", query="q=test&limit=10") }}'
OUTPUT=$(run_binary "build_url_query.tmpl")
assert_equals "$OUTPUT" "https://example.com/search?q=test&limit=10" "build_url includes query string"

# Test 18: Build complete URL
create_template "build_url_complete.tmpl" '{{ build_url(scheme="https", host="api.example.com", port=8080, path="/v1/users", query="active=true") }}'
OUTPUT=$(run_binary "build_url_complete.tmpl")
assert_equals "$OUTPUT" "https://api.example.com:8080/v1/users?active=true" "build_url builds complete URL"

# Test 19: Build HTTP URL
create_template "build_url_http.tmpl" '{{ build_url(scheme="http", host="localhost", port=3000, path="/api") }}'
OUTPUT=$(run_binary "build_url_http.tmpl")
assert_equals "$OUTPUT" "http://localhost:3000/api" "build_url works with HTTP scheme"

# ============================================================================
# query_string Tests
# ============================================================================

# Test 20: Query string with simple params
create_template "query_string_simple.tmpl" '{% set params = {"page": 1, "limit": 20} %}{{ query_string(params=params) }}'
OUTPUT=$(run_binary "query_string_simple.tmpl")
assert_contains "$OUTPUT" "page=1" "query_string includes page param"
assert_contains "$OUTPUT" "limit=20" "query_string includes limit param"

# Test 21: Query string with string values
create_template "query_string_strings.tmpl" '{% set params = {"name": "test", "sort": "asc"} %}{{ query_string(params=params) }}'
OUTPUT=$(run_binary "query_string_strings.tmpl")
assert_contains "$OUTPUT" "name=test" "query_string includes string values"

# Test 22: Query string with special characters
create_template "query_string_special.tmpl" '{% set params = {"query": "hello world"} %}{{ query_string(params=params) }}'
OUTPUT=$(run_binary "query_string_special.tmpl")
# URL encoding can be + or %20 for spaces
if [[ "$OUTPUT" == *"query=hello+world"* ]] || [[ "$OUTPUT" == *"query=hello%20world"* ]]; then
    pass "query_string encodes special characters"
else
    fail "query_string encodes special characters" "Expected 'query=hello+world' or 'query=hello%20world', got '$OUTPUT'"
fi

# Test 23: Query string with email encoding
create_template "query_string_email.tmpl" '{% set params = {"email": "user@example.com"} %}{{ query_string(params=params) }}'
OUTPUT=$(run_binary "query_string_email.tmpl")
assert_equals "$OUTPUT" "email=user%40example.com" "query_string encodes @ symbol"

# Test 24: Query string with boolean values
create_template "query_string_bool.tmpl" '{% set params = {"active": true} %}{{ query_string(params=params) }}'
OUTPUT=$(run_binary "query_string_bool.tmpl")
assert_contains "$OUTPUT" "active=true" "query_string handles boolean values"

# Test 25: Query string empty object
create_template "query_string_empty.tmpl" '{% set params = {} %}{{ query_string(params=params) }}'
OUTPUT=$(run_binary "query_string_empty.tmpl")
assert_equals "$OUTPUT" "" "query_string returns empty string for empty object"

# ============================================================================
# Combined use cases
# ============================================================================

# Test 26: Build URL with query_string function
create_template "combined_build_query.tmpl" '{% set params = {"page": 1, "limit": 10} %}{{ build_url(scheme="https", host="api.example.com", path="/users", query=query_string(params=params)) }}'
OUTPUT=$(run_binary "combined_build_query.tmpl")
assert_contains "$OUTPUT" "https://api.example.com/users?" "combined build_url with query_string"

# Test 26b: Build URL with query object directly
create_template "build_url_query_object.tmpl" '{% set params = {"page": 1, "limit": 10} %}{{ build_url(host="api.example.com", path="/users", query=params) }}'
OUTPUT=$(run_binary "build_url_query_object.tmpl")
assert_contains "$OUTPUT" "https://api.example.com/users?" "build_url accepts query as object"
assert_contains "$OUTPUT" "page=1" "build_url query object includes page"
assert_contains "$OUTPUT" "limit=10" "build_url query object includes limit"

# Test 27: Parse and rebuild URL
create_template "combined_parse_build.tmpl" '{% set original = parse_url(url="https://example.com:8080/api") %}{{ build_url(scheme=original.scheme, host=original.host, port=original.port, path=original.path) }}'
OUTPUT=$(run_binary "combined_parse_build.tmpl")
assert_equals "$OUTPUT" "https://example.com:8080/api" "parse_url and build_url round-trip"

# Test 28: API request with auth header
create_template "combined_api_request.tmpl" 'curl -H "Authorization: {{ basic_auth(username="user", password="pass") }}" {{ build_url(scheme="https", host="api.example.com", path="/v1/data") }}'
OUTPUT=$(run_binary "combined_api_request.tmpl")
assert_equals "$OUTPUT" 'curl -H "Authorization: Basic dXNlcjpwYXNz" https://api.example.com/v1/data' "combined API request example"
