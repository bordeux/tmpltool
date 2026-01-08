## Hash & Crypto Functions

Hash functions (MD5, SHA1, SHA256, SHA512), UUID generation, and random string generation.

**See also:** [Function Reference](../FUNCTIONS.md) | [Main Documentation](../README.md)

Hash functions support both **function syntax** and **filter syntax**. Both produce identical results.

#### `md5(string)` / `| md5`

Calculate MD5 hash of a string.

```jinja
{# Function syntax #}
Checksum: {{ md5(string="hello world") }}
{# Output: 5eb63bbbe01eeed093cb22bb8f5acdc3 #}

{# Filter syntax #}
Checksum: {{ "hello world" | md5 }}
{# Output: 5eb63bbbe01eeed093cb22bb8f5acdc3 #}
```

#### `sha1(string)` / `| sha1`

Calculate SHA1 hash of a string.

```jinja
{# Function syntax #}
Hash: {{ sha1(string="tmpltool") }}

{# Filter syntax #}
Hash: {{ "tmpltool" | sha1 }}
{# Output: c054a2a60ca2fe935ea1056bd90386194116f14f #}
```

#### `sha256(string)` / `| sha256`

Calculate SHA256 hash (recommended for password hashing).

```jinja
{# Function syntax #}
{% set password = get_env(name="PASSWORD", default="secret") %}
Password hash: {{ sha256(string=password) }}

{# Filter syntax #}
Password hash: {{ password | sha256 }}
```

#### `sha512(string)` / `| sha512`

Calculate SHA512 hash (most secure).

```jinja
{# Function syntax #}
Secure hash: {{ sha512(string="secure-data") }}

{# Filter syntax #}
Secure hash: {{ "secure-data" | sha512 }}
```

#### Chaining Hash Filters

Filter syntax enables chaining multiple operations:

```jinja
{# Chain hash with encoding #}
{{ "hello" | sha256 | md5 }}

{# Equivalent function syntax (nested) #}
{{ md5(string=sha256(string="hello")) }}
```

**Important:** These hash functions are for checksums and general-purpose hashing. For production password storage, use dedicated password hashing libraries with salt and proper key derivation functions (bcrypt, argon2, etc.).

#### `uuid(version)`

Generate a UUID (Universally Unique Identifier) with configurable version.

**Arguments:**
- `version` (optional) - UUID version to generate: `"v4"` (default) or `"v7"`

**UUID Versions:**
- `v4` - Random UUID (default) - suitable for most use cases
- `v7` - Time-ordered UUID - sortable by creation time, ideal for database primary keys

**Examples:**
```
{# Default v4 (random) #}
Request ID: {{ uuid() }}
Session ID: {{ uuid(version="v4") }}

{# v7 (time-ordered, sortable) #}
Database ID: {{ uuid(version="v7") }}
Event ID: {{ uuid(version="v7") }}
{# v7 UUIDs are lexicographically sortable by creation time #}
```

**When to use which version:**
- Use **v4** for general-purpose unique identifiers where ordering doesn't matter
- Use **v7** for database primary keys, event logs, or anywhere you need time-based sorting

#### `random_string(length, charset)`

Generate a random string with customizable length and character set.

**Arguments:**
- `length` (required) - Length of string (1-10000)
- `charset` (optional) - Character set preset or custom string (default: `alphanumeric`)

**Character Set Presets:**
- `alphanumeric` - Letters and digits (a-z, A-Z, 0-9) - **default**
- `alphabetic` or `alpha` - Letters only (a-z, A-Z)
- `lowercase` or `lower` - Lowercase letters (a-z)
- `uppercase` or `upper` - Uppercase letters (A-Z)
- `numeric` or `digits` - Digits only (0-9)
- `hex` or `hexadecimal` - Hexadecimal (0-9, a-f)
- `hex_upper` - Hexadecimal uppercase (0-9, A-F)
- Custom string - Any custom character set (e.g., `"abc123"`)

**Examples:**
```
{# Alphanumeric (default) #}
API Key: {{ random_string(length=32) }}

{# Lowercase only #}
Username: user_{{ random_string(length=8, charset="lowercase") }}

{# Numeric PIN #}
PIN: {{ random_string(length=4, charset="numeric") }}

{# Hexadecimal token #}
Token: {{ random_string(length=16, charset="hex") }}

{# Custom charset #}
Password: {{ random_string(length=12, charset="abc123") }}
```

**Practical Example:**
```yaml
application:
  instance_id: {{ uuid() }}
  secret_key: {{ random_string(length=64) }}
  api_token: {{ random_string(length=32, charset="hex") }}
  csrf_token: {{ random_string(length=40, charset="hex") }}

security:
  password_hash: {{ sha256(string=get_env(name="PASSWORD")) }}
```

