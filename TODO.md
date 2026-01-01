# TODO - tmpltool Enhancement Roadmap

This document outlines potential improvements, new features, and ideas for enhancing tmpltool. Items are organized by priority and category.

---

## MiniJinja Built-in Reference

Before implementing new functions/filters, note that MiniJinja already provides these built-ins that users can use directly:

### Built-in Filters (use via `|` pipe syntax)
| Filter | Description | Example |
|--------|-------------|---------|
| `abs` | Absolute value | `{{ -5\|abs }}` → `5` |
| `batch(n)` | Batch items into groups | `{{ items\|batch(3) }}` |
| `bool` | Convert to boolean | `{{ "yes"\|bool }}` |
| `capitalize` | Capitalize first letter | `{{ "hello"\|capitalize }}` |
| `default(val)` | Default if undefined | `{{ x\|default("N/A") }}` |
| `dictsort` | Sort dictionary | `{{ obj\|dictsort }}` |
| `escape` / `e` | HTML escape | `{{ html\|escape }}` |
| `first` | First element | `{{ arr\|first }}` |
| `float` | Convert to float | `{{ "3.14"\|float }}` |
| `groupby(attr)` | Group by attribute | `{{ users\|groupby("role") }}` |
| `indent(n)` | Indent text | `{{ text\|indent(4) }}` |
| `int` | Convert to integer | `{{ "42"\|int }}` |
| `items` | Get dict items | `{{ obj\|items }}` |
| `join(sep)` | Join array | `{{ arr\|join(", ") }}` |
| `last` | Last element | `{{ arr\|last }}` |
| `length` | Get length | `{{ arr\|length }}` |
| `list` | Convert to list | `{{ iter\|list }}` |
| `lower` | Lowercase | `{{ "HELLO"\|lower }}` |
| `map(attr)` | Extract attribute | `{{ users\|map(attribute="name") }}` |
| `max` | Maximum value | `{{ nums\|max }}` |
| `min` | Minimum value | `{{ nums\|min }}` |
| `pprint` | Pretty print | `{{ obj\|pprint }}` |
| `reject(test)` | Filter out matching | `{{ nums\|reject("odd") }}` |
| `replace(old,new)` | Replace substring | `{{ s\|replace("a","b") }}` |
| `reverse` | Reverse order | `{{ arr\|reverse }}` |
| `round(n)` | Round number | `{{ 3.7\|round }}` |
| `select(test)` | Keep matching | `{{ nums\|select("even") }}` |
| `slice(n)` | Slice into n parts | `{{ arr\|slice(3) }}` |
| `sort` | Sort items | `{{ arr\|sort }}` |
| `split(sep)` | Split string | `{{ s\|split(",") }}` |
| `string` | Convert to string | `{{ num\|string }}` |
| `sum` | Sum values | `{{ nums\|sum }}` |
| `title` | Title Case | `{{ "hello world"\|title }}` |
| `tojson` | Convert to JSON | `{{ obj\|tojson }}` |
| `trim` | Trim whitespace | `{{ s\|trim }}` |
| `unique` | Remove duplicates | `{{ arr\|unique }}` |
| `upper` | Uppercase | `{{ "hello"\|upper }}` |
| `urlencode` | URL encode | `{{ url\|urlencode }}` |

### Built-in Functions
| Function | Description | Example |
|----------|-------------|---------|
| `range(n)` | Generate sequence | `{% for i in range(5) %}` |
| `range(start,end)` | Range with bounds | `{% for i in range(1,10) %}` |
| `range(start,end,step)` | Range with step | `{% for i in range(0,10,2) %}` |
| `dict(...)` | Create dictionary | `{{ dict(a=1, b=2) }}` |
| `debug(...)` | Debug output | `{{ debug() }}` |

### Built-in Tests (use with `is` keyword)
- `defined`, `undefined`, `none`, `true`, `false`
- `odd`, `even`, `divisibleby(n)`
- `eq`, `ne`, `lt`, `le`, `gt`, `ge`
- `in`, `iterable`, `mapping`, `sequence`, `string`, `number`

---

## High Priority

### New Functions

