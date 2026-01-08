## Path Manipulation Functions

Path manipulation functions: basename, dirname, join_path, normalize_path, and path checks.

**See also:** [Function Reference](../FUNCTIONS.md) | [Main Documentation](../README.md)

Functions for manipulating file paths and checking filesystem metadata. These functions do not read file contents and work without security restrictions.

#### `basename(path)` / `| basename`

Extract the filename from a path.

**Arguments:**
- `path` (required) - File path

**Returns:** Filename (last component of the path)

**Function syntax:**
```jinja
{{ basename(path="/path/to/file.txt") }}
{# Output: file.txt #}
```

**Filter syntax:**
```jinja
{{ "/path/to/file.txt" | basename }}
{# Output: file.txt #}

{# Use with glob results #}
{% for file in glob(pattern="src/**/*.rs") %}
  - {{ file | basename }}
{% endfor %}
```

#### `dirname(path)` / `| dirname`

Extract the directory portion from a path.

**Arguments:**
- `path` (required) - File path

**Returns:** Directory path (all components except the last)

**Function syntax:**
```jinja
{{ dirname(path="/path/to/file.txt") }}
{# Output: /path/to #}
```

**Filter syntax:**
```jinja
{{ "/path/to/file.txt" | dirname }}
{# Output: /path/to #}

{# Get parent directory #}
{% set file_path = "config/app/settings.json" %}
Config directory: {{ file_path | dirname }}
{# Output: config/app #}
```

#### `file_extension(path)` / `| file_extension`

Extract the file extension from a path.

**Arguments:**
- `path` (required) - File path

**Returns:** File extension without the dot (empty string if no extension)

**Function syntax:**
```jinja
{{ file_extension(path="document.pdf") }}
{# Output: pdf #}

{{ file_extension(path="/path/to/file.tar.gz") }}
{# Output: gz #}
```

**Filter syntax:**
```jinja
{{ "document.pdf" | file_extension }}
{# Output: pdf #}

{# Chain with basename #}
{{ "/path/to/file.tar.gz" | basename | file_extension }}
{# Output: gz #}

{# Group files by extension #}
{% for file in glob(pattern="docs/*") %}
  {% if file | file_extension == "md" %}
    - Markdown: {{ file }}
  {% endif %}
{% endfor %}
```

#### `join_path(parts)` / `| join_path`

Join multiple path components into a single path.

**Arguments:**
- `parts` (required) - Array of path components

**Returns:** Joined path string

**Function syntax:**
```jinja
{{ join_path(parts=["path", "to", "file.txt"]) }}
{# Output: path/to/file.txt #}

{{ join_path(parts=["/home", "user", "documents"]) }}
{# Output: /home/user/documents #}
```

**Filter syntax:**
```jinja
{{ ["path", "to", "file.txt"] | join_path }}
{# Output: path/to/file.txt #}

{# Build dynamic paths #}
{% set path_parts = ["config", "production", "settings.json"] %}
{{ path_parts | join_path }}
{# Output: config/production/settings.json #}
```

#### `normalize_path(path)` / `| normalize_path`

Normalize a path by resolving `.` (current directory) and `..` (parent directory) components.

**Arguments:**
- `path` (required) - Path to normalize

**Returns:** Normalized path string

**Function syntax:**
```jinja
{{ normalize_path(path="./foo/bar") }}
{# Output: foo/bar #}

{{ normalize_path(path="a/b/c/../../d") }}
{# Output: a/d #}
```

**Filter syntax:**
```jinja
{{ "./foo/bar" | normalize_path }}
{# Output: foo/bar #}

{# Clean up path components #}
{% set messy_path = "./config/../data/./files.txt" %}
{{ messy_path | normalize_path }}
{# Output: data/files.txt #}
```

#### `is_file(path)` / `{% if path is file %}`

Check if a path exists and is a file. Supports both function syntax and "is" test syntax.

**Function Syntax Arguments:**
- `path` (required) - Path to check

**Is-Test Syntax:**
- The value must be a string representing a file path

**Returns:** Boolean (true if path exists and is a file)

**Examples:**
```jinja
{# Function syntax #}
{% if is_file(path="config.txt") %}
  Config file found!
{% endif %}

{# Is-test syntax (preferred for readability) #}
{% if "config.txt" is file %}
  Config file found!
{% endif %}

{# With variables #}
{% set config_path = "config.json" %}
{% if config_path is file %}
  {{ read_file(path=config_path) }}
{% endif %}
```

