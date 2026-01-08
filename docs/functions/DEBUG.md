## Debugging & Development Functions

Debugging and development functions: type checking, assertions, and warnings.

**See also:** [Function Reference](../FUNCTIONS.md) | [Main Documentation](../README.md)

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

