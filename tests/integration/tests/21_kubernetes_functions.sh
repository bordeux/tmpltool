#!/usr/bin/env bash
# Test: Kubernetes functions (k8s_resource_request, k8s_label_safe, k8s_dns_label_safe)

echo "Test: Kubernetes functions"

# ============================================================================
# k8s_resource_request Tests
# ============================================================================

# Test 1: k8s_resource_request - string values
create_template "k8s_resource_strings.tmpl" '{{ k8s_resource_request(cpu="500m", memory="512Mi") }}'
OUTPUT=$(run_binary "k8s_resource_strings.tmpl")
assert_contains "$OUTPUT" "requests:" "resource request has requests key"
assert_contains "$OUTPUT" 'cpu: "500m"' "resource request has cpu"
assert_contains "$OUTPUT" 'memory: "512Mi"' "resource request has memory"

# Test 2: k8s_resource_request - numeric CPU
create_template "k8s_resource_numeric_cpu.tmpl" '{{ k8s_resource_request(cpu=0.5, memory="512Mi") }}'
OUTPUT=$(run_binary "k8s_resource_numeric_cpu.tmpl")
assert_contains "$OUTPUT" 'cpu: "500m"' "numeric CPU converted to millicores"

# Test 3: k8s_resource_request - whole number CPU
create_template "k8s_resource_whole_cpu.tmpl" '{{ k8s_resource_request(cpu=2, memory="1Gi") }}'
OUTPUT=$(run_binary "k8s_resource_whole_cpu.tmpl")
assert_contains "$OUTPUT" 'cpu: "2000m"' "whole number CPU converted to millicores"

# Test 4: k8s_resource_request - numeric memory (Mi)
create_template "k8s_resource_numeric_mem_mi.tmpl" '{{ k8s_resource_request(cpu="500m", memory=512) }}'
OUTPUT=$(run_binary "k8s_resource_numeric_mem_mi.tmpl")
assert_contains "$OUTPUT" 'memory: "512Mi"' "numeric memory converted to Mi"

# Test 5: k8s_resource_request - numeric memory (Gi)
create_template "k8s_resource_numeric_mem_gi.tmpl" '{{ k8s_resource_request(cpu="1000m", memory=1024) }}'
OUTPUT=$(run_binary "k8s_resource_numeric_mem_gi.tmpl")
assert_contains "$OUTPUT" 'memory: "1Gi"' "numeric memory converted to Gi"

# Test 6: k8s_resource_request - both numeric
create_template "k8s_resource_both_numeric.tmpl" '{{ k8s_resource_request(cpu=1.5, memory=2048) }}'
OUTPUT=$(run_binary "k8s_resource_both_numeric.tmpl")
assert_contains "$OUTPUT" 'cpu: "1500m"' "numeric CPU converted"
assert_contains "$OUTPUT" 'memory: "2Gi"' "numeric memory converted to Gi"

# Test 7: k8s_resource_request - in deployment template
create_template "k8s_deployment.tmpl" 'apiVersion: apps/v1
kind: Deployment
metadata:
  name: my-app
spec:
  template:
    spec:
      containers:
      - name: app
        image: myapp:latest
        resources:
          {{ k8s_resource_request(cpu="500m", memory="512Mi") | indent(10) }}'
OUTPUT=$(run_binary "k8s_deployment.tmpl")
assert_contains "$OUTPUT" "Deployment" "deployment has kind"
assert_contains "$OUTPUT" "my-app" "deployment has name"
assert_contains "$OUTPUT" "requests:" "deployment has resource requests"

# Test 8: k8s_resource_request - with variables
create_template "k8s_resource_vars.tmpl" '{% set app_config = {"cpu": "250m", "memory": "256Mi"} %}
resources:
  {{ k8s_resource_request(cpu=app_config.cpu, memory=app_config.memory) | indent(2) }}'
OUTPUT=$(run_binary "k8s_resource_vars.tmpl")
assert_contains "$OUTPUT" 'cpu: "250m"' "resource request from config var"
assert_contains "$OUTPUT" 'memory: "256Mi"' "memory from config var"

# Test 9: k8s_resource_request - environment-based
create_template "k8s_resource_env.tmpl" '{% set env = "production" %}
{% set cpu = ternary(condition=env == "production", true_val="2000m", false_val="500m") %}
{% set memory = ternary(condition=env == "production", true_val="2Gi", false_val="512Mi") %}
{{ k8s_resource_request(cpu=cpu, memory=memory) }}'
OUTPUT=$(run_binary "k8s_resource_env.tmpl")
assert_contains "$OUTPUT" 'cpu: "2000m"' "production CPU resources"
assert_contains "$OUTPUT" 'memory: "2Gi"' "production memory resources"

# ============================================================================
# k8s_label_safe Tests
# ============================================================================

# Test 10: k8s_label_safe - simple
create_template "k8s_label_simple.tmpl" '{{ k8s_label_safe(value="my-app") }}'
OUTPUT=$(run_binary "k8s_label_simple.tmpl")
assert_equals "my-app" "$OUTPUT" "simple label unchanged"

