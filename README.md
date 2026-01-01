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
  - [Object Manipulation Functions](#object-manipulation-functions)
  - [Validation Functions](#validation-functions)
  - [System & Network Functions](#system--network-functions)
  - [Math Functions](#math-functions)
  - [Array Functions](#array-functions)
  - [Statistical Functions](#statistical-functions)
  - [Predicate Functions](#predicate-functions)
  - [Kubernetes Functions](#kubernetes-functions)
  - [Web & URL Functions](#web--url-functions)
  - [Logic Functions](#logic-functions)
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
# Download binary for your platform from releases
# https://github.com/bordeux/tmpltool/releases

# Or use Docker to copy the binary (recommended for CI/CD):
# Create a Dockerfile to extract the binary
cat > Dockerfile << 'EOF'
FROM alpine:latest
COPY --from=ghcr.io/bordeux/tmpltool:latest /tmpltool /usr/local/bin/tmpltool
EOF

docker build -t myapp .
# Now tmpltool is available in your image at /usr/local/bin/tmpltool

# Create and render template
echo 'Hello {{ get_env(name="USER", default="World") }}!' > greeting.tmpl
tmpltool greeting.tmpl
# Output: Hello World!
```

## Features

- **Environment Variables**: Access env vars with `get_env()` and filter with `filter_env()`
- **Hash & Crypto**: MD5, SHA1, SHA256, SHA512, UUID generation, random strings
- **Encoding & Security**: Base64, hex, bcrypt, HMAC, HTML/XML/shell escaping, secure random strings
- **Filesystem**: Read files, check existence, list directories, glob patterns, file info, path manipulation
- **Data Parsing**: Parse and read JSON, YAML, TOML files
- **Data Serialization**: Convert objects to JSON, YAML, TOML strings with pretty-printing options
- **Object Manipulation**: Deep merge, get/set nested values by path, extract keys/values, check key existence
- **Validation**: Validate emails, URLs, IPs, UUIDs, regex matching
- **System & Network**: Get hostname, username, directories, IP addresses, DNS resolution, port availability
- **Web & URL**: Parse and build URLs, generate query strings, HTTP Basic Auth headers
- **Kubernetes**: Resource requests, label sanitization, ConfigMap/Secret references for manifests
- **Math & Logic**: Min/max, rounding, percentages, default values, ternary operations, range checks
- **Array & Statistics**: Sorting, grouping, chunking, sum/avg/median, unique values, flattening
- **Debugging & Development**: Debug output, type checking, assertions, warnings, error handling
- **String Filters**: 12+ filters for case conversion, indentation, padding, quoting, and more
- **Security**: Built-in protections with optional `--trust` mode
- **Flexible I/O**: File or stdin input, file or stdout output
- **Full Jinja2 Syntax**: Conditionals, loops, filters, and more
- **Single Binary**: No runtime dependencies, static binaries available
- **Docker-Friendly**: Extract binary from Docker image (multi-arch support)

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

Docker images are available for extracting the binary into your own images (similar to gomplate pattern):

**In Your Dockerfile:**
```dockerfile
# Multi-stage build to copy tmpltool binary
FROM ghcr.io/bordeux/tmpltool:latest AS tmpltool

FROM alpine:latest
# Copy the binary from the tmpltool image
COPY --from=tmpltool /tmpltool /usr/local/bin/tmpltool

# Now use tmpltool in your build process
COPY config.tmpl /app/
RUN tmpltool /app/config.tmpl -o /app/config.json --validate json
```

**Available Tags:**
- `latest` - Latest stable release
- `v1.x.x` - Specific version tags
- Multi-arch support: `linux/amd64`, `linux/arm64`

**For Local Testing:**
```bash
# Extract binary to local system
docker create --name tmpltool-tmp ghcr.io/bordeux/tmpltool:latest
docker cp tmpltool-tmp:/tmpltool ./tmpltool
docker rm tmpltool-tmp
chmod +x ./tmpltool
./tmpltool --version
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

### Object Manipulation Functions

Work with objects (maps/dictionaries) to merge, access nested values, and inspect structure. These functions are essential for complex configuration generation and data transformation.

#### `object_merge(obj1, obj2)`

Deep merge two objects. When keys conflict, values from `obj2` override values from `obj1`. Nested objects are merged recursively.

**Arguments:**
- `obj1` (required) - First object (base)
- `obj2` (required) - Second object (overlay, takes precedence)

**Returns:** New object with merged values

**Examples:**
```jinja
{# Simple merge #}
{% set base = {"a": 1, "b": 2} %}
{% set overlay = {"c": 3, "d": 4} %}
{% set merged = object_merge(obj1=base, obj2=overlay) %}
{{ to_json(object=merged) }}
{# Output: {"a":1,"b":2,"c":3,"d":4} #}

{# Override values #}
{% set defaults = {"host": "localhost", "port": 8080, "debug": false} %}
{% set custom = {"port": 3000, "debug": true} %}
{% set config = object_merge(obj1=defaults, obj2=custom) %}
{{ to_json(object=config) }}
{# Output: {"host":"localhost","port":3000,"debug":true} #}

{# Deep merge nested objects #}
{% set base_config = {
  "server": {"host": "localhost", "port": 8080},
  "database": {"host": "db.local", "port": 5432}
} %}
{% set env_overrides = {
  "server": {"port": 9000, "ssl": true},
  "cache": {"enabled": true}
} %}
{% set final_config = object_merge(obj1=base_config, obj2=env_overrides) %}
{{ to_json(object=final_config, pretty=true) }}
{# Output:
{
  "server": {
    "host": "localhost",
    "port": 9000,
    "ssl": true
  },
  "database": {
    "host": "db.local",
    "port": 5432
  },
  "cache": {
    "enabled": true
  }
}
#}
```

#### `object_get(object, path)`

Get nested value from an object using dot-separated path notation. Supports accessing nested objects and array indices.

**Arguments:**
- `object` (required) - Object to query
- `path` (required) - Dot-separated path (e.g., "a.b.c" or "items.0")

**Returns:** Value at the specified path, or undefined if not found

**Examples:**
```jinja
{# Simple property access #}
{% set config = {"host": "localhost", "port": 8080} %}
{{ object_get(object=config, path="host") }}
{# Output: localhost #}

{# Nested property access #}
{% set config = {
  "server": {
    "database": {
      "host": "db.example.com",
      "port": 5432
    }
  }
} %}
{{ object_get(object=config, path="server.database.host") }}
{# Output: db.example.com #}

{# Array index access #}
{% set data = {"items": [10, 20, 30, 40]} %}
{{ object_get(object=data, path="items.1") }}
{# Output: 20 #}

{# Safe access with default fallback #}
{% set config = {"server": {"host": "localhost"}} %}
{% set port = object_get(object=config, path="server.port") %}
{% if port is undefined %}
  Port not configured, using default: 8080
{% else %}
  Port: {{ port }}
{% endif %}

{# Complex nested access #}
{% set k8s_config = {
  "spec": {
    "template": {
      "spec": {
        "containers": [
          {"name": "app", "image": "myapp:latest"}
        ]
      }
    }
  }
} %}
{{ object_get(object=k8s_config, path="spec.template.spec.containers.0.image") }}
{# Output: myapp:latest #}
```

#### `object_set(object, path, value)`

Set nested value in an object using dot-separated path notation. Creates intermediate objects as needed.

**Arguments:**
- `object` (required) - Object to modify
- `path` (required) - Dot-separated path (e.g., "a.b.c")
- `value` (required) - Value to set

**Returns:** New object with the value set at the specified path

**Examples:**
```jinja
{# Simple property set #}
{% set config = {"host": "localhost"} %}
{% set updated = object_set(object=config, path="port", value=8080) %}
{{ to_json(object=updated) }}
{# Output: {"host":"localhost","port":8080} #}

{# Set nested property #}
{% set config = {"server": {"host": "localhost"}} %}
{% set updated = object_set(object=config, path="server.port", value=8080) %}
{{ to_json(object=updated) }}
{# Output: {"server":{"host":"localhost","port":8080}} #}

{# Create nested path automatically #}
{% set config = {} %}
{% set updated = object_set(object=config, path="database.primary.host", value="db1.example.com") %}
{{ to_json(object=updated, pretty=true) }}
{# Output:
{
  "database": {
    "primary": {
      "host": "db1.example.com"
    }
  }
}
#}

{# Build configuration step by step #}
{% set config = {} %}
{% set config = object_set(object=config, path="server.host", value=get_env(name="HOST", default="0.0.0.0")) %}
{% set config = object_set(object=config, path="server.port", value=get_env(name="PORT", default="8080") | int) %}
{% set config = object_set(object=config, path="database.url", value=get_env(name="DATABASE_URL")) %}
{{ to_json(object=config, pretty=true) }}
```

#### `object_keys(object)`

Get all keys from an object as an array.

**Arguments:**
- `object` (required) - Object to get keys from

**Returns:** Array of string keys

**Examples:**
```jinja
{# Get all keys #}
{% set config = {"host": "localhost", "port": 8080, "debug": true} %}
{% set keys = object_keys(object=config) %}
{{ to_json(object=keys) }}
{# Output: ["host","port","debug"] #}

{# Iterate over keys #}
{% set config = {"host": "localhost", "port": 8080, "debug": true} %}
Configuration keys:
{% for key in object_keys(object=config) %}
  - {{ key }}
{% endfor %}
{# Output:
Configuration keys:
  - host
  - port
  - debug
#}

{# Dynamic configuration display #}
{% set config = {
  "SERVER_HOST": "localhost",
  "SERVER_PORT": 8080,
  "DATABASE_URL": "postgres://localhost/mydb"
} %}
# Environment Variables
{% for key in object_keys(object=config) %}
{{ key }}={{ config[key] }}
{% endfor %}
```

#### `object_values(object)`

Get all values from an object as an array.

**Arguments:**
- `object` (required) - Object to get values from

**Returns:** Array of values

**Examples:**
```jinja
{# Get all values #}
{% set config = {"a": 1, "b": 2, "c": 3} %}
{% set values = object_values(object=config) %}
{{ to_json(object=values) }}
{# Output: [1,2,3] #}

{# Process all values #}
{% set ports = {"http": 80, "https": 443, "app": 8080} %}
Open ports:
{% for port in object_values(object=ports) %}
  - {{ port }}
{% endfor %}
{# Output:
Open ports:
  - 80
  - 443
  - 8080
#}

{# Mixed type values #}
{% set config = {"str": "hello", "num": 42, "bool": true} %}
{% for value in object_values(object=config) %}
  Value: {{ value }} (type: {{ type_of(value=value) }})
{% endfor %}
```

#### `object_has_key(object, key)`

Check if an object has a specific key.

**Arguments:**
- `object` (required) - Object to check
- `key` (required) - Key to check for

**Returns:** Boolean - true if key exists, false otherwise

**Examples:**
```jinja
{# Simple key check #}
{% set config = {"host": "localhost", "port": 8080} %}
{{ object_has_key(object=config, key="host") }}
{# Output: true #}

{{ object_has_key(object=config, key="database") }}
{# Output: false #}

{# Conditional configuration #}
{% set config = {"host": "localhost", "port": 8080} %}
{% if object_has_key(object=config, key="debug") %}
Debug mode: {{ config.debug }}
{% else %}
Debug mode not configured (using default: false)
{% endif %}

{# Validate required fields #}
{% set config = read_json_file(path="config.json") %}
{% set required_keys = ["host", "port", "database_url"] %}
{% for key in required_keys %}
  {% if not object_has_key(object=config, key=key) %}
ERROR: Missing required configuration key: {{ key }}
  {% endif %}
{% endfor %}

{# Feature flags #}
{% set features = {"api": true, "websockets": true} %}
{% if object_has_key(object=features, key="websockets") and features.websockets %}
  WebSocket support enabled
{% endif %}
```

**Practical Example - Configuration Merging:**
```jinja
{# Load base configuration #}
{% set base_config = read_json_file(path="config.base.json") %}

{# Load environment-specific overrides #}
{% set env = get_env(name="ENVIRONMENT", default="development") %}
{% set env_config_path = "config." ~ env ~ ".json" %}

{% if file_exists(path=env_config_path) %}
  {% set env_config = read_json_file(path=env_config_path) %}
  {% set config = object_merge(obj1=base_config, obj2=env_config) %}
{% else %}
  {% set config = base_config %}
{% endif %}

{# Apply environment variable overrides #}
{% if get_env(name="DATABASE_URL") %}
  {% set config = object_set(object=config, path="database.url", value=get_env(name="DATABASE_URL")) %}
{% endif %}

{% if get_env(name="PORT") %}
  {% set config = object_set(object=config, path="server.port", value=get_env(name="PORT") | int) %}
{% endif %}

{# Validate required keys #}
{% set required = ["server.host", "server.port", "database.url"] %}
{% for key_path in required %}
  {% if object_get(object=config, path=key_path) is undefined %}
ERROR: Missing required configuration: {{ key_path }}
  {% endif %}
{% endfor %}

{# Output final configuration #}
{{ to_json(object=config, pretty=true) }}
```

### Predicate Functions

Check conditions on arrays and strings with predicate functions. Useful for filtering, validation, and conditional logic.

#### `array_any(array, predicate)`

Check if any element in an array matches a predicate value.

**Arguments:**
- `array` (required) - Array to check
- `predicate` (required) - Value to match against

**Returns:** `true` if any element equals the predicate, `false` otherwise

**Examples:**
```jinja
{# Check if array contains a specific number #}
{% set numbers = [1, 2, 3, 4, 5] %}
{% if array_any(array=numbers, predicate=3) %}
  Found 3 in the array!
{% endif %}

{# Check if any environment is production #}
{% set environments = ["dev", "staging", "prod"] %}
{% if array_any(array=environments, predicate="prod") %}
  Production environment detected - enabling safeguards
{% endif %}

{# Validate required services #}
{% set services = ["web", "api", "database"] %}
{% if array_any(array=services, predicate="database") %}
  Configuring database connection
{% endif %}
```

#### `array_all(array, predicate)`

Check if all elements in an array match a predicate value.

**Arguments:**
- `array` (required) - Array to check
- `predicate` (required) - Value to match against

**Returns:** `true` if all elements equal the predicate, `false` otherwise

**Note:** Returns `true` for empty arrays (vacuous truth)

**Examples:**
```jinja
{# Check if all statuses are "active" #}
{% set statuses = ["active", "active", "active"] %}
{% if array_all(array=statuses, predicate="active") %}
  All systems operational
{% endif %}

{# Verify uniform configuration #}
{% set replicas = [3, 3, 3] %}
{% if array_all(array=replicas, predicate=3) %}
  All services scaled to 3 replicas
{% endif %}

{# Validate security settings #}
{% set ssl_enabled = [true, true, true, true] %}
{% if array_all(array=ssl_enabled, predicate=true) %}
  SSL enabled on all endpoints ✓
{% else %}
  WARNING: Some endpoints do not have SSL enabled!
{% endif %}
```

#### `array_contains(array, value)`

Check if an array contains a specific value.

**Arguments:**
- `array` (required) - Array to search
- `value` (required) - Value to find

**Returns:** `true` if the array contains the value, `false` otherwise

**Examples:**
```jinja
{# Check if feature flag is enabled #}
{% set enabled_features = ["dark-mode", "notifications", "analytics"] %}
{% if array_contains(array=enabled_features, value="analytics") %}
  <!-- Google Analytics -->
  <script src="analytics.js"></script>
{% endif %}

{# Validate allowed file types #}
{% set allowed_types = [".jpg", ".png", ".gif", ".webp"] %}
{% set file_ext = ".png" %}
{% if array_contains(array=allowed_types, value=file_ext) %}
  File type {{ file_ext }} is allowed
{% else %}
  ERROR: File type {{ file_ext }} is not allowed
{% endif %}

{# Check if user has admin role #}
{% set user_roles = ["user", "editor", "admin"] %}
{% if array_contains(array=user_roles, value="admin") %}
  Admin access granted
{% endif %}
```

#### `starts_with(string, prefix)`

Check if a string starts with a specific prefix.

**Arguments:**
- `string` (required) - String to check
- `prefix` (required) - Prefix to match

**Returns:** `true` if the string starts with the prefix, `false` otherwise

**Note:** Case-sensitive comparison

**Examples:**
```jinja
{# Validate URL protocol #}
{% set url = "https://example.com" %}
{% if starts_with(string=url, prefix="https://") %}
  Secure connection ✓
{% else %}
  WARNING: Insecure connection
{% endif %}

{# Filter files by prefix #}
{% set files = ["config.yaml", "config.prod.yaml", "data.json"] %}
Configuration files:
{% for file in files %}
  {% if starts_with(string=file, prefix="config") %}
  - {{ file }}
  {% endif %}
{% endfor %}

{# Check environment variable prefix #}
{% set var_name = "MYAPP_DATABASE_URL" %}
{% if starts_with(string=var_name, prefix="MYAPP_") %}
  Application-specific variable detected
{% endif %}

{# Path validation #}
{% set path = "/usr/local/bin/app" %}
{% if starts_with(string=path, prefix="/usr/") %}
  System path detected
{% endif %}
```

#### `ends_with(string, suffix)`

Check if a string ends with a specific suffix.

**Arguments:**
- `string` (required) - String to check
- `suffix` (required) - Suffix to match

**Returns:** `true` if the string ends with the suffix, `false` otherwise

**Note:** Case-sensitive comparison

**Examples:**
```jinja
{# Detect file types #}
{% set filename = "config.yaml" %}
{% if ends_with(string=filename, suffix=".yaml") %}
  YAML configuration file
  {% include "yaml-handler.tmpl" %}
{% elif ends_with(string=filename, suffix=".json") %}
  JSON configuration file
  {% include "json-handler.tmpl" %}
{% endif %}

{# Filter by file extension #}
{% set files = ["app.py", "test.py", "config.yaml", "README.md"] %}
Python files:
{% for file in files %}
  {% if ends_with(string=file, suffix=".py") %}
  - {{ file }}
  {% endif %}
{% endfor %}

{# Check domain names #}
{% set domain = "api.example.com" %}
{% if ends_with(string=domain, suffix=".com") %}
  Commercial domain
{% elif ends_with(string=domain, suffix=".org") %}
  Organization domain
{% endif %}

{# Detect archive files #}
{% set filename = "backup.tar.gz" %}
{% if ends_with(string=filename, suffix=".tar.gz") %}
  Compressed tar archive
{% elif ends_with(string=filename, suffix=".zip") %}
  ZIP archive
{% endif %}
```

### Kubernetes Functions

Kubernetes-specific helpers for manifest generation and label sanitization.

#### `k8s_resource_request(cpu, memory)`

Format Kubernetes resource requests in YAML format.

**Arguments:**
- `cpu` (required): CPU request - string like `"500m"` or number (converted to millicores)
- `memory` (required): Memory request - string like `"512Mi"` or number in MiB (auto-converted to Mi/Gi)

**Returns:** YAML-formatted resource request block

**Numeric conversions:**
- CPU: `0.5` → `"500m"`, `2` → `"2000m"`
- Memory: `512` → `"512Mi"`, `1024` → `"1Gi"`, `2048` → `"2Gi"`

**Example:**
```jinja
{# Basic usage with strings #}
{{ k8s_resource_request(cpu="500m", memory="512Mi") }}
{# Output:
requests:
  cpu: "500m"
  memory: "512Mi"
#}

{# With numeric values (auto-formatted) #}
{{ k8s_resource_request(cpu=0.5, memory=512) }}
{# Output:
requests:
  cpu: "500m"
  memory: "512Mi"
#}

{# In a Kubernetes deployment #}
apiVersion: apps/v1
kind: Deployment
metadata:
  name: my-app
spec:
  template:
    spec:
      containers:
      - name: app
        image: myapp:latest
        resources:
          {{ k8s_resource_request(cpu="1000m", memory="1Gi") | indent(10) }}
```

#### `k8s_label_safe(value)`

Sanitize string to be Kubernetes label-safe.

**Arguments:**
- `value` (required): String to sanitize

**Returns:** Sanitized string following Kubernetes label requirements:
- Max 63 characters
- Only alphanumeric, dashes, underscores, dots
- Must start and end with alphanumeric
- Lowercase

**Example:**
```jinja
{# Sanitize label value #}
{{ k8s_label_safe(value="My App (v2.0)") }}
{# Output: my-app-v2.0 #}

{# Use in labels #}
metadata:
  labels:
    app: {{ k8s_label_safe(value=app_name) }}
    version: {{ k8s_label_safe(value=version) }}
```

#### `k8s_dns_label_safe(value)`

Format DNS-safe label (max 63 chars, lowercase, alphanumeric and dashes only).

**Arguments:**
- `value` (required): String to format

**Returns:** DNS-safe string suitable for Kubernetes resource names

**Example:**
```jinja
{# Format DNS label #}
{{ k8s_dns_label_safe(value="My Service Name") }}
{# Output: my-service-name #}

{# Use in service names #}
apiVersion: v1
kind: Service
metadata:
  name: {{ k8s_dns_label_safe(value=service_name) }}
```

#### `k8s_env_var_ref(var_name, source, name)`

Generate Kubernetes environment variable reference (ConfigMap or Secret).

**Arguments:**
- `var_name` (required): The environment variable name/key
- `source` (optional): Source type - `"configmap"` or `"secret"` (default: `"configmap"`)
- `name` (optional): Name of the ConfigMap/Secret (default: auto-generated from var_name)

**Returns:** YAML-formatted `valueFrom` reference

**Example:**
```jinja
{# ConfigMap reference #}
- name: DATABASE_HOST
  {{ k8s_env_var_ref(var_name="DATABASE_HOST", source="configmap", name="app-config") | indent(2) }}
{# Output:
  valueFrom:
    configMapKeyRef:
      name: app-config
      key: DATABASE_HOST
#}

{# Secret reference with auto-generated name #}
- name: DB_PASSWORD
  {{ k8s_env_var_ref(var_name="DB_PASSWORD", source="secret") | indent(2) }}
{# Output:
  valueFrom:
    secretKeyRef:
      name: db-password
      key: DB_PASSWORD
#}
```

#### `k8s_secret_ref(secret_name, key, optional)`

Generate Kubernetes Secret reference for environment variables.

**Arguments:**
- `secret_name` (required): Name of the Secret
- `key` (required): Key within the Secret
- `optional` (optional): Whether the Secret is optional (default: `false`)

**Returns:** YAML-formatted `valueFrom` secretKeyRef

**Example:**
```jinja
{# Basic secret reference #}
- name: DB_PASSWORD
  {{ k8s_secret_ref(secret_name="db-credentials", key="password") | indent(2) }}
{# Output:
  valueFrom:
    secretKeyRef:
      name: db-credentials
      key: password
#}

{# Optional secret #}
- name: OPTIONAL_TOKEN
  {{ k8s_secret_ref(secret_name="tokens", key="api_token", optional=true) | indent(2) }}
{# Output:
  valueFrom:
    secretKeyRef:
      name: tokens
      key: api_token
      optional: true
#}
```

#### `k8s_configmap_ref(configmap_name, key, optional)`

Generate Kubernetes ConfigMap reference for environment variables.

**Arguments:**
- `configmap_name` (required): Name of the ConfigMap
- `key` (required): Key within the ConfigMap
- `optional` (optional): Whether the ConfigMap is optional (default: `false`)

**Returns:** YAML-formatted `valueFrom` configMapKeyRef

**Example:**
```jinja
{# Basic ConfigMap reference #}
- name: DATABASE_HOST
  {{ k8s_configmap_ref(configmap_name="app-config", key="database_host") | indent(2) }}
{# Output:
  valueFrom:
    configMapKeyRef:
      name: app-config
      key: database_host
#}

{# Optional ConfigMap #}
- name: FEATURE_FLAG
  {{ k8s_configmap_ref(configmap_name="features", key="new_ui", optional=true) | indent(2) }}
{# Output:
  valueFrom:
    configMapKeyRef:
      name: features
      key: new_ui
      optional: true
#}

{# Complete deployment example #}
apiVersion: apps/v1
kind: Deployment
metadata:
  name: {{ k8s_dns_label_safe(value=app_name) }}
spec:
  template:
    spec:
      containers:
        - name: app
          image: myapp:latest
          env:
            - name: ENVIRONMENT
              value: "production"
            - name: DATABASE_HOST
              {{ k8s_configmap_ref(configmap_name="app-config", key="db_host") | indent(14) }}
            - name: DATABASE_PASSWORD
              {{ k8s_secret_ref(secret_name="db-credentials", key="password") | indent(14) }}
          resources:
            {{ k8s_resource_request(cpu="500m", memory="512Mi") | indent(12) }}
```

### Web & URL Functions

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

#### `parse_url(url)`

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
{# Parse URL #}
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

{# Extract host from environment variable #}
{% set db_url = parse_url(url=get_env(name="DATABASE_URL")) %}
DB_HOST={{ db_url.host }}
DB_PORT={{ db_url.port }}
DB_NAME={{ url.path | trim_start_matches(pat="/") }}
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

### Logic Functions

Conditional logic and default value handling.

#### `default(value, default)`

Return default value if the provided value is falsy.

**Arguments:**
- `value` (required): Value to check
- `default` (required): Default value to return if value is falsy

**Returns:** The value if truthy, otherwise the default

**Falsy values:** `null`, `undefined`, `false`, `0`, empty string `""`, empty array `[]`

**Example:**
```jinja
{# Use default for empty string #}
{{ default(value="", default="N/A") }}
{# Output: N/A #}

{# Use actual value if truthy #}
{{ default(value="Hello", default="N/A") }}
{# Output: Hello #}

{# Configuration with defaults #}
{% set config = {"port": 8080} %}
Host: {{ default(value=config.host, default="localhost") }}
Port: {{ default(value=config.port, default=3000) }}
```

#### `coalesce(values)`

Return the first non-null value from an array.

**Arguments:**
- `values` (required): Array of values to check

**Returns:** First value that is not null/undefined, or null if all are null

**Example:**
```jinja
{# Find first non-null value #}
{% set a = none %}
{% set b = none %}
{% set c = "found" %}
{{ coalesce(values=[a, b, c]) }}
{# Output: found #}

{# Configuration precedence #}
{% set env_host = none %}
{% set config_host = "prod.example.com" %}
{% set default_host = "localhost" %}
Host: {{ coalesce(values=[env_host, config_host, default_host]) }}
{# Output: Host: prod.example.com #}
```

#### `ternary(condition, true_val, false_val)`

Ternary operator - return one value based on a condition.

**Arguments:**
- `condition` (required): Boolean condition to evaluate
- `true_val` (required): Value to return if condition is true
- `false_val` (required): Value to return if condition is false

**Returns:** `true_val` if condition is truthy, otherwise `false_val`

**Example:**
```jinja
{# Simple ternary #}
{{ ternary(condition=true, true_val="Yes", false_val="No") }}
{# Output: Yes #}

{# With comparison #}
{% set score = 85 %}
Result: {{ ternary(condition=score >= 60, true_val="Pass", false_val="Fail") }}
{# Output: Result: Pass #}

{# Status indicator #}
{% set cpu_usage = 75 %}
Status: {{ ternary(
  condition=cpu_usage > 90,
  true_val="Critical",
  false_val="Normal"
) }}
```

#### `in_range(value, min, max)`

Check if a numeric value is within a range (inclusive).

**Arguments:**
- `value` (required): Numeric value to check
- `min` (required): Minimum value (inclusive)
- `max` (required): Maximum value (inclusive)

**Returns:** `true` if min <= value <= max, `false` otherwise

**Example:**
```jinja
{# Check if in range #}
{{ in_range(value=50, min=0, max=100) }}
{# Output: true #}

{# Validate port number #}
{% set port = 8080 %}
{% if in_range(value=port, min=1024, max=65535) %}
  Valid port number
{% else %}
  Invalid port number
{% endif %}

{# Temperature range check #}
{% set temp = 22 %}
Comfortable: {{ in_range(value=temp, min=18, max=25) }}

{# Resource usage validation #}
{% set cpu = 75.5 %}
{% if in_range(value=cpu, min=0, max=80) %}
  CPU usage normal
{% else %}
  CPU usage high
{% endif %}
```

### Math Functions

Perform mathematical calculations and operations.

#### `min(a, b)`

Return the minimum of two values.

**Arguments:**
- `a` (required): First number
- `b` (required): Second number

**Returns:** The smaller of the two values

**Example:**
```jinja
{# Find minimum #}
{{ min(a=10, b=20) }}
{# Output: 10 #}

{# With variables #}
{% set cpu1 = 45.2 %}
{% set cpu2 = 38.7 %}
Lowest CPU: {{ min(a=cpu1, b=cpu2) }}%
```

#### `max(a, b)`

Return the maximum of two values.

**Arguments:**
- `a` (required): First number
- `b` (required): Second number

**Returns:** The larger of the two values

**Example:**
```jinja
{# Find maximum #}
{{ max(a=10, b=20) }}
{# Output: 20 #}

{# With variables #}
{% set memory1 = 2048 %}
{% set memory2 = 4096 %}
Peak memory: {{ max(a=memory1, b=memory2) }}MB
```

#### `abs(number)`

Return the absolute value of a number.

**Arguments:**
- `number` (required): Number to get absolute value of

**Returns:** The absolute value (always positive)

**Example:**
```jinja
{# Absolute value #}
{{ abs(number=-42) }}
{# Output: 42 #}

{# Temperature difference #}
{% set temp1 = 25 %}
{% set temp2 = 18 %}
Difference: {{ abs(number=temp1 - temp2) }}°C
```

#### `round(number, decimals=0)`

Round a number to N decimal places.

**Arguments:**
- `number` (required): Number to round
- `decimals` (optional): Number of decimal places (default: 0)

**Returns:** The number rounded to the specified decimal places

**Example:**
```jinja
{# Round to nearest integer #}
{{ round(number=3.7) }}
{# Output: 4 #}

{# Round to 2 decimal places #}
{{ round(number=3.14159, decimals=2) }}
{# Output: 3.14 #}

{# Price calculation #}
{% set price = 19.999 %}
Price: ${{ round(number=price, decimals=2) }}
```

#### `ceil(number)`

Round up to the nearest integer.

**Arguments:**
- `number` (required): Number to round up

**Returns:** The smallest integer greater than or equal to the number

**Example:**
```jinja
{# Round up #}
{{ ceil(number=3.1) }}
{# Output: 4 #}

{# Calculate required servers #}
{% set users = 150 %}
{% set users_per_server = 50 %}
Servers needed: {{ ceil(number=users / users_per_server) }}
```

#### `floor(number)`

Round down to the nearest integer.

**Arguments:**
- `number` (required): Number to round down

**Returns:** The largest integer less than or equal to the number

**Example:**
```jinja
{# Round down #}
{{ floor(number=3.9) }}
{# Output: 3 #}

{# Calculate filled pages #}
{% set items = 47 %}
{% set items_per_page = 10 %}
Full pages: {{ floor(number=items / items_per_page) }}
```

#### `percentage(value, total)`

Calculate percentage.

**Arguments:**
- `value` (required): The part value
- `total` (required): The total/whole value

**Returns:** The percentage (0-100)

**Example:**
```jinja
{# Calculate percentage #}
{{ percentage(value=25, total=100) }}
{# Output: 25.0 #}

{# Progress calculation #}
{% set completed = 7 %}
{% set total_tasks = 10 %}
Progress: {{ round(number=percentage(value=completed, total=total_tasks), decimals=1) }}%

{# Disk usage #}
{% set used = 450 %}
{% set capacity = 500 %}
Disk usage: {{ round(number=percentage(value=used, total=capacity), decimals=2) }}%
```

### Statistical Functions

Calculate statistics on numeric arrays.

#### `array_sum(array)`

Calculate the sum of all values in an array.

**Arguments:**
- `array` (required): Array of numbers to sum

**Returns:** Sum of all values (integer if no decimals, float otherwise)

**Example:**
```jinja
{# Sum of integers #}
{% set numbers = [1, 2, 3, 4, 5] %}
Total: {{ array_sum(array=numbers) }}
{# Output: Total: 15 #}

{# Sum of prices #}
{% set prices = [10.5, 20.25, 5.75] %}
Total: ${{ array_sum(array=prices) }}
{# Output: Total: $36.5 #}

{# Calculate total disk usage #}
{% set sizes = [1024, 2048, 512, 4096] %}
Total MB: {{ array_sum(array=sizes) }}
```

#### `array_avg(array)`

Calculate the average (mean) of all values in an array.

**Arguments:**
- `array` (required): Array of numbers

**Returns:** Arithmetic mean of all values (0 for empty arrays)

**Example:**
```jinja
{# Average score #}
{% set scores = [85, 90, 78, 92, 88] %}
Average: {{ array_avg(array=scores) }}
{# Output: Average: 86.6 #}

{# CPU usage over time #}
{% set cpu = [45.2, 52.1, 48.7, 50.3] %}
Avg CPU: {{ array_avg(array=cpu) }}%

{# Empty array handling #}
{% set empty = [] %}
Default: {{ array_avg(array=empty) }}
{# Output: Default: 0 #}
```

#### `array_median(array)`

Calculate the median value of an array.

**Arguments:**
- `array` (required): Array of numbers

**Returns:** Middle value for odd-length arrays, average of two middle values for even-length arrays

**Example:**
```jinja
{# Median of odd-length array #}
{% set nums = [1, 3, 5, 7, 9] %}
Median: {{ array_median(array=nums) }}
{# Output: Median: 5 #}

{# Median of even-length array #}
{% set nums = [1, 2, 3, 4] %}
Median: {{ array_median(array=nums) }}
{# Output: Median: 2.5 #}

{# Response time analysis #}
{% set response_times = [120, 95, 150, 105, 130] %}
Median response: {{ array_median(array=response_times) }}ms
```

#### `array_min(array)`

Find the minimum value in an array.

**Arguments:**
- `array` (required): Array of numbers

**Returns:** Smallest value in the array

**Example:**
```jinja
{# Find minimum #}
{% set numbers = [42, 17, 99, 8, 55] %}
Minimum: {{ array_min(array=numbers) }}
{# Output: Minimum: 8 #}

{# Lowest price #}
{% set prices = [10.99, 5.49, 15.99, 7.25] %}
Best deal: ${{ array_min(array=prices) }}

{# Temperature range #}
{% set temps = [-5, 3, 8, -2, 12] %}
Low: {{ array_min(array=temps) }}°C
```

#### `array_max(array)`

Find the maximum value in an array.

**Arguments:**
- `array` (required): Array of numbers

**Returns:** Largest value in the array

**Example:**
```jinja
{# Find maximum #}
{% set numbers = [42, 17, 99, 8, 55] %}
Maximum: {{ array_max(array=numbers) }}
{# Output: Maximum: 99 #}

{# Peak memory usage #}
{% set memory = [512, 768, 1024, 896] %}
Peak: {{ array_max(array=memory) }}MB

{# Temperature range #}
{% set temps = [-5, 3, 8, -2, 12] %}
High: {{ array_max(array=temps) }}°C
```

**Real-world use case - Resource allocation:**
```jinja
{% set cpu_usage = [45, 62, 78, 55, 91, 67] %}
{% set mem_usage = [2048, 3072, 4096, 2560] %}

CPU Statistics:
  Average: {{ array_avg(array=cpu_usage) }}%
  Peak: {{ array_max(array=cpu_usage) }}%
  Median: {{ array_median(array=cpu_usage) }}%

Memory Statistics:
  Total: {{ array_sum(array=mem_usage) }}MB
  Average: {{ array_avg(array=mem_usage) }}MB
  Peak: {{ array_max(array=mem_usage) }}MB

{% if array_max(array=cpu_usage) > 90 %}
Alert: High CPU usage detected!
{% endif %}
```

### Array Manipulation Functions

Utility functions for working with arrays.

#### `array_count(array)`

Count the number of items in an array (alias for length).

**Arguments:**
- `array` (required): Array to count

**Returns:** Number of items in the array

**Example:**
```jinja
{# Count items #}
{% set items = ["apple", "banana", "cherry"] %}
Total: {{ array_count(array=items) }}
{# Output: Total: 3 #}

{# Empty array #}
{% set empty = [] %}
Count: {{ array_count(array=empty) }}
{# Output: Count: 0 #}

{# Conditional based on count #}
{% set tasks = ["task1", "task2", "task3"] %}
{% if array_count(array=tasks) > 2 %}
Multiple tasks pending
{% endif %}
```

#### `array_chunk(array, size)`

Split an array into chunks of specified size.

**Arguments:**
- `array` (required): Array to split
- `size` (required): Size of each chunk (must be > 0)

**Returns:** Array of arrays, where each sub-array has at most `size` elements

**Example:**
```jinja
{# Split into pairs #}
{% set nums = [1, 2, 3, 4, 5, 6] %}
{% for chunk in array_chunk(array=nums, size=2) %}
  Chunk: {{ chunk }}
{% endfor %}
{# Output:
   Chunk: [1, 2]
   Chunk: [3, 4]
   Chunk: [5, 6]
#}

{# Pagination #}
{% set items = ["a", "b", "c", "d", "e", "f", "g"] %}
{% for page in array_chunk(array=items, size=3) %}
  Page {{ loop.index }}: {{ page | join(", ") }}
{% endfor %}
{# Output:
   Page 1: a, b, c
   Page 2: d, e, f
   Page 3: g
#}

{# Grid layout #}
{% set products = ["Product1", "Product2", "Product3", "Product4"] %}
{% for row in array_chunk(array=products, size=2) %}
<div class="row">
  {% for item in row %}
  <div class="col">{{ item }}</div>
  {% endfor %}
</div>
{% endfor %}
```

#### `array_zip(array1, array2)`

Combine two arrays into pairs (like a zipper).

**Arguments:**
- `array1` (required): First array
- `array2` (required): Second array

**Returns:** Array of two-element arrays (pairs). Length is the minimum of the two input arrays.

**Example:**
```jinja
{# Combine keys and values #}
{% set keys = ["name", "age", "city"] %}
{% set values = ["Alice", 30, "NYC"] %}
{% for pair in array_zip(array1=keys, array2=values) %}
  {{ pair[0] }}: {{ pair[1] }}
{% endfor %}
{# Output:
   name: Alice
   age: 30
   city: NYC
#}

{# Configuration mapping #}
{% set env_vars = ["HOST", "PORT", "DEBUG"] %}
{% set defaults = ["localhost", "8080", "false"] %}
{% for pair in array_zip(array1=env_vars, array2=defaults) %}
{{ pair[0] }}={{ pair[1] }}
{% endfor %}

{# Different lengths - stops at shorter #}
{% set a = [1, 2, 3, 4] %}
{% set b = ["a", "b"] %}
{{ array_zip(array1=a, array2=b) }}
{# Output: [[1, "a"], [2, "b"]] #}
```

**Real-world use case - Environment variables with defaults:**
```jinja
{% set var_names = ["DATABASE_HOST", "DATABASE_PORT", "DATABASE_NAME", "DATABASE_USER"] %}
{% set defaults = ["localhost", "5432", "myapp", "postgres"] %}

# Database configuration
{% for pair in array_zip(array1=var_names, array2=defaults) %}
export {{ pair[0] }}="${{ pair[0] }}:-{{ pair[1] }}}"
{% endfor %}

{# Output:
export DATABASE_HOST="${DATABASE_HOST:-localhost}"
export DATABASE_PORT="${DATABASE_PORT:-5432}"
export DATABASE_NAME="${DATABASE_NAME:-myapp}"
export DATABASE_USER="${DATABASE_USER:-postgres}"
#}
```

#### `array_sort_by(array, key)`

Sort an array of objects by a specified key.

**Arguments:**
- `array` (required): Array of objects to sort
- `key` (required): Object key name to sort by

**Returns:** New array sorted by the key value (ascending order)

**Example:**
```jinja
{# Sort users by age #}
{% set users = [
  {"name": "Alice", "age": 30},
  {"name": "Bob", "age": 25},
  {"name": "Charlie", "age": 35}
] %}
{% for user in array_sort_by(array=users, key="age") %}
  {{ user.name }}: {{ user.age }}
{% endfor %}
{# Output:
   Bob: 25
   Alice: 30
   Charlie: 35
#}

{# Sort by string key #}
{% set products = [
  {"name": "Zebra Toy", "price": 15},
  {"name": "Apple Pie", "price": 10},
  {"name": "Mango Juice", "price": 12}
] %}
{% for product in array_sort_by(array=products, key="name") %}
  {{ product.name }}
{% endfor %}
{# Output: Apple Pie, Mango Juice, Zebra Toy #}
```

#### `array_group_by(array, key)`

Group array items by a key value.

**Arguments:**
- `array` (required): Array of objects to group
- `key` (required): Object key name to group by

**Returns:** Object with keys as group names and values as arrays of grouped items

**Example:**
```jinja
{# Group users by department #}
{% set users = [
  {"name": "Alice", "dept": "Engineering"},
  {"name": "Bob", "dept": "Sales"},
  {"name": "Charlie", "dept": "Engineering"}
] %}
{% set grouped = array_group_by(array=users, key="dept") %}
{% for dept, members in grouped | items %}
  {{ dept }}:
  {% for user in members %}
    - {{ user.name }}
  {% endfor %}
{% endfor %}
{# Output:
   Engineering:
     - Alice
     - Charlie
   Sales:
     - Bob
#}

{# Group by numeric value #}
{% set tasks = [
  {"name": "Task1", "priority": 1},
  {"name": "Task2", "priority": 2},
  {"name": "Task3", "priority": 1}
] %}
{% set by_priority = array_group_by(array=tasks, key="priority") %}
High priority: {{ by_priority["1"] | length }} tasks
```

#### `array_unique(array)`

Remove duplicate values from an array.

**Arguments:**
- `array` (required): Array to deduplicate

**Returns:** New array with duplicates removed (first occurrence kept)

**Example:**
```jinja
{# Remove duplicate numbers #}
{% set nums = [1, 2, 2, 3, 1, 4, 3, 5] %}
{{ array_unique(array=nums) }}
{# Output: [1, 2, 3, 4, 5] #}

{# Unique tags #}
{% set tags = ["docker", "kubernetes", "docker", "helm", "kubernetes"] %}
Unique tags: {{ array_unique(array=tags) | join(", ") }}
{# Output: Unique tags: docker, kubernetes, helm #}

{# Use in conditional #}
{% set all_tags = ["prod", "dev", "prod", "staging", "dev"] %}
{% set unique_envs = array_unique(array=all_tags) %}
{% if unique_envs | length > 2 %}
  Multiple environments detected
{% endif %}
```

#### `array_flatten(array)`

Flatten nested arrays by one level.

**Arguments:**
- `array` (required): Array with nested arrays

**Returns:** New array with nested arrays flattened one level

**Example:**
```jinja
{# Flatten nested arrays #}
{% set nested = [[1, 2], [3, 4], [5]] %}
{{ array_flatten(array=nested) }}
{# Output: [1, 2, 3, 4, 5] #}

{# Mixed with non-arrays #}
{% set mixed = [["a", "b"], "c", ["d", "e"]] %}
{{ array_flatten(array=mixed) }}
{# Output: ["a", "b", "c", "d", "e"] #}

{# Only flattens one level #}
{% set deep = [[1, [2, 3]], [4]] %}
{{ array_flatten(array=deep) }}
{# Output: [1, [2, 3], 4] #}

{# Collect values from multiple sources #}
{% set server1_ips = ["10.0.1.1", "10.0.1.2"] %}
{% set server2_ips = ["10.0.2.1", "10.0.2.2"] %}
{% set server3_ips = ["10.0.3.1"] %}
{% set all_ips = array_flatten(array=[server1_ips, server2_ips, server3_ips]) %}
Total IPs: {{ all_ips | length }}
```

**Real-world use case - Task management dashboard:**
```jinja
{% set tasks = [
  {"name": "Fix bug #123", "status": "done", "assignee": "Alice"},
  {"name": "Deploy v2.0", "status": "in_progress", "assignee": "Bob"},
  {"name": "Write docs", "status": "done", "assignee": "Alice"},
  {"name": "Code review", "status": "pending", "assignee": "Charlie"}
] %}

{# Group by status #}
{% set by_status = array_group_by(array=tasks, key="status") %}

Task Status Dashboard:
{% for status, items in by_status | items %}
{{ status | upper }} ({{ items | length }} tasks):
  {% for task in array_sort_by(array=items, key="name") %}
  - {{ task.name }} ({{ task.assignee }})
  {% endfor %}
{% endfor %}

{# Get unique assignees #}
{% set all_assignees = [] %}
{% for task in tasks %}
  {% set _ = all_assignees.append(task.assignee) %}
{% endfor %}
Unique assignees: {{ array_unique(array=all_assignees) | join(", ") }}
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
