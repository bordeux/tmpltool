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
  - [Filesystem Functions](#filesystem-functions)
  - [Data Parsing Functions](#data-parsing-functions)
  - [Validation Functions](#validation-functions)
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
- **Filesystem**: Read files, check existence, list directories, glob patterns, file info
- **Data Parsing**: Parse and read JSON, YAML, TOML files
- **Validation**: Validate emails, URLs, IPs, UUIDs, regex matching
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

**Common filters:**
- `upper`, `lower`, `title` - Case conversion
- `trim`, `truncate` - String operations
- `slugify` - Convert to URL-friendly slug
- `urlencode` - URL encoding
- `filesizeformat` - Format bytes (e.g., "1.5 KB")
- `date(format="%Y-%m-%d")` - Date formatting
- `split(pat=",")` - Split string into array
- `length` - Get array/string length

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
