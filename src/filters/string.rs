/// String manipulation filters for MiniJinja templates
use minijinja::Value;

/// Slugify a string - convert to lowercase, replace spaces with hyphens, remove special chars
///
/// # Arguments
///
/// * `value` - The string to slugify
///
/// # Example
///
/// ```jinja
/// {{ "Hello World!" | slugify }}  => "hello-world"
/// {{ "jane smith" | slugify }}  => "jane-smith"
/// ```
pub fn slugify_filter(value: &Value) -> Result<String, minijinja::Error> {
    let s = value.as_str().ok_or_else(|| {
        minijinja::Error::new(
            minijinja::ErrorKind::InvalidOperation,
            "slugify filter requires a string",
        )
    })?;

    let slug = s
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
        .join("-");

    Ok(slug)
}

/// Indent text by N spaces
///
/// # Arguments
///
/// * `value` - The string to indent
/// * `spaces` - Number of spaces to indent (default: 4)
///
/// # Example
///
/// ```jinja
/// {{ "line1\nline2" | indent(2) }}  => "  line1\n  line2"
/// {{ "text" | indent }}  => "    text"
/// ```
pub fn indent_filter(value: &Value, spaces: Option<usize>) -> Result<String, minijinja::Error> {
    let s = value.as_str().ok_or_else(|| {
        minijinja::Error::new(
            minijinja::ErrorKind::InvalidOperation,
            "indent filter requires a string",
        )
    })?;

    let indent_count = spaces.unwrap_or(4);
    let indent_str = " ".repeat(indent_count);

    let result = s
        .lines()
        .map(|line| {
            if line.is_empty() {
                line.to_string()
            } else {
                format!("{}{}", indent_str, line)
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    Ok(result)
}

/// Remove common leading whitespace from all lines
///
/// # Arguments
///
/// * `value` - The string to dedent
///
/// # Example
///
/// ```jinja
/// {{ "  line1\n  line2" | dedent }}  => "line1\nline2"
/// ```
pub fn dedent_filter(value: &Value) -> Result<String, minijinja::Error> {
    let s = value.as_str().ok_or_else(|| {
        minijinja::Error::new(
            minijinja::ErrorKind::InvalidOperation,
            "dedent filter requires a string",
        )
    })?;

    let lines: Vec<&str> = s.lines().collect();
    if lines.is_empty() {
        return Ok(String::new());
    }

    // Find minimum indentation (ignoring empty lines)
    let min_indent = lines
        .iter()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.chars().take_while(|c| c.is_whitespace()).count())
        .min()
        .unwrap_or(0);

    // Remove that many characters from each line
    let result = lines
        .iter()
        .map(|line| {
            if line.len() >= min_indent {
                &line[min_indent..]
            } else {
                line
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    Ok(result)
}

/// Quote a string with the specified quote style
///
/// # Arguments
///
/// * `value` - The string to quote
/// * `style` - Quote style: "single", "double", or "backtick" (default: "double")
///
/// # Example
///
/// ```jinja
/// {{ "hello" | quote }}  => "\"hello\""
/// {{ "hello" | quote("single") }}  => "'hello'"
/// {{ "hello" | quote("backtick") }}  => "`hello`"
/// ```
pub fn quote_filter(value: &Value, style: Option<String>) -> Result<String, minijinja::Error> {
    let s = value.as_str().ok_or_else(|| {
        minijinja::Error::new(
            minijinja::ErrorKind::InvalidOperation,
            "quote filter requires a string",
        )
    })?;

    let quote_style = style.as_deref().unwrap_or("double");

    let result = match quote_style {
        "single" => format!("'{}'", s),
        "double" => format!("\"{}\"", s),
        "backtick" => format!("`{}`", s),
        _ => {
            return Err(minijinja::Error::new(
                minijinja::ErrorKind::InvalidOperation,
                format!(
                    "Invalid quote style '{}'. Use 'single', 'double', or 'backtick'",
                    quote_style
                ),
            ));
        }
    };

    Ok(result)
}

/// Escape quotes in a string
///
/// # Arguments
///
/// * `value` - The string to escape
///
/// # Example
///
/// ```jinja
/// {{ "It's a \"test\"" | escape_quotes }}  => "It\\'s a \\\"test\\\""
/// ```
pub fn escape_quotes_filter(value: &Value) -> Result<String, minijinja::Error> {
    let s = value.as_str().ok_or_else(|| {
        minijinja::Error::new(
            minijinja::ErrorKind::InvalidOperation,
            "escape_quotes filter requires a string",
        )
    })?;

    let result = s
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\'', "\\'");

    Ok(result)
}

/// Convert string to snake_case
///
/// # Arguments
///
/// * `value` - The string to convert
///
/// # Example
///
/// ```jinja
/// {{ "HelloWorld" | to_snake_case }}  => "hello_world"
/// {{ "hello-world" | to_snake_case }}  => "hello_world"
/// ```
pub fn to_snake_case_filter(value: &Value) -> Result<String, minijinja::Error> {
    let s = value.as_str().ok_or_else(|| {
        minijinja::Error::new(
            minijinja::ErrorKind::InvalidOperation,
            "to_snake_case filter requires a string",
        )
    })?;

    let mut result = String::new();
    let mut prev_is_lower = false;

    for (i, c) in s.chars().enumerate() {
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

    Ok(result)
}

/// Convert string to camelCase
///
/// # Arguments
///
/// * `value` - The string to convert
///
/// # Example
///
/// ```jinja
/// {{ "hello_world" | to_camel_case }}  => "helloWorld"
/// {{ "hello-world" | to_camel_case }}  => "helloWorld"
/// ```
pub fn to_camel_case_filter(value: &Value) -> Result<String, minijinja::Error> {
    let s = value.as_str().ok_or_else(|| {
        minijinja::Error::new(
            minijinja::ErrorKind::InvalidOperation,
            "to_camel_case filter requires a string",
        )
    })?;

    let mut result = String::new();
    let mut capitalize_next = false;
    let mut first_char = true;

    for c in s.chars() {
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

    Ok(result)
}

/// Convert string to PascalCase
///
/// # Arguments
///
/// * `value` - The string to convert
///
/// # Example
///
/// ```jinja
/// {{ "hello_world" | to_pascal_case }}  => "HelloWorld"
/// {{ "hello-world" | to_pascal_case }}  => "HelloWorld"
/// ```
pub fn to_pascal_case_filter(value: &Value) -> Result<String, minijinja::Error> {
    let s = value.as_str().ok_or_else(|| {
        minijinja::Error::new(
            minijinja::ErrorKind::InvalidOperation,
            "to_pascal_case filter requires a string",
        )
    })?;

    let mut result = String::new();
    let mut capitalize_next = true;

    for c in s.chars() {
        if c == '_' || c == '-' || c == ' ' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_uppercase().next().unwrap());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }

    Ok(result)
}

/// Convert string to kebab-case
///
/// # Arguments
///
/// * `value` - The string to convert
///
/// # Example
///
/// ```jinja
/// {{ "HelloWorld" | to_kebab_case }}  => "hello-world"
/// {{ "hello_world" | to_kebab_case }}  => "hello-world"
/// ```
pub fn to_kebab_case_filter(value: &Value) -> Result<String, minijinja::Error> {
    let s = value.as_str().ok_or_else(|| {
        minijinja::Error::new(
            minijinja::ErrorKind::InvalidOperation,
            "to_kebab_case filter requires a string",
        )
    })?;

    let mut result = String::new();
    let mut prev_is_lower = false;

    for (i, c) in s.chars().enumerate() {
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

    Ok(result)
}

/// Pad string on the left to a minimum length
///
/// # Arguments
///
/// * `value` - The string to pad
/// * `length` - Target minimum length
/// * `pad_char` - Character to pad with (default: space)
///
/// # Example
///
/// ```jinja
/// {{ "5" | pad_left(3, "0") }}  => "005"
/// {{ "hi" | pad_left(5) }}  => "   hi"
/// ```
pub fn pad_left_filter(
    value: &Value,
    length: usize,
    pad_char: Option<String>,
) -> Result<String, minijinja::Error> {
    let s = value.as_str().ok_or_else(|| {
        minijinja::Error::new(
            minijinja::ErrorKind::InvalidOperation,
            "pad_left filter requires a string",
        )
    })?;

    let pad_str = pad_char.as_deref().unwrap_or(" ");
    let pad_ch = pad_str.chars().next().unwrap_or(' ');

    let current_len = s.chars().count();
    if current_len >= length {
        return Ok(s.to_string());
    }

    let padding = pad_ch.to_string().repeat(length - current_len);
    Ok(format!("{}{}", padding, s))
}

/// Pad string on the right to a minimum length
///
/// # Arguments
///
/// * `value` - The string to pad
/// * `length` - Target minimum length
/// * `pad_char` - Character to pad with (default: space)
///
/// # Example
///
/// ```jinja
/// {{ "5" | pad_right(3, "0") }}  => "500"
/// {{ "hi" | pad_right(5) }}  => "hi   "
/// ```
pub fn pad_right_filter(
    value: &Value,
    length: usize,
    pad_char: Option<String>,
) -> Result<String, minijinja::Error> {
    let s = value.as_str().ok_or_else(|| {
        minijinja::Error::new(
            minijinja::ErrorKind::InvalidOperation,
            "pad_right filter requires a string",
        )
    })?;

    let pad_str = pad_char.as_deref().unwrap_or(" ");
    let pad_ch = pad_str.chars().next().unwrap_or(' ');

    let current_len = s.chars().count();
    if current_len >= length {
        return Ok(s.to_string());
    }

    let padding = pad_ch.to_string().repeat(length - current_len);
    Ok(format!("{}{}", s, padding))
}

/// Repeat string N times
///
/// # Arguments
///
/// * `value` - The string to repeat
/// * `count` - Number of times to repeat
///
/// # Example
///
/// ```jinja
/// {{ "ab" | repeat(3) }}  => "ababab"
/// {{ "-" | repeat(5) }}  => "-----"
/// ```
pub fn repeat_filter(value: &Value, count: usize) -> Result<String, minijinja::Error> {
    let s = value.as_str().ok_or_else(|| {
        minijinja::Error::new(
            minijinja::ErrorKind::InvalidOperation,
            "repeat filter requires a string",
        )
    })?;

    Ok(s.repeat(count))
}

/// Reverse a string
///
/// # Arguments
///
/// * `value` - The string to reverse
///
/// # Example
///
/// ```jinja
/// {{ "hello" | reverse }}  => "olleh"
/// {{ "12345" | reverse }}  => "54321"
/// ```
pub fn reverse_filter(value: &Value) -> Result<String, minijinja::Error> {
    let s = value.as_str().ok_or_else(|| {
        minijinja::Error::new(
            minijinja::ErrorKind::InvalidOperation,
            "reverse filter requires a string",
        )
    })?;

    Ok(s.chars().rev().collect())
}
