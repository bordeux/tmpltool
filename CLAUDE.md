# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`tmpltool` is a fast, single-binary command-line template rendering tool built in Rust. It uses MiniJinja (Jinja2-compatible) templates with environment variables and provides extensive custom functions for hash generation, filesystem operations, data parsing, and validation.

## Big Picture - What This Tool Does

### Purpose
`tmpltool` is a **configuration file generator** that renders Jinja2-style templates by combining:
- Environment variables
- File contents (JSON, YAML, TOML)
- System information (hostname, IP, OS)
- Generated values (UUIDs, random strings, hashes)

It's designed for **DevOps/SRE workflows** like generating Kubernetes manifests, Docker Compose files, application configs, and deployment scripts.

### Typical Use Cases
1. **Kubernetes manifests** - Generate YAML with environment-specific values, resource limits, labels
2. **Docker Compose** - Template service configurations with dynamic ports, hosts, secrets
3. **Application configs** - Create config files (JSON/YAML/TOML) from environment variables
4. **Build scripts** - Generate shell scripts with embedded build metadata (git hash, timestamp)
5. **CI/CD pipelines** - Produce deployment configs that vary by environment

### Function Categories (100+ functions)

| Category | Examples | Description |
|----------|----------|-------------|
| **Environment** | `get_env`, `filter_env` | Access and filter environment variables |
| **Hash & Crypto** | `md5`, `sha256`, `sha512`, `bcrypt`, `hmac_sha256` | Checksums, password hashing, signatures |
| **Encoding** | `base64_encode/decode`, `hex_encode/decode`, `escape_html/xml/shell` | Data encoding and escaping |
| **UUID & Random** | `uuid`, `random_string`, `generate_secret` | Generate unique IDs and secure random values |
| **Date/Time** | `now`, `format_date`, `parse_date`, `date_add`, `date_diff` | Timestamps, formatting, date math |
| **Filesystem** | `read_file`, `file_exists`, `glob`, `file_size`, `list_dir` | Read files, check existence, find files |
| **Path Manipulation** | `basename`, `dirname`, `join_path`, `file_extension` | Path operations (no security restrictions) |
| **Data Parsing** | `parse_json/yaml/toml`, `read_json_file`, `read_yaml_file` | Parse structured data |
| **Data Serialization** | `to_json`, `to_yaml`, `to_toml` | Convert objects to formatted strings |
| **Object Manipulation** | `object_merge`, `object_get/set`, `object_keys/values`, `object_pick/omit` | Deep merge, nested access, transformation |
| **Validation** | `is_email`, `is_url`, `is_ip`, `is_uuid`, `matches_regex` | Validate string formats |
| **String Manipulation** | `slugify`, `to_snake_case`, `indent`, `truncate`, `regex_replace` | Case conversion, formatting, regex |
| **Array Functions** | `array_sort_by`, `array_group_by`, `array_unique`, `array_filter_by`, `array_pluck` | Sort, group, filter, transform arrays |
| **Statistics** | `array_sum`, `array_avg`, `array_median`, `array_min/max` | Numeric calculations on arrays |
| **Math** | `min`, `max`, `round`, `ceil`, `floor`, `percentage`, `abs` | Mathematical operations |
| **Logic** | `default`, `coalesce`, `ternary`, `in_range` | Conditional logic helpers |
| **Predicates** | `array_any`, `array_all`, `array_contains`, `starts_with`, `ends_with` | Boolean checks |
| **System & Network** | `get_hostname`, `get_ip_address`, `resolve_dns`, `is_port_available`, `get_os` | System info, DNS, network |
| **CIDR/IP** | `cidr_contains`, `cidr_network`, `cidr_netmask`, `ip_to_int` | IP address and subnet operations |
| **Kubernetes** | `k8s_resource_request`, `k8s_label_safe`, `k8s_probe`, `k8s_secret_ref` | K8s-specific YAML helpers |
| **Web/URL** | `parse_url`, `build_url`, `query_string`, `basic_auth` | URL manipulation, HTTP auth |
| **Command Execution** | `exec`, `exec_raw` | Run shell commands (trust mode only) |
| **Debugging** | `debug`, `inspect`, `type_of`, `assert`, `warn`, `abort` | Development and error handling |

