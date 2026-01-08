## Date/Time Functions

Date and time functions for formatting, parsing, timezone conversion, and date arithmetic.

**See also:** [Function Reference](../FUNCTIONS.md) | [Main Documentation](../README.md)

Work with dates, times, and timestamps. All functions use Unix timestamps (seconds since epoch) for consistent timezone-independent representation.

#### `now(format)`

Get the current Unix timestamp, or formatted date string.

**Arguments:**
- `format` (optional) - Format string. If provided, returns formatted string.

**Returns:** Unix timestamp (integer) by default, or formatted string if `format` is provided.

**Examples:**
```
{# Get Unix timestamp (default) #}
Timestamp: {{ now() }}
{# Output: 1704067200 #}

{# Use with format_date for formatting #}
{{ format_date(timestamp=now(), format="%Y-%m-%d %H:%M:%S") }}
{# Output: 2024-12-31 12:34:56 #}

{# Or use format parameter directly #}
{{ now(format="%Y-%m-%d %H:%M:%S") }}
{# Output: 2024-12-31 12:34:56 #}

{# Date only #}
{{ now(format="%Y-%m-%d") }}
{# Output: 2024-12-31 #}
```

#### `format_date(timestamp, format)` / `| format_date`

Format a Unix timestamp with a custom format string.

**Arguments:**
- `timestamp` (required) - Unix timestamp in seconds
- `format` (optional) - Format string (default: `"%Y-%m-%d %H:%M:%S"`)

**Returns:** Formatted date string

**Common Format Specifiers:**
- `%Y` - Year (4 digits), e.g., 2024
- `%m` - Month (01-12)
- `%d` - Day (01-31)
- `%H` - Hour 24h (00-23)
- `%I` - Hour 12h (01-12)
- `%M` - Minute (00-59)
- `%S` - Second (00-59)
- `%p` - AM/PM
- `%A` - Weekday (full), e.g., Monday
- `%B` - Month (full), e.g., January

