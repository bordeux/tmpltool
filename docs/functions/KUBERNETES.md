## Kubernetes Functions

Kubernetes-specific helpers for manifest generation, label sanitization, and resource management.

**See also:** [Function Reference](../FUNCTIONS.md) | [Main Documentation](../README.md)

Kubernetes-specific helpers for manifest generation and label sanitization.

#### `k8s_resource_request(cpu, memory)`

Format Kubernetes resource requests in YAML format.

**Arguments:**
- `cpu` (required): CPU request - string like `"500m"` or number (converted to millicores)
- `memory` (required): Memory request - string like `"512Mi"` or number in MiB (auto-converted to Mi/Gi)

**Returns:** YAML-formatted resource request block

**Numeric conversions:**
- CPU: `0.5` → `"500m"`, `2` → `"2000m"`
- Memory: `512` → `"512Mi"`, `1024` → `"1Gi"`, `2048` → `"2Gi"`

**Example:**
```jinja
{# Basic usage with strings #}
{{ k8s_resource_request(cpu="500m", memory="512Mi") }}
{# Output:
requests:
  cpu: "500m"
  memory: "512Mi"
#}

{# With numeric values (auto-formatted) #}
{{ k8s_resource_request(cpu=0.5, memory=512) }}
{# Output:
requests:
  cpu: "500m"
  memory: "512Mi"
#}

{# In a Kubernetes deployment #}
apiVersion: apps/v1
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
          {{ k8s_resource_request(cpu="1000m", memory="1Gi") | indent(10) }}
```

#### `k8s_label_safe(value)` / `| k8s_label_safe`

Sanitize string to be Kubernetes label-safe.

**Arguments:**
- `value` (required): String to sanitize

**Returns:** Sanitized string following Kubernetes label requirements:
- Max 63 characters
- Only alphanumeric, dashes, underscores, dots
- Must start and end with alphanumeric
- Lowercase

**Example:**
```jinja
{# Function syntax #}
{{ k8s_label_safe(value="My App (v2.0)") }}
{# Output: my-app-v2.0 #}

{# Filter syntax #}
{{ "My App (v2.0)" | k8s_label_safe }}
{# Output: my-app-v2.0 #}

{# Use in labels #}
metadata:
  labels:
    app: {{ app_name | k8s_label_safe }}
    version: {{ version | k8s_label_safe }}
```

#### `k8s_dns_label_safe(value)` / `| k8s_dns_label_safe`

Format DNS-safe label (max 63 chars, lowercase, alphanumeric and dashes only).

**Arguments:**
- `value` (required): String to format

**Returns:** DNS-safe string suitable for Kubernetes resource names

**Example:**
```jinja
{# Function syntax #}
{{ k8s_dns_label_safe(value="My Service Name") }}
{# Output: my-service-name #}

{# Filter syntax #}
{{ "My Service Name" | k8s_dns_label_safe }}
{# Output: my-service-name #}

{# Use in service names #}
apiVersion: v1
kind: Service
metadata:
  name: {{ service_name | k8s_dns_label_safe }}
```

#### `k8s_env_var_ref(var_name, source, name)`

Generate Kubernetes environment variable reference (ConfigMap or Secret).

**Arguments:**
- `var_name` (required): The environment variable name/key
- `source` (optional): Source type - `"configmap"` or `"secret"` (default: `"configmap"`)
- `name` (optional): Name of the ConfigMap/Secret (default: auto-generated from var_name)

**Returns:** YAML-formatted `valueFrom` reference

**Example:**
```jinja
{# ConfigMap reference #}
- name: DATABASE_HOST
  {{ k8s_env_var_ref(var_name="DATABASE_HOST", source="configmap", name="app-config") | indent(2) }}
{# Output:
  valueFrom:
    configMapKeyRef:
      name: app-config
      key: DATABASE_HOST
#}

{# Secret reference with auto-generated name #}
- name: DB_PASSWORD
  {{ k8s_env_var_ref(var_name="DB_PASSWORD", source="secret") | indent(2) }}
{# Output:
  valueFrom:
    secretKeyRef:
      name: db-password
      key: DB_PASSWORD
#}
```

#### `k8s_secret_ref(secret_name, key, optional)`

