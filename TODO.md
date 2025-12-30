# TODO - Feature Ideas & Improvements

This document tracks potential features, improvements, and ideas for tmpltool.

## Custom Functions

### File System Functions
- [ ] `read_file(path)` - Read content from a file into the template
- [ ] `file_exists(path)` - Check if a file exists (returns boolean)
- [ ] `list_dir(path)` - List files in a directory
- [ ] `glob(pattern)` - List files by pattern
- [ ] `file_size(path)` - Get file size in bytes
- [ ] `file_modified(path)` - Get file modification timestamp

### Data Parsing Functions
- [ ] `parse_json(string)` - Parse JSON string into object
- [ ] `parse_yaml(string)` - Parse YAML string into object
- [ ] `parse_toml(string)` - Parse TOML string into object
- [ ] `read_json_file(path)` - Read and parse JSON file
- [ ] `read_yaml_file(path)` - Read and parse YAML file
- [ ] `read_toml_file(path)` - Read and parse TOML file

### String Manipulation Functions
- [ ] `regex_match(pattern, string)` - Match regex pattern
- [ ] `regex_replace(pattern, replacement, string)` - Replace using regex
- [ ] `substring(string, start, end)` - Extract substring
- [ ] `pad_left(string, length, char)` - Pad string on left
- [ ] `pad_right(string, length, char)` - Pad string on right
- [ ] `repeat(string, count)` - Repeat string N times

### Encoding/Decoding Functions
- [ ] `base64_encode(string)` - Encode to base64
- [ ] `base64_decode(string)` - Decode from base64
- [ ] `hex_encode(string)` - Encode to hexadecimal
- [ ] `hex_decode(string)` - Decode from hexadecimal
- [ ] `url_encode(string)` - URL encode (currently available as filter)
- [ ] `url_decode(string)` - URL decode
- [ ] `json_escape(string)` - Escape string for JSON
- [ ] `html_escape(string)` - Escape HTML entities
- [ ] `html_unescape(string)` - Unescape HTML entities

### Hash/Crypto Functions
- [x] `md5(string)` - Calculate MD5 hash ✅ **Implemented v1.0.4**
- [x] `sha1(string)` - Calculate SHA1 hash ✅ **Implemented v1.0.4**
- [x] `sha256(string)` - Calculate SHA256 hash ✅ **Implemented v1.0.4**
- [x] `sha512(string)` - Calculate SHA512 hash ✅ **Implemented v1.0.4**
- [x] `uuid()` - Generate UUID v4 ✅ **Implemented v1.0.4**
- [x] `random_string(length, charset)` - Generate random string ✅ **Implemented v1.0.4**

### Date/Time Functions
- [ ] `format_date(timestamp, format)` - Format timestamp with custom format
- [ ] `parse_date(string, format)` - Parse date string
- [ ] `date_add(timestamp, duration)` - Add duration to timestamp
- [ ] `date_diff(timestamp1, timestamp2)` - Calculate difference between dates
- [ ] `timestamp()` - Get current Unix timestamp
- [ ] `iso8601()` - Get current time in ISO 8601 format (alias for now())

### Math Functions
- [ ] `abs(number)` - Absolute value
- [ ] `ceil(number)` - Round up
- [ ] `floor(number)` - Round down
- [ ] `round(number, decimals)` - Round to N decimals
- [ ] `min(array)` - Find minimum value
- [ ] `max(array)` - Find maximum value
- [ ] `sum(array)` - Sum array values
- [ ] `avg(array)` - Calculate average

### Network Functions
- [ ] `http_get(url)` - Fetch content from URL (consider security implications)
- [ ] `resolve_dns(hostname)` - Resolve DNS hostname to IP

### Shell/Process Functions
- [ ] `exec(command)` - Execute shell command and return output (consider security implications)
- [ ] `hostname()` - Get system hostname
- [ ] `username()` - Get current username
- [ ] `cwd()` - Get current working directory

### Validation Functions
- [ ] `is_email(string)` - Validate email format
- [ ] `is_url(string)` - Validate URL format
- [ ] `is_ip(string)` - Validate IP address
- [ ] `is_uuid(string)` - Validate UUID format
- [ ] `matches_regex(pattern, string)` - Check if string matches regex

## CLI Enhancements

### Input/Output Options
- [ ] Output directory: `tmpltool -i templates/ -o output/`
- [ ] In-place editing: `tmpltool -i template.tmpl --in-place`
- [ ] Batch processing with glob patterns: `tmpltool templates/*.tmpl`

### Variable Management
- [ ] Load variables from JSON file: `tmpltool --env-json vars.json template.tmpl`
- [ ] Load variables from YAML file: `tmpltool --env-yaml vars.yaml template.tmpl`
- [ ] Pass variables via CLI: `tmpltool --var key=value template.tmpl`
- [ ] Environment file support: `tmpltool --env-file .env template.tmpl`
- [ ] Variable precedence: CLI > env file > environment

### Development Features
- [ ] Watch mode: `tmpltool --watch template.tmpl` (auto-reload on changes)
- [ ] REPL mode: `tmpltool --repl` (interactive template testing)
- [ ] Dry-run mode: `tmpltool --dry-run` (validate without writing)
- [ ] Verbose mode: `tmpltool --verbose` (show debug information)
- [ ] Quiet mode: `tmpltool --quiet` (suppress all output except errors)
- [ ] Validate mode: `tmpltool --validate template.tmpl` (syntax check only)

