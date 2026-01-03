//! String functions that support both function and filter syntax.
//!
//! # Function Syntax
//! ```jinja
//! {{ regex_replace(string="hello123", pattern="[0-9]+", replacement="-") }}
//! {{ truncate(string="Hello World", length=8) }}
//! {{ word_count(string="Hello World") }}
//! ```
//!
//! # Filter Syntax
//! ```jinja
//! {{ "hello123" | regex_replace(pattern="[0-9]+", replacement="-") }}
//! {{ "Hello World" | truncate(length=8) }}
//! {{ "Hello World" | word_count }}
//! ```
//!
//! # Chaining
//! ```jinja
//! {{ text | strip_html | normalize_whitespace | truncate(length=100) }}
//! {{ content | word_count }}
//! ```

use super::FilterFunction;
use minijinja::value::Kwargs;
use minijinja::{Environment, Error, ErrorKind, Value};
use regex::Regex;

/// Helper to extract string from Value
fn extract_string(value: &Value, fn_name: &str) -> Result<String, Error> {
    value.as_str().map(|s| s.to_string()).ok_or_else(|| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("{} requires a string value", fn_name),
        )
    })
}

// ============================================
// RegexReplace
// ============================================

/// Replace substrings using regex pattern.
///
/// # Function Syntax
/// ```jinja
/// {{ regex_replace(string="hello123world", pattern="[0-9]+", replacement="-") }}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ "hello123world" | regex_replace(pattern="[0-9]+", replacement="-") }}
/// {{ text | regex_replace(pattern="\\s+", replacement="_") }}
/// ```
pub struct RegexReplace;

impl RegexReplace {
    fn compute(input: &str, pattern: &str, replacement: &str) -> Result<String, Error> {
        let regex = Regex::new(pattern).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Invalid regex pattern '{}': {}", pattern, e),
            )
        })?;
        Ok(regex.replace_all(input, replacement).to_string())
    }
}

impl FilterFunction for RegexReplace {
    const NAME: &'static str = "regex_replace";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let string: String = kwargs.get("string")?;
        let pattern: String = kwargs.get("pattern")?;
        let replacement: String = kwargs.get("replacement")?;
        Ok(Value::from(Self::compute(&string, &pattern, &replacement)?))
    }

    fn call_as_filter(value: &Value, kwargs: Kwargs) -> Result<Value, Error> {
        let string = extract_string(value, "regex_replace")?;
        let pattern: String = kwargs.get("pattern")?;
        let replacement: String = kwargs.get("replacement")?;
        Ok(Value::from(Self::compute(&string, &pattern, &replacement)?))
    }
}

// ============================================
// Substring
// ============================================

/// Extract substring by position.
///
/// # Function Syntax
/// ```jinja
/// {{ substring(string="hello world", start=0, length=5) }}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ "hello world" | substring(start=0, length=5) }}
/// {{ text | substring(start=-5) }}
/// ```
pub struct Substring;

impl Substring {
    fn compute(input: &str, start: i64, length: Option<usize>) -> String {
        let chars: Vec<char> = input.chars().collect();
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

        chars[start_idx..end_idx].iter().collect()
    }
}

impl FilterFunction for Substring {
    const NAME: &'static str = "substring";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let string: String = kwargs.get("string")?;
        let start: i64 = kwargs.get("start")?;
        let length: Option<usize> = kwargs.get("length")?;
        Ok(Value::from(Self::compute(&string, start, length)))
    }

    fn call_as_filter(value: &Value, kwargs: Kwargs) -> Result<Value, Error> {
        let string = extract_string(value, "substring")?;
        let start: i64 = kwargs.get("start")?;
        let length: Option<usize> = kwargs.get("length")?;
        Ok(Value::from(Self::compute(&string, start, length)))
    }
}

// ============================================
// Truncate
// ============================================

/// Truncate string with suffix.
///
/// # Function Syntax
/// ```jinja
/// {{ truncate(string="Hello World", length=8) }}
/// {{ truncate(string="Hello World", length=8, suffix=">>") }}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ "Hello World" | truncate(length=8) }}
/// {{ text | truncate(length=100, suffix="...") }}
/// ```
pub struct Truncate;

