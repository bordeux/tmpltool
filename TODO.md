# TODO - Feature Ideas for tmpltool

This document contains ideas for new functions and features to make tmpltool more useful for configuration file templating.

## Current Functions Summary

### ✅ Environment & Context
- [x] `get_env(name, default)` - Get environment variable
- [x] `filter_env(pattern)` - Filter environment variables by glob pattern
- [x] `now()` - Get current Unix timestamp
- [x] `get_random(start, end)` - Generate random integer

### ✅ Cryptography & Hashing
- [x] `md5(string)` - MD5 hash
- [x] `sha1(string)` - SHA1 hash
- [x] `sha256(string)` - SHA256 hash
- [x] `sha512(string)` - SHA512 hash
- [x] `uuid()` - Generate UUID v4
- [x] `random_string(length, charset)` - Generate random string

### ✅ Encoding & Security
- [x] `base64_encode(string)` - Base64 encode
- [x] `base64_decode(string)` - Base64 decode
- [x] `hex_encode(string)` - Hexadecimal encode
- [x] `hex_decode(string)` - Hexadecimal decode
- [x] `bcrypt(password, rounds)` - Bcrypt hash (for password storage)
- [x] `generate_secret(length, charset)` - Generate cryptographically secure random string
- [x] `hmac_sha256(key, message)` - HMAC-SHA256 signature
- [x] `escape_html(string)` - Escape HTML entities
- [x] `escape_xml(string)` - Escape XML entities
- [x] `escape_shell(string)` - Escape shell command arguments

### ✅ Filesystem Operations
- [x] `read_file(path)` - Read file content
- [x] `file_exists(path)` - Check file existence
- [x] `list_dir(path)` - List directory contents
- [x] `glob(pattern)` - Find files by glob pattern
- [x] `file_size(path)` - Get file size
- [x] `file_modified(path)` - Get file modification time
- [x] `basename(path)` - Get filename from path
- [x] `dirname(path)` - Get directory from path
- [x] `file_extension(path)` - Get file extension
- [x] `join_path(parts)` - Join path components
- [x] `normalize_path(path)` - Normalize path
- [x] `is_file(path)` - Check if path is a file
- [x] `is_dir(path)` - Check if path is a directory
- [x] `is_symlink(path)` - Check if path is a symlink
- [x] `read_lines(path, max_lines)` - Read first N lines from file

### ✅ Data Parsing
- [x] `parse_json(string)` - Parse JSON string
- [x] `parse_yaml(string)` - Parse YAML string
- [x] `parse_toml(string)` - Parse TOML string
- [x] `read_json_file(path)` - Read and parse JSON file
- [x] `read_yaml_file(path)` - Read and parse YAML file
- [x] `read_toml_file(path)` - Read and parse TOML file

### ✅ Data Serialization
- [x] `to_json(object, pretty)` - Convert object to JSON string
- [x] `to_yaml(object)` - Convert object to YAML string
- [x] `to_toml(object)` - Convert object to TOML string

### ✅ Object Manipulation
- [x] `object_merge(obj1, obj2)` - Deep merge two objects
- [x] `object_get(object, path)` - Get nested value by path
- [x] `object_set(object, path, value)` - Set nested value by path
- [x] `object_keys(object)` - Get object keys as array
- [x] `object_values(object)` - Get object values as array
- [x] `object_has_key(object, key)` - Check if object has key

### ✅ Validation
- [x] `is_email(string)` - Validate email format
- [x] `is_url(string)` - Validate URL format
- [x] `is_ip(string)` - Validate IP address (IPv4/IPv6)
- [x] `is_uuid(string)` - Validate UUID format
- [x] `matches_regex(pattern, string)` - Regex pattern matching

### ✅ Debugging & Development
- [x] `debug(value)` - Print value to stderr and return it
- [x] `type_of(value)` - Get type of value
- [x] `inspect(value)` - Pretty-print value structure
- [x] `assert(condition, message)` - Assert condition or fail
- [x] `warn(message)` - Print warning to stderr
- [x] `abort(message)` - Abort rendering with error

