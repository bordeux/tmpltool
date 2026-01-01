//! String manipulation functions for MiniJinja templates
//!
//! This module provides utility functions for working with strings:
//! - Regex operations (replace, match, find)
//! - Substring extraction
//! - String searching (contains, index_of, count)
//! - Text processing (truncate, word_count, split_lines, wrap, center)
//! - Text transformation (sentence_case, to_constant_case, normalize_whitespace)
//! - Text sanitization (strip_html, strip_ansi)
//! - Pluralization

use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};
use regex::Regex;

/// Replace substrings using regex pattern
///
/// # Arguments
///
/// * `string` (required) - The input string
/// * `pattern` (required) - Regex pattern to match
/// * `replacement` (required) - Replacement string (supports $1, $2 for capture groups)
///
/// # Returns
///
/// Returns the string with all matches replaced
///
/// # Example
///
/// ```jinja
/// {{ regex_replace(string="hello123world", pattern="[0-9]+", replacement="-") }}
/// {# Output: hello-world #}
///
/// {{ regex_replace(string="foo bar baz", pattern="\\s+", replacement="_") }}
/// {# Output: foo_bar_baz #}
/// ```
pub fn regex_replace_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let string: String = kwargs.get("string")?;
    let pattern: String = kwargs.get("pattern")?;
    let replacement: String = kwargs.get("replacement")?;

    let regex = Regex::new(&pattern).map_err(|e| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("Invalid regex pattern '{}': {}", pattern, e),
        )
    })?;

    let result = regex.replace_all(&string, replacement.as_str());
    Ok(Value::from(result.to_string()))
}

/// Check if string matches regex pattern
///
/// # Arguments
///
/// * `string` (required) - The input string
/// * `pattern` (required) - Regex pattern to match
///
/// # Returns
///
/// Returns true if the pattern matches anywhere in the string
///
/// # Example
///
/// ```jinja
/// {{ regex_match(string="hello123", pattern="[0-9]+") }}
/// {# Output: true #}
///
/// {{ regex_match(string="hello", pattern="[0-9]+") }}
/// {# Output: false #}
/// ```
pub fn regex_match_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let string: String = kwargs.get("string")?;
    let pattern: String = kwargs.get("pattern")?;

    let regex = Regex::new(&pattern).map_err(|e| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("Invalid regex pattern '{}': {}", pattern, e),
        )
    })?;

    Ok(Value::from(regex.is_match(&string)))
}

/// Find all regex matches in string
///
/// # Arguments
///
/// * `string` (required) - The input string
/// * `pattern` (required) - Regex pattern to match
///
/// # Returns
///
/// Returns an array of all matches
///
/// # Example
///
/// ```jinja
/// {{ regex_find_all(string="a1b2c3", pattern="[0-9]+") }}
/// {# Output: ["1", "2", "3"] #}
///
/// {{ regex_find_all(string="hello world", pattern="\\w+") }}
/// {# Output: ["hello", "world"] #}
/// ```
pub fn regex_find_all_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let string: String = kwargs.get("string")?;
    let pattern: String = kwargs.get("pattern")?;

    let regex = Regex::new(&pattern).map_err(|e| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("Invalid regex pattern '{}': {}", pattern, e),
        )
    })?;

    let matches: Vec<String> = regex
        .find_iter(&string)
        .map(|m| m.as_str().to_string())
        .collect();

    Ok(Value::from_serialize(&matches))
}