impl Truncate {
    fn compute(input: &str, length: usize, suffix: &str) -> String {
        let chars: Vec<char> = input.chars().collect();

        if chars.len() <= length {
            return input.to_string();
        }

        let suffix_len = suffix.chars().count();
        if length <= suffix_len {
            return suffix.to_string();
        }

        let truncate_at = length - suffix_len;
        chars[..truncate_at].iter().collect::<String>() + suffix
    }
}

impl FilterFunction for Truncate {
    const NAME: &'static str = "truncate";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let string: String = kwargs.get("string")?;
        let length: usize = kwargs.get("length")?;
        let suffix: String = kwargs.get("suffix").unwrap_or_else(|_| "...".to_string());
        Ok(Value::from(Self::compute(&string, length, &suffix)))
    }

    fn call_as_filter(value: &Value, kwargs: Kwargs) -> Result<Value, Error> {
        let string = extract_string(value, "truncate")?;
        let length: usize = kwargs.get("length")?;
        let suffix: String = kwargs.get("suffix").unwrap_or_else(|_| "...".to_string());
        Ok(Value::from(Self::compute(&string, length, &suffix)))
    }
}

// ============================================
// WordCount
// ============================================

/// Count words in string.
///
/// # Function Syntax
/// ```jinja
/// {{ word_count(string="Hello World") }}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ "Hello World" | word_count }}
/// {{ text | word_count }}
/// ```
pub struct WordCount;

impl WordCount {
    fn compute(input: &str) -> usize {
        input.split_whitespace().count()
    }
}

impl FilterFunction for WordCount {
    const NAME: &'static str = "word_count";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let string: String = kwargs.get("string")?;
        Ok(Value::from(Self::compute(&string)))
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        let string = extract_string(value, "word_count")?;
        Ok(Value::from(Self::compute(&string)))
    }
}

// ============================================
// SplitLines
// ============================================

/// Split string into lines array.
///
/// # Function Syntax
/// ```jinja
/// {{ split_lines(string=text) }}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ text | split_lines }}
/// ```
pub struct SplitLines;

impl SplitLines {
    fn compute(input: &str) -> Vec<String> {
        input.lines().map(|s| s.to_string()).collect()
    }
}

impl FilterFunction for SplitLines {
    const NAME: &'static str = "split_lines";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let string: String = kwargs.get("string")?;
        Ok(Value::from_serialize(Self::compute(&string)))
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        let string = extract_string(value, "split_lines")?;
        Ok(Value::from_serialize(Self::compute(&string)))
    }
}

// ============================================
// Wrap
// ============================================

/// Word wrap text at specified width.
///
/// # Function Syntax
/// ```jinja
/// {{ wrap(string="The quick brown fox", width=10) }}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ "The quick brown fox" | wrap(width=10) }}
/// {{ text | wrap(width=80, indent="  ") }}
/// ```
pub struct Wrap;

impl Wrap {
    fn compute(input: &str, width: usize, indent: &str) -> Result<String, Error> {
        if width == 0 {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                "width must be greater than 0",
            ));
        }

        let mut result = String::new();
        let mut first_line = true;

        for line in input.lines() {
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
                    current_line.push_str(word);
                    current_width = word_len;
                } else if current_width + 1 + word_len <= effective_width {
                    current_line.push(' ');
                    current_line.push_str(word);
                    current_width += 1 + word_len;
                } else {
                    if !result.is_empty() {
                        result.push('\n');
                    }
                    if !first_line {
                        result.push_str(indent);
                    }
                    result.push_str(&current_line);
                    first_line = false;

                    current_line = word.to_string();
                    current_width = word_len;
                }
            }

            if !current_line.is_empty() {
                if !result.is_empty() {
                    result.push('\n');
                }
                if !first_line {
                    result.push_str(indent);
                }
                result.push_str(&current_line);
                first_line = false;
            }
        }

        Ok(result)
    }
}

impl FilterFunction for Wrap {
    const NAME: &'static str = "wrap";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let string: String = kwargs.get("string")?;
        let width: usize = kwargs.get("width")?;
        let indent: String = kwargs.get("indent").unwrap_or_default();
        Ok(Value::from(Self::compute(&string, width, &indent)?))
    }

    fn call_as_filter(value: &Value, kwargs: Kwargs) -> Result<Value, Error> {
        let string = extract_string(value, "wrap")?;
        let width: usize = kwargs.get("width")?;
        let indent: String = kwargs.get("indent").unwrap_or_default();
        Ok(Value::from(Self::compute(&string, width, &indent)?))
    }
}

