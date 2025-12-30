## Description

<!-- Provide a brief description of your changes -->

## Type of Change

<!-- Mark the relevant option with an "x" -->

- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update
- [ ] Code refactoring
- [ ] Performance improvement
- [ ] Test improvement
- [ ] Build/CI improvement
- [ ] Other (please describe):

## Related Issues

<!-- Link to related issues. Use "Closes #123" to auto-close issues when merged -->

Closes #
Related to #

## Changes Made

<!-- Provide a detailed list of changes -->

-
-
-

## Testing

<!-- Describe the tests you ran and how to reproduce them -->

**Test environment:**
- OS: [e.g., Ubuntu 22.04, macOS 14.0, Windows 11]
- Rust version: [e.g., 1.75.0]

**How to test:**

1.
2.
3.

**Test results:**

```bash
# Paste relevant test output
cargo test
```

## Examples

<!-- If applicable, provide examples of the changes -->

**Before:**
```bash
# Show the old behavior
```

**After:**
```bash
# Show the new behavior
```

**Template example (if applicable):**
```
# Show template usage
{{ new_function(arg="value") }}
```

## Checklist

<!-- Mark completed items with an "x" -->

### Code Quality

- [ ] My code follows the project's style guidelines
- [ ] I have run `cargo fmt` to format my code
- [ ] I have run `cargo clippy` and addressed all warnings
- [ ] I have run `cargo make qa` and all checks pass
- [ ] My changes generate no new warnings or errors

### Testing

- [ ] I have added tests that prove my fix is effective or that my feature works
- [ ] New and existing unit tests pass locally with my changes
- [ ] I have tested my changes manually
- [ ] I have tested on multiple platforms (if applicable)

### Documentation

- [ ] I have updated the README.md (if applicable)
- [ ] I have added/updated code comments for complex logic
- [ ] I have added/updated examples (if applicable)
- [ ] I have updated TODO.md to mark completed features (if applicable)

### Commits

- [ ] My commits follow the [Conventional Commits](https://www.conventionalcommits.org/) format
  - `feat:` for new features
  - `fix:` for bug fixes
  - `docs:` for documentation
  - `refactor:` for refactoring
  - `test:` for tests
  - `chore:` for maintenance
- [ ] Each commit has a clear and descriptive message
- [ ] I have squashed unnecessary commits (if applicable)

### Breaking Changes

<!-- If this PR includes breaking changes, fill out this section -->

- [ ] I have marked this PR with breaking changes
- [ ] I have documented the breaking changes in the commit message with `BREAKING CHANGE:`
- [ ] I have updated the migration guide (if applicable)

**Breaking changes description (if applicable):**
```
# Describe what breaks and how users should migrate
```

## Additional Notes

<!-- Add any additional notes, concerns, or questions for reviewers -->

## Screenshots (if applicable)

<!-- Add screenshots to help explain your changes -->

## Performance Impact

<!-- If applicable, describe any performance implications -->

- [ ] No performance impact
- [ ] Performance improved
- [ ] Performance may be affected (explain below)

**Performance notes:**
```
# Describe performance impact and any benchmarks
```

## Security Considerations

<!-- If applicable, describe security implications -->

- [ ] No security impact
- [ ] Security improved
- [ ] Security implications (explain below)

**Security notes:**
```
# Describe security implications
```