# Test 11: k8s_label_safe - uppercase
create_template "k8s_label_uppercase.tmpl" '{{ k8s_label_safe(value="MyApp") }}'
OUTPUT=$(run_binary "k8s_label_uppercase.tmpl")
assert_equals "myapp" "$OUTPUT" "uppercase converted to lowercase"

# Test 12: k8s_label_safe - spaces
create_template "k8s_label_spaces.tmpl" '{{ k8s_label_safe(value="My App Name") }}'
OUTPUT=$(run_binary "k8s_label_spaces.tmpl")
assert_equals "my-app-name" "$OUTPUT" "spaces converted to dashes"

# Test 13: k8s_label_safe - special characters
create_template "k8s_label_special.tmpl" '{{ k8s_label_safe(value="My App (v2.0)!") }}'
OUTPUT=$(run_binary "k8s_label_special.tmpl")
assert_contains "$OUTPUT" "my-app" "special chars converted"

# Test 14: k8s_label_safe - underscores and dots
create_template "k8s_label_underscore_dot.tmpl" '{{ k8s_label_safe(value="my_app.v1") }}'
OUTPUT=$(run_binary "k8s_label_underscore_dot.tmpl")
assert_equals "my_app.v1" "$OUTPUT" "underscores and dots preserved"

# Test 15: k8s_label_safe - in labels
create_template "k8s_labels.tmpl" '{% set app_name = "My Application" %}
{% set version = "v2.0.1" %}
metadata:
  labels:
    app: {{ k8s_label_safe(value=app_name) }}
    version: {{ k8s_label_safe(value=version) }}'
OUTPUT=$(run_binary "k8s_labels.tmpl")
assert_contains "$OUTPUT" "app: my-application" "app label sanitized"
assert_contains "$OUTPUT" "version: v2.0.1" "version label sanitized"

# Test 16: k8s_label_safe - long string truncation
create_template "k8s_label_long.tmpl" '{{ k8s_label_safe(value="this-is-a-very-long-label-name-that-exceeds-the-kubernetes-maximum-label-length-limit") }}'
OUTPUT=$(run_binary "k8s_label_long.tmpl")
LENGTH=${#OUTPUT}
if [ $LENGTH -gt 63 ]; then
  fail "Label too long: $LENGTH characters" "$LENGTH chars (expected <= 63)"
else
  pass "Label truncated to <= 63 chars"
fi

# ============================================================================
# k8s_dns_label_safe Tests
# ============================================================================

# Test 17: k8s_dns_label_safe - simple
create_template "k8s_dns_simple.tmpl" '{{ k8s_dns_label_safe(value="my-service") }}'
OUTPUT=$(run_binary "k8s_dns_simple.tmpl")
assert_equals "my-service" "$OUTPUT" "simple DNS label unchanged"

# Test 18: k8s_dns_label_safe - uppercase
create_template "k8s_dns_uppercase.tmpl" '{{ k8s_dns_label_safe(value="MyService") }}'
OUTPUT=$(run_binary "k8s_dns_uppercase.tmpl")
assert_equals "myservice" "$OUTPUT" "uppercase converted to lowercase"

# Test 19: k8s_dns_label_safe - spaces
create_template "k8s_dns_spaces.tmpl" '{{ k8s_dns_label_safe(value="My Service Name") }}'
OUTPUT=$(run_binary "k8s_dns_spaces.tmpl")
assert_equals "my-service-name" "$OUTPUT" "spaces converted to dashes"

# Test 20: k8s_dns_label_safe - underscores removed
create_template "k8s_dns_underscore.tmpl" '{{ k8s_dns_label_safe(value="my_service") }}'
OUTPUT=$(run_binary "k8s_dns_underscore.tmpl")
assert_equals "my-service" "$OUTPUT" "underscores converted to dashes"

# Test 21: k8s_dns_label_safe - dots removed
create_template "k8s_dns_dots.tmpl" '{{ k8s_dns_label_safe(value="my.service.v1") }}'
OUTPUT=$(run_binary "k8s_dns_dots.tmpl")
assert_equals "my-service-v1" "$OUTPUT" "dots converted to dashes"

# Test 22: k8s_dns_label_safe - multiple dashes
create_template "k8s_dns_multiple_dashes.tmpl" '{{ k8s_dns_label_safe(value="my---service") }}'
OUTPUT=$(run_binary "k8s_dns_multiple_dashes.tmpl")
assert_equals "my-service" "$OUTPUT" "multiple dashes collapsed"

# Test 23: k8s_dns_label_safe - in service name
create_template "k8s_service.tmpl" '{% set service_name = "My Service Name" %}
apiVersion: v1
kind: Service
metadata:
  name: {{ k8s_dns_label_safe(value=service_name) }}
spec:
  selector:
    app: myapp'
OUTPUT=$(run_binary "k8s_service.tmpl")
assert_contains "$OUTPUT" "name: my-service-name" "service name sanitized"

# Test 24: k8s_dns_label_safe - long string
create_template "k8s_dns_long.tmpl" '{{ k8s_dns_label_safe(value="this-is-a-very-long-dns-label-that-exceeds-the-kubernetes-maximum-dns-label-length-limit") }}'
OUTPUT=$(run_binary "k8s_dns_long.tmpl")
LENGTH=${#OUTPUT}
if [ $LENGTH -gt 63 ]; then
  fail "DNS label too long: $LENGTH characters" "$LENGTH chars (expected <= 63)"
elif [[ $OUTPUT == *- ]]; then
  fail "DNS label ends with dash: $OUTPUT" "Should not end with dash"
else
  pass "DNS label truncated correctly"
fi

# ============================================================================
# Combined Use Cases
# ============================================================================

# Test 25: Full Kubernetes deployment
create_template "k8s_full_deployment.tmpl" '{% set app_name = "My Application" %}
{% set version = "v2.0.1" %}
{% set cpu = "500m" %}
{% set memory = 512 %}
---
apiVersion: apps/v1
kind: Deployment
metadata:
  name: {{ k8s_dns_label_safe(value=app_name) }}
  labels:
    app: {{ k8s_label_safe(value=app_name) }}
    version: {{ k8s_label_safe(value=version) }}
spec:
  replicas: 3
  selector:
    matchLabels:
      app: {{ k8s_label_safe(value=app_name) }}
  template:
    metadata:
      labels:
        app: {{ k8s_label_safe(value=app_name) }}
        version: {{ k8s_label_safe(value=version) }}
    spec:
      containers:
      - name: {{ k8s_dns_label_safe(value=app_name) }}
        image: mycompany/{{ k8s_dns_label_safe(value=app_name) }}:{{ version }}
        resources:
          {{ k8s_resource_request(cpu=cpu, memory=memory) | indent(10) }}'
OUTPUT=$(run_binary "k8s_full_deployment.tmpl")
assert_contains "$OUTPUT" "name: my-application" "deployment name"
assert_contains "$OUTPUT" "app: my-application" "app label"
assert_contains "$OUTPUT" 'cpu: "500m"' "cpu resource"
assert_contains "$OUTPUT" 'memory: "512Mi"' "memory resource"

# Test 26: Multiple services with loop
create_template "k8s_multiple_services.tmpl" '{% set services = [
  {"name": "Frontend Service", "cpu": "100m", "memory": "128Mi"},
  {"name": "Backend API", "cpu": "500m", "memory": "512Mi"},
  {"name": "Database Server", "cpu": "1000m", "memory": "2Gi"}
] %}
{% for service in services %}
---
apiVersion: apps/v1
kind: Deployment
metadata:
  name: {{ k8s_dns_label_safe(value=service.name) }}
  labels:
    app: {{ k8s_label_safe(value=service.name) }}
