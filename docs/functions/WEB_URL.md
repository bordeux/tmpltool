## Web & URL Functions

URL parsing, encoding/decoding, query string generation, and HTTP Basic Auth.

**See also:** [Function Reference](../FUNCTIONS.md) | [Main Documentation](../README.md)

URL manipulation and HTTP authentication helpers.

#### `basic_auth(username, password)`

Generate HTTP Basic Authentication header value.

**Arguments:**
- `username` (required): The username for authentication
- `password` (required): The password for authentication

**Returns:** Base64-encoded Basic authentication header value

**Example:**
```jinja
{# Generate Basic Auth header #}
Authorization: {{ basic_auth(username="admin", password="secret123") }}
{# Output: Authorization: Basic YWRtaW46c2VjcmV0MTIz #}

{# Use with environment variables #}
Authorization: {{ basic_auth(username=get_env(name="API_USER"), password=get_env(name="API_PASS")) }}

{# In nginx config #}
proxy_set_header Authorization "{{ basic_auth(username="api_user", password="api_key") }}";
```

#### `url_encode(string)` / `| url_encode`

URL-encode a string for safe use in URLs.

**Arguments:**
- `string` (required): The string to encode

**Returns:** URL-encoded string

**Example:**
```jinja
{# Function syntax #}
{{ url_encode(string="hello world") }}
{# Output: hello%20world #}

{# Filter syntax #}
{{ "hello world" | url_encode }}
{# Output: hello%20world #}

{# Encoding special characters #}
{{ "foo=bar&baz=qux" | url_encode }}
{# Output: foo%3Dbar%26baz%3Dqux #}

{# In a query parameter #}
{% set search_term = "jinja templates" %}
https://example.com/search?q={{ search_term | url_encode }}
{# Output: https://example.com/search?q=jinja%20templates #}
```

#### `url_decode(string)` / `| url_decode`

Decode a percent-encoded URL string.

**Arguments:**
- `string` (required): The URL-encoded string to decode

**Returns:** Decoded string

**Example:**
```jinja
{# Function syntax #}
{{ url_decode(string="hello%20world") }}
{# Output: hello world #}

{# Filter syntax #}
{{ "hello%20world" | url_decode }}
{# Output: hello world #}

{# Decoding special characters #}
{{ "foo%3Dbar%26baz%3Dqux" | url_decode }}
{# Output: foo=bar&baz=qux #}

{# Chaining with other operations #}
{{ encoded_input | url_decode | upper }}
```

#### `parse_url(url)` / `| parse_url`

Parse a URL into its component parts.

**Arguments:**
- `url` (required): The URL string to parse

**Returns:** Object with the following fields:
- `scheme`: URL scheme (http, https, etc.)
- `host`: Hostname
- `port`: Port number (or default for scheme)
- `path`: Path component
- `query`: Query string (without ?)
- `fragment`: Fragment/hash (without #)
- `username`: Username from URL (if present)
- `password`: Password from URL (if present)

**Example:**
```jinja
{# Function syntax #}
{% set url = parse_url(url="https://user:pass@api.example.com:8080/v1/users?limit=10#section") %}
Scheme: {{ url.scheme }}
Host: {{ url.host }}
Port: {{ url.port }}
Path: {{ url.path }}
Query: {{ url.query }}
{# Output:
Scheme: https
Host: api.example.com
Port: 8080
Path: /v1/users
Query: limit=10
#}

{# Filter syntax #}
{% set url = "https://api.example.com:8080/path" | parse_url %}
{{ url.host }}:{{ url.port }}
{# Output: api.example.com:8080 #}

{# Extract host from environment variable #}
{% set db_url = get_env(name="DATABASE_URL") | parse_url %}
DB_HOST={{ db_url.host }}
DB_PORT={{ db_url.port }}
DB_NAME={{ db_url.path | trim_start_matches(pat="/") }}
```

#### `build_url(scheme, host, port, path, query)`

Construct a URL from components.

**Arguments:**
- `scheme` (optional): URL scheme (default: `"https"`)
- `host` (required): Hostname
- `port` (optional): Port number
- `path` (optional): Path component (default: `"/"`)
- `query` (optional): Query string (string) or query parameters (object)

**Returns:** Constructed URL string

**Example:**
```jinja
{# Basic URL with defaults #}
{{ build_url(host="api.example.com") }}
{# Output: https://api.example.com/ #}

{# Full URL with all components #}
{{ build_url(scheme="http", host="localhost", port=8080, path="/api/v1", query="debug=true") }}
{# Output: http://localhost:8080/api/v1?debug=true #}

{# Query as object (auto-serialized) #}
{{ build_url(host="api.example.com", path="/search", query={"q": "jinja templates", "limit": 20}) }}
{# Output: https://api.example.com/search?q=jinja+templates&limit=20 #}

{# Build API endpoint from config #}
{% set api_url = build_url(
    scheme="https",
    host=get_env(name="API_HOST", default="api.example.com"),
    port=get_env(name="API_PORT") | default(value=443),
    path="/v2/data"
) %}
API_ENDPOINT={{ api_url }}
```

#### `query_string(params)`

Build a URL query string from an object.

**Arguments:**
- `params` (required): Object containing key-value pairs for the query string

**Returns:** URL-encoded query string (without leading `?`)

**Example:**
```jinja
{# Basic query string #}
{% set params = {"name": "John Doe", "age": 30, "city": "New York"} %}
{{ query_string(params=params) }}
{# Output: name=John+Doe&age=30&city=New+York #}

{# URL encoding for special characters #}
{% set search = {"q": "hello world", "filter": "type=user&active=true"} %}
?{{ query_string(params=search) }}
{# Output: ?q=hello+world&filter=type%3Duser%26active%3Dtrue #}

{# Build complete URL with query #}
{% set endpoint = "https://api.example.com/search" %}
{% set params = {"page": 1, "limit": 50, "sort": "created_at"} %}
{{ endpoint }}?{{ query_string(params=params) }}
{# Output: https://api.example.com/search?page=1&limit=50&sort=created_at #}
```

