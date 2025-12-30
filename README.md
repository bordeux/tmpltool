# tmpltool

[![CI](https://github.com/bordeux/tmpltool/actions/workflows/ci.yml/badge.svg)](https://github.com/bordeux/tmpltool/actions/workflows/ci.yml)
[![Release](https://github.com/bordeux/tmpltool/actions/workflows/release.yml/badge.svg)](https://github.com/bordeux/tmpltool/actions/workflows/release.yml)
[![codecov](https://codecov.io/gh/bordeux/tmpltool/branch/master/graph/badge.svg)](https://codecov.io/gh/bordeux/tmpltool)
[![GitHub release](https://img.shields.io/github/v/release/bordeux/tmpltool)](https://github.com/bordeux/tmpltool/releases)
[![Docker](https://img.shields.io/badge/docker-ghcr.io-blue)](https://github.com/bordeux/tmpltool/pkgs/container/tmpltool)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A fast and simple command-line template rendering tool using [Tera](https://keats.github.io/tera/) templates with environment variables.

## Features

- Render Tera templates with all environment variables available as context
- Output to file or stdout (for piping)
- Simple CLI interface
- Single binary executable
- Full Tera template syntax support (variables, conditionals, loops, filters, etc.)

## Installation

### From GitHub Releases

Download pre-built binaries for your platform from the [releases page](https://github.com/bordeux/tmpltool/releases):

- **Linux**: `tmpltool-linux-x86_64`, `tmpltool-linux-x86_64-musl` (static), `tmpltool-linux-aarch64` (ARM64)
- **macOS**: `tmpltool-macos-x86_64` (Intel), `tmpltool-macos-aarch64` (Apple Silicon)
- **Windows**: `tmpltool-windows-x86_64.exe`

Extract and place in your PATH:

```bash
# Linux/macOS example
tar -xzf tmpltool-linux-x86_64.tar.gz
sudo mv tmpltool /usr/local/bin/
chmod +x /usr/local/bin/tmpltool
```

### Using Docker

Pull from GitHub Container Registry:

```bash
docker pull ghcr.io/bordeux/tmpltool:latest
```

Run with Docker:

```bash
# Using a template file
docker run --rm -v $(pwd):/workspace -w /workspace ghcr.io/bordeux/tmpltool:latest template.tmpl

# With environment variables
docker run --rm -e NAME=Alice -v $(pwd):/workspace -w /workspace \
  ghcr.io/bordeux/tmpltool:latest greeting.tmpl

# Output to file
docker run --rm -v $(pwd):/workspace -w /workspace \
  ghcr.io/bordeux/tmpltool:latest template.tmpl -o output.txt
```

Create a shell alias for convenience:

```bash
alias tmpltool='docker run --rm -v $(pwd):/workspace -w /workspace ghcr.io/bordeux/tmpltool:latest'
```

### From Source

```bash
cargo install --path .
```

Or build manually:

```bash
cargo build --release
# Binary will be at: ./target/release/tmpltool
```

## Usage

```bash
# Read from file
tmpltool [TEMPLATE] [OPTIONS]

# Read from stdin
cat template.txt | tmpltool [OPTIONS]
```

### Arguments

- `[TEMPLATE]` - Path to the template file (optional)
  - If omitted, reads template from stdin

### Options

- `-o, --output <FILE>` - Output file path (optional)
  - If not specified, output is printed to stdout

### Input/Output Combinations

tmpltool supports all standard Unix I/O patterns:

| Input  | Output | Command |
|--------|--------|---------|
| File   | stdout | `tmpltool template.txt` |
| File   | File   | `tmpltool template.txt -o output.txt` |
| stdin  | stdout | `cat template.txt \| tmpltool` |
| stdin  | File   | `cat template.txt \| tmpltool -o output.txt` |

### Examples

#### File to stdout

```bash
tmpltool template.txt
```

#### File to file

```bash
tmpltool template.txt -o output.txt
```

#### stdin to stdout (pipe)

```bash
cat template.txt | tmpltool
echo "Hello {{ get_env(name=\"NAME\", default=\"World\") }}!" | tmpltool
```

#### stdin to file

```bash
cat template.txt | tmpltool -o output.txt
```

#### Chaining with other tools

```bash
# Generate and validate
tmpltool config.json.tmpl | jq .

# Generate from stdin and apply
cat k8s-deployment.yaml.tmpl | tmpltool | kubectl apply -f -

# Combine multiple templates
cat header.tmpl body.tmpl footer.tmpl | tmpltool > complete.html
```

#### Using Environment Variables

Create a template file `greeting.tmpl`:
```
Hello {{ USER }}!
Your home directory is: {{ HOME }}
Your shell: {{ SHELL }}
```

Render it:
```bash
tmpltool greeting.tmpl
```

Output:
```
Hello username!
Your home directory is: /home/username
Your shell: /bin/bash
```

#### Setting Custom Environment Variables

Create a template `config.tmpl`:
```
Database: {{ DB_HOST }}:{{ DB_PORT }}
Environment: {{ APP_ENV }}
Debug: {{ DEBUG }}
```

Render with custom variables:
```bash
DB_HOST=localhost DB_PORT=5432 APP_ENV=production DEBUG=false tmpltool config.tmpl
```

Output:
```
Database: localhost:5432
Environment: production
Debug: false
```

#### Using Conditionals

Template `status.tmpl`:
```
{% if DEBUG %}
DEBUG MODE ENABLED
Log level: verbose
{% else %}
Production mode
Log level: error
{% endif %}
```

Render:
```bash
DEBUG=true tmpltool status.tmpl
```

#### Using Loops

If you set an environment variable with a list (this example uses Tera's split filter):

Template `list.tmpl`:
```
{% set items = ITEMS | split(pat=",") %}
Items:
{% for item in items %}
  - {{ item }}
{% endfor %}
```

Render:
```bash
ITEMS="apple,banana,orange" tmpltool list.tmpl
```

Output:
```
Items:
  - apple
  - banana
  - orange
```

#### Using Filters

Template `formatted.tmpl`:
```
Uppercase: {{ NAME | upper }}
Lowercase: {{ NAME | lower }}
Title Case: {{ NAME | title }}
```

Render:
```bash
NAME="john doe" tmpltool formatted.tmpl
```

Output:
```
Uppercase: JOHN DOE
Lowercase: john doe
Title Case: John Doe
```

#### Complex Example - Docker Compose Generator

Template `docker-compose.tmpl`:
```yaml
version: '3.8'

services:
  {{ SERVICE_NAME }}:
    image: {{ DOCKER_IMAGE }}
    ports:
      - "{{ HOST_PORT }}:{{ CONTAINER_PORT }}"
    environment:
      - NODE_ENV={{ NODE_ENV }}
      {% if DATABASE_URL %}
      - DATABASE_URL={{ DATABASE_URL }}
      {% endif %}
    {% if ENABLE_VOLUMES %}
    volumes:
      - ./app:/app
    {% endif %}
```

Render:
```bash
SERVICE_NAME=web \
DOCKER_IMAGE=node:18 \
HOST_PORT=3000 \
CONTAINER_PORT=3000 \
NODE_ENV=development \
DATABASE_URL=postgres://localhost/mydb \
ENABLE_VOLUMES=true \
tmpltool docker-compose.tmpl -o docker-compose.yml
```

#### Pipeline Usage

Generate and validate JSON config:
```bash
tmpltool config.json.tmpl | jq .
```

Generate and apply Kubernetes config:
```bash
tmpltool k8s-deployment.yaml.tmpl | kubectl apply -f -
```

Generate nginx config and test it:
```bash
tmpltool nginx.conf.tmpl | nginx -t -c /dev/stdin
```

## Examples

The `examples/` directory contains ready-to-use template examples demonstrating various features:

- **`basic.tmpl`** - Basic variable substitution and conditionals
- **`greeting.tmpl`** - Simple greeting with `get_env()` function
- **`config.tmpl`** - Application configuration file generation
- **`docker-compose.tmpl`** - Docker Compose with sensible defaults
- **`config-with-defaults.tmpl`** - Advanced config using `get_env()` function (recommended)

### Try an Example

```bash
# Basic example with environment variables
CUSTOM_VAR="Hello World" tmpltool examples/basic.tmpl

# Greeting with defaults
tmpltool examples/greeting.tmpl

# Generate a docker-compose.yml with all defaults (works out of the box!)
tmpltool examples/docker-compose.tmpl -o docker-compose.yml

# Generate docker-compose.yml with custom values
SERVICE_NAME=web \
DATABASE_URL=postgres://db:5432/myapp \
ENABLE_VOLUMES=true \
tmpltool examples/docker-compose.tmpl -o docker-compose.yml

# Config with get_env() function and defaults
tmpltool examples/config-with-defaults.tmpl
```

See the [examples/README.md](examples/README.md) for detailed documentation of each example.

## Template Syntax

tmpltool uses the [Tera](https://keats.github.io/tera/) template engine. Here are some common syntax patterns:

### Variables
```
{{ VARIABLE_NAME }}
```

### Conditionals
```
{% if CONDITION %}
  ...
{% elif OTHER_CONDITION %}
  ...
{% else %}
  ...
{% endif %}
```

### Loops
```
{% for item in items %}
  {{ item }}
{% endfor %}
```

### Filters
```
{{ variable | filter_name }}
{{ variable | filter_name(arg=value) }}
```

### Built-in `get_env()` Function

tmpltool uses Tera's built-in `get_env()` function for accessing environment variables with optional defaults:

```
{{ get_env(name="VARIABLE_NAME", default="fallback_value") }}
```

**Examples:**
```
# With default value (recommended)
port = {{ get_env(name="PORT", default="8080") }}
database = {{ get_env(name="DB_URL", default="postgres://localhost/mydb") }}

# Without default (will error if variable doesn't exist)
api_key = {{ get_env(name="API_KEY") }}

# Use in conditionals
{% if get_env(name="DEBUG", default="false") == "true" %}
  Debug mode enabled
{% endif %}
```

**Benefits:**
- No template errors when environment variables are missing
- Sensible defaults for development
- Easy to override in production
- Self-documenting configuration

See [examples/config-with-defaults.tmpl](examples/config-with-defaults.tmpl) for a complete example.

### Comments
```
{# This is a comment #}
```

For complete Tera syntax documentation, visit: https://keats.github.io/tera/docs/

## Error Handling

- If a template file doesn't exist, tmpltool will exit with an error
- If a template has syntax errors, tmpltool will report the error location
- If a referenced environment variable doesn't exist:
  - Using `{{ VAR }}` syntax will error (strict mode)
  - Using `{{ get_env(name="VAR", default="...") }}` will use the default value
  - Using `{{ get_env(name="VAR") }}` without default will error

## Help

```bash
tmpltool --help
```

## Version

```bash
tmpltool --version
```

## Development

### Prerequisites

- Rust 1.70 or higher
- Cargo (comes with Rust)

Install Rust from [rustup.rs](https://rustup.rs/) if you haven't already:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Project Structure

```
tmpltool/
├── Cargo.toml              # Project dependencies and metadata
├── Cargo.lock              # Locked dependency versions
├── Makefile.toml           # cargo-make task definitions
├── src/
│   ├── main.rs             # Entry point (binary)
│   ├── lib.rs              # Library root
│   ├── cli.rs              # CLI argument parsing
│   ├── renderer.rs         # Template rendering logic
│   └── functions/          # Custom Tera functions (extensibility point)
│       └── mod.rs          # Functions module (for registering custom functions)
├── tests/                  # Integration tests (one test per file)
│   ├── common.rs           # Shared test utilities
│   ├── fixtures/           # Test fixtures (templates & expected outputs)
│   │   ├── templates/      # Input template files
│   │   ├── expected/       # Expected output files
│   │   └── README.md       # Fixtures documentation
│   ├── test_successful_rendering.rs
│   ├── test_missing_template_file.rs
│   ├── test_invalid_template_syntax.rs
│   ├── test_environment_variable_substitution.rs
│   ├── test_template_with_conditionals.rs
│   ├── test_template_with_missing_variable.rs
│   ├── test_multiline_template.rs
│   ├── test_stdout_output.rs
│   └── test_direct_var_access_fails.rs
├── examples/               # Example templates
│   ├── basic.tmpl          # Basic usage example
│   ├── greeting.tmpl       # Simple greeting with defaults
│   ├── config.tmpl         # Config file generation
│   ├── config-with-defaults.tmpl # Advanced config with get_env() function
│   ├── docker-compose.tmpl # Docker Compose with defaults
│   └── README.md           # Examples documentation
├── .gitignore              # Git ignore rules
├── .editorconfig           # Editor configuration
└── README.md               # This file
```

#### Module Organization

**Source Code (`src/`)** - No tests in source files:
- **`main.rs`** - Minimal binary entry point, just parses CLI args and calls the library
- **`lib.rs`** - Public library API, exports main functionality
- **`cli.rs`** - CLI argument parsing using clap
- **`renderer.rs`** - Core template rendering logic (no unit tests)
- **`functions/`** - Custom Tera functions (modular, one file per function)
  - **`mod.rs`** - Registers custom functions (currently empty - built-in functions like `get_env()` work automatically)

**Tests (`tests/`)** - All tests as integration tests:
- **`common.rs`** - Shared test utilities and fixture helpers
- **`fixtures/`** - Test fixtures (templates and expected outputs)
- **Individual test files** - One test per file for better organization (11 tests total)

#### Adding New Custom Functions

To add a new custom function:

1. Create a new file in `src/functions/` (e.g., `my_function.rs`)
2. Implement your function following the Tera function signature
3. Add the module declaration to `src/functions/mod.rs`
4. Register your function in the `register_all()` function
5. Write tests in your function file

Note: Tera's built-in functions like `get_env()`, `now()`, and `get_random()` are automatically available when the "builtins" feature is enabled.

### Building

Build the project in debug mode:

```bash
cargo build
```

Build optimized release binary:

```bash
cargo build --release
```

The release binary will be located at `./target/release/tmpltool`.

### Using cargo-make (Task Runner)

This project includes a comprehensive `Makefile.toml` for [cargo-make](https://github.com/sagiegurari/cargo-make), providing standardized tasks for building, testing, and cross-platform compilation.

#### Installation

Install cargo-make globally:

```bash
cargo install --force cargo-make
```

#### Available Tasks

View all available tasks:

```bash
cargo make
```

**Common Development Tasks:**

```bash
# Build and test
cargo make build              # Build debug binary
cargo make build-release      # Build optimized binary
cargo make test              # Run all tests
cargo make test-verbose      # Run tests with output
cargo make run               # Run with example template

# Code quality
cargo make format            # Format code with rustfmt
cargo make clippy            # Run clippy linter
cargo make qa                # Full quality check (format + clippy + test)
cargo make ci                # CI checks (format-check + clippy + test)

# Utilities
cargo make clean             # Clean build artifacts
cargo make check             # Fast compile check
cargo make docs              # Generate and open documentation
cargo make test-examples     # Test all example templates
```

**Cross-Platform Builds:**

Build release binaries for different platforms:

```bash
# Individual platforms
cargo make build-linux-x86_64      # Linux x86_64
cargo make build-linux-musl        # Linux (static, musl libc)
cargo make build-macos-x86_64      # macOS Intel
cargo make build-macos-aarch64     # macOS Apple Silicon
cargo make build-windows-x86_64    # Windows x86_64

# Build for all platforms
cargo make build-all-platforms
```

**Note:** Cross-compilation may require installing additional targets:

```bash
# Add targets for cross-compilation
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-unknown-linux-musl
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
rustup target add x86_64-pc-windows-gnu
```

**Workflow Tasks:**

```bash
cargo make dev                # Quick dev check (check + test)
cargo make pre-commit         # Pre-commit checks
cargo make release-prepare    # Full release preparation
cargo make all                # Complete build and test suite
```

**Additional Tools:**

```bash
cargo make audit              # Security audit (requires cargo-audit)
cargo make outdated           # Check outdated dependencies (requires cargo-outdated)
cargo make bloat              # Analyze binary size (requires cargo-bloat)
```

### Running in Development

Run without building a binary:

```bash
cargo run -- template.txt -o output.txt
```

Run with environment variables:

```bash
CUSTOM_VAR="test" cargo run -- template.txt
```

### Testing

Run all unit tests:

```bash
cargo test
```

Run tests with output:

```bash
cargo test -- --nocapture
```

Run a specific test:

```bash
cargo test test_successful_rendering
```

Run tests in verbose mode:

```bash
cargo test -- --test-threads=1 --nocapture
```

#### Test Coverage

The project includes comprehensive test coverage. **All tests are located in `tests/` directory** - there are no unit tests in `src/` files.

**Integration Tests in `tests/`** (11 tests, one per file):
- `test_simple_rendering.rs` - Simple static template rendering
- `test_successful_rendering.rs` - Template rendering with environment variables
- `test_env_with_default.rs` - Environment variable with default fallback
- `test_missing_template_file.rs` - Missing template file handling
- `test_invalid_template_syntax.rs` - Invalid template syntax handling
- `test_environment_variable_substitution.rs` - Environment variable substitution with `get_env()`
- `test_template_with_conditionals.rs` - Conditional logic (if/else)
- `test_template_with_missing_variable.rs` - Missing variable detection
- `test_multiline_template.rs` - Multiline templates
- `test_stdout_output.rs` - Stdout output functionality
- `test_direct_var_access_fails.rs` - Direct variable access fails (security test)

**Test Infrastructure:**
- `common.rs` - Shared test utilities and fixture helpers
- `fixtures/` - Test fixtures (templates and expected outputs)

**Documentation Tests** (2 tests):
- Library documentation examples

Total: **13 tests** covering integration and documentation scenarios.

#### Adding New Integration Tests

To add a new integration test:

1. Create a new file in `tests/` (e.g., `tests/test_my_feature.rs`)
2. Import the common utilities: `mod common;` and `use common::*;`
3. Import dependencies: `use tmpltool::render_template;`
4. Write your test function with `#[test]` attribute
5. Use helper functions: `get_test_file_path()` and `cleanup_test_file()`

**Example:**

```rust
mod common;

use common::{cleanup_test_file, get_test_file_path};
use std::fs;
use tmpltool::render_template;

#[test]
fn test_my_feature() {
    let template_path = get_test_file_path("my_template.txt");
    let output_path = get_test_file_path("my_output.txt");

    // Create test template
    fs::write(&template_path, "{{ get_env(name=\"TEST\") }}").unwrap();

    // Run render_template
    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
    );

    // Assert results
    assert!(result.is_ok());

    // Cleanup
    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}
```

Each test file is compiled as a separate test binary, making tests more isolated and easier to debug.

#### Using Test Fixtures

The project uses test fixtures to make tests easier to maintain. Fixtures are template files and their expected outputs stored in `tests/fixtures/`.

**Fixture Directory Structure:**

```
tests/fixtures/
├── templates/           # Input template files
│   ├── simple.tmpl
│   ├── with_env.tmpl
│   ├── multiline.tmpl
│   ├── conditionals.tmpl
│   └── docker-compose.tmpl
└── expected/            # Expected output files
    ├── simple.txt
    ├── with_env.txt
    ├── multiline.txt
    └── docker-compose.txt
```

**Using Fixtures in Tests:**

```rust
mod common;

use common::{
    cleanup_test_file, get_test_file_path,
    read_fixture_expected, read_fixture_template
};
use std::fs;
use tmpltool::render_template;

#[test]
fn test_my_feature() {
    let output_path = get_test_file_path("output.txt");

    // Read template from fixtures
    let template_content = read_fixture_template("my_template.tmpl");
    let template_path = get_test_file_path("template.txt");
    fs::write(&template_path, template_content).unwrap();

    // Render template
    let result = render_template(
        Some(template_path.to_str().unwrap()),
        Some(output_path.to_str().unwrap()),
    );

    // Compare with expected output
    assert!(result.is_ok());
    let output = fs::read_to_string(&output_path).unwrap();
    let expected = read_fixture_expected("my_template.txt");
    assert_eq!(output, expected);

    // Cleanup
    cleanup_test_file(&template_path);
    cleanup_test_file(&output_path);
}
```

**Benefits:**
- ✅ Test data separated from test logic
- ✅ Easy to maintain and review template changes
- ✅ Reusable across multiple tests
- ✅ Can use real-world template examples

See [tests/fixtures/README.md](tests/fixtures/README.md) for more details.

### Code Quality

Format code:

```bash
cargo fmt
```

Run linter (clippy):

```bash
cargo clippy
```

Run clippy with all warnings:

```bash
cargo clippy -- -W clippy::all
```

### Dependencies

The project uses minimal dependencies:

- **[tera](https://crates.io/crates/tera)** (v1.x) - Template engine with `builtins` feature enabled
  - Provides built-in filters: `slugify`, `date`, `filesizeformat`, `urlencode`, etc.
  - Provides built-in functions: `get_env()`, `now()`, `get_random()`
- **[clap](https://crates.io/crates/clap)** (v4.x) - Command-line argument parsing

To update dependencies:

```bash
cargo update
```

### Manual Testing

Create a test template:

```bash
cat > test.tmpl << 'EOF'
User: {{ USER }}
Home: {{ HOME }}
Custom: {{ CUSTOM_VAR }}
EOF
```

Test it:

```bash
CUSTOM_VAR="Hello World" ./target/release/tmpltool test.tmpl
```

### Debugging

Run with debug output:

```bash
RUST_BACKTRACE=1 cargo run -- template.txt
```

Full backtrace:

```bash
RUST_BACKTRACE=full cargo run -- template.txt
```

### Performance

Benchmark the binary size:

```bash
ls -lh target/release/tmpltool
```

Profile with release build:

```bash
cargo build --release
time ./target/release/tmpltool large_template.txt -o output.txt
```

### Installing Locally

Install from the project directory:

```bash
cargo install --path .
```

This installs the binary to `~/.cargo/bin/tmpltool` (make sure this is in your PATH).

Uninstall:

```bash
cargo uninstall tmpltool
```

### Using as a Library

tmpltool can also be used as a library in other Rust projects:

```rust
use tmpltool::render_template;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Render to file
    render_template("template.txt", Some("output.txt"))?;

    // Render to stdout
    render_template("template.txt", None)?;

    Ok(())
}
```

The library exposes:
- `render_template(template_path: &str, output_path: Option<&str>)` - Main rendering function
- `Cli` - Command-line argument structure

## CI/CD

This project uses GitHub Actions for continuous integration and automated releases.

### Continuous Integration

Every pull request and push to master/main triggers:

- **Code Formatting Check** - Ensures code follows Rust style guidelines (`rustfmt`)
- **Linting** - Runs `clippy` with strict warnings
- **Multi-Platform Tests** - Tests on Ubuntu, macOS, and Windows
- **Code Coverage** - Generates coverage reports with `cargo-tarpaulin` and uploads to Codecov
- **cargo-make QA** - Runs comprehensive quality checks
- **Example Testing** - Tests all example templates to ensure they work

### Automated Releases

Releases are fully automated using [semantic-release](https://github.com/semantic-release/semantic-release):

1. **Commit Analysis** - Analyzes commit messages to determine the next version
2. **Version Bumping** - Updates `Cargo.toml` with the new version
3. **CHANGELOG Generation** - Automatically generates `CHANGELOG.md` from commits
4. **Multi-Platform Builds** - Builds release binaries for:
   - Linux (x86_64, x86_64-musl, aarch64)
   - macOS (x86_64 Intel, aarch64 Apple Silicon)
   - Windows (x86_64)
5. **GitHub Release** - Creates a new GitHub release with all binaries
6. **Docker Image** - Builds and publishes multi-arch Docker image to GHCR

### Commit Convention

This project follows [Conventional Commits](https://www.conventionalcommits.org/) for automatic versioning:

- `feat: description` - New feature (minor version bump: 1.2.0 → 1.3.0)
- `fix: description` - Bug fix (patch version bump: 1.2.0 → 1.2.1)
- `feat!: description` or `BREAKING CHANGE:` - Breaking change (major version bump: 1.2.0 → 2.0.0)
- `docs:`, `refactor:`, `perf:`, `build:` - Other changes (patch bump)
- `style:`, `test:`, `chore:`, `ci:` - No version bump

**Examples:**

```bash
# Feature (minor bump)
git commit -m "feat: add slugify filter support"

# Bug fix (patch bump)
git commit -m "fix: correct multiline template rendering"

# Breaking change (major bump)
git commit -m "feat!: change default output behavior

BREAKING CHANGE: Output now goes to stdout by default instead of file"
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed contribution guidelines.

## License

This project is open source.

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

For detailed contribution guidelines, including commit conventions, development workflow, and testing requirements, see [CONTRIBUTING.md](CONTRIBUTING.md).

### Quick Start

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests and QA checks (`cargo make qa`)
5. Commit using [conventional commits](#commit-convention)
6. Push to your fork
7. Open a Pull Request