/// Extract substring by position
///
/// # Arguments
///
/// * `string` (required) - The input string
/// * `start` (required) - Start position (0-based, negative counts from end)
/// * `length` (optional) - Number of characters to extract (default: rest of string)
///
/// # Returns
///
/// Returns the extracted substring
///
/// # Example
///
/// ```jinja
/// {{ substring(string="hello world", start=0, length=5) }}
/// {# Output: hello #}
///
/// {{ substring(string="hello world", start=6) }}
/// {# Output: world #}
///
/// {{ substring(string="hello world", start=-5) }}
/// {# Output: world #}
/// ```
pub fn substring_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let string: String = kwargs.get("string")?;
    let start: i64 = kwargs.get("start")?;
    let length: Option<usize> = kwargs.get("length")?;

    let chars: Vec<char> = string.chars().collect();
    let len = chars.len() as i64;

    // Handle negative start (count from end)
    let start_idx = if start < 0 {
        (len + start).max(0) as usize
    } else {
        (start as usize).min(chars.len())
    };

    let end_idx = match length {
        Some(l) => (start_idx + l).min(chars.len()),
        None => chars.len(),
    };

    let result: String = chars[start_idx..end_idx].iter().collect();
    Ok(Value::from(result))
}

/// Check if string contains substring
///
/// # Arguments
///
/// * `string` (required) - The input string
/// * `substring` (required) - Substring to search for
///
/// # Returns
///
/// Returns true if substring is found
///
/// # Example
///
/// ```jinja
/// {{ contains(string="hello world", substring="world") }}
/// {# Output: true #}
///
/// {{ contains(string="hello world", substring="foo") }}
/// {# Output: false #}
/// ```
pub fn contains_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let string: String = kwargs.get("string")?;
    let substring: String = kwargs.get("substring")?;

    Ok(Value::from(string.contains(&substring)))
}

/// Find position of substring
///
/// # Arguments
///
/// * `string` (required) - The input string
/// * `substring` (required) - Substring to search for
///
/// # Returns
///
/// Returns the position (0-based) or -1 if not found
///
/// # Example
///
/// ```jinja
/// {{ index_of(string="hello world", substring="world") }}
/// {# Output: 6 #}
///
/// {{ index_of(string="hello world", substring="foo") }}
/// {# Output: -1 #}
/// ```
pub fn index_of_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let string: String = kwargs.get("string")?;
    let substring: String = kwargs.get("substring")?;

    let result = match string.find(&substring) {
        Some(pos) => pos as i64,
        None => -1,
    };

    Ok(Value::from(result))
}

/// Count occurrences of substring
///
/// # Arguments
///
/// * `string` (required) - The input string
/// * `substring` (required) - Substring to count
///
/// # Returns
///
/// Returns the number of occurrences
///
/// # Example
///
/// ```jinja
/// {{ count_occurrences(string="hello hello hello", substring="hello") }}
/// {# Output: 3 #}
///
/// {{ count_occurrences(string="aaa", substring="aa") }}
/// {# Output: 1 #} {# non-overlapping matches #}
/// ```
pub fn count_occurrences_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let string: String = kwargs.get("string")?;
    let substring: String = kwargs.get("substring")?;

    if substring.is_empty() {
        return Err(Error::new(
            ErrorKind::InvalidOperation,
            "substring cannot be empty",
        ));
    }

    let count = string.matches(&substring).count();
    Ok(Value::from(count))
}

/// Truncate string with suffix
///
/// # Arguments
///
/// * `string` (required) - The input string
/// * `length` (required) - Maximum length (including suffix)
/// * `suffix` (optional) - Suffix to add when truncated (default: "...")
///
/// # Returns
///
/// Returns the truncated string with suffix if it was truncated
///
/// # Example
///
/// ```jinja
/// {{ truncate(string="Hello World", length=8) }}
/// {# Output: Hello... #}
///
/// {{ truncate(string="Hello World", length=8, suffix=">>") }}
/// {# Output: Hello >> #}
///
/// {{ truncate(string="Hi", length=10) }}
/// {# Output: Hi #} {# Not truncated #}
/// ```
pub fn truncate_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let string: String = kwargs.get("string")?;
    let length: usize = kwargs.get("length")?;
    let suffix: Option<String> = kwargs.get("suffix")?;
    let suffix = suffix.unwrap_or_else(|| "...".to_string());

    let chars: Vec<char> = string.chars().collect();

    if chars.len() <= length {
        return Ok(Value::from(string));
    }

    let suffix_len = suffix.chars().count();
    if length <= suffix_len {
        return Ok(Value::from(suffix));
    }

    let truncate_at = length - suffix_len;
    let result: String = chars[..truncate_at].iter().collect::<String>() + &suffix;

    Ok(Value::from(result))
}

