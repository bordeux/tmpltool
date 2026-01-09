#!/usr/bin/env bash
# Test: Kubernetes Extended Functions


echo "Test: Kubernetes Extended Functions"

# ==================== helm_tpl Tests ====================

# Test: helm_tpl() - simple substitution
create_template "helm1.tmpltool" '{{ helm_tpl(template="Hello {{ .name }}!", values={"name": "World"}) }}'
OUTPUT=$(run_binary "helm1.tmpltool")
assert_equals "Hello World!" "$OUTPUT" "helm_tpl() simple substitution"

# Test: helm_tpl() - nested values
create_template "helm2.tmpltool" '{{ helm_tpl(template="{{ .app.name }}-{{ .app.version }}", values={"app": {"name": "myapp", "version": "1.0"}}) }}'
OUTPUT=$(run_binary "helm2.tmpltool")
assert_equals "myapp-1.0" "$OUTPUT" "helm_tpl() nested values"

# Test: helm_tpl() - no placeholders
create_template "helm3.tmpltool" '{{ helm_tpl(template="static-text", values={}) }}'
OUTPUT=$(run_binary "helm3.tmpltool")
assert_equals "static-text" "$OUTPUT" "helm_tpl() no placeholders"

# ==================== k8s_annotation_safe Tests ====================

# Test: k8s_annotation_safe() - simple text
create_template "anno1.tmpltool" '{{ k8s_annotation_safe(value="simple annotation") }}'
OUTPUT=$(run_binary "anno1.tmpltool")
assert_equals "simple annotation" "$OUTPUT" "k8s_annotation_safe() simple text"

# Test: k8s_annotation_safe() - with escaped newlines (literal \n in template)
create_template "anno2.tmpltool" '{{ k8s_annotation_safe(value="line1 line2") }}'
OUTPUT=$(run_binary "anno2.tmpltool")
assert_equals "line1 line2" "$OUTPUT" "k8s_annotation_safe() preserves spaces"

# ==================== k8s_quantity_to_bytes Tests ====================

# Test: k8s_quantity_to_bytes() - Gi
create_template "qty2b1.tmpltool" '{{ k8s_quantity_to_bytes(quantity="1Gi") }}'
OUTPUT=$(run_binary "qty2b1.tmpltool")
assert_equals "1073741824" "$OUTPUT" "k8s_quantity_to_bytes() 1Gi"

# Test: k8s_quantity_to_bytes() - Mi
create_template "qty2b2.tmpltool" '{{ k8s_quantity_to_bytes(quantity="512Mi") }}'
OUTPUT=$(run_binary "qty2b2.tmpltool")
assert_equals "536870912" "$OUTPUT" "k8s_quantity_to_bytes() 512Mi"

# Test: k8s_quantity_to_bytes() - Ki
create_template "qty2b3.tmpltool" '{{ k8s_quantity_to_bytes(quantity="100Ki") }}'
OUTPUT=$(run_binary "qty2b3.tmpltool")
assert_equals "102400" "$OUTPUT" "k8s_quantity_to_bytes() 100Ki"

# Test: k8s_quantity_to_bytes() - decimal G
create_template "qty2b4.tmpltool" '{{ k8s_quantity_to_bytes(quantity="1G") }}'
OUTPUT=$(run_binary "qty2b4.tmpltool")
assert_equals "1000000000" "$OUTPUT" "k8s_quantity_to_bytes() 1G"

# Test: k8s_quantity_to_bytes() - millicores
create_template "qty2b5.tmpltool" '{{ k8s_quantity_to_bytes(quantity="500m") }}'
OUTPUT=$(run_binary "qty2b5.tmpltool")
assert_equals "500" "$OUTPUT" "k8s_quantity_to_bytes() 500m (millicores)"

# ==================== k8s_bytes_to_quantity Tests ====================

# Test: k8s_bytes_to_quantity() - 1Gi
create_template "b2qty1.tmpltool" '{{ k8s_bytes_to_quantity(bytes=1073741824) }}'
OUTPUT=$(run_binary "b2qty1.tmpltool")
assert_equals "1Gi" "$OUTPUT" "k8s_bytes_to_quantity() 1Gi"