### Key Design Decisions
1. **Explicit environment access** - Use `get_env()` instead of auto-exposing all env vars (security)
2. **Dual syntax for filters** - Most functions work as both `func(arg=x)` and `x | func` (flexibility)
3. **Dual syntax for is-functions** - All `is_*` functions work as both `is_email(string=x)` and `{% if x is email %}` (readability)
4. **Security by default** - Filesystem restricted to CWD; use `--trust` for full access
5. **Output validation** - `--validate json/yaml/toml` ensures valid output format
6. **Single binary** - No runtime dependencies, easy to deploy in containers
7. **IDE integration** - `--ide-json` outputs all function metadata (83 functions) for editor autocomplete/docs

### CLI Quick Reference
```bash
# Basic usage
tmpltool template.tmpl                    # Render to stdout
tmpltool template.tmpl -o output.txt      # Render to file
echo '{{ now() }}' | tmpltool             # Pipe template from stdin

# With options
tmpltool --trust system.tmpl              # Allow filesystem access outside CWD
tmpltool config.tmpl --validate json      # Validate output is valid JSON
tmpltool --ide-json                       # Output function metadata as JSON (for IDE integration)

# With environment variables
DB_HOST=prod-db APP_ENV=production tmpltool config.tmpl
```

### Template Quick Reference
```jinja
{# Environment variables #}
{{ get_env(name="PORT", default="8080") }}
{% for var in filter_env(pattern="DB_*") %}
  {{ var.key }}={{ var.value }}
{% endfor %}

{# File operations #}
{% set config = read_json_file(path="config.json") %}
{{ config.server.host }}

{# Conditionals (use {% set %} first) #}
{% set debug = get_env(name="DEBUG", default="false") %}
{% if debug == "true" %}Debug enabled{% endif %}

{# Kubernetes example #}
resources:
  {{ k8s_resource_request(cpu="500m", memory="512Mi") | indent(2) }}
```

## Common Commands

**Note:** This project uses `cargo-make` for task automation. Install it once with:
```bash
cargo install --force cargo-make
```

### Building
```bash
# Debug build
cargo make build

# Release build (optimized)
cargo make build-release
# Binary location: ./target/release/tmpltool

# Fast compile check (no binary)
cargo make check

# Clean build artifacts
cargo make clean
```

### Testing
```bash
# Run all tests
cargo make test

# Run with verbose output
cargo make test-verbose

# Run specific test (use cargo directly)
cargo test test_name

# Test all example templates
cargo make test-examples
```

### Code Quality
```bash
# Format code (auto-fix)
cargo make format

# Check formatting without changes
cargo make format-check

# Run linter
cargo make clippy

# Run linter with auto-fix
cargo make clippy-fix

# Full QA check (format + clippy + test)
cargo make qa

# CI checks (format-check + clippy + test)
cargo make ci

# Pre-commit checks
cargo make pre-commit
```

### Running the Tool
```bash
# Run with example (uses cargo make)
cargo make run

# From source with custom template (use cargo directly)
cargo run -- examples/greeting.tmpl

# From release binary
./target/release/tmpltool examples/greeting.tmpl

# With environment variables
NAME="Alice" ./target/release/tmpltool examples/greeting.tmpl

# With trust mode (allows filesystem access outside CWD)
./target/release/tmpltool --trust system_info.tmpl

# Output to file
./target/release/tmpltool template.tmpl -o output.txt

# Read from stdin
echo 'Hello {{ get_env(name="USER") }}!' | ./target/release/tmpltool
```

### Documentation
```bash
# Generate and open documentation
cargo make docs

# Generate documentation without opening
cargo make docs-build
```

### Utilities
```bash
# Install binary to ~/.cargo/bin
cargo make install

# Uninstall binary
cargo make uninstall

# Security audit of dependencies
cargo make audit

# Check for outdated dependencies
cargo make outdated

# Update dependencies
cargo make update
```

### Cross-Platform Builds
```bash
cargo make build-linux-x86_64      # Linux x86_64
cargo make build-linux-musl        # Linux (static)
cargo make build-macos-x86_64      # macOS Intel
cargo make build-macos-aarch64     # macOS Apple Silicon
cargo make build-windows-x86_64    # Windows
cargo make build-all-platforms     # All platforms
```

