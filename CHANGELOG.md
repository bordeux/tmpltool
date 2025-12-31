## 1.0.0 (2025-12-31)


### Features

* initial release of tmpltool v0.1.0 ([29d3910](https://github.com/bordeux/tmpltool/commit/29d3910d494470920550cd35cd98e938f6e5f0d6))

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-12-31

### Added
- Initial release of tmpltool
- Environment variable access with `get_env()` and `filter_env()`
- Hash & crypto functions (MD5, SHA1, SHA256, SHA512, UUID, random strings)
- Filesystem operations (read, exists, list, glob, size, modified)
- Data parsing for JSON, YAML, and TOML
- Validation functions (email, URL, IP, UUID, regex)
- Template include functionality with security controls
- Trust mode for advanced filesystem access
- Full Jinja2 syntax support via MiniJinja
- Comprehensive test suite (267 tests)
- Docker support with multi-arch images
- CLI with flexible I/O (file or stdin/stdout)
- Prepare first public release
