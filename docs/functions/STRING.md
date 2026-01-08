## String Manipulation Functions

String manipulation: regex operations, substring extraction, truncation, and more.

**See also:** [Function Reference](../FUNCTIONS.md) | [Main Documentation](../README.md)

Advanced string operations including regex support.

#### `regex_replace(string, pattern, replacement)`

Replace substrings using a regex pattern. Supports both function and filter syntax.

**Arguments:**
- `string` (required): The input string
- `pattern` (required): Regex pattern to match
- `replacement` (required): Replacement string (supports `$1`, `$2` for capture groups)

**Returns:** String with all matches replaced

**Example:**
```jinja
{# Function syntax #}
{{ regex_replace(string="hello123world", pattern="[0-9]+", replacement="-") }}
{# Output: hello-world #}

{# Filter syntax #}
{{ "hello123world" | regex_replace(pattern="[0-9]+", replacement="-") }}
{# Output: hello-world #}

{{ "foo bar baz" | regex_replace(pattern="\\s+", replacement="_") }}
{# Output: foo_bar_baz #}

{# Using capture groups #}
{{ "hello world" | regex_replace(pattern="(\\w+) (\\w+)", replacement="$2 $1") }}
{# Output: world hello #}
```

#### `regex_match(string, pattern)`

Check if a string matches a regex pattern.

**Arguments:**
- `string` (required): The input string
- `pattern` (required): Regex pattern to match

**Returns:** Boolean - true if the pattern matches anywhere in the string

**Example:**
```jinja
{{ regex_match(string="hello123", pattern="[0-9]+") }}
{# Output: true #}

{{ regex_match(string="hello", pattern="[0-9]+") }}
{# Output: false #}

{# Validate email format #}
{% if regex_match(string=email, pattern="^[\\w.-]+@[\\w.-]+\\.\\w+$") %}
  Valid email
{% endif %}
```

#### `regex_find_all(string, pattern)`

Find all matches of a regex pattern in a string.

**Arguments:**
- `string` (required): The input string
- `pattern` (required): Regex pattern to match

**Returns:** Array of all matches

**Example:**
```jinja
{{ regex_find_all(string="a1b2c3", pattern="[0-9]+") | tojson }}
{# Output: ["1", "2", "3"] #}

{{ regex_find_all(string="hello world", pattern="\\w+") | tojson }}
{# Output: ["hello", "world"] #}

{# Extract all URLs #}
{% set urls = regex_find_all(string=text, pattern="https?://[\\w./]+") %}
Found {{ urls | length }} URLs
```

#### `substring(string, start, length)`

Extract a substring by position. Supports both function and filter syntax.

**Arguments:**
- `string` (required): The input string
- `start` (required): Start position (0-based, negative counts from end)
- `length` (optional): Number of characters to extract (default: rest of string)

**Returns:** The extracted substring

**Example:**
```jinja
{# Function syntax #}
{{ substring(string="hello world", start=0, length=5) }}
{# Output: hello #}

{# Filter syntax #}
{{ "hello world" | substring(start=0, length=5) }}
{# Output: hello #}

{{ "hello world" | substring(start=6) }}
{# Output: world #}

{# Negative start counts from end #}
{{ "hello world" | substring(start=-5) }}
{# Output: world #}
```

#### `contains(string, substring)`

Check if a string contains a substring.

**Arguments:**
- `string` (required): The input string
- `substring` (required): Substring to search for

**Returns:** Boolean - true if substring is found

**Example:**
```jinja
{{ contains(string="hello world", substring="world") }}
{# Output: true #}

{{ contains(string="hello world", substring="foo") }}
{# Output: false #}

{% if contains(string=filename, substring=".txt") %}
  Text file detected
{% endif %}
```

#### `index_of(string, substring)`

Find the position of a substring.

**Arguments:**
- `string` (required): The input string
- `substring` (required): Substring to search for

