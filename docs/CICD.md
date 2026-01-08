# CI/CD

This project uses GitHub Actions for continuous integration and automated releases.

## Continuous Integration

Every pull request and push to master triggers:
- Code formatting check (`rustfmt`)
- Linting (`clippy`)
- Multi-platform tests (Ubuntu, macOS, Windows)
- Code coverage (uploaded to Codecov)
- Example template testing

## Automated Releases

Releases use [semantic-release](https://github.com/semantic-release/semantic-release):
1. Analyzes commit messages
2. Determines next version
3. Updates `Cargo.toml`
4. Generates `CHANGELOG.md`
5. Builds multi-platform binaries
6. Creates GitHub release
7. Publishes Docker images to GHCR

## Commit Convention

Follow [Conventional Commits](https://www.conventionalcommits.org/):

- `feat: description` - New feature (minor bump: 1.2.0 → 1.3.0)
- `fix: description` - Bug fix (patch bump: 1.2.0 → 1.2.1)
- `feat!: description` - Breaking change (major bump: 1.2.0 → 2.0.0)
- `docs:`, `refactor:`, `perf:` - Other changes (patch bump)
- `style:`, `test:`, `chore:`, `ci:` - No version bump

**Examples:**
```bash
git commit -m "feat: add validation functions"
git commit -m "fix: correct path resolution"
git commit -m "feat!: change output behavior

BREAKING CHANGE: Output now defaults to stdout"
```