### ✅ Filters
- [x] `slugify` - Convert string to URL-friendly slug
- [x] `urlencode` - URL encode string
- [x] `filesizeformat` - Format bytes to human-readable size

---

## 📋 Proposed New Features

### ✅ Network & System Functions
*Useful for nginx, apache, docker, kubernetes configs*

- [x] `get_hostname()` - Get system hostname
- [x] `get_ip_address(interface)` - Get IP address of network interface (optional interface parameter)
- [x] `resolve_dns(hostname)` - Resolve hostname to IP address
- [x] `is_port_available(port)` - Check if port is available
- [x] `get_username()` - Get current system username
- [x] `get_home_dir()` - Get user's home directory
- [x] `get_temp_dir()` - Get system temporary directory

### 🔢 Math & Calculation Functions
*Useful for resource calculations, sizing configs*

- [ ] `min(a, b)` - Return minimum value
- [ ] `max(a, b)` - Return maximum value
- [ ] `abs(number)` - Absolute value
- [ ] `round(number, decimals)` - Round to N decimal places
- [ ] `ceil(number)` - Round up
- [ ] `floor(number)` - Round down
- [ ] `percentage(value, total)` - Calculate percentage
- [ ] `bytes_to_mb(bytes)` - Convert bytes to megabytes
- [ ] `mb_to_bytes(mb)` - Convert megabytes to bytes

### ✅ String Manipulation Functions (Filters)
*Extended string operations for config generation*

- [x] `indent(spaces)` - Indent text by N spaces
- [x] `dedent` - Remove common leading whitespace
- [x] `quote(style)` - Quote string (single/double/backtick)
- [x] `escape_quotes` - Escape quotes in string
- [x] `to_snake_case` - Convert to snake_case
- [x] `to_camel_case` - Convert to camelCase
- [x] `to_pascal_case` - Convert to PascalCase
- [x] `to_kebab_case` - Convert to kebab-case
- [x] `pad_left(length, char)` - Pad string on left
- [x] `pad_right(length, char)` - Pad string on right
- [x] `repeat(count)` - Repeat string N times
- [x] `reverse` - Reverse string

**Note:** These are implemented as filters (e.g., `{{ "text" | indent(2) }}`), not functions.

### ✅ Date & Time Functions
*Enhanced datetime handling for logs, timestamps*

- [x] `format_date(timestamp, format)` - Format Unix timestamp
- [x] `parse_date(string, format)` - Parse date string to timestamp
- [x] `date_add(timestamp, days)` - Add days to timestamp
- [x] `date_diff(timestamp1, timestamp2)` - Difference in days
- [x] `get_year(timestamp)` - Extract year
- [x] `get_month(timestamp)` - Extract month
- [x] `get_day(timestamp)` - Extract day
- [x] `get_hour(timestamp)` - Extract hour
- [x] `get_minute(timestamp)` - Extract minute
- [x] `timezone_convert(timestamp, from_tz, to_tz)` - Convert timezones
- [x] `is_leap_year(year)` - Check if leap year

### ✅ Security & Encoding Functions
*Additional security utilities*

- [x] `base64_encode(string)` - Base64 encode
- [x] `base64_decode(string)` - Base64 decode
- [x] `hex_encode(string)` - Hexadecimal encode
- [x] `hex_decode(string)` - Hexadecimal decode
- [x] `bcrypt(password, rounds)` - Bcrypt hash (for password storage)
- [x] `generate_secret(length, charset)` - Generate cryptographically secure random string
- [x] `hmac_sha256(key, message)` - HMAC-SHA256
- [x] `escape_html(string)` - Escape HTML entities
- [x] `escape_xml(string)` - Escape XML entities
- [x] `escape_shell(string)` - Escape shell command arguments

### ✅ Advanced Filesystem Functions
*Extended filesystem operations*

- [x] `basename(path)` - Get filename from path
- [x] `dirname(path)` - Get directory from path
- [x] `file_extension(path)` - Get file extension
- [x] `join_path(parts)` - Join path components
- [x] `normalize_path(path)` - Normalize path (resolve .., .)
- [x] `is_file(path)` - Check if path is a file
- [x] `is_dir(path)` - Check if path is a directory
- [x] `is_symlink(path)` - Check if path is a symlink
- [x] `read_lines(path, max_lines)` - Read first N lines from file