# Test: k8s_bytes_to_quantity() - 512Mi
create_template "b2qty2.tmpltool" '{{ k8s_bytes_to_quantity(bytes=536870912) }}'
OUTPUT=$(run_binary "b2qty2.tmpltool")
assert_equals "512Mi" "$OUTPUT" "k8s_bytes_to_quantity() 512Mi"

# Test: k8s_bytes_to_quantity() - forced unit
create_template "b2qty3.tmpltool" '{{ k8s_bytes_to_quantity(bytes=1073741824, unit="Mi") }}'
OUTPUT=$(run_binary "b2qty3.tmpltool")
assert_equals "1024Mi" "$OUTPUT" "k8s_bytes_to_quantity() forced Mi"

# Test: k8s_bytes_to_quantity() - zero
create_template "b2qty4.tmpltool" '{{ k8s_bytes_to_quantity(bytes=0) }}'
OUTPUT=$(run_binary "b2qty4.tmpltool")
assert_equals "0" "$OUTPUT" "k8s_bytes_to_quantity() zero"

# ==================== k8s_selector Tests ====================

# Test: k8s_selector() - single label
create_template "sel1.tmpltool" '{{ k8s_selector(labels={"app": "nginx"}) }}'
OUTPUT=$(run_binary "sel1.tmpltool")
assert_equals "app=nginx" "$OUTPUT" "k8s_selector() single label"

# Test: k8s_selector() - multiple labels
create_template "sel2.tmpltool" '{{ k8s_selector(labels={"app": "nginx", "env": "prod"}) }}'
OUTPUT=$(run_binary "sel2.tmpltool")
assert_contains "$OUTPUT" "app=nginx" "k8s_selector() has app label"
assert_contains "$OUTPUT" "env=prod" "k8s_selector() has env label"

# Test: k8s_selector() - empty
create_template "sel3.tmpltool" '{{ k8s_selector(labels={}) }}'
OUTPUT=$(run_binary "sel3.tmpltool")
assert_equals "" "$OUTPUT" "k8s_selector() empty"

# ==================== k8s_pod_affinity Tests ====================

# Test: k8s_pod_affinity() - required
create_template "aff1.tmpltool" '{{ k8s_pod_affinity(key="app", operator="In", values=["web", "api"], type="required") }}'
OUTPUT=$(run_binary "aff1.tmpltool")
assert_contains "$OUTPUT" "requiredDuringSchedulingIgnoredDuringExecution" "k8s_pod_affinity() has required"
assert_contains "$OUTPUT" "key: app" "k8s_pod_affinity() has key"
assert_contains "$OUTPUT" "operator: In" "k8s_pod_affinity() has operator"

# Test: k8s_pod_affinity() - preferred
create_template "aff2.tmpltool" '{{ k8s_pod_affinity(key="app", operator="In", values=["web"], type="preferred") }}'
OUTPUT=$(run_binary "aff2.tmpltool")
assert_contains "$OUTPUT" "preferredDuringSchedulingIgnoredDuringExecution" "k8s_pod_affinity() preferred"
assert_contains "$OUTPUT" "weight: 100" "k8s_pod_affinity() has weight"

# Test: k8s_pod_affinity() - Exists operator
create_template "aff3.tmpltool" '{{ k8s_pod_affinity(key="app", operator="Exists") }}'
OUTPUT=$(run_binary "aff3.tmpltool")
assert_contains "$OUTPUT" "operator: Exists" "k8s_pod_affinity() Exists"

# ==================== k8s_toleration Tests ====================

# Test: k8s_toleration() - Equal operator
create_template "tol1.tmpltool" '{{ k8s_toleration(key="dedicated", value="gpu", effect="NoSchedule") }}'
OUTPUT=$(run_binary "tol1.tmpltool")
assert_contains "$OUTPUT" "key: dedicated" "k8s_toleration() has key"
assert_contains "$OUTPUT" "operator: Equal" "k8s_toleration() Equal operator"
assert_contains "$OUTPUT" "value: gpu" "k8s_toleration() has value"
assert_contains "$OUTPUT" "effect: NoSchedule" "k8s_toleration() has effect"