Generate Kubernetes Secret reference for environment variables.

**Arguments:**
- `secret_name` (required): Name of the Secret
- `key` (required): Key within the Secret
- `optional` (optional): Whether the Secret is optional (default: `false`)

**Returns:** YAML-formatted `valueFrom` secretKeyRef

**Example:**
```jinja
{# Basic secret reference #}
- name: DB_PASSWORD
  {{ k8s_secret_ref(secret_name="db-credentials", key="password") | indent(2) }}
{# Output:
  valueFrom:
    secretKeyRef:
      name: db-credentials
      key: password
#}

{# Optional secret #}
- name: OPTIONAL_TOKEN
  {{ k8s_secret_ref(secret_name="tokens", key="api_token", optional=true) | indent(2) }}
{# Output:
  valueFrom:
    secretKeyRef:
      name: tokens
      key: api_token
      optional: true
#}
```

#### `k8s_configmap_ref(configmap_name, key, optional)`

Generate Kubernetes ConfigMap reference for environment variables.

**Arguments:**
- `configmap_name` (required): Name of the ConfigMap
- `key` (required): Key within the ConfigMap
- `optional` (optional): Whether the ConfigMap is optional (default: `false`)

**Returns:** YAML-formatted `valueFrom` configMapKeyRef

**Example:**
```jinja
{# Basic ConfigMap reference #}
- name: DATABASE_HOST
  {{ k8s_configmap_ref(configmap_name="app-config", key="database_host") | indent(2) }}
{# Output:
  valueFrom:
    configMapKeyRef:
      name: app-config
      key: database_host
#}

{# Optional ConfigMap #}
- name: FEATURE_FLAG
  {{ k8s_configmap_ref(configmap_name="features", key="new_ui", optional=true) | indent(2) }}
{# Output:
  valueFrom:
    configMapKeyRef:
      name: features
      key: new_ui
      optional: true
#}

{# Complete deployment example #}
apiVersion: apps/v1
kind: Deployment
metadata:
  name: {{ k8s_dns_label_safe(value=app_name) }}
spec:
  template:
    spec:
      containers:
        - name: app
          image: myapp:latest
          env:
            - name: ENVIRONMENT
              value: "production"
            - name: DATABASE_HOST
              {{ k8s_configmap_ref(configmap_name="app-config", key="db_host") | indent(14) }}
            - name: DATABASE_PASSWORD
              {{ k8s_secret_ref(secret_name="db-credentials", key="password") | indent(14) }}
          resources:
            {{ k8s_resource_request(cpu="500m", memory="512Mi") | indent(12) }}
```

#### `helm_tpl(template, values)`

Perform Helm-style templating with `{{ .key }}` syntax.

**Arguments:**
- `template` (required): Template string with Helm-style placeholders
- `values` (required): Object containing values to substitute

**Returns:** Rendered template string

**Example:**
```jinja
{# Basic Helm templating #}
{{ helm_tpl(template="Hello {{ .name }}!", values={"name": "World"}) }}
{# Output: Hello World! #}

{# Complex template #}
{% set tpl = "{{ .app.name }}-{{ .app.version }}" %}
{% set vals = {"app": {"name": "myapp", "version": "1.0"}} %}
{{ helm_tpl(template=tpl, values=vals) }}
{# Output: myapp-1.0 #}

{# In Kubernetes manifest #}
metadata:
  name: {{ helm_tpl(template="{{ .Release.Name }}-{{ .Chart.Name }}", values={"Release": {"Name": "prod"}, "Chart": {"Name": "webapp"}}) }}
```

#### `k8s_annotation_safe(value)` / `| k8s_annotation_safe`

Sanitize a string for use as a Kubernetes annotation value.

**Arguments:**
- `value` (required): The string to sanitize

**Returns:** Sanitized string safe for annotation values (max 64KB, control chars replaced with spaces)

**Example:**
```jinja
{# Function syntax #}
{{ k8s_annotation_safe(value="Description with\nnewlines and\ttabs") }}
{# Output: Description with newlines and tabs #}

{# Filter syntax #}
{{ "Description with\nnewlines" | k8s_annotation_safe }}
{# Output: Description with newlines #}

{# In Kubernetes manifest #}
metadata:
  annotations:
    description: "{{ description | k8s_annotation_safe }}"
    config: "{{ config_obj | to_json | k8s_annotation_safe }}"
```