## Architecture

### High-Level Structure

The codebase follows a modular architecture with clear separation of concerns:

```
src/
├── main.rs           - Entry point (CLI parsing, error handling)
├── lib.rs            - Public API exports
├── cli.rs            - Command-line argument definitions (Clap)
├── context.rs        - Template execution context (base path, trust mode)
├── renderer.rs       - Core template rendering logic (MiniJinja setup)
├── functions/        - Custom template functions (modular)
│   ├── mod.rs        - Function registration with MiniJinja
│   ├── metadata.rs   - FunctionMetadata types for IDE integration
│   ├── environment.rs
│   ├── hash.rs
│   ├── filesystem.rs
│   ├── data_parsing.rs
│   ├── validation.rs
│   ├── datetime.rs
│   ├── random.rs
│   └── uuid_gen.rs
├── is_functions/     - Is-functions with dual syntax (function + "is" test)
│   ├── mod.rs        - Registration for both function and "is" test syntax
│   ├── traits.rs     - IsFunction and ContextIsFunction trait definitions
│   ├── validation.rs - Email, Url, Ip, Uuid validators
│   ├── datetime.rs   - LeapYear check
│   ├── network.rs    - PortAvailable check
│   └── filesystem.rs - File, Dir, Symlink checks (context-aware)
└── filter_functions/ - Unified filter-functions (dual syntax)
    ├── mod.rs        - Registration for both function and filter syntax
    ├── traits.rs     - FilterFunction trait definition
    ├── string.rs     - String manipulation (slugify, case conversion, etc.)
    ├── formatting.rs - Formatting (filesizeformat, urlencode)
    ├── hash.rs       - Hash functions (md5, sha256, etc.)
    ├── encoding.rs   - Encoding (base64, hex, escape)
    └── ...
```

### Key Architectural Patterns

**1. Template Context (`TemplateContext`)**
- Manages base directory for relative path resolution
- Enforces security restrictions (trust mode vs. restricted mode)
- Shared across all filesystem functions via `Arc<TemplateContext>`
- Created in `renderer.rs`, passed to function registration

**2. Function Registration Pattern**
- All functions registered in `functions::mod.rs::register_all()`
- Simple functions (no context): Direct function references
- Context-aware functions: Factory pattern using closures with `Arc<TemplateContext>`
- MiniJinja's `Kwargs` pattern for named arguments: `kwargs.get("arg_name")?`

**3. Security Model**
- Default mode: Only relative paths within CWD allowed
- Trust mode (`--trust` flag): Unrestricted filesystem access
- Path validation in `TemplateContext::validate_and_resolve_path()`
- Security checks: Absolute paths (`/`), parent traversal (`..`)

**4. Rendering Flow**
```
main.rs → render_template() → read_template() → render() → write_output()
                                                    ↓
                              Environment::new() + functions::register_all()
                                                    ↓
                              Template parsing + rendering with context
```

**5. Trait-Based Metadata System**
- All filter/is-functions implement traits with required `METADATA` constant
- `FilterFunction` trait: dual function + filter syntax (e.g., `md5`)
- `IsFunction` / `ContextIsFunction` traits: dual function + is-test syntax (e.g., `is_email`)
- Metadata includes: name, category, description, arguments, return type, examples, syntax variants
- `--ide-json` collects metadata from all traits via `get_all_metadata()` in lib.rs
- Ensures documentation stays in sync with implementation (single source of truth)

### Adding New Functions

Functions use a trait-based system with **required metadata** for IDE integration. When implementing traits (`FilterFunction`, `IsFunction`, `ContextIsFunction`), you MUST provide a `METADATA` constant.

**For filter functions (dual function + filter syntax):**

