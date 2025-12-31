# tmpltool

[![CI](https://github.com/bordeux/tmpltool/actions/workflows/ci.yml/badge.svg)](https://github.com/bordeux/tmpltool/actions/workflows/ci.yml)
[![Release](https://github.com/bordeux/tmpltool/actions/workflows/release.yml/badge.svg)](https://github.com/bordeux/tmpltool/actions/workflows/release.yml)
[![codecov](https://codecov.io/gh/bordeux/tmpltool/branch/master/graph/badge.svg)](https://codecov.io/gh/bordeux/tmpltool)
[![GitHub release](https://img.shields.io/github/v/release/bordeux/tmpltool)](https://github.com/bordeux/tmpltool/releases)
[![Docker](https://img.shields.io/badge/docker-ghcr.io-blue)](https://github.com/bordeux/tmpltool/pkgs/container/tmpltool)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A fast and simple command-line template rendering tool using [MiniJinja](https://github.com/mitsuhiko/minijinja) templates with environment variables.

## Table of Contents

- [Quick Start](#quick-start)
- [Features](#features)
- [Installation](#installation)
- [CLI Reference](#cli-reference)
- [Basic Usage](#basic-usage)
- [Template Syntax](#template-syntax)
- [Function Reference](#function-reference)
  - [Environment Variables](#environment-variables)
  - [Hash & Crypto Functions](#hash--crypto-functions)
  - [Encoding & Security Functions](#encoding--security-functions)
  - [Date/Time Functions](#datetime-functions)
  - [Command Execution Functions](#command-execution-functions)
  - [Filesystem Functions](#filesystem-functions)
  - [Path Manipulation Functions](#path-manipulation-functions)
  - [Data Parsing Functions](#data-parsing-functions)
  - [Data Serialization Functions](#data-serialization-functions)
  - [Validation Functions](#validation-functions)
  - [Debugging & Development Functions](#debugging--development-functions)
- [Advanced Examples](#advanced-examples)
- [Error Handling](#error-handling)
- [Development](#development)
- [CI/CD](#cicd)
- [Contributing](#contributing)
- [License](#license)

## Quick Start

Get started in 30 seconds:

```bash
# Download for your platform (or use Docker)
docker pull ghcr.io/bordeux/tmpltool:latest

# Create a simple template
cat > greeting.tmpl << 'EOF'
Hello {{ get_env(name="USER", default="World") }}!
EOF

# Render it
docker run --rm -v $(pwd):/workspace -w /workspace ghcr.io/bordeux/tmpltool:latest greeting.tmpl
# Output: Hello World!

# Or with your own name
docker run --rm -e USER=Alice -v $(pwd):/workspace -w /workspace ghcr.io/bordeux/tmpltool:latest greeting.tmpl
# Output: Hello Alice!
```

**Without Docker:**
```bash
# Install binary from releases
# See Installation section below

# Create and render template
echo 'Hello {{ get_env(name="USER", default="World") }}!' > greeting.tmpl
tmpltool greeting.tmpl
```

## Features

- **Environment Variables**: Access env vars with `get_env()` and filter with `filter_env()`
- **Hash & Crypto**: MD5, SHA1, SHA256, SHA512, UUID generation, random strings
- **Encoding & Security**: Base64, hex, bcrypt, HMAC, HTML/XML/shell escaping, secure random strings
- **Filesystem**: Read files, check existence, list directories, glob patterns, file info, path manipulation
- **Data Parsing**: Parse and read JSON, YAML, TOML files
- **Data Serialization**: Convert objects to JSON, YAML, TOML strings with pretty-printing options
- **Validation**: Validate emails, URLs, IPs, UUIDs, regex matching
- **System & Network**: Get hostname, username, directories, IP addresses, DNS resolution, port availability
- **Debugging & Development**: Debug output, type checking, assertions, warnings, error handling
- **String Filters**: 12+ filters for case conversion, indentation, padding, quoting, and more
- **Security**: Built-in protections with optional `--trust` mode
- **Flexible I/O**: File or stdin input, file or stdout output
- **Full Jinja2 Syntax**: Conditionals, loops, filters, and more
- **Single Binary**: No runtime dependencies
- **Docker Support**: Multi-arch images available

## Installation

### From GitHub Releases

Download pre-built binaries for your platform from the [releases page](https://github.com/bordeux/tmpltool/releases):

- **Linux**: `tmpltool-linux-x86_64`, `tmpltool-linux-x86_64-musl` (static), `tmpltool-linux-aarch64` (ARM64)
- **macOS**: `tmpltool-macos-x86_64` (Intel), `tmpltool-macos-aarch64` (Apple Silicon)
- **Windows**: `tmpltool-windows-x86_64.exe`

Extract and place in your PATH:

```bash
# Linux/macOS example
tar -xzf tmpltool-linux-x86_64.tar.gz
sudo mv tmpltool /usr/local/bin/
chmod +x /usr/local/bin/tmpltool
```

### Using Docker

Pull from GitHub Container Registry:

```bash
docker pull ghcr.io/bordeux/tmpltool:latest
```

Create a shell alias for convenience:

```bash
alias tmpltool='docker run --rm -v $(pwd):/workspace -w /workspace ghcr.io/bordeux/tmpltool:latest'
```

### From Source

```bash
cargo install --path .
```

Or build manually:

```bash
cargo build --release
# Binary will be at: ./target/release/tmpltool
```

## CLI Reference

### Syntax

```bash
tmpltool [TEMPLATE] [OPTIONS]
cat template.txt | tmpltool [OPTIONS]
```

### Arguments

- `[TEMPLATE]` - Path to template file (optional, reads from stdin if omitted)

### Options

- `-o, --output <FILE>` - Output file path (prints to stdout if not specified)
- `--trust` - Trust mode: Allow filesystem functions to access absolute paths and parent directories
  - **WARNING:** Only use with trusted templates. Disables security restrictions.
- `--validate <FORMAT>` - Validate output format (json, yaml, or toml)
  - Validates the rendered output conforms to the specified format
  - Exits with error code 1 if validation fails
  - No output on success, error message only on validation failure

### Input/Output Patterns

| Input  | Output | Command |
|--------|--------|---------|
| File   | stdout | `tmpltool template.txt` |
| File   | File   | `tmpltool template.txt -o output.txt` |
| stdin  | stdout | `cat template.txt \| tmpltool` |
| stdin  | File   | `cat template.txt \| tmpltool -o output.txt` |

### Examples

```bash
# File to stdout
tmpltool template.txt

# File to file
tmpltool template.txt -o output.txt

# Stdin to stdout (pipe)
echo "Hello {{ get_env(name=\"USER\") }}!" | tmpltool

# With environment variables
DB_HOST=postgres tmpltool config.tmpl -o config.txt

# Chaining with other tools
tmpltool config.json.tmpl | jq .
cat k8s-deployment.yaml.tmpl | tmpltool | kubectl apply -f -

# Trust mode for system files
tmpltool --trust system_info.tmpl  # Can read /etc/passwd, etc.

# Validate JSON output
tmpltool config.json.tmpl --validate json
# Exits with error if output is invalid JSON

# Validate YAML output
tmpltool k8s-deploy.yaml.tmpl --validate yaml -o deployment.yaml

# Validate TOML output
tmpltool Cargo.toml.tmpl --validate toml
```

## Basic Usage

### Simple Variable Substitution

**Template** (`greeting.tmpl`):
```
Hello {{ get_env(name="USER") }}!
Your home directory is: {{ get_env(name="HOME") }}
```

**Render:**
```bash
tmpltool greeting.tmpl
```

### Using Default Values

**Template** (`config.tmpl`):
```
Database: {{ get_env(name="DB_HOST", default="localhost") }}:{{ get_env(name="DB_PORT", default="5432") }}
Environment: {{ get_env(name="APP_ENV", default="development") }}
Debug: {{ get_env(name="DEBUG", default="false") }}
```

**Render with defaults:**
```bash
tmpltool config.tmpl
# Output:
# Database: localhost:5432
# Environment: development
# Debug: false
```

**Render with custom values:**
```bash
DB_HOST=postgres DB_PORT=5432 APP_ENV=production tmpltool config.tmpl
# Output:
# Database: postgres:5432
# Environment: production
# Debug: false
```

### Conditionals and Loops

**Template** (`status.tmpl`):
```
{% set debug = get_env(name="DEBUG", default="false") %}
{% if debug == "true" %}
DEBUG MODE ENABLED
Log level: verbose
{% else %}
Production mode
Log level: error
{% endif %}

{% set items_str = get_env(name="ITEMS", default="apple,banana,orange") %}
{% set items = items_str | split(pat=",") %}
Items:
{% for item in items %}
  - {{ item }}
{% endfor %}
```

**Render:**
```bash
DEBUG=true ITEMS="apple,banana,orange,grape" tmpltool status.tmpl
```

### Filtering Environment Variables

**Template** (`server-vars.tmpl`):
```
Server Configuration:
{% for var in filter_env(pattern="SERVER_*") %}
  {{ var.key }}={{ var.value }}
{% endfor %}
```

**Render:**
```bash
SERVER_HOST=localhost SERVER_PORT=8080 SERVER_NAME=myapp tmpltool server-vars.tmpl
# Output:
# Server Configuration:
#   SERVER_HOST=localhost
#   SERVER_NAME=myapp
#   SERVER_PORT=8080
```

### Working with Files

**Template** (`build-report.tmpl`):
```
# Build Report

{% if file_exists(path="README.md") %}
✓ README.md found ({{ file_size(path="README.md") | filesizeformat }})
{% else %}
✗ README.md missing
{% endif %}

Source files:
{% for file in glob(pattern="src/**/*.rs") %}
  - {{ file }} ({{ file_size(path=file) | filesizeformat }})
{% endfor %}
```

**Render:**
```bash
tmpltool build-report.tmpl
```

## Template Syntax

tmpltool uses the [MiniJinja](https://github.com/mitsuhiko/minijinja) template engine, which is compatible with Python's Jinja2. For complete documentation, visit: https://docs.rs/minijinja/

### Variables

```
{{ variable_name }}
```

**Note:** Environment variables are NOT automatically available. Use the `get_env()` function to access them.

### Conditionals

```
{% if CONDITION %}
  ...
{% elif OTHER_CONDITION %}
  ...
{% else %}
  ...
{% endif %}
```

**Important:** `get_env()` cannot be used directly in `{% if %}` conditions. Use `{% set %}` to assign to a variable first:

```
{% set debug = get_env(name="DEBUG", default="false") %}
{% if debug == "true" %}
  Debug mode enabled
{% endif %}
```

### Loops

```
{% for item in items %}
  {{ item }}
{% endfor %}
```

Access loop metadata:
```
{% for item in items %}
  {{ loop.index }}: {{ item }}
  {% if loop.first %}(first){% endif %}
  {% if loop.last %}(last){% endif %}
{% endfor %}
```

### Filters

```
{{ variable | filter_name }}
{{ variable | filter_name(arg=value) }}
```

**Built-in MiniJinja filters:**
- `upper`, `lower`, `title` - Case conversion
- `trim`, `truncate` - String operations
- `date(format="%Y-%m-%d")` - Date formatting
- `split(pat=",")` - Split string into array
- `length` - Get array/string length

**Custom string manipulation filters:**
- `slugify` - Convert to URL-friendly slug (e.g., "Hello World" → "hello-world")
- `indent(spaces=4)` - Indent text by N spaces (useful for YAML/configs)
- `dedent` - Remove common leading whitespace
- `quote(style="double")` - Quote string (single/double/backtick)
- `escape_quotes` - Escape quotes in string
- `to_snake_case` - Convert to snake_case (e.g., "HelloWorld" → "hello_world")
- `to_camel_case` - Convert to camelCase (e.g., "hello_world" → "helloWorld")
- `to_pascal_case` - Convert to PascalCase (e.g., "hello_world" → "HelloWorld")
- `to_kebab_case` - Convert to kebab-case (e.g., "HelloWorld" → "hello-world")
- `pad_left(length, char=" ")` - Pad string on left
- `pad_right(length, char=" ")` - Pad string on right
- `repeat(count)` - Repeat string N times
- `reverse` - Reverse string

**Formatting filters:**
- `urlencode` - URL encoding
- `filesizeformat` - Format bytes (e.g., "1.5 KB")

**Examples:**
```
{# Case conversion #}
{{ "hello_world" | to_camel_case }}  {# Output: helloWorld #}
{{ "HelloWorld" | to_snake_case }}   {# Output: hello_world #}

{# Indentation for configs #}
{{ "host: localhost\nport: 8080" | indent(2) }}

{# Padding for alignment #}
{{ "1" | pad_left(4, "0") }}  {# Output: 0001 #}

{# Creating separators #}
{{ "=" | repeat(40) }}  {# Output: ======================================== #}

{# Chaining filters #}
{{ "hello_world" | to_pascal_case | reverse }}  {# Output: dlroWolleH #}
```

### Comments

```
{# This is a comment #}
```

### Setting Variables

```
{% set variable_name = value %}
{% set name = get_env(name="USER", default="guest") %}
```

## Function Reference

### Environment Variables

#### `get_env(name, default)`

Access environment variables with optional default values.

**Arguments:**
- `name` (required) - Environment variable name
- `default` (optional) - Fallback value if variable doesn't exist

**Returns:** String value of the environment variable or default

**Examples:**
```
{# With default (recommended) #}
port = {{ get_env(name="PORT", default="8080") }}

{# Without default (will error if variable doesn't exist) #}
api_key = {{ get_env(name="API_KEY") }}

{# Use in variables (requires {% set %}) #}
{% set debug = get_env(name="DEBUG", default="false") %}
{% if debug == "true" %}
  Debug mode enabled
{% endif %}
```

#### `filter_env(pattern)`

Filter environment variables by glob pattern.

**Arguments:**
- `pattern` (required) - Glob pattern (`*` matches any characters, `?` matches one character)

**Returns:** Array of objects with `key` and `value` fields, sorted alphabetically

**Examples:**
```
{# Match all SERVER_* variables #}
{% for var in filter_env(pattern="SERVER_*") %}
export {{ var.key }}="{{ var.value }}"
{% endfor %}

{# Match any variable ending with _PORT #}
{% for var in filter_env(pattern="*_PORT") %}
{{ var.key }}: {{ var.value }}
{% endfor %}

{# Count matching variables #}
{% set db_vars = filter_env(pattern="DATABASE_*") %}
Found {{ db_vars | length }} database variables
```

### Hash & Crypto Functions

#### `md5(string)`

Calculate MD5 hash of a string.

```
Checksum: {{ md5(string="hello world") }}
{# Output: 5eb63bbbe01eeed093cb22bb8f5acdc3 #}
```

#### `sha1(string)`

Calculate SHA1 hash of a string.

```
Hash: {{ sha1(string="tmpltool") }}
{# Output: c054a2a60ca2fe935ea1056bd90386194116f14f #}
```

#### `sha256(string)`

Calculate SHA256 hash (recommended for password hashing).

```
{% set password = get_env(name="PASSWORD", default="secret") %}
Password hash: {{ sha256(string=password) }}
```

#### `sha512(string)`

Calculate SHA512 hash (most secure).

```
Secure hash: {{ sha512(string="secure-data") }}
```

**Important:** These hash functions are for checksums and general-purpose hashing. For production password storage, use dedicated password hashing libraries with salt and proper key derivation functions (bcrypt, argon2, etc.).

#### `uuid()`

Generate a random UUID v4 (Universally Unique Identifier).

```
Request ID: {{ uuid() }}
Session ID: {{ uuid() }}
{# Each call generates a unique identifier #}
```

#### `random_string(length, charset)`

Generate a random string with customizable length and character set.

**Arguments:**
- `length` (required) - Length of string (1-10000)
- `charset` (optional) - Character set preset or custom string (default: `alphanumeric`)

**Character Set Presets:**
- `alphanumeric` - Letters and digits (a-z, A-Z, 0-9) - **default**
- `alphabetic` or `alpha` - Letters only (a-z, A-Z)
- `lowercase` or `lower` - Lowercase letters (a-z)
- `uppercase` or `upper` - Uppercase letters (A-Z)
- `numeric` or `digits` - Digits only (0-9)
- `hex` or `hexadecimal` - Hexadecimal (0-9, a-f)
- `hex_upper` - Hexadecimal uppercase (0-9, A-F)
- Custom string - Any custom character set (e.g., `"abc123"`)

**Examples:**
```
{# Alphanumeric (default) #}
API Key: {{ random_string(length=32) }}

{# Lowercase only #}
Username: user_{{ random_string(length=8, charset="lowercase") }}

{# Numeric PIN #}
PIN: {{ random_string(length=4, charset="numeric") }}

{# Hexadecimal token #}
Token: {{ random_string(length=16, charset="hex") }}

{# Custom charset #}
Password: {{ random_string(length=12, charset="abc123") }}
```

**Practical Example:**
```yaml
application:
  instance_id: {{ uuid() }}
  secret_key: {{ random_string(length=64) }}
  api_token: {{ random_string(length=32, charset="hex") }}
  csrf_token: {{ random_string(length=40, charset="hex") }}

security:
  password_hash: {{ sha256(string=get_env(name="PASSWORD")) }}
```

### Encoding & Security Functions

Functions for encoding, decoding, password hashing, and escaping data for various contexts.

#### `base64_encode(string)`

Encode a string to Base64 format.

**Arguments:**
- `string` (required) - String to encode

**Returns:** Base64-encoded string

**Examples:**
```jinja
{{ base64_encode(string="Hello World") }}
{# Output: SGVsbG8gV29ybGQ= #}

{# Basic Authentication header #}
{% set credentials = "admin:password123" %}
Authorization: Basic {{ base64_encode(string=credentials) }}
```

#### `base64_decode(string)`

Decode a Base64-encoded string.

**Arguments:**
- `string` (required) - Base64 string to decode

**Returns:** Decoded string

**Examples:**
```jinja
{{ base64_decode(string="SGVsbG8gV29ybGQ=") }}
{# Output: Hello World #}
```

#### `hex_encode(string)`

Encode a string to hexadecimal format.

**Arguments:**
- `string` (required) - String to encode

**Returns:** Hexadecimal string (lowercase)

**Examples:**
```jinja
{{ hex_encode(string="Hello") }}
{# Output: 48656c6c6f #}
```

#### `hex_decode(string)`

Decode a hexadecimal-encoded string.

**Arguments:**
- `string` (required) - Hexadecimal string to decode

**Returns:** Decoded string

**Examples:**
```jinja
{{ hex_decode(string="48656c6c6f") }}
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

#### `escape_html(string)`

Escape HTML entities to prevent XSS attacks.

**Arguments:**
- `string` (required) - String to escape

**Returns:** HTML-escaped string

**Examples:**
```jinja
{# Escape user input for HTML #}
{% set user_input = '<script>alert("XSS")</script>' %}
<div>{{ escape_html(string=user_input) }}</div>
{# Output: &lt;script&gt;alert(&quot;XSS&quot;)&lt;/script&gt; #}

{# Safe HTML output #}
<p>User comment: {{ escape_html(string=get_env(name="USER_COMMENT", default="")) }}</p>
```

#### `escape_xml(string)`

Escape XML entities.

**Arguments:**
- `string` (required) - String to escape

**Returns:** XML-escaped string

**Examples:**
```jinja
{# Escape for XML #}
{% set content = '<tag attr="value">text & more</tag>' %}
<data>{{ escape_xml(string=content) }}</data>
{# Output: &lt;tag attr=&quot;value&quot;&gt;text &amp; more&lt;/tag&gt; #}
```

#### `escape_shell(string)`

Escape string for safe use in shell commands.

**Arguments:**
- `string` (required) - String to escape

**Returns:** Shell-escaped string (single-quoted)

**Examples:**
```jinja
{# Safe shell argument #}
{% set filename = "my file with spaces.txt" %}
Command: cat {{ escape_shell(string=filename) }}
{# Output: cat 'my file with spaces.txt' #}

{# Escape special characters #}
{% set message = "it's working!" %}
echo {{ escape_shell(string=message) }}
{# Output: echo 'it'\''s working!' #}
```

**Security Warning:** While `escape_shell` helps prevent injection, the safest approach is to avoid dynamic shell commands entirely when possible. Use `exec()` function only with trusted, hardcoded commands.

### Date/Time Functions

Work with dates, times, and timestamps. All functions use Unix timestamps (seconds since epoch) for consistent timezone-independent representation.

#### `now()`

Get the current timestamp in ISO 8601 format.

**Returns:** Current timestamp as ISO 8601 string (e.g., `"2024-12-31T12:34:56.789+00:00"`)

**Examples:**
```
Current time: {{ now() }}
{# Output: 2024-12-31T12:34:56.789+00:00 #}

{# Use with date filter for custom formatting #}
{{ now() | date(format="%Y-%m-%d %H:%M:%S") }}
```

#### `format_date(timestamp, format)`

Format a Unix timestamp with a custom format string.

**Arguments:**
- `timestamp` (required) - Unix timestamp in seconds
- `format` (optional) - Format string (default: `"%Y-%m-%d %H:%M:%S"`)

**Returns:** Formatted date string

**Common Format Specifiers:**
- `%Y` - Year (4 digits), e.g., 2024
- `%m` - Month (01-12)
- `%d` - Day (01-31)
- `%H` - Hour 24h (00-23)
- `%I` - Hour 12h (01-12)
- `%M` - Minute (00-59)
- `%S` - Second (00-59)
- `%p` - AM/PM
- `%A` - Weekday (full), e.g., Monday
- `%B` - Month (full), e.g., January

[Full format reference](https://docs.rs/chrono/latest/chrono/format/strftime/index.html)

**Examples:**
```
{% set ts = 1704067200 %}
ISO date: {{ format_date(timestamp=ts, format="%Y-%m-%d") }}
{# Output: 2024-01-01 #}

US format: {{ format_date(timestamp=ts, format="%m/%d/%Y") }}
{# Output: 01/01/2024 #}

Full: {{ format_date(timestamp=ts, format="%B %d, %Y at %I:%M %p") }}
{# Output: January 01, 2024 at 12:00 AM #}
```

#### `parse_date(string, format)`

Parse a date string into a Unix timestamp.

**Arguments:**
- `string` (required) - Date string to parse
- `format` (required) - Format string matching the input

**Returns:** Unix timestamp (integer)

**Examples:**
```
{% set ts = parse_date(string="2024-01-01 12:00:00", format="%Y-%m-%d %H:%M:%S") %}
Timestamp: {{ ts }}
{# Output: 1704110400 #}

{# Date-only formats (time set to midnight) #}
{% set ts = parse_date(string="12/25/2024", format="%m/%d/%Y") %}
{{ format_date(timestamp=ts, format="%Y-%m-%d") }}
{# Output: 2024-12-25 #}
```

#### `date_add(timestamp, days)`

Add or subtract days from a Unix timestamp.

**Arguments:**
- `timestamp` (required) - Unix timestamp in seconds
- `days` (required) - Number of days to add (can be negative)

**Returns:** New Unix timestamp

**Examples:**
```
{% set today = parse_date(string="2024-01-01", format="%Y-%m-%d") %}

{# Add days #}
Next week: {{ format_date(timestamp=date_add(timestamp=today, days=7), format="%Y-%m-%d") }}
{# Output: 2024-01-08 #}

{# Subtract days #}
Last week: {{ format_date(timestamp=date_add(timestamp=today, days=-7), format="%Y-%m-%d") }}
{# Output: 2023-12-25 #}
```

#### `date_diff(timestamp1, timestamp2)`

Calculate the difference in days between two timestamps.

**Arguments:**
- `timestamp1` (required) - First Unix timestamp
- `timestamp2` (required) - Second Unix timestamp

**Returns:** Difference in days (timestamp1 - timestamp2)

**Examples:**
```
{% set start = parse_date(string="2024-01-01", format="%Y-%m-%d") %}
{% set end = parse_date(string="2024-01-31", format="%Y-%m-%d") %}

Days between: {{ date_diff(timestamp1=end, timestamp2=start) }}
{# Output: 30 #}
```

#### `get_year(timestamp)`, `get_month(timestamp)`, `get_day(timestamp)`

Extract date components from a Unix timestamp.

**Arguments:**
- `timestamp` (required) - Unix timestamp in seconds

**Returns:** Integer component value (year: 4-digit, month: 1-12, day: 1-31)

**Examples:**
```
{% set ts = parse_date(string="2024-12-25", format="%Y-%m-%d") %}
Year: {{ get_year(timestamp=ts) }}    {# Output: 2024 #}
Month: {{ get_month(timestamp=ts) }}  {# Output: 12 #}
Day: {{ get_day(timestamp=ts) }}      {# Output: 25 #}
```

#### `get_hour(timestamp)`, `get_minute(timestamp)`

Extract time components from a Unix timestamp.

**Arguments:**
- `timestamp` (required) - Unix timestamp in seconds

**Returns:** Integer component value (hour: 0-23, minute: 0-59)

**Examples:**
```
{% set ts = parse_date(string="2024-01-01 15:30:00", format="%Y-%m-%d %H:%M:%S") %}
Hour: {{ get_hour(timestamp=ts) }}      {# Output: 15 #}
Minute: {{ get_minute(timestamp=ts) }}  {# Output: 30 #}
```

#### `timezone_convert(timestamp, from_tz, to_tz)`

Convert a timestamp between timezones.

**Arguments:**
- `timestamp` (required) - Unix timestamp in seconds
- `from_tz` (required) - Source timezone (e.g., "UTC", "America/New_York")
- `to_tz` (required) - Target timezone (e.g., "Europe/London", "Asia/Tokyo")

**Returns:** Unix timestamp (note: Unix timestamps are timezone-independent)

**Note:** Unix timestamps are always UTC-relative. This function is useful when formatting times in different timezones.

**Examples:**
```
{% set utc_ts = 1704067200 %}
{{ timezone_convert(timestamp=utc_ts, from_tz="UTC", to_tz="America/New_York") }}
```

#### `is_leap_year(year)`

Check if a year is a leap year.

**Arguments:**
- `year` (required) - Year to check (4-digit integer)

**Returns:** Boolean (true if leap year, false otherwise)

**Examples:**
```
{% if is_leap_year(year=2024) %}
2024 is a leap year
{% endif %}

{% set years = [2020, 2021, 2022, 2023, 2024] %}
{% for year in years %}
{{ year }}: {% if is_leap_year(year=year) %}Leap{% else %}Regular{% endif %}
{% endfor %}
```

**Practical Example - Certificate Expiration:**
```yaml
{% set cert_expiry = parse_date(string="2025-06-15", format="%Y-%m-%d") %}
{% set today = parse_date(string="2024-12-31", format="%Y-%m-%d") %}
{% set days_until_expiry = date_diff(timestamp1=cert_expiry, timestamp2=today) %}

certificates:
  ssl_cert:
    expires: {{ format_date(timestamp=cert_expiry, format="%B %d, %Y") }}
    days_remaining: {{ days_until_expiry }}
    {% if days_until_expiry < 30 %}
    warning: "Certificate expires in {{ days_until_expiry }} days - RENEW IMMEDIATELY"
    priority: critical
    {% elif days_until_expiry < 90 %}
    warning: "Certificate expires in {{ days_until_expiry }} days - schedule renewal"
    priority: high
    {% else %}
    status: valid
    priority: normal
    {% endif %}
```

**Practical Example - Backup Schedule:**
```bash
#!/bin/bash
{% set backup_ts = parse_date(string="2024-01-15 02:00:00", format="%Y-%m-%d %H:%M:%S") %}
# Weekly backups
{% for week in range(0, 4) %}
WEEKLY_BACKUP_{{ week + 1 }}="{{ format_date(timestamp=date_add(timestamp=backup_ts, days=week * 7), format="%Y-%m-%d") }}"
{% endfor %}

# Retention: Keep backups for 30 days
{% set retention_cutoff = date_add(timestamp=backup_ts, days=-30) %}
DELETE_BEFORE="{{ format_date(timestamp=retention_cutoff, format="%Y-%m-%d") }}"
```

### Command Execution Functions

Execute external commands from templates. These functions provide the ability to run shell commands and capture their output.

**SECURITY WARNING:** Command execution is a powerful feature that can pose security risks. These functions are **only available in trust mode** (`--trust` flag).

#### `exec(command, timeout)`

Execute a command and return stdout as a string. Throws an error if the command fails (non-zero exit code).

**Arguments:**
- `command` (required) - Command to execute (executed via system shell)
- `timeout` (optional) - Timeout in seconds (default: 30, max: 300)

**Returns:** Standard output as string

**Security:** Only available with `--trust` flag

**Examples:**
```jinja
{# Simple usage - get output directly #}
Hostname: {{ exec(command="hostname") }}

{# Use with filters #}
System: {{ exec(command="uname -s") | trim }}

{# Use in variable #}
{% set git_hash = exec(command="git rev-parse --short HEAD 2>/dev/null || echo 'unknown'") %}
Commit: {{ git_hash | trim }}

{# This will throw an error if command fails #}
{{ exec(command="ls /nonexistent") }}  {# Error! #}
```

#### `exec_raw(command, timeout)`

Execute a command and return a detailed result object. Never throws based on exit code - you control all error handling.

**Arguments:**
- `command` (required) - Command to execute (executed via system shell)
- `timeout` (optional) - Timeout in seconds (default: 30, max: 300)

**Returns:** Object with fields:
- `exit_code` - Exit code (integer, 0 = success)
- `stdout` - Standard output (string)
- `stderr` - Standard error (string)
- `success` - Boolean (true if exit_code == 0)

**Security:** Only available with `--trust` flag

**Examples:**
```jinja
{# Full control over result #}
{% set result = exec_raw(command="ls -la /tmp") %}
{% if result.success %}
Files:
{{ result.stdout }}
{% else %}
Error (exit {{ result.exit_code }}): {{ result.stderr }}
{% endif %}

{# Handle expected non-zero exit (e.g., grep) #}
{% set result = exec_raw(command="grep foo /etc/hosts") %}
{% if result.exit_code == 0 %}
Found: {{ result.stdout }}
{% elif result.exit_code == 1 %}
Not found
{% else %}
Error: {{ result.stderr }}
{% endif %}

{# Check if a tool is available #}
{% set docker = exec_raw(command="which docker") %}
{% if docker.success %}
Docker available at: {{ docker.stdout | trim }}
{% else %}
Docker not installed
{% endif %}
```

**Practical Example - Build Information:**
```yaml
build:
  commit: {{ exec(command="git rev-parse --short HEAD 2>/dev/null || echo 'dev'") | trim }}
  branch: {{ exec(command="git branch --show-current 2>/dev/null || echo 'unknown'") | trim }}
  date: {{ exec(command="date -u +%Y-%m-%dT%H:%M:%SZ") | trim }}
  user: {{ exec(command="whoami") | trim }}
```

**Practical Example - Conditional Configuration:**
```yaml
{% set node_check = exec_raw(command="which node") %}
{% set docker_check = exec_raw(command="which docker") %}

services:
  node_enabled: {{ node_check.success | lower }}
  {% if node_check.success %}
  node_version: {{ exec(command="node --version") | trim }}
  {% endif %}

  docker_enabled: {{ docker_check.success | lower }}
  {% if docker_check.success %}
  docker_path: {{ docker_check.stdout | trim }}
  {% endif %}
```

**Practical Example - Dynamic Worker Configuration:**
```yaml
{% set cpu_count = exec(command="nproc 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo '2'") | trim | int %}

workers:
  count: {{ cpu_count * 2 }}
  per_worker_connections: 1000
  total_capacity: {{ cpu_count * 2000 }}
```

**Security Considerations:**

⚠️ **Command Injection Risk:** Never use untrusted input in commands
```jinja
{# ❌ DANGEROUS - DO NOT DO THIS #}
{% set user_input = get_env(name="USER_INPUT") %}
{{ exec(command="echo " ~ user_input) }}  {# COMMAND INJECTION! #}

{# ✓ SAFE - Use only hardcoded, trusted commands #}
{{ exec(command="hostname") }}
{{ exec(command="date") }}
```

**Notes:**
- Commands are executed via the system shell (`sh -c` on Unix, `cmd /C` on Windows)
- All shell features work: pipes (`|`), redirections (`>`, `2>&1`), command substitution (`$()`)
- Use `exec()` for simple cases where you just want output
- Use `exec_raw()` when you need to check exit codes or handle errors
- Always use `2>/dev/null || echo 'fallback'` patterns for robust error handling
- Keep commands fast - they block template rendering

### Filesystem Functions

All filesystem functions enforce security restrictions to prevent unauthorized access. Only relative paths within the current working directory are allowed unless `--trust` mode is enabled.

**Security Restrictions:**
- ✗ No absolute paths (e.g., `/etc/passwd`)
- ✗ No parent directory traversal (e.g., `../../secret.txt`)
- ✓ Only relative paths within current directory

**Trust Mode:** Use `--trust` flag to bypass these restrictions for trusted templates.

```bash
tmpltool --trust template.tmpl  # Can access any file
```

#### `read_file(path)`

Read the content of a file into the template.

**Arguments:**
- `path` (required) - Relative path to the file

**Returns:** String containing file content

**Examples:**
```
{# Read a configuration file #}
{% set config = read_file(path="config.txt") %}
{{ config }}

{# Read and include LICENSE #}
License:
{{ read_file(path="LICENSE") }}

{# Use with filters #}
First 100 chars: {{ read_file(path="README.md") | truncate(length=100) }}
```

#### `file_exists(path)`

Check if a file exists at the specified path.

**Arguments:**
- `path` (required) - Relative path to check

**Returns:** Boolean (`true` if exists, `false` otherwise)

**Examples:**
```
{# Conditional file inclusion #}
{% if file_exists(path="custom-config.txt") %}
Custom config found!
{{ read_file(path="custom-config.txt") }}
{% else %}
Using default configuration
{% endif %}

{# Check multiple files #}
{% set has_readme = file_exists(path="README.md") %}
{% set has_license = file_exists(path="LICENSE") %}
Documentation: {% if has_readme %}✓{% else %}✗{% endif %}
License: {% if has_license %}✓{% else %}✗{% endif %}
```

#### `list_dir(path)`

List all files and directories in a directory.

**Arguments:**
- `path` (required) - Relative path to the directory

**Returns:** Array of filenames (sorted alphabetically)

**Examples:**
```
{# List files in a directory #}
Files in data/:
{% for file in list_dir(path="data") %}
  - {{ file }}
{% endfor %}

{# Count files #}
{% set files = list_dir(path="templates") %}
Total templates: {{ files | length }}

{# Filter by extension #}
{% set all_files = list_dir(path="src") %}
Rust files:
{% for file in all_files %}
{% if file is ending_with(".rs") %}
  - {{ file }}
{% endif %}
{% endfor %}
```

#### `glob(pattern)`

List all files matching a glob pattern.

**Arguments:**
- `pattern` (required) - Glob pattern (`*` matches any characters, `?` matches one character, `**` matches any number of directories)

**Returns:** Array of file paths (sorted alphabetically)

**Examples:**
```
{# Find all text files #}
Text files:
{% for file in glob(pattern="*.txt") %}
  - {{ file }}
{% endfor %}

{# Find files in subdirectories #}
All Rust files:
{% for file in glob(pattern="src/**/*.rs") %}
  - {{ file }}
{% endfor %}

{# Match specific patterns #}
Config files:
{% for file in glob(pattern="config*.{json,yaml,toml}") %}
  - {{ file }}
{% endfor %}

{# Use in conditionals #}
{% set test_files = glob(pattern="tests/**/*.rs") %}
{% if test_files | length > 0 %}
Found {{ test_files | length }} test files
{% endif %}
```

#### `file_size(path)`

Get the size of a file in bytes.

**Arguments:**
- `path` (required) - Relative path to the file

**Returns:** File size as a number (in bytes)

**Examples:**
```
{# Get file size #}
README size: {{ file_size(path="README.md") }} bytes

{# Format with built-in filter #}
README size: {{ file_size(path="README.md") | filesizeformat }}

{# Compare file sizes #}
{% set size_a = file_size(path="file_a.txt") %}
{% set size_b = file_size(path="file_b.txt") %}
{% if size_a > size_b %}
file_a.txt is larger
{% else %}
file_b.txt is larger
{% endif %}

{# Calculate total size #}
{% set files = glob(pattern="data/*.json") %}
{% set total_size = 0 %}
{% for file in files %}
{% set total_size = total_size + file_size(path=file) %}
{% endfor %}
Total data size: {{ total_size | filesizeformat }}
```

#### `file_modified(path)`

Get the last modification time of a file as a Unix timestamp.

**Arguments:**
- `path` (required) - Relative path to the file

**Returns:** Unix timestamp (seconds since January 1, 1970)

**Examples:**
```
{# Get modification timestamp #}
Last modified: {{ file_modified(path="config.json") }}

{# Format with date filter #}
{% set timestamp = file_modified(path="README.md") %}
Last updated: {{ timestamp | date(format="%Y-%m-%d %H:%M:%S") }}

{# Check if file is recent #}
{% set mod_time = file_modified(path="cache.dat") %}
{% set now_time = now() %}
{% set age_seconds = now_time - mod_time %}
{% if age_seconds < 3600 %}
Cache is fresh (less than 1 hour old)
{% else %}
Cache is stale ({{ age_seconds / 3600 }} hours old)
{% endif %}
```

**Practical Example - Build Report:**
```
# Build Report
Generated: {{ now() | date(format="%Y-%m-%d %H:%M:%S") }}

## Source Files
{% set rs_files = glob(pattern="src/**/*.rs") %}
Total Rust files: {{ rs_files | length }}

{% for file in rs_files %}
- {{ file }}
  Size: {{ file_size(path=file) | filesizeformat }}
  Modified: {{ file_modified(path=file) | date(format="%Y-%m-%d") }}
{% endfor %}

## Configuration
{% if file_exists(path="Cargo.toml") %}
✓ Cargo.toml found ({{ file_size(path="Cargo.toml") }} bytes)
{% else %}
✗ Cargo.toml missing
{% endif %}

## Tests
{% set test_files = glob(pattern="tests/**/*.rs") %}
Test files: {{ test_files | length }}
```

### Path Manipulation Functions

Functions for manipulating file paths and checking filesystem metadata. These functions do not read file contents and work without security restrictions.

#### `basename(path)`

Extract the filename from a path.

**Arguments:**
- `path` (required) - File path

**Returns:** Filename (last component of the path)

**Examples:**
```jinja
{{ basename(path="/path/to/file.txt") }}
{# Output: file.txt #}

{{ basename(path="folder/document.pdf") }}
{# Output: document.pdf #}

{# Use with glob results #}
{% for file in glob(pattern="src/**/*.rs") %}
  - {{ basename(path=file) }}
{% endfor %}
```

#### `dirname(path)`

Extract the directory portion from a path.

**Arguments:**
- `path` (required) - File path

**Returns:** Directory path (all components except the last)

**Examples:**
```jinja
{{ dirname(path="/path/to/file.txt") }}
{# Output: /path/to #}

{{ dirname(path="folder/document.pdf") }}
{# Output: folder #}

{# Get parent directory #}
{% set file_path = "config/app/settings.json" %}
Config directory: {{ dirname(path=file_path) }}
{# Output: config/app #}
```

#### `file_extension(path)`

Extract the file extension from a path.

**Arguments:**
- `path` (required) - File path

**Returns:** File extension without the dot (empty string if no extension)

**Examples:**
```jinja
{{ file_extension(path="document.pdf") }}
{# Output: pdf #}

{{ file_extension(path="/path/to/file.tar.gz") }}
{# Output: gz #}

{{ file_extension(path="README") }}
{# Output: (empty) #}

{# Group files by extension #}
{% set files = glob(pattern="docs/*") %}
{% for file in files %}
  {% if file_extension(path=file) == "md" %}
    - Markdown: {{ file }}
  {% elif file_extension(path=file) == "pdf" %}
    - PDF: {{ file }}
  {% endif %}
{% endfor %}
```

#### `join_path(parts)`

Join multiple path components into a single path.

**Arguments:**
- `parts` (required) - Array of path components

**Returns:** Joined path string

**Examples:**
```jinja
{{ join_path(parts=["path", "to", "file.txt"]) }}
{# Output: path/to/file.txt #}

{{ join_path(parts=["/home", "user", "documents"]) }}
{# Output: /home/user/documents #}

{# Build dynamic paths #}
{% set base_dir = "config" %}
{% set env = get_env(name="APP_ENV", default="development") %}
{% set config_path = join_path(parts=[base_dir, env, "settings.json"]) %}
Config file: {{ config_path }}
{# Output: config/development/settings.json #}
```

#### `normalize_path(path)`

Normalize a path by resolving `.` (current directory) and `..` (parent directory) components.

**Arguments:**
- `path` (required) - Path to normalize

**Returns:** Normalized path string

**Examples:**
```jinja
{{ normalize_path(path="./foo/bar") }}
{# Output: foo/bar #}

{{ normalize_path(path="foo/../bar") }}
{# Output: bar #}

{{ normalize_path(path="a/b/c/../../d") }}
{# Output: a/d #}

{# Clean up path components #}
{% set messy_path = "./config/../data/./files.txt" %}
Clean path: {{ normalize_path(path=messy_path) }}
{# Output: data/files.txt #}
```

#### `is_file(path)`

Check if a path exists and is a file.

**Arguments:**
- `path` (required) - Path to check

**Returns:** Boolean (true if path exists and is a file)

**Examples:**
```jinja
{% if is_file(path="config.txt") %}
  Config file found!
{% else %}
  Config file missing
{% endif %}

{# Check before reading #}
{% if is_file(path="README.md") %}
  {{ read_file(path="README.md") }}
{% endif %}
```

#### `is_dir(path)`

Check if a path exists and is a directory.

**Arguments:**
- `path` (required) - Path to check

**Returns:** Boolean (true if path exists and is a directory)

**Examples:**
```jinja
{% if is_dir(path="src") %}
  Source directory exists
{% else %}
  Source directory not found
{% endif %}

{# Conditional directory operations #}
{% if is_dir(path="tests") %}
  {% set test_files = glob(pattern="tests/**/*.rs") %}
  Found {{ test_files | length }} test files
{% endif %}
```

#### `is_symlink(path)`

Check if a path is a symbolic link.

**Arguments:**
- `path` (required) - Path to check

**Returns:** Boolean (true if path is a symlink)

**Examples:**
```jinja
{% if is_symlink(path="current") %}
  'current' is a symbolic link
{% else %}
  'current' is not a symbolic link
{% endif %}
```

#### `read_lines(path, max_lines)`

Read lines from a file with flexible line selection.

**Arguments:**
- `path` (required) - Path to file
- `max_lines` (optional) - Number of lines to read (default: 10, max abs value: 10000)
  - **Positive number**: Read first N lines
  - **Negative number**: Read last N lines
  - **Zero**: Read entire file

**Returns:** Array of strings (lines without newline characters)

**Security:** Requires `--trust` flag for absolute paths or parent directory traversal

**Examples:**
```jinja
{# Read first 5 lines #}
{% set first_lines = read_lines(path="log.txt", max_lines=5) %}
Recent log entries:
{% for line in first_lines %}
  {{ loop.index }}: {{ line }}
{% endfor %}

{# Read last 5 lines #}
{% set last_lines = read_lines(path="log.txt", max_lines=-5) %}
Latest log entries:
{% for line in last_lines %}
  {{ line }}
{% endfor %}

{# Read entire file #}
{% set all_lines = read_lines(path="config.txt", max_lines=0) %}
Total lines: {{ all_lines | length }}

{# Preview file content #}
{% if is_file(path="README.md") %}
  README preview (first 3 lines):
  {% for line in read_lines(path="README.md", max_lines=3) %}
    {{ line }}
  {% endfor %}
{% endif %}

{# Process log file tail #}
{% set log_tail = read_lines(path="app.log", max_lines=-10) %}
{% for line in log_tail %}
  {% if "ERROR" in line %}
    ⚠️  {{ line }}
  {% endif %}
{% endfor %}
```

**Practical Example - Project Structure:**
```jinja
# Project Analysis

## Directory Structure
{% for item in ["src", "tests", "examples", "docs"] %}
  {% if is_dir(path=item) %}
    ✓ {{ item }}/ ({{ glob(pattern=item ~ "/**/*") | length }} files)
  {% else %}
    ✗ {{ item }}/ (missing)
  {% endif %}
{% endfor %}

## Configuration Files
{% for config_file in ["Cargo.toml", "package.json", ".gitignore"] %}
  {% if is_file(path=config_file) %}
    ✓ {{ config_file }}
    {% set lines = read_lines(path=config_file, max_lines=3) %}
    Preview: {{ lines[0] | truncate(length=50) }}
  {% else %}
    ✗ {{ config_file }} (not found)
  {% endif %}
{% endfor %}

## Source Files by Type
{% set all_files = glob(pattern="src/**/*") %}
{% for file in all_files %}
  {% set ext = file_extension(path=file) %}
  {% if ext == "rs" %}
    - Rust: {{ basename(path=file) }}
  {% elif ext == "md" %}
    - Markdown: {{ basename(path=file) }}
  {% endif %}
{% endfor %}
```

### Data Parsing Functions

Parse structured data formats (JSON, YAML, TOML) from strings or files. Useful for loading configuration files, processing API responses, or working with structured data.

**Security Note:** File-reading functions enforce the same security restrictions as other filesystem functions.

#### `parse_json(string)`

Parse a JSON string into an object.

**Arguments:**
- `string` (required) - JSON string to parse

**Returns:** Parsed JSON object

**Examples:**
```
{% set config = parse_json(string='{"name": "myapp", "port": 8080, "debug": true}') %}
Application: {{ config.name }}
Port: {{ config.port }}
Debug mode: {{ config.debug }}
```

#### `parse_yaml(string)`

Parse a YAML string into an object.

**Arguments:**
- `string` (required) - YAML string to parse

**Returns:** Parsed YAML object

**Examples:**
```
{% set data = parse_yaml(string="
name: myapp
settings:
  theme: dark
  notifications: true
") %}
App: {{ data.name }}
Theme: {{ data.settings.theme }}
```

#### `parse_toml(string)`

Parse a TOML string into an object.

**Arguments:**
- `string` (required) - TOML string to parse

**Returns:** Parsed TOML object

**Examples:**
```
{% set config = parse_toml(string='
[database]
host = "localhost"
port = 5432

[cache]
enabled = true
') %}
Database: {{ config.database.host }}:{{ config.database.port }}
Cache: {{ config.cache.enabled }}
```

#### `read_json_file(path)`

Read and parse a JSON file.

**Arguments:**
- `path` (required) - Relative path to JSON file

**Returns:** Parsed JSON object

**Example JSON file** (`config/settings.json`):
```json
{
  "app_name": "MyApp",
  "version": "1.0.0",
  "features": {
    "auth": true,
    "api": true
  }
}
```

**Template:**
```
{% set config = read_json_file(path="config/settings.json") %}
# {{ config.app_name }} v{{ config.version }}

Features:
{% if config.features.auth %}
- Authentication: Enabled
{% endif %}
{% if config.features.api %}
- API: Enabled
{% endif %}
```

#### `read_yaml_file(path)`

Read and parse a YAML file.

**Arguments:**
- `path` (required) - Relative path to YAML file

**Returns:** Parsed YAML object

**Example YAML file** (`config.yaml`):
```yaml
services:
  - name: web
    port: 8080
  - name: api
    port: 3000

environment: production
```

**Template:**
```
{% set config = read_yaml_file(path="config.yaml") %}
Environment: {{ config.environment }}

Services:
{% for service in config.services %}
  - {{ service.name }}: port {{ service.port }}
{% endfor %}
```

#### `read_toml_file(path)`

Read and parse a TOML file.

**Arguments:**
- `path` (required) - Relative path to TOML file

**Returns:** Parsed TOML object

**Example TOML file** (`Cargo.toml`):
```toml
[package]
name = "myapp"
version = "1.0.0"

[dependencies]
serde = "1.0"
tokio = "1.0"
```

**Template:**
```
{% set cargo = read_toml_file(path="Cargo.toml") %}
# {{ cargo.package.name }}

Version: {{ cargo.package.version }}

Dependencies:
{% for dep, version in cargo.dependencies %}
- {{ dep }}: {{ version }}
{% endfor %}
```

**Practical Example - Multi-format Configuration:**
```
{# Load configuration from different sources #}
{% set json_config = read_json_file(path="config.json") %}
{% set yaml_config = read_yaml_file(path="config.yaml") %}
{% set toml_config = read_toml_file(path="Cargo.toml") %}

# Application Configuration Report

## From JSON ({{ json_config.app_name }})
- Version: {{ json_config.version }}
- Debug: {{ json_config.debug }}

## From YAML
Environment: {{ yaml_config.environment }}
{% for service in yaml_config.services %}
- Service {{ service.name }}: {{ service.host }}:{{ service.port }}
{% endfor %}

## From TOML ({{ toml_config.package.name }})
Rust Version: {{ toml_config.package.edition }}
Dependencies: {{ toml_config.dependencies | length }}
```

### Data Serialization Functions

Convert objects and data structures to formatted strings (JSON, YAML, TOML). Useful for generating configuration files, API payloads, or converting between formats.

#### `to_json(object, pretty)`

Convert an object to a JSON string.

**Arguments:**
- `object` (required) - Object/value to convert to JSON
- `pretty` (optional) - Enable pretty-printing with indentation (default: false)

**Returns:** JSON string

**Examples:**
```jinja
{# Simple JSON serialization #}
{% set config = {"host": "localhost", "port": 8080, "debug": true} %}
{{ to_json(object=config) }}
{# Output: {"host":"localhost","port":8080,"debug":true} #}

{# Pretty-printed JSON #}
{{ to_json(object=config, pretty=true) }}
{# Output:
{
  "host": "localhost",
  "port": 8080,
  "debug": true
}
#}

{# Convert array to JSON #}
{% set items = [1, 2, 3, 4, 5] %}
{{ to_json(object=items) }}
{# Output: [1,2,3,4,5] #}

{# Generate API payload #}
{% set api_request = {
  "method": "POST",
  "endpoint": "/api/users",
  "data": {
    "username": get_env(name="USERNAME"),
    "email": get_env(name="EMAIL")
  }
} %}
{{ to_json(object=api_request, pretty=true) }}
```

#### `to_yaml(object)`

Convert an object to a YAML string.

**Arguments:**
- `object` (required) - Object/value to convert to YAML

**Returns:** YAML string

**Examples:**
```jinja
{# Simple YAML serialization #}
{% set config = {"host": "localhost", "port": 8080, "debug": true} %}
{{ to_yaml(object=config) }}
{# Output:
host: localhost
port: 8080
debug: true
#}

{# Convert array to YAML #}
{% set items = ["apple", "banana", "cherry"] %}
{{ to_yaml(object=items) }}
{# Output:
- apple
- banana
- cherry
#}

{# Generate Kubernetes config #}
{% set k8s_config = {
  "apiVersion": "v1",
  "kind": "ConfigMap",
  "metadata": {
    "name": get_env(name="APP_NAME", default="myapp"),
    "namespace": get_env(name="NAMESPACE", default="default")
  },
  "data": {
    "database.url": get_env(name="DATABASE_URL"),
    "cache.enabled": "true"
  }
} %}
{{ to_yaml(object=k8s_config) }}
```

#### `to_toml(object)`

Convert an object to a TOML string.

**Arguments:**
- `object` (required) - Object/value to convert to TOML

**Returns:** TOML string

**Note:** TOML has specific requirements:
- Root level must be a table (object/map)
- Arrays must contain elements of the same type

**Examples:**
```jinja
{# Simple TOML serialization #}
{% set config = {"title": "My App", "version": "1.0.0"} %}
{{ to_toml(object=config) }}
{# Output:
title = "My App"
version = "1.0.0"
#}

{# Generate Cargo.toml dependencies #}
{% set cargo_config = {
  "package": {
    "name": get_env(name="PACKAGE_NAME", default="myapp"),
    "version": "1.0.0",
    "edition": "2021"
  },
  "dependencies": {
    "serde": "1.0",
    "tokio": {"version": "1.0", "features": ["full"]}
  }
} %}
{{ to_toml(object=cargo_config) }}
{# Output:
[package]
name = "myapp"
version = "1.0.0"
edition = "2021"

[dependencies]
serde = "1.0"

[dependencies.tokio]
version = "1.0"
features = ["full"]
#}

{# Array of tables #}
{% set database_config = {
  "database": [
    {"name": "primary", "host": "db1.example.com", "port": 5432},
    {"name": "replica", "host": "db2.example.com", "port": 5432}
  ]
} %}
{{ to_toml(object=database_config) }}
{# Output:
[[database]]
name = "primary"
host = "db1.example.com"
port = 5432

[[database]]
name = "replica"
host = "db2.example.com"
port = 5432
#}
```

**Practical Example - Format Conversion:**
```jinja
{# Read JSON, convert to YAML #}
{% set json_config = read_json_file(path="config.json") %}

# Generated YAML from JSON config
{{ to_yaml(object=json_config) }}

{# Read environment variables and generate TOML #}
{% set env_config = {
  "server": {
    "host": get_env(name="SERVER_HOST", default="0.0.0.0"),
    "port": get_env(name="SERVER_PORT", default="8080") | int,
    "workers": get_env(name="WORKERS", default="4") | int
  },
  "database": {
    "url": get_env(name="DATABASE_URL", default="postgres://localhost/mydb"),
    "max_connections": get_env(name="DB_MAX_CONN", default="10") | int
  }
} %}

{{ to_toml(object=env_config) }}
```

### System & Network Functions

Access system information and perform network operations.

#### `get_hostname()`

Get the system hostname.

**Arguments:** None

**Returns:** String containing the system hostname

**Example:**
```
Server: {{ get_hostname() }}
{# Output: Server: myserver.local #}
```

#### `get_username()`

Get the current system username.

**Arguments:** None

**Returns:** String containing the current username

**Example:**
```
User: {{ get_username() }}
{# Output: User: john #}
```

#### `get_home_dir()`

Get the user's home directory.

**Arguments:** None

**Returns:** String containing the home directory path

**Example:**
```
Home: {{ get_home_dir() }}
{# Output: Home: /Users/john #}
```

#### `get_temp_dir()`

Get the system temporary directory.

**Arguments:** None

**Returns:** String containing the temp directory path

**Example:**
```
Temp: {{ get_temp_dir() }}
{# Output: Temp: /tmp #}
```

#### `get_ip_address(interface)`

Get IP address of a network interface or the primary local IP.

**Arguments:**
- `interface` (optional) - Network interface name (e.g., "eth0", "en0")

**Returns:** String containing the IP address

**Example:**
```
{# Get primary local IP #}
Local IP: {{ get_ip_address() }}
{# Output: Local IP: 192.168.1.100 #}

{# Get specific interface IP #}
Eth0 IP: {{ get_ip_address(interface="eth0") }}
```

#### `resolve_dns(hostname)`

Resolve a hostname to an IP address using DNS.

**Arguments:**
- `hostname` (required) - Hostname to resolve

**Returns:** String containing the resolved IP address

**Example:**
```
Google IP: {{ resolve_dns(hostname="google.com") }}
{# Output: Google IP: 142.250.190.46 #}

Local: {{ resolve_dns(hostname="localhost") }}
{# Output: Local: 127.0.0.1 or ::1 #}
```

#### `is_port_available(port)`

Check if a port is available (not in use).

**Arguments:**
- `port` (required) - Port number to check (1-65535)

**Returns:** Boolean (`true` if available, `false` if in use)

**Example:**
```
{% if is_port_available(port=8080) %}
  Port 8080 is available
{% else %}
  Port 8080 is already in use
{% endif %}

{# Dynamic port selection #}
{% if is_port_available(port=3000) %}
APP_PORT=3000
{% elif is_port_available(port=3001) %}
APP_PORT=3001
{% else %}
APP_PORT=8080
{% endif %}
```

**Practical Example - Dynamic Application Config:**
```yaml
application:
  hostname: {{ get_hostname() }}
  user: {{ get_username() }}

network:
  bind_ip: {{ get_ip_address() }}
  {% if is_port_available(port=8080) %}
  port: 8080
  {% else %}
  port: 8081  # Fallback port
  {% endif %}

paths:
  home: {{ get_home_dir() }}
  temp: {{ get_temp_dir() }}
  logs: {{ get_home_dir() }}/logs/app.log

services:
  database: {{ resolve_dns(hostname="db.local") }}
  cache: {{ resolve_dns(hostname="redis.local") }}
```

### Validation Functions

Validate strings against specific formats. Useful for validating user input, configuration values, or data from external sources.

#### `is_email(string)`

Validate if a string is a valid email address format.

**Arguments:**
- `string` (required) - String to validate

**Returns:** Boolean (`true` if valid email, `false` otherwise)

**Examples:**
```
Email: user@example.com
Valid: {{ is_email(string="user@example.com") }}
{# Output: Valid: true #}

Email: invalid-email
Valid: {{ is_email(string="invalid-email") }}
{# Output: Valid: false #}
```

#### `is_url(string)`

Validate if a string is a valid URL (supports http, https, ftp, file schemes).

**Arguments:**
- `string` (required) - String to validate

**Returns:** Boolean (`true` if valid URL, `false` otherwise)

**Examples:**
```
URL: https://example.com/path
Valid: {{ is_url(string="https://example.com/path") }}
{# Output: Valid: true #}

URL: not-a-url
Valid: {{ is_url(string="not-a-url") }}
{# Output: Valid: false #}
```

#### `is_ip(string)`

Validate if a string is a valid IP address (IPv4 or IPv6).

**Arguments:**
- `string` (required) - String to validate

**Returns:** Boolean (`true` if valid IP, `false` otherwise)

**Examples:**
```
IPv4: 192.168.1.1
Valid: {{ is_ip(string="192.168.1.1") }}
{# Output: Valid: true #}

IPv6: 2001:db8::1
Valid: {{ is_ip(string="2001:db8::1") }}
{# Output: Valid: true #}

Invalid: 256.1.1.1
Valid: {{ is_ip(string="256.1.1.1") }}
{# Output: Valid: false #}
```

#### `is_uuid(string)`

Validate if a string is a valid UUID format.

**Arguments:**
- `string` (required) - String to validate

**Returns:** Boolean (`true` if valid UUID, `false` otherwise)

**Examples:**
```
UUID: 550e8400-e29b-41d4-a716-446655440000
Valid: {{ is_uuid(string="550e8400-e29b-41d4-a716-446655440000") }}
{# Output: Valid: true #}

Invalid: not-a-uuid
Valid: {{ is_uuid(string="not-a-uuid") }}
{# Output: Valid: false #}
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

### Debugging & Development Functions

Functions for debugging templates, validating data, and controlling template execution flow during development and production.

#### `debug(value)`

Print a value to stderr and return it unchanged. Useful for inspecting values during template development.

**Arguments:**
- `value` (required) - Value to debug

**Returns:** The same value (allows chaining)

**Examples:**
```jinja
{# Debug a variable #}
{% set config = debug(value=parse_json(string='{"port": 8080}')) %}
Port: {{ config.port }}

{# Debug in a pipeline #}
Result: {{ get_env(name="PATH") | debug }}

{# Debug intermediate values #}
{% set users = debug(value=filter_env(pattern="USER_*")) %}
Found {{ users | length }} user variables
```

**Output to stderr:**
```
[DEBUG] {"port": 8080}
[DEBUG] /usr/local/bin:/usr/bin:/bin
[DEBUG] [{"key": "USER_NAME", "value": "admin"}]
```

#### `type_of(value)`

Get the type of a value. Returns a string describing the value type.

**Arguments:**
- `value` (required) - Value to check

**Returns:** String type name: `"string"`, `"number"`, `"bool"`, `"array"`, `"object"`, `"undefined"`

**Examples:**
```jinja
{{ type_of(value="hello") }}  {# Output: string #}
{{ type_of(value=123) }}      {# Output: number #}
{{ type_of(value=true) }}     {# Output: bool #}
{{ type_of(value=[1,2,3]) }}  {# Output: array #}

{# Conditional logic based on type #}
{% set data = get_env(name="DATA", default="[]") %}
{% if type_of(value=data) == "string" %}
  {% set data = parse_json(string=data) %}
{% endif %}

{# Type-safe processing #}
{% if type_of(value=config.workers) == "number" %}
  Workers: {{ config.workers }}
{% else %}
  Workers: {{ config.workers | int }}
{% endif %}
```

#### `inspect(value)`

Pretty-print a value's structure to stderr and return it unchanged. Shows detailed structure of complex objects and arrays.

**Arguments:**
- `value` (required) - Value to inspect

**Returns:** The same value (allows chaining)

**Examples:**
```jinja
{# Inspect complex data structures #}
{% set config = inspect(value=read_json_file(path="config.json")) %}

{# Inspect and continue #}
{% set env_vars = inspect(value=filter_env(pattern="DB_*")) %}
Database variables: {{ env_vars | length }}
```

**Output to stderr:**
```
[INSPECT] {
  "database": {
    "host": "localhost",
    "port": 5432,
    "name": "myapp"
  },
  "redis": {
    "host": "localhost",
    "port": 6379
  }
}
```

#### `assert(condition, message)`

Assert that a condition is true, otherwise abort rendering with an error message.

**Arguments:**
- `condition` (required) - Boolean condition to check
- `message` (optional) - Error message if assertion fails (default: "Assertion failed")

**Returns:** `true` if condition passes

**Examples:**
```jinja
{# Assert required environment variable #}
{% set port = get_env(name="PORT", default="") %}
{{ assert(condition=port != "", message="PORT environment variable is required") }}

{# Assert file exists before reading #}
{{ assert(condition=file_exists(path="config.json"), message="config.json not found") }}
{% set config = read_file(path="config.json") %}

{# Assert valid range #}
{% set workers = get_env(name="WORKERS", default="4") | int %}
{{ assert(condition=workers >= 1 and workers <= 100, message="WORKERS must be between 1 and 100") }}

{# Assert valid email format #}
{% set admin_email = get_env(name="ADMIN_EMAIL") %}
{{ assert(condition=is_email(string=admin_email), message="ADMIN_EMAIL must be valid email") }}
```

**Error output (if assertion fails):**
```
Error: PORT environment variable is required
```

#### `warn(message)`

Print a warning message to stderr and continue rendering. Non-fatal warnings for deprecated features or missing optional configuration.

**Arguments:**
- `message` (required) - Warning message

**Returns:** Empty string (no template output)

**Examples:**
```jinja
{# Warn about missing optional config #}
{% if not file_exists(path="custom.conf") %}
  {{ warn(message="custom.conf not found, using defaults") }}
{% endif %}

{# Warn about deprecated environment variable #}
{% set old_var = get_env(name="DEPRECATED_VAR", default="") %}
{% if old_var %}
  {{ warn(message="DEPRECATED_VAR is deprecated, use NEW_VAR instead") }}
  {% set new_var = old_var %}
{% else %}
  {% set new_var = get_env(name="NEW_VAR", default="default") %}
{% endif %}

{# Warn about potentially unsafe configuration #}
{% set debug = get_env(name="DEBUG", default="false") %}
{% set env = get_env(name="APP_ENV", default="development") %}
{% if debug == "true" and env == "production" %}
  {{ warn(message="WARNING: DEBUG mode enabled in production environment") }}
{% endif %}
```

**Output to stderr:**
```
[WARNING] custom.conf not found, using defaults
[WARNING] DEPRECATED_VAR is deprecated, use NEW_VAR instead
[WARNING] WARNING: DEBUG mode enabled in production environment
```

#### `abort(message)`

Immediately abort template rendering with an error message. Use for critical failures where rendering should not continue.

**Arguments:**
- `message` (required) - Error message

**Returns:** Never returns (always throws error)

**Examples:**
```jinja
{# Abort if critical file missing #}
{% if not file_exists(path="critical.conf") %}
  {{ abort(message="Critical configuration file 'critical.conf' is missing") }}
{% endif %}

{# Abort if environment is invalid #}
{% set env = get_env(name="APP_ENV", default="") %}
{% if env not in ["development", "staging", "production"] %}
  {{ abort(message="Invalid APP_ENV: must be development, staging, or production, got: " ~ env) }}
{% endif %}

{# Abort on validation failure #}
{% set port = get_env(name="PORT", default="8080") | int %}
{% if port < 1024 or port > 65535 %}
  {{ abort(message="Invalid PORT: must be between 1024 and 65535, got: " ~ port) }}
{% endif %}

{# Abort if required secrets are missing #}
{% set api_key = get_env(name="API_KEY", default="") %}
{% set db_password = get_env(name="DB_PASSWORD", default="") %}
{% if api_key == "" or db_password == "" %}
  {{ abort(message="Missing required secrets: API_KEY and DB_PASSWORD must be set") }}
{% endif %}
```

**Error output:**
```
Error: Critical configuration file 'critical.conf' is missing
```

**Practical Example - Configuration Validation:**
```yaml
# Production Configuration Template

# Validate critical environment
{% set env = get_env(name="APP_ENV", default="") %}
{{ assert(condition=env in ["staging", "production"], message="APP_ENV must be staging or production") }}

# Validate required secrets
{% set db_url = get_env(name="DATABASE_URL", default="") %}
{{ assert(condition=db_url != "", message="DATABASE_URL is required") }}

{% set api_key = get_env(name="API_KEY", default="") %}
{{ assert(condition=api_key != "", message="API_KEY is required") }}

# Warn about debug mode
{% set debug = get_env(name="DEBUG", default="false") %}
{% if debug == "true" %}
  {{ warn(message="DEBUG mode is enabled in " ~ env) }}
{% endif %}

# Debug configuration for troubleshooting
{% set config = {
  "environment": env,
  "database": db_url,
  "debug": debug
} %}
{{ inspect(value=config) }}

# Type-safe port configuration
{% set port = get_env(name="PORT", default="8080") %}
{% if type_of(value=port) == "string" %}
  {% set port = port | int %}
{% endif %}
{{ assert(condition=port > 0 and port < 65536, message="PORT must be valid") }}

application:
  environment: {{ env }}
  port: {{ port }}
  debug: {{ debug }}
  database_url: {{ db_url }}
  api_key: {{ api_key }}
```

**Use Cases:**
- ✅ **Development**: Debug complex data structures with `debug()` and `inspect()`
- ✅ **Validation**: Ensure configuration correctness with `assert()`
- ✅ **Type Safety**: Check value types with `type_of()` before operations
- ✅ **Graceful Degradation**: Use `warn()` for non-critical issues
- ✅ **Fail Fast**: Use `abort()` for critical failures requiring immediate attention

## Advanced Examples

### Docker Compose Generator

**Template** (`docker-compose.tmpl`):
```yaml
version: '3.8'

services:
  {{ get_env(name="SERVICE_NAME", default="app") }}:
    image: {{ get_env(name="DOCKER_IMAGE", default="node:18") }}
    ports:
      - "{{ get_env(name="HOST_PORT", default="3000") }}:{{ get_env(name="CONTAINER_PORT", default="3000") }}"
    environment:
      - NODE_ENV={{ get_env(name="NODE_ENV", default="development") }}
      {% set db_url = get_env(name="DATABASE_URL", default="") %}
      {% if db_url %}
      - DATABASE_URL={{ db_url }}
      {% endif %}
    {% set enable_volumes = get_env(name="ENABLE_VOLUMES", default="false") %}
    {% if enable_volumes == "true" %}
    volumes:
      - ./app:/app
    {% endif %}
```

**Render with custom values:**
```bash
SERVICE_NAME=web \
DOCKER_IMAGE=node:20 \
HOST_PORT=8080 \
NODE_ENV=production \
DATABASE_URL=postgres://db:5432/mydb \
ENABLE_VOLUMES=true \
tmpltool docker-compose.tmpl -o docker-compose.yml
```

**Or use defaults:**
```bash
tmpltool docker-compose.tmpl -o docker-compose.yml
```

### Comprehensive Application Configuration

See [examples/comprehensive-app-config.tmpl](examples/comprehensive-app-config.tmpl) for a complete example demonstrating all features:

- All hash functions (MD5, SHA1, SHA256, SHA512)
- UUID generation
- Random strings with various charsets
- Environment variables with defaults
- Pattern filtering with `filter_env()`
- Conditionals (if/elif/else)
- Loops with ranges and arrays
- Filters (upper, lower, trim, slugify, replace, split, length)
- Comments
- Complex nested logic

### Pipeline Usage

```bash
# Generate and validate JSON config
tmpltool config.json.tmpl | jq .

# Generate and apply Kubernetes config
tmpltool k8s-deployment.yaml.tmpl | kubectl apply -f -

# Generate nginx config and test it
tmpltool nginx.conf.tmpl | nginx -t -c /dev/stdin

# Combine multiple templates
cat header.tmpl body.tmpl footer.tmpl | tmpltool > complete.html
```

### Trust Mode - System Files

**Template** (`system_info.tmpl`):
```
# System Information

## Hostname
{{ read_file(path="/etc/hostname") }}

## Hosts File (first 200 chars)
{{ read_file(path="/etc/hosts") | truncate(length=200) }}

## Files in /etc (first 10)
{% for file in list_dir(path="/etc") | slice(end=10) %}
- {{ file }}
{% endfor %}
```

**Render:**
```bash
# Without --trust: Security error
tmpltool system_info.tmpl
# Error: Security: Absolute paths are not allowed

# With --trust: Works!
tmpltool --trust system_info.tmpl -o system_info.md
```

**WARNING:** Only use `--trust` with templates you completely trust. Malicious templates could read sensitive files like SSH keys, passwords, or system configurations.

## Error Handling

- **Missing template file:** tmpltool exits with an error
- **Invalid template syntax:** Error location is reported
- **Environment variables:**
  - Direct access not supported: `{{ ENV_VAR }}` causes an error
  - With default (recommended): `{{ get_env(name="VAR", default="...") }}` uses default if missing
  - Without default: `{{ get_env(name="VAR") }}` errors if variable doesn't exist
- **Filesystem errors:** Clear error messages for missing files, permission issues, or security violations

## Development

### Prerequisites

- Rust 1.70 or higher
- Cargo (comes with Rust)

Install Rust from [rustup.rs](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release
# Binary at: ./target/release/tmpltool
```

### Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Code Quality

```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Full QA check (format + clippy + test)
cargo make qa
```

### Using cargo-make

Install cargo-make:

```bash
cargo install --force cargo-make
```

**Common tasks:**
```bash
cargo make build              # Build debug
cargo make build-release      # Build release
cargo make test              # Run tests
cargo make qa                # Full QA (format + clippy + test)
cargo make ci                # CI checks
cargo make docs              # Generate docs
```

**Cross-platform builds:**
```bash
cargo make build-linux-x86_64      # Linux x86_64
cargo make build-linux-musl        # Linux (static)
cargo make build-macos-x86_64      # macOS Intel
cargo make build-macos-aarch64     # macOS Apple Silicon
cargo make build-windows-x86_64    # Windows
cargo make build-all-platforms     # All platforms
```

For more development details, see the [full development guide](README.md.backup#development) in the backup.

## CI/CD

This project uses GitHub Actions for continuous integration and automated releases.

### Continuous Integration

Every pull request and push to master triggers:
- Code formatting check (`rustfmt`)
- Linting (`clippy`)
- Multi-platform tests (Ubuntu, macOS, Windows)
- Code coverage (uploaded to Codecov)
- Example template testing

### Automated Releases

Releases use [semantic-release](https://github.com/semantic-release/semantic-release):
1. Analyzes commit messages
2. Determines next version
3. Updates `Cargo.toml`
4. Generates `CHANGELOG.md`
5. Builds multi-platform binaries
6. Creates GitHub release
7. Publishes Docker images to GHCR

### Commit Convention

Follow [Conventional Commits](https://www.conventionalcommits.org/):

- `feat: description` - New feature (minor bump: 1.2.0 → 1.3.0)
- `fix: description` - Bug fix (patch bump: 1.2.0 → 1.2.1)
- `feat!: description` - Breaking change (major bump: 1.2.0 → 2.0.0)
- `docs:`, `refactor:`, `perf:` - Other changes (patch bump)
- `style:`, `test:`, `chore:`, `ci:` - No version bump

**Examples:**
```bash
git commit -m "feat: add validation functions"
git commit -m "fix: correct path resolution"
git commit -m "feat!: change output behavior

BREAKING CHANGE: Output now defaults to stdout"
```

## Contributing

Contributions are welcome! For detailed guidelines, see [CONTRIBUTING.md](CONTRIBUTING.md).

### Quick Start

1. Fork the repository
2. Clone and install dependencies:
   ```bash
   git clone https://github.com/YOUR_USERNAME/tmpltool.git
   cd tmpltool
   npm install  # Installs commit hooks
   ```
3. Create a feature branch
4. Make your changes
5. Run QA checks: `cargo make qa`
6. Commit using conventional commits
7. Push and open a Pull Request

**Note:** Commit messages are automatically validated. Invalid commits will be rejected.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

For more examples, see the [examples/](examples/) directory and [examples/README.md](examples/README.md).

For complete MiniJinja/Jinja2 syntax documentation, visit:
- MiniJinja Docs: https://docs.rs/minijinja/
- Jinja2 Template Designer: https://jinja.palletsprojects.com/templates/