// ============================================
// Center
// ============================================

/// Center text with padding.
///
/// # Function Syntax
/// ```jinja
/// {{ center(string="hello", width=11) }}
/// {{ center(string="hi", width=10, char="-") }}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ "hello" | center(width=11) }}
/// {{ title | center(width=40, char="=") }}
/// ```
pub struct Center;

impl Center {
    fn compute(input: &str, width: usize, pad_char: char) -> String {
        let str_len = input.chars().count();

        if str_len >= width {
            return input.to_string();
        }

        let total_padding = width - str_len;
        let left_padding = total_padding / 2;
        let right_padding = total_padding - left_padding;

        format!(
            "{}{}{}",
            pad_char.to_string().repeat(left_padding),
            input,
            pad_char.to_string().repeat(right_padding)
        )
    }
}

impl FilterFunction for Center {
    const NAME: &'static str = "center";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let string: String = kwargs.get("string")?;
        let width: usize = kwargs.get("width")?;
        let pad_char: Option<String> = kwargs.get("char")?;
        let pad_char = pad_char.and_then(|s| s.chars().next()).unwrap_or(' ');
        Ok(Value::from(Self::compute(&string, width, pad_char)))
    }

    fn call_as_filter(value: &Value, kwargs: Kwargs) -> Result<Value, Error> {
        let string = extract_string(value, "center")?;
        let width: usize = kwargs.get("width")?;
        let pad_char: Option<String> = kwargs.get("char")?;
        let pad_char = pad_char.and_then(|s| s.chars().next()).unwrap_or(' ');
        Ok(Value::from(Self::compute(&string, width, pad_char)))
    }
}

// ============================================
// StripHtml
// ============================================

/// Remove HTML tags from string.
///
/// # Function Syntax
/// ```jinja
/// {{ strip_html(string="<p>Hello <b>World</b></p>") }}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ "<p>Hello <b>World</b></p>" | strip_html }}
/// {{ html_content | strip_html }}
/// ```
pub struct StripHtml;

impl StripHtml {
    fn compute(input: &str) -> String {
        let regex = Regex::new(r"<[^>]*>").unwrap();
        regex.replace_all(input, "").to_string()
    }
}

impl FilterFunction for StripHtml {
    const NAME: &'static str = "strip_html";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let string: String = kwargs.get("string")?;
        Ok(Value::from(Self::compute(&string)))
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        let string = extract_string(value, "strip_html")?;
        Ok(Value::from(Self::compute(&string)))
    }
}

// ============================================
// StripAnsi
// ============================================

/// Remove ANSI escape codes from string.
///
/// # Function Syntax
/// ```jinja
/// {{ strip_ansi(string=colored_text) }}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ colored_text | strip_ansi }}
/// ```
pub struct StripAnsi;

impl StripAnsi {
    fn compute(input: &str) -> String {
        let regex = Regex::new(r"\x1b\[[0-9;]*[a-zA-Z]|\x1b\].*?\x07|\x1b[PX^_].*?\x1b\\").unwrap();
        regex.replace_all(input, "").to_string()
    }
}

impl FilterFunction for StripAnsi {
    const NAME: &'static str = "strip_ansi";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let string: String = kwargs.get("string")?;
        Ok(Value::from(Self::compute(&string)))
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        let string = extract_string(value, "strip_ansi")?;
        Ok(Value::from(Self::compute(&string)))
    }
}

// ============================================
// NormalizeWhitespace
// ============================================

/// Normalize whitespace in string.
///
/// Collapses multiple consecutive whitespace characters into a single space
/// and trims leading/trailing whitespace.
///
/// # Function Syntax
/// ```jinja
/// {{ normalize_whitespace(string="  hello   world  ") }}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ "  hello   world  " | normalize_whitespace }}
/// {{ text | normalize_whitespace }}
/// ```
pub struct NormalizeWhitespace;