# Test: k8s_toleration() - Exists operator
create_template "tol2.tmpltool" '{{ k8s_toleration(key="node.kubernetes.io/not-ready", operator="Exists", effect="NoExecute") }}'
OUTPUT=$(run_binary "tol2.tmpltool")
assert_contains "$OUTPUT" "operator: Exists" "k8s_toleration() Exists"
assert_contains "$OUTPUT" "effect: NoExecute" "k8s_toleration() NoExecute"

# ==================== k8s_probe Tests ====================

# Test: k8s_probe() - HTTP probe
create_template "probe1.tmpltool" '{{ k8s_probe(type="http", path="/health", port=8080) }}'
OUTPUT=$(run_binary "probe1.tmpltool")
assert_contains "$OUTPUT" "httpGet:" "k8s_probe() httpGet"
assert_contains "$OUTPUT" "path: /health" "k8s_probe() has path"
assert_contains "$OUTPUT" "port: 8080" "k8s_probe() has port"
assert_contains "$OUTPUT" "periodSeconds:" "k8s_probe() has period"

# Test: k8s_probe() - TCP probe
create_template "probe2.tmpltool" '{{ k8s_probe(type="tcp", port=5432) }}'
OUTPUT=$(run_binary "probe2.tmpltool")
assert_contains "$OUTPUT" "tcpSocket:" "k8s_probe() tcpSocket"
assert_contains "$OUTPUT" "port: 5432" "k8s_probe() tcp port"

# Test: k8s_probe() - Exec probe
create_template "probe3.tmpltool" '{{ k8s_probe(type="exec", command=["cat", "/tmp/healthy"]) }}'
OUTPUT=$(run_binary "probe3.tmpltool")
assert_contains "$OUTPUT" "exec:" "k8s_probe() exec"
assert_contains "$OUTPUT" "command:" "k8s_probe() has command"

# Test: k8s_probe() - custom timings
create_template "probe4.tmpltool" '{{ k8s_probe(type="http", path="/healthz", port=8080, initial_delay=15, period=20, failure_threshold=5) }}'
OUTPUT=$(run_binary "probe4.tmpltool")
assert_contains "$OUTPUT" "initialDelaySeconds: 15" "k8s_probe() initial delay"
assert_contains "$OUTPUT" "periodSeconds: 20" "k8s_probe() period"
assert_contains "$OUTPUT" "failureThreshold: 5" "k8s_probe() failure threshold"

# ==================== Practical Examples ====================

# Test: Kubernetes deployment snippet
create_template "k8s_deploy.tmpltool" 'spec:
  containers:
    - name: app
      resources:
        {{ k8s_resource_request(cpu="500m", memory="512Mi") | indent(8) }}
      livenessProbe:
        {{ k8s_probe(type="http", path="/healthz", port=8080, initial_delay=10) | indent(8) }}
      tolerations:
        {{ k8s_toleration(key="dedicated", value="webapp", effect="NoSchedule") | indent(8) }}'
OUTPUT=$(run_binary "k8s_deploy.tmpltool")
assert_contains "$OUTPUT" "requests:" "k8s deployment has requests"
assert_contains "$OUTPUT" "httpGet:" "k8s deployment has probe"
assert_contains "$OUTPUT" "key: dedicated" "k8s deployment has toleration"

# Test: Roundtrip quantity conversion
create_template "qty_roundtrip.tmpltool" '{% set bytes = k8s_quantity_to_bytes(quantity="2Gi") %}{{ k8s_bytes_to_quantity(bytes=bytes) }}'
OUTPUT=$(run_binary "qty_roundtrip.tmpltool")
assert_equals "2Gi" "$OUTPUT" "quantity roundtrip conversion"
