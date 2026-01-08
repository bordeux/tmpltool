# Data Parsing & Serialization Functions

This document covers both data parsing (reading JSON, YAML, TOML) and data serialization (converting objects to strings).

## Data Parsing Functions

Parse and serialize JSON, YAML, and TOML data formats.

**See also:** [Function Reference](../FUNCTIONS.md) | [Main Documentation](../README.md)

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

## Data Serialization Functions

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