impl NormalizeWhitespace {
    fn compute(input: &str) -> String {
        let regex = Regex::new(r"\s+").unwrap();
        regex.replace_all(input, " ").trim().to_string()
    }
}

impl FilterFunction for NormalizeWhitespace {
    const NAME: &'static str = "normalize_whitespace";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let string: String = kwargs.get("string")?;
        Ok(Value::from(Self::compute(&string)))
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        let string = extract_string(value, "normalize_whitespace")?;
        Ok(Value::from(Self::compute(&string)))
    }
}

// ============================================
// Slugify (migrated from filters)
// ============================================

/// Convert string to URL-friendly slug.
///
/// # Function Syntax
/// ```jinja
/// {{ slugify(string="Hello World!") }}
/// {# Output: hello-world #}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ "Hello World!" | slugify }}
/// {{ title | slugify }}
/// ```
pub struct Slugify;

impl Slugify {
    fn compute(input: &str) -> String {
        input
            .to_lowercase()
            .chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() {
                    c
                } else if c.is_whitespace() || c == '-' || c == '_' {
                    '-'
                } else {
                    '\0' // Will be filtered out
                }
            })
            .filter(|&c| c != '\0')
            .collect::<String>()
            // Remove duplicate hyphens
            .split('-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("-")
    }
}

impl FilterFunction for Slugify {
    const NAME: &'static str = "slugify";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let string: String = kwargs.get("string")?;
        Ok(Value::from(Self::compute(&string)))
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        let string = extract_string(value, "slugify")?;
        Ok(Value::from(Self::compute(&string)))
    }
}

// ============================================
// Indent (migrated from filters)
// ============================================

/// Indent text by N spaces.
///
/// # Function Syntax
/// ```jinja
/// {{ indent(string="line1\nline2", spaces=2) }}
/// {# Output: "  line1\n  line2" #}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ "text" | indent }}
/// {{ "text" | indent(spaces=2) }}
/// ```
pub struct Indent;

