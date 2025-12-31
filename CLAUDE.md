# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`tmpltool` is a fast, single-binary command-line template rendering tool built in Rust. It uses MiniJinja (Jinja2-compatible) templates with environment variables and provides extensive custom functions for hash generation, filesystem operations, data parsing, and validation.

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
│   ├── environment.rs
│   ├── hash.rs
│   ├── filesystem.rs
│   ├── data_parsing.rs
│   ├── validation.rs
│   ├── datetime.rs
│   ├── random.rs
│   └── uuid_gen.rs
└── filters/          - Custom template filters
    ├── mod.rs
    ├── formatting.rs
    └── string.rs
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

### Adding New Functions

When adding new template functions:

1. **Create function file** in `src/functions/` (e.g., `network.rs`)
2. **Implement function** using MiniJinja patterns:
   ```rust
   use minijinja::value::Kwargs;
   use minijinja::{Error, Value};

   pub fn my_function(kwargs: Kwargs) -> Result<Value, Error> {
       let arg: String = kwargs.get("arg_name")?;
       // Implementation
       Ok(Value::from(result))
   }
   ```
3. **Add module declaration** in `src/functions/mod.rs`: `pub mod network;`
4. **Register function** in `register_all()`: `env.add_function("my_function", network::my_function);`
5. **Write tests** in `tests/test_my_function.rs`
6. **Document** in README.md with examples

**For context-aware functions (filesystem access):**
```rust
use std::sync::Arc;
use crate::TemplateContext;

pub fn create_my_fn(context: Arc<TemplateContext>) -> impl Fn(Kwargs) -> Result<Value, Error> {
    move |kwargs: Kwargs| {
        let path: String = kwargs.get("path")?;
        let resolved = context.validate_and_resolve_path(&path)?;
        // Use resolved path
        Ok(Value::from(result))
    }
}
```

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

See `TODO.md` for comprehensive list of proposed features organized by category:
- Network & System Functions (hostname, IP, DNS resolution)
- Math & Calculation Functions (min, max, round, percentage)
- Enhanced String Manipulation (case conversion, padding)
- Advanced Date/Time Functions (parsing, timezone conversion)
- Security & Encoding (base64, bcrypt, HMAC)
- Container/Orchestration Helpers (Kubernetes label sanitization)

When implementing features from TODO.md:
- Follow existing patterns (modular function files)
- Add comprehensive tests
- Document with real-world examples
- Consider security implications
- Update README.md with usage examples