#### `k8s_quantity_to_bytes(quantity)`

Convert a Kubernetes quantity string to bytes.

**Arguments:**
- `quantity` (required): Kubernetes quantity string (e.g., "1Gi", "500Mi", "100m")

**Returns:** Integer number of bytes (or millicores for CPU)

**Supported suffixes:**
- Binary: Ki (1024), Mi (1024²), Gi (1024³), Ti (1024⁴), Pi (1024⁵), Ei (1024⁶)
- Decimal: K (1000), M (1000²), G (1000³), T (1000⁴), P (1000⁵), E (1000⁶)
- CPU: m (millicores)

**Example:**
```jinja
{# Convert memory quantities #}
{{ k8s_quantity_to_bytes(quantity="1Gi") }}
{# Output: 1073741824 #}

{{ k8s_quantity_to_bytes(quantity="500Mi") }}
{# Output: 524288000 #}

{# Convert CPU quantities #}
{{ k8s_quantity_to_bytes(quantity="500m") }}
{# Output: 500 (millicores) #}

{# Calculate total memory from multiple pods #}
{% set pod_memory = k8s_quantity_to_bytes(quantity="256Mi") %}
{% set replicas = 3 %}
Total bytes: {{ pod_memory * replicas }}
```

#### `k8s_bytes_to_quantity(bytes, unit)`

Convert bytes to a Kubernetes quantity string.

**Arguments:**
- `bytes` (required): Number of bytes to convert
- `unit` (optional): Target unit (default: auto-selects appropriate unit)
  - Supported: "Ki", "Mi", "Gi", "Ti", "Pi", "Ei", "K", "M", "G", "T", "P", "E"

**Returns:** Kubernetes quantity string

**Example:**
```jinja
{# Auto-select appropriate unit #}
{{ k8s_bytes_to_quantity(bytes=1073741824) }}
{# Output: 1Gi #}

{{ k8s_bytes_to_quantity(bytes=536870912) }}
{# Output: 512Mi #}

{# Force specific unit #}
{{ k8s_bytes_to_quantity(bytes=1073741824, unit="Mi") }}
{# Output: 1024Mi #}

{# In resource calculations #}
{% set total_bytes = 2147483648 %}
resources:
  requests:
    memory: "{{ k8s_bytes_to_quantity(bytes=total_bytes / 2) }}"
  limits:
    memory: "{{ k8s_bytes_to_quantity(bytes=total_bytes) }}"
```

#### `k8s_selector(labels)`

Generate a Kubernetes label selector string from a labels object.

**Arguments:**
- `labels` (required): Object containing key-value pairs for the selector

**Returns:** Label selector string in the format "key1=value1,key2=value2"

**Example:**
```jinja
{# Basic selector #}
{{ k8s_selector(labels={"app": "nginx", "env": "prod"}) }}
{# Output: app=nginx,env=prod #}

{# In Kubernetes Service #}
spec:
  selector:
    {{ k8s_selector(labels={"app": app_name, "version": version}) }}

{# With kubectl #}
kubectl get pods -l "{{ k8s_selector(labels=pod_labels) }}"
```

#### `k8s_pod_affinity(key, operator, values, topology_key, type)`

Generate Kubernetes pod affinity/anti-affinity YAML.

**Arguments:**
- `key` (required): Label key to match
- `operator` (required): Match operator ("In", "NotIn", "Exists", "DoesNotExist")
- `values` (optional): Array of values to match (required for In/NotIn)
- `topology_key` (optional): Topology key (default: "kubernetes.io/hostname")
- `type` (optional): Affinity type - "required" or "preferred" (default: "required")

**Returns:** YAML string for pod affinity configuration