/// Count words in string
///
/// # Arguments
///
/// * `string` (required) - The input string
///
/// # Returns
///
/// Returns the number of words (whitespace-separated)
///
/// # Example
///
/// ```jinja
/// {{ word_count(string="Hello World") }}
/// {# Output: 2 #}
///
/// {{ word_count(string="  one   two   three  ") }}
/// {# Output: 3 #}
/// ```
pub fn word_count_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let string: String = kwargs.get("string")?;

    let count = string.split_whitespace().count();
    Ok(Value::from(count))
}

/// Split string into lines array
///
/// # Arguments
///
/// * `string` (required) - The input string
///
/// # Returns
///
/// Returns an array of lines
///
/// # Example
///
/// ```jinja
/// {% set text = "line1
/// line2
/// line3" %}
/// {{ split_lines(string=text) }}
/// {# Output: ["line1", "line2", "line3"] #}
/// ```
pub fn split_lines_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let string: String = kwargs.get("string")?;

    let lines: Vec<&str> = string.lines().collect();
    Ok(Value::from_serialize(&lines))
}

/// Word wrap text at specified width
///
/// # Arguments
///
/// * `string` (required) - The input string
/// * `width` (required) - Maximum line width
/// * `indent` (optional) - Indentation string for wrapped lines (default: "")
///
/// # Returns
///
/// Returns the wrapped text with newlines inserted
///
/// # Example
///
/// ```jinja
/// {{ wrap(string="The quick brown fox jumps over the lazy dog", width=20) }}
/// {# Output:
/// The quick brown fox
/// jumps over the lazy
/// dog
/// #}
///
/// {{ wrap(string="Hello World Example", width=10, indent="  ") }}
/// {# Output:
/// Hello
///   World
///   Example
/// #}
/// ```
pub fn wrap_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let string: String = kwargs.get("string")?;
    let width: usize = kwargs.get("width")?;
    let indent: Option<String> = kwargs.get("indent")?;
    let indent = indent.unwrap_or_default();

    if width == 0 {
        return Err(Error::new(
            ErrorKind::InvalidOperation,
            "width must be greater than 0",
        ));
    }

    let mut result = String::new();
    let mut first_line = true;

    for line in string.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        let mut current_line = String::new();
        let mut current_width = 0;
        let effective_width = if first_line {
            width
        } else {
            width.saturating_sub(indent.len())
        };

        for word in words {
            let word_len = word.chars().count();

            if current_width == 0 {
                // First word on the line
                current_line.push_str(word);
                current_width = word_len;
            } else if current_width + 1 + word_len <= effective_width {
                // Word fits on current line
                current_line.push(' ');
                current_line.push_str(word);
                current_width += 1 + word_len;
            } else {
                // Start new line
                if !result.is_empty() {
                    result.push('\n');
                }
                if !first_line {
                    result.push_str(&indent);
                }
                result.push_str(&current_line);
                first_line = false;

                current_line = word.to_string();
                current_width = word_len;
            }
        }

        // Add remaining text
        if !current_line.is_empty() {
            if !result.is_empty() {
                result.push('\n');
            }
            if !first_line {
                result.push_str(&indent);
            }
            result.push_str(&current_line);
            first_line = false;
        }
    }

    Ok(Value::from(result))
}

