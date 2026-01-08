# CLI Reference

## Syntax

```bash
tmpltool [TEMPLATE] [OPTIONS]
cat template.txt | tmpltool [OPTIONS]
```

## Arguments

- `[TEMPLATE]` - Path to template file (optional, reads from stdin if omitted)

## Options

- `-o, --output <FILE>` - Output file path (prints to stdout if not specified)
- `--trust` - Trust mode: Allow filesystem functions to access absolute paths and parent directories
  - **WARNING:** Only use with trusted templates. Disables security restrictions.
- `--validate <FORMAT>` - Validate output format (json, yaml, or toml)
  - Validates the rendered output conforms to the specified format
  - Exits with error code 1 if validation fails
  - No output on success, error message only on validation failure
- `--ide <FORMAT>` - Output function metadata for IDE integration (json, yaml, or toml)
  - Prints all available functions with descriptions, arguments, return types, and examples
  - Exits immediately after printing metadata (does not render templates)
  - Useful for building IDE plugins, autocomplete, and documentation generators

## Input/Output Patterns

| Input  | Output | Command |
|--------|--------|---------|
| File   | stdout | `tmpltool template.txt` |
| File   | File   | `tmpltool template.txt -o output.txt` |
| stdin  | stdout | `cat template.txt \| tmpltool` |
| stdin  | File   | `cat template.txt \| tmpltool -o output.txt` |

## Examples

```bash
# File to stdout
tmpltool template.txt

# File to file
tmpltool template.txt -o output.txt

# Stdin to stdout (pipe)
echo "Hello {{ get_env(name=\"USER\") }}!" | tmpltool

# With environment variables
DB_HOST=postgres tmpltool config.tmpl -o config.txt

# Chaining with other tools
tmpltool config.json.tmpl | jq .
cat k8s-deployment.yaml.tmpl | tmpltool | kubectl apply -f -

# Trust mode for system files
tmpltool --trust system_info.tmpl  # Can read /etc/passwd, etc.

# Validate JSON output
tmpltool config.json.tmpl --validate json
# Exits with error if output is invalid JSON

# Validate YAML output
tmpltool k8s-deploy.yaml.tmpl --validate yaml -o deployment.yaml

# Validate TOML output
tmpltool Cargo.toml.tmpl --validate toml
```
