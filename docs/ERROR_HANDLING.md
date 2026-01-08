# Error Handling

- **Missing template file:** tmpltool exits with an error
- **Invalid template syntax:** Error location is reported
- **Environment variables:**
  - Direct access not supported: `{{ ENV_VAR }}` causes an error
  - With default (recommended): `{{ get_env(name="VAR", default="...") }}` uses default if missing
  - Without default: `{{ get_env(name="VAR") }}` errors if variable doesn't exist
- **Filesystem errors:** Clear error messages for missing files, permission issues, or security violations
