# tmpltool

A fast and simple command-line template rendering tool using [Tera](https://keats.github.io/tera/) templates with environment variables.

## Features

- Render Tera templates with all environment variables available as context
- Output to file or stdout (for piping)
- Simple CLI interface
- Single binary executable
- Full Tera template syntax support (variables, conditionals, loops, filters, etc.)

## Installation

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
echo "Hello {{ env(name=\"NAME\", default=\"World\") }}!" | tmpltool
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
- **`greeting.tmpl`** - Simple greeting with `env()` function
- **`config.tmpl`** - Application configuration file generation
- **`docker-compose.tmpl`** - Docker Compose with sensible defaults
- **`config-with-defaults.tmpl`** - Advanced config using `env()` function (recommended)

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

# Config with env() function and defaults
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

### Custom `env()` Function

tmpltool provides a custom `env()` function for accessing environment variables with optional defaults:

```
{{ env(name="VARIABLE_NAME", default="fallback_value") }}
```

**Examples:**
```
# With default value (recommended)
port = {{ env(name="PORT", default="8080") }}
database = {{ env(name="DB_URL", default="postgres://localhost/mydb") }}

# Without default (will error if variable doesn't exist)
api_key = {{ env(name="API_KEY") }}

# Use in conditionals
{% if env(name="DEBUG", default="false") == "true" %}
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
  - Using `{{ env(name="VAR", default="...") }}` will use the default value
  - Using `{{ env(name="VAR") }}` without default will error

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
├── src/
│   ├── main.rs             # Entry point (binary)
│   ├── lib.rs              # Library root
│   ├── cli.rs              # CLI argument parsing
│   ├── renderer.rs         # Template rendering logic
│   └── functions/          # Custom Tera functions
│       ├── mod.rs          # Functions module (registers all functions)
│       └── env.rs          # env() function implementation
├── tests/
│   └── integration_tests.rs # Integration tests
├── examples/               # Example templates
│   ├── basic.tmpl          # Basic usage example
│   ├── greeting.tmpl       # Simple greeting with defaults
│   ├── config.tmpl         # Config file generation
│   ├── config-with-defaults.tmpl # Advanced config with env() function
│   ├── docker-compose.tmpl # Docker Compose with defaults
│   └── README.md           # Examples documentation
├── .gitignore              # Git ignore rules
├── .editorconfig           # Editor configuration
└── README.md               # This file
```

#### Module Organization

- **`main.rs`** - Minimal binary entry point, just parses CLI args and calls the library
- **`lib.rs`** - Public library API, exports main functionality
- **`cli.rs`** - CLI argument parsing using clap
- **`renderer.rs`** - Core template rendering logic with environment variable handling
- **`functions/`** - Custom Tera functions (modular, one file per function)
  - **`mod.rs`** - Registers all functions, easy to add new ones
  - **`env.rs`** - Implementation of the `env()` function
- **`tests/integration_tests.rs`** - Integration tests that test the public API

#### Adding New Custom Functions

To add a new custom function:

1. Create a new file in `src/functions/` (e.g., `uppercase.rs`)
2. Implement your function following the Tera function signature
3. Add the module declaration to `src/functions/mod.rs`
4. Register your function in the `register_all()` function
5. Write tests in your function file

See `src/functions/env.rs` for a complete example.

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

The project includes comprehensive test coverage:

**Unit Tests in `src/renderer.rs`** (7 tests):
- Context building from environment variables
- Template rendering
- Invalid template syntax handling
- Template file reading
- Missing template file handling
- `env()` function in template rendering
- `env()` function with default in template

**Unit Tests in `src/functions/env.rs`** (5 tests):
- `env()` function with existing variable
- `env()` function with default fallback
- `env()` function without default (error handling)
- `env()` function with missing name argument
- `env()` function with numeric default

**Integration Tests in `tests/integration_tests.rs`** (8 tests):
- Successful template rendering
- Missing template file handling
- Invalid template syntax handling
- Environment variable substitution
- Conditional logic (if/else)
- Missing variable detection
- Multiline templates
- Stdout output functionality

**Function Tests** (2 tests in lib tests):
- Documentation tests

Total: **22 tests** covering unit, integration, and function-specific scenarios.

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

- **[tera](https://crates.io/crates/tera)** (v1.x) - Template engine
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

## License

This project is open source.

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

### How to Contribute

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests (`cargo test`)
5. Run formatter (`cargo fmt`)
6. Run clippy (`cargo clippy`)
7. Commit your changes (`git commit -m 'Add amazing feature'`)
8. Push to the branch (`git push origin feature/amazing-feature`)
9. Open a Pull Request
