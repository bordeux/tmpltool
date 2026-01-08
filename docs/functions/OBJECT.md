## Object Manipulation Functions

Object manipulation: merge, get/set nested values, flatten, and transform objects.

**See also:** [Function Reference](../FUNCTIONS.md) | [Main Documentation](../README.md)

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

