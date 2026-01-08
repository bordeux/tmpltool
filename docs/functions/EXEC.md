## Command Execution Functions

Execute shell commands and capture output (requires --trust flag).

**See also:** [Function Reference](../FUNCTIONS.md) | [Main Documentation](../README.md)

Execute external commands from templates. These functions provide the ability to run shell commands and capture their output.

**SECURITY WARNING:** Command execution is a powerful feature that can pose security risks. These functions are **only available in trust mode** (`--trust` flag).

#### `exec(command, timeout)`

Execute a command and return stdout as a string. Throws an error if the command fails (non-zero exit code).

**Arguments:**
- `command` (required) - Command to execute (executed via system shell)
- `timeout` (optional) - Timeout in seconds (default: 30, max: 300)

**Returns:** Standard output as string

**Security:** Only available with `--trust` flag

**Examples:**
```jinja
{# Simple usage - get output directly #}
Hostname: {{ exec(command="hostname") }}

{# Use with filters #}
System: {{ exec(command="uname -s") | trim }}

{# Use in variable #}
{% set git_hash = exec(command="git rev-parse --short HEAD 2>/dev/null || echo 'unknown'") %}
Commit: {{ git_hash | trim }}

{# This will throw an error if command fails #}
{{ exec(command="ls /nonexistent") }}  {# Error! #}
```

#### `exec_raw(command, timeout)`

Execute a command and return a detailed result object. Never throws based on exit code - you control all error handling.

**Arguments:**
- `command` (required) - Command to execute (executed via system shell)
- `timeout` (optional) - Timeout in seconds (default: 30, max: 300)

**Returns:** Object with fields:
- `exit_code` - Exit code (integer, 0 = success)
- `stdout` - Standard output (string)
- `stderr` - Standard error (string)
- `success` - Boolean (true if exit_code == 0)

**Security:** Only available with `--trust` flag

**Examples:**
```jinja
{# Full control over result #}
{% set result = exec_raw(command="ls -la /tmp") %}
{% if result.success %}
Files:
{{ result.stdout }}
{% else %}
Error (exit {{ result.exit_code }}): {{ result.stderr }}
{% endif %}

{# Handle expected non-zero exit (e.g., grep) #}
{% set result = exec_raw(command="grep foo /etc/hosts") %}
{% if result.exit_code == 0 %}
Found: {{ result.stdout }}
{% elif result.exit_code == 1 %}
Not found
{% else %}
Error: {{ result.stderr }}
{% endif %}

{# Check if a tool is available #}
{% set docker = exec_raw(command="which docker") %}
{% if docker.success %}
Docker available at: {{ docker.stdout | trim }}
{% else %}
Docker not installed
{% endif %}
```

**Practical Example - Build Information:**
```yaml
build:
  commit: {{ exec(command="git rev-parse --short HEAD 2>/dev/null || echo 'dev'") | trim }}
  branch: {{ exec(command="git branch --show-current 2>/dev/null || echo 'unknown'") | trim }}
  date: {{ exec(command="date -u +%Y-%m-%dT%H:%M:%SZ") | trim }}
  user: {{ exec(command="whoami") | trim }}
```

**Practical Example - Conditional Configuration:**
```yaml
{% set node_check = exec_raw(command="which node") %}
{% set docker_check = exec_raw(command="which docker") %}

services:
  node_enabled: {{ node_check.success | lower }}
  {% if node_check.success %}
  node_version: {{ exec(command="node --version") | trim }}
  {% endif %}

  docker_enabled: {{ docker_check.success | lower }}
  {% if docker_check.success %}
  docker_path: {{ docker_check.stdout | trim }}
  {% endif %}
```

**Practical Example - Dynamic Worker Configuration:**
```yaml
{% set cpu_count = exec(command="nproc 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo '2'") | trim | int %}

workers:
  count: {{ cpu_count * 2 }}
  per_worker_connections: 1000
  total_capacity: {{ cpu_count * 2000 }}
```

**Security Considerations:**

⚠️ **Command Injection Risk:** Never use untrusted input in commands
```jinja
{# ❌ DANGEROUS - DO NOT DO THIS #}
{% set user_input = get_env(name="USER_INPUT") %}
{{ exec(command="echo " ~ user_input) }}  {# COMMAND INJECTION! #}

{# ✓ SAFE - Use only hardcoded, trusted commands #}
{{ exec(command="hostname") }}
{{ exec(command="date") }}
```

**Notes:**
- Commands are executed via the system shell (`sh -c` on Unix, `cmd /C` on Windows)
- All shell features work: pipes (`|`), redirections (`>`, `2>&1`), command substitution (`$()`)
- Use `exec()` for simple cases where you just want output
- Use `exec_raw()` when you need to check exit codes or handle errors
- Always use `2>/dev/null || echo 'fallback'` patterns for robust error handling
- Keep commands fast - they block template rendering

