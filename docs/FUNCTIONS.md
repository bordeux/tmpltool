# Function Reference

This document provides a comprehensive reference for all available functions in tmpltool. Functions are organized by category.

## Table of Contents

- [Environment Variables](functions/ENVIRONMENT.md)
- [Hash & Crypto Functions](functions/HASH_CRYPTO.md)
- [Encoding & Security Functions](functions/ENCODING.md)
- [Date/Time Functions](functions/DATETIME.md)
- [Command Execution Functions](functions/EXEC.md)
- [Filesystem Functions](functions/FILESYSTEM.md)
- [Path Manipulation Functions](functions/PATH.md)
- [Data Parsing Functions](functions/DATA.md#parsing)
- [Data Serialization Functions](functions/DATA.md#serialization)
- [Object Manipulation Functions](functions/OBJECT.md)
- [Validation Functions](functions/VALIDATION.md)
- [System & Network Functions](functions/SYSTEM_NETWORK.md)
- [Math Functions](functions/MATH.md)
- [Array Functions](functions/ARRAY.md)
- [Statistical Functions](functions/STATISTICS.md)
- [Predicate Functions](functions/PREDICATE.md)
- [Kubernetes Functions](functions/KUBERNETES.md)
- [Web & URL Functions](functions/WEB_URL.md)
- [Logic Functions](functions/LOGIC.md)
- [String Manipulation Functions](functions/STRING.md)
- [Debugging & Development Functions](functions/DEBUG.md)

## Quick Reference

### Environment Variables
- `get_env(name, default)` - Get environment variable with optional default
- `filter_env(pattern)` - Filter environment variables by glob pattern

### Hash & Crypto
- `md5(string)` / `| md5` - Calculate MD5 hash
- `sha1(string)` / `| sha1` - Calculate SHA1 hash
- `sha256(string)` / `| sha256` - Calculate SHA256 hash
- `sha512(string)` / `| sha512` - Calculate SHA512 hash
- `uuid(version)` - Generate UUID (v4 or v7)
- `random_string(length, charset)` - Generate random string

### Encoding & Security
- `base64_encode(string)` / `| base64_encode` - Base64 encoding
- `base64_decode(string)` / `| base64_decode` - Base64 decoding
- `hex_encode(string)` / `| hex_encode` - Hexadecimal encoding
- `hex_decode(string)` / `| hex_decode` - Hexadecimal decoding
- `bcrypt(password, rounds)` - Generate bcrypt hash
- `generate_secret(length, charset)` - Generate cryptographically secure random string
- `hmac_sha256(key, message)` - Generate HMAC-SHA256 signature
- `escape_html(string)` / `| escape_html` - Escape HTML entities
- `escape_xml(string)` / `| escape_xml` - Escape XML entities
- `escape_shell(string)` / `| escape_shell` - Escape for shell commands

### Date/Time
- `now(format)` - Get current timestamp or formatted date
- `format_date(timestamp, format)` / `| format_date` - Format timestamp
- `parse_date(string, format)` - Parse date string to timestamp
- `date_add(timestamp, days)` - Add/subtract days
- `date_diff(timestamp1, timestamp2)` - Calculate day difference
- `get_year(timestamp)` / `| get_year` - Extract year
- `get_month(timestamp)` / `| get_month` - Extract month
- `get_day(timestamp)` / `| get_day` - Extract day
- `get_hour(timestamp)` / `| get_hour` - Extract hour
- `get_minute(timestamp)` / `| get_minute` - Extract minute
- `get_second(timestamp)` / `| get_second` - Extract second
- `timezone_convert(timestamp, from_tz, to_tz)` - Convert timezone
- `is_leap_year(year)` / `{% if year is leap_year %}` - Check leap year

### Filesystem
- `read_file(path)` - Read file content
- `file_exists(path)` - Check if file exists
- `list_dir(path)` - List directory contents
- `glob(pattern)` - Find files matching pattern
- `file_size(path)` - Get file size in bytes
- `file_modified(path)` - Get file modification time

### Path Manipulation
- `basename(path)` / `| basename` - Extract filename
- `dirname(path)` / `| dirname` - Extract directory
- `file_extension(path)` / `| file_extension` - Extract file extension
- `join_path(parts)` / `| join_path` - Join path components
- `normalize_path(path)` / `| normalize_path` - Normalize path
- `is_file(path)` / `{% if path is file %}` - Check if path is file
- `is_dir(path)` / `{% if path is dir %}` - Check if path is directory
- `is_symlink(path)` / `{% if path is symlink %}` - Check if path is symlink
- `read_lines(path, max_lines)` - Read lines from file

### Data Parsing & Serialization
- `parse_json(string)` / `| parse_json` - Parse JSON string
- `parse_yaml(string)` / `| parse_yaml` - Parse YAML string
- `parse_toml(string)` / `| parse_toml` - Parse TOML string
- `read_json_file(path)` - Read and parse JSON file
- `read_yaml_file(path)` - Read and parse YAML file
- `read_toml_file(path)` - Read and parse TOML file
- `to_json(object, pretty)` / `| to_json` - Convert to JSON
- `to_yaml(object)` / `| to_yaml` - Convert to YAML
- `to_toml(object)` / `| to_toml` - Convert to TOML

### Object Manipulation
- `object_merge(obj1, obj2)` - Deep merge objects
- `object_get(object, path)` - Get nested value by path
- `object_set(object, path, value)` - Set nested value
- `object_keys(object)` / `| object_keys` - Get all keys
- `object_values(object)` / `| object_values` - Get all values
- `object_has_key(object, key)` - Check if key exists
- `json_path(object, path)` - Query with JSONPath
- `object_pick(object, keys)` - Pick specific keys
- `object_omit(object, keys)` - Omit specific keys
- `object_rename_keys(object, mapping)` - Rename keys
- `object_flatten(object, delimiter)` / `| object_flatten` - Flatten nested object
- `object_unflatten(object, delimiter)` - Unflatten object

### Validation
- `is_email(string)` / `{% if x is email %}` - Validate email
- `is_url(string)` / `{% if x is url %}` - Validate URL
- `is_ip(string)` / `{% if x is ip %}` - Validate IP address
- `is_uuid(string)` / `{% if x is uuid %}` - Validate UUID

### System & Network
- `get_hostname()` - Get system hostname
- `get_username()` - Get current username
- `get_home_dir()` - Get home directory
- `get_temp_dir()` - Get temp directory
- `get_ip_address(interface)` - Get IP address
- `get_interfaces()` - Get network interfaces
- `resolve_dns(hostname)` - Resolve DNS name
- `is_port_available(port)` / `{% if port is port_available %}` - Check port availability
- `get_os()` - Get operating system
- `get_arch()` - Get architecture
- `get_cwd()` - Get current working directory
- `cidr_contains(cidr, ip)` - Check if IP in CIDR
- `cidr_network(cidr)` - Get CIDR network
- `cidr_broadcast(cidr)` - Get CIDR broadcast
- `cidr_netmask(cidr)` - Get CIDR netmask
- `ip_to_int(ip)` - Convert IP to integer
- `int_to_ip(int)` - Convert integer to IP

### Math
- `min(a, b)` - Minimum of two values
- `max(a, b)` - Maximum of two values
- `abs(number)` - Absolute value
- `round(number, decimals)` - Round number
- `ceil(number)` - Ceiling
- `floor(number)` - Floor
- `percentage(value, total)` - Calculate percentage

### Array & Statistics
- `array_sum(array)` / `| array_sum` - Sum array
- `array_avg(array)` / `| array_avg` - Average
- `array_median(array)` / `| array_median` - Median
- `array_min(array)` / `| array_min` - Minimum
- `array_max(array)` / `| array_max` - Maximum
- `array_count(array)` - Count elements
- `array_chunk(array, size)` - Chunk array
- `array_zip(array1, array2)` - Zip arrays
- `array_sort_by(array, key)` - Sort by key
- `array_group_by(array, key)` - Group by key
- `array_unique(array)` / `| array_unique` - Unique values
- `array_flatten(array)` / `| array_flatten` - Flatten nested array

### Kubernetes
- `k8s_resource_request(cpu, memory)` - Format resource requests
- `k8s_label_safe(value)` / `| k8s_label_safe` - Sanitize label
- `k8s_dns_label_safe(value)` / `| k8s_dns_label_safe` - Sanitize DNS label
- `k8s_env_var_ref(var_name, source, name)` - Create env var reference
- `k8s_secret_ref(secret_name, key, optional)` - Create secret reference
- `k8s_configmap_ref(configmap_name, key, optional)` - Create ConfigMap reference
- `k8s_probe(type, path, port, ...)` - Generate probe YAML

### Web & URL
- `basic_auth(username, password)` - Generate Basic Auth header
- `url_encode(string)` / `| url_encode` - URL encode
- `url_decode(string)` / `| url_decode` - URL decode
- `parse_url(url)` / `| parse_url` - Parse URL
- `build_url(scheme, host, port, path, query)` - Build URL
- `query_string(params)` - Generate query string

### Logic
- `default(value, default)` - Default value
- `coalesce(values)` - First non-null value
- `ternary(condition, true_val, false_val)` - Ternary operator
- `in_range(value, min, max)` - Check if value in range

### String Manipulation
- `regex_replace(string, pattern, replacement)` - Regex replace
- `regex_match(string, pattern)` - Regex match
- `substring(string, start, length)` - Extract substring
- `contains(string, substring)` - Check if contains
- `truncate(string, length, suffix)` - Truncate string
- And many more...

For detailed documentation and examples for each function, see the individual category pages linked above.