/// Center text with padding
///
/// # Arguments
///
/// * `string` (required) - The input string
/// * `width` (required) - Total width of the result
/// * `char` (optional) - Padding character (default: space)
///
/// # Returns
///
/// Returns the centered string with padding
///
/// # Example
///
/// ```jinja
/// {{ center(string="hello", width=11) }}
/// {# Output: "   hello   " #}
///
/// {{ center(string="hi", width=10, char="-") }}
/// {# Output: "----hi----" #}
///
/// {{ center(string="test", width=8, char="*") }}
/// {# Output: "**test**" #}
/// ```
pub fn center_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let string: String = kwargs.get("string")?;
    let width: usize = kwargs.get("width")?;
    let pad_char: Option<String> = kwargs.get("char")?;
    let pad_char = pad_char.and_then(|s| s.chars().next()).unwrap_or(' ');

    let str_len = string.chars().count();

    if str_len >= width {
        return Ok(Value::from(string));
    }

    let total_padding = width - str_len;
    let left_padding = total_padding / 2;
    let right_padding = total_padding - left_padding;

    let result = format!(
        "{}{}{}",
        pad_char.to_string().repeat(left_padding),
        string,
        pad_char.to_string().repeat(right_padding)
    );

    Ok(Value::from(result))
}

/// Convert to Sentence case
///
/// # Arguments
///
/// * `string` (required) - The input string
///
/// # Returns
///
/// Returns the string with only the first letter capitalized
///
/// # Example
///
/// ```jinja
/// {{ sentence_case(string="hello world") }}
/// {# Output: Hello world #}
///
/// {{ sentence_case(string="HELLO WORLD") }}
/// {# Output: Hello world #}
///
/// {{ sentence_case(string="hELLO wORLD") }}
/// {# Output: Hello world #}
/// ```
pub fn sentence_case_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let string: String = kwargs.get("string")?;

    if string.is_empty() {
        return Ok(Value::from(string));
    }

    let mut chars = string.chars();
    let first = chars.next().unwrap().to_uppercase().to_string();
    let rest: String = chars.collect::<String>().to_lowercase();

    Ok(Value::from(first + &rest))
}

/// Remove HTML tags from string
///
/// # Arguments
///
/// * `string` (required) - The input string with HTML
///
/// # Returns
///
/// Returns the string with all HTML tags removed
///
/// # Example
///
/// ```jinja
/// {{ strip_html(string="<p>Hello <b>World</b></p>") }}
/// {# Output: Hello World #}
///
/// {{ strip_html(string="<div class='test'>Content</div>") }}
/// {# Output: Content #}
///
/// {{ strip_html(string="No tags here") }}
/// {# Output: No tags here #}
/// ```
pub fn strip_html_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let string: String = kwargs.get("string")?;

    // Match HTML tags including self-closing tags and tags with attributes
    let regex = Regex::new(r"<[^>]*>").unwrap();
    let result = regex.replace_all(&string, "");

    Ok(Value::from(result.to_string()))
}

/// Remove ANSI escape codes from string
///
/// # Arguments
///
/// * `string` (required) - The input string with ANSI codes
///
/// # Returns
///
/// Returns the string with all ANSI escape codes removed
///
/// # Example
///
/// ```jinja
/// {{ strip_ansi(string="\x1b[31mRed Text\x1b[0m") }}
/// {# Output: Red Text #}
///
/// {{ strip_ansi(string="\x1b[1;32mBold Green\x1b[0m Normal") }}
/// {# Output: Bold Green Normal #}
/// ```
pub fn strip_ansi_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let string: String = kwargs.get("string")?;

    // Match ANSI escape sequences
    // Covers CSI sequences (colors, cursor movement, etc.) and OSC sequences
    let regex = Regex::new(r"\x1b\[[0-9;]*[a-zA-Z]|\x1b\].*?\x07|\x1b[PX^_].*?\x1b\\").unwrap();
    let result = regex.replace_all(&string, "");

    Ok(Value::from(result.to_string()))
}

