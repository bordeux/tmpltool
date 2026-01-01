//! String manipulation functions for MiniJinja templates
//!
//! This module provides utility functions for working with strings:
//! - Regex operations (replace, match, find)
//! - Substring extraction
//! - String searching (contains, index_of, count)
//! - Text processing (truncate, word_count, split_lines)

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
