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
