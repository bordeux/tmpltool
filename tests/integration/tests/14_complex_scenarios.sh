#!/usr/bin/env bash
# Test: Complex real-world scenarios


echo "Test: Complex real-world scenarios"

# Test: Complex configuration template
create_template "complex.tmpl" '# Server Configuration
server:
  host: {{ get_env(name="SERVER_HOST", default="0.0.0.0") }}
  port: {{ get_env(name="SERVER_PORT", default="8080") }}
  {% if get_env(name="ENABLE_SSL", default="false") == "true" %}
  ssl:
    enabled: true
    cert: {{ get_env(name="SSL_CERT_PATH") }}
  {% endif %}'

OUTPUT=$(SERVER_HOST="localhost" SERVER_PORT="3000" run_binary "complex.tmpl")
assert_contains "$OUTPUT" "host: localhost" "Complex template renders host correctly"
assert_contains "$OUTPUT" "port: 3000" "Complex template renders port correctly"
