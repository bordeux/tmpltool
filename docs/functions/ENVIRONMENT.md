## Environment Variables

Functions for accessing and filtering environment variables.

**See also:** [Function Reference](../FUNCTIONS.md) | [Main Documentation](../README.md)

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

