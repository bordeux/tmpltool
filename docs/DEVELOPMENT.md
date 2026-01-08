# Development

## Prerequisites

- Rust 1.70 or higher
- Cargo (comes with Rust)

Install Rust from [rustup.rs](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Building

```bash
# Debug build
cargo build

# Release build
cargo build --release
# Binary at: ./target/release/tmpltool
```

## Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

## Code Quality

```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Full QA check (format + clippy + test)
cargo make qa
```

## Using cargo-make

Install cargo-make:

```bash
cargo install --force cargo-make
```

**Common tasks:**
```bash
cargo make build              # Build debug
cargo make build-release      # Build release
cargo make test              # Run tests
cargo make qa                # Full QA (format + clippy + test)
cargo make ci                # CI checks
cargo make docs              # Generate docs
```

**Cross-platform builds:**
```bash
cargo make build-linux-x86_64      # Linux x86_64
cargo make build-linux-musl        # Linux (static)
cargo make build-macos-x86_64      # macOS Intel
cargo make build-macos-aarch64     # macOS Apple Silicon
cargo make build-windows-x86_64    # Windows
cargo make build-all-platforms     # All platforms
```