Add to `src/filter_functions/` and implement `FilterFunction` trait:
```rust
use crate::filter_functions::FilterFunction;
use crate::functions::metadata::{ArgumentMetadata, FunctionMetadata, SyntaxVariants};
use minijinja::value::Kwargs;
use minijinja::{Error, Value};

pub struct MyFilter;

impl FilterFunction for MyFilter {
    const NAME: &'static str = "my_filter";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "my_filter",
        category: "string",
        description: "Description of what this does",
        arguments: &[ArgumentMetadata {
            name: "string",
            arg_type: "string",
            required: true,
            default: None,
            description: "Input string",
        }],
        return_type: "string",
        examples: &[
            "{{ my_filter(string=\"hello\") }}",
            "{{ \"hello\" | my_filter }}",
        ],
        syntax: SyntaxVariants::FUNCTION_AND_FILTER,
    };

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let input: String = kwargs.get("string")?;
        Ok(Value::from(input.to_uppercase()))
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        let input = value.as_str().unwrap_or_default();
        Ok(Value::from(input.to_uppercase()))
    }
}
```

Then register in `src/filter_functions/mod.rs`:
```rust
MyFilter::register(env);
```

And add to `get_all_metadata()`:
```rust
&my_module::MyFilter::METADATA,
```

**For is-functions (dual function + is-test syntax):**

Add to `src/is_functions/` and implement `IsFunction` or `ContextIsFunction` trait:
```rust
use crate::is_functions::IsFunction;
use crate::functions::metadata::{ArgumentMetadata, FunctionMetadata, SyntaxVariants};

pub struct MyCheck;

impl IsFunction for MyCheck {
    const FUNCTION_NAME: &'static str = "is_my_check";
    const IS_NAME: &'static str = "my_check";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "is_my_check",
        category: "validation",
        description: "Check if value passes my_check",
        arguments: &[...],
        return_type: "boolean",
        examples: &[
            "{{ is_my_check(string=\"test\") }}",
            "{% if \"test\" is my_check %}...{% endif %}",
        ],
        syntax: SyntaxVariants::FUNCTION_AND_TEST,
    };

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> { ... }
    fn call_as_is(value: &Value) -> bool { ... }
}
```

**For standalone functions in `src/functions/`:**

These still use direct function registration:
```rust
pub fn my_function(kwargs: Kwargs) -> Result<Value, Error> {
    let arg: String = kwargs.get("arg_name")?;
    Ok(Value::from(result))
}
```

Register in `register_all()`: `env.add_function("my_function", my_module::my_function);`

**For context-aware functions (filesystem access):**
```rust
use std::sync::Arc;
use crate::TemplateContext;

pub fn create_my_fn(context: Arc<TemplateContext>) -> impl Fn(Kwargs) -> Result<Value, Error> {
    move |kwargs: Kwargs| {
        let path: String = kwargs.get("path")?;
        let resolved = context.validate_and_resolve_path(&path)?;
        Ok(Value::from(result))
    }
}
```

**Testing:** Write tests in `tests/test_my_function.rs` (never in src folder)

### Testing Philosophy

- Unit tests in `tests/` directory
- Integration tests use actual template rendering
- Security tests verify trust mode restrictions
- Example templates in `examples/` serve as integration tests
- Test helper pattern: `render_template_from_string()` in test files

## Development Workflow

### Before Committing

```bash
# Run full QA check (recommended)
cargo make qa

# Or use pre-commit task
cargo make pre-commit

# Quick development check
cargo make dev
```

### Release Preparation

```bash
# Prepare for release (clean + format + clippy + test + build-release)
cargo make release-prepare

# Full build and test suite
cargo make all
```