### 📊 Data Transformation Functions
*Advanced data manipulation*

**Serialization:**
- [x] `to_json(object, pretty)` - Convert object to JSON string
- [x] `to_yaml(object)` - Convert object to YAML string
- [x] `to_toml(object)` - Convert object to TOML string

**Object Functions:**
- [x] `object_merge(obj1, obj2)` - Deep merge two objects
- [x] `object_get(object, path)` - Get nested value by path (e.g., "a.b.c")
- [x] `object_set(object, path, value)` - Set nested value by path
- [x] `object_keys(object)` - Get object keys as array
- [x] `object_values(object)` - Get object values as array
- [x] `object_has_key(object, key)` - Check if object has key

**Array Functions:**
- [ ] `array_sort_by(array, key)` - Sort array by object key
- [ ] `array_group_by(array, key)` - Group array items by key
- [ ] `array_unique(array)` - Remove duplicates from array
- [ ] `array_flatten(array)` - Flatten nested arrays

### 🌍 Internationalization & Localization
*i18n support for multi-language configs*

- [ ] `translate(key, locale)` - Translate string
- [ ] `format_number(number, locale)` - Locale-aware number formatting
- [ ] `format_currency(amount, currency, locale)` - Format currency
- [ ] `pluralize(count, singular, plural)` - Pluralize based on count

### 🔍 Conditional & Logic Functions
*Enhanced conditional logic*

**General Logic:**
- [ ] `default(value, default)` - Return default if value is falsy
- [ ] `coalesce(values...)` - Return first non-null value
- [ ] `ternary(condition, true_val, false_val)` - Ternary operator
- [ ] `in_range(value, min, max)` - Check if value in range

**Array Predicates:**
- [x] `array_any(array, predicate)` - Check if any item matches
- [x] `array_all(array, predicate)` - Check if all items match
- [x] `array_contains(array, value)` - Check if array contains value

**String Predicates:**
- [x] `starts_with(string, prefix)` - Check string starts with prefix
- [x] `ends_with(string, suffix)` - Check string ends with suffix

### 🐳 Container & Orchestration Helpers
*Specific for Docker, Kubernetes, docker-compose*

- [ ] `docker_image_tag(image, tag)` - Format Docker image with tag
- [ ] `k8s_label_safe(string)` - Convert to Kubernetes-safe label
- [ ] `dns_label_safe(string)` - Convert to DNS-safe label (max 63 chars)
- [ ] `resource_request(cpu, memory)` - Format k8s resource request
- [ ] `env_var_ref(var_name)` - Format environment variable reference
- [ ] `secret_ref(secret_name, key)` - Format secret reference
- [ ] `configmap_ref(cm_name, key)` - Format ConfigMap reference

### 🌐 Web & API Helpers
*For nginx, apache, API configs*

- [ ] `basic_auth(username, password)` - Generate basic auth header
- [ ] `jwt_decode(token)` - Decode JWT token (header and payload only)
- [ ] `parse_url(url)` - Parse URL into components
- [ ] `build_url(scheme, host, port, path)` - Build URL from components
- [ ] `query_string(params)` - Build URL query string from object
- [ ] `mime_type(filename)` - Guess MIME type from filename
- [ ] `http_status_text(code)` - Get HTTP status text from code

### ✅ Debugging & Development Functions
*Helpful during template development*

- [x] `debug(value)` - Print value to stderr and return it
- [x] `type_of(value)` - Get type of value (string, number, array, etc.)
- [x] `inspect(value)` - Pretty-print value structure
- [x] `assert(condition, message)` - Assert condition or fail with message
- [x] `warn(message)` - Print warning to stderr
- [x] `abort(message)` - Abort rendering with error message

### ✅ Statistical & Array Functions
*For data processing and analysis*

