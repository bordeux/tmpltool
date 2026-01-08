## Validation Functions

Validate emails, URLs, IP addresses, and UUIDs.

**See also:** [Function Reference](../FUNCTIONS.md) | [Main Documentation](../README.md)

Validate strings against specific formats. Useful for validating user input, configuration values, or data from external sources.

These functions support two syntaxes:
- **Function syntax:** `{{ is_email(string="...") }}` or `{% if is_email(string=var) %}`
- **"Is" syntax:** `{% if var is email %}` (more readable for conditionals)

#### `is_email(string)` / `{% if x is email %}`

Validate if a string is a valid email address format.

**Function Arguments:**
- `string` (required) - String to validate

**Returns:** Boolean (`true` if valid email, `false` otherwise)

**Examples:**
```jinja
{# Function syntax #}
Email: user@example.com
Valid: {{ is_email(string="user@example.com") }}
{# Output: Valid: true #}

{# "Is" syntax (preferred for conditionals) #}
{% if user_email is email %}
  <p>Valid email address!</p>
{% else %}
  <p>Please enter a valid email.</p>
{% endif %}

{# Negation with "is not" #}
{% if input is not email %}
  <p>Invalid email format.</p>
{% endif %}
```

#### `is_url(string)` / `{% if x is url %}`

Validate if a string is a valid URL (supports http, https, ftp, file schemes).

**Function Arguments:**
- `string` (required) - String to validate

**Returns:** Boolean (`true` if valid URL, `false` otherwise)

**Examples:**
```jinja
{# Function syntax #}
URL: https://example.com/path
Valid: {{ is_url(string="https://example.com/path") }}
{# Output: Valid: true #}

{# "Is" syntax #}
{% if api_endpoint is url %}
  <a href="{{ api_endpoint }}">API Docs</a>
{% endif %}
```

#### `is_ip(string)` / `{% if x is ip %}`

Validate if a string is a valid IP address (IPv4 or IPv6).

**Function Arguments:**
- `string` (required) - String to validate

**Returns:** Boolean (`true` if valid IP, `false` otherwise)

**Examples:**
```jinja
{# Function syntax #}
IPv4: 192.168.1.1
Valid: {{ is_ip(string="192.168.1.1") }}
{# Output: Valid: true #}

IPv6: 2001:db8::1
Valid: {{ is_ip(string="2001:db8::1") }}
{# Output: Valid: true #}

{# "Is" syntax #}
{% if server_address is ip %}
  server: {{ server_address }}
{% else %}
  # Using hostname, resolve to IP
  server: {{ resolve_dns(hostname=server_address) }}
{% endif %}
```

#### `is_uuid(string)` / `{% if x is uuid %}`

Validate if a string is a valid UUID format.

**Function Arguments:**
- `string` (required) - String to validate

**Returns:** Boolean (`true` if valid UUID, `false` otherwise)

**Examples:**
```jinja
{# Function syntax #}
UUID: 550e8400-e29b-41d4-a716-446655440000
Valid: {{ is_uuid(string="550e8400-e29b-41d4-a716-446655440000") }}
{# Output: Valid: true #}

{# "Is" syntax #}
{% if request_id is uuid %}
  X-Request-ID: {{ request_id }}
{% else %}
  X-Request-ID: {{ uuid() }}
{% endif %}
```

#### `matches_regex(pattern, string)`

Check if a string matches a regular expression pattern.

**Arguments:**
- `pattern` (required) - Regular expression pattern
- `string` (required) - String to match against

**Returns:** Boolean (`true` if matches, `false` otherwise)

**Examples:**
```
{# Validate alphanumeric #}
{% if matches_regex(pattern="^[A-Za-z0-9]+$", string="Test123") %}
  Valid alphanumeric string
{% endif %}

{# Validate phone number format #}
{% set phone = get_env(name="PHONE", default="") %}
{% if matches_regex(pattern="^\\d{3}-\\d{3}-\\d{4}$", string=phone) %}
  Phone number format: XXX-XXX-XXXX
{% endif %}

{# Check for specific pattern #}
{% if matches_regex(pattern="^prod-", string="prod-server-01") %}
  This is a production server
{% endif %}
```

**Practical Example - Configuration Validation:**
```
# Configuration Validation Report

{% set email = get_env(name="ADMIN_EMAIL", default="") %}
Admin Email: {{ email }}
{% if is_email(string=email) %}
✓ Valid email format
{% else %}
✗ Invalid email format
{% endif %}

{% set api_url = get_env(name="API_URL", default="") %}
API URL: {{ api_url }}
{% if is_url(string=api_url) %}
✓ Valid URL format
{% else %}
✗ Invalid URL format
{% endif %}

{% set server_ip = get_env(name="SERVER_IP", default="") %}
Server IP: {{ server_ip %}
{% if is_ip(string=server_ip) %}
✓ Valid IP address
{% else %}
✗ Invalid IP address
{% endif %}

{% set correlation_id = get_env(name="CORRELATION_ID", default="") %}
Correlation ID: {{ correlation_id }}
{% if is_uuid(string=correlation_id) %}
✓ Valid UUID format
{% else %}
✗ Invalid UUID format
{% endif %}
```