**Commit message format:** This project uses [Conventional Commits](https://www.conventionalcommits.org/):
- `feat: description` - New feature (minor version bump)
- `fix: description` - Bug fix (patch version bump)
- `feat!: description` - Breaking change (major version bump)
- `docs:`, `refactor:`, `perf:` - Other changes (patch bump)
- `style:`, `test:`, `chore:`, `ci:` - No version bump

Husky pre-commit hooks validate commit message format.

**Important:** Do not include Claude model references (e.g., "Co-Authored-By: Claude") in commit messages. Keep commits clean and professional.

### Debugging Template Rendering

When debugging template issues:
1. Check MiniJinja error output (detailed with line/column info)
2. Test with minimal template first
3. Use `--trust` for filesystem debugging
4. Verify environment variables: `env | grep VAR_NAME`
5. Test functions in isolation (unit tests)

## Important Implementation Details

### MiniJinja vs. Tera
- Previously used Tera, migrated to MiniJinja
- MiniJinja is more lightweight, faster, better maintained
- Syntax is Jinja2-compatible
- Built-in filters available: `upper`, `lower`, `trim`, `slugify`, `filesizeformat`, `date`, etc.

### Environment Variables
- **NOT** automatically available in templates (unlike shell scripts)
- Must use `get_env(name="VAR", default="value")` function
- Design decision: Explicit is safer than implicit

### Path Resolution
- Templates can include other templates: `{% include "partial.tmpl" %}`
- Paths resolved relative to template's directory (or CWD if stdin)
- Lazy loading via `Environment::set_loader()`

### Error Handling
- Use descriptive error messages with context
- Include path information in filesystem errors
- MiniJinja provides excellent error formatting (use it)
- Return `Box<dyn std::error::Error>` from public APIs

## File Organization

### Examples Directory
`examples/` contains demonstration templates:
- `greeting.tmpl` - Simple variable substitution
- `basic.tmpl` - Environment variable filtering
- `config-with-defaults.tmpl` - Default values
- `docker-compose.tmpl` - Real-world Docker Compose generation
- `comprehensive-app-config.tmpl` - All features showcase

### Tests Organization
- `tests/test_*_unit.rs` - Unit tests for specific functions
- `tests/test_*_functions.rs` - Integration tests for function categories
- `tests/test_successful_rendering.rs` - End-to-end rendering tests
- `tests/test_invalid_template_syntax.rs` - Error handling tests

## Dependencies

Core dependencies:
- `minijinja` - Template engine (Jinja2-compatible)
- `clap` - CLI argument parsing (derive API)
- `serde` / `serde_json` - Serialization
- `regex` - Pattern matching
- `md-5`, `sha1`, `sha2` - Cryptographic hashing
- `uuid` - UUID generation
- `rand` - Random number/string generation
- `glob` - File pattern matching
- `serde_yaml`, `toml` - YAML/TOML parsing
- `chrono` - Date/time handling

## CI/CD

GitHub Actions workflows:
- `.github/workflows/ci.yml` - Format, clippy, tests, coverage
- `.github/workflows/release.yml` - Automated releases with semantic-release

Releases are automated:
1. Commit with conventional format
2. Push to `master`
3. `semantic-release` determines version
4. Updates `Cargo.toml`, generates `CHANGELOG.md`
5. Builds multi-platform binaries
6. Creates GitHub release
7. Publishes Docker images to GHCR

## Security Considerations

When working on filesystem functions:
- **Always** validate paths through `TemplateContext::validate_and_resolve_path()`
- Check trust mode before allowing absolute/parent paths
- Use descriptive security error messages (mention `--trust` flag)
- Test both restricted and trust modes
- Consider symlink attacks in path validation

When adding crypto functions:
- Document appropriate use cases (checksums vs. password hashing)
- Use established crates (`md-5`, `sha2`) not custom implementations
- Generate secure random values with `rand::thread_rng()`

## Future Enhancements

Most core functionality is now implemented. See `TODO.md` for any remaining proposed features.

**Already implemented (previously planned):**
- Network & System Functions: `get_hostname`, `get_ip_address`, `resolve_dns`, `is_port_available`, `get_os`, `get_arch`, CIDR functions
- Math Functions: `min`, `max`, `round`, `ceil`, `floor`, `percentage`, `abs`
- String Manipulation: `slugify`, `to_snake_case/camel_case/pascal_case/kebab_case`, `indent`, `pad_left/right`, `truncate`
- Date/Time: `parse_date`, `date_add`, `date_diff`, `timezone_convert`, `get_year/month/day/hour/minute/second`
- Encoding: `base64_encode/decode`, `hex_encode/decode`, `bcrypt`, `hmac_sha256`, `escape_html/xml/shell`
- Kubernetes: `k8s_resource_request`, `k8s_label_safe`, `k8s_probe`, `k8s_secret_ref`, `k8s_configmap_ref`, `k8s_toleration`, `k8s_pod_affinity`

When implementing new features:
- Follow existing patterns (modular function files in `src/functions/` or `src/filter_functions/`)
- Add comprehensive tests in `tests/` directory
- Document with real-world examples in README.md
- Consider security implications (filesystem access, command execution)
- Use the dual syntax pattern (function + filter) where appropriate
