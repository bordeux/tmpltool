## Filesystem Functions

Read files, check existence, list directories, glob patterns, and get file metadata.

**See also:** [Function Reference](../FUNCTIONS.md) | [Main Documentation](../README.md)

All filesystem functions enforce security restrictions to prevent unauthorized access. Only relative paths within the current working directory are allowed unless `--trust` mode is enabled.

**Security Restrictions:**
- ✗ No absolute paths (e.g., `/etc/passwd`)
- ✗ No parent directory traversal (e.g., `../../secret.txt`)
- ✓ Only relative paths within current directory

**Trust Mode:** Use `--trust` flag to bypass these restrictions for trusted templates.

```bash
tmpltool --trust template.tmpltool  # Can access any file
```

#### `read_file(path)`

Read the content of a file into the template.

**Arguments:**
- `path` (required) - Relative path to the file

**Returns:** String containing file content

**Examples:**
```
{# Read a configuration file #}
{% set config = read_file(path="config.txt") %}
{{ config }}

{# Read and include LICENSE #}
License:
{{ read_file(path="LICENSE") }}

{# Use with filters #}
First 100 chars: {{ read_file(path="README.md") | truncate(length=100) }}
```

#### `file_exists(path)`

Check if a file exists at the specified path.

**Arguments:**
- `path` (required) - Relative path to check

**Returns:** Boolean (`true` if exists, `false` otherwise)

**Examples:**
```
{# Conditional file inclusion #}
{% if file_exists(path="custom-config.txt") %}
Custom config found!
{{ read_file(path="custom-config.txt") }}
{% else %}
Using default configuration
{% endif %}

{# Check multiple files #}
{% set has_readme = file_exists(path="README.md") %}
{% set has_license = file_exists(path="LICENSE") %}
Documentation: {% if has_readme %}✓{% else %}✗{% endif %}
License: {% if has_license %}✓{% else %}✗{% endif %}
```

#### `list_dir(path)`

List all files and directories in a directory.

**Arguments:**
- `path` (required) - Relative path to the directory

**Returns:** Array of filenames (sorted alphabetically)

**Examples:**
```
{# List files in a directory #}
Files in data/:
{% for file in list_dir(path="data") %}
  - {{ file }}
{% endfor %}

{# Count files #}
{% set files = list_dir(path="templates") %}
Total templates: {{ files | length }}

{# Filter by extension #}
{% set all_files = list_dir(path="src") %}
Rust files:
{% for file in all_files %}
{% if file is ending_with(".rs") %}
  - {{ file }}
{% endif %}
{% endfor %}
```

#### `glob(pattern)`

List all files matching a glob pattern.

**Arguments:**
- `pattern` (required) - Glob pattern (`*` matches any characters, `?` matches one character, `**` matches any number of directories)

**Returns:** Array of file paths (sorted alphabetically)

**Examples:**
```
{# Find all text files #}
Text files:
{% for file in glob(pattern="*.txt") %}
  - {{ file }}
{% endfor %}

{# Find files in subdirectories #}
All Rust files:
{% for file in glob(pattern="src/**/*.rs") %}
  - {{ file }}
{% endfor %}

{# Match specific patterns #}
Config files:
{% for file in glob(pattern="config*.{json,yaml,toml}") %}
  - {{ file }}
{% endfor %}

{# Use in conditionals #}
{% set test_files = glob(pattern="tests/**/*.rs") %}
{% if test_files | length > 0 %}
Found {{ test_files | length }} test files
{% endif %}
```

#### `file_size(path)`

Get the size of a file in bytes.

**Arguments:**
- `path` (required) - Relative path to the file

**Returns:** File size as a number (in bytes)

**Examples:**
```
{# Get file size #}
README size: {{ file_size(path="README.md") }} bytes

{# Format with built-in filter #}
README size: {{ file_size(path="README.md") | filesizeformat }}

{# Compare file sizes #}
{% set size_a = file_size(path="file_a.txt") %}
{% set size_b = file_size(path="file_b.txt") %}
{% if size_a > size_b %}
file_a.txt is larger
{% else %}
file_b.txt is larger
{% endif %}

{# Calculate total size #}
{% set files = glob(pattern="data/*.json") %}
{% set total_size = 0 %}
{% for file in files %}
{% set total_size = total_size + file_size(path=file) %}
{% endfor %}
Total data size: {{ total_size | filesizeformat }}
```

#### `file_modified(path)`

Get the last modification time of a file as a Unix timestamp.

**Arguments:**
- `path` (required) - Relative path to the file

**Returns:** Unix timestamp (seconds since January 1, 1970)

**Examples:**
```
{# Get modification timestamp #}
Last modified: {{ file_modified(path="config.json") }}

{# Format with format_date function #}
{% set timestamp = file_modified(path="README.md") %}
Last updated: {{ format_date(timestamp=timestamp, format="%Y-%m-%d %H:%M:%S") }}

{# Check if file is recent #}
{% set mod_time = file_modified(path="cache.dat") %}
{% set now_time = now() %}
{% set age_seconds = now_time - mod_time %}
{% if age_seconds < 3600 %}
Cache is fresh (less than 1 hour old)
{% else %}
Cache is stale ({{ age_seconds / 3600 }} hours old)
{% endif %}
```

**Practical Example - Build Report:**
```
# Build Report
Generated: {{ now(format="%Y-%m-%d %H:%M:%S") }}

## Source Files
{% set rs_files = glob(pattern="src/**/*.rs") %}
Total Rust files: {{ rs_files | length }}

{% for file in rs_files %}
- {{ file }}
  Size: {{ file_size(path=file) | filesizeformat }}
  Modified: {{ format_date(timestamp=file_modified(path=file), format="%Y-%m-%d") }}
{% endfor %}

## Configuration
{% if file_exists(path="Cargo.toml") %}
✓ Cargo.toml found ({{ file_size(path="Cargo.toml") }} bytes)
{% else %}
✗ Cargo.toml missing
{% endif %}

## Tests
{% set test_files = glob(pattern="tests/**/*.rs") %}
Test files: {{ test_files | length }}
```

