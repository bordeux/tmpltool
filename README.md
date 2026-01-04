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
- [IDE Integration](#ide-integration)
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
- `--ide <FORMAT>` - Output function metadata for IDE integration (json, yaml, or toml)
  - Prints all available functions with descriptions, arguments, return types, and examples
  - Exits immediately after printing metadata (does not render templates)
  - Useful for building IDE plugins, autocomplete, and documentation generators

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

### Is Syntax (Validation & Checks)

The `is` syntax provides readable conditionals for validation and type checking. All is-functions support both function syntax and the more readable "is" syntax:

| Test | Function Equivalent | Description |
|------|---------------------|-------------|
| `{% if x is email %}` | `is_email(string=x)` | Valid email format |
| `{% if x is url %}` | `is_url(string=x)` | Valid URL format |
| `{% if x is ip %}` | `is_ip(string=x)` | Valid IPv4/IPv6 address |
| `{% if x is uuid %}` | `is_uuid(string=x)` | Valid UUID format |
| `{% if y is leap_year %}` | `is_leap_year(year=y)` | Year is a leap year |
| `{% if p is port_available %}` | `is_port_available(port=p)` | Port is free to use |
| `{% if f is file %}` | `is_file(path=f)` | Path is an existing file |
| `{% if d is dir %}` | `is_dir(path=d)` | Path is a directory |
| `{% if s is symlink %}` | `is_symlink(path=s)` | Path is a symbolic link |

**Examples:**
```jinja
{# Validate user input #}
{% if user_email is email %}
  Valid email: {{ user_email }}
{% else %}
  Invalid email format
{% endif %}

{# Check filesystem #}
{% if "config.json" is file %}
  {% set config = read_json_file(path="config.json") %}
{% endif %}

{# Port availability #}
{% if 8080 is port_available %}
  port: 8080
{% elif 3000 is port_available %}
  port: 3000
{% endif %}

{# Negation with "is not" #}
{% if user_input is not uuid %}
  Warning: Invalid ID format
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

**String manipulation (function + filter syntax):**

All string filters support both function and filter syntax:

- `slugify(string)` / `| slugify` - Convert to URL-friendly slug (e.g., "Hello World" → "hello-world")
- `indent(string, spaces=4)` / `| indent(spaces=4)` - Indent text by N spaces (useful for YAML/configs)
- `dedent(string)` / `| dedent` - Remove common leading whitespace
- `quote(string, style="double")` / `| quote(style="double")` - Quote string (single/double/backtick)
- `escape_quotes(string)` / `| escape_quotes` - Escape quotes in string
- `to_snake_case(string)` / `| to_snake_case` - Convert to snake_case (e.g., "HelloWorld" → "hello_world")
- `to_camel_case(string)` / `| to_camel_case` - Convert to camelCase (e.g., "hello_world" → "helloWorld")
- `to_pascal_case(string)` / `| to_pascal_case` - Convert to PascalCase (e.g., "hello_world" → "HelloWorld")
- `to_kebab_case(string)` / `| to_kebab_case` - Convert to kebab-case (e.g., "HelloWorld" → "hello-world")
- `pad_left(string, length, char=" ")` / `| pad_left(length, char=" ")` - Pad string on left
- `pad_right(string, length, char=" ")` / `| pad_right(length, char=" ")` - Pad string on right
- `repeat(string, count)` / `| repeat(count)` - Repeat string N times
- `reverse(string)` / `| reverse` - Reverse string

**Formatting (function + filter syntax):**
- `filesizeformat(bytes)` / `| filesizeformat` - Format bytes (e.g., "1.5 KB")
- `urlencode(string)` / `| urlencode` - URL encoding (percent-encoding)

**Examples:**
```
{# Case conversion - both syntaxes work #}
{{ "hello_world" | to_camel_case }}           {# Output: helloWorld #}
{{ to_camel_case(string="hello_world") }}     {# Output: helloWorld #}

{{ "HelloWorld" | to_snake_case }}            {# Output: hello_world #}
{{ to_snake_case(string="HelloWorld") }}      {# Output: hello_world #}

{# Slugify #}
{{ "Hello World!" | slugify }}                {# Output: hello-world #}
{{ slugify(string="Hello World!") }}          {# Output: hello-world #}

{# Indentation for configs #}
{{ "host: localhost\nport: 8080" | indent(spaces=2) }}
{{ indent(string="host: localhost", spaces=4) }}

{# Padding for alignment #}
{{ "1" | pad_left(length=4, char="0") }}      {# Output: 0001 #}
{{ pad_left(string="5", length=3, char="0") }} {# Output: 005 #}

{# Creating separators #}
{{ "=" | repeat(count=40) }}                  {# Output: ======================================== #}
{{ repeat(string="-", count=5) }}             {# Output: ----- #}

{# Quoting #}
{{ "hello" | quote(style="single") }}         {# Output: 'hello' #}
{{ quote(string="world", style="backtick") }} {# Output: `world` #}

{# Chaining filters #}
{{ "hello_world" | to_pascal_case | reverse }}  {# Output: dlroWolleH #}

{# Formatting - both syntaxes work #}
{{ 1048576 | filesizeformat }}                {# Output: 1 MB #}
{{ filesizeformat(bytes=1048576) }}           {# Output: 1 MB #}

{{ "hello world" | urlencode }}               {# Output: hello%20world #}
{{ urlencode(string="hello world") }}         {# Output: hello%20world #}
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

Hash functions support both **function syntax** and **filter syntax**. Both produce identical results.

#### `md5(string)` / `| md5`

Calculate MD5 hash of a string.

```jinja
{# Function syntax #}
Checksum: {{ md5(string="hello world") }}
{# Output: 5eb63bbbe01eeed093cb22bb8f5acdc3 #}

{# Filter syntax #}
Checksum: {{ "hello world" | md5 }}
{# Output: 5eb63bbbe01eeed093cb22bb8f5acdc3 #}
```

#### `sha1(string)` / `| sha1`

Calculate SHA1 hash of a string.

```jinja
{# Function syntax #}
Hash: {{ sha1(string="tmpltool") }}

{# Filter syntax #}
Hash: {{ "tmpltool" | sha1 }}
{# Output: c054a2a60ca2fe935ea1056bd90386194116f14f #}
```

#### `sha256(string)` / `| sha256`

Calculate SHA256 hash (recommended for password hashing).

```jinja
{# Function syntax #}
{% set password = get_env(name="PASSWORD", default="secret") %}
Password hash: {{ sha256(string=password) }}

{# Filter syntax #}
Password hash: {{ password | sha256 }}
```

#### `sha512(string)` / `| sha512`

Calculate SHA512 hash (most secure).

```jinja
{# Function syntax #}
Secure hash: {{ sha512(string="secure-data") }}

{# Filter syntax #}
Secure hash: {{ "secure-data" | sha512 }}
```

#### Chaining Hash Filters

Filter syntax enables chaining multiple operations:

```jinja
{# Chain hash with encoding #}
{{ "hello" | sha256 | md5 }}

{# Equivalent function syntax (nested) #}
{{ md5(string=sha256(string="hello")) }}
```

**Important:** These hash functions are for checksums and general-purpose hashing. For production password storage, use dedicated password hashing libraries with salt and proper key derivation functions (bcrypt, argon2, etc.).

#### `uuid(version)`

Generate a UUID (Universally Unique Identifier) with configurable version.

**Arguments:**
- `version` (optional) - UUID version to generate: `"v4"` (default) or `"v7"`

**UUID Versions:**
- `v4` - Random UUID (default) - suitable for most use cases
- `v7` - Time-ordered UUID - sortable by creation time, ideal for database primary keys

**Examples:**
```
{# Default v4 (random) #}
Request ID: {{ uuid() }}
Session ID: {{ uuid(version="v4") }}

{# v7 (time-ordered, sortable) #}
Database ID: {{ uuid(version="v7") }}
Event ID: {{ uuid(version="v7") }}
{# v7 UUIDs are lexicographically sortable by creation time #}
```

**When to use which version:**
- Use **v4** for general-purpose unique identifiers where ordering doesn't matter
- Use **v7** for database primary keys, event logs, or anywhere you need time-based sorting

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

### Date/Time Functions

Work with dates, times, and timestamps. All functions use Unix timestamps (seconds since epoch) for consistent timezone-independent representation.

#### `now(format)`

Get the current Unix timestamp, or formatted date string.

**Arguments:**
- `format` (optional) - Format string. If provided, returns formatted string.

**Returns:** Unix timestamp (integer) by default, or formatted string if `format` is provided.

**Examples:**
```
{# Get Unix timestamp (default) #}
Timestamp: {{ now() }}
{# Output: 1704067200 #}

{# Use with format_date for formatting #}
{{ format_date(timestamp=now(), format="%Y-%m-%d %H:%M:%S") }}
{# Output: 2024-12-31 12:34:56 #}

{# Or use format parameter directly #}
{{ now(format="%Y-%m-%d %H:%M:%S") }}
{# Output: 2024-12-31 12:34:56 #}

{# Date only #}
{{ now(format="%Y-%m-%d") }}
{# Output: 2024-12-31 #}
```

#### `format_date(timestamp, format)` / `| format_date`

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

**Function syntax:**
```
{% set ts = 1704067200 %}
ISO date: {{ format_date(timestamp=ts, format="%Y-%m-%d") }}
{# Output: 2024-01-01 #}

Full: {{ format_date(timestamp=ts, format="%B %d, %Y at %I:%M %p") }}
{# Output: January 01, 2024 at 12:00 AM #}
```

**Filter syntax:**
```
{% set ts = 1704067200 %}
{{ ts | format_date(format="%Y-%m-%d") }}
{# Output: 2024-01-01 #}

{# Chain with now() #}
{{ now() | format_date(format="%B %d, %Y") }}
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

#### `get_year(timestamp)` / `| get_year`

Extract the year from a Unix timestamp.

**Arguments:**
- `timestamp` (required) - Unix timestamp in seconds

**Returns:** Integer (4-digit year)

**Function syntax:**
```
{{ get_year(timestamp=1704067200) }}
{# Output: 2024 #}
```

**Filter syntax:**
```
{{ 1704067200 | get_year }}
{{ now() | get_year }}
```

#### `get_month(timestamp)` / `| get_month`

Extract the month from a Unix timestamp.

**Arguments:**
- `timestamp` (required) - Unix timestamp in seconds

**Returns:** Integer (1-12)

**Function syntax:**
```
{{ get_month(timestamp=1704067200) }}
{# Output: 1 #}
```

**Filter syntax:**
```
{{ 1704067200 | get_month }}
{{ now() | get_month }}
```

#### `get_day(timestamp)` / `| get_day`

Extract the day from a Unix timestamp.

**Arguments:**
- `timestamp` (required) - Unix timestamp in seconds

**Returns:** Integer (1-31)

**Function syntax:**
```
{{ get_day(timestamp=1704067200) }}
{# Output: 1 #}
```

**Filter syntax:**
```
{{ 1704067200 | get_day }}
{{ now() | get_day }}
```

#### `get_hour(timestamp)` / `| get_hour`

Extract the hour from a Unix timestamp.

**Arguments:**
- `timestamp` (required) - Unix timestamp in seconds

**Returns:** Integer (0-23)

**Function syntax:**
```
{{ get_hour(timestamp=1704110400) }}
{# Output: 12 #}
```

**Filter syntax:**
```
{{ 1704110400 | get_hour }}
{{ now() | get_hour }}
```

#### `get_minute(timestamp)` / `| get_minute`

Extract the minute from a Unix timestamp.

**Arguments:**
- `timestamp` (required) - Unix timestamp in seconds

**Returns:** Integer (0-59)

**Function syntax:**
```
{{ get_minute(timestamp=1704068700) }}
{# Output: 25 #}
```

**Filter syntax:**
```
{{ 1704068700 | get_minute }}
{{ now() | get_minute }}
```

#### `get_second(timestamp)` / `| get_second`

Extract the second from a Unix timestamp.

**Arguments:**
- `timestamp` (required) - Unix timestamp in seconds

**Returns:** Integer (0-59)

**Function syntax:**
```
{{ get_second(timestamp=1704067245) }}
{# Output: 45 #}
```

**Filter syntax:**
```
{{ 1704067245 | get_second }}
{{ now() | get_second }}
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

#### `is_leap_year(year)` / `{% if year is leap_year %}`

Check if a year is a leap year. Supports both function syntax and "is" test syntax.

**Function Syntax Arguments:**
- `year` (required) - Year to check (4-digit integer)

**Is-Test Syntax:**
- The value must be an integer or a string that can be parsed as an integer

**Returns:** Boolean (true if leap year, false otherwise)

**Examples:**
```jinja
{# Function syntax #}
{% if is_leap_year(year=2024) %}
2024 is a leap year
{% endif %}

{# Is-test syntax (preferred for readability) #}
{% if 2024 is leap_year %}
2024 is a leap year
{% endif %}

{# With variables #}
{% set years = [2020, 2021, 2022, 2023, 2024] %}
{% for year in years %}
{{ year }}: {% if year is leap_year %}Leap{% else %}Regular{% endif %}
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

{# Format with format_date function #}
{% set timestamp = file_modified(path="README.md") %}
Last updated: {{ format_date(timestamp=timestamp, format="%Y-%m-%d %H:%M:%S") }}

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
Generated: {{ now(format="%Y-%m-%d %H:%M:%S") }}

## Source Files
{% set rs_files = glob(pattern="src/**/*.rs") %}
Total Rust files: {{ rs_files | length }}

{% for file in rs_files %}
- {{ file }}
  Size: {{ file_size(path=file) | filesizeformat }}
  Modified: {{ format_date(timestamp=file_modified(path=file), format="%Y-%m-%d") }}
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

#### `basename(path)` / `| basename`

Extract the filename from a path.

**Arguments:**
- `path` (required) - File path

**Returns:** Filename (last component of the path)

**Function syntax:**
```jinja
{{ basename(path="/path/to/file.txt") }}
{# Output: file.txt #}
```

**Filter syntax:**
```jinja
{{ "/path/to/file.txt" | basename }}
{# Output: file.txt #}

{# Use with glob results #}
{% for file in glob(pattern="src/**/*.rs") %}
  - {{ file | basename }}
{% endfor %}
```

#### `dirname(path)` / `| dirname`

Extract the directory portion from a path.

**Arguments:**
- `path` (required) - File path

**Returns:** Directory path (all components except the last)

**Function syntax:**
```jinja
{{ dirname(path="/path/to/file.txt") }}
{# Output: /path/to #}
```

**Filter syntax:**
```jinja
{{ "/path/to/file.txt" | dirname }}
{# Output: /path/to #}

{# Get parent directory #}
{% set file_path = "config/app/settings.json" %}
Config directory: {{ file_path | dirname }}
{# Output: config/app #}
```

#### `file_extension(path)` / `| file_extension`

Extract the file extension from a path.

**Arguments:**
- `path` (required) - File path

**Returns:** File extension without the dot (empty string if no extension)

**Function syntax:**
```jinja
{{ file_extension(path="document.pdf") }}
{# Output: pdf #}

{{ file_extension(path="/path/to/file.tar.gz") }}
{# Output: gz #}
```

**Filter syntax:**
```jinja
{{ "document.pdf" | file_extension }}
{# Output: pdf #}

{# Chain with basename #}
{{ "/path/to/file.tar.gz" | basename | file_extension }}
{# Output: gz #}

{# Group files by extension #}
{% for file in glob(pattern="docs/*") %}
  {% if file | file_extension == "md" %}
    - Markdown: {{ file }}
  {% endif %}
{% endfor %}
```

#### `join_path(parts)` / `| join_path`

Join multiple path components into a single path.

**Arguments:**
- `parts` (required) - Array of path components

**Returns:** Joined path string

**Function syntax:**
```jinja
{{ join_path(parts=["path", "to", "file.txt"]) }}
{# Output: path/to/file.txt #}

{{ join_path(parts=["/home", "user", "documents"]) }}
{# Output: /home/user/documents #}
```

**Filter syntax:**
```jinja
{{ ["path", "to", "file.txt"] | join_path }}
{# Output: path/to/file.txt #}

{# Build dynamic paths #}
{% set path_parts = ["config", "production", "settings.json"] %}
{{ path_parts | join_path }}
{# Output: config/production/settings.json #}
```

#### `normalize_path(path)` / `| normalize_path`

Normalize a path by resolving `.` (current directory) and `..` (parent directory) components.

**Arguments:**
- `path` (required) - Path to normalize

**Returns:** Normalized path string

**Function syntax:**
```jinja
{{ normalize_path(path="./foo/bar") }}
{# Output: foo/bar #}

{{ normalize_path(path="a/b/c/../../d") }}
{# Output: a/d #}
```

**Filter syntax:**
```jinja
{{ "./foo/bar" | normalize_path }}
{# Output: foo/bar #}

{# Clean up path components #}
{% set messy_path = "./config/../data/./files.txt" %}
{{ messy_path | normalize_path }}
{# Output: data/files.txt #}
```

#### `is_file(path)` / `{% if path is file %}`

Check if a path exists and is a file. Supports both function syntax and "is" test syntax.

**Function Syntax Arguments:**
- `path` (required) - Path to check

**Is-Test Syntax:**
- The value must be a string representing a file path

**Returns:** Boolean (true if path exists and is a file)

**Examples:**
```jinja
{# Function syntax #}
{% if is_file(path="config.txt") %}
  Config file found!
{% endif %}

{# Is-test syntax (preferred for readability) #}
{% if "config.txt" is file %}
  Config file found!
{% endif %}

{# With variables #}
{% set config_path = "config.json" %}
{% if config_path is file %}
  {{ read_file(path=config_path) }}
{% endif %}
```

#### `is_dir(path)` / `{% if path is dir %}`

Check if a path exists and is a directory. Supports both function syntax and "is" test syntax.

**Function Syntax Arguments:**
- `path` (required) - Path to check

**Is-Test Syntax:**
- The value must be a string representing a directory path

**Returns:** Boolean (true if path exists and is a directory)

**Examples:**
```jinja
{# Function syntax #}
{% if is_dir(path="src") %}
  Source directory exists
{% endif %}

{# Is-test syntax (preferred for readability) #}
{% if "src" is dir %}
  Source directory exists
{% endif %}

{# With variables #}
{% set test_dir = "tests" %}
{% if test_dir is dir %}
  {% set test_files = glob(pattern="tests/**/*.rs") %}
  Found {{ test_files | length }} test files
{% endif %}
```

#### `is_symlink(path)` / `{% if path is symlink %}`

Check if a path is a symbolic link. Supports both function syntax and "is" test syntax.

**Function Syntax Arguments:**
- `path` (required) - Path to check

**Is-Test Syntax:**
- The value must be a string representing a path

**Returns:** Boolean (true if path is a symlink)

**Examples:**
```jinja
{# Function syntax #}
{% if is_symlink(path="current") %}
  'current' is a symbolic link
{% endif %}

{# Is-test syntax (preferred for readability) #}
{% if "current" is symlink %}
  'current' is a symbolic link
{% endif %}

{# With variables #}
{% set link_path = "latest" %}
{% if link_path is symlink %}
  Following symlink...
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

#### `parse_json(string)` / `| parse_json`

Parse a JSON string into an object. Available as both function and filter.

**Arguments:**
- `string` (required for function syntax) - JSON string to parse

**Returns:** Parsed JSON object

**Function Syntax:**
```jinja
{% set config = parse_json(string='{"name": "myapp", "port": 8080, "debug": true}') %}
Application: {{ config.name }}
Port: {{ config.port }}
Debug mode: {{ config.debug }}
```

**Filter Syntax:**
```jinja
{% set json_str = '{"name": "Alice", "age": 30}' %}
{% set user = json_str | parse_json %}
Name: {{ user.name }}, Age: {{ user.age }}
```

#### `parse_yaml(string)` / `| parse_yaml`

Parse a YAML string into an object. Available as both function and filter.

**Arguments:**
- `string` (required for function syntax) - YAML string to parse

**Returns:** Parsed YAML object

**Function Syntax:**
```jinja
{% set data = parse_yaml(string="
name: myapp
settings:
  theme: dark
  notifications: true
") %}
App: {{ data.name }}
Theme: {{ data.settings.theme }}
```

**Filter Syntax:**
```jinja
{% set yaml_str = "name: Bob
age: 25" %}
{% set user = yaml_str | parse_yaml %}
Name: {{ user.name }}, Age: {{ user.age }}
```

#### `parse_toml(string)` / `| parse_toml`

Parse a TOML string into an object. Available as both function and filter.

**Arguments:**
- `string` (required for function syntax) - TOML string to parse

**Returns:** Parsed TOML object

**Function Syntax:**
```jinja
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

**Filter Syntax:**
```jinja
{% set toml_str = 'name = "Charlie"
age = 35' %}
{% set user = toml_str | parse_toml %}
Name: {{ user.name }}, Age: {{ user.age }}
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

#### `to_json(object, pretty)` / `| to_json`

Convert an object to a JSON string. Available as both function and filter.

**Arguments:**
- `object` (required for function syntax) - Object/value to convert to JSON
- `pretty` (optional) - Enable pretty-printing with indentation (default: false)

**Returns:** JSON string

**Function Syntax:**
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
```

**Filter Syntax:**
```jinja
{# Simple filter syntax #}
{% set config = {"host": "localhost", "port": 8080} %}
{{ config | to_json }}
{# Output: {"host":"localhost","port":8080} #}

{# Pretty-printed with filter #}
{{ config | to_json(pretty=true) }}

{# Chaining with other filters #}
{{ config | to_json | base64_encode }}
```

**More Examples:**
```jinja
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

#### `to_yaml(object)` / `| to_yaml`

Convert an object to a YAML string. Available as both function and filter.

**Arguments:**
- `object` (required for function syntax) - Object/value to convert to YAML

**Returns:** YAML string

**Function Syntax:**
```jinja
{# Simple YAML serialization #}
{% set config = {"host": "localhost", "port": 8080, "debug": true} %}
{{ to_yaml(object=config) }}
{# Output:
host: localhost
port: 8080
debug: true
#}
```

**Filter Syntax:**
```jinja
{# Simple filter syntax #}
{% set config = {"host": "localhost", "port": 8080} %}
{{ config | to_yaml }}
{# Output:
host: localhost
port: 8080
#}

{# Trim trailing newline #}
{{ config | to_yaml | trim }}
```

**More Examples:**
```jinja
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

#### `to_toml(object)` / `| to_toml`

Convert an object to a TOML string. Available as both function and filter.

**Arguments:**
- `object` (required for function syntax) - Object/value to convert to TOML

**Returns:** TOML string

**Note:** TOML has specific requirements:
- Root level must be a table (object/map)
- Arrays must contain elements of the same type

**Function Syntax:**
```jinja
{# Simple TOML serialization #}
{% set config = {"title": "My App", "version": "1.0.0"} %}
{{ to_toml(object=config) }}
{# Output:
title = "My App"
version = "1.0.0"
#}
```

**Filter Syntax:**
```jinja
{# Simple filter syntax #}
{% set config = {"title": "MyApp", "version": "1.0.0"} %}
{{ config | to_toml }}
{# Output:
title = "MyApp"
version = "1.0.0"
#}

{# Trim trailing newline #}
{{ config | to_toml | trim }}
```

**More Examples:**
```jinja
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

#### `object_keys(object)` / `| object_keys`

Get all keys from an object as an array.

**Arguments:**
- `object` (required) - Object to get keys from

**Returns:** Array of string keys

**Examples:**
```jinja
{# Function syntax #}
{% set config = {"host": "localhost", "port": 8080, "debug": true} %}
{% set keys = object_keys(object=config) %}
{{ to_json(object=keys) }}
{# Output: ["host","port","debug"] #}

{# Filter syntax #}
{% set keys = config | object_keys %}
{{ keys | join(sep=", ") }}
{# Output: host, port, debug #}

{# Iterate over keys with filter syntax #}
{% for key in config | object_keys %}
  - {{ key }}: {{ config[key] }}
{% endfor %}

{# Chaining - get count of keys #}
{{ config | object_keys | length }}
{# Output: 3 #}
```

#### `object_values(object)` / `| object_values`

Get all values from an object as an array.

**Arguments:**
- `object` (required) - Object to get values from

**Returns:** Array of values

**Examples:**
```jinja
{# Function syntax #}
{% set config = {"a": 1, "b": 2, "c": 3} %}
{% set values = object_values(object=config) %}
{{ to_json(object=values) }}
{# Output: [1,2,3] #}

{# Filter syntax #}
{% set ports = {"http": 80, "https": 443, "app": 8080} %}
{% for port in ports | object_values %}
  - {{ port }}
{% endfor %}
{# Output:
  - 80
  - 443
  - 8080
#}

{# Chaining filters #}
{% set scores = {"alice": 95, "bob": 87, "charlie": 92} %}
Average: {{ scores | object_values | array_avg }}
{# Output: Average: 91.33... #}
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

#### `json_path(object, path)`

Query objects using JSONPath-like syntax.

**Supported Syntax:**
- `$.key` or `key` - Access object property
- `$.key1.key2` - Nested property access
- `$.array[0]` - Array index access
- `$.array[*]` - Wildcard (returns all elements)
- `$.users[*].name` - Extract property from all array elements

**Arguments:**
- `object` (required): Object or array to query
- `path` (required): JSONPath expression

**Returns:** The matched value(s). For wildcard queries, returns an array.

```jinja
{% set data = {"users": [{"name": "Alice", "age": 30}, {"name": "Bob", "age": 25}]} %}

{# Access nested property #}
{{ json_path(object=data, path="$.users[0].name") }}
{# Output: Alice #}

{# Wildcard - extract all names #}
{{ json_path(object=data, path="$.users[*].name") | tojson }}
{# Output: ["Alice", "Bob"] #}

{# Access by index #}
{{ json_path(object=data, path="$.users[1].age") }}
{# Output: 25 #}

{# Simple nested access #}
{% set config = {"server": {"host": "localhost", "port": 8080}} %}
{{ json_path(object=config, path="server.port") }}
{# Output: 8080 #}
```

#### `object_pick(object, keys)`

Create a new object containing only the specified keys.

**Arguments:**
- `object` (required): Source object
- `keys` (required): Array of keys to keep

**Returns:** A new object containing only the specified keys

```jinja
{% set user = {"name": "Alice", "email": "alice@example.com", "password": "secret", "age": 30} %}

{# Pick only public fields #}
{% set public = object_pick(object=user, keys=["name", "email", "age"]) %}
{{ public | tojson }}
{# Output: {"name":"Alice","email":"alice@example.com","age":30} #}

{# Useful for API responses #}
{% set response = object_pick(object=data, keys=["id", "title", "created_at"]) %}
```

#### `object_omit(object, keys)`

Create a new object excluding the specified keys.

**Arguments:**
- `object` (required): Source object
- `keys` (required): Array of keys to exclude

**Returns:** A new object with the specified keys removed

```jinja
{% set user = {"name": "Alice", "email": "alice@example.com", "password": "secret", "internal_id": 123} %}

{# Remove sensitive fields #}
{% set safe = object_omit(object=user, keys=["password", "internal_id"]) %}
{{ safe | tojson }}
{# Output: {"name":"Alice","email":"alice@example.com"} #}

{# Clean up debug fields before output #}
{% set output = object_omit(object=config, keys=["debug", "verbose", "_internal"]) %}
```

#### `object_rename_keys(object, mapping)`

Rename object keys using a mapping.

**Arguments:**
- `object` (required): Source object
- `mapping` (required): Object mapping old keys to new keys

**Returns:** A new object with renamed keys

```jinja
{% set data = {"firstName": "Alice", "lastName": "Smith", "emailAddress": "alice@example.com"} %}

{# Convert camelCase to snake_case #}
{% set renamed = object_rename_keys(object=data, mapping={
  "firstName": "first_name",
  "lastName": "last_name",
  "emailAddress": "email"
}) %}
{{ renamed | tojson }}
{# Output: {"first_name":"Alice","last_name":"Smith","email":"alice@example.com"} #}

{# API response transformation #}
{% set api_data = object_rename_keys(object=response, mapping={"userId": "user_id", "createdAt": "created_at"}) %}
```

#### `object_flatten(object, delimiter)` / `| object_flatten`

Flatten a nested object to dot-notation keys.

**Arguments:**
- `object` (required): Nested object to flatten
- `delimiter` (optional): Delimiter for keys (default: ".")

**Returns:** A flat object with delimited keys

```jinja
{# Function syntax #}
{% set nested = {"server": {"host": "localhost", "port": 8080}, "database": {"name": "mydb"}} %}
{% set flat = object_flatten(object=nested) %}
{{ flat | tojson }}
{# Output: {"server.host":"localhost","server.port":8080,"database.name":"mydb"} #}

{# Filter syntax #}
{% set flat = nested | object_flatten %}
{{ flat["server.host"] }}
{# Output: localhost #}

{# Filter syntax with delimiter #}
{% set flat_underscore = nested | object_flatten(delimiter="_") %}
{{ flat_underscore["server_host"] }}
{# Output: localhost #}

{# Chaining - get all flattened keys #}
{{ nested | object_flatten | object_keys | join(sep=", ") }}
{# Output: server.host, server.port, database.name #}
```

#### `object_unflatten(object, delimiter)`

Unflatten a flat object with delimited keys to a nested structure.

**Arguments:**
- `object` (required): Flat object with delimited keys
- `delimiter` (optional): Delimiter used in keys (default: ".")

**Returns:** A nested object structure

```jinja
{% set flat = {"server.host": "localhost", "server.port": 8080, "database.name": "mydb"} %}

{# Unflatten to nested structure #}
{% set nested = object_unflatten(object=flat) %}
{{ nested | tojson }}
{# Output: {"server":{"host":"localhost","port":8080},"database":{"name":"mydb"}} #}

{# With custom delimiter #}
{% set flat_underscore = {"server_host": "localhost", "server_port": 8080} %}
{% set nested = object_unflatten(object=flat_underscore, delimiter="_") %}
{{ nested | tojson }}
{# Output: {"server":{"host":"localhost","port":8080}} #}

{# Useful for parsing environment variables into config #}
{% set env_config = object_unflatten(object=env_vars, delimiter="_") %}
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

#### `k8s_label_safe(value)` / `| k8s_label_safe`

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
{# Function syntax #}
{{ k8s_label_safe(value="My App (v2.0)") }}
{# Output: my-app-v2.0 #}

{# Filter syntax #}
{{ "My App (v2.0)" | k8s_label_safe }}
{# Output: my-app-v2.0 #}

{# Use in labels #}
metadata:
  labels:
    app: {{ app_name | k8s_label_safe }}
    version: {{ version | k8s_label_safe }}
```

#### `k8s_dns_label_safe(value)` / `| k8s_dns_label_safe`

Format DNS-safe label (max 63 chars, lowercase, alphanumeric and dashes only).

**Arguments:**
- `value` (required): String to format

**Returns:** DNS-safe string suitable for Kubernetes resource names

**Example:**
```jinja
{# Function syntax #}
{{ k8s_dns_label_safe(value="My Service Name") }}
{# Output: my-service-name #}

{# Filter syntax #}
{{ "My Service Name" | k8s_dns_label_safe }}
{# Output: my-service-name #}

{# Use in service names #}
apiVersion: v1
kind: Service
metadata:
  name: {{ service_name | k8s_dns_label_safe }}
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

#### `helm_tpl(template, values)`

Perform Helm-style templating with `{{ .key }}` syntax.

**Arguments:**
- `template` (required): Template string with Helm-style placeholders
- `values` (required): Object containing values to substitute

**Returns:** Rendered template string

**Example:**
```jinja
{# Basic Helm templating #}
{{ helm_tpl(template="Hello {{ .name }}!", values={"name": "World"}) }}
{# Output: Hello World! #}

{# Complex template #}
{% set tpl = "{{ .app.name }}-{{ .app.version }}" %}
{% set vals = {"app": {"name": "myapp", "version": "1.0"}} %}
{{ helm_tpl(template=tpl, values=vals) }}
{# Output: myapp-1.0 #}

{# In Kubernetes manifest #}
metadata:
  name: {{ helm_tpl(template="{{ .Release.Name }}-{{ .Chart.Name }}", values={"Release": {"Name": "prod"}, "Chart": {"Name": "webapp"}}) }}
```

#### `k8s_annotation_safe(value)` / `| k8s_annotation_safe`

Sanitize a string for use as a Kubernetes annotation value.

**Arguments:**
- `value` (required): The string to sanitize

**Returns:** Sanitized string safe for annotation values (max 64KB, control chars replaced with spaces)

**Example:**
```jinja
{# Function syntax #}
{{ k8s_annotation_safe(value="Description with\nnewlines and\ttabs") }}
{# Output: Description with newlines and tabs #}

{# Filter syntax #}
{{ "Description with\nnewlines" | k8s_annotation_safe }}
{# Output: Description with newlines #}

{# In Kubernetes manifest #}
metadata:
  annotations:
    description: "{{ description | k8s_annotation_safe }}"
    config: "{{ config_obj | to_json | k8s_annotation_safe }}"
```

#### `k8s_quantity_to_bytes(quantity)`

Convert a Kubernetes quantity string to bytes.

**Arguments:**
- `quantity` (required): Kubernetes quantity string (e.g., "1Gi", "500Mi", "100m")

**Returns:** Integer number of bytes (or millicores for CPU)

**Supported suffixes:**
- Binary: Ki (1024), Mi (1024²), Gi (1024³), Ti (1024⁴), Pi (1024⁵), Ei (1024⁶)
- Decimal: K (1000), M (1000²), G (1000³), T (1000⁴), P (1000⁵), E (1000⁶)
- CPU: m (millicores)

**Example:**
```jinja
{# Convert memory quantities #}
{{ k8s_quantity_to_bytes(quantity="1Gi") }}
{# Output: 1073741824 #}

{{ k8s_quantity_to_bytes(quantity="500Mi") }}
{# Output: 524288000 #}

{# Convert CPU quantities #}
{{ k8s_quantity_to_bytes(quantity="500m") }}
{# Output: 500 (millicores) #}

{# Calculate total memory from multiple pods #}
{% set pod_memory = k8s_quantity_to_bytes(quantity="256Mi") %}
{% set replicas = 3 %}
Total bytes: {{ pod_memory * replicas }}
```

#### `k8s_bytes_to_quantity(bytes, unit)`

Convert bytes to a Kubernetes quantity string.

**Arguments:**
- `bytes` (required): Number of bytes to convert
- `unit` (optional): Target unit (default: auto-selects appropriate unit)
  - Supported: "Ki", "Mi", "Gi", "Ti", "Pi", "Ei", "K", "M", "G", "T", "P", "E"

**Returns:** Kubernetes quantity string

**Example:**
```jinja
{# Auto-select appropriate unit #}
{{ k8s_bytes_to_quantity(bytes=1073741824) }}
{# Output: 1Gi #}

{{ k8s_bytes_to_quantity(bytes=536870912) }}
{# Output: 512Mi #}

{# Force specific unit #}
{{ k8s_bytes_to_quantity(bytes=1073741824, unit="Mi") }}
{# Output: 1024Mi #}

{# In resource calculations #}
{% set total_bytes = 2147483648 %}
resources:
  requests:
    memory: "{{ k8s_bytes_to_quantity(bytes=total_bytes / 2) }}"
  limits:
    memory: "{{ k8s_bytes_to_quantity(bytes=total_bytes) }}"
```

#### `k8s_selector(labels)`

Generate a Kubernetes label selector string from a labels object.

**Arguments:**
- `labels` (required): Object containing key-value pairs for the selector

**Returns:** Label selector string in the format "key1=value1,key2=value2"

**Example:**
```jinja
{# Basic selector #}
{{ k8s_selector(labels={"app": "nginx", "env": "prod"}) }}
{# Output: app=nginx,env=prod #}

{# In Kubernetes Service #}
spec:
  selector:
    {{ k8s_selector(labels={"app": app_name, "version": version}) }}

{# With kubectl #}
kubectl get pods -l "{{ k8s_selector(labels=pod_labels) }}"
```

#### `k8s_pod_affinity(key, operator, values, topology_key, type)`

Generate Kubernetes pod affinity/anti-affinity YAML.

**Arguments:**
- `key` (required): Label key to match
- `operator` (required): Match operator ("In", "NotIn", "Exists", "DoesNotExist")
- `values` (optional): Array of values to match (required for In/NotIn)
- `topology_key` (optional): Topology key (default: "kubernetes.io/hostname")
- `type` (optional): Affinity type - "required" or "preferred" (default: "required")

**Returns:** YAML string for pod affinity configuration

**Example:**
```jinja
{# Required pod affinity #}
{{ k8s_pod_affinity(key="app", operator="In", values=["cache", "db"]) }}
{# Output:
requiredDuringSchedulingIgnoredDuringExecution:
  - labelSelector:
      matchExpressions:
        - key: app
          operator: In
          values:
            - cache
            - db
    topologyKey: kubernetes.io/hostname
#}

{# Preferred anti-affinity for spreading pods #}
{{ k8s_pod_affinity(key="app", operator="In", values=["web"], type="preferred") }}
{# Output:
preferredDuringSchedulingIgnoredDuringExecution:
  - weight: 100
    podAffinityTerm:
      labelSelector:
        matchExpressions:
          - key: app
            operator: In
            values:
              - web
      topologyKey: kubernetes.io/hostname
#}

{# In Deployment spec #}
spec:
  affinity:
    podAntiAffinity:
      {{ k8s_pod_affinity(key="app", operator="In", values=[app_name], topology_key="topology.kubernetes.io/zone") | indent(6) }}
```

#### `k8s_toleration(key, operator, value, effect)`

Generate Kubernetes toleration YAML.

**Arguments:**
- `key` (required): Taint key to tolerate
- `operator` (optional): Match operator - "Equal" or "Exists" (default: "Equal")
- `value` (optional): Taint value to match (required when operator is "Equal")
- `effect` (optional): Taint effect - "NoSchedule", "PreferNoSchedule", or "NoExecute"

**Returns:** YAML string for toleration configuration

**Example:**
```jinja
{# Basic toleration #}
{{ k8s_toleration(key="dedicated", value="gpu", effect="NoSchedule") }}
{# Output:
- key: dedicated
  operator: Equal
  value: gpu
  effect: NoSchedule
#}

{# Exists operator (matches any value) #}
{{ k8s_toleration(key="node.kubernetes.io/not-ready", operator="Exists", effect="NoExecute") }}
{# Output:
- key: node.kubernetes.io/not-ready
  operator: Exists
  effect: NoExecute
#}

{# In Pod spec #}
spec:
  tolerations:
    {{ k8s_toleration(key="dedicated", value="high-memory", effect="NoSchedule") | indent(4) }}
    {{ k8s_toleration(key="node.kubernetes.io/unreachable", operator="Exists", effect="NoExecute") | indent(4) }}
```

#### `k8s_probe(type, path, port, initial_delay, period, timeout, success_threshold, failure_threshold, command)`

Generate Kubernetes liveness/readiness probe YAML.

**Arguments:**
- `type` (required): Probe type - "http", "tcp", or "exec"
- `path` (optional): HTTP path for http probes (default: "/healthz")
- `port` (optional): Port number for http/tcp probes (default: 8080)
- `initial_delay` (optional): Initial delay in seconds (default: 0)
- `period` (optional): Period between probes in seconds (default: 10)
- `timeout` (optional): Timeout in seconds (default: 1)
- `success_threshold` (optional): Success threshold (default: 1)
- `failure_threshold` (optional): Failure threshold (default: 3)
- `command` (optional): Command array for exec probes

**Returns:** YAML string for probe configuration

**Example:**
```jinja
{# HTTP health check #}
{{ k8s_probe(type="http", path="/health", port=8080) }}
{# Output:
httpGet:
  path: /health
  port: 8080
initialDelaySeconds: 0
periodSeconds: 10
timeoutSeconds: 1
successThreshold: 1
failureThreshold: 3
#}

{# TCP socket check with custom timings #}
{{ k8s_probe(type="tcp", port=5432, initial_delay=30, period=20) }}
{# Output:
tcpSocket:
  port: 5432
initialDelaySeconds: 30
periodSeconds: 20
timeoutSeconds: 1
successThreshold: 1
failureThreshold: 3
#}

{# Exec probe with command #}
{{ k8s_probe(type="exec", command=["cat", "/tmp/healthy"]) }}
{# Output:
exec:
  command:
    - cat
    - /tmp/healthy
initialDelaySeconds: 0
periodSeconds: 10
timeoutSeconds: 1
successThreshold: 1
failureThreshold: 3
#}

{# In container spec #}
containers:
  - name: app
    livenessProbe:
      {{ k8s_probe(type="http", path="/healthz", port=8080, initial_delay=15, failure_threshold=5) | indent(6) }}
    readinessProbe:
      {{ k8s_probe(type="http", path="/ready", port=8080, period=5) | indent(6) }}
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

Return the absolute value of a number. Supports both function and filter syntax.

**Arguments:**
- `number` (required): Number to get absolute value of

**Returns:** The absolute value (always positive)

**Example:**
```jinja
{# Function syntax #}
{{ abs(number=-42) }}
{# Output: 42 #}

{# Filter syntax #}
{{ -42 | abs }}
{# Output: 42 #}

{# Temperature difference - function syntax #}
{% set temp1 = 25 %}
{% set temp2 = 18 %}
Difference: {{ abs(number=temp1 - temp2) }}°C

{# Chaining filters #}
{{ -3.7 | abs | ceil }}
{# Output: 4 #}
```

#### `round(number, decimals=0)`

Round a number to N decimal places. Supports both function and filter syntax.

**Arguments:**
- `number` (required): Number to round
- `decimals` (optional): Number of decimal places (default: 0)

**Returns:** The number rounded to the specified decimal places

**Example:**
```jinja
{# Function syntax #}
{{ round(number=3.7) }}
{# Output: 4 #}

{# Filter syntax #}
{{ 3.7 | round }}
{# Output: 4 #}

{# Round to 2 decimal places - function syntax #}
{{ round(number=3.14159, decimals=2) }}
{# Output: 3.14 #}

{# Round to 2 decimal places - filter syntax #}
{{ 3.14159 | round(decimals=2) }}
{# Output: 3.14 #}

{# Price calculation with filter chaining #}
{% set price = 19.999 %}
Price: ${{ price | round(decimals=2) }}
```

#### `ceil(number)`

Round up to the nearest integer. Supports both function and filter syntax.

**Arguments:**
- `number` (required): Number to round up

**Returns:** The smallest integer greater than or equal to the number

**Example:**
```jinja
{# Function syntax #}
{{ ceil(number=3.1) }}
{# Output: 4 #}

{# Filter syntax #}
{{ 3.1 | ceil }}
{# Output: 4 #}

{# Calculate required servers - function syntax #}
{% set users = 150 %}
{% set users_per_server = 50 %}
Servers needed: {{ ceil(number=users / users_per_server) }}

{# With filter chaining #}
{{ -3.7 | abs | ceil }}
{# Output: 4 #}
```

#### `floor(number)`

Round down to the nearest integer. Supports both function and filter syntax.

**Arguments:**
- `number` (required): Number to round down

**Returns:** The largest integer less than or equal to the number

**Example:**
```jinja
{# Function syntax #}
{{ floor(number=3.9) }}
{# Output: 3 #}

{# Filter syntax #}
{{ 3.9 | floor }}
{# Output: 3 #}

{# Calculate filled pages - function syntax #}
{% set items = 47 %}
{% set items_per_page = 10 %}
Full pages: {{ floor(number=items / items_per_page) }}

{# With filter chaining #}
{{ 3.999 | floor | abs }}
{# Output: 3 #}
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

#### `array_sum(array)` / `| array_sum`

Calculate the sum of all values in an array.

**Arguments:**
- `array` (required): Array of numbers to sum

**Returns:** Sum of all values (integer if no decimals, float otherwise)

**Function syntax:**
```jinja
{# Sum of integers #}
{% set numbers = [1, 2, 3, 4, 5] %}
Total: {{ array_sum(array=numbers) }}
{# Output: Total: 15 #}

{# Sum of prices #}
{% set prices = [10.5, 20.25, 5.75] %}
Total: ${{ array_sum(array=prices) }}
{# Output: Total: $36.5 #}
```

**Filter syntax:**
```jinja
{% set numbers = [1, 2, 3, 4, 5] %}
{{ numbers | array_sum }}
{# Output: 15 #}

{# Chaining: unique then sum #}
{% set nums = [1, 2, 2, 3, 3] %}
{{ nums | array_unique | array_sum }}
{# Output: 6 #}
```

#### `array_avg(array)` / `| array_avg`

Calculate the average (mean) of all values in an array.

**Arguments:**
- `array` (required): Array of numbers

**Returns:** Arithmetic mean of all values (0 for empty arrays)

**Function syntax:**
```jinja
{# Average score #}
{% set scores = [85, 90, 78, 92, 88] %}
Average: {{ array_avg(array=scores) }}
{# Output: Average: 86.6 #}

{# Empty array handling #}
{% set empty = [] %}
Default: {{ array_avg(array=empty) }}
{# Output: Default: 0 #}
```

**Filter syntax:**
```jinja
{% set scores = [85, 90, 78, 92, 88] %}
{{ scores | array_avg }}
{# Output: 86.6 #}
```

#### `array_median(array)` / `| array_median`

Calculate the median value of an array.

**Arguments:**
- `array` (required): Array of numbers

**Returns:** Middle value for odd-length arrays, average of two middle values for even-length arrays

**Function syntax:**
```jinja
{# Median of odd-length array #}
{% set nums = [1, 3, 5, 7, 9] %}
Median: {{ array_median(array=nums) }}
{# Output: Median: 5 #}

{# Median of even-length array #}
{% set nums = [1, 2, 3, 4] %}
Median: {{ array_median(array=nums) }}
{# Output: Median: 2.5 #}
```

**Filter syntax:**
```jinja
{% set nums = [1, 3, 5, 7, 9] %}
{{ nums | array_median }}
{# Output: 5 #}
```

#### `array_min(array)` / `| array_min`

Find the minimum value in an array.

**Arguments:**
- `array` (required): Array of numbers

**Returns:** Smallest value in the array

**Function syntax:**
```jinja
{# Find minimum #}
{% set numbers = [42, 17, 99, 8, 55] %}
Minimum: {{ array_min(array=numbers) }}
{# Output: Minimum: 8 #}

{# Lowest price #}
{% set prices = [10.99, 5.49, 15.99, 7.25] %}
Best deal: ${{ array_min(array=prices) }}
```

**Filter syntax:**
```jinja
{% set numbers = [42, 17, 99, 8, 55] %}
{{ numbers | array_min }}
{# Output: 8 #}
```

#### `array_max(array)` / `| array_max`

Find the maximum value in an array.

**Arguments:**
- `array` (required): Array of numbers

**Returns:** Largest value in the array

**Function syntax:**
```jinja
{# Find maximum #}
{% set numbers = [42, 17, 99, 8, 55] %}
Maximum: {{ array_max(array=numbers) }}
{# Output: Maximum: 99 #}

{# Peak memory usage #}
{% set memory = [512, 768, 1024, 896] %}
Peak: {{ array_max(array=memory) }}MB
```

**Filter syntax:**
```jinja
{% set numbers = [42, 17, 99, 8, 55] %}
{{ numbers | array_max }}
{# Output: 99 #}
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

#### `array_unique(array)` / `| array_unique`

Remove duplicate values from an array.

**Arguments:**
- `array` (required): Array to deduplicate

**Returns:** New array with duplicates removed (first occurrence kept)

**Function syntax:**
```jinja
{# Remove duplicate numbers #}
{% set nums = [1, 2, 2, 3, 1, 4, 3, 5] %}
{{ array_unique(array=nums) }}
{# Output: [1, 2, 3, 4, 5] #}

{# Unique tags #}
{% set tags = ["docker", "kubernetes", "docker", "helm", "kubernetes"] %}
Unique tags: {{ array_unique(array=tags) | join(", ") }}
{# Output: Unique tags: docker, kubernetes, helm #}
```

**Filter syntax:**
```jinja
{% set nums = [1, 2, 2, 3, 3, 3] %}
{{ nums | array_unique | join(", ") }}
{# Output: 1, 2, 3 #}

{# Chaining with sum #}
{{ nums | array_unique | array_sum }}
{# Output: 6 #}
```

#### `array_flatten(array)` / `| array_flatten`

Flatten nested arrays by one level.

**Arguments:**
- `array` (required): Array with nested arrays

**Returns:** New array with nested arrays flattened one level

**Function syntax:**
```jinja
{# Flatten nested arrays #}
{% set nested = [[1, 2], [3, 4], [5]] %}
{{ array_flatten(array=nested) }}
{# Output: [1, 2, 3, 4, 5] #}

{# Only flattens one level #}
{% set deep = [[1, [2, 3]], [4]] %}
{{ array_flatten(array=deep) }}
{# Output: [1, [2, 3], 4] #}
```

**Filter syntax:**
```jinja
{% set nested = [[1, 2], [3, 4], [5]] %}
{{ nested | array_flatten | join(", ") }}
{# Output: 1, 2, 3, 4, 5 #}

{# Collect values from multiple sources #}
{% set server1_ips = ["10.0.1.1", "10.0.1.2"] %}
{% set server2_ips = ["10.0.2.1"] %}
{{ [server1_ips, server2_ips] | array_flatten | join(", ") }}
{# Output: 10.0.1.1, 10.0.1.2, 10.0.2.1 #}
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

#### `array_take(array, n)`

Take the first N elements from an array.

**Arguments:**
- `array` (required): Source array
- `n` (required): Number of elements to take

**Returns:** Array with the first N elements

**Example:**
```jinja
{{ array_take(array=[1, 2, 3, 4, 5], n=3) }}
{# Output: [1, 2, 3] #}

{# Taking more than available returns all elements #}
{{ array_take(array=[1, 2], n=5) }}
{# Output: [1, 2] #}
```

#### `array_drop(array, n)`

Skip the first N elements from an array.

**Arguments:**
- `array` (required): Source array
- `n` (required): Number of elements to skip

**Returns:** Array with elements after the first N

**Example:**
```jinja
{{ array_drop(array=[1, 2, 3, 4, 5], n=2) }}
{# Output: [3, 4, 5] #}

{# Dropping more than available returns empty array #}
{{ array_drop(array=[1, 2], n=5) }}
{# Output: [] #}
```

#### `array_index_of(array, value)`

Find the index of an element in an array.

**Arguments:**
- `array` (required): Array to search
- `value` (required): Value to find

**Returns:** Index (0-based) or -1 if not found

**Example:**
```jinja
{{ array_index_of(array=["a", "b", "c"], value="b") }}
{# Output: 1 #}

{{ array_index_of(array=[1, 2, 3], value=5) }}
{# Output: -1 #}
```

#### `array_find(array, key, value)`

Find the first matching object in an array of objects.

**Arguments:**
- `array` (required): Array of objects to search
- `key` (required): Key to match
- `value` (required): Value to match

**Returns:** The first matching object or null if not found

**Example:**
```jinja
{% set users = [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}] %}
{{ array_find(array=users, key="id", value=2) | tojson }}
{# Output: {"id": 2, "name": "Bob"} #}

{{ array_find(array=users, key="id", value=99) }}
{# Output: null #}
```

#### `array_filter_by(array, key, op, value)`

Filter an array of objects by a key with comparison operators.

**Arguments:**
- `array` (required): Array of objects to filter
- `key` (required): Key to compare
- `op` (required): Operator: `"eq"`, `"ne"`, `"gt"`, `"lt"`, `"gte"`, `"lte"`, `"contains"`
- `value` (required): Value to compare against

**Returns:** Filtered array of matching objects

**Example:**
```jinja
{% set items = [{"price": 10}, {"price": 20}, {"price": 30}] %}
{{ array_filter_by(array=items, key="price", op="gt", value=15) | tojson }}
{# Output: [{"price": 20}, {"price": 30}] #}

{% set users = [{"name": "Alice"}, {"name": "Bob"}, {"name": "Charlie"}] %}
{{ array_filter_by(array=users, key="name", op="contains", value="li") | tojson }}
{# Output: [{"name": "Alice"}, {"name": "Charlie"}] #}
```

#### `array_pluck(array, key)`

Extract values from an array of objects by key (supports dot notation for nested keys).

**Arguments:**
- `array` (required): Array of objects
- `key` (required): Key path to extract (e.g., `"user.name"`)

**Returns:** Array of extracted values

**Example:**
```jinja
{% set users = [{"name": "Alice"}, {"name": "Bob"}] %}
{{ array_pluck(array=users, key="name") | tojson }}
{# Output: ["Alice", "Bob"] #}

{# Nested keys with dot notation #}
{% set data = [{"user": {"name": "Alice"}}, {"user": {"name": "Bob"}}] %}
{{ array_pluck(array=data, key="user.name") | tojson }}
{# Output: ["Alice", "Bob"] #}
```

#### `array_intersection(array1, array2)`

Get elements that exist in both arrays.

**Arguments:**
- `array1` (required): First array
- `array2` (required): Second array

**Returns:** Array of common elements

**Example:**
```jinja
{{ array_intersection(array1=[1, 2, 3, 4], array2=[3, 4, 5, 6]) | tojson }}
{# Output: [3, 4] #}

{{ array_intersection(array1=["a", "b", "c"], array2=["b", "c", "d"]) | tojson }}
{# Output: ["b", "c"] #}
```

#### `array_difference(array1, array2)`

Get elements in the first array that are not in the second.

**Arguments:**
- `array1` (required): First array
- `array2` (required): Second array

**Returns:** Array of elements in array1 but not in array2

**Example:**
```jinja
{{ array_difference(array1=[1, 2, 3, 4], array2=[3, 4, 5, 6]) | tojson }}
{# Output: [1, 2] #}

{{ array_difference(array1=["a", "b", "c"], array2=["b"]) | tojson }}
{# Output: ["a", "c"] #}
```

#### `array_union(array1, array2)`

Get all unique elements from both arrays.

**Arguments:**
- `array1` (required): First array
- `array2` (required): Second array

**Returns:** Array of all unique elements from both arrays

**Example:**
```jinja
{{ array_union(array1=[1, 2, 3], array2=[3, 4, 5]) | tojson }}
{# Output: [1, 2, 3, 4, 5] #}

{{ array_union(array1=["a", "b"], array2=["b", "c"]) | tojson }}
{# Output: ["a", "b", "c"] #}
```

#### `array_symmetric_difference(array1, array2)`

Get elements that are in either array but not in both.

**Arguments:**
- `array1` (required): First array
- `array2` (required): Second array

**Returns:** Array of elements in either array but not in both

**Example:**
```jinja
{{ array_symmetric_difference(array1=[1, 2, 3, 4], array2=[3, 4, 5, 6]) | tojson }}
{# Output: [1, 2, 5, 6] #}

{{ array_symmetric_difference(array1=["a", "b", "c"], array2=["b", "c", "d"]) | tojson }}
{# Output: ["a", "d"] #}
```

### String Manipulation Functions

Advanced string operations including regex support.

#### `regex_replace(string, pattern, replacement)`

Replace substrings using a regex pattern. Supports both function and filter syntax.

**Arguments:**
- `string` (required): The input string
- `pattern` (required): Regex pattern to match
- `replacement` (required): Replacement string (supports `$1`, `$2` for capture groups)

**Returns:** String with all matches replaced

**Example:**
```jinja
{# Function syntax #}
{{ regex_replace(string="hello123world", pattern="[0-9]+", replacement="-") }}
{# Output: hello-world #}

{# Filter syntax #}
{{ "hello123world" | regex_replace(pattern="[0-9]+", replacement="-") }}
{# Output: hello-world #}

{{ "foo bar baz" | regex_replace(pattern="\\s+", replacement="_") }}
{# Output: foo_bar_baz #}

{# Using capture groups #}
{{ "hello world" | regex_replace(pattern="(\\w+) (\\w+)", replacement="$2 $1") }}
{# Output: world hello #}
```

#### `regex_match(string, pattern)`

Check if a string matches a regex pattern.

**Arguments:**
- `string` (required): The input string
- `pattern` (required): Regex pattern to match

**Returns:** Boolean - true if the pattern matches anywhere in the string

**Example:**
```jinja
{{ regex_match(string="hello123", pattern="[0-9]+") }}
{# Output: true #}

{{ regex_match(string="hello", pattern="[0-9]+") }}
{# Output: false #}

{# Validate email format #}
{% if regex_match(string=email, pattern="^[\\w.-]+@[\\w.-]+\\.\\w+$") %}
  Valid email
{% endif %}
```

#### `regex_find_all(string, pattern)`

Find all matches of a regex pattern in a string.

**Arguments:**
- `string` (required): The input string
- `pattern` (required): Regex pattern to match

**Returns:** Array of all matches

**Example:**
```jinja
{{ regex_find_all(string="a1b2c3", pattern="[0-9]+") | tojson }}
{# Output: ["1", "2", "3"] #}

{{ regex_find_all(string="hello world", pattern="\\w+") | tojson }}
{# Output: ["hello", "world"] #}

{# Extract all URLs #}
{% set urls = regex_find_all(string=text, pattern="https?://[\\w./]+") %}
Found {{ urls | length }} URLs
```

#### `substring(string, start, length)`

Extract a substring by position. Supports both function and filter syntax.

**Arguments:**
- `string` (required): The input string
- `start` (required): Start position (0-based, negative counts from end)
- `length` (optional): Number of characters to extract (default: rest of string)

**Returns:** The extracted substring

**Example:**
```jinja
{# Function syntax #}
{{ substring(string="hello world", start=0, length=5) }}
{# Output: hello #}

{# Filter syntax #}
{{ "hello world" | substring(start=0, length=5) }}
{# Output: hello #}

{{ "hello world" | substring(start=6) }}
{# Output: world #}

{# Negative start counts from end #}
{{ "hello world" | substring(start=-5) }}
{# Output: world #}
```

#### `contains(string, substring)`

Check if a string contains a substring.

**Arguments:**
- `string` (required): The input string
- `substring` (required): Substring to search for

**Returns:** Boolean - true if substring is found

**Example:**
```jinja
{{ contains(string="hello world", substring="world") }}
{# Output: true #}

{{ contains(string="hello world", substring="foo") }}
{# Output: false #}

{% if contains(string=filename, substring=".txt") %}
  Text file detected
{% endif %}
```

#### `index_of(string, substring)`

Find the position of a substring.

**Arguments:**
- `string` (required): The input string
- `substring` (required): Substring to search for

**Returns:** Position (0-based) or -1 if not found

**Example:**
```jinja
{{ index_of(string="hello world", substring="world") }}
{# Output: 6 #}

{{ index_of(string="hello world", substring="foo") }}
{# Output: -1 #}
```

#### `count_occurrences(string, substring)`

Count occurrences of a substring.

**Arguments:**
- `string` (required): The input string
- `substring` (required): Substring to count

**Returns:** Number of non-overlapping occurrences

**Example:**
```jinja
{{ count_occurrences(string="hello hello hello", substring="hello") }}
{# Output: 3 #}

{{ count_occurrences(string="abcabc", substring="abc") }}
{# Output: 2 #}
```

#### `truncate(string, length, suffix)`

Truncate a string with a suffix. Supports both function and filter syntax.

**Arguments:**
- `string` (required): The input string
- `length` (required): Maximum length (including suffix)
- `suffix` (optional): Suffix to add when truncated (default: `"..."`)

**Returns:** Truncated string with suffix if it was truncated

**Example:**
```jinja
{# Function syntax #}
{{ truncate(string="Hello World", length=8) }}
{# Output: Hello... #}

{# Filter syntax #}
{{ "Hello World" | truncate(length=8) }}
{# Output: Hello... #}

{{ "Hello World" | truncate(length=8, suffix=">>") }}
{# Output: Hello >> #}

{# Not truncated if already short enough #}
{{ "Hi" | truncate(length=10) }}
{# Output: Hi #}
```

#### `word_count(string)`

Count words in a string. Supports both function and filter syntax.

**Arguments:**
- `string` (required): The input string

**Returns:** Number of words (whitespace-separated)

**Example:**
```jinja
{# Function syntax #}
{{ word_count(string="Hello World") }}
{# Output: 2 #}

{# Filter syntax #}
{{ "Hello World" | word_count }}
{# Output: 2 #}

{{ "  one   two   three  " | word_count }}
{# Output: 3 #}
```

#### `split_lines(string)`

Split a string into an array of lines. Supports both function and filter syntax.

**Arguments:**
- `string` (required): The input string

**Returns:** Array of lines

**Example:**
```jinja
{# Function syntax #}
{% set text = "line1
line2
line3" %}
{{ split_lines(string=text) | tojson }}
{# Output: ["line1", "line2", "line3"] #}

{# Filter syntax #}
{{ text | split_lines | tojson }}
{# Output: ["line1", "line2", "line3"] #}

{% for line in content | split_lines %}
  Line: {{ line }}
{% endfor %}
```

#### `wrap(string, width, indent)`

Word wrap text at a specified width. Supports both function and filter syntax.

**Arguments:**
- `string` (required): The input string to wrap
- `width` (required): Maximum line width
- `indent` (optional): Indentation string for wrapped lines (default: "")

**Returns:** The wrapped text with newlines inserted

**Example:**
```jinja
{# Function syntax #}
{{ wrap(string="The quick brown fox jumps over the lazy dog", width=20) }}

{# Filter syntax #}
{{ "The quick brown fox jumps over the lazy dog" | wrap(width=20) }}
{# Output:
The quick brown fox
jumps over the lazy
dog
#}

{{ "Hello World Example" | wrap(width=10, indent="  ") }}
{# Output:
Hello
  World
  Example
#}
```

#### `center(string, width, char)`

Center text with padding. Supports both function and filter syntax.

**Arguments:**
- `string` (required): The input string
- `width` (required): Total width of the result
- `char` (optional): Padding character (default: space)

**Returns:** The centered string with padding

**Example:**
```jinja
{# Function syntax #}
{{ center(string="hello", width=11) }}
{# Output: "   hello   " #}

{# Filter syntax #}
{{ "hello" | center(width=11) }}
{# Output: "   hello   " #}

{{ "hi" | center(width=10, char="-") }}
{# Output: "----hi----" #}

{{ "test" | center(width=8, char="*") }}
{# Output: "**test**" #}
```

#### `sentence_case(string)`

Convert to Sentence case (first letter capitalized, rest lowercase).

**Arguments:**
- `string` (required): The input string

**Returns:** The string in sentence case

```jinja
{{ sentence_case(string="hello world") }}
{# Output: Hello world #}

{{ sentence_case(string="HELLO WORLD") }}
{# Output: Hello world #}

{{ sentence_case(string="hELLO wORLD") }}
{# Output: Hello world #}
```

#### `strip_html(string)`

Remove HTML tags from a string. Supports both function and filter syntax.

**Arguments:**
- `string` (required): The input string with HTML

**Returns:** The string with all HTML tags removed

**Example:**
```jinja
{# Function syntax #}
{{ strip_html(string="<p>Hello <b>World</b></p>") }}
{# Output: Hello World #}

{# Filter syntax #}
{{ "<p>Hello <b>World</b></p>" | strip_html }}
{# Output: Hello World #}

{{ "<div class='test'>Content</div>" | strip_html }}
{# Output: Content #}

{{ html_content | strip_html | normalize_whitespace }}
{# Chaining with other filters #}
```

#### `strip_ansi(string)`

Remove ANSI escape codes from a string. Supports both function and filter syntax.

**Arguments:**
- `string` (required): The input string with ANSI codes

**Returns:** The string with all ANSI escape codes removed

**Example:**
```jinja
{# Function syntax #}
{{ strip_ansi(string="\x1b[31mRed Text\x1b[0m") }}
{# Output: Red Text #}

{# Filter syntax #}
{{ "\x1b[31mRed Text\x1b[0m" | strip_ansi }}
{# Output: Red Text #}

{{ terminal_output | strip_ansi }}
{# Remove color codes from terminal output #}
```

#### `normalize_whitespace(string)`

Normalize whitespace by collapsing multiple spaces/tabs/newlines into a single space. Supports both function and filter syntax.

**Arguments:**
- `string` (required): The input string

**Returns:** The string with normalized whitespace (trimmed and collapsed)

**Example:**
```jinja
{# Function syntax #}
{{ normalize_whitespace(string="  hello   world  ") }}
{# Output: hello world #}

{# Filter syntax #}
{{ "  hello   world  " | normalize_whitespace }}
{# Output: hello world #}

{{ "line1\n\n\nline2\t\tline3" | normalize_whitespace }}
{# Output: line1 line2 line3 #}

{# Chaining with other filters #}
{{ html_content | strip_html | normalize_whitespace | truncate(length=100) }}
```

#### `to_constant_case(string)`

Convert to CONSTANT_CASE (uppercase with underscores).

**Arguments:**
- `string` (required): The input string

**Returns:** The string in CONSTANT_CASE format

```jinja
{{ to_constant_case(string="hello world") }}
{# Output: HELLO_WORLD #}

{{ to_constant_case(string="helloWorld") }}
{# Output: HELLO_WORLD #}

{{ to_constant_case(string="hello-world-test") }}
{# Output: HELLO_WORLD_TEST #}

{{ to_constant_case(string="HTTPResponse") }}
{# Output: HTTPRESPONSE #}
```

#### `pluralize(count, singular, plural)`

Pluralize a word based on count.

**Arguments:**
- `count` (required): The count to check
- `singular` (required): The singular form of the word
- `plural` (optional): The plural form (default: singular + "s")

**Returns:** Singular if count is 1, otherwise plural

```jinja
{{ pluralize(count=1, singular="item") }}
{# Output: item #}

{{ pluralize(count=5, singular="item") }}
{# Output: items #}

{{ pluralize(count=0, singular="child", plural="children") }}
{# Output: children #}

{# Use with variables #}
You have {{ count }} {{ pluralize(count=count, singular="message", plural="messages") }}
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

#### `get_interfaces()`

Get a list of all network interfaces with their IP addresses.

**Arguments:** None

**Returns:** List of objects with interface information:
- `name` - Interface name (e.g., "eth0", "en0", "lo")
- `ip` - IP address assigned to the interface
- `is_loopback` - Boolean indicating if this is a loopback interface

**Example:**
```
{# List all interfaces #}
{% for iface in get_interfaces() %}
  {{ iface.name }}: {{ iface.ip }}{% if iface.is_loopback %} (loopback){% endif %}
{% endfor %}

{# Filter non-loopback interfaces #}
{% for iface in get_interfaces() | selectattr("is_loopback", "equalto", false) %}
  {{ iface.name }}: {{ iface.ip }}
{% endfor %}

{# Find first non-loopback IP #}
{% set external_iface = get_interfaces() | selectattr("is_loopback", "equalto", false) | first %}
Bind IP: {{ external_iface.ip }}
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

#### `is_port_available(port)` / `{% if port is port_available %}`

Check if a port is available (not in use). Supports both function syntax and "is" test syntax.

**Function Syntax Arguments:**
- `port` (required) - Port number to check (1-65535)

**Is-Test Syntax:**
- The value must be an integer between 1 and 65535, or a string that can be parsed as such

**Returns:** Boolean (`true` if available, `false` if in use)

**Examples:**
```jinja
{# Function syntax #}
{% if is_port_available(port=8080) %}
  Port 8080 is available
{% else %}
  Port 8080 is already in use
{% endif %}

{# Is-test syntax (preferred for readability) #}
{% if 8080 is port_available %}
  Port 8080 is available
{% endif %}

{# With variables #}
{% set my_port = 3000 %}
{% if my_port is port_available %}
APP_PORT={{ my_port }}
{% elif 3001 is port_available %}
APP_PORT=3001
{% else %}
APP_PORT=8080
{% endif %}
```

#### `get_os()`

Get the operating system name.

**Arguments:** None

**Returns:** String containing the OS name (e.g., "linux", "macos", "windows")

**Example:**
```
OS: {{ get_os() }}
{# Output: OS: macos #}

{% if get_os() == "linux" %}
  Running on Linux
{% elif get_os() == "macos" %}
  Running on macOS
{% endif %}
```

#### `get_arch()`

Get the CPU architecture.

**Arguments:** None

**Returns:** String containing the architecture (e.g., "x86_64", "aarch64", "arm")

**Example:**
```
Arch: {{ get_arch() }}
{# Output: Arch: aarch64 #}

{% if get_arch() == "aarch64" %}
  Running on ARM64
{% elif get_arch() == "x86_64" %}
  Running on x86-64
{% endif %}
```

#### `get_cwd()`

Get the current working directory.

**Arguments:** None

**Returns:** String containing the current working directory path

**Example:**
```
CWD: {{ get_cwd() }}
{# Output: CWD: /home/user/projects/myapp #}
```

#### `cidr_contains(cidr, ip)`

Check if an IP address is within a CIDR range.

**Arguments:**
- `cidr` (required) - CIDR notation (e.g., "192.168.1.0/24")
- `ip` (required) - IP address to check

**Returns:** Boolean (`true` if IP is in range, `false` otherwise)

**Example:**
```
{% if cidr_contains(cidr="192.168.1.0/24", ip="192.168.1.100") %}
  IP is in the subnet
{% else %}
  IP is outside the subnet
{% endif %}
{# Output: IP is in the subnet #}

{% if cidr_contains(cidr="10.0.0.0/8", ip="192.168.1.1") %}
  In private range
{% else %}
  Not in 10.x.x.x range
{% endif %}
{# Output: Not in 10.x.x.x range #}
```

#### `cidr_network(cidr)`

Get the network address from a CIDR notation.

**Arguments:**
- `cidr` (required) - CIDR notation (e.g., "192.168.1.100/24")

**Returns:** String containing the network address

**Example:**
```
Network: {{ cidr_network(cidr="192.168.1.100/24") }}
{# Output: Network: 192.168.1.0 #}

Network: {{ cidr_network(cidr="10.20.30.40/16") }}
{# Output: Network: 10.20.0.0 #}
```

#### `cidr_broadcast(cidr)`

Get the broadcast address from a CIDR notation.

**Arguments:**
- `cidr` (required) - CIDR notation (e.g., "192.168.1.0/24")

**Returns:** String containing the broadcast address

**Example:**
```
Broadcast: {{ cidr_broadcast(cidr="192.168.1.0/24") }}
{# Output: Broadcast: 192.168.1.255 #}

Broadcast: {{ cidr_broadcast(cidr="10.0.0.0/8") }}
{# Output: Broadcast: 10.255.255.255 #}
```

#### `cidr_netmask(cidr)`

Get the netmask from a CIDR notation.

**Arguments:**
- `cidr` (required) - CIDR notation (e.g., "192.168.1.0/24")

**Returns:** String containing the netmask

**Example:**
```
Netmask: {{ cidr_netmask(cidr="192.168.1.0/24") }}
{# Output: Netmask: 255.255.255.0 #}

Netmask: {{ cidr_netmask(cidr="10.0.0.0/8") }}
{# Output: Netmask: 255.0.0.0 #}

Netmask: {{ cidr_netmask(cidr="172.16.0.0/12") }}
{# Output: Netmask: 255.240.0.0 #}
```

#### `ip_to_int(ip)`

Convert an IPv4 address to its integer representation.

**Arguments:**
- `ip` (required) - IPv4 address (e.g., "192.168.1.1")

**Returns:** Integer representation of the IP address

**Example:**
```
Integer: {{ ip_to_int(ip="192.168.1.1") }}
{# Output: Integer: 3232235777 #}

Integer: {{ ip_to_int(ip="0.0.0.0") }}
{# Output: Integer: 0 #}

Integer: {{ ip_to_int(ip="255.255.255.255") }}
{# Output: Integer: 4294967295 #}
```

#### `int_to_ip(int)`

Convert an integer to its IPv4 address representation.

**Arguments:**
- `int` (required) - Integer value (0 to 4294967295)

**Returns:** String containing the IPv4 address

**Example:**
```
IP: {{ int_to_ip(int=3232235777) }}
{# Output: IP: 192.168.1.1 #}

IP: {{ int_to_ip(int=0) }}
{# Output: IP: 0.0.0.0 #}

IP: {{ int_to_ip(int=4294967295) }}
{# Output: IP: 255.255.255.255 #}
```

**Practical Example - Network Configuration:**
```yaml
network:
  # CIDR operations
  cidr: 192.168.1.0/24
  network_addr: {{ cidr_network(cidr="192.168.1.0/24") }}
  broadcast: {{ cidr_broadcast(cidr="192.168.1.0/24") }}
  netmask: {{ cidr_netmask(cidr="192.168.1.0/24") }}

  # IP validation
  {% set client_ip = "192.168.1.50" %}
  {% if cidr_contains(cidr="192.168.1.0/24", ip=client_ip) %}
  client_allowed: true
  {% else %}
  client_allowed: false
  {% endif %}

  # System info
  os: {{ get_os() }}
  arch: {{ get_arch() }}
  cwd: {{ get_cwd() }}
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

## IDE Integration

The `--ide` flag outputs comprehensive metadata about all available functions, making it easy to build IDE plugins, autocomplete systems, and documentation generators.

### Output Formats

```bash
# JSON output (array of function metadata)
tmpltool --ide json > functions.json

# YAML output (list of function metadata)
tmpltool --ide yaml > functions.yaml

# TOML output (wrapped in [[functions]] array)
tmpltool --ide toml > functions.toml
```

### Metadata Structure

Each function includes:

| Field | Description |
|-------|-------------|
| `name` | Function name (e.g., `get_env`, `md5`) |
| `category` | Category grouping (e.g., `environment`, `hash`, `string`) |
| `description` | What the function does |
| `arguments` | Array of argument definitions with name, type, required flag, default value, and description |
| `return_type` | Type of value returned |
| `examples` | Usage examples showing both function and filter syntax where applicable |
| `syntax.function` | Whether callable as `func(arg=value)` |
| `syntax.filter` | Whether callable as `value \| filter` |
| `syntax.is_test` | Whether usable in `{% if value is test %}` |

### Example Output (JSON)

```json
[
  {
    "name": "get_env",
    "category": "environment",
    "description": "Get environment variable with optional default value",
    "arguments": [
      {
        "name": "name",
        "arg_type": "string",
        "required": true,
        "default": null,
        "description": "Environment variable name"
      },
      {
        "name": "default",
        "arg_type": "string",
        "required": false,
        "default": null,
        "description": "Default value if variable is not set"
      }
    ],
    "return_type": "string",
    "examples": [
      "{{ get_env(name=\"HOME\") }}",
      "{{ get_env(name=\"PORT\", default=\"8080\") }}"
    ],
    "syntax": {
      "function": true,
      "filter": false,
      "is_test": false
    }
  }
]
```

### Use Cases

- **IDE Plugins**: Provide autocomplete suggestions with argument hints and documentation
- **Language Servers**: Power hover documentation and signature help
- **Documentation Generators**: Automatically generate function reference documentation
- **Validation Tools**: Verify template function usage against available functions
- **CI/CD Integration**: Generate function lists for pipeline documentation

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