**Returns:** Position (0-based) or -1 if not found

**Example:**
```jinja
{{ index_of(string="hello world", substring="world") }}
{# Output: 6 #}

{{ index_of(string="hello world", substring="foo") }}
{# Output: -1 #}
```

#### `count_occurrences(string, substring)`

Count occurrences of a substring.

**Arguments:**
- `string` (required): The input string
- `substring` (required): Substring to count

**Returns:** Number of non-overlapping occurrences

**Example:**
```jinja
{{ count_occurrences(string="hello hello hello", substring="hello") }}
{# Output: 3 #}

{{ count_occurrences(string="abcabc", substring="abc") }}
{# Output: 2 #}
```

#### `truncate(string, length, suffix)`

Truncate a string with a suffix. Supports both function and filter syntax.

**Arguments:**
- `string` (required): The input string
- `length` (required): Maximum length (including suffix)
- `suffix` (optional): Suffix to add when truncated (default: `"..."`)

**Returns:** Truncated string with suffix if it was truncated

**Example:**
```jinja
{# Function syntax #}
{{ truncate(string="Hello World", length=8) }}
{# Output: Hello... #}

{# Filter syntax #}
{{ "Hello World" | truncate(length=8) }}
{# Output: Hello... #}

{{ "Hello World" | truncate(length=8, suffix=">>") }}
{# Output: Hello >> #}

{# Not truncated if already short enough #}
{{ "Hi" | truncate(length=10) }}
{# Output: Hi #}
```

#### `word_count(string)`

Count words in a string. Supports both function and filter syntax.

**Arguments:**
- `string` (required): The input string

**Returns:** Number of words (whitespace-separated)

**Example:**
```jinja
{# Function syntax #}
{{ word_count(string="Hello World") }}
{# Output: 2 #}

{# Filter syntax #}
{{ "Hello World" | word_count }}
{# Output: 2 #}

{{ "  one   two   three  " | word_count }}
{# Output: 3 #}
```

#### `split_lines(string)`

Split a string into an array of lines. Supports both function and filter syntax.

**Arguments:**
- `string` (required): The input string

**Returns:** Array of lines

**Example:**
```jinja
{# Function syntax #}
{% set text = "line1
line2
line3" %}
{{ split_lines(string=text) | tojson }}
{# Output: ["line1", "line2", "line3"] #}

{# Filter syntax #}
{{ text | split_lines | tojson }}
{# Output: ["line1", "line2", "line3"] #}

{% for line in content | split_lines %}
  Line: {{ line }}
{% endfor %}
```

#### `wrap(string, width, indent)`

Word wrap text at a specified width. Supports both function and filter syntax.

**Arguments:**
- `string` (required): The input string to wrap
- `width` (required): Maximum line width
- `indent` (optional): Indentation string for wrapped lines (default: "")

**Returns:** The wrapped text with newlines inserted

**Example:**
```jinja
{# Function syntax #}
{{ wrap(string="The quick brown fox jumps over the lazy dog", width=20) }}

{# Filter syntax #}
{{ "The quick brown fox jumps over the lazy dog" | wrap(width=20) }}
{# Output:
The quick brown fox
jumps over the lazy
dog
#}

{{ "Hello World Example" | wrap(width=10, indent="  ") }}
{# Output:
Hello
  World
  Example
#}
```

#### `center(string, width, char)`

Center text with padding. Supports both function and filter syntax.

**Arguments:**
- `string` (required): The input string
- `width` (required): Total width of the result
- `char` (optional): Padding character (default: space)

**Returns:** The centered string with padding

**Example:**
```jinja
{# Function syntax #}
{{ center(string="hello", width=11) }}
{# Output: "   hello   " #}

{# Filter syntax #}
{{ "hello" | center(width=11) }}
{# Output: "   hello   " #}

{{ "hi" | center(width=10, char="-") }}
{# Output: "----hi----" #}

{{ "test" | center(width=8, char="*") }}
{# Output: "**test**" #}
```

