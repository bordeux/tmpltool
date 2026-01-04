use minijinja::Value;
use minijinja::value::Kwargs;
use tmpltool::functions::Function;
use tmpltool::functions::kubernetes::K8sResourceRequest;

// ============================================================================
// k8s_resource_request Tests
// ============================================================================

#[test]
fn test_k8s_resource_request_strings() {
    let result = K8sResourceRequest::call(Kwargs::from_iter(vec![
        ("cpu", Value::from("500m")),
        ("memory", Value::from("512Mi")),
    ]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("requests:"));
    assert!(output.contains("cpu: \"500m\""));
    assert!(output.contains("memory: \"512Mi\""));
}

#[test]
fn test_k8s_resource_request_numeric_cpu() {
    let result = K8sResourceRequest::call(Kwargs::from_iter(vec![
        ("cpu", Value::from(0.5)),
        ("memory", Value::from("512Mi")),
    ]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("cpu: \"500m\""));
}

#[test]
fn test_k8s_resource_request_numeric_cpu_whole() {
    let result = K8sResourceRequest::call(Kwargs::from_iter(vec![
        ("cpu", Value::from(2)),
        ("memory", Value::from("1Gi")),
    ]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("cpu: \"2000m\""));
}

#[test]
fn test_k8s_resource_request_numeric_memory_mi() {
    let result = K8sResourceRequest::call(Kwargs::from_iter(vec![
        ("cpu", Value::from("500m")),
        ("memory", Value::from(512)),
    ]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("memory: \"512Mi\""));
}

#[test]
fn test_k8s_resource_request_numeric_memory_gi() {
    let result = K8sResourceRequest::call(Kwargs::from_iter(vec![
        ("cpu", Value::from("1000m")),
        ("memory", Value::from(1024)),
    ]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("memory: \"1Gi\""));
}

#[test]
fn test_k8s_resource_request_numeric_memory_gi_fractional() {
    let result = K8sResourceRequest::call(Kwargs::from_iter(vec![
        ("cpu", Value::from("500m")),
        ("memory", Value::from(2560)),
    ]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("memory: \"2.50Gi\""));
}

#[test]
fn test_k8s_resource_request_both_numeric() {
    let result = K8sResourceRequest::call(Kwargs::from_iter(vec![
        ("cpu", Value::from(1.5)),
        ("memory", Value::from(2048)),
    ]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("cpu: \"1500m\""));
    assert!(output.contains("memory: \"2Gi\""));
}

#[test]
fn test_k8s_resource_request_yaml_format() {
    let result = K8sResourceRequest::call(Kwargs::from_iter(vec![
        ("cpu", Value::from("500m")),
        ("memory", Value::from("512Mi")),
    ]))
    .unwrap();

    let output = result.to_string();
    let lines: Vec<&str> = output.lines().collect();
    assert_eq!(lines.len(), 3);
    assert_eq!(lines[0], "requests:");
    assert!(lines[1].starts_with("  cpu:"));
    assert!(lines[2].starts_with("  memory:"));
}

#[test]
fn test_k8s_resource_request_error_invalid_cpu() {
    let result = K8sResourceRequest::call(Kwargs::from_iter(vec![
        ("cpu", Value::from(vec![1, 2, 3])),
        ("memory", Value::from("512Mi")),
    ]));

    assert!(result.is_err());
}

#[test]
fn test_k8s_resource_request_error_invalid_memory() {
    let result = K8sResourceRequest::call(Kwargs::from_iter(vec![
        ("cpu", Value::from("500m")),
        ("memory", Value::from(true)),
    ]));

    assert!(result.is_err());
}

#[test]
fn test_k8s_resource_request_missing_params() {
    let result = K8sResourceRequest::call(Kwargs::from_iter(vec![("cpu", Value::from("500m"))]));

    assert!(result.is_err());
}

// Note: k8s_label_safe_fn and k8s_dns_label_safe_fn tests removed - these functions
// are now in filter_functions/kubernetes.rs with dual function+filter syntax support.
// See tests/test_filters_integration.rs for integration tests of these filters.

// ============================================================================
// Additional Edge Case Tests for Coverage Improvement
// ============================================================================

// --- k8s_resource_request additional edge cases ---

#[test]
fn test_k8s_resource_request_fractional_mi() {
    // Memory value with fractional Mi (less than 1024)
    let result = K8sResourceRequest::call(Kwargs::from_iter(vec![
        ("cpu", Value::from("500m")),
        ("memory", Value::from(256.5)),
    ]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("memory: \"256.50Mi\""));
}

#[test]
fn test_k8s_resource_request_small_cpu() {
    let result = K8sResourceRequest::call(Kwargs::from_iter(vec![
        ("cpu", Value::from(0.1)),
        ("memory", Value::from("128Mi")),
    ]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("cpu: \"100m\""));
}

#[test]
fn test_k8s_resource_request_integer_cpu() {
    let result = K8sResourceRequest::call(Kwargs::from_iter(vec![
        ("cpu", Value::from(4)),
        ("memory", Value::from("4Gi")),
    ]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("cpu: \"4000m\""));
}

#[test]
fn test_k8s_resource_request_whole_gi_memory() {
    let result = K8sResourceRequest::call(Kwargs::from_iter(vec![
        ("cpu", Value::from("1000m")),
        ("memory", Value::from(2048)), // 2Gi in Mi
    ]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("memory: \"2Gi\""));
}

// --- k8s_resource_request error handling ---

#[test]
fn test_k8s_resource_request_null_cpu() {
    let result = K8sResourceRequest::call(Kwargs::from_iter(vec![
        ("cpu", Value::from(())),
        ("memory", Value::from("512Mi")),
    ]));

    assert!(result.is_err());
}

#[test]
fn test_k8s_resource_request_null_memory() {
    let result = K8sResourceRequest::call(Kwargs::from_iter(vec![
        ("cpu", Value::from("500m")),
        ("memory", Value::from(())),
    ]));

    assert!(result.is_err());
}
