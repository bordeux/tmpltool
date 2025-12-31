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
