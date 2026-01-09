# tmpltool

[![CI](https://github.com/bordeux/tmpltool/actions/workflows/ci.yml/badge.svg)](https://github.com/bordeux/tmpltool/actions/workflows/ci.yml)
[![Release](https://github.com/bordeux/tmpltool/actions/workflows/release.yml/badge.svg)](https://github.com/bordeux/tmpltool/actions/workflows/release.yml)
[![codecov](https://codecov.io/gh/bordeux/tmpltool/branch/master/graph/badge.svg)](https://codecov.io/gh/bordeux/tmpltool)
[![GitHub release](https://img.shields.io/github/v/release/bordeux/tmpltool)](https://github.com/bordeux/tmpltool/releases)
[![Docker](https://img.shields.io/badge/docker-ghcr.io-blue)](https://github.com/bordeux/tmpltool/pkgs/container/tmpltool)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A fast and simple command-line template rendering tool using [MiniJinja](https://github.com/mitsuhiko/minijinja) templates with environment variables.

## Table of Contents

- [Quick Start](#quick-start)
- [Features](#features)
- [Installation](#installation)
- [Basic Usage](#basic-usage)
- [Documentation](#documentation)
- [Contributing](#contributing)
- [License](#license)

## Quick Start

Get started in 30 seconds:

```bash
# Install tmpltool (works on macOS, Linux, and more)
curl -fsSL https://raw.githubusercontent.com/bordeux/repo/master/install.sh | sh -s -- tmpltool

# Create and render template
echo 'Hello {{ get_env(name="USER", default="World") }}!' > greeting.tmpltool
tmpltool greeting.tmpltool
# Output: Hello World!
```

## Features

- **Environment Variables**: Access env vars with `get_env()` and filter with `filter_env()`
- **Hash & Crypto**: MD5, SHA1, SHA256, SHA512, UUID generation, random strings
- **Encoding & Security**: Base64, hex, bcrypt, HMAC, HTML/XML/shell escaping, secure random strings
- **Filesystem**: Read files, check existence, list directories, glob patterns, file info, path manipulation
- **Data Parsing**: Parse and read JSON, YAML, TOML files
- **Data Serialization**: Convert objects to JSON, YAML, TOML strings with pretty-printing options
- **Object Manipulation**: Deep merge, get/set nested values by path, extract keys/values, check key existence
- **Validation**: Validate emails, URLs, IPs, UUIDs, regex matching
- **System & Network**: Get hostname, username, directories, IP addresses, DNS resolution, port availability
- **Web & URL**: Parse and build URLs, generate query strings, HTTP Basic Auth headers
- **Kubernetes**: Resource requests, label sanitization, ConfigMap/Secret references for manifests
- **Math & Logic**: Min/max, rounding, percentages, default values, ternary operations, range checks
- **Array & Statistics**: Sorting, grouping, chunking, sum/avg/median, unique values, flattening
- **Debugging & Development**: Debug output, type checking, assertions, warnings, error handling
- **String Filters**: 12+ filters for case conversion, indentation, padding, quoting, and more
- **Security**: Built-in protections with optional `--trust` mode
- **Flexible I/O**: File or stdin input, file or stdout output
- **Full Jinja2 Syntax**: Conditionals, loops, filters, and more
- **Single Binary**: No runtime dependencies, static binaries available
- **Docker-Friendly**: Extract binary from Docker image (multi-arch support)

## Installation

The easiest way to install tmpltool:

```bash
curl -fsSL https://raw.githubusercontent.com/bordeux/repo/master/install.sh | sh -s -- tmpltool
```

For detailed installation instructions including macOS, Linux, Docker, and building from source, see the [Installation Guide](docs/INSTALLATION.md).

## Documentation

📚 **Complete documentation is available in the [docs/](docs/) directory:**

- **[Installation Guide](docs/INSTALLATION.md)** - Detailed installation instructions for all platforms
- **[CLI Reference](docs/CLI.md)** - Complete command-line interface documentation
- **[Template Syntax](docs/TEMPLATE_SYNTAX.md)** - MiniJinja/Jinja2 template syntax guide
- **[Function Reference](docs/FUNCTIONS.md)** - Complete reference for all available functions
- **[Examples](docs/EXAMPLES.md)** - Advanced examples and use cases
- **[Error Handling](docs/ERROR_HANDLING.md)** - Understanding and handling errors
- **[IDE Integration](docs/IDE_INTEGRATION.md)** - Using `--ide` flag for IDE plugins
- **[Development Guide](docs/DEVELOPMENT.md)** - Building, testing, and contributing
- **[CI/CD](docs/CICD.md)** - Continuous integration and release process

See [docs/README.md](docs/README.md) for the full documentation index.

## Basic Usage

### Simple Variable Substitution

**Template** (`greeting.tmpltool`):
```
Hello {{ get_env(name="USER") }}!
Your home directory is: {{ get_env(name="HOME") }}
```

**Render:**
```bash
tmpltool greeting.tmpltool
```

### Using Default Values

**Template** (`config.tmpltool`):
```
Database: {{ get_env(name="DB_HOST", default="localhost") }}:{{ get_env(name="DB_PORT", default="5432") }}
Environment: {{ get_env(name="APP_ENV", default="development") }}
Debug: {{ get_env(name="DEBUG", default="false") }}
```

**Render with defaults:**
```bash
tmpltool config.tmpltool
# Output:
# Database: localhost:5432
# Environment: development
# Debug: false
```

**Render with custom values:**
```bash
DB_HOST=postgres DB_PORT=5432 APP_ENV=production tmpltool config.tmpltool
# Output:
# Database: postgres:5432
# Environment: production
# Debug: false
```

### Conditionals and Loops

**Template** (`status.tmpltool`):
```
{% set debug = get_env(name="DEBUG", default="false") %}
{% if debug == "true" %}
DEBUG MODE ENABLED
Log level: verbose
{% else %}
Production mode
Log level: error
{% endif %}

{% set items_str = get_env(name="ITEMS", default="apple,banana,orange") %}
{% set items = items_str | split(pat=",") %}
Items:
{% for item in items %}
  - {{ item }}
{% endfor %}
```

**Render:**
```bash
DEBUG=true ITEMS="apple,banana,orange,grape" tmpltool status.tmpltool
```

### Filtering Environment Variables

**Template** (`server-vars.tmpltool`):
```
Server Configuration:
{% for var in filter_env(pattern="SERVER_*") %}
  {{ var.key }}={{ var.value }}
{% endfor %}
```

**Render:**
```bash
SERVER_HOST=localhost SERVER_PORT=8080 SERVER_NAME=myapp tmpltool server-vars.tmpltool
# Output:
# Server Configuration:
#   SERVER_HOST=localhost
#   SERVER_NAME=myapp
#   SERVER_PORT=8080
```

### Working with Files

**Template** (`build-report.tmpltool`):
```
# Build Report

{% if file_exists(path="README.md") %}
✓ README.md found ({{ file_size(path="README.md") | filesizeformat }})
{% else %}
✗ README.md missing
{% endif %}

Source files:
{% for file in glob(pattern="src/**/*.rs") %}
  - {{ file }} ({{ file_size(path=file) | filesizeformat }})
{% endfor %}
```

**Render:**
```bash
tmpltool build-report.tmpltool
```

## Template Syntax

tmpltool uses the [MiniJinja](https://github.com/mitsuhiko/minijinja) template engine, which is compatible with Python's Jinja2.

**📖 For complete template syntax documentation, see [Template Syntax Guide](docs/TEMPLATE_SYNTAX.md).**

## Function Reference

**📖 For complete function reference, see [Function Reference](docs/FUNCTIONS.md).**

The function reference includes documentation for:
- Environment Variables (`get_env`, `filter_env`)
- Hash & Crypto Functions (MD5, SHA1, SHA256, SHA512, UUID, random strings)
- Encoding & Security Functions (Base64, hex, bcrypt, HMAC, escaping)
- Date/Time Functions (formatting, parsing, timezone conversion)
- Command Execution Functions (`exec`, `exec_raw`) - requires `--trust` flag
- Filesystem Functions (read files, list directories, glob patterns)
- Path Manipulation Functions (basename, dirname, join_path, normalize_path)
- Data Parsing & Serialization (JSON, YAML, TOML)
- Object Manipulation Functions (merge, get/set nested values, flatten)
- Validation Functions (email, URL, IP, UUID validation)
- System & Network Functions (hostname, IP addresses, DNS, port checking)
- Math Functions (min, max, round, ceil, floor, percentage)
- Array & Statistical Functions (sum, avg, median, sort, group, unique)
- Predicate Functions (array_any, array_all, contains, starts_with, ends_with)
- Kubernetes Functions (resource requests, label sanitization, probes)
- Web & URL Functions (URL parsing, query strings, Basic Auth)
- Logic Functions (default, coalesce, ternary, in_range)
- String Manipulation Functions (regex, substring, truncate, and more)
- Debugging & Development Functions (type checking, assertions, warnings)

## IDE Integration

**📖 See [IDE Integration Guide](docs/IDE_INTEGRATION.md) for details on using the `--ide` flag.**

## Advanced Examples

**📖 See [Examples Guide](docs/EXAMPLES.md) for advanced use cases and examples.**

## Error Handling

**📖 See [Error Handling Guide](docs/ERROR_HANDLING.md) for error handling details.**

## Development

**📖 See [Development Guide](docs/DEVELOPMENT.md) for building, testing, and contributing.**

## CI/CD

**📖 See [CI/CD Guide](docs/CICD.md) for continuous integration and release process.**

## Contributing

Contributions are welcome! For detailed guidelines, see [CONTRIBUTING.md](CONTRIBUTING.md).

### Quick Start

1. Fork the repository
2. Clone and install dependencies:
   ```bash
   git clone https://github.com/YOUR_USERNAME/tmpltool.git
   cd tmpltool
   npm install  # Installs commit hooks
   ```
3. Create a feature branch
4. Make your changes
5. Run QA checks: `cargo make qa`
6. Commit using conventional commits
7. Push and open a Pull Request

**Note:** Commit messages are automatically validated. Invalid commits will be rejected.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

For more examples, see the [examples/](examples/) directory and [examples/README.md](examples/README.md).

For complete MiniJinja/Jinja2 syntax documentation, visit:
- MiniJinja Docs: https://docs.rs/minijinja/
- Jinja2 Template Designer: https://jinja.palletsprojects.com/templates/
