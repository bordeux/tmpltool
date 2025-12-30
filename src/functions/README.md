# Custom Tera Functions

This directory contains all custom functions that can be used in Tera templates.

## Philosophy

Each function is in its own file for:
- **Better organization** - Easy to find specific functions
- **Focused development** - Work on one function at a time
- **Easy testing** - Each function has its own test suite
- **Simple additions** - Adding new functions is straightforward

## Current Functions

### `env()` - Environment Variable with Defaults

**File:** `env.rs`

Get environment variables with optional default values.

**Syntax:**
```tera
{{ env(name="VARIABLE_NAME", default="fallback_value") }}
```

**Examples:**
```tera
port = {{ env(name="PORT", default="8080") }}
database = {{ env(name="DB_URL", default="postgres://localhost/mydb") }}
```

## Adding a New Function

Follow these steps to add a new custom function:

### 1. Create the Function File

Create a new file in `src/functions/` (e.g., `uppercase.rs`):

```rust
use std::collections::HashMap;
use tera::Value;

/// Converts a string to uppercase
///
/// # Arguments
///
/// * `args` - HashMap containing function arguments
///   - `value` (required): The string to convert
///
/// # Example
///
/// ```tera
/// {{ uppercase(value="hello") }}
/// ```
pub fn uppercase_function(args: &HashMap<String, Value>) -> tera::Result<Value> {
    let value = args
        .get("value")
        .and_then(|v| v.as_str())
        .ok_or_else(|| tera::Error::msg("uppercase() requires a 'value' argument"))?;

    Ok(Value::String(value.to_uppercase()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uppercase() {
        let mut args = HashMap::new();
        args.insert("value".to_string(), Value::String("hello".to_string()));

        let result = uppercase_function(&args);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str().unwrap(), "HELLO");
    }
}
```

### 2. Declare the Module

Add to `src/functions/mod.rs`:

```rust
pub mod uppercase;
```

### 3. Register the Function

Add to the `register_all()` function in `src/functions/mod.rs`:

```rust
pub fn register_all(tera: &mut Tera) {
    tera.register_function("env", env::env_function);
    tera.register_function("uppercase", uppercase::uppercase_function);
    // Add more here...
}
```

### 4. Write Tests

Add tests in your function file (as shown in step 1).

### 5. Test Everything

```bash
cargo test
```

### 6. Document in README

Add your function to the main `README.md` in the Template Syntax section.

## Function Signature

All Tera custom functions must follow this signature:

```rust
pub fn my_function(args: &HashMap<String, Value>) -> tera::Result<Value>
```

**Parameters:**
- `args` - HashMap of named arguments passed from the template

**Returns:**
- `tera::Result<Value>` - Either the result value or an error

## Best Practices

1. **Always validate arguments** - Check for required arguments
2. **Provide helpful error messages** - Users should know what went wrong
3. **Write comprehensive tests** - Test both success and error cases
4. **Document your function** - Add doc comments and examples
5. **Keep it focused** - One function should do one thing well

## Example Function Ideas

Here are some ideas for useful functions you could add:

- `file_read(path="...")` - Read file contents
- `json_parse(value="...")` - Parse JSON string
- `base64_encode(value="...")` - Base64 encoding
- `random(min=0, max=100)` - Generate random numbers
- `timestamp()` - Current Unix timestamp
- `format_date(format="...", value="...")` - Date formatting
- `regex_match(pattern="...", value="...")` - Regular expression matching
- `hash(value="...", algorithm="sha256")` - Cryptographic hashing

## Resources

- [Tera Documentation](https://keats.github.io/tera/docs/)
- [Tera Functions Reference](https://keats.github.io/tera/docs/#functions)
- Main project README: `../../README.md`