impl Indent {
    fn compute(input: &str, spaces: usize) -> String {
        let indent_str = " ".repeat(spaces);

        input
            .lines()
            .map(|line| {
                if line.is_empty() {
                    line.to_string()
                } else {
                    format!("{}{}", indent_str, line)
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl FilterFunction for Indent {
    const NAME: &'static str = "indent";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let string: String = kwargs.get("string")?;
        let spaces: usize = kwargs.get("spaces").unwrap_or(4);
        Ok(Value::from(Self::compute(&string, spaces)))
    }

    fn call_as_filter(value: &Value, kwargs: Kwargs) -> Result<Value, Error> {
        let string = extract_string(value, "indent")?;
        let spaces: usize = kwargs.get("spaces").unwrap_or(4);
        Ok(Value::from(Self::compute(&string, spaces)))
    }

    /// Custom registration to support positional argument: `| indent(4)`
    fn register(env: &mut Environment) {
        env.add_function(Self::NAME, Self::call_as_function);
        // Register filter with optional positional argument for spaces
        env.add_filter(
            Self::NAME,
            |value: &Value, spaces: Option<usize>| -> Result<Value, Error> {
                let string = extract_string(value, "indent")?;
                Ok(Value::from(Self::compute(&string, spaces.unwrap_or(4))))
            },
        );
    }
}

// ============================================
// Dedent (migrated from filters)
// ============================================

/// Remove common leading whitespace from all lines.
///
/// # Function Syntax
/// ```jinja
/// {{ dedent(string="  line1\n  line2") }}
/// {# Output: "line1\nline2" #}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ "  line1\n  line2" | dedent }}
/// {{ text | dedent }}
/// ```
pub struct Dedent;

impl Dedent {
    fn compute(input: &str) -> String {
        let lines: Vec<&str> = input.lines().collect();
        if lines.is_empty() {
            return String::new();
        }

        // Find minimum indentation (ignoring empty lines)
        let min_indent = lines
            .iter()
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.chars().take_while(|c| c.is_whitespace()).count())
            .min()
            .unwrap_or(0);

        // Remove that many characters from each line
        lines
            .iter()
            .map(|line| {
                if line.len() >= min_indent {
                    &line[min_indent..]
                } else {
                    line
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl FilterFunction for Dedent {
    const NAME: &'static str = "dedent";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let string: String = kwargs.get("string")?;
        Ok(Value::from(Self::compute(&string)))
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        let string = extract_string(value, "dedent")?;
        Ok(Value::from(Self::compute(&string)))
    }
}

// ============================================
// Quote (migrated from filters)
// ============================================

/// Quote a string with the specified style.
///
/// # Function Syntax
/// ```jinja
/// {{ quote(string="hello") }}
/// {# Output: "hello" #}
///
/// {{ quote(string="hello", style="single") }}
/// {# Output: 'hello' #}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ "hello" | quote }}
/// {{ "hello" | quote(style="single") }}
/// {{ "hello" | quote(style="backtick") }}
/// ```
pub struct Quote;

impl Quote {
    fn compute(input: &str, style: &str) -> Result<String, Error> {
        match style {
            "single" => Ok(format!("'{}'", input)),
            "double" => Ok(format!("\"{}\"", input)),
            "backtick" => Ok(format!("`{}`", input)),
            _ => Err(Error::new(
                ErrorKind::InvalidOperation,
                format!(
                    "Invalid quote style '{}'. Use 'single', 'double', or 'backtick'",
                    style
                ),
            )),
        }
    }
}

impl FilterFunction for Quote {
    const NAME: &'static str = "quote";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let string: String = kwargs.get("string")?;
        let style: String = kwargs.get("style").unwrap_or_else(|_| "double".to_string());
        Ok(Value::from(Self::compute(&string, &style)?))
    }

    fn call_as_filter(value: &Value, kwargs: Kwargs) -> Result<Value, Error> {
        let string = extract_string(value, "quote")?;
        let style: String = kwargs.get("style").unwrap_or_else(|_| "double".to_string());
        Ok(Value::from(Self::compute(&string, &style)?))
    }
}

// ============================================
// EscapeQuotes (migrated from filters)
// ============================================

/// Escape quotes in a string.
///
/// # Function Syntax
/// ```jinja
/// {{ escape_quotes(string="It's a \"test\"") }}
/// {# Output: It\'s a \"test\" #}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ "It's a \"test\"" | escape_quotes }}
/// ```
pub struct EscapeQuotes;

impl EscapeQuotes {
    fn compute(input: &str) -> String {
        input
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\'', "\\'")
    }
}

impl FilterFunction for EscapeQuotes {
    const NAME: &'static str = "escape_quotes";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let string: String = kwargs.get("string")?;
        Ok(Value::from(Self::compute(&string)))
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        let string = extract_string(value, "escape_quotes")?;
        Ok(Value::from(Self::compute(&string)))
    }
}

// ============================================
// ToSnakeCase (migrated from filters)
// ============================================

/// Convert string to snake_case.
///
/// # Function Syntax
/// ```jinja
/// {{ to_snake_case(string="HelloWorld") }}
/// {# Output: hello_world #}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ "HelloWorld" | to_snake_case }}
/// {{ "hello-world" | to_snake_case }}
/// ```
pub struct ToSnakeCase;

impl ToSnakeCase {
    fn compute(input: &str) -> String {
        let mut result = String::new();
        let mut prev_is_lower = false;

        for (i, c) in input.chars().enumerate() {
            if c.is_uppercase() {
                if i > 0 && prev_is_lower {
                    result.push('_');
                }
                result.push(c.to_lowercase().next().unwrap());
                prev_is_lower = false;
            } else if c.is_alphanumeric() {
                result.push(c);
                prev_is_lower = c.is_lowercase();
            } else if c == '-' || c == ' ' || c == '_' {
                if !result.is_empty() && !result.ends_with('_') {
                    result.push('_');
                }
                prev_is_lower = false;
            }
        }

        result
    }
}

impl FilterFunction for ToSnakeCase {
    const NAME: &'static str = "to_snake_case";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let string: String = kwargs.get("string")?;
        Ok(Value::from(Self::compute(&string)))
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        let string = extract_string(value, "to_snake_case")?;
        Ok(Value::from(Self::compute(&string)))
    }
}

// ============================================
// ToCamelCase (migrated from filters)
// ============================================

/// Convert string to camelCase.
///
/// # Function Syntax
/// ```jinja
/// {{ to_camel_case(string="hello_world") }}
/// {# Output: helloWorld #}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ "hello_world" | to_camel_case }}
/// {{ "hello-world" | to_camel_case }}
/// ```
pub struct ToCamelCase;

impl ToCamelCase {
    fn compute(input: &str) -> String {
        let mut result = String::new();
        let mut capitalize_next = false;
        let mut first_char = true;

        for c in input.chars() {
            if c == '_' || c == '-' || c == ' ' {
                capitalize_next = true;
            } else if capitalize_next {
                result.push(c.to_uppercase().next().unwrap());
                capitalize_next = false;
                first_char = false;
            } else if first_char {
                result.push(c.to_lowercase().next().unwrap());
                first_char = false;
            } else {
                result.push(c);
            }
        }

        result
    }
}

impl FilterFunction for ToCamelCase {
    const NAME: &'static str = "to_camel_case";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let string: String = kwargs.get("string")?;
        Ok(Value::from(Self::compute(&string)))
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        let string = extract_string(value, "to_camel_case")?;
        Ok(Value::from(Self::compute(&string)))
    }
}

// ============================================
// ToPascalCase (migrated from filters)
// ============================================

/// Convert string to PascalCase.
///
/// # Function Syntax
/// ```jinja
/// {{ to_pascal_case(string="hello_world") }}
/// {# Output: HelloWorld #}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ "hello_world" | to_pascal_case }}
/// {{ "hello-world" | to_pascal_case }}
/// ```
pub struct ToPascalCase;

impl ToPascalCase {
    fn compute(input: &str) -> String {
        let mut result = String::new();
        let mut capitalize_next = true;

        for c in input.chars() {
            if c == '_' || c == '-' || c == ' ' {
                capitalize_next = true;
            } else if capitalize_next {
                result.push(c.to_uppercase().next().unwrap());
                capitalize_next = false;
            } else {
                result.push(c);
            }
        }

        result
    }
}

impl FilterFunction for ToPascalCase {
    const NAME: &'static str = "to_pascal_case";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let string: String = kwargs.get("string")?;
        Ok(Value::from(Self::compute(&string)))
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        let string = extract_string(value, "to_pascal_case")?;
        Ok(Value::from(Self::compute(&string)))
    }
}

// ============================================
// ToKebabCase (migrated from filters)
// ============================================

/// Convert string to kebab-case.
///
/// # Function Syntax
/// ```jinja
/// {{ to_kebab_case(string="HelloWorld") }}
/// {# Output: hello-world #}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ "HelloWorld" | to_kebab_case }}
/// {{ "hello_world" | to_kebab_case }}
/// ```
pub struct ToKebabCase;

impl ToKebabCase {
    fn compute(input: &str) -> String {
        let mut result = String::new();
        let mut prev_is_lower = false;

        for (i, c) in input.chars().enumerate() {
            if c.is_uppercase() {
                if i > 0 && prev_is_lower {
                    result.push('-');
                }
                result.push(c.to_lowercase().next().unwrap());
                prev_is_lower = false;
            } else if c.is_alphanumeric() {
                result.push(c);
                prev_is_lower = c.is_lowercase();
            } else if c == '_' || c == ' ' || c == '-' {
                if !result.is_empty() && !result.ends_with('-') {
                    result.push('-');
                }
                prev_is_lower = false;
            }
        }

        result
    }
}

impl FilterFunction for ToKebabCase {
    const NAME: &'static str = "to_kebab_case";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let string: String = kwargs.get("string")?;
        Ok(Value::from(Self::compute(&string)))
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        let string = extract_string(value, "to_kebab_case")?;
        Ok(Value::from(Self::compute(&string)))
    }
}