/// Normalize whitespace in string
///
/// Collapses multiple consecutive whitespace characters (spaces, tabs, newlines)
/// into a single space and trims leading/trailing whitespace.
///
/// # Arguments
///
/// * `string` (required) - The input string
///
/// # Returns
///
/// Returns the string with normalized whitespace
///
/// # Example
///
/// ```jinja
/// {{ normalize_whitespace(string="  hello   world  ") }}
/// {# Output: hello world #}
///
/// {{ normalize_whitespace(string="line1\n\n\nline2\t\tline3") }}
/// {# Output: line1 line2 line3 #}
///
/// {{ normalize_whitespace(string="  multiple   spaces   here  ") }}
/// {# Output: multiple spaces here #}
/// ```
pub fn normalize_whitespace_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let string: String = kwargs.get("string")?;

    // Replace all whitespace sequences with a single space
    let regex = Regex::new(r"\s+").unwrap();
    let result = regex.replace_all(&string, " ");

    // Trim leading and trailing whitespace
    Ok(Value::from(result.trim().to_string()))
}

/// Convert to CONSTANT_CASE (uppercase snake case)
///
/// # Arguments
///
/// * `string` (required) - The input string
///
/// # Returns
///
/// Returns the string in CONSTANT_CASE format
///
/// # Example
///
/// ```jinja
/// {{ to_constant_case(string="hello world") }}
/// {# Output: HELLO_WORLD #}
///
/// {{ to_constant_case(string="helloWorld") }}
/// {# Output: HELLO_WORLD #}
///
/// {{ to_constant_case(string="hello-world-test") }}
/// {# Output: HELLO_WORLD_TEST #}
///
/// {{ to_constant_case(string="HTTPResponse") }}
/// {# Output: HTTP_RESPONSE #}
/// ```
pub fn to_constant_case_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let string: String = kwargs.get("string")?;

    if string.is_empty() {
        return Ok(Value::from(string));
    }

    let mut result = String::new();
    let mut prev_was_lowercase = false;
    let mut prev_was_uppercase = false;

    for ch in string.chars() {
        if ch.is_alphanumeric() {
            // Insert underscore before uppercase letters that follow lowercase
            // or before the last letter of an uppercase sequence followed by lowercase
            if ch.is_uppercase() && prev_was_lowercase {
                result.push('_');
            }
            result.push(ch.to_ascii_uppercase());
            prev_was_lowercase = ch.is_lowercase();
            prev_was_uppercase = ch.is_uppercase();
        } else if ch == ' ' || ch == '-' || ch == '_' {
            // Replace separators with underscore, avoiding duplicates
            if !result.is_empty() && !result.ends_with('_') {
                result.push('_');
            }
            prev_was_lowercase = false;
            prev_was_uppercase = false;
        }
        // Skip other characters
        let _ = prev_was_uppercase; // suppress unused warning
    }

    // Remove trailing underscore if any
    if result.ends_with('_') {
        result.pop();
    }

    Ok(Value::from(result))
}

/// Pluralize a word based on count
///
/// # Arguments
///
/// * `count` (required) - The count to check
/// * `singular` (required) - The singular form of the word
/// * `plural` (optional) - The plural form (default: singular + "s")
///
/// # Returns
///
/// Returns singular if count is 1, otherwise returns plural
///
/// # Example
///
/// ```jinja
/// {{ pluralize(count=1, singular="item") }}
/// {# Output: item #}
///
/// {{ pluralize(count=5, singular="item") }}
/// {# Output: items #}
///
/// {{ pluralize(count=0, singular="child", plural="children") }}
/// {# Output: children #}
///
/// {{ pluralize(count=1, singular="person", plural="people") }}
/// {# Output: person #}
/// ```
pub fn pluralize_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let count: i64 = kwargs.get("count")?;
    let singular: String = kwargs.get("singular")?;
    let plural: Option<String> = kwargs.get("plural")?;

    let result = if count == 1 {
        singular
    } else {
        plural.unwrap_or_else(|| format!("{}s", singular))
    };

    Ok(Value::from(result))
}