**Example:**
```jinja
{# Required pod affinity #}
{{ k8s_pod_affinity(key="app", operator="In", values=["cache", "db"]) }}
{# Output:
requiredDuringSchedulingIgnoredDuringExecution:
  - labelSelector:
      matchExpressions:
        - key: app
          operator: In
          values:
            - cache
            - db
    topologyKey: kubernetes.io/hostname
#}

{# Preferred anti-affinity for spreading pods #}
{{ k8s_pod_affinity(key="app", operator="In", values=["web"], type="preferred") }}
{# Output:
preferredDuringSchedulingIgnoredDuringExecution:
  - weight: 100
    podAffinityTerm:
      labelSelector:
        matchExpressions:
          - key: app
            operator: In
            values:
              - web
      topologyKey: kubernetes.io/hostname
#}

{# In Deployment spec #}
spec:
  affinity:
    podAntiAffinity:
      {{ k8s_pod_affinity(key="app", operator="In", values=[app_name], topology_key="topology.kubernetes.io/zone") | indent(6) }}
```

#### `k8s_toleration(key, operator, value, effect)`

Generate Kubernetes toleration YAML.

**Arguments:**
- `key` (required): Taint key to tolerate
- `operator` (optional): Match operator - "Equal" or "Exists" (default: "Equal")
- `value` (optional): Taint value to match (required when operator is "Equal")
- `effect` (optional): Taint effect - "NoSchedule", "PreferNoSchedule", or "NoExecute"

**Returns:** YAML string for toleration configuration

**Example:**
```jinja
{# Basic toleration #}
{{ k8s_toleration(key="dedicated", value="gpu", effect="NoSchedule") }}
{# Output:
- key: dedicated
  operator: Equal
  value: gpu
  effect: NoSchedule
#}

{# Exists operator (matches any value) #}
{{ k8s_toleration(key="node.kubernetes.io/not-ready", operator="Exists", effect="NoExecute") }}
{# Output:
- key: node.kubernetes.io/not-ready
  operator: Exists
  effect: NoExecute
#}

{# In Pod spec #}
spec:
  tolerations:
    {{ k8s_toleration(key="dedicated", value="high-memory", effect="NoSchedule") | indent(4) }}
    {{ k8s_toleration(key="node.kubernetes.io/unreachable", operator="Exists", effect="NoExecute") | indent(4) }}
```

#### `k8s_probe(type, path, port, initial_delay, period, timeout, success_threshold, failure_threshold, command)`

Generate Kubernetes liveness/readiness probe YAML.

**Arguments:**
- `type` (required): Probe type - "http", "tcp", or "exec"
- `path` (optional): HTTP path for http probes (default: "/healthz")
- `port` (optional): Port number for http/tcp probes (default: 8080)
- `initial_delay` (optional): Initial delay in seconds (default: 0)
- `period` (optional): Period between probes in seconds (default: 10)
- `timeout` (optional): Timeout in seconds (default: 1)
- `success_threshold` (optional): Success threshold (default: 1)
- `failure_threshold` (optional): Failure threshold (default: 3)
- `command` (optional): Command array for exec probes

**Returns:** YAML string for probe configuration

**Example:**
```jinja
{# HTTP health check #}
{{ k8s_probe(type="http", path="/health", port=8080) }}
{# Output:
httpGet:
  path: /health
  port: 8080
initialDelaySeconds: 0
periodSeconds: 10
timeoutSeconds: 1
successThreshold: 1
failureThreshold: 3
#}

{# TCP socket check with custom timings #}
{{ k8s_probe(type="tcp", port=5432, initial_delay=30, period=20) }}
{# Output:
tcpSocket:
  port: 5432
initialDelaySeconds: 30
periodSeconds: 20
timeoutSeconds: 1
successThreshold: 1
failureThreshold: 3
#}

{# Exec probe with command #}
{{ k8s_probe(type="exec", command=["cat", "/tmp/healthy"]) }}
{# Output:
exec:
  command:
    - cat
    - /tmp/healthy
initialDelaySeconds: 0
periodSeconds: 10
timeoutSeconds: 1
successThreshold: 1
failureThreshold: 3
#}

{# In container spec #}
containers:
  - name: app
    livenessProbe:
      {{ k8s_probe(type="http", path="/healthz", port=8080, initial_delay=15, failure_threshold=5) | indent(6) }}
    readinessProbe:
      {{ k8s_probe(type="http", path="/ready", port=8080, period=5) | indent(6) }}
```