#### `sentence_case(string)`

Convert to Sentence case (first letter capitalized, rest lowercase).

**Arguments:**
- `string` (required): The input string

**Returns:** The string in sentence case

```jinja
{{ sentence_case(string="hello world") }}
{# Output: Hello world #}

{{ sentence_case(string="HELLO WORLD") }}
{# Output: Hello world #}

{{ sentence_case(string="hELLO wORLD") }}
{# Output: Hello world #}
```

#### `strip_html(string)`

Remove HTML tags from a string. Supports both function and filter syntax.

**Arguments:**
- `string` (required): The input string with HTML

**Returns:** The string with all HTML tags removed

**Example:**
```jinja
{# Function syntax #}
{{ strip_html(string="<p>Hello <b>World</b></p>") }}
{# Output: Hello World #}

{# Filter syntax #}
{{ "<p>Hello <b>World</b></p>" | strip_html }}
{# Output: Hello World #}

{{ "<div class='test'>Content</div>" | strip_html }}
{# Output: Content #}

{{ html_content | strip_html | normalize_whitespace }}
{# Chaining with other filters #}
```

#### `strip_ansi(string)`

Remove ANSI escape codes from a string. Supports both function and filter syntax.

**Arguments:**
- `string` (required): The input string with ANSI codes

**Returns:** The string with all ANSI escape codes removed

**Example:**
```jinja
{# Function syntax #}
{{ strip_ansi(string="\x1b[31mRed Text\x1b[0m") }}
{# Output: Red Text #}

{# Filter syntax #}
{{ "\x1b[31mRed Text\x1b[0m" | strip_ansi }}
{# Output: Red Text #}

{{ terminal_output | strip_ansi }}
{# Remove color codes from terminal output #}
```

#### `normalize_whitespace(string)`

Normalize whitespace by collapsing multiple spaces/tabs/newlines into a single space. Supports both function and filter syntax.

**Arguments:**
- `string` (required): The input string

**Returns:** The string with normalized whitespace (trimmed and collapsed)

**Example:**
```jinja
{# Function syntax #}
{{ normalize_whitespace(string="  hello   world  ") }}
{# Output: hello world #}

{# Filter syntax #}
{{ "  hello   world  " | normalize_whitespace }}
{# Output: hello world #}

{{ "line1\n\n\nline2\t\tline3" | normalize_whitespace }}
{# Output: line1 line2 line3 #}

{# Chaining with other filters #}
{{ html_content | strip_html | normalize_whitespace | truncate(length=100) }}
```

#### `to_constant_case(string)`

Convert to CONSTANT_CASE (uppercase with underscores).

**Arguments:**
- `string` (required): The input string

**Returns:** The string in CONSTANT_CASE format

```jinja
{{ to_constant_case(string="hello world") }}
{# Output: HELLO_WORLD #}

{{ to_constant_case(string="helloWorld") }}
{# Output: HELLO_WORLD #}

{{ to_constant_case(string="hello-world-test") }}
{# Output: HELLO_WORLD_TEST #}

{{ to_constant_case(string="HTTPResponse") }}
{# Output: HTTPRESPONSE #}
```

#### `pluralize(count, singular, plural)`

Pluralize a word based on count.

**Arguments:**
- `count` (required): The count to check
- `singular` (required): The singular form of the word
- `plural` (optional): The plural form (default: singular + "s")

**Returns:** Singular if count is 1, otherwise plural

```jinja
{{ pluralize(count=1, singular="item") }}
{# Output: item #}

{{ pluralize(count=5, singular="item") }}
{# Output: items #}

{{ pluralize(count=0, singular="child", plural="children") }}
{# Output: children #}

{# Use with variables #}
You have {{ count }} {{ pluralize(count=count, singular="message", plural="messages") }}
```