**Statistical Functions:**
- [x] `array_sum(array)` - Sum of array values
- [x] `array_avg(array)` - Average of array values
- [x] `array_median(array)` - Median of array values
- [x] `array_min(array)` - Minimum value in array
- [x] `array_max(array)` - Maximum value in array

**Array Manipulation:**
- [x] `array_count(array)` - Count array items (alias for length)
- [x] `array_chunk(array, size)` - Split array into chunks
- [x] `array_zip(array1, array2)` - Combine two arrays into pairs

### 🎨 Template Composition
*Advanced templating features*

- [ ] `render_string(template_string, context)` - Render template from string
- [ ] `include_raw(path)` - Include file without rendering
- [ ] `include_once(path)` - Include file only once (prevent duplicates)

---

## 🎯 High Priority Features
*Most useful for common configuration scenarios*

### For Web Server Configs (Nginx, Apache)
1. `get_hostname()` - Get server hostname
2. `get_ip_address(interface)` - Get server IP
3. `base64_encode()` / `base64_decode()` - For basic auth
4. `escape_shell()` - For command escaping
5. `dns_label_safe()` - For domain name validation

### For Docker & Kubernetes
1. `k8s_label_safe()` - Kubernetes label formatting
2. `dns_label_safe()` - DNS-compliant names
3. `indent()` - YAML indentation
4. `base64_encode()` - For secrets
5. `resource_request()` - Format resource limits

### For Application Configs
1. `object_merge()` - Merge configuration objects
2. `object_get()` - Access nested config values
3. `default()` - Provide fallback values
4. `to_json()` / `to_yaml()` - Convert between formats
5. `coalesce()` - First non-null value

### For Database Configs
1. `escape_quotes()` - SQL string escaping
2. `format_number()` - Connection pool sizes
3. `bytes_to_mb()` - Memory configuration
4. `min()` / `max()` - Resource limits

---

## 📝 Implementation Notes

### Function Categories Priority
1. **High Priority**: Network, Math, String manipulation (covers 80% of use cases)
2. **Medium Priority**: Advanced filesystem, Date/time, Encoding
3. **Low Priority**: Statistical, i18n, Debugging (nice-to-have)

### Security Considerations
- All filesystem operations must respect `--trust` mode
- Path functions must validate against directory traversal
- Shell escaping must be secure and tested
- Encoding/decoding must handle errors gracefully

### Testing Requirements
- Each new function must have unit tests
- Integration tests for security features
- Example templates demonstrating usage
- Documentation with use cases

### Documentation Structure
For each new function, document:
- Purpose and use case
- Parameters with types
- Return value
- Examples (minimum 2)
- Security considerations (if applicable)
- Related functions

---

## 🔄 Ongoing Improvements

### Performance Optimizations
- [ ] Optimize glob operations for large directories (reduce syscalls)
- [ ] Add benchmarks for all functions (identify bottlenecks)
- [ ] Profile template rendering performance (measure overhead)
- [ ] Lazy-load dependencies (faster startup time)
- [ ] Parallel file operations for glob/list_dir (when safe)

### Developer Experience
- [ ] Better error messages with line numbers
- [ ] Template validation mode (dry-run)
- [ ] Auto-completion for functions in IDEs
- [ ] Template debugging mode with step-through

### CI/CD Integration
- [ ] GitHub Actions integration examples
- [ ] GitLab CI examples
- [ ] Jenkins pipeline examples
- [ ] Terraform integration examples

---

## 📚 References

### Similar Tools for Inspiration
- **Ansible Jinja2 filters**: https://docs.ansible.com/ansible/latest/user_guide/playbooks_filters.html
- **Helm template functions**: https://helm.sh/docs/chart_template_guide/function_list/
- **Terraform functions**: https://www.terraform.io/language/functions
- **Gomplate**: https://docs.gomplate.ca/functions/

### Configuration Management Use Cases
- Nginx configuration generation
- Apache virtual host templates
- Docker Compose file generation
- Kubernetes manifests
- Database connection strings
- Application property files
- CI/CD pipeline configs
- Monitoring tool configs (Prometheus, Grafana)

---

**Last Updated**: 2025-12-31
**Version**: 1.0.0