#### String Manipulation
- [x] `regex_replace(string, pattern, replacement)` - Replace with regex support (MiniJinja's replace is literal only)
- [x] `regex_match(string, pattern)` - Check if string matches regex pattern
- [x] `regex_find_all(string, pattern)` - Find all regex matches
- [x] `substring(string, start, length)` - Extract substring by position
- [x] `contains(string, substring)` - Check if string contains substring
- [x] `index_of(string, substring)` - Find position of substring (-1 if not found)
- [x] `count_occurrences(string, substring)` - Count substring occurrences
- [x] `truncate(string, length, suffix)` - Truncate with ellipsis or custom suffix
- [x] `word_count(string)` - Count words in string
- [x] `split_lines(string)` - Split into lines array (MiniJinja's `lines` returns iterator)

#### Array Operations
- [x] `array_take(array, n)` - Take first N elements (simpler than slice)
- [x] `array_drop(array, n)` - Skip first N elements
- [x] `array_index_of(array, value)` - Find element index (-1 if not found)
- [x] `array_find(array, key, value)` - Find first matching object in array
- [x] `array_filter_by(array, key, op, value)` - Filter with operators (eq, ne, gt, lt, gte, lte, contains)
- [x] `array_pluck(array, key)` - Extract nested key (supports dot notation like "user.name")

#### Set Operations
- [x] `array_intersection(array1, array2)` - Common elements between arrays
- [x] `array_difference(array1, array2)` - Elements in array1 not in array2
- [x] `array_union(array1, array2)` - Combined unique elements
- [x] `array_symmetric_difference(array1, array2)` - Elements in either but not both

### New Filters

- [x] `wrap(width, indent)` - Word wrap text at specified width
- [x] `center(width, char)` - Center text with padding
- [x] `sentence_case` - Convert to Sentence case (first letter of first word only)
- [x] `strip_html` - Remove HTML tags from string
- [x] `strip_ansi` - Remove ANSI escape codes
- [x] `normalize_whitespace` - Collapse multiple spaces/newlines to single space
- [x] `to_constant_case` - Convert to CONSTANT_CASE (uppercase snake)
- [x] `pluralize(count, singular, plural)` - Pluralize based on count

---

## Medium Priority

### Data Processing

#### JSON/Object Operations
- [x] `json_path(object, path)` - JSONPath query support (e.g., `$.users[*].name`)
- [x] `object_pick(object, keys)` - Create new object with only specified keys
- [x] `object_omit(object, keys)` - Create new object without specified keys
- [x] `object_rename_keys(object, mapping)` - Rename object keys
- [x] `object_flatten(object, delimiter)` - Flatten nested object to dot notation
- [x] `object_unflatten(object, delimiter)` - Unflatten dot notation to nested object

#### Type Checking & Conversion
- [ ] `to_array(value)` - Wrap value in array if not already
- [ ] `is_array(value)` - Check if value is array
- [ ] `is_object(value)` - Check if value is object/mapping
- [ ] `is_string(value)` - Check if value is string
- [ ] `is_number(value)` - Check if value is number
- [ ] `is_null(value)` - Check if value is null/none
- [ ] `is_empty(value)` - Check if value is empty ([], {}, "", null)

#### Validation Functions
- [ ] `is_numeric(string)` - Check if string is numeric
- [ ] `is_alphanumeric(string)` - Check if string is alphanumeric
- [ ] `is_json(string)` - Check if string is valid JSON
- [ ] `is_yaml(string)` - Check if string is valid YAML
- [ ] `is_base64(string)` - Check if string is valid base64
- [ ] `is_hex(string)` - Check if string is valid hex
- [ ] `is_semver(string)` - Validate semantic version format
- [ ] `is_cron(string)` - Validate cron expression format
- [ ] `is_cidr(string)` - Validate CIDR notation
- [ ] `is_mac_address(string)` - Validate MAC address format
- [ ] `is_hostname(string)` - Validate hostname format
- [ ] `is_port(value)` - Validate port number (1-65535)

### Date/Time Enhancements

- [ ] `date_format_relative(timestamp)` - "2 hours ago", "in 3 days"
- [ ] `date_start_of(timestamp, unit)` - Start of day/week/month/year
- [ ] `date_end_of(timestamp, unit)` - End of day/week/month/year
- [ ] `days_in_month(year, month)` - Get days in a month
- [ ] `week_of_year(timestamp)` - Get ISO week number
- [ ] `day_of_year(timestamp)` - Get day of year (1-366)
- [ ] `day_of_week(timestamp)` - Get day of week (0-6 or name)
- [ ] `is_weekend(timestamp)` - Check if date is weekend
- [ ] `is_weekday(timestamp)` - Check if date is weekday
- [ ] `business_days_between(date1, date2)` - Count business days
- [ ] `add_business_days(timestamp, days)` - Add business days

### Network & System

- [x] `cidr_contains(cidr, ip)` - Check if IP is in CIDR range
- [x] `cidr_network(cidr)` - Get network address from CIDR
- [x] `cidr_broadcast(cidr)` - Get broadcast address from CIDR
- [x] `cidr_netmask(cidr)` - Get netmask from CIDR
- [x] `ip_to_int(ip)` - Convert IP to integer
- [x] `int_to_ip(int)` - Convert integer to IP
- [x] `get_os()` - Get operating system name
- [x] `get_arch()` - Get CPU architecture
- [x] `get_cwd()` - Get current working directory

## Low Priority / Future Ideas

### DevOps & Cloud Integrations

#### Helm/Kubernetes Extended
- [x] `helm_tpl(template, values)` - Helm-style templating
- [x] `k8s_annotation_safe(string)` - Sanitize for annotation values
- [x] `k8s_quantity_to_bytes(quantity)` - Convert K8s quantity (1Gi) to bytes
- [x] `k8s_bytes_to_quantity(bytes)` - Convert bytes to K8s quantity
- [x] `k8s_selector(labels)` - Generate label selector string
- [x] `k8s_pod_affinity(key, operator, values)` - Generate pod affinity YAML
- [x] `k8s_toleration(key, operator, value, effect)` - Generate toleration YAML
- [x] `k8s_probe(type, path, port, options)` - Generate liveness/readiness probe

#### Docker Helpers
- [ ] `docker_image_tag(registry, repo, tag)` - Build Docker image reference
- [ ] `docker_port_mapping(container, host, protocol)` - Generate port mapping

### Template Features

#### Control Flow Helpers
- [ ] `repeat(value, count)` - Repeat value N times (returns array)
- [ ] `enumerate(array)` - Return array of [index, value] pairs
- [ ] `zip_longest(...arrays, default)` - Zip arrays with default for shorter ones

#### Debugging & Development
- [ ] `timer_start(name)` / `timer_end(name)` - Measure rendering time
- [ ] `template_vars()` - List all available variables in scope
- [ ] `env_dump()` - Dump all environment variables (debug mode only)

#### Caching
- [ ] `cache(key, ttl, expression)` - Cache expensive computations
- [ ] `memoize(function, args)` - Memoize function results

### CLI Enhancements

- [ ] `--env-file <FILE>` - Load environment variables from .env file
- [ ] `--set KEY=VALUE` - Set variables from command line
- [ ] `--strict` - Fail on undefined variables (instead of empty string)
- [ ] `--dry-run` - Validate template without writing output
- [ ] `--format <FORMAT>` - Auto-format output (json, yaml, toml with pretty-printing)
- [ ] `--watch` - Watch template file and re-render on changes
- [ ] `--diff` - Show diff when output file exists
- [ ] `--backup` - Create backup before overwriting output file
- [ ] `--template-dir <DIR>` - Set base directory for template includes
- [ ] `--data <FILE>` - Load variables from JSON/YAML file
- [ ] `--quiet` - Suppress non-error output
- [ ] `--verbose` - Enable verbose logging
- [ ] `--version-check` - Check for newer versions

### Output Validation

- [ ] `--validate jsonschema=<SCHEMA_FILE>` - Validate against JSON Schema
- [ ] `--validate xmlschema=<XSD_FILE>` - Validate against XML Schema
- [ ] `--validate custom=<SCRIPT>` - Run custom validation script
- [ ] Inline validation directives in templates

### Performance Optimizations

- [ ] Template pre-compilation and caching
- [ ] Parallel template rendering for multiple outputs
- [ ] Lazy evaluation for expensive operations
- [ ] Memory-mapped file reading for large files
- [ ] Streaming output for very large templates
- [ ] Template syntax validation without full rendering

---

## Documentation Improvements

- [ ] Add performance benchmarks vs similar tools (gomplate, envsubst, etc.)
- [ ] Create migration guides from other templating tools
- [ ] Add troubleshooting guide for common errors
- [ ] Create video tutorials for common use cases
- [ ] Document all MiniJinja built-in filters/functions available
- [ ] Add more real-world example templates:
  - [ ] Nginx configuration
  - [ ] Apache configuration
  - [ ] Prometheus configuration
  - [ ] Grafana dashboard JSON
  - [ ] GitHub Actions workflow
  - [ ] GitLab CI configuration
  - [ ] Terraform variables
  - [ ] Ansible inventory
  - [ ] Systemd service files
- [ ] Interactive playground/REPL for testing templates
- [ ] Function quick-reference card (PDF)

---

## Testing Improvements

- [ ] Add property-based testing (proptest/quickcheck)
- [ ] Add fuzzing tests for parser edge cases
- [ ] Add mutation testing to verify test quality
- [ ] Add performance regression tests
- [ ] Add memory leak tests
- [ ] Increase code coverage to 90%+
- [ ] Add stress tests for large templates/files
- [ ] Add concurrent access tests

---

## Code Quality & Maintenance

- [ ] Refactor large modules (filesystem.rs: 666 lines, filters/string.rs: 491 lines)
- [ ] Add more inline documentation with examples
- [ ] Standardize error messages across all functions
- [ ] Add telemetry/metrics collection (opt-in)
- [ ] Create architecture decision records (ADRs)
- [ ] Add SECURITY.md with security policy
- [ ] Set up automated dependency updates (Dependabot/Renovate)
- [ ] Add SBOM (Software Bill of Materials) generation

---

## Platform & Distribution

- [ ] Publish to crates.io for `cargo install tmpltool`
- [ ] Create Homebrew formula for macOS
- [x] Create APT/DEB package for Debian/Ubuntu
- [ ] Create RPM package for Fedora/RHEL
- [ ] Create Chocolatey package for Windows
- [ ] Create Nix package
- [ ] Create Alpine APK package
- [ ] Provide WebAssembly build for browser usage
- [ ] Create VS Code extension for template editing
- [ ] Create JetBrains plugin for template editing

---

## Community & Ecosystem

- [ ] Create plugin/extension system for custom functions
- [ ] Allow loading custom functions from shared libraries
- [ ] Create template library/registry for sharing templates
- [ ] Add GitHub Discussions for community support
- [ ] Create contributing guide with code style guidelines
- [ ] Set up Discord/Slack community
- [ ] Add "Powered by tmpltool" badge for projects

---

## Known Issues to Fix

- [ ] Review and improve error messages for better user experience
- [ ] Ensure consistent behavior across all platforms (Windows path handling)
- [ ] Add better handling for binary files in `read_file`
- [ ] Improve performance of `glob` function for large directories
- [ ] Add timeout options for `exec` and `exec_raw` functions
- [ ] Handle Unicode edge cases in string functions
- [ ] Improve memory usage for very large templates

---

## Notes

### MiniJinja Compatibility
- tmpltool extends MiniJinja with custom functions and filters
- All MiniJinja built-in filters and functions are available
- Custom functions should complement, not duplicate, MiniJinja features
- When in doubt, check MiniJinja docs: https://docs.rs/minijinja/latest/minijinja/

### Prioritization Criteria
- **High Priority**: Features frequently requested or blocking common use cases
- **Medium Priority**: Nice-to-have features that improve usability
- **Low Priority**: Advanced features for specialized use cases

### Implementation Guidelines
- All new functions should follow existing patterns in CLAUDE.md
- Write comprehensive unit tests for all new functions
- Add integration tests for binary verification
- Update README.md with documentation and examples
- Keep backward compatibility - no breaking changes without major version bump

### Security Considerations
- All file operations must respect trust mode
- Cryptographic functions should use established libraries
- Command execution must be sandboxed appropriately
- Never expose sensitive data in error messages

---

*Last updated: 2026-01-01*
