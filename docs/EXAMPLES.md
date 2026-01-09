# Advanced Examples

## Docker Compose Generator

**Template** (`docker-compose.tmpltool`):
```yaml
version: '3.8'

services:
  {{ get_env(name="SERVICE_NAME", default="app") }}:
    image: {{ get_env(name="DOCKER_IMAGE", default="node:18") }}
    ports:
      - "{{ get_env(name="HOST_PORT", default="3000") }}:{{ get_env(name="CONTAINER_PORT", default="3000") }}"
    environment:
      - NODE_ENV={{ get_env(name="NODE_ENV", default="development") }}
      {% set db_url = get_env(name="DATABASE_URL", default="") %}
      {% if db_url %}
      - DATABASE_URL={{ db_url }}
      {% endif %}
    {% set enable_volumes = get_env(name="ENABLE_VOLUMES", default="false") %}
    {% if enable_volumes == "true" %}
    volumes:
      - ./app:/app
    {% endif %}
```

**Render with custom values:**
```bash
SERVICE_NAME=web \
DOCKER_IMAGE=node:20 \
HOST_PORT=8080 \
NODE_ENV=production \
DATABASE_URL=postgres://db:5432/mydb \
ENABLE_VOLUMES=true \
tmpltool docker-compose.tmpltool -o docker-compose.yml
```

**Or use defaults:**
```bash
tmpltool docker-compose.tmpltool -o docker-compose.yml
```

## Comprehensive Application Configuration

See [examples/comprehensive-app-config.tmpltool](examples/comprehensive-app-config.tmpltool) for a complete example demonstrating all features:

- All hash functions (MD5, SHA1, SHA256, SHA512)
- UUID generation
- Random strings with various charsets
- Environment variables with defaults
- Pattern filtering with `filter_env()`
- Conditionals (if/elif/else)
- Loops with ranges and arrays
- Filters (upper, lower, trim, slugify, replace, split, length)
- Comments
- Complex nested logic

## Pipeline Usage

```bash
# Generate and validate JSON config
tmpltool config.json.tmpltool | jq .

# Generate and apply Kubernetes config
tmpltool k8s-deployment.yaml.tmpltool | kubectl apply -f -

# Generate nginx config and test it
tmpltool nginx.conf.tmpltool | nginx -t -c /dev/stdin

# Combine multiple templates
cat header.tmpltool body.tmpltool footer.tmpltool | tmpltool > complete.html
```

## Trust Mode - System Files

**Template** (`system_info.tmpltool`):
```
# System Information

## Hostname
{{ read_file(path="/etc/hostname") }}

## Hosts File (first 200 chars)
{{ read_file(path="/etc/hosts") | truncate(length=200) }}

## Files in /etc (first 10)
{% for file in list_dir(path="/etc") | slice(end=10) %}
- {{ file }}
{% endfor %}
```

**Render:**
```bash
# Without --trust: Security error
tmpltool system_info.tmpltool
# Error: Security: Absolute paths are not allowed

# With --trust: Works!
tmpltool --trust system_info.tmpltool -o system_info.md
```

**WARNING:** Only use `--trust` with templates you completely trust. Malicious templates could read sensitive files like SSH keys, passwords, or system configurations.
