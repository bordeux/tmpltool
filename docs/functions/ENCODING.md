## Encoding & Security Functions

Base64, hex encoding/decoding, bcrypt, HMAC, and HTML/XML/shell escaping functions.

**See also:** [Function Reference](../FUNCTIONS.md) | [Main Documentation](../README.md)

Functions for encoding, decoding, password hashing, and escaping data for various contexts.

#### `base64_encode`

Encode a string to Base64 format. Supports both function and filter syntax.

**Arguments:**
- `string` (required) - String to encode

**Returns:** Base64-encoded string

**Examples:**
```jinja
{# Function syntax #}
{{ base64_encode(string="Hello World") }}
{# Output: SGVsbG8gV29ybGQ= #}

{# Filter syntax #}
{{ "Hello World" | base64_encode }}
{# Output: SGVsbG8gV29ybGQ= #}

{# Basic Authentication header #}
{% set credentials = "admin:password123" %}
Authorization: Basic {{ credentials | base64_encode }}
```

#### `base64_decode`

Decode a Base64-encoded string. Supports both function and filter syntax.

**Arguments:**
- `string` (required) - Base64 string to decode

**Returns:** Decoded string

**Examples:**
```jinja
{# Function syntax #}
{{ base64_decode(string="SGVsbG8gV29ybGQ=") }}
{# Output: Hello World #}

{# Filter syntax #}
{{ "SGVsbG8gV29ybGQ=" | base64_decode }}
{# Output: Hello World #}
```

#### `hex_encode`

Encode a string to hexadecimal format. Supports both function and filter syntax.

**Arguments:**
- `string` (required) - String to encode

**Returns:** Hexadecimal string (lowercase)

**Examples:**
```jinja
{# Function syntax #}
{{ hex_encode(string="Hello") }}
{# Output: 48656c6c6f #}

{# Filter syntax #}
{{ "Hello" | hex_encode }}
{# Output: 48656c6c6f #}
```

#### `hex_decode`

Decode a hexadecimal-encoded string. Supports both function and filter syntax.

**Arguments:**
- `string` (required) - Hexadecimal string to decode

**Returns:** Decoded string

**Examples:**
```jinja
{# Function syntax #}
{{ hex_decode(string="48656c6c6f") }}
{# Output: Hello #}

{# Filter syntax #}
{{ "48656c6c6f" | hex_decode }}
{# Output: Hello #}
```

#### `bcrypt(password, rounds)`

Generate a bcrypt hash for password storage. Each run produces a different hash due to the random salt.

**Arguments:**
- `password` (required) - Password to hash
- `rounds` (optional) - Cost factor from 4-31 (default: 12, higher = more secure but slower)

**Returns:** Bcrypt hash string

**Examples:**
```jinja
{# Generate password hash #}
Password hash: {{ bcrypt(password="mypassword") }}

{# Higher security (slower) #}
Password hash: {{ bcrypt(password="mypassword", rounds=14) }}

{# Use with environment variable #}
{% set user_pass = get_env(name="USER_PASSWORD") %}
DB_PASSWORD_HASH={{ bcrypt(password=user_pass, rounds=12) }}
```

**Note:** Use bcrypt for password storage, not the SHA functions. Bcrypt includes automatic salting and is designed to be computationally expensive to prevent brute-force attacks.

#### `generate_secret(length, charset)`

Generate a cryptographically secure random string.

**Arguments:**
- `length` (required) - Length of string to generate (1-1024)
- `charset` (optional) - Character set: `"alphanumeric"` (default), `"hex"`, or `"base64"`

**Returns:** Cryptographically secure random string

**Examples:**
```jinja
{# Generate API key #}
API_KEY={{ generate_secret(length=32) }}

{# Generate hex token #}
SECRET_TOKEN={{ generate_secret(length=64, charset="hex") }}

{# Generate base64 secret #}
WEBHOOK_SECRET={{ generate_secret(length=48, charset="base64") }}
```

**Practical Example:**
```bash
# Generate secure credentials
API_KEY={{ generate_secret(length=32, charset="hex") }}
JWT_SECRET={{ generate_secret(length=64, charset="base64") }}
SESSION_SECRET={{ generate_secret(length=32) }}
CSRF_TOKEN={{ generate_secret(length=40, charset="hex") }}
```

#### `hmac_sha256(key, message)`

Generate HMAC-SHA256 signature for message authentication.

**Arguments:**
- `key` (required) - Secret key
- `message` (required) - Message to sign

**Returns:** HMAC signature as hexadecimal string

**Examples:**
```jinja
{# Sign a message #}
{% set signature = hmac_sha256(key="secret_key", message="important data") %}
X-Signature: {{ signature }}

{# Webhook signature #}
{% set payload = '{"user_id": 123, "action": "update"}' %}
{% set webhook_secret = get_env(name="WEBHOOK_SECRET") %}
X-Hub-Signature-256: sha256={{ hmac_sha256(key=webhook_secret, message=payload) }}
```

#### `escape_html`

Escape HTML entities to prevent XSS attacks. Supports both function and filter syntax.

**Arguments:**
- `string` (required) - String to escape

**Returns:** HTML-escaped string

**Examples:**
```jinja
{# Function syntax #}
{% set user_input = '<script>alert("XSS")</script>' %}
<div>{{ escape_html(string=user_input) }}</div>
{# Output: &lt;script&gt;alert(&quot;XSS&quot;)&lt;/script&gt; #}

{# Filter syntax #}
<div>{{ user_input | escape_html }}</div>

{# Safe HTML output from env var #}
<p>User comment: {{ get_env(name="USER_COMMENT", default="") | escape_html }}</p>
```

#### `escape_xml`

Escape XML entities. Supports both function and filter syntax.

**Arguments:**
- `string` (required) - String to escape

**Returns:** XML-escaped string

**Examples:**
```jinja
{# Function syntax #}
{% set content = '<tag attr="value">text & more</tag>' %}
<data>{{ escape_xml(string=content) }}</data>
{# Output: &lt;tag attr=&quot;value&quot;&gt;text &amp; more&lt;/tag&gt; #}

{# Filter syntax #}
<data>{{ content | escape_xml }}</data>
```

#### `escape_shell`

Escape string for safe use in shell commands. Supports both function and filter syntax.

**Arguments:**
- `string` (required) - String to escape

**Returns:** Shell-escaped string (single-quoted)

**Examples:**
```jinja
{# Function syntax #}
{% set filename = "my file with spaces.txt" %}
Command: cat {{ escape_shell(string=filename) }}
{# Output: cat 'my file with spaces.txt' #}

{# Filter syntax #}
Command: cat {{ filename | escape_shell }}

{# Escape special characters #}
{% set message = "it's working!" %}
echo {{ message | escape_shell }}
{# Output: echo 'it'\''s working!' #}
```

**Security Warning:** While `escape_shell` helps prevent injection, the safest approach is to avoid dynamic shell commands entirely when possible. Use `exec()` function only with trusted, hardcoded commands.

#### Chaining Encoding Filters

Encoding functions can be chained with hash functions for powerful transformations:

```jinja
{# Encode then hash #}
{{ "Hello" | base64_encode | sha256 }}
{# First: "SGVsbG8=" then: SHA-256 of that #}

{# Hash then encode #}
{{ "Hello" | sha256 | base64_encode }}
{# First: hex hash, then: base64 of the hex string #}

{# Multi-step encoding #}
{{ "secret" | hex_encode | base64_encode }}

{# Decode chain #}
{{ "NjE2MjYz" | base64_decode | hex_decode }}
{# Output: abc #}
```