// ============================================
// PadLeft (migrated from filters)
// ============================================

/// Pad string on the left to a minimum length.
///
/// # Function Syntax
/// ```jinja
/// {{ pad_left(string="5", length=3, char="0") }}
/// {# Output: 005 #}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ "5" | pad_left(length=3, char="0") }}
/// {{ "hi" | pad_left(length=5) }}
/// ```
pub struct PadLeft;

impl PadLeft {
    fn compute(input: &str, length: usize, pad_char: char) -> String {
        let current_len = input.chars().count();
        if current_len >= length {
            return input.to_string();
        }

        let padding = pad_char.to_string().repeat(length - current_len);
        format!("{}{}", padding, input)
    }
}

impl FilterFunction for PadLeft {
    const NAME: &'static str = "pad_left";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let string: String = kwargs.get("string")?;
        let length: usize = kwargs.get("length")?;
        let pad_char: Option<String> = kwargs.get("char")?;
        let pad_char = pad_char.and_then(|s| s.chars().next()).unwrap_or(' ');
        Ok(Value::from(Self::compute(&string, length, pad_char)))
    }

    fn call_as_filter(value: &Value, kwargs: Kwargs) -> Result<Value, Error> {
        let string = extract_string(value, "pad_left")?;
        let length: usize = kwargs.get("length")?;
        let pad_char: Option<String> = kwargs.get("char")?;
        let pad_char = pad_char.and_then(|s| s.chars().next()).unwrap_or(' ');
        Ok(Value::from(Self::compute(&string, length, pad_char)))
    }
}

