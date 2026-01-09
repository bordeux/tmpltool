## Predicate Functions

Predicate functions for checking conditions on arrays and strings.

**See also:** [Function Reference](../FUNCTIONS.md) | [Main Documentation](../README.md)

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
  {% include "yaml-handler.tmpltool" %}
{% elif ends_with(string=filename, suffix=".json") %}
  JSON configuration file
  {% include "json-handler.tmpltool" %}
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

