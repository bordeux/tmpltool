# Template Syntax

tmpltool uses the [MiniJinja](https://github.com/mitsuhiko/minijinja) template engine, which is compatible with Python's Jinja2. For complete documentation, visit: https://docs.rs/minijinja/

## Variables

```
{{ variable_name }}
```

**Note:** Environment variables are NOT automatically available. Use the `get_env()` function to access them.

## Conditionals

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

## Is Syntax (Validation & Checks)

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

## Loops

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

## Filters

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

## Comments

```
{# This is a comment #}
```

## Setting Variables

```
{% set variable_name = value %}
{% set name = get_env(name="USER", default="guest") %}
```

## Template Includes

Template includes allow you to modularize your templates by splitting them into reusable partials. This promotes code reuse and keeps templates maintainable.

### Basic Syntax

```jinja
{% include "./path/to/partial.tmpl" %}
```

**Important:** Always use relative paths starting with `./` for includes.

### Same Directory Include

Include a template from the same directory:

```
templates/
├── main.tmpl
└── header.tmpl
```

**main.tmpl:**
```jinja
{% include "./header.tmpl" %}
Main content here
```

**header.tmpl:**
```jinja
=== Header ===
```

**Output:**
```
=== Header ===
Main content here
```

### Subdirectory Include

Include templates from subdirectories:

```
templates/
├── main.tmpl
└── partials/
    ├── header.tmpl
    └── footer.tmpl
```

**main.tmpl:**
```jinja
{% include "./partials/header.tmpl" %}
Main content
{% include "./partials/footer.tmpl" %}
```

### Nested Includes

Includes can be nested—an included template can include other templates:

```
templates/
├── main.tmpl
├── layout.tmpl
└── components/
    └── nav.tmpl
```

**main.tmpl:**
```jinja
{% include "./layout.tmpl" %}
```

**layout.tmpl:**
```jinja
<header>
{% include "./components/nav.tmpl" %}
</header>
<main>Content</main>
```

**components/nav.tmpl:**
```jinja
<nav>Navigation menu</nav>
```

**Output:**
```html
<header>
<nav>Navigation menu</nav>
</header>
<main>Content</main>
```

### Multiple Includes

Combine multiple partials to build complex configurations:

```jinja
{% include "./partials/header.tmpl" %}
{% include "./partials/database.tmpl" %}
{% include "./partials/logging.tmpl" %}
{% include "./partials/footer.tmpl" %}
```

### Conditional Includes

Use conditionals to include templates based on environment variables:

```jinja
{% set env = get_env(name="ENVIRONMENT", default="development") %}

{% if env == "production" %}
{% include "./configs/production.tmpl" %}
{% elif env == "staging" %}
{% include "./configs/staging.tmpl" %}
{% else %}
{% include "./configs/development.tmpl" %}
{% endif %}
```

Or toggle optional sections:

```jinja
{% set enable_monitoring = get_env(name="ENABLE_MONITORING", default="false") %}

{% if enable_monitoring == "true" %}
{% include "./partials/monitoring.tmpl" %}
{% endif %}
```

### Variables in Included Templates

Included templates have access to all variables defined in the parent template:

**main.tmpl:**
```jinja
{% set app_name = get_env(name="APP_NAME", default="myapp") %}
{% set port = get_env(name="PORT", default="8080") %}
{% include "./server-config.tmpl" %}
```

**server-config.tmpl:**
```jinja
server:
  name: {{ app_name }}
  port: {{ port }}
```

Included templates can also use all tmpltool functions like `get_env()`, `read_file()`, etc.

### Security Restrictions

By default, includes are restricted for security:

| Path Type | Default Mode | Trust Mode (`--trust`) |
|-----------|--------------|------------------------|
| Same directory (`./file.tmpl`) | ✅ Allowed | ✅ Allowed |
| Subdirectory (`./sub/file.tmpl`) | ✅ Allowed | ✅ Allowed |
| Parent directory (`../file.tmpl`) | ❌ Blocked | ✅ Allowed |
| Absolute path (`/etc/file`) | ❌ Blocked | ✅ Allowed |

**Parent directory access blocked (default):**
```jinja
{# This will fail without --trust #}
{% include "../shared/common.tmpl" %}
```

**Error:**
```
Security: Parent directory (..) traversal is not allowed: ../shared/common.tmpl.
Use --trust to bypass this restriction.
```

**Enable with trust mode:**
```bash
tmpltool --trust template.tmpl
```

### Real-World Example: Docker Compose

Organize a Docker Compose template with reusable service partials:

```
templates/
├── docker-compose.tmpl
└── services/
    ├── web.tmpl
    ├── database.tmpl
    └── redis.tmpl
```

**docker-compose.tmpl:**
```jinja
version: "3.8"

services:
{% include "./services/web.tmpl" %}

{% set use_db = get_env(name="USE_DATABASE", default="true") %}
{% if use_db == "true" %}
{% include "./services/database.tmpl" %}
{% endif %}

{% set use_cache = get_env(name="USE_CACHE", default="false") %}
{% if use_cache == "true" %}
{% include "./services/redis.tmpl" %}
{% endif %}
```

**services/web.tmpl:**
```jinja
  web:
    image: {{ get_env(name="WEB_IMAGE", default="nginx:latest") }}
    ports:
      - "{{ get_env(name="WEB_PORT", default="80") }}:80"
```

**services/database.tmpl:**
```jinja
  database:
    image: postgres:15
    environment:
      POSTGRES_DB: {{ get_env(name="DB_NAME", default="app") }}
      POSTGRES_USER: {{ get_env(name="DB_USER", default="postgres") }}
```

**Render:**
```bash
WEB_PORT=8080 USE_CACHE=true tmpltool templates/docker-compose.tmpl
```

### Real-World Example: Kubernetes Manifests

Split Kubernetes manifests into reusable components:

```
k8s/
├── deployment.tmpl
└── components/
    ├── metadata.tmpl
    ├── containers.tmpl
    └── resources.tmpl
```

**deployment.tmpl:**
```jinja
apiVersion: apps/v1
kind: Deployment
{% include "./components/metadata.tmpl" %}
spec:
  replicas: {{ get_env(name="REPLICAS", default="3") }}
  template:
    spec:
      containers:
{% include "./components/containers.tmpl" %}
```

**components/metadata.tmpl:**
```jinja
metadata:
  name: {{ get_env(name="APP_NAME", default="myapp") }}
  namespace: {{ get_env(name="NAMESPACE", default="default") }}
  labels:
    app: {{ get_env(name="APP_NAME", default="myapp") }}
```

### Best Practices

1. **Use `./` prefix** - Always start include paths with `./` for clarity
2. **Organize partials** - Keep partials in subdirectories like `partials/`, `components/`, or `includes/`
3. **Name clearly** - Use descriptive names like `header.tmpl`, `db-config.tmpl`
4. **Keep partials focused** - Each partial should do one thing well
5. **Document dependencies** - Comment which variables a partial expects
6. **Avoid deep nesting** - Limit include depth for maintainability (2-3 levels max)
