# Contributing to tmpltool

Thank you for your interest in contributing to tmpltool! This document provides guidelines for contributing to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Commit Convention](#commit-convention)
- [Pull Request Process](#pull-request-process)
- [Testing](#testing)
- [Code Style](#code-style)

## Code of Conduct

This project adheres to a code of conduct. By participating, you are expected to uphold this code. Please be respectful and constructive in your interactions.

## Getting Started

### Prerequisites

- Rust 1.70 or higher ([Install Rust](https://rustup.rs/))
- Node.js 18 or higher ([Install Node.js](https://nodejs.org/)) - for commit validation
- cargo-make (optional but recommended): `cargo install --force cargo-make`

### Setup

1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/tmpltool.git
   cd tmpltool
   ```
3. Add the upstream repository:
   ```bash
   git remote add upstream https://github.com/ORIGINAL_OWNER/tmpltool.git
   ```
4. Install Node.js dependencies (for commit validation):
   ```bash
   npm install
   ```
   This will:
   - Install commitlint and husky
   - Set up git hooks to validate commit messages
   - Prevent commits that don't follow conventional commit format
5. Create a new branch:
   ```bash
   git checkout -b feature/my-feature
   ```

## Development Workflow

### Quick Development Cycle

```bash
# Build and test
cargo make dev

# Run all quality checks
cargo make qa

# Format code
cargo make format

# Run clippy
cargo make clippy

# Run tests
cargo make test

# Test examples
cargo make test-examples
```

### Making Changes

1. Make your changes in your feature branch
2. Add tests for your changes
3. Ensure all tests pass: `cargo make qa`
4. Commit your changes using [conventional commits](#commit-convention)
5. Push to your fork
6. Create a pull request

## Commit Convention

This project uses [Conventional Commits](https://www.conventionalcommits.org/) for automated versioning and changelog generation.

**Important:** Commit messages are automatically validated using commitlint. Invalid commits will be rejected before they're created.

### Commit Validation

When you try to commit, a git hook will automatically check your commit message format. If it doesn't follow the conventional commit format, you'll see an error like:

```
⧗   input: bad commit message
✖   type must be one of [feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert] [type-enum]
✖   found 1 problems, 0 warnings
husky - commit-msg hook exited with code 1 (error)
```

To fix this, make sure your commit message follows the format below.

### Commit Message Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

- **feat**: A new feature (triggers minor version bump)
- **fix**: A bug fix (triggers patch version bump)
- **docs**: Documentation only changes (triggers patch version bump)
- **style**: Code style changes (formatting, missing semi-colons, etc.)
- **refactor**: Code refactoring (triggers patch version bump)
- **perf**: Performance improvements (triggers patch version bump)
- **test**: Adding or updating tests
- **build**: Changes to build system or dependencies (triggers patch version bump)
- **ci**: CI/CD configuration changes
- **chore**: Other changes that don't modify src or test files
- **revert**: Reverts a previous commit (triggers patch version bump)

### Breaking Changes

For breaking changes, add `BREAKING CHANGE:` in the commit body or add `!` after the type:

```
feat!: remove support for direct variable access

BREAKING CHANGE: Environment variables must now be accessed via get_env() function
```

This will trigger a **major** version bump.

### Examples

```bash
# Feature addition (minor version bump)
git commit -m "feat: add slugify filter support"

# Bug fix (patch version bump)
git commit -m "fix: correct multiline template rendering"

# Documentation (patch version bump)
git commit -m "docs: update README with new examples"

# Breaking change (major version bump)
git commit -m "feat!: change default output behavior

BREAKING CHANGE: Output now goes to stdout by default instead of file"

# Multiple changes
git commit -m "feat: add new template filters

- Add uppercase filter
- Add lowercase filter
- Add trim filter"
```

### Scope (Optional)

You can add a scope to provide additional context:

```bash
git commit -m "feat(cli): add --version flag"
git commit -m "fix(renderer): handle empty templates correctly"
git commit -m "test(fixtures): add more test cases"
```

## Pull Request Process

1. **Update tests**: Add tests for your changes
2. **Update documentation**: Update README.md if needed
3. **Run quality checks**: `cargo make qa`
4. **Commit with conventional commits**: Follow the commit convention
5. **Create PR**: Provide a clear description of your changes
6. **CI checks**: Ensure all CI checks pass
7. **Code review**: Address any review comments
8. **Merge**: Once approved, your PR will be merged

### PR Title

PR titles should also follow the conventional commit format:

```
feat: add support for custom filters
fix: resolve template parsing issue
docs: improve installation instructions
```

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_successful_rendering

# Run tests with output
cargo test -- --nocapture

# Run tests with cargo-make
cargo make test
cargo make test-verbose
```

### Test Organization

- All tests are in the `tests/` directory
- Each test is in its own file
- Test fixtures are in `tests/fixtures/`
- Use helper functions from `tests/common.rs`

### Adding Tests

See [tests/fixtures/README.md](tests/fixtures/README.md) for instructions on adding test fixtures.

Example test:

```rust
mod common;

use common::{cleanup_test_file, get_test_file_path, read_fixture_expected, read_fixture_template};
use std::fs;
use tmpltool::render_template;

#[test]
fn test_my_feature() {
    let output_path = get_test_file_path("output.txt");

    // Read template from fixtures
    let template_content = read_fixture_template("my_template.tmpl");
    let template_path = get_test_file_path("template.txt");
    fs::write(&template_path, template_content).unwrap();

    // Run the function
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

## Code Style

### Formatting

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt --check
```

### Linting

```bash
# Run clippy
cargo clippy

# Run clippy with all warnings
cargo clippy -- -W clippy::all
```

### Code Guidelines

- Follow Rust naming conventions
- Add documentation for public APIs
- Keep functions small and focused
- Write descriptive variable names
- Add comments for complex logic

## Release Process

Releases are automated using semantic-release:

1. Commit changes using conventional commits
2. Push to master/main branch
3. CI/CD automatically:
   - Analyzes commits
   - Determines version bump
   - Updates `Cargo.toml` and `CHANGELOG.md`
   - Builds binaries for all platforms
   - Creates GitHub release
   - Publishes Docker image

## Troubleshooting

### Commit Validation Not Working

If commit validation isn't working:

1. Make sure you ran `npm install` after cloning the repository
2. Check that `.husky/commit-msg` exists and is executable
3. Reinstall hooks: `npm run prepare`

### Bypassing Commit Validation (NOT Recommended)

In rare cases, you may need to bypass validation (e.g., fixing a broken commit history):

```bash
git commit --no-verify -m "your message"
```

**Warning:** Only use `--no-verify` when absolutely necessary, as it will bypass the commit validation.

### Testing Commit Messages

You can test a commit message without making a commit:

```bash
echo "feat: add new feature" | npx commitlint
```

## Questions?

If you have questions, please:
- Check existing issues
- Create a new issue for discussion
- Ask in pull request comments

Thank you for contributing! 🎉