### Output Formatting
- [ ] Strip whitespace: `tmpltool --strip-whitespace template.tmpl`

### Error Handling
- [ ] Strict mode: `tmpltool --strict` (fail on undefined variables)
- [ ] Ignore errors: `tmpltool --ignore-errors` (continue on errors)
- [ ] JSON error output: `tmpltool --error-format json` (for tooling integration)

## Template Features

### Template Organization
- [ ] Template inheritance support (extend/block)
- [ ] Template includes from filesystem
- [ ] Template includes from URLs
- [ ] Macro library support
- [ ] Partial templates

### Configuration
- [ ] Custom delimiters: `tmpltool --delimiters '<<' '>>'`
- [ ] Configuration file: `.tmpltool.toml` or `.tmpltool.yaml`
- [ ] Per-project configuration
- [ ] Global configuration in `~/.config/tmpltool/config.toml`

### Template Functions
- [ ] Custom function plugins (dynamic loading)
- [ ] JavaScript-based custom functions (via embedded runtime)
- [ ] Lua-based custom functions (via embedded runtime)

## Quality of Life Improvements

### Documentation
- [ ] Man page: `man tmpltool`
- [ ] Built-in help for functions: `tmpltool --list-functions`
- [ ] Function documentation: `tmpltool --doc filter_env`
- [ ] Example templates library
- [ ] Interactive tutorial

### Shell Integration
- [ ] Shell completion (bash, zsh, fish)
- [ ] Environment variable completion
- [ ] Template file completion

### Performance
- [ ] Template caching for repeated renders
- [ ] Parallel processing for multiple files
- [ ] Lazy evaluation for complex expressions
- [ ] Memory-mapped file reading for large files

### Security
- [ ] Sandbox mode (disable file system access, exec, network)
- [ ] Allowlist for allowed functions
- [ ] Security audit mode (report potentially dangerous operations)
- [ ] Secrets filtering (avoid logging sensitive env vars)

## Testing & Quality

### Testing Tools
- [ ] Template test runner: `tmpltool test tests/`
- [ ] Snapshot testing support
- [ ] Coverage reporting for templates
- [ ] Benchmark mode for performance testing

### Linting
- [ ] Template linter (check for common issues)
- [ ] Style guide enforcement
- [ ] Unused variable detection
- [ ] Cyclomatic complexity warnings

## Distribution & Packaging

### Package Managers
- [ ] Homebrew formula (macOS/Linux)
- [ ] APT repository (Debian/Ubuntu)
- [ ] RPM repository (RedHat/Fedora)
- [ ] Chocolatey package (Windows)
- [ ] Scoop package (Windows)
- [ ] AUR package (Arch Linux)

### Installation
- [ ] Single-binary installer script
- [ ] Docker image on Docker Hub
- [ ] Snap package
- [ ] Flatpak package

## Integration & Ecosystem

### CI/CD Integration
- [ ] GitHub Actions integration
- [ ] GitLab CI templates
- [ ] Jenkins plugin
- [ ] CircleCI orb

### Editor Integration
- [ ] VSCode extension (syntax highlighting, snippets)
- [ ] Vim plugin
- [ ] Emacs mode
- [ ] Language Server Protocol (LSP) server

### Tools Integration
- [ ] Terraform integration (template provider)
- [ ] Kubernetes integration (ConfigMap/Secret generation)
- [ ] Ansible integration (template module)
- [ ] Docker Compose integration

## Advanced Features

### Data Sources
- [ ] Database query support (PostgreSQL, MySQL, SQLite)
- [ ] Redis data fetching
- [ ] S3 object fetching
- [ ] Consul KV store integration
- [ ] Vault secrets integration

### Output Formats
- [ ] Generate multiple outputs from one template
- [ ] Split output into multiple files
- [ ] Archive output (tar, zip)
- [ ] Stream output to remote destinations

### Templating Enhancements
- [ ] Conditional includes based on environment
- [ ] Dynamic template loading
- [ ] Template composition (merge multiple templates)
- [ ] Template inheritance chains

## Ideas for Future Major Versions

### Version 2.0
- [ ] Plugin system with hot-reload
- [ ] Built-in template registry/marketplace
- [ ] Cloud-based template sharing
- [ ] Web UI for template development
- [ ] REST API server mode

### Template Language Extensions
- [ ] TypeScript-like type hints for templates
- [ ] Template compilation to standalone binaries
- [ ] Template optimization/minification
- [ ] Template debugging tools

## Community & Contribution

- [ ] Contributing guidelines (CONTRIBUTING.md)
- [ ] Code of conduct
- [ ] Issue templates (bug report, feature request)
- [ ] PR template
- [ ] Roadmap document
- [ ] Changelog automation (already implemented with semantic-release)
- [ ] Community templates repository

## Questions to Consider

1. Should we support multiple template engines (Tera, Handlebars, Liquid)?
2. Should we add a server mode (HTTP API for rendering)?
3. Should we support template compilation for better performance?
4. Should we add a GUI for non-technical users?
5. How to balance features vs. simplicity?
6. What's the security model for file/network/exec access?
7. Should we support plugins written in other languages (Python, JavaScript)?

---

**Note**: This is a living document. Ideas should be evaluated based on:
- User demand and use cases
- Maintenance burden
- Security implications
- Performance impact
- Alignment with tool's philosophy (simple, fast, secure)

**Priority Legend** (to be added as we prioritize):
- 🔥 High priority
- ⭐ Nice to have
- 💡 Needs discussion
- ⚠️ Security/complexity concerns
