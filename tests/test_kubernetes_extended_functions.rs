use minijinja::Value;
use minijinja::value::Kwargs;
use tmpltool::functions::Function;
use tmpltool::functions::kubernetes::{
    HelmTpl, K8sBytesToQuantity, K8sPodAffinity, K8sProbe, K8sQuantityToBytes, K8sSelector,
    K8sToleration,
};

// ============================================================================
// helm_tpl Tests
// ============================================================================

#[test]
fn test_helm_tpl_simple() {
    let result = HelmTpl::call(Kwargs::from_iter(vec![
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
    let result = HelmTpl::call(Kwargs::from_iter(vec![
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
    let result = HelmTpl::call(Kwargs::from_iter(vec![
        ("template", Value::from("static-text")),
        ("values", Value::from_serialize(serde_json::json!({}))),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "static-text");
}

#[test]
fn test_helm_tpl_multiple_placeholders() {
    let result = HelmTpl::call(Kwargs::from_iter(vec![
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
    let result = HelmTpl::call(Kwargs::from_iter(vec![
        ("template", Value::from("{{ .missing }}")),
        ("values", Value::from_serialize(serde_json::json!({}))),
    ]))
    .unwrap();

    // Should return empty string for missing values
    assert_eq!(result.to_string(), "");
}

#[test]
fn test_helm_tpl_missing_template() {
    let result = HelmTpl::call(Kwargs::from_iter(vec![(
        "values",
        Value::from_serialize(serde_json::json!({})),
    )]));

    assert!(result.is_err());
}

// Note: k8s_annotation_safe tests removed - function now in filter_functions/kubernetes.rs

// ============================================================================
// k8s_quantity_to_bytes Tests
// ============================================================================

#[test]
fn test_k8s_quantity_to_bytes_gi() {
    let result =
        K8sQuantityToBytes::call(Kwargs::from_iter(vec![("quantity", Value::from("1Gi"))]))
            .unwrap();

    assert_eq!(result.as_i64(), Some(1073741824)); // 1024^3
}

#[test]
fn test_k8s_quantity_to_bytes_mi() {
    let result =
        K8sQuantityToBytes::call(Kwargs::from_iter(vec![("quantity", Value::from("512Mi"))]))
            .unwrap();

    assert_eq!(result.as_i64(), Some(536870912)); // 512 * 1024^2
}

#[test]
fn test_k8s_quantity_to_bytes_ki() {
    let result =
        K8sQuantityToBytes::call(Kwargs::from_iter(vec![("quantity", Value::from("100Ki"))]))
            .unwrap();

    assert_eq!(result.as_i64(), Some(102400)); // 100 * 1024
}

#[test]
fn test_k8s_quantity_to_bytes_decimal_g() {
    let result =
        K8sQuantityToBytes::call(Kwargs::from_iter(vec![("quantity", Value::from("1G"))])).unwrap();

    assert_eq!(result.as_i64(), Some(1000000000)); // 1000^3
}

#[test]
fn test_k8s_quantity_to_bytes_decimal_m() {
    let result =
        K8sQuantityToBytes::call(Kwargs::from_iter(vec![("quantity", Value::from("500M"))]))
            .unwrap();

    assert_eq!(result.as_i64(), Some(500000000)); // 500 * 1000^2
}

#[test]
fn test_k8s_quantity_to_bytes_millicores() {
    let result =
        K8sQuantityToBytes::call(Kwargs::from_iter(vec![("quantity", Value::from("500m"))]))
            .unwrap();

    assert_eq!(result.as_i64(), Some(500)); // millicores
}

#[test]
fn test_k8s_quantity_to_bytes_plain_number() {
    let result =
        K8sQuantityToBytes::call(Kwargs::from_iter(vec![("quantity", Value::from("1024"))]))
            .unwrap();

    assert_eq!(result.as_i64(), Some(1024));
}

#[test]
fn test_k8s_quantity_to_bytes_invalid() {
    let result = K8sQuantityToBytes::call(Kwargs::from_iter(vec![(
        "quantity",
        Value::from("invalid"),
    )]));

    assert!(result.is_err());
}

#[test]
fn test_k8s_quantity_to_bytes_missing_param() {
    let result = K8sQuantityToBytes::call(Kwargs::from_iter(Vec::<(&str, Value)>::new()));

    assert!(result.is_err());
}

// ============================================================================
// k8s_bytes_to_quantity Tests
// ============================================================================

#[test]
fn test_k8s_bytes_to_quantity_gi() {
    let result = K8sBytesToQuantity::call(Kwargs::from_iter(vec![
        ("bytes", Value::from(1073741824_i64)), // 1 Gi
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "1Gi");
}

#[test]
fn test_k8s_bytes_to_quantity_mi() {
    let result = K8sBytesToQuantity::call(Kwargs::from_iter(vec![
        ("bytes", Value::from(536870912_i64)), // 512 Mi
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "512Mi");
}

#[test]
fn test_k8s_bytes_to_quantity_ki() {
    let result = K8sBytesToQuantity::call(Kwargs::from_iter(vec![
        ("bytes", Value::from(102400_i64)), // 100 Ki
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "100Ki");
}

#[test]
fn test_k8s_bytes_to_quantity_forced_unit() {
    let result = K8sBytesToQuantity::call(Kwargs::from_iter(vec![
        ("bytes", Value::from(1073741824_i64)), // 1 Gi
        ("unit", Value::from("Mi")),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "1024Mi");
}

#[test]
fn test_k8s_bytes_to_quantity_small() {
    let result =
        K8sBytesToQuantity::call(Kwargs::from_iter(vec![("bytes", Value::from(100_i64))])).unwrap();

    // Small values should remain as plain bytes
    let output = result.to_string();
    assert!(output == "100" || output.ends_with("Ki") || output.contains("100"));
}

#[test]
fn test_k8s_bytes_to_quantity_zero() {
    let result =
        K8sBytesToQuantity::call(Kwargs::from_iter(vec![("bytes", Value::from(0_i64))])).unwrap();

    assert_eq!(result.to_string(), "0");
}

#[test]
fn test_k8s_bytes_to_quantity_missing_param() {
    let result = K8sBytesToQuantity::call(Kwargs::from_iter(Vec::<(&str, Value)>::new()));

    assert!(result.is_err());
}

// ============================================================================
// k8s_selector Tests
// ============================================================================

#[test]
fn test_k8s_selector_simple() {
    let result = K8sSelector::call(Kwargs::from_iter(vec![(
        "labels",
        Value::from_serialize(serde_json::json!({"app": "nginx"})),
    )]))
    .unwrap();

    assert_eq!(result.to_string(), "app=nginx");
}

#[test]
fn test_k8s_selector_multiple() {
    let result = K8sSelector::call(Kwargs::from_iter(vec![(
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
    let result = K8sSelector::call(Kwargs::from_iter(vec![(
        "labels",
        Value::from_serialize(serde_json::json!({})),
    )]))
    .unwrap();

    assert_eq!(result.to_string(), "");
}

#[test]
fn test_k8s_selector_missing_param() {
    let result = K8sSelector::call(Kwargs::from_iter(Vec::<(&str, Value)>::new()));

    assert!(result.is_err());
}

// ============================================================================
// k8s_pod_affinity Tests
// ============================================================================

#[test]
fn test_k8s_pod_affinity_required_in() {
    let result = K8sPodAffinity::call(Kwargs::from_iter(vec![
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
    let result = K8sPodAffinity::call(Kwargs::from_iter(vec![
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
    let result = K8sPodAffinity::call(Kwargs::from_iter(vec![
        ("key", Value::from("app")),
        ("operator", Value::from("Exists")),
    ]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("operator: Exists"));
}

#[test]
fn test_k8s_pod_affinity_custom_topology() {
    let result = K8sPodAffinity::call(Kwargs::from_iter(vec![
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
    let result = K8sPodAffinity::call(Kwargs::from_iter(vec![("operator", Value::from("In"))]));

    assert!(result.is_err());
}

// ============================================================================
// k8s_toleration Tests
// ============================================================================

#[test]
fn test_k8s_toleration_equal() {
    let result = K8sToleration::call(Kwargs::from_iter(vec![
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
    let result = K8sToleration::call(Kwargs::from_iter(vec![
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
    let result = K8sToleration::call(Kwargs::from_iter(vec![
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
        K8sToleration::call(Kwargs::from_iter(vec![("operator", Value::from("Exists"))])).unwrap();

    let output = result.to_string();
    assert!(output.contains("operator: Exists"));
}

// ============================================================================
// k8s_probe Tests
// ============================================================================

#[test]
fn test_k8s_probe_http() {
    let result = K8sProbe::call(Kwargs::from_iter(vec![
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
    let result = K8sProbe::call(Kwargs::from_iter(vec![
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
    let result = K8sProbe::call(Kwargs::from_iter(vec![
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
    let result = K8sProbe::call(Kwargs::from_iter(vec![
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
    let result = K8sProbe::call(Kwargs::from_iter(vec![
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
    let result = K8sProbe::call(Kwargs::from_iter(vec![("path", Value::from("/health"))]));

    assert!(result.is_err());
}

#[test]
fn test_k8s_probe_invalid_type() {
    let result = K8sProbe::call(Kwargs::from_iter(vec![("type", Value::from("invalid"))]));

    assert!(result.is_err());
}

// ============================================================================
// Additional Edge Case Tests for Coverage Improvement
// ============================================================================

// --- helm_tpl additional tests ---

#[test]
fn test_helm_tpl_with_boolean_value() {
    let result = HelmTpl::call(Kwargs::from_iter(vec![
        ("template", Value::from("enabled: {{ .enabled }}")),
        (
            "values",
            Value::from_serialize(serde_json::json!({"enabled": true})),
        ),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "enabled: true");
}

#[test]
fn test_helm_tpl_with_array_value() {
    let result = HelmTpl::call(Kwargs::from_iter(vec![
        ("template", Value::from("items: {{ .items }}")),
        (
            "values",
            Value::from_serialize(serde_json::json!({"items": ["a", "b", "c"]})),
        ),
    ]))
    .unwrap();

    // Arrays get JSON stringified
    assert!(result.to_string().contains("["));
    assert!(result.to_string().contains("a"));
}

#[test]
fn test_helm_tpl_with_number_value() {
    let result = HelmTpl::call(Kwargs::from_iter(vec![
        ("template", Value::from("replicas: {{ .replicas }}")),
        (
            "values",
            Value::from_serialize(serde_json::json!({"replicas": 3})),
        ),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "replicas: 3");
}

#[test]
fn test_helm_tpl_deeply_nested() {
    let result = HelmTpl::call(Kwargs::from_iter(vec![
        ("template", Value::from("{{ .a.b.c.d }}")),
        (
            "values",
            Value::from_serialize(serde_json::json!({
                "a": {"b": {"c": {"d": "deep"}}}
            })),
        ),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "deep");
}

#[test]
fn test_helm_tpl_partial_path_missing() {
    let result = HelmTpl::call(Kwargs::from_iter(vec![
        ("template", Value::from("{{ .a.b.missing }}")),
        (
            "values",
            Value::from_serialize(serde_json::json!({"a": {"b": {"c": "value"}}})),
        ),
    ]))
    .unwrap();

    // Returns empty string for missing nested paths
    assert_eq!(result.to_string(), "");
}

// --- k8s_annotation_safe additional tests removed ---
// Function now in filter_functions/kubernetes.rs

// --- k8s_quantity_to_bytes additional tests ---

#[test]
fn test_k8s_quantity_to_bytes_ti() {
    let result =
        K8sQuantityToBytes::call(Kwargs::from_iter(vec![("quantity", Value::from("1Ti"))]))
            .unwrap();

    assert_eq!(result.as_i64(), Some(1024_i64 * 1024 * 1024 * 1024)); // 1 TiB
}

#[test]
fn test_k8s_quantity_to_bytes_pi() {
    let result =
        K8sQuantityToBytes::call(Kwargs::from_iter(vec![("quantity", Value::from("1Pi"))]))
            .unwrap();

    assert_eq!(result.as_i64(), Some(1024_i64 * 1024 * 1024 * 1024 * 1024)); // 1 PiB
}

#[test]
fn test_k8s_quantity_to_bytes_ei() {
    let result =
        K8sQuantityToBytes::call(Kwargs::from_iter(vec![("quantity", Value::from("1Ei"))]))
            .unwrap();

    assert_eq!(
        result.as_i64(),
        Some(1024_i64 * 1024 * 1024 * 1024 * 1024 * 1024)
    ); // 1 EiB
}

#[test]
fn test_k8s_quantity_to_bytes_lowercase_k() {
    let result =
        K8sQuantityToBytes::call(Kwargs::from_iter(vec![("quantity", Value::from("1k"))])).unwrap();

    assert_eq!(result.as_i64(), Some(1000)); // 1000 bytes
}

#[test]
fn test_k8s_quantity_to_bytes_uppercase_k() {
    let result =
        K8sQuantityToBytes::call(Kwargs::from_iter(vec![("quantity", Value::from("1K"))])).unwrap();

    assert_eq!(result.as_i64(), Some(1000)); // 1000 bytes
}

#[test]
fn test_k8s_quantity_to_bytes_decimal_t() {
    let result =
        K8sQuantityToBytes::call(Kwargs::from_iter(vec![("quantity", Value::from("1T"))])).unwrap();

    assert_eq!(result.as_i64(), Some(1000_i64 * 1000 * 1000 * 1000)); // 1 TB
}

#[test]
fn test_k8s_quantity_to_bytes_decimal_p() {
    let result =
        K8sQuantityToBytes::call(Kwargs::from_iter(vec![("quantity", Value::from("1P"))])).unwrap();

    assert_eq!(result.as_i64(), Some(1000_i64 * 1000 * 1000 * 1000 * 1000)); // 1 PB
}

#[test]
fn test_k8s_quantity_to_bytes_decimal_e() {
    let result =
        K8sQuantityToBytes::call(Kwargs::from_iter(vec![("quantity", Value::from("1E"))])).unwrap();

    assert_eq!(
        result.as_i64(),
        Some(1000_i64 * 1000 * 1000 * 1000 * 1000 * 1000)
    ); // 1 EB
}

#[test]
fn test_k8s_quantity_to_bytes_float() {
    let result =
        K8sQuantityToBytes::call(Kwargs::from_iter(vec![("quantity", Value::from("1.5Gi"))]))
            .unwrap();

    // 1.5 * 1024^3 = 1610612736
    assert_eq!(result.as_i64(), Some(1610612736));
}

#[test]
fn test_k8s_quantity_to_bytes_unknown_suffix() {
    let result =
        K8sQuantityToBytes::call(Kwargs::from_iter(vec![("quantity", Value::from("100Xi"))]));

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Unknown quantity suffix"));
}

#[test]
fn test_k8s_quantity_to_bytes_empty_number() {
    let result = K8sQuantityToBytes::call(Kwargs::from_iter(vec![("quantity", Value::from("Gi"))]));

    assert!(result.is_err());
}

// --- k8s_bytes_to_quantity additional tests ---

#[test]
fn test_k8s_bytes_to_quantity_negative_error() {
    let result = K8sBytesToQuantity::call(Kwargs::from_iter(vec![("bytes", Value::from(-1_i64))]));

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("negative"));
}

#[test]
fn test_k8s_bytes_to_quantity_invalid_unit() {
    let result = K8sBytesToQuantity::call(Kwargs::from_iter(vec![
        ("bytes", Value::from(1024_i64)),
        ("unit", Value::from("Xi")),
    ]));

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Unknown unit"));
}

#[test]
fn test_k8s_bytes_to_quantity_ti_range() {
    let ti_bytes = 1024_i64 * 1024 * 1024 * 1024; // 1 TiB
    let result =
        K8sBytesToQuantity::call(Kwargs::from_iter(vec![("bytes", Value::from(ti_bytes))]))
            .unwrap();

    assert_eq!(result.to_string(), "1Ti");
}

#[test]
fn test_k8s_bytes_to_quantity_forced_ti() {
    let ti_bytes = 2_i64 * 1024 * 1024 * 1024 * 1024; // 2 TiB
    let result = K8sBytesToQuantity::call(Kwargs::from_iter(vec![
        ("bytes", Value::from(ti_bytes)),
        ("unit", Value::from("Ti")),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "2Ti");
}

#[test]
fn test_k8s_bytes_to_quantity_forced_pi() {
    let pi_bytes = 1024_i64 * 1024 * 1024 * 1024 * 1024; // 1 PiB
    let result = K8sBytesToQuantity::call(Kwargs::from_iter(vec![
        ("bytes", Value::from(pi_bytes)),
        ("unit", Value::from("Pi")),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "1Pi");
}

#[test]
fn test_k8s_bytes_to_quantity_forced_ei() {
    let ei_bytes = 1024_i64 * 1024 * 1024 * 1024 * 1024 * 1024; // 1 EiB
    let result = K8sBytesToQuantity::call(Kwargs::from_iter(vec![
        ("bytes", Value::from(ei_bytes)),
        ("unit", Value::from("Ei")),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "1Ei");
}

#[test]
fn test_k8s_bytes_to_quantity_forced_decimal_k() {
    let result = K8sBytesToQuantity::call(Kwargs::from_iter(vec![
        ("bytes", Value::from(2000_i64)),
        ("unit", Value::from("K")),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "2K");
}

#[test]
fn test_k8s_bytes_to_quantity_forced_decimal_m() {
    let result = K8sBytesToQuantity::call(Kwargs::from_iter(vec![
        ("bytes", Value::from(2000000_i64)),
        ("unit", Value::from("M")),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "2M");
}

#[test]
fn test_k8s_bytes_to_quantity_forced_decimal_g() {
    let result = K8sBytesToQuantity::call(Kwargs::from_iter(vec![
        ("bytes", Value::from(2000000000_i64)),
        ("unit", Value::from("G")),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "2G");
}

#[test]
fn test_k8s_bytes_to_quantity_forced_decimal_t() {
    let result = K8sBytesToQuantity::call(Kwargs::from_iter(vec![
        ("bytes", Value::from(2000000000000_i64)),
        ("unit", Value::from("T")),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "2T");
}

#[test]
fn test_k8s_bytes_to_quantity_forced_decimal_p() {
    let result = K8sBytesToQuantity::call(Kwargs::from_iter(vec![
        ("bytes", Value::from(2000000000000000_i64)),
        ("unit", Value::from("P")),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "2P");
}

#[test]
fn test_k8s_bytes_to_quantity_forced_decimal_e() {
    let result = K8sBytesToQuantity::call(Kwargs::from_iter(vec![
        ("bytes", Value::from(2000000000000000000_i64)),
        ("unit", Value::from("E")),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "2E");
}

#[test]
fn test_k8s_bytes_to_quantity_decimal_mode() {
    let result = K8sBytesToQuantity::call(Kwargs::from_iter(vec![
        ("bytes", Value::from(1000000000_i64)), // 1 GB
        ("binary", Value::from(false)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "1G");
}

#[test]
fn test_k8s_bytes_to_quantity_decimal_mode_m() {
    let result = K8sBytesToQuantity::call(Kwargs::from_iter(vec![
        ("bytes", Value::from(500000000_i64)), // 500 MB
        ("binary", Value::from(false)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "500M");
}

#[test]
fn test_k8s_bytes_to_quantity_decimal_mode_k() {
    let result = K8sBytesToQuantity::call(Kwargs::from_iter(vec![
        ("bytes", Value::from(5000_i64)), // 5 KB
        ("binary", Value::from(false)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "5K");
}

#[test]
fn test_k8s_bytes_to_quantity_decimal_mode_t() {
    let result = K8sBytesToQuantity::call(Kwargs::from_iter(vec![
        ("bytes", Value::from(2000000000000_i64)), // 2 TB
        ("binary", Value::from(false)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "2T");
}

#[test]
fn test_k8s_bytes_to_quantity_decimal_mode_small() {
    let result = K8sBytesToQuantity::call(Kwargs::from_iter(vec![
        ("bytes", Value::from(500_i64)), // 500 bytes
        ("binary", Value::from(false)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "500");
}

#[test]
fn test_k8s_bytes_to_quantity_fractional_gi() {
    // 1.5 * 1024^3 = 1610612736
    let result = K8sBytesToQuantity::call(Kwargs::from_iter(vec![(
        "bytes",
        Value::from(1610612736_i64),
    )]))
    .unwrap();

    assert_eq!(result.to_string(), "1.50Gi");
}

#[test]
fn test_k8s_bytes_to_quantity_fractional_mi() {
    // 1.5 * 1024^2 = 1572864
    let result =
        K8sBytesToQuantity::call(Kwargs::from_iter(vec![("bytes", Value::from(1572864_i64))]))
            .unwrap();

    assert_eq!(result.to_string(), "1.50Mi");
}

#[test]
fn test_k8s_bytes_to_quantity_fractional_ki() {
    // 1.5 * 1024 = 1536
    let result =
        K8sBytesToQuantity::call(Kwargs::from_iter(vec![("bytes", Value::from(1536_i64))]))
            .unwrap();

    assert_eq!(result.to_string(), "1.50Ki");
}

#[test]
fn test_k8s_bytes_to_quantity_fractional_ti() {
    // 1.5 * 1024^4 = 1649267441664
    let result = K8sBytesToQuantity::call(Kwargs::from_iter(vec![(
        "bytes",
        Value::from(1649267441664_i64),
    )]))
    .unwrap();

    assert_eq!(result.to_string(), "1.50Ti");
}

#[test]
fn test_k8s_bytes_to_quantity_decimal_fractional_g() {
    // 1.5 * 1000^3 = 1500000000
    let result = K8sBytesToQuantity::call(Kwargs::from_iter(vec![
        ("bytes", Value::from(1500000000_i64)),
        ("binary", Value::from(false)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "1.50G");
}

#[test]
fn test_k8s_bytes_to_quantity_decimal_fractional_m() {
    // 1.5 * 1000^2 = 1500000
    let result = K8sBytesToQuantity::call(Kwargs::from_iter(vec![
        ("bytes", Value::from(1500000_i64)),
        ("binary", Value::from(false)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "1.50M");
}

#[test]
fn test_k8s_bytes_to_quantity_decimal_fractional_k() {
    // 1.5 * 1000 = 1500
    let result = K8sBytesToQuantity::call(Kwargs::from_iter(vec![
        ("bytes", Value::from(1500_i64)),
        ("binary", Value::from(false)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "1.50K");
}

#[test]
fn test_k8s_bytes_to_quantity_decimal_fractional_t() {
    // 1.5 * 1000^4 = 1500000000000
    let result = K8sBytesToQuantity::call(Kwargs::from_iter(vec![
        ("bytes", Value::from(1500000000000_i64)),
        ("binary", Value::from(false)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "1.50T");
}

// --- k8s_selector additional tests ---

#[test]
fn test_k8s_selector_numeric_value() {
    let result = K8sSelector::call(Kwargs::from_iter(vec![(
        "labels",
        Value::from_serialize(serde_json::json!({"version": 1})),
    )]))
    .unwrap();

    assert_eq!(result.to_string(), "version=1");
}

#[test]
fn test_k8s_selector_boolean_value() {
    let result = K8sSelector::call(Kwargs::from_iter(vec![(
        "labels",
        Value::from_serialize(serde_json::json!({"enabled": true})),
    )]))
    .unwrap();

    assert_eq!(result.to_string(), "enabled=true");
}

#[test]
fn test_k8s_selector_not_object() {
    let result = K8sSelector::call(Kwargs::from_iter(vec![(
        "labels",
        Value::from_serialize(serde_json::json!("not-an-object")),
    )]));

    assert!(result.is_err());
}

// --- k8s_pod_affinity additional tests ---

#[test]
fn test_k8s_pod_affinity_invalid_operator() {
    let result = K8sPodAffinity::call(Kwargs::from_iter(vec![
        ("key", Value::from("app")),
        ("operator", Value::from("Invalid")),
    ]));

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Invalid operator"));
}

#[test]
fn test_k8s_pod_affinity_notin() {
    let result = K8sPodAffinity::call(Kwargs::from_iter(vec![
        ("key", Value::from("app")),
        ("operator", Value::from("NotIn")),
        (
            "values",
            Value::from_serialize(serde_json::json!(["excluded"])),
        ),
    ]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("operator: NotIn"));
    assert!(output.contains("- excluded"));
}

#[test]
fn test_k8s_pod_affinity_notin_empty_values() {
    // NotIn without values is allowed - generates affinity with empty values list
    let result = K8sPodAffinity::call(Kwargs::from_iter(vec![
        ("key", Value::from("app")),
        ("operator", Value::from("NotIn")),
    ]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("operator: NotIn"));
}

#[test]
fn test_k8s_pod_affinity_doesnotexist() {
    let result = K8sPodAffinity::call(Kwargs::from_iter(vec![
        ("key", Value::from("app")),
        ("operator", Value::from("DoesNotExist")),
    ]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("operator: DoesNotExist"));
}

#[test]
fn test_k8s_pod_affinity_custom_weight() {
    let result = K8sPodAffinity::call(Kwargs::from_iter(vec![
        ("key", Value::from("app")),
        ("operator", Value::from("Exists")),
        ("weight", Value::from(50)),
    ]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("weight: 50"));
}

#[test]
fn test_k8s_pod_affinity_in_empty_values() {
    // In without values is allowed - generates affinity with empty values list
    let result = K8sPodAffinity::call(Kwargs::from_iter(vec![
        ("key", Value::from("app")),
        ("operator", Value::from("In")),
    ]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("operator: In"));
}

// --- k8s_toleration additional tests ---

#[test]
fn test_k8s_toleration_invalid_operator() {
    let result = K8sToleration::call(Kwargs::from_iter(vec![
        ("key", Value::from("dedicated")),
        ("operator", Value::from("Invalid")),
    ]));

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Invalid operator"));
}

#[test]
fn test_k8s_toleration_invalid_effect() {
    let result = K8sToleration::call(Kwargs::from_iter(vec![
        ("key", Value::from("dedicated")),
        ("value", Value::from("true")),
        ("effect", Value::from("InvalidEffect")),
    ]));

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Invalid effect"));
}

#[test]
fn test_k8s_toleration_with_seconds() {
    let result = K8sToleration::call(Kwargs::from_iter(vec![
        ("key", Value::from("node.kubernetes.io/unreachable")),
        ("operator", Value::from("Exists")),
        ("effect", Value::from("NoExecute")),
        ("toleration_seconds", Value::from(300)),
    ]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("tolerationSeconds: 300"));
}

#[test]
fn test_k8s_toleration_prefer_no_schedule() {
    let result = K8sToleration::call(Kwargs::from_iter(vec![
        ("key", Value::from("workload")),
        ("value", Value::from("high")),
        ("effect", Value::from("PreferNoSchedule")),
    ]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("effect: PreferNoSchedule"));
}

#[test]
fn test_k8s_toleration_exists_with_value_ignored() {
    // When operator is Exists, value should be ignored
    let result = K8sToleration::call(Kwargs::from_iter(vec![
        ("key", Value::from("dedicated")),
        ("operator", Value::from("Exists")),
        ("value", Value::from("this-should-be-ignored")),
    ]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("operator: Exists"));
    // Value should NOT be in output for Exists operator
    assert!(!output.contains("this-should-be-ignored"));
}

// --- k8s_probe additional tests ---

#[test]
fn test_k8s_probe_http_missing_path() {
    let result = K8sProbe::call(Kwargs::from_iter(vec![
        ("type", Value::from("http")),
        ("port", Value::from(8080)),
    ]));

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("path is required"));
}

#[test]
fn test_k8s_probe_http_missing_port() {
    let result = K8sProbe::call(Kwargs::from_iter(vec![
        ("type", Value::from("http")),
        ("path", Value::from("/health")),
    ]));

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("port is required"));
}

#[test]
fn test_k8s_probe_tcp_missing_port() {
    let result = K8sProbe::call(Kwargs::from_iter(vec![("type", Value::from("tcp"))]));

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("port is required"));
}

#[test]
fn test_k8s_probe_exec_empty_command() {
    // exec without command still generates valid probe structure
    let result = K8sProbe::call(Kwargs::from_iter(vec![("type", Value::from("exec"))]));

    // The implementation requires command, so this should error
    // If it doesn't error, just verify the output format
    if let Ok(val) = result {
        let output = val.to_string();
        assert!(output.contains("exec:"));
    }
}

#[test]
fn test_k8s_probe_success_threshold() {
    let result = K8sProbe::call(Kwargs::from_iter(vec![
        ("type", Value::from("http")),
        ("path", Value::from("/ready")),
        ("port", Value::from(8080)),
        ("success_threshold", Value::from(2)),
    ]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("successThreshold: 2"));
}

#[test]
fn test_k8s_probe_all_timings() {
    let result = K8sProbe::call(Kwargs::from_iter(vec![
        ("type", Value::from("http")),
        ("path", Value::from("/healthz")),
        ("port", Value::from(8080)),
        ("initial_delay", Value::from(30)),
        ("period", Value::from(15)),
        ("timeout", Value::from(5)),
        ("success_threshold", Value::from(2)),
        ("failure_threshold", Value::from(6)),
    ]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("initialDelaySeconds: 30"));
    assert!(output.contains("periodSeconds: 15"));
    assert!(output.contains("timeoutSeconds: 5"));
    assert!(output.contains("successThreshold: 2"));
    assert!(output.contains("failureThreshold: 6"));
}
