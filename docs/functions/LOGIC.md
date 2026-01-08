## Logic Functions

Logic functions: default values, coalesce, ternary operations, and range checks.

**See also:** [Function Reference](../FUNCTIONS.md) | [Main Documentation](../README.md)

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