[Full format reference](https://docs.rs/chrono/latest/chrono/format/strftime/index.html)

**Function syntax:**
```
{% set ts = 1704067200 %}
ISO date: {{ format_date(timestamp=ts, format="%Y-%m-%d") }}
{# Output: 2024-01-01 #}

Full: {{ format_date(timestamp=ts, format="%B %d, %Y at %I:%M %p") }}
{# Output: January 01, 2024 at 12:00 AM #}
```

**Filter syntax:**
```
{% set ts = 1704067200 %}
{{ ts | format_date(format="%Y-%m-%d") }}
{# Output: 2024-01-01 #}

{# Chain with now() #}
{{ now() | format_date(format="%B %d, %Y") }}
```

#### `parse_date(string, format)`

Parse a date string into a Unix timestamp.

**Arguments:**
- `string` (required) - Date string to parse
- `format` (required) - Format string matching the input

**Returns:** Unix timestamp (integer)

**Examples:**
```
{% set ts = parse_date(string="2024-01-01 12:00:00", format="%Y-%m-%d %H:%M:%S") %}
Timestamp: {{ ts }}
{# Output: 1704110400 #}

{# Date-only formats (time set to midnight) #}
{% set ts = parse_date(string="12/25/2024", format="%m/%d/%Y") %}
{{ format_date(timestamp=ts, format="%Y-%m-%d") }}
{# Output: 2024-12-25 #}
```

#### `date_add(timestamp, days)`

Add or subtract days from a Unix timestamp.

**Arguments:**
- `timestamp` (required) - Unix timestamp in seconds
- `days` (required) - Number of days to add (can be negative)

**Returns:** New Unix timestamp

**Examples:**
```
{% set today = parse_date(string="2024-01-01", format="%Y-%m-%d") %}

{# Add days #}
Next week: {{ format_date(timestamp=date_add(timestamp=today, days=7), format="%Y-%m-%d") }}
{# Output: 2024-01-08 #}

{# Subtract days #}
Last week: {{ format_date(timestamp=date_add(timestamp=today, days=-7), format="%Y-%m-%d") }}
{# Output: 2023-12-25 #}
```

#### `date_diff(timestamp1, timestamp2)`

Calculate the difference in days between two timestamps.

**Arguments:**
- `timestamp1` (required) - First Unix timestamp
- `timestamp2` (required) - Second Unix timestamp

**Returns:** Difference in days (timestamp1 - timestamp2)

**Examples:**
```
{% set start = parse_date(string="2024-01-01", format="%Y-%m-%d") %}
{% set end = parse_date(string="2024-01-31", format="%Y-%m-%d") %}

Days between: {{ date_diff(timestamp1=end, timestamp2=start) }}
{# Output: 30 #}
```

#### `get_year(timestamp)` / `| get_year`

Extract the year from a Unix timestamp.

**Arguments:**
- `timestamp` (required) - Unix timestamp in seconds

**Returns:** Integer (4-digit year)

**Function syntax:**
```
{{ get_year(timestamp=1704067200) }}
{# Output: 2024 #}
```

**Filter syntax:**
```
{{ 1704067200 | get_year }}
{{ now() | get_year }}
```

#### `get_month(timestamp)` / `| get_month`

Extract the month from a Unix timestamp.

**Arguments:**
- `timestamp` (required) - Unix timestamp in seconds

**Returns:** Integer (1-12)

**Function syntax:**
```
{{ get_month(timestamp=1704067200) }}
{# Output: 1 #}
```

**Filter syntax:**
```
{{ 1704067200 | get_month }}
{{ now() | get_month }}
```

#### `get_day(timestamp)` / `| get_day`

Extract the day from a Unix timestamp.

**Arguments:**
- `timestamp` (required) - Unix timestamp in seconds

**Returns:** Integer (1-31)

**Function syntax:**
```
{{ get_day(timestamp=1704067200) }}
{# Output: 1 #}
```

**Filter syntax:**
```
{{ 1704067200 | get_day }}
{{ now() | get_day }}
```

#### `get_hour(timestamp)` / `| get_hour`

Extract the hour from a Unix timestamp.

**Arguments:**
- `timestamp` (required) - Unix timestamp in seconds

**Returns:** Integer (0-23)

**Function syntax:**
```
{{ get_hour(timestamp=1704110400) }}
{# Output: 12 #}
```

**Filter syntax:**
```
{{ 1704110400 | get_hour }}
{{ now() | get_hour }}
```

#### `get_minute(timestamp)` / `| get_minute`

Extract the minute from a Unix timestamp.

**Arguments:**
- `timestamp` (required) - Unix timestamp in seconds

**Returns:** Integer (0-59)

**Function syntax:**
```
{{ get_minute(timestamp=1704068700) }}
{# Output: 25 #}
```

**Filter syntax:**
```
{{ 1704068700 | get_minute }}
{{ now() | get_minute }}
```

#### `get_second(timestamp)` / `| get_second`

Extract the second from a Unix timestamp.

**Arguments:**
- `timestamp` (required) - Unix timestamp in seconds

**Returns:** Integer (0-59)

**Function syntax:**
```
{{ get_second(timestamp=1704067245) }}
{# Output: 45 #}
```

**Filter syntax:**
```
{{ 1704067245 | get_second }}
{{ now() | get_second }}
```

#### `timezone_convert(timestamp, from_tz, to_tz)`

Convert a timestamp between timezones.

**Arguments:**
- `timestamp` (required) - Unix timestamp in seconds
- `from_tz` (required) - Source timezone (e.g., "UTC", "America/New_York")
- `to_tz` (required) - Target timezone (e.g., "Europe/London", "Asia/Tokyo")

**Returns:** Unix timestamp (note: Unix timestamps are timezone-independent)

**Note:** Unix timestamps are always UTC-relative. This function is useful when formatting times in different timezones.

**Examples:**
```
{% set utc_ts = 1704067200 %}
{{ timezone_convert(timestamp=utc_ts, from_tz="UTC", to_tz="America/New_York") }}
```

#### `is_leap_year(year)` / `{% if year is leap_year %}`

Check if a year is a leap year. Supports both function syntax and "is" test syntax.

**Function Syntax Arguments:**
- `year` (required) - Year to check (4-digit integer)

**Is-Test Syntax:**
- The value must be an integer or a string that can be parsed as an integer

**Returns:** Boolean (true if leap year, false otherwise)

**Examples:**
```jinja
{# Function syntax #}
{% if is_leap_year(year=2024) %}
2024 is a leap year
{% endif %}

{# Is-test syntax (preferred for readability) #}
{% if 2024 is leap_year %}
2024 is a leap year
{% endif %}

{# With variables #}
{% set years = [2020, 2021, 2022, 2023, 2024] %}
{% for year in years %}
{{ year }}: {% if year is leap_year %}Leap{% else %}Regular{% endif %}
{% endfor %}
```

**Practical Example - Certificate Expiration:**
```yaml
{% set cert_expiry = parse_date(string="2025-06-15", format="%Y-%m-%d") %}
{% set today = parse_date(string="2024-12-31", format="%Y-%m-%d") %}
{% set days_until_expiry = date_diff(timestamp1=cert_expiry, timestamp2=today) %}

certificates:
  ssl_cert:
    expires: {{ format_date(timestamp=cert_expiry, format="%B %d, %Y") }}
    days_remaining: {{ days_until_expiry }}
    {% if days_until_expiry < 30 %}
    warning: "Certificate expires in {{ days_until_expiry }} days - RENEW IMMEDIATELY"
    priority: critical
    {% elif days_until_expiry < 90 %}
    warning: "Certificate expires in {{ days_until_expiry }} days - schedule renewal"
    priority: high
    {% else %}
    status: valid
    priority: normal
    {% endif %}
```

**Practical Example - Backup Schedule:**
```bash
#!/bin/bash
{% set backup_ts = parse_date(string="2024-01-15 02:00:00", format="%Y-%m-%d %H:%M:%S") %}
# Weekly backups
{% for week in range(0, 4) %}
WEEKLY_BACKUP_{{ week + 1 }}="{{ format_date(timestamp=date_add(timestamp=backup_ts, days=week * 7), format="%Y-%m-%d") }}"
{% endfor %}

# Retention: Keep backups for 30 days
{% set retention_cutoff = date_add(timestamp=backup_ts, days=-30) %}
DELETE_BEFORE="{{ format_date(timestamp=retention_cutoff, format="%Y-%m-%d") }}"
```

