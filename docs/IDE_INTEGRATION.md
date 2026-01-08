# IDE Integration

The `--ide` flag outputs comprehensive metadata about all available functions, making it easy to build IDE plugins, autocomplete systems, and documentation generators.

## Output Formats

```bash
# JSON output (array of function metadata)
tmpltool --ide json > functions.json

# YAML output (list of function metadata)
tmpltool --ide yaml > functions.yaml

# TOML output (wrapped in [[functions]] array)
tmpltool --ide toml > functions.toml
```

## Metadata Structure

Each function includes:

| Field | Description |
|-------|-------------|
| `name` | Function name (e.g., `get_env`, `md5`) |
| `category` | Category grouping (e.g., `environment`, `hash`, `string`) |
| `description` | What the function does |
| `arguments` | Array of argument definitions with name, type, required flag, default value, and description |
| `return_type` | Type of value returned |
| `examples` | Usage examples showing both function and filter syntax where applicable |
| `syntax.function` | Whether callable as `func(arg=value)` |
| `syntax.filter` | Whether callable as `value \| filter` |
| `syntax.is_test` | Whether usable in `{% if value is test %}` |

## Example Output (JSON)

```json
[
  {
    "name": "get_env",
    "category": "environment",
    "description": "Get environment variable with optional default value",
    "arguments": [
      {
        "name": "name",
        "arg_type": "string",
        "required": true,
        "default": null,
        "description": "Environment variable name"
      },
      {
        "name": "default",
        "arg_type": "string",
        "required": false,
        "default": null,
        "description": "Default value if variable is not set"
      }
    ],
    "return_type": "string",
    "examples": [
      "{{ get_env(name=\"HOME\") }}",
      "{{ get_env(name=\"PORT\", default=\"8080\") }}"
    ],
    "syntax": {
      "function": true,
      "filter": false,
      "is_test": false
    }
  }
]
```

## Use Cases

- **IDE Plugins**: Provide autocomplete suggestions with argument hints and documentation
- **Language Servers**: Power hover documentation and signature help
- **Documentation Generators**: Automatically generate function reference documentation
- **Validation Tools**: Verify template function usage against available functions
- **CI/CD Integration**: Generate function lists for pipeline documentation