// ============================================
// PadRight (migrated from filters)
// ============================================

/// Pad string on the right to a minimum length.
///
/// # Function Syntax
/// ```jinja
/// {{ pad_right(string="5", length=3, char="0") }}
/// {# Output: 500 #}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ "5" | pad_right(length=3, char="0") }}
/// {{ "hi" | pad_right(length=5) }}
/// ```
pub struct PadRight;

impl PadRight {
    fn compute(input: &str, length: usize, pad_char: char) -> String {
        let current_len = input.chars().count();
        if current_len >= length {
            return input.to_string();
        }

        let padding = pad_char.to_string().repeat(length - current_len);
        format!("{}{}", input, padding)
    }
}

impl FilterFunction for PadRight {
    const NAME: &'static str = "pad_right";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let string: String = kwargs.get("string")?;
        let length: usize = kwargs.get("length")?;
        let pad_char: Option<String> = kwargs.get("char")?;
        let pad_char = pad_char.and_then(|s| s.chars().next()).unwrap_or(' ');
        Ok(Value::from(Self::compute(&string, length, pad_char)))
    }

    fn call_as_filter(value: &Value, kwargs: Kwargs) -> Result<Value, Error> {
        let string = extract_string(value, "pad_right")?;
        let length: usize = kwargs.get("length")?;
        let pad_char: Option<String> = kwargs.get("char")?;
        let pad_char = pad_char.and_then(|s| s.chars().next()).unwrap_or(' ');
        Ok(Value::from(Self::compute(&string, length, pad_char)))
    }
}

// ============================================
// Repeat (migrated from filters)
// ============================================

/// Repeat string N times.
///
/// # Function Syntax
/// ```jinja
/// {{ repeat(string="ab", count=3) }}
/// {# Output: ababab #}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ "ab" | repeat(count=3) }}
/// {{ "-" | repeat(count=5) }}
/// ```
pub struct Repeat;

impl Repeat {
    fn compute(input: &str, count: usize) -> String {
        input.repeat(count)
    }
}

impl FilterFunction for Repeat {
    const NAME: &'static str = "repeat";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let string: String = kwargs.get("string")?;
        let count: usize = kwargs.get("count")?;
        Ok(Value::from(Self::compute(&string, count)))
    }

    fn call_as_filter(value: &Value, kwargs: Kwargs) -> Result<Value, Error> {
        let string = extract_string(value, "repeat")?;
        let count: usize = kwargs.get("count")?;
        Ok(Value::from(Self::compute(&string, count)))
    }
}

// ============================================
// Reverse (migrated from filters)
// ============================================

/// Reverse a string.
///
/// # Function Syntax
/// ```jinja
/// {{ reverse(string="hello") }}
/// {# Output: olleh #}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ "hello" | reverse }}
/// {{ "12345" | reverse }}
/// ```
pub struct Reverse;

impl Reverse {
    fn compute(input: &str) -> String {
        input.chars().rev().collect()
    }
}

impl FilterFunction for Reverse {
    const NAME: &'static str = "reverse";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let string: String = kwargs.get("string")?;
        Ok(Value::from(Self::compute(&string)))
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        let string = extract_string(value, "reverse")?;
        Ok(Value::from(Self::compute(&string)))
    }
}