spec:
  template:
    spec:
      containers:
      - name: {{ k8s_dns_label_safe(value=service.name) }}
        resources:
          {{ k8s_resource_request(cpu=service.cpu, memory=service.memory) | indent(10) }}
{% endfor %}'
OUTPUT=$(run_binary "k8s_multiple_services.tmpl")
assert_contains "$OUTPUT" "name: frontend-service" "first service name"
assert_contains "$OUTPUT" "name: backend-api" "second service name"
assert_contains "$OUTPUT" "name: database-server" "third service name"
assert_contains "$OUTPUT" 'cpu: "100m"' "first service CPU"
assert_contains "$OUTPUT" 'memory: "2Gi"' "third service memory"

# Test 27: Environment-based resources
create_template "k8s_env_resources.tmpl" '{% set env = "production" %}
{% set app = "my-app" %}
{% set cpu = ternary(condition=env == "production", true_val=2, false_val=0.5) %}
{% set memory = ternary(condition=env == "production", true_val=2048, false_val=512) %}
metadata:
  name: {{ k8s_dns_label_safe(value=app) }}-{{ env }}
  labels:
    app: {{ k8s_label_safe(value=app) }}
    environment: {{ env }}
spec:
  containers:
  - name: app
    resources:
      {{ k8s_resource_request(cpu=cpu, memory=memory) | indent(6) }}'
OUTPUT=$(run_binary "k8s_env_resources.tmpl")
assert_contains "$OUTPUT" "name: my-app-production" "environment suffix"
assert_contains "$OUTPUT" 'cpu: "2000m"' "production CPU"
assert_contains "$OUTPUT" 'memory: "2Gi"' "production memory"

# ============================================================================
# Error Cases
# ============================================================================

# Test 28: Error - k8s_resource_request missing cpu
create_template "error_k8s_missing_cpu.tmpl" '{{ k8s_resource_request(memory="512Mi") }}'
OUTPUT=$(run_binary_expect_error "error_k8s_missing_cpu.tmpl")
assert_contains "$OUTPUT" "error" "error on missing cpu"

# Test 29: Error - k8s_resource_request missing memory
create_template "error_k8s_missing_memory.tmpl" '{{ k8s_resource_request(cpu="500m") }}'
OUTPUT=$(run_binary_expect_error "error_k8s_missing_memory.tmpl")
assert_contains "$OUTPUT" "error" "error on missing memory"
