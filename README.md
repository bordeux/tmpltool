# tmpltool

[![CI](https://github.com/bordeux/tmpltool/actions/workflows/ci.yml/badge.svg)](https://github.com/bordeux/tmpltool/actions/workflows/ci.yml)
[![Release](https://github.com/bordeux/tmpltool/actions/workflows/release.yml/badge.svg)](https://github.com/bordeux/tmpltool/actions/workflows/release.yml)
[![codecov](https://codecov.io/gh/bordeux/tmpltool/branch/master/graph/badge.svg)](https://codecov.io/gh/bordeux/tmpltool)
[![GitHub release](https://img.shields.io/github/v/release/bordeux/tmpltool)](https://github.com/bordeux/tmpltool/releases)
[![Docker](https://img.shields.io/badge/docker-ghcr.io-blue)](https://github.com/bordeux/tmpltool/pkgs/container/tmpltool)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A fast and simple command-line template rendering tool using [Tera](https://keats.github.io/tera/) templates with environment variables.

## Features

- Render Tera templates with environment variable support via `get_env()` function
- Filter environment variables by pattern with `filter_env()` function
- Cryptographic hash functions: `md5()`, `sha1()`, `sha256()`, `sha512()`
- UUID generation with `uuid()` function
- Random string generation with `random_string()` function
- Filesystem functions: `read_file()`, `file_exists()`, `list_dir()`, `glob()`, `file_size()`, `file_modified()`
- Output to file or stdout (for piping)
- Simple CLI interface
- Single binary executable
- Full Tera template syntax support (variables, conditionals, loops, filters, etc.)
- Built-in filters: slugify, date, urlencode, filesizeformat, and more
- Default values for environment variables (no errors when variables are missing)

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

Run with Docker:

```bash
# Using a template file
docker run --rm -v $(pwd):/workspace -w /workspace ghcr.io/bordeux/tmpltool:latest template.tmpl

# With environment variables
docker run --rm -e NAME=Alice -v $(pwd):/workspace -w /workspace \
  ghcr.io/bordeux/tmpltool:latest greeting.tmpl

# Output to file
docker run --rm -v $(pwd):/workspace -w /workspace \
  ghcr.io/bordeux/tmpltool:latest template.tmpl -o output.txt
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

## Usage

```bash
# Read from file
tmpltool [TEMPLATE] [OPTIONS]

# Read from stdin
cat template.txt | tmpltool [OPTIONS]
```

### Arguments

- `[TEMPLATE]` - Path to the template file (optional)
  - If omitted, reads template from stdin

### Options

- `-o, --output <FILE>` - Output file path (optional)
  - If not specified, output is printed to stdout
- `--trust` - Trust mode: Allow filesystem functions to access absolute paths and parent directories (optional)
  - **WARNING:** This disables security restrictions. Only use with trusted templates.
  - Without this flag, filesystem functions are restricted to relative paths within the current working directory
  - With this flag, you can access any file on the system (e.g., `/etc/passwd`, `../../secret.txt`)

### Input/Output Combinations

tmpltool supports all standard Unix I/O patterns:

| Input  | Output | Command |
|--------|--------|---------|
| File   | stdout | `tmpltool template.txt` |
| File   | File   | `tmpltool template.txt -o output.txt` |
| stdin  | stdout | `cat template.txt \| tmpltool` |
| stdin  | File   | `cat template.txt \| tmpltool -o output.txt` |

### Examples

#### File to stdout

```bash
tmpltool template.txt
```

#### File to file

```bash
tmpltool template.txt -o output.txt
```

#### stdin to stdout (pipe)

```bash
cat template.txt | tmpltool
echo "Hello {{ get_env(name=\"NAME\", default=\"World\") }}!" | tmpltool
```

#### stdin to file

```bash
cat template.txt | tmpltool -o output.txt
```

#### Chaining with other tools

```bash
# Generate and validate
tmpltool config.json.tmpl | jq .

# Generate from stdin and apply
cat k8s-deployment.yaml.tmpl | tmpltool | kubectl apply -f -

# Combine multiple templates
cat header.tmpl body.tmpl footer.tmpl | tmpltool > complete.html
```

#### Using Trust Mode for System Files

```bash
# Create a template that reads system files
cat > system_info.tmpl << 'EOF'
# System Information

## Hostname
{{ read_file(path="/etc/hostname") }}

## Hosts File (first 200 chars)
{{ read_file(path="/etc/hosts") | truncate(length=200) }}

## Files in /etc (first 10)
{% for file in list_dir(path="/etc") | slice(end=10) %}
- {{ file }}
{% endfor %}
EOF

# Without --trust: Security error
tmpltool system_info.tmpl
# Error: Security: Absolute paths and parent directory (..) access are not allowed

# With --trust: Works!
tmpltool --trust system_info.tmpl -o system_info.md
```

#### Using Environment Variables

Create a template file `greeting.tmpl`:
```
Hello {{ get_env(name="USER") }}!
Your home directory is: {{ get_env(name="HOME") }}
Your shell: {{ get_env(name="SHELL") }}
```

Render it:
```bash
tmpltool greeting.tmpl
```

Output:
```
Hello username!
Your home directory is: /home/username
Your shell: /bin/bash
```

**Note:** These environment variables (USER, HOME, SHELL) are typically available on Unix systems.

#### Setting Custom Environment Variables

Create a template `config.tmpl`:
```
Database: {{ get_env(name="DB_HOST", default="localhost") }}:{{ get_env(name="DB_PORT", default="5432") }}
Environment: {{ get_env(name="APP_ENV", default="development") }}
Debug: {{ get_env(name="DEBUG", default="false") }}
```

Render with custom variables:
```bash
DB_HOST=postgres DB_PORT=5432 APP_ENV=production DEBUG=true tmpltool config.tmpl
```

Output:
```
Database: postgres:5432
Environment: production
Debug: true
```

Or render without any environment variables (using defaults):
```bash
tmpltool config.tmpl
```

Output:
```
Database: localhost:5432
Environment: development
Debug: false
```

#### Using Conditionals

Template `status.tmpl`:
```
{% set debug = get_env(name="DEBUG", default="false") %}
{% if debug == "true" %}
DEBUG MODE ENABLED
Log level: verbose
{% else %}
Production mode
Log level: error
{% endif %}
```

Render with DEBUG enabled:
```bash
DEBUG=true tmpltool status.tmpl
```

Output:
```
DEBUG MODE ENABLED
Log level: verbose
```

Render without DEBUG (uses default):
```bash
tmpltool status.tmpl
```

Output:
```
Production mode
Log level: error
```

#### Using Loops

You can use environment variables with Tera's split filter to create lists:

Template `list.tmpl`:
```
{% set items_str = get_env(name="ITEMS", default="apple,banana,orange") %}
{% set items = items_str | split(pat=",") %}
Items:
{% for item in items %}
  - {{ item }}
{% endfor %}
```

Render with custom list:
```bash
ITEMS="apple,banana,orange,grape" tmpltool list.tmpl
```

Output:
```
Items:
  - apple
  - banana
  - orange
  - grape
```

Or use the default list:
```bash
tmpltool list.tmpl
```

Output:
```
Items:
  - apple
  - banana
  - orange
```

#### Using Filters

Template `formatted.tmpl`:
```
{% set name = get_env(name="NAME", default="john doe") %}
Uppercase: {{ name | upper }}
Lowercase: {{ name | lower }}
Title Case: {{ name | title }}
Slugified: {{ name | slugify }}
```

Render with custom name:
```bash
NAME="Jane Smith" tmpltool formatted.tmpl
```

Output:
```
Uppercase: JANE SMITH
Lowercase: jane smith
Title Case: Jane Smith
Slugified: jane-smith
```

Or use default:
```bash
tmpltool formatted.tmpl
```

Output:
```
Uppercase: JOHN DOE
Lowercase: john doe
Title Case: John Doe
Slugified: john-doe
```

#### Filtering Environment Variables by Pattern

Use the `filter_env()` function to get all environment variables matching a pattern:

Template `server-vars.tmpl`:
```
Server Configuration:
{% for var in filter_env(pattern="SERVER_*") %}
  {{ var.key }}={{ var.value }}
{% endfor %}
```

Set environment variables:
```bash
SERVER_HOST=localhost \
SERVER_PORT=8080 \
SERVER_NAME=myapp \
OTHER_VAR=ignored \
tmpltool server-vars.tmpl
```

Output:
```
Server Configuration:
  SERVER_HOST=localhost
  SERVER_NAME=myapp
  SERVER_PORT=8080
```

**Pattern Syntax:**
- `*` - matches any characters (e.g., `SERVER_*` matches `SERVER_HOST`, `SERVER_PORT`, etc.)
- `?` - matches exactly one character (e.g., `DB_?` matches `DB_A`, `DB_B`, but not `DB_AB`)
- Patterns can be at the beginning, middle, or end (e.g., `*_PORT`, `APP_*_NAME`)

The results are returned as an array of objects with `key` and `value` fields, sorted alphabetically by key.

#### Complex Example - Docker Compose Generator

Template `docker-compose.tmpl`:
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

Render with all custom values:
```bash
SERVICE_NAME=web \
DOCKER_IMAGE=node:20 \
HOST_PORT=8080 \
CONTAINER_PORT=3000 \
NODE_ENV=production \
DATABASE_URL=postgres://db:5432/mydb \
ENABLE_VOLUMES=true \
tmpltool docker-compose.tmpl -o docker-compose.yml
```

Or use all defaults (works out of the box!):
```bash
tmpltool docker-compose.tmpl -o docker-compose.yml
```

This generates a working docker-compose.yml with sensible defaults.

#### Pipeline Usage

Generate and validate JSON config:
```bash
tmpltool config.json.tmpl | jq .
```

Generate and apply Kubernetes config:
```bash
tmpltool k8s-deployment.yaml.tmpl | kubectl apply -f -
```

Generate nginx config and test it:
```bash
tmpltool nginx.conf.tmpl | nginx -t -c /dev/stdin
```

#### Comprehensive Example - All Features

This example demonstrates all tmpltool features in a single template:

Template `comprehensive-app-config.tmpl`:
```yaml
# Application Configuration
# Generated: {{ now() }}
# Instance ID: {{ uuid() }}

{# ============================================
   Service Configuration
   ============================================ #}
service:
  name: {{ get_env(name="APP_NAME", default="myapp") | upper }}
  version: {{ get_env(name="APP_VERSION", default="1.0.0") }}
  environment: {{ get_env(name="ENV", default="development") | upper }}

  # Unique identifiers
  instance_id: {{ uuid() }}
  deployment_id: {{ uuid() }}

{# ============================================
   Security & Authentication
   ============================================ #}
security:
  # Hash functions for integrity checks
  config_checksum: {{ md5(string="v1.0-config") }}
  license_hash: {{ sha256(string=get_env(name="LICENSE_KEY", default="trial-license")) }}

  # Generated secrets
  api_key: {{ random_string(length=32, charset="hex") }}
  secret_token: {{ random_string(length=64) }}
  csrf_token: {{ random_string(length=40, charset="hex") }}
  session_secret: {{ random_string(length=32, charset="alphanumeric") }}

  # Password hashing (example - use proper password hashing in production!)
  {% set admin_pwd = get_env(name="ADMIN_PASSWORD", default="changeme123") %}
  admin_password_hash: {{ sha512(string=admin_pwd) }}

{# ============================================
   Database Configuration
   ============================================ #}
database:
  # Filter all DB_* environment variables
  {% set db_vars = filter_env(pattern="DB_*") %}
  {% if db_vars | length > 0 %}
  # From environment:
  {% for var in db_vars %}
  {{ var.key | lower | replace(from="db_", to="") }}: {{ var.value }}
  {% endfor %}
  {% else %}
  # Default configuration:
  host: {{ get_env(name="DB_HOST", default="localhost") }}
  port: {{ get_env(name="DB_PORT", default="5432") }}
  name: {{ get_env(name="DB_NAME", default="myapp_db") }}
  user: {{ get_env(name="DB_USER", default="app_user") }}
  {% endif %}

  # Connection pool
  max_connections: {{ get_env(name="DB_MAX_CONNECTIONS", default="20") }}
  connection_id: {{ uuid() }}

{# ============================================
   Server Configuration
   ============================================ #}
server:
  {% set servers = filter_env(pattern="SERVER_*") %}
  {% if servers | length > 0 %}
  # Detected server configuration:
  {% for srv in servers %}
  {{ srv.key | lower | replace(from="server_", to="") }}: {{ srv.value }}
  {% endfor %}
  {% else %}
  # Default server configuration:
  host: {{ get_env(name="HOST", default="0.0.0.0") }}
  port: {{ get_env(name="PORT", default="8080") }}
  protocol: {{ get_env(name="PROTOCOL", default="http") }}
  {% endif %}

  # TLS/SSL
  {% set enable_tls = get_env(name="ENABLE_TLS", default="false") %}
  {% if enable_tls == "true" %}
  tls:
    enabled: true
    cert_path: {{ get_env(name="TLS_CERT_PATH", default="/etc/ssl/cert.pem") }}
    key_path: {{ get_env(name="TLS_KEY_PATH", default="/etc/ssl/key.pem") }}
  {% else %}
  tls:
    enabled: false
  {% endif %}

{# ============================================
   Logging Configuration
   ============================================ #}
logging:
  {% set env_type = get_env(name="ENV", default="development") %}
  {% if env_type == "production" %}
  level: ERROR
  format: json
  output: /var/log/app/production.log
  {% elif env_type == "staging" %}
  level: WARN
  format: json
  output: /var/log/app/staging.log
  {% else %}
  level: DEBUG
  format: text
  output: stdout
  {% endif %}

  # Log rotation ID
  rotation_id: {{ uuid() }}

{# ============================================
   Feature Flags
   ============================================ #}
features:
  {% set features = get_env(name="FEATURES", default="api,web,admin") | split(pat=",") %}
  enabled: [{% for feature in features %}"{{ feature | trim }}"{% if not loop.last %}, {% endif %}{% endfor %}]
  count: {{ features | length }}

  # Feature-specific settings
  {% for feature in features %}
  {{ feature | trim | slugify }}:
    enabled: true
    token: {{ random_string(length=16, charset="hex") }}
  {% endfor %}

{# ============================================
   External Services
   ============================================ #}
external_services:
  # All API_* environment variables
  {% set api_vars = filter_env(pattern="API_*") %}
  {% if api_vars | length > 0 %}
  apis:
  {% for api in api_vars %}
    {{ api.key | lower | replace(from="api_", to="") }}:
      url: {{ api.value }}
      key: {{ random_string(length=32, charset="hex") }}
      checksum: {{ md5(string=api.value) }}
  {% endfor %}
  {% else %}
  apis: []
  {% endif %}

{# ============================================
   Cache Configuration
   ============================================ #}
cache:
  {% set cache_type = get_env(name="CACHE_TYPE", default="memory") %}
  type: {{ cache_type }}
  ttl: {{ get_env(name="CACHE_TTL", default="3600") }}

  {% if cache_type == "redis" %}
  redis:
    host: {{ get_env(name="REDIS_HOST", default="localhost") }}
    port: {{ get_env(name="REDIS_PORT", default="6379") }}
    db: {{ get_env(name="REDIS_DB", default="0") }}
    password_hash: {{ sha256(string=get_env(name="REDIS_PASSWORD", default="")) }}
  {% endif %}

{# ============================================
   Monitoring & Metrics
   ============================================ #}
monitoring:
  enabled: {{ get_env(name="ENABLE_MONITORING", default="true") }}
  endpoint: {{ get_env(name="METRICS_ENDPOINT", default="/metrics") }}

  # Unique tracking IDs
  cluster_id: {{ uuid() }}
  node_id: {{ uuid() }}

  # Sample intervals (in seconds)
  {% set intervals = get_env(name="SAMPLE_INTERVALS", default="10,30,60") | split(pat=",") %}
  sample_intervals: [{% for interval in intervals %}{{ interval }}{% if not loop.last %}, {% endif %}{% endfor %}]

{# ============================================
   Recovery & Backup
   ============================================ #}
recovery:
  # Recovery codes (for 2FA backup)
  codes:
  {% for i in range(end=5) %}
    - {{ random_string(length=8, charset="uppercase") }}-{{ random_string(length=8, charset="uppercase") }}
  {% endfor %}

  # Backup encryption key
  backup_key: {{ random_string(length=64, charset="hex") }}
  backup_key_hash: {{ sha256(string=get_env(name="BACKUP_PASSPHRASE", default="default-passphrase")) }}

{# ============================================
   Metadata
   ============================================ #}
metadata:
  generated_at: {{ now() }}
  generated_by: tmpltool
  template_version: "2.0"
  config_hash: {{ sha1(string="comprehensive-config-v2.0") }}

  # All environment variables used
  environment_variables:
  {% set all_env = filter_env(pattern="*") %}
    total_count: {{ all_env | length }}
    app_vars: {{ filter_env(pattern="APP_*") | length }}
    db_vars: {{ filter_env(pattern="DB_*") | length }}
    server_vars: {{ filter_env(pattern="SERVER_*") | length }}
```

Set environment variables and render:
```bash
# Set application variables
export APP_NAME="mywebapp"
export APP_VERSION="2.1.0"
export ENV="production"

# Set database variables
export DB_HOST="db.example.com"
export DB_PORT="5432"
export DB_NAME="production_db"
export DB_USER="app_prod"
export DB_MAX_CONNECTIONS="50"

# Set server variables
export SERVER_HOST="api.example.com"
export SERVER_PORT="443"
export SERVER_PROTOCOL="https"

# Enable features
export ENABLE_TLS="true"
export TLS_CERT_PATH="/etc/ssl/certs/app.crt"
export TLS_KEY_PATH="/etc/ssl/private/app.key"

# Set security
export ADMIN_PASSWORD="SecureP@ssw0rd123"
export LICENSE_KEY="PROD-ABC123-XYZ789"

# Set features
export FEATURES="api,web,admin,analytics,reporting"

# External services
export API_PAYMENT_URL="https://api.payment.example.com"
export API_EMAIL_URL="https://api.email.example.com"

# Cache configuration
export CACHE_TYPE="redis"
export REDIS_HOST="cache.example.com"
export REDIS_PORT="6379"
export REDIS_PASSWORD="redis-secure-pass"

# Render the configuration
tmpltool comprehensive-app-config.tmpl -o app-config.yaml
```

This example demonstrates:
- ✅ All hash functions: `md5()`, `sha1()`, `sha256()`, `sha512()`
- ✅ UUID generation: `uuid()`
- ✅ Random strings: `random_string()` with various charsets
- ✅ Environment variables: `get_env()` with defaults
- ✅ Pattern filtering: `filter_env()`
- ✅ Conditionals: `if/elif/else`
- ✅ Loops: `for` loops with ranges and arrays
- ✅ Filters: `upper`, `lower`, `trim`, `slugify`, `replace`, `split`, `length`
- ✅ Comments: `{# ... #}`
- ✅ String operations: concatenation and formatting
- ✅ Complex logic: nested conditions and loops

**Note:** The comprehensive example does not include filesystem functions. For filesystem function examples, see the [Filesystem Functions](#filesystem-functions) section.

## Examples

The `examples/` directory contains ready-to-use template examples demonstrating various features:

- **`basic.tmpl`** - Basic variable substitution and conditionals
- **`greeting.tmpl`** - Simple greeting with `get_env()` function
- **`config.tmpl`** - Application configuration file generation
- **`docker-compose.tmpl`** - Docker Compose with sensible defaults
- **`config-with-defaults.tmpl`** - Advanced config using `get_env()` function (recommended)
- **`server-config.tmpl`** - Server configuration using `filter_env()` pattern matching
- **`hash-crypto.tmpl`** - Demonstrates all hash functions, UUID, and random string generation
- **`comprehensive-app-config.tmpl`** - Complete showcase of ALL features (recommended for learning)

### Try an Example

```bash
# Basic example with environment variables
CUSTOM_VAR="Hello World" tmpltool examples/basic.tmpl

# Greeting with defaults
tmpltool examples/greeting.tmpl

# Generate a docker-compose.yml with all defaults (works out of the box!)
tmpltool examples/docker-compose.tmpl -o docker-compose.yml

# Generate docker-compose.yml with custom values
SERVICE_NAME=web \
DATABASE_URL=postgres://db:5432/myapp \
ENABLE_VOLUMES=true \
tmpltool examples/docker-compose.tmpl -o docker-compose.yml

# Config with get_env() function and defaults
tmpltool examples/config-with-defaults.tmpl

# Hash and crypto functions
tmpltool examples/hash-crypto.tmpl

# Comprehensive example with ALL features (great for learning!)
tmpltool examples/comprehensive-app-config.tmpl

# Comprehensive example with environment variables
APP_NAME="MyWebApp" \
ENV="production" \
DB_HOST="db.example.com" \
FEATURES="api,web,admin" \
tmpltool examples/comprehensive-app-config.tmpl -o app-config.yaml
```

See the [examples/README.md](examples/README.md) for detailed documentation of each example.

## Template Syntax

tmpltool uses the [Tera](https://keats.github.io/tera/) template engine. Here are some common syntax patterns:

### Variables
```
{{ variable_name }}
```

**Note:** Environment variables are NOT automatically available as variables. Use the `get_env()` function to access them (see below).

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

### Loops
```
{% for item in items %}
  {{ item }}
{% endfor %}
```

### Filters
```
{{ variable | filter_name }}
{{ variable | filter_name(arg=value) }}
```

### Built-in `get_env()` Function

tmpltool uses Tera's built-in `get_env()` function for accessing environment variables with optional defaults:

```
{{ get_env(name="VARIABLE_NAME", default="fallback_value") }}
```

**Examples:**
```
# With default value (recommended)
port = {{ get_env(name="PORT", default="8080") }}
database = {{ get_env(name="DB_URL", default="postgres://localhost/mydb") }}

# Without default (will error if variable doesn't exist)
api_key = {{ get_env(name="API_KEY") }}

# Use in conditionals (requires {% set %} first)
{% set debug = get_env(name="DEBUG", default="false") %}
{% if debug == "true" %}
  Debug mode enabled
{% endif %}
```

**Benefits:**
- No template errors when environment variables are missing
- Sensible defaults for development
- Easy to override in production
- Self-documenting configuration

**Important Notes:**
- `get_env()` cannot be used directly in `{% if %}` conditions - use `{% set %}` to assign to a variable first
- Direct environment variable access (e.g., `{{ ENV_VAR }}`) is not supported - always use `get_env()`

See [examples/config-with-defaults.tmpl](examples/config-with-defaults.tmpl) for a complete example.

### Custom `filter_env()` Function

tmpltool provides a custom `filter_env()` function to filter environment variables by glob pattern:

```
{% for var in filter_env(pattern="PATTERN") %}
  {{ var.key }}={{ var.value }}
{% endfor %}
```

**Arguments:**
- `pattern` (required) - A glob pattern to match environment variable names
  - `*` matches any characters
  - `?` matches exactly one character

**Returns:**
- An array of objects, each with:
  - `key` - The environment variable name
  - `value` - The environment variable value
- Results are sorted alphabetically by key

**Examples:**
```
# Match all SERVER_* variables
{% for var in filter_env(pattern="SERVER_*") %}
export {{ var.key }}="{{ var.value }}"
{% endfor %}

# Match all database variables
{% set db_vars = filter_env(pattern="DATABASE_*") %}
{% if db_vars | length > 0 %}
Found {{ db_vars | length }} database variables
{% endif %}

# Match any variable ending with _PORT
{% for var in filter_env(pattern="*_PORT") %}
{{ var.key }}: {{ var.value }}
{% endfor %}
```

See [examples/server-config.tmpl](examples/server-config.tmpl) for a complete example.

### Hash Functions

tmpltool provides cryptographic hash functions for generating checksums and hashes:

#### `md5(string)`
Calculates MD5 hash of a string.

```
Checksum: {{ md5(string="hello world") }}
# Output: 5eb63bbbe01eeed093cb22bb8f5acdc3
```

#### `sha1(string)`
Calculates SHA1 hash of a string.

```
Hash: {{ sha1(string="tmpltool") }}
# Output: c054a2a60ca2fe935ea1056bd90386194116f14f
```

#### `sha256(string)`
Calculates SHA256 hash of a string (recommended for password hashing).

```
{% set password = get_env(name="PASSWORD", default="secret") %}
Password hash: {{ sha256(string=password) }}
# Output: fcf730b6d95236ecd3c9fc2d92d7b6b2bb061514961aec041d6c7a7192f592e4
```

#### `sha512(string)`
Calculates SHA512 hash of a string (most secure).

```
Secure hash: {{ sha512(string="secure-data") }}
# Output: a5c18d86d1d07cc2b22b12284e2f8e5b9705761003f149467995927e36f0e447ddfb158b89a28c0b4d5ac419c979c1fc435a3378b619aed1bab0d15c3b583db9
```

**Important:** These hash functions are for checksums and general-purpose hashing. For production password storage, use dedicated password hashing libraries with salt and proper key derivation functions (bcrypt, argon2, etc.).

### UUID Generation

#### `uuid()`
Generates a random UUID v4 (Universally Unique Identifier).

```
Request ID: {{ uuid() }}
Session ID: {{ uuid() }}
# Output:
# Request ID: c5b78641-89f8-4d04-a4c9-d53ba4d433f9
# Session ID: aabc7fe1-f8ed-45ff-944d-9c24f3823ac0
```

Each call to `uuid()` generates a unique identifier.

### Random String Generation

#### `random_string(length, charset)`
Generates a random string with customizable length and character set.

**Arguments:**
- `length` (required) - Length of the string to generate (1-10000)
- `charset` (optional) - Character set to use (default: `alphanumeric`)

**Character Set Presets:**
- `alphanumeric` - Letters (a-z, A-Z) and digits (0-9) - **default**
- `alphabetic` or `alpha` - Letters only (a-z, A-Z)
- `lowercase` or `lower` - Lowercase letters only (a-z)
- `uppercase` or `upper` - Uppercase letters only (A-Z)
- `numeric` or `digits` - Digits only (0-9)
- `hex` or `hexadecimal` - Hexadecimal characters (0-9, a-f)
- `hex_upper` - Hexadecimal uppercase (0-9, A-F)
- Custom string - Any custom character set (e.g., `"abc123"`)

**Examples:**
```
# Alphanumeric (default)
API Key: {{ random_string(length=32) }}
# Output: 0QY92XIYYKIvMVuVc8a7u8O4v19VacO9

# Lowercase only
Username: user_{{ random_string(length=8, charset="lowercase") }}
# Output: user_lvaycaxa

# Uppercase only
Code: {{ random_string(length=6, charset="uppercase") }}
# Output: YFVLRV

# Numeric only
PIN: {{ random_string(length=4, charset="numeric") }}
# Output: 5858

# Hexadecimal
Token: {{ random_string(length=16, charset="hex") }}
# Output: bd2954f90019649b

# Custom charset
Password: {{ random_string(length=12, charset="abc123") }}
# Output: 3bb3c31bb23c
```

**Practical Example - Secure Configuration:**
```yaml
application:
  instance_id: {{ uuid() }}
  secret_key: {{ random_string(length=64) }}
  api_token: {{ random_string(length=32, charset="hex") }}

security:
  password_hash: {{ sha256(string=get_env(name="PASSWORD")) }}
  csrf_token: {{ random_string(length=40, charset="hex") }}
```

See [examples/hash-crypto.tmpl](examples/hash-crypto.tmpl) for a complete example.

### Filesystem Functions

tmpltool provides secure filesystem functions for reading files and querying file information within templates. All filesystem functions enforce security restrictions to prevent unauthorized access.

**Security Note:** All filesystem functions only allow access to relative paths within the current working directory. Absolute paths (starting with `/`) and parent directory traversal (`..`) are explicitly blocked.

#### `read_file(path)`
Reads the content of a file into the template.

**Arguments:**
- `path` (required) - Relative path to the file to read

**Returns:** String containing the file content

**Examples:**
```
# Read a configuration file
{% set config = read_file(path="config.txt") %}
{{ config }}

# Read and include file content
License:
{{ read_file(path="LICENSE") }}

# Use with filters
First 100 chars: {{ read_file(path="README.md") | truncate(length=100) }}
```

#### `file_exists(path)`
Checks if a file exists at the specified path.

**Arguments:**
- `path` (required) - Relative path to check

**Returns:** Boolean (`true` if file exists, `false` otherwise)

**Examples:**
```
# Conditional file inclusion
{% if file_exists(path="custom-config.txt") %}
Custom config found!
{{ read_file(path="custom-config.txt") }}
{% else %}
Using default configuration
{% endif %}

# Check multiple files
{% set has_readme = file_exists(path="README.md") %}
{% set has_license = file_exists(path="LICENSE") %}
Documentation: {% if has_readme %}✓{% else %}✗{% endif %}
License: {% if has_license %}✓{% else %}✗{% endif %}
```

#### `list_dir(path)`
Lists all files and directories in a directory.

**Arguments:**
- `path` (required) - Relative path to the directory

**Returns:** Array of filenames (sorted alphabetically)

**Examples:**
```
# List files in a directory
Files in data/:
{% for file in list_dir(path="data") %}
  - {{ file }}
{% endfor %}

# Count files
{% set files = list_dir(path="templates") %}
Total templates: {{ files | length }}

# Filter by extension
{% set all_files = list_dir(path="src") %}
Rust files:
{% for file in all_files %}
{% if file is ending_with(".rs") %}
  - {{ file }}
{% endif %}
{% endfor %}
```

#### `glob(pattern)`
Lists all files matching a glob pattern.

**Arguments:**
- `pattern` (required) - Glob pattern to match files
  - `*` matches any characters
  - `?` matches exactly one character
  - `**` matches any number of directories

**Returns:** Array of file paths (sorted alphabetically)

**Examples:**
```
# Find all text files
Text files:
{% for file in glob(pattern="*.txt") %}
  - {{ file }}
{% endfor %}

# Find files in subdirectories
All Rust files:
{% for file in glob(pattern="src/**/*.rs") %}
  - {{ file }}
{% endfor %}

# Match specific patterns
Config files:
{% for file in glob(pattern="config*.{json,yaml,toml}") %}
  - {{ file }}
{% endfor %}

# Use in conditionals
{% set test_files = glob(pattern="tests/**/*.rs") %}
{% if test_files | length > 0 %}
Found {{ test_files | length }} test files
{% endif %}
```

#### `file_size(path)`
Gets the size of a file in bytes.

**Arguments:**
- `path` (required) - Relative path to the file

**Returns:** File size as a number (in bytes)

**Examples:**
```
# Get file size
README size: {{ file_size(path="README.md") }} bytes

# Format with built-in filter
README size: {{ file_size(path="README.md") | filesizeformat }}

# Compare file sizes
{% set size_a = file_size(path="file_a.txt") %}
{% set size_b = file_size(path="file_b.txt") %}
{% if size_a > size_b %}
file_a.txt is larger
{% else %}
file_b.txt is larger
{% endif %}

# Calculate total size
{% set files = glob(pattern="data/*.json") %}
{% set total_size = 0 %}
{% for file in files %}
{% set total_size = total_size + file_size(path=file) %}
{% endfor %}
Total data size: {{ total_size | filesizeformat }}
```

#### `file_modified(path)`
Gets the last modification time of a file as a Unix timestamp (seconds since epoch).

**Arguments:**
- `path` (required) - Relative path to the file

**Returns:** Unix timestamp (number of seconds since January 1, 1970)

**Examples:**
```
# Get modification timestamp
Last modified: {{ file_modified(path="config.json") }}

# Format with date filter
{% set timestamp = file_modified(path="README.md") %}
Last updated: {{ timestamp | date(format="%Y-%m-%d %H:%M:%S") }}

# Check if file is recent
{% set mod_time = file_modified(path="cache.dat") %}
{% set now_time = now() %}
{% set age_seconds = now_time - mod_time %}
{% if age_seconds < 3600 %}
Cache is fresh (less than 1 hour old)
{% else %}
Cache is stale ({{ age_seconds / 3600 }} hours old)
{% endif %}

# Find most recently modified file
{% set files = glob(pattern="logs/*.log") %}
{% set newest_time = 0 %}
{% set newest_file = "" %}
{% for file in files %}
{% set mod_time = file_modified(path=file) %}
{% if mod_time > newest_time %}
{% set newest_time = mod_time %}
{% set newest_file = file %}
{% endif %}
{% endfor %}
Most recent log: {{ newest_file }}
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
{% for test in test_files %}
- {{ test }}
{% endfor %}
```

**Security Restrictions:**

All filesystem functions enforce the following security rules:

1. **No Absolute Paths** - Paths starting with `/` are rejected
   ```
   {{ read_file(path="/etc/passwd") }}  # ✗ ERROR: Security violation
   ```

2. **No Parent Directory Traversal** - Paths containing `..` are rejected
   ```
   {{ read_file(path="../../secret.txt") }}  # ✗ ERROR: Security violation
   ```

3. **Relative Paths Only** - Only files within the current working directory are accessible
   ```
   {{ read_file(path="config.txt") }}        # ✓ OK
   {{ read_file(path="data/file.txt") }}     # ✓ OK
   {{ file_exists(path="subdir/test.txt") }} # ✓ OK
   ```

These restrictions ensure templates can only access files in the current working directory and its subdirectories, preventing unauthorized access to system files or files outside the project.

**Trust Mode:**

You can bypass these security restrictions by using the `--trust` command-line flag:

```bash
# Without --trust: Security error
tmpltool template.tmpl  # ERROR if template tries to read /etc/passwd

# With --trust: Unrestricted access
tmpltool --trust template.tmpl  # OK, can read any file
```

**When to use `--trust`:**
- When you need to access system files or configuration outside your project
- When reading files from absolute paths (e.g., `/etc/hosts`, `/var/log/app.log`)
- When accessing parent directories (e.g., `../config/settings.yml`)
- When you fully trust the template source and know what files it accesses

**WARNING:** Only use `--trust` with templates you completely trust. Malicious templates could read sensitive files like SSH keys, passwords, or system configurations.

### Comments
```
{# This is a comment #}
```

For complete Tera syntax documentation, visit: https://keats.github.io/tera/docs/

## Error Handling

- If a template file doesn't exist, tmpltool will exit with an error
- If a template has syntax errors, tmpltool will report the error location
- Environment variable handling:
  - **Direct access not supported:** `{{ ENV_VAR }}` will cause an error - environment variables are not automatically available
  - **With default (recommended):** `{{ get_env(name="VAR", default="...") }}` will use the default value if the variable doesn't exist
  - **Without default:** `{{ get_env(name="VAR") }}` will error if the variable doesn't exist

## Help

```bash
tmpltool --help
```

## Version

```bash
tmpltool --version
```

## Development

### Prerequisites

- Rust 1.70 or higher
- Cargo (comes with Rust)

Install Rust from [rustup.rs](https://rustup.rs/) if you haven't already:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Project Structure

```
tmpltool/
├── Cargo.toml              # Project dependencies and metadata
├── Cargo.lock              # Locked dependency versions
├── Makefile.toml           # cargo-make task definitions
├── src/
│   ├── main.rs             # Entry point (binary)
│   ├── lib.rs              # Library root
│   ├── cli.rs              # CLI argument parsing
│   ├── renderer.rs         # Template rendering logic
│   └── functions/          # Custom Tera functions (extensibility point)
│       └── mod.rs          # Functions module (for registering custom functions)
├── tests/                  # Integration tests (one test per file)
│   ├── common.rs           # Shared test utilities
│   ├── fixtures/           # Test fixtures (templates & expected outputs)
│   │   ├── templates/      # Input template files
│   │   ├── expected/       # Expected output files
│   │   └── README.md       # Fixtures documentation
│   ├── test_successful_rendering.rs
│   ├── test_missing_template_file.rs
│   ├── test_invalid_template_syntax.rs
│   ├── test_environment_variable_substitution.rs
│   ├── test_template_with_conditionals.rs
│   ├── test_template_with_missing_variable.rs
│   ├── test_multiline_template.rs
│   ├── test_stdout_output.rs
│   └── test_direct_var_access_fails.rs
├── examples/               # Example templates
│   ├── basic.tmpl          # Basic usage example
│   ├── greeting.tmpl       # Simple greeting with defaults
│   ├── config.tmpl         # Config file generation
│   ├── config-with-defaults.tmpl # Advanced config with get_env() function
│   ├── docker-compose.tmpl # Docker Compose with defaults
│   └── README.md           # Examples documentation
├── .gitignore              # Git ignore rules
├── .editorconfig           # Editor configuration
└── README.md               # This file
```

#### Module Organization

**Source Code (`src/`)** - No tests in source files:
- **`main.rs`** - Minimal binary entry point, just parses CLI args and calls the library
- **`lib.rs`** - Public library API, exports main functionality
- **`cli.rs`** - CLI argument parsing using clap
- **`renderer.rs`** - Core template rendering logic (no unit tests)
- **`functions/`** - Custom Tera functions (modular, one file per function)
  - **`mod.rs`** - Registers custom functions (currently empty - built-in functions like `get_env()` work automatically)

**Tests (`tests/`)** - All tests as integration tests:
- **`common.rs`** - Shared test utilities and fixture helpers
- **`fixtures/`** - Test fixtures (templates and expected outputs)
- **Individual test files** - One test per file for better organization (11 tests total)

#### Adding New Custom Functions

To add a new custom function:

1. Create a new file in `src/functions/` (e.g., `my_function.rs`)
2. Implement your function following the Tera function signature
3. Add the module declaration to `src/functions/mod.rs`
4. Register your function in the `register_all()` function
5. Write tests in your function file

Note: Tera's built-in functions like `get_env()`, `now()`, and `get_random()` are automatically available when the "builtins" feature is enabled.

### Building

Build the project in debug mode:

```bash
cargo build
```

Build optimized release binary:

```bash
cargo build --release
```

The release binary will be located at `./target/release/tmpltool`.

### Using cargo-make (Task Runner)

This project includes a comprehensive `Makefile.toml` for [cargo-make](https://github.com/sagiegurari/cargo-make), providing standardized tasks for building, testing, and cross-platform compilation.

#### Installation

Install cargo-make globally:

```bash
cargo install --force cargo-make
```

#### Available Tasks

View all available tasks:

```bash
cargo make
```

**Common Development Tasks:**

```bash
# Build and test
cargo make build              # Build debug binary
cargo make build-release      # Build optimized binary
cargo make test              # Run all tests
cargo make test-verbose      # Run tests with output
cargo make run               # Run with example template

# Code quality
cargo make format            # Format code with rustfmt
cargo make clippy            # Run clippy linter
cargo make qa                # Full quality check (format + clippy + test)
cargo make ci                # CI checks (format-check + clippy + test)

# Utilities
cargo make clean             # Clean build artifacts
cargo make check             # Fast compile check
cargo make docs              # Generate and open documentation
cargo make test-examples     # Test all example templates
```

**Cross-Platform Builds:**

Build release binaries for different platforms:

```bash
# Individual platforms
cargo make build-linux-x86_64      # Linux x86_64
cargo make build-linux-musl        # Linux (static, musl libc)
cargo make build-macos-x86_64      # macOS Intel
cargo make build-macos-aarch64     # macOS Apple Silicon
cargo make build-windows-x86_64    # Windows x86_64

# Build for all platforms
cargo make build-all-platforms
```

**Note:** Cross-compilation may require installing additional targets:

```bash
# Add targets for cross-compilation
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-unknown-linux-musl
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
rustup target add x86_64-pc-windows-gnu
```

**Workflow Tasks:**

```bash
cargo make dev                # Quick dev check (check + test)
cargo make pre-commit         # Pre-commit checks
cargo make release-prepare    # Full release preparation
cargo make all                # Complete build and test suite
```

**Additional Tools:**

```bash
cargo make audit              # Security audit (requires cargo-audit)
cargo make outdated           # Check outdated dependencies (requires cargo-outdated)
cargo make bloat              # Analyze binary size (requires cargo-bloat)
```

### Running in Development

Run without building a binary:

```bash
cargo run -- template.txt -o output.txt
```

Run with environment variables:

```bash
CUSTOM_VAR="test" cargo run -- template.txt
```

### Testing

Run all unit tests:

```bash
cargo test
```

Run tests with output:

```bash
cargo test -- --nocapture
```

Run a specific test:

```bash
cargo test test_successful_rendering
```

Run tests in verbose mode:

```bash
cargo test -- --test-threads=1 --nocapture
```

#### Test Coverage

The project includes comprehensive test coverage. **All tests are located in `tests/` directory** - there are no unit tests in `src/` files.

**Integration Tests in `tests/`** (11 tests, one per file):
- `test_simple_rendering.rs` - Simple static template rendering
- `test_successful_rendering.rs` - Template rendering with environment variables
- `test_env_with_default.rs` - Environment variable with default fallback
- `test_missing_template_file.rs` - Missing template file handling
- `test_invalid_template_syntax.rs` - Invalid template syntax handling
- `test_environment_variable_substitution.rs` - Environment variable substitution with `get_env()`
- `test_template_with_conditionals.rs` - Conditional logic (if/else)
- `test_template_with_missing_variable.rs` - Missing variable detection
- `test_multiline_template.rs` - Multiline templates
- `test_stdout_output.rs` - Stdout output functionality
- `test_direct_var_access_fails.rs` - Direct variable access fails (security test)

**Unit Tests in `tests/`** (58 tests across multiple test files):
- `test_filter_env_unit.rs` - Environment variable filtering (6 tests)
- `test_hash_unit.rs` - Hash functions (6 tests)
- `test_uuid_unit.rs` - UUID generation (3 tests)
- `test_random_string_unit.rs` - Random string generation (11 tests)
- `test_filesystem_unit.rs` - Filesystem functions (23 tests)
- `test_hash_crypto_functions.rs` - Hash and crypto integration (17 tests)
- `test_comprehensive.rs` - Comprehensive template validation (2 tests)

**Test Infrastructure:**
- `common.rs` - Shared test utilities and fixture helpers
- `fixtures/` - Test fixtures (templates and expected outputs)

**Documentation Tests** (2 tests):
- Library documentation examples

Total: **71 tests** covering integration, unit tests, and documentation scenarios.

#### Adding New Integration Tests

To add a new integration test:

1. Create a new file in `tests/` (e.g., `tests/test_my_feature.rs`)
2. Import the common utilities: `mod common;` and `use common::*;`
3. Import dependencies: `use tmpltool::render_template;`
4. Write your test function with `#[test]` attribute
5. Use helper functions: `get_test_file_path()` and `cleanup_test_file()`

**Example:**

```rust
mod common;

use common::{cleanup_test_file, get_test_file_path};
use std::fs;
use tmpltool::render_template;

#[test]
fn test_my_feature() {
    let template_path = get_test_file_path("my_template.txt");
    let output_path = get_test_file_path("my_output.txt");

    // Create test template
    fs::write(&template_path, "{{ get_env(name=\"TEST\") }}").unwrap();

    // Run render_template
    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
    );

    // Assert results
    assert!(result.is_ok());

    // Cleanup
    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}
```

Each test file is compiled as a separate test binary, making tests more isolated and easier to debug.

#### Using Test Fixtures

The project uses test fixtures to make tests easier to maintain. Fixtures are template files and their expected outputs stored in `tests/fixtures/`.

**Fixture Directory Structure:**

```
tests/fixtures/
├── templates/           # Input template files
│   ├── simple.tmpl
│   ├── with_env.tmpl
│   ├── multiline.tmpl
│   ├── conditionals.tmpl
│   └── docker-compose.tmpl
└── expected/            # Expected output files
    ├── simple.txt
    ├── with_env.txt
    ├── multiline.txt
    └── docker-compose.txt
```

**Using Fixtures in Tests:**

```rust
mod common;

use common::{
    cleanup_test_file, get_test_file_path,
    read_fixture_expected, read_fixture_template
};
use std::fs;
use tmpltool::render_template;

#[test]
fn test_my_feature() {
    let output_path = get_test_file_path("output.txt");

    // Read template from fixtures
    let template_content = read_fixture_template("my_template.tmpl");
    let template_path = get_test_file_path("template.txt");
    fs::write(&template_path, template_content).unwrap();

    // Render template
    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
    );

    // Compare with expected output
    assert!(result.is_ok());
    let output = fs::read_to_string(&output_path).unwrap();
    let expected = read_fixture_expected("my_template.txt");
    assert_eq!(output, expected);

    // Cleanup
    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}
```

**Benefits:**
- ✅ Test data separated from test logic
- ✅ Easy to maintain and review template changes
- ✅ Reusable across multiple tests
- ✅ Can use real-world template examples

See [tests/fixtures/README.md](tests/fixtures/README.md) for more details.

### Code Quality

Format code:

```bash
cargo fmt
```

Run linter (clippy):

```bash
cargo clippy
```

Run clippy with all warnings:

```bash
cargo clippy -- -W clippy::all
```

### Dependencies

The project uses minimal dependencies:

- **[tera](https://crates.io/crates/tera)** (v1.x) - Template engine with `builtins` feature enabled
  - Provides built-in filters: `slugify`, `date`, `filesizeformat`, `urlencode`, etc.
  - Provides built-in functions: `get_env()`, `now()`, `get_random()`
- **[clap](https://crates.io/crates/clap)** (v4.x) - Command-line argument parsing
- **[regex](https://crates.io/crates/regex)** (v1.x) - Regular expressions for pattern matching
- **[md-5](https://crates.io/crates/md-5)** (v0.10) - MD5 hash implementation
- **[sha1](https://crates.io/crates/sha1)** (v0.10) - SHA1 hash implementation
- **[sha2](https://crates.io/crates/sha2)** (v0.10) - SHA256 and SHA512 hash implementations
- **[uuid](https://crates.io/crates/uuid)** (v1.x) - UUID generation
- **[rand](https://crates.io/crates/rand)** (v0.8) - Random number generation
- **[glob](https://crates.io/crates/glob)** (v0.3) - Glob pattern matching for filesystem operations

To update dependencies:

```bash
cargo update
```

### Manual Testing

Create a test template:

```bash
cat > test.tmpl << 'EOF'
User: {{ USER }}
Home: {{ HOME }}
Custom: {{ CUSTOM_VAR }}
EOF
```

Test it:

```bash
CUSTOM_VAR="Hello World" ./target/release/tmpltool test.tmpl
```

### Debugging

Run with debug output:

```bash
RUST_BACKTRACE=1 cargo run -- template.txt
```

Full backtrace:

```bash
RUST_BACKTRACE=full cargo run -- template.txt
```

### Performance

Benchmark the binary size:

```bash
ls -lh target/release/tmpltool
```

Profile with release build:

```bash
cargo build --release
time ./target/release/tmpltool large_template.txt -o output.txt
```

### Installing Locally

Install from the project directory:

```bash
cargo install --path .
```

This installs the binary to `~/.cargo/bin/tmpltool` (make sure this is in your PATH).

Uninstall:

```bash
cargo uninstall tmpltool
```

### Using as a Library

tmpltool can also be used as a library in other Rust projects:

```rust
use tmpltool::render_template;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Render to file
    render_template("template.txt", Some("output.txt"))?;

    // Render to stdout
    render_template("template.txt", None)?;

    Ok(())
}
```

The library exposes:
- `render_template(template_path: &str, output_path: Option<&str>)` - Main rendering function
- `Cli` - Command-line argument structure

## CI/CD

This project uses GitHub Actions for continuous integration and automated releases.

### Continuous Integration

Every pull request and push to master/main triggers:

- **Code Formatting Check** - Ensures code follows Rust style guidelines (`rustfmt`)
- **Linting** - Runs `clippy` with strict warnings
- **Multi-Platform Tests** - Tests on Ubuntu, macOS, and Windows
- **Code Coverage** - Generates coverage reports with `cargo-tarpaulin` and uploads to Codecov
- **cargo-make QA** - Runs comprehensive quality checks
- **Example Testing** - Tests all example templates to ensure they work

### Automated Releases

Releases are fully automated using [semantic-release](https://github.com/semantic-release/semantic-release):

1. **Commit Analysis** - Analyzes commit messages to determine the next version
2. **Version Bumping** - Updates `Cargo.toml` with the new version
3. **CHANGELOG Generation** - Automatically generates `CHANGELOG.md` from commits
4. **Multi-Platform Builds** - Builds release binaries for:
   - Linux (x86_64, x86_64-musl, aarch64)
   - macOS (x86_64 Intel, aarch64 Apple Silicon)
   - Windows (x86_64)
5. **GitHub Release** - Creates a new GitHub release with all binaries
6. **Docker Image** - Builds and publishes multi-arch Docker image to GHCR

### Commit Convention

This project follows [Conventional Commits](https://www.conventionalcommits.org/) for automatic versioning:

- `feat: description` - New feature (minor version bump: 1.2.0 → 1.3.0)
- `fix: description` - Bug fix (patch version bump: 1.2.0 → 1.2.1)
- `feat!: description` or `BREAKING CHANGE:` - Breaking change (major version bump: 1.2.0 → 2.0.0)
- `docs:`, `refactor:`, `perf:`, `build:` - Other changes (patch bump)
- `style:`, `test:`, `chore:`, `ci:` - No version bump

**Examples:**

```bash
# Feature (minor bump)
git commit -m "feat: add slugify filter support"

# Bug fix (patch bump)
git commit -m "fix: correct multiline template rendering"

# Breaking change (major bump)
git commit -m "feat!: change default output behavior

BREAKING CHANGE: Output now goes to stdout by default instead of file"
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed contribution guidelines.

## License

This project is open source.

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

For detailed contribution guidelines, including commit conventions, development workflow, and testing requirements, see [CONTRIBUTING.md](CONTRIBUTING.md).

### Quick Start

1. Fork the repository
2. Clone your fork and install dependencies:
   ```bash
   git clone https://github.com/bordeux/tmpltool.git
   cd tmpltool
   npm install  # Installs commit validation hooks
   ```
3. Create a feature branch (`git checkout -b feature/amazing-feature`)
4. Make your changes
5. Run tests and QA checks (`cargo make qa`)
6. Commit using [conventional commits](#commit-convention) - invalid commits will be automatically rejected
7. Push to your fork
8. Open a Pull Request

**Note:** Commit messages are automatically validated. If your commit is rejected, make sure it follows the [conventional commit format](#commit-convention).
