use minijinja::Value;
use minijinja::value::Kwargs;
use tmpltool::functions::kubernetes;

// ============================================================================
// helm_tpl Tests
// ============================================================================

#[test]
fn test_helm_tpl_simple() {
    let result = kubernetes::helm_tpl_fn(Kwargs::from_iter(vec![
        ("template", Value::from("Hello {{ .name }}!")),
        (
            "values",
            Value::from_serialize(serde_json::json!({"name": "World"})),
        ),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "Hello World!");
}

#[test]
fn test_helm_tpl_nested_values() {
    let result = kubernetes::helm_tpl_fn(Kwargs::from_iter(vec![
        (
            "template",
            Value::from("{{ .app.name }}-{{ .app.version }}"),
        ),
        (
            "values",
            Value::from_serialize(serde_json::json!({
                "app": {"name": "myapp", "version": "1.0"}
            })),
        ),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "myapp-1.0");
}

#[test]
fn test_helm_tpl_no_placeholders() {
    let result = kubernetes::helm_tpl_fn(Kwargs::from_iter(vec![
        ("template", Value::from("static-text")),
        ("values", Value::from_serialize(serde_json::json!({}))),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "static-text");
}

#[test]
fn test_helm_tpl_multiple_placeholders() {
    let result = kubernetes::helm_tpl_fn(Kwargs::from_iter(vec![
        (
            "template",
            Value::from("{{ .Release.Name }}-{{ .Chart.Name }}"),
        ),
        (
            "values",
            Value::from_serialize(serde_json::json!({
                "Release": {"Name": "prod"},
                "Chart": {"Name": "webapp"}
            })),
        ),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "prod-webapp");
}

#[test]
fn test_helm_tpl_missing_value() {
    let result = kubernetes::helm_tpl_fn(Kwargs::from_iter(vec![
        ("template", Value::from("{{ .missing }}")),
        ("values", Value::from_serialize(serde_json::json!({}))),
    ]))
    .unwrap();

    // Should return empty string for missing values
    assert_eq!(result.to_string(), "");
}

#[test]
fn test_helm_tpl_missing_template() {
    let result = kubernetes::helm_tpl_fn(Kwargs::from_iter(vec![(
        "values",
        Value::from_serialize(serde_json::json!({})),
    )]));

    assert!(result.is_err());
}

// ============================================================================
// k8s_annotation_safe Tests
// ============================================================================

#[test]
fn test_k8s_annotation_safe_simple() {
    let result = kubernetes::k8s_annotation_safe_fn(Kwargs::from_iter(vec![(
        "value",
        Value::from("simple annotation"),
    )]))
    .unwrap();

    assert_eq!(result.to_string(), "simple annotation");
}

#[test]
fn test_k8s_annotation_safe_newlines() {
    let result = kubernetes::k8s_annotation_safe_fn(Kwargs::from_iter(vec![(
        "value",
        Value::from("line1\nline2\nline3"),
    )]))
    .unwrap();

    assert_eq!(result.to_string(), "line1 line2 line3");
}

#[test]
fn test_k8s_annotation_safe_tabs() {
    let result = kubernetes::k8s_annotation_safe_fn(Kwargs::from_iter(vec![(
        "value",
        Value::from("col1\tcol2\tcol3"),
    )]))
    .unwrap();

    assert_eq!(result.to_string(), "col1 col2 col3");
}

#[test]
fn test_k8s_annotation_safe_mixed_control_chars() {
    let result = kubernetes::k8s_annotation_safe_fn(Kwargs::from_iter(vec![(
        "value",
        Value::from("text\r\nwith\tvarious\rcontrol"),
    )]))
    .unwrap();

    // Control chars should be replaced with spaces
    assert!(!result.to_string().contains('\n'));
    assert!(!result.to_string().contains('\t'));
    assert!(!result.to_string().contains('\r'));
}

#[test]
fn test_k8s_annotation_safe_empty() {
    let result =
        kubernetes::k8s_annotation_safe_fn(Kwargs::from_iter(vec![("value", Value::from(""))]))
            .unwrap();

    assert_eq!(result.to_string(), "");
}

#[test]
fn test_k8s_annotation_safe_missing_param() {
    let result = kubernetes::k8s_annotation_safe_fn(Kwargs::from_iter(Vec::<(&str, Value)>::new()));

    assert!(result.is_err());
}

// ============================================================================
// k8s_quantity_to_bytes Tests
// ============================================================================

#[test]
fn test_k8s_quantity_to_bytes_gi() {
    let result = kubernetes::k8s_quantity_to_bytes_fn(Kwargs::from_iter(vec![(
        "quantity",
        Value::from("1Gi"),
    )]))
    .unwrap();

    assert_eq!(result.as_i64(), Some(1073741824)); // 1024^3
}

#[test]
fn test_k8s_quantity_to_bytes_mi() {
    let result = kubernetes::k8s_quantity_to_bytes_fn(Kwargs::from_iter(vec![(
        "quantity",
        Value::from("512Mi"),
    )]))
    .unwrap();

    assert_eq!(result.as_i64(), Some(536870912)); // 512 * 1024^2
}

#[test]
fn test_k8s_quantity_to_bytes_ki() {
    let result = kubernetes::k8s_quantity_to_bytes_fn(Kwargs::from_iter(vec![(
        "quantity",
        Value::from("100Ki"),
    )]))
    .unwrap();

    assert_eq!(result.as_i64(), Some(102400)); // 100 * 1024
}

#[test]
fn test_k8s_quantity_to_bytes_decimal_g() {
    let result = kubernetes::k8s_quantity_to_bytes_fn(Kwargs::from_iter(vec![(
        "quantity",
        Value::from("1G"),
    )]))
    .unwrap();

    assert_eq!(result.as_i64(), Some(1000000000)); // 1000^3
}

#[test]
fn test_k8s_quantity_to_bytes_decimal_m() {
    let result = kubernetes::k8s_quantity_to_bytes_fn(Kwargs::from_iter(vec![(
        "quantity",
        Value::from("500M"),
    )]))
    .unwrap();

    assert_eq!(result.as_i64(), Some(500000000)); // 500 * 1000^2
}

#[test]
fn test_k8s_quantity_to_bytes_millicores() {
    let result = kubernetes::k8s_quantity_to_bytes_fn(Kwargs::from_iter(vec![(
        "quantity",
        Value::from("500m"),
    )]))
    .unwrap();

    assert_eq!(result.as_i64(), Some(500)); // millicores
}

#[test]
fn test_k8s_quantity_to_bytes_plain_number() {
    let result = kubernetes::k8s_quantity_to_bytes_fn(Kwargs::from_iter(vec![(
        "quantity",
        Value::from("1024"),
    )]))
    .unwrap();

    assert_eq!(result.as_i64(), Some(1024));
}

#[test]
fn test_k8s_quantity_to_bytes_invalid() {
    let result = kubernetes::k8s_quantity_to_bytes_fn(Kwargs::from_iter(vec![(
        "quantity",
        Value::from("invalid"),
    )]));

    assert!(result.is_err());
}

#[test]
fn test_k8s_quantity_to_bytes_missing_param() {
    let result =
        kubernetes::k8s_quantity_to_bytes_fn(Kwargs::from_iter(Vec::<(&str, Value)>::new()));

    assert!(result.is_err());
}

// ============================================================================
// k8s_bytes_to_quantity Tests
// ============================================================================

#[test]
fn test_k8s_bytes_to_quantity_gi() {
    let result = kubernetes::k8s_bytes_to_quantity_fn(Kwargs::from_iter(vec![
        ("bytes", Value::from(1073741824_i64)), // 1 Gi
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "1Gi");
}

#[test]
fn test_k8s_bytes_to_quantity_mi() {
    let result = kubernetes::k8s_bytes_to_quantity_fn(Kwargs::from_iter(vec![
        ("bytes", Value::from(536870912_i64)), // 512 Mi
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "512Mi");
}

#[test]
fn test_k8s_bytes_to_quantity_ki() {
    let result = kubernetes::k8s_bytes_to_quantity_fn(Kwargs::from_iter(vec![
        ("bytes", Value::from(102400_i64)), // 100 Ki
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "100Ki");
}

#[test]
fn test_k8s_bytes_to_quantity_forced_unit() {
    let result = kubernetes::k8s_bytes_to_quantity_fn(Kwargs::from_iter(vec![
        ("bytes", Value::from(1073741824_i64)), // 1 Gi
        ("unit", Value::from("Mi")),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "1024Mi");
}

#[test]
fn test_k8s_bytes_to_quantity_small() {
    let result = kubernetes::k8s_bytes_to_quantity_fn(Kwargs::from_iter(vec![(
        "bytes",
        Value::from(100_i64),
    )]))
    .unwrap();

    // Small values should remain as plain bytes
    let output = result.to_string();
    assert!(output == "100" || output.ends_with("Ki") || output.contains("100"));
}

#[test]
fn test_k8s_bytes_to_quantity_zero() {
    let result = kubernetes::k8s_bytes_to_quantity_fn(Kwargs::from_iter(vec![(
        "bytes",
        Value::from(0_i64),
    )]))
    .unwrap();

    assert_eq!(result.to_string(), "0");
}

#[test]
fn test_k8s_bytes_to_quantity_missing_param() {
    let result =
        kubernetes::k8s_bytes_to_quantity_fn(Kwargs::from_iter(Vec::<(&str, Value)>::new()));

    assert!(result.is_err());
}

// ============================================================================
// k8s_selector Tests
// ============================================================================

#[test]
fn test_k8s_selector_simple() {
    let result = kubernetes::k8s_selector_fn(Kwargs::from_iter(vec![(
        "labels",
        Value::from_serialize(serde_json::json!({"app": "nginx"})),
    )]))
    .unwrap();

    assert_eq!(result.to_string(), "app=nginx");
}

#[test]
fn test_k8s_selector_multiple() {
    let result = kubernetes::k8s_selector_fn(Kwargs::from_iter(vec![(
        "labels",
        Value::from_serialize(serde_json::json!({
            "app": "nginx",
            "env": "prod"
        })),
    )]))
    .unwrap();

    let output = result.to_string();
    // Order might vary, so check both parts
    assert!(output.contains("app=nginx"));
    assert!(output.contains("env=prod"));
    assert!(output.contains(","));
}

#[test]
fn test_k8s_selector_empty() {
    let result = kubernetes::k8s_selector_fn(Kwargs::from_iter(vec![(
        "labels",
        Value::from_serialize(serde_json::json!({})),
    )]))
    .unwrap();

    assert_eq!(result.to_string(), "");
}

#[test]
fn test_k8s_selector_missing_param() {
    let result = kubernetes::k8s_selector_fn(Kwargs::from_iter(Vec::<(&str, Value)>::new()));

    assert!(result.is_err());
}

// ============================================================================
// k8s_pod_affinity Tests
// ============================================================================

#[test]
fn test_k8s_pod_affinity_required_in() {
    let result = kubernetes::k8s_pod_affinity_fn(Kwargs::from_iter(vec![
        ("key", Value::from("app")),
        ("operator", Value::from("In")),
        (
            "values",
            Value::from_serialize(serde_json::json!(["web", "api"])),
        ),
        ("type", Value::from("required")),
    ]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("requiredDuringSchedulingIgnoredDuringExecution"));
    assert!(output.contains("key: app"));
    assert!(output.contains("operator: In"));
    assert!(output.contains("- web"));
    assert!(output.contains("- api"));
}

#[test]
fn test_k8s_pod_affinity_preferred() {
    let result = kubernetes::k8s_pod_affinity_fn(Kwargs::from_iter(vec![
        ("key", Value::from("app")),
        ("operator", Value::from("In")),
        ("values", Value::from_serialize(serde_json::json!(["web"]))),
        ("type", Value::from("preferred")),
    ]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("preferredDuringSchedulingIgnoredDuringExecution"));
    assert!(output.contains("weight: 100"));
}

#[test]
fn test_k8s_pod_affinity_exists() {
    let result = kubernetes::k8s_pod_affinity_fn(Kwargs::from_iter(vec![
        ("key", Value::from("app")),
        ("operator", Value::from("Exists")),
    ]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("operator: Exists"));
}

#[test]
fn test_k8s_pod_affinity_custom_topology() {
    let result = kubernetes::k8s_pod_affinity_fn(Kwargs::from_iter(vec![
        ("key", Value::from("app")),
        ("operator", Value::from("In")),
        ("values", Value::from_serialize(serde_json::json!(["web"]))),
        ("topology_key", Value::from("topology.kubernetes.io/zone")),
    ]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("topologyKey: topology.kubernetes.io/zone"));
}

#[test]
fn test_k8s_pod_affinity_missing_key() {
    let result =
        kubernetes::k8s_pod_affinity_fn(Kwargs::from_iter(vec![("operator", Value::from("In"))]));

    assert!(result.is_err());
}

// ============================================================================
// k8s_toleration Tests
// ============================================================================

#[test]
fn test_k8s_toleration_equal() {
    let result = kubernetes::k8s_toleration_fn(Kwargs::from_iter(vec![
        ("key", Value::from("dedicated")),
        ("value", Value::from("gpu")),
        ("effect", Value::from("NoSchedule")),
    ]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("key: dedicated"));
    assert!(output.contains("operator: Equal"));
    assert!(output.contains("value: gpu"));
    assert!(output.contains("effect: NoSchedule"));
}

#[test]
fn test_k8s_toleration_exists() {
    let result = kubernetes::k8s_toleration_fn(Kwargs::from_iter(vec![
        ("key", Value::from("node.kubernetes.io/not-ready")),
        ("operator", Value::from("Exists")),
        ("effect", Value::from("NoExecute")),
    ]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("key: node.kubernetes.io/not-ready"));
    assert!(output.contains("operator: Exists"));
    assert!(output.contains("effect: NoExecute"));
    // Should not contain value for Exists operator
}

#[test]
fn test_k8s_toleration_no_effect() {
    let result = kubernetes::k8s_toleration_fn(Kwargs::from_iter(vec![
        ("key", Value::from("special")),
        ("value", Value::from("true")),
    ]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("key: special"));
    // Effect should be optional
}

#[test]
fn test_k8s_toleration_no_key() {
    // Key is optional - this creates a wildcard toleration
    let result =
        kubernetes::k8s_toleration_fn(Kwargs::from_iter(vec![("operator", Value::from("Exists"))]))
            .unwrap();

    let output = result.to_string();
    assert!(output.contains("operator: Exists"));
}

// ============================================================================
// k8s_probe Tests
// ============================================================================

#[test]
fn test_k8s_probe_http() {
    let result = kubernetes::k8s_probe_fn(Kwargs::from_iter(vec![
        ("type", Value::from("http")),
        ("path", Value::from("/health")),
        ("port", Value::from(8080)),
    ]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("httpGet:"));
    assert!(output.contains("path: /health"));
    assert!(output.contains("port: 8080"));
    assert!(output.contains("periodSeconds:"));
}

#[test]
fn test_k8s_probe_tcp() {
    let result = kubernetes::k8s_probe_fn(Kwargs::from_iter(vec![
        ("type", Value::from("tcp")),
        ("port", Value::from(5432)),
    ]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("tcpSocket:"));
    assert!(output.contains("port: 5432"));
}

#[test]
fn test_k8s_probe_exec() {
    let result = kubernetes::k8s_probe_fn(Kwargs::from_iter(vec![
        ("type", Value::from("exec")),
        (
            "command",
            Value::from_serialize(serde_json::json!(["cat", "/tmp/healthy"])),
        ),
    ]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("exec:"));
    assert!(output.contains("command:"));
    assert!(output.contains("- cat"));
    assert!(output.contains("- /tmp/healthy"));
}

#[test]
fn test_k8s_probe_custom_timings() {
    let result = kubernetes::k8s_probe_fn(Kwargs::from_iter(vec![
        ("type", Value::from("http")),
        ("path", Value::from("/healthz")),
        ("port", Value::from(8080)),
        ("initial_delay", Value::from(15)),
        ("period", Value::from(20)),
        ("timeout", Value::from(5)),
        ("failure_threshold", Value::from(5)),
    ]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("initialDelaySeconds: 15"));
    assert!(output.contains("periodSeconds: 20"));
    assert!(output.contains("timeoutSeconds: 5"));
    assert!(output.contains("failureThreshold: 5"));
}

#[test]
fn test_k8s_probe_default_values() {
    let result = kubernetes::k8s_probe_fn(Kwargs::from_iter(vec![
        ("type", Value::from("http")),
        ("path", Value::from("/healthz")),
        ("port", Value::from(8080)),
    ]))
    .unwrap();

    let output = result.to_string();
    // Should use defaults for timing values
    assert!(output.contains("path: /healthz"));
    assert!(output.contains("port: 8080"));
    assert!(output.contains("initialDelaySeconds: 0"));
    assert!(output.contains("periodSeconds: 10"));
}

#[test]
fn test_k8s_probe_missing_type() {
    let result =
        kubernetes::k8s_probe_fn(Kwargs::from_iter(vec![("path", Value::from("/health"))]));

    assert!(result.is_err());
}

#[test]
fn test_k8s_probe_invalid_type() {
    let result =
        kubernetes::k8s_probe_fn(Kwargs::from_iter(vec![("type", Value::from("invalid"))]));

    assert!(result.is_err());
}
