# CLI Reference

## Syntax

```bash
tmpltool [TEMPLATE] [OPTIONS]
cat template.txt | tmpltool [OPTIONS]
```

## Arguments

- `[TEMPLATE]` - Path to template file (optional, reads from stdin if omitted)

## Options

- `-h, --help` - Print help information and exit
- `-V, --version` - Print version information and exit
- `-o, --output <FILE>` - Output file path (prints to stdout if not specified)
- `--trust` - Trust mode: Allow filesystem functions to access absolute paths and parent directories
  - **WARNING:** Only use with trusted templates. Disables security restrictions.
- `--validate <FORMAT>` - Validate output format (json, yaml, or toml)
  - Validates the rendered output conforms to the specified format
  - Exits with error code 1 if validation fails
  - No output on success, error message only on validation failure
- `--ide <FORMAT>` - Output function metadata for IDE integration (json, yaml, or toml)
  - Prints all available functions with descriptions, arguments, return types, and examples
  - Exits immediately after printing metadata (does not render templates)
  - Useful for building IDE plugins, autocomplete, and documentation generators
- `--env <FILE>` - Load environment variables from .env file(s)
  - Can be specified multiple times: `--env .env --env .env.local`
  - Files are loaded in order; later files override variables from earlier ones
  - Supports standard .env format: `KEY=value`, comments (`#`), and quoted values

## Input/Output Patterns

| Input  | Output | Command |
|--------|--------|---------|
| File   | stdout | `tmpltool template.txt` |
| File   | File   | `tmpltool template.txt -o output.txt` |
| stdin  | stdout | `cat template.txt \| tmpltool` |
| stdin  | File   | `cat template.txt \| tmpltool -o output.txt` |

## Examples

```bash
# Get help
tmpltool --help

# Check version
tmpltool --version

# File to stdout
tmpltool template.txt

# File to file
tmpltool template.txt -o output.txt

# Stdin to stdout (pipe)
echo "Hello {{ get_env(name=\"USER\") }}!" | tmpltool

# With environment variables
DB_HOST=postgres tmpltool config.tmpltool -o config.txt

# Chaining with other tools
tmpltool config.json.tmpltool | jq .
cat k8s-deployment.yaml.tmpltool | tmpltool | kubectl apply -f -

# Trust mode for system files
tmpltool --trust system_info.tmpltool  # Can read /etc/passwd, etc.

# Validate JSON output
tmpltool config.json.tmpltool --validate json
# Exits with error if output is invalid JSON

# Validate YAML output
tmpltool k8s-deploy.yaml.tmpltool --validate yaml -o deployment.yaml

# Validate TOML output
tmpltool Cargo.toml.tmpltool --validate toml

# Load variables from .env file
tmpltool --env .env config.tmpltool

# Load from multiple env files (later files override earlier)
tmpltool --env .env --env .env.local config.tmpltool

# Combine with other options
tmpltool --env .env --env .env.production --validate json -o config.json config.tmpltool
```

## Environment Files (.env)

The `--env` flag loads variables from `.env` files before template rendering. This is useful for:
- Storing configuration separately from templates
- Managing different environments (development, staging, production)
- Avoiding environment variable pollution in your shell

### .env File Format

```bash
# This is a comment
DATABASE_URL=postgres://localhost:5432/mydb
API_KEY=secret123

# Quoted values preserve spaces
APP_NAME="My Application"
DESCRIPTION='Single quotes work too'

# Empty values
EMPTY_VAR=

# Special characters in unquoted values
URL=https://example.com/path?key=value
```

### Multiple Environment Files

Load multiple files to layer configurations:

```bash
# .env (base configuration)
APP_NAME=myapp
LOG_LEVEL=info
DATABASE_URL=postgres://localhost/dev

# .env.production (production overrides)
LOG_LEVEL=warn
DATABASE_URL=postgres://prod-server/prod

# Load both - production values override base
tmpltool --env .env --env .env.production app.config.tmpltool
```

### Example Workflow

**Template (`config.tmpltool`):**
```jinja
server:
  host: {{ get_env(name="SERVER_HOST", default="localhost") }}
  port: {{ get_env(name="SERVER_PORT", default="8080") }}
database:
  url: {{ get_env(name="DATABASE_URL") }}
logging:
  level: {{ get_env(name="LOG_LEVEL", default="info") }}
```

**Environment file (`.env`):**
```bash
SERVER_HOST=0.0.0.0
SERVER_PORT=3000
DATABASE_URL=postgres://db:5432/myapp
LOG_LEVEL=debug
```

**Render:**
```bash
tmpltool --env .env config.tmpltool -o config.yaml
```

**Output (`config.yaml`):**
```yaml
server:
  host: 0.0.0.0
  port: 3000
database:
  url: postgres://db:5432/myapp
logging:
  level: debug
```
