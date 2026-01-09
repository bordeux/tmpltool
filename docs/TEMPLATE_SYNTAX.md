# Template Syntax

tmpltool uses the [MiniJinja](https://github.com/mitsuhiko/minijinja) template engine, which is compatible with Python's Jinja2. For complete documentation, visit: https://docs.rs/minijinja/

## Table of Contents

- [Syntax Overview](#syntax-overview)
- [Variables](#variables)
- [Expressions & Operators](#expressions--operators)
- [Template Tags Reference](#template-tags-reference) — if, for, set, with, filter, raw, autoescape, do
- [Whitespace Control](#whitespace-control)
- [Is Syntax (Validation & Checks)](#is-syntax-validation--checks)
- [Comments](#comments)
- [Filters](#filters)
- [Template Includes](#template-includes)

---

## Syntax Overview

Templates contain three types of delimiters:

| Delimiter | Purpose | Example |
|-----------|---------|---------|
| `{{ ... }}` | Expressions (output) | `{{ variable }}` |
| `{% ... %}` | Statements (logic) | `{% if condition %}` |
| `{# ... #}` | Comments (ignored) | `{# note #}` |

## Variables

```jinja
{{ variable_name }}
```

**Note:** Environment variables are NOT automatically available. Use the `get_env()` function to access them:

```jinja
{{ get_env(name="MY_VAR", default="fallback") }}
```

---

## Expressions & Operators

### Literals

```jinja
{{ "string" }}           {# String #}
{{ 42 }}                 {# Integer #}
{{ 3.14 }}               {# Float #}
{{ true }} {{ false }}   {# Booleans #}
{{ none }}               {# Null/None #}
{{ [1, 2, 3] }}          {# Array/List #}
{{ {"key": "value"} }}   {# Object/Map #}
```

### Arithmetic Operators

```jinja
{{ 10 + 5 }}    {# 15 - Addition #}
{{ 10 - 5 }}    {# 5 - Subtraction #}
{{ 10 * 5 }}    {# 50 - Multiplication #}
{{ 10 / 3 }}    {# 3.33... - Division #}
{{ 10 // 3 }}   {# 3 - Integer division #}
{{ 10 % 3 }}    {# 1 - Modulo #}
{{ 2 ** 8 }}    {# 256 - Power #}
```

### Comparison Operators

```jinja
{{ a == b }}    {# Equal #}
{{ a != b }}    {# Not equal #}
{{ a < b }}     {# Less than #}
{{ a <= b }}    {# Less than or equal #}
{{ a > b }}     {# Greater than #}
{{ a >= b }}    {# Greater than or equal #}
```

### Logical Operators

```jinja
{{ a and b }}   {# Logical AND #}
{{ a or b }}    {# Logical OR #}
{{ not a }}     {# Logical NOT #}
```

### Other Operators

```jinja
{# String concatenation #}
{{ "Hello, " ~ name ~ "!" }}

{# Membership test #}
{{ "foo" in ["foo", "bar"] }}     {# true #}
{{ "x" not in "abc" }}            {# true #}

{# Ternary/conditional expression #}
{{ "yes" if condition else "no" }}

{# Default value (if undefined or none) #}
{{ value | default("fallback") }}

{# Attribute access #}
{{ object.property }}
{{ object["property"] }}

{# Array indexing #}
{{ array[0] }}
{{ array[-1] }}    {# Last element #}

{# Slicing #}
{{ array[1:3] }}   {# Elements 1 and 2 #}
{{ array[:2] }}    {# First 2 elements #}
{{ array[2:] }}    {# From index 2 to end #}
```

---

## Template Tags Reference

### {% if %} — Conditionals

Conditional statement execution with multiple branches.

```jinja
{% if condition %}
  content when true
{% elif other_condition %}
  alternative content
{% else %}
  fallback content
{% endif %}
```

**Important:** `get_env()` cannot be used directly in `{% if %}` conditions. Use `{% set %}` to assign to a variable first:

```jinja
{% set debug = get_env(name="DEBUG", default="false") %}
{% if debug == "true" %}
  Debug mode enabled
{% endif %}
```

**Truthiness:** Empty strings, `0`, `false`, `none`, and empty collections are falsy. Everything else is truthy.

### {% for %} — Loops

Loop over sequences (arrays, objects, ranges).

```jinja
{% for item in items %}
  {{ item }}
{% endfor %}
```

**Loop Variables:**

| Variable | Description |
|----------|-------------|
| `loop.index` | Current iteration (1-indexed) |
| `loop.index0` | Current iteration (0-indexed) |
| `loop.first` | True if first iteration |
| `loop.last` | True if last iteration |
| `loop.length` | Total number of items |
| `loop.revindex` | Iterations remaining (1-indexed) |
| `loop.revindex0` | Iterations remaining (0-indexed) |
| `loop.depth` | Nesting level (starts at 1) |
| `loop.depth0` | Nesting level (starts at 0) |

**Examples:**

```jinja
{# Basic loop with index #}
{% for user in users %}
  {{ loop.index }}. {{ user.name }}
{% endfor %}

{# Alternating row styles #}
{% for row in rows %}
  <tr class="{{ loop.cycle('odd', 'even') }}">
    <td>{{ row }}</td>
  </tr>
{% endfor %}

{# Loop with else (empty collection) #}
{% for item in items %}
  {{ item }}
{% else %}
  No items found.
{% endfor %}

{# Unpacking tuples/pairs #}
{% for key, value in object | items %}
  {{ key }}: {{ value }}
{% endfor %}

{# Filtering during iteration #}
{% for user in users if user.active %}
  {{ user.name }}
{% endfor %}

{# Range loop #}
{% for i in range(5) %}
  Item {{ i }}
{% endfor %}
```

### {% set %} — Variable Assignment

Assign values to variables in the current scope.

```jinja
{% set variable = value %}
{% set name = get_env(name="USER", default="guest") %}
```

**Unpacking:**

```jinja
{% set a, b = [1, 2] %}
{% set first, rest = items | first, items | slice(1) %}
```

**Block form** (capture template output):

```jinja
{% set navigation %}
  <nav>
    <a href="/">Home</a>
    <a href="/about">About</a>
  </nav>
{% endset %}

{# Use later #}
{{ navigation }}
```

**With filter:**

```jinja
{% set title | upper %}
  page title
{% endset %}
{# title = "PAGE TITLE" #}
```

### {% with %} — Scoped Variables

Create a new inner scope for temporary variables.

```jinja
{% with %}
  {% set temp = "value" %}
  {{ temp }}  {# works here #}
{% endwith %}
{# temp is not accessible here #}
```

**Inline assignment:**

```jinja
{% with foo = 42, bar = "hello" %}
  {{ foo }} - {{ bar }}
{% endwith %}
```

### {% filter %} — Block Filters

Apply filters to entire blocks of content.

```jinja
{% filter upper %}
  this text will be uppercase
{% endfilter %}

{% filter indent(width=4) %}
line 1
line 2
{% endfilter %}
```

### {% raw %} — Escape Template Syntax

Output literal template syntax without processing.

```jinja
{% raw %}
  This {{ will not }} be processed.
  {% for item in seq %} is literal text.
{% endraw %}
```

Useful when generating templates or documentation containing Jinja syntax.

### {% autoescape %} — Control Auto-Escaping

Toggle HTML auto-escaping within template regions.

```jinja
{% autoescape true %}
  {{ user_input }}  {# HTML entities escaped #}
{% endautoescape %}

{% autoescape false %}
  {{ trusted_html }}  {# Output as-is #}
{% endautoescape %}
```

### {% do %} — Silent Execution

Execute expressions without outputting results (useful for side-effects).

```jinja
{% do my_list.append(item) %}
```

---

## Whitespace Control

Control whitespace around template tags using `-`:

```jinja
{# Remove whitespace before tag #}
{%- if true %}

{# Remove whitespace after tag #}
{% if true -%}

{# Remove whitespace on both sides #}
{%- if true -%}

{# Also works with expressions #}
{{- variable -}}
```

**Example:**

```jinja
<ul>
{%- for item in items %}
  <li>{{ item }}</li>
{%- endfor %}
</ul>
```

Without `-`, you'd get extra blank lines. With `-`, whitespace is trimmed.

---

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

---

## Comments

```jinja
{# This is a comment - not rendered in output #}

{#
  Multi-line comments
  are also supported
#}
```

---

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

---

## Template Includes

Template includes allow you to modularize your templates by splitting them into reusable partials. This promotes code reuse and keeps templates maintainable.

### Basic Syntax

```jinja
{% include "./path/to/partial.tmpltool" %}
```

**Important:** Always use relative paths starting with `./` for includes.

### Same Directory Include

Include a template from the same directory:

```
templates/
├── main.tmpltool
└── header.tmpltool
```

**main.tmpltool:**
```jinja
{% include "./header.tmpltool" %}
Main content here
```

**header.tmpltool:**
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
├── main.tmpltool
└── partials/
    ├── header.tmpltool
    └── footer.tmpltool
```

**main.tmpltool:**
```jinja
{% include "./partials/header.tmpltool" %}
Main content
{% include "./partials/footer.tmpltool" %}
```

### Nested Includes

Includes can be nested—an included template can include other templates:

```
templates/
├── main.tmpltool
├── layout.tmpltool
└── components/
    └── nav.tmpltool
```

**main.tmpltool:**
```jinja
{% include "./layout.tmpltool" %}
```

**layout.tmpltool:**
```jinja
<header>
{% include "./components/nav.tmpltool" %}
</header>
<main>Content</main>
```

**components/nav.tmpltool:**
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
{% include "./partials/header.tmpltool" %}
{% include "./partials/database.tmpltool" %}
{% include "./partials/logging.tmpltool" %}
{% include "./partials/footer.tmpltool" %}
```

### Conditional Includes

Use conditionals to include templates based on environment variables:

```jinja
{% set env = get_env(name="ENVIRONMENT", default="development") %}

{% if env == "production" %}
{% include "./configs/production.tmpltool" %}
{% elif env == "staging" %}
{% include "./configs/staging.tmpltool" %}
{% else %}
{% include "./configs/development.tmpltool" %}
{% endif %}
```

Or toggle optional sections:

```jinja
{% set enable_monitoring = get_env(name="ENABLE_MONITORING", default="false") %}

{% if enable_monitoring == "true" %}
{% include "./partials/monitoring.tmpltool" %}
{% endif %}
```

### Variables in Included Templates

Included templates have access to all variables defined in the parent template:

**main.tmpltool:**
```jinja
{% set app_name = get_env(name="APP_NAME", default="myapp") %}
{% set port = get_env(name="PORT", default="8080") %}
{% include "./server-config.tmpltool" %}
```

**server-config.tmpltool:**
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
| Same directory (`./file.tmpltool`) | ✅ Allowed | ✅ Allowed |
| Subdirectory (`./sub/file.tmpltool`) | ✅ Allowed | ✅ Allowed |
| Parent directory (`../file.tmpltool`) | ❌ Blocked | ✅ Allowed |
| Absolute path (`/etc/file`) | ❌ Blocked | ✅ Allowed |

**Parent directory access blocked (default):**
```jinja
{# This will fail without --trust #}
{% include "../shared/common.tmpltool" %}
```

**Error:**
```
Security: Parent directory (..) traversal is not allowed: ../shared/common.tmpltool.
Use --trust to bypass this restriction.
```

**Enable with trust mode:**
```bash
tmpltool --trust template.tmpltool
```

### Real-World Example: Docker Compose

Organize a Docker Compose template with reusable service partials:

```
templates/
├── docker-compose.tmpltool
└── services/
    ├── web.tmpltool
    ├── database.tmpltool
    └── redis.tmpltool
```

**docker-compose.tmpltool:**
```jinja
version: "3.8"

services:
{% include "./services/web.tmpltool" %}

{% set use_db = get_env(name="USE_DATABASE", default="true") %}
{% if use_db == "true" %}
{% include "./services/database.tmpltool" %}
{% endif %}

{% set use_cache = get_env(name="USE_CACHE", default="false") %}
{% if use_cache == "true" %}
{% include "./services/redis.tmpltool" %}
{% endif %}
```

**services/web.tmpltool:**
```jinja
  web:
    image: {{ get_env(name="WEB_IMAGE", default="nginx:latest") }}
    ports:
      - "{{ get_env(name="WEB_PORT", default="80") }}:80"
```

**services/database.tmpltool:**
```jinja
  database:
    image: postgres:15
    environment:
      POSTGRES_DB: {{ get_env(name="DB_NAME", default="app") }}
      POSTGRES_USER: {{ get_env(name="DB_USER", default="postgres") }}
```

**Render:**
```bash
WEB_PORT=8080 USE_CACHE=true tmpltool templates/docker-compose.tmpltool
```

### Real-World Example: Kubernetes Manifests

Split Kubernetes manifests into reusable components:

```
k8s/
├── deployment.tmpltool
└── components/
    ├── metadata.tmpltool
    ├── containers.tmpltool
    └── resources.tmpltool
```

**deployment.tmpltool:**
```jinja
apiVersion: apps/v1
kind: Deployment
{% include "./components/metadata.tmpltool" %}
spec:
  replicas: {{ get_env(name="REPLICAS", default="3") }}
  template:
    spec:
      containers:
{% include "./components/containers.tmpltool" %}
```

**components/metadata.tmpltool:**
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
3. **Name clearly** - Use descriptive names like `header.tmpltool`, `db-config.tmpltool`
4. **Keep partials focused** - Each partial should do one thing well
5. **Document dependencies** - Comment which variables a partial expects
6. **Avoid deep nesting** - Limit include depth for maintainability (2-3 levels max)