#### `is_dir(path)` / `{% if path is dir %}`

Check if a path exists and is a directory. Supports both function syntax and "is" test syntax.

**Function Syntax Arguments:**
- `path` (required) - Path to check

**Is-Test Syntax:**
- The value must be a string representing a directory path

**Returns:** Boolean (true if path exists and is a directory)

**Examples:**
```jinja
{# Function syntax #}
{% if is_dir(path="src") %}
  Source directory exists
{% endif %}

{# Is-test syntax (preferred for readability) #}
{% if "src" is dir %}
  Source directory exists
{% endif %}

{# With variables #}
{% set test_dir = "tests" %}
{% if test_dir is dir %}
  {% set test_files = glob(pattern="tests/**/*.rs") %}
  Found {{ test_files | length }} test files
{% endif %}
```

#### `is_symlink(path)` / `{% if path is symlink %}`

Check if a path is a symbolic link. Supports both function syntax and "is" test syntax.

**Function Syntax Arguments:**
- `path` (required) - Path to check

**Is-Test Syntax:**
- The value must be a string representing a path

**Returns:** Boolean (true if path is a symlink)

**Examples:**
```jinja
{# Function syntax #}
{% if is_symlink(path="current") %}
  'current' is a symbolic link
{% endif %}

{# Is-test syntax (preferred for readability) #}
{% if "current" is symlink %}
  'current' is a symbolic link
{% endif %}

{# With variables #}
{% set link_path = "latest" %}
{% if link_path is symlink %}
  Following symlink...
{% endif %}
```

#### `read_lines(path, max_lines)`

Read lines from a file with flexible line selection.

**Arguments:**
- `path` (required) - Path to file
- `max_lines` (optional) - Number of lines to read (default: 10, max abs value: 10000)
  - **Positive number**: Read first N lines
  - **Negative number**: Read last N lines
  - **Zero**: Read entire file

**Returns:** Array of strings (lines without newline characters)

**Security:** Requires `--trust` flag for absolute paths or parent directory traversal

**Examples:**
```jinja
{# Read first 5 lines #}
{% set first_lines = read_lines(path="log.txt", max_lines=5) %}
Recent log entries:
{% for line in first_lines %}
  {{ loop.index }}: {{ line }}
{% endfor %}

{# Read last 5 lines #}
{% set last_lines = read_lines(path="log.txt", max_lines=-5) %}
Latest log entries:
{% for line in last_lines %}
  {{ line }}
{% endfor %}

{# Read entire file #}
{% set all_lines = read_lines(path="config.txt", max_lines=0) %}
Total lines: {{ all_lines | length }}

{# Preview file content #}
{% if is_file(path="README.md") %}
  README preview (first 3 lines):
  {% for line in read_lines(path="README.md", max_lines=3) %}
    {{ line }}
  {% endfor %}
{% endif %}

{# Process log file tail #}
{% set log_tail = read_lines(path="app.log", max_lines=-10) %}
{% for line in log_tail %}
  {% if "ERROR" in line %}
    ⚠️  {{ line }}
  {% endif %}
{% endfor %}
```

**Practical Example - Project Structure:**
```jinja
# Project Analysis

## Directory Structure
{% for item in ["src", "tests", "examples", "docs"] %}
  {% if is_dir(path=item) %}
    ✓ {{ item }}/ ({{ glob(pattern=item ~ "/**/*") | length }} files)
  {% else %}
    ✗ {{ item }}/ (missing)
  {% endif %}
{% endfor %}

## Configuration Files
{% for config_file in ["Cargo.toml", "package.json", ".gitignore"] %}
  {% if is_file(path=config_file) %}
    ✓ {{ config_file }}
    {% set lines = read_lines(path=config_file, max_lines=3) %}
    Preview: {{ lines[0] | truncate(length=50) }}
  {% else %}
    ✗ {{ config_file }} (not found)
  {% endif %}
{% endfor %}

## Source Files by Type
{% set all_files = glob(pattern="src/**/*") %}
{% for file in all_files %}
  {% set ext = file_extension(path=file) %}
  {% if ext == "rs" %}
    - Rust: {{ basename(path=file) }}
  {% elif ext == "md" %}
    - Markdown: {{ basename(path=file) }}
  {% endif %}
{% endfor %}
```

