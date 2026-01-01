use minijinja::Value;
use minijinja::value::Kwargs;
use tmpltool::functions::kubernetes;

// ============================================================================
// k8s_resource_request Tests
// ============================================================================

#[test]
fn test_k8s_resource_request_strings() {
    let result = kubernetes::k8s_resource_request_fn(Kwargs::from_iter(vec![
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
    let result = kubernetes::k8s_resource_request_fn(Kwargs::from_iter(vec![
        ("cpu", Value::from(0.5)),
        ("memory", Value::from("512Mi")),
    ]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("cpu: \"500m\""));
}

#[test]
fn test_k8s_resource_request_numeric_cpu_whole() {
    let result = kubernetes::k8s_resource_request_fn(Kwargs::from_iter(vec![
        ("cpu", Value::from(2)),
        ("memory", Value::from("1Gi")),
    ]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("cpu: \"2000m\""));
}

#[test]
fn test_k8s_resource_request_numeric_memory_mi() {
    let result = kubernetes::k8s_resource_request_fn(Kwargs::from_iter(vec![
        ("cpu", Value::from("500m")),
        ("memory", Value::from(512)),
    ]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("memory: \"512Mi\""));
}

#[test]
fn test_k8s_resource_request_numeric_memory_gi() {
    let result = kubernetes::k8s_resource_request_fn(Kwargs::from_iter(vec![
        ("cpu", Value::from("1000m")),
        ("memory", Value::from(1024)),
    ]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("memory: \"1Gi\""));
}

#[test]
fn test_k8s_resource_request_numeric_memory_gi_fractional() {
    let result = kubernetes::k8s_resource_request_fn(Kwargs::from_iter(vec![
        ("cpu", Value::from("500m")),
        ("memory", Value::from(2560)),
    ]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("memory: \"2.50Gi\""));
}

#[test]
fn test_k8s_resource_request_both_numeric() {
    let result = kubernetes::k8s_resource_request_fn(Kwargs::from_iter(vec![
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
    let result = kubernetes::k8s_resource_request_fn(Kwargs::from_iter(vec![
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
    let result = kubernetes::k8s_resource_request_fn(Kwargs::from_iter(vec![
        ("cpu", Value::from(vec![1, 2, 3])),
        ("memory", Value::from("512Mi")),
    ]));

    assert!(result.is_err());
}

#[test]
fn test_k8s_resource_request_error_invalid_memory() {
    let result = kubernetes::k8s_resource_request_fn(Kwargs::from_iter(vec![
        ("cpu", Value::from("500m")),
        ("memory", Value::from(true)),
    ]));

    assert!(result.is_err());
}

#[test]
fn test_k8s_resource_request_missing_params() {
    let result =
        kubernetes::k8s_resource_request_fn(Kwargs::from_iter(vec![("cpu", Value::from("500m"))]));

    assert!(result.is_err());
}

// ============================================================================
// k8s_label_safe Tests
// ============================================================================

#[test]
fn test_k8s_label_safe_simple() {
    let result =
        kubernetes::k8s_label_safe_fn(Kwargs::from_iter(vec![("value", Value::from("my-app"))]))
            .unwrap();

    assert_eq!(result.to_string(), "my-app");
}

#[test]
fn test_k8s_label_safe_uppercase() {
    let result =
        kubernetes::k8s_label_safe_fn(Kwargs::from_iter(vec![("value", Value::from("MyApp"))]))
            .unwrap();

    assert_eq!(result.to_string(), "myapp");
}

#[test]
fn test_k8s_label_safe_spaces() {
    let result = kubernetes::k8s_label_safe_fn(Kwargs::from_iter(vec![(
        "value",
        Value::from("My App Name"),
    )]))
    .unwrap();

    assert_eq!(result.to_string(), "my-app-name");
}

#[test]
fn test_k8s_label_safe_special_chars() {
    let result = kubernetes::k8s_label_safe_fn(Kwargs::from_iter(vec![(
        "value",
        Value::from("My App (v2.0)!"),
    )]))
    .unwrap();

    assert_eq!(result.to_string(), "my-app-v2.0");
}

#[test]
fn test_k8s_label_safe_leading_trailing() {
    let result = kubernetes::k8s_label_safe_fn(Kwargs::from_iter(vec![(
        "value",
        Value::from("--my-app--"),
    )]))
    .unwrap();

    assert_eq!(result.to_string(), "my-app");
}

#[test]
fn test_k8s_label_safe_underscores_dots() {
    let result =
        kubernetes::k8s_label_safe_fn(Kwargs::from_iter(vec![("value", Value::from("my_app.v1"))]))
            .unwrap();

    assert_eq!(result.to_string(), "my_app.v1");
}

#[test]
fn test_k8s_label_safe_multiple_dashes() {
    let result =
        kubernetes::k8s_label_safe_fn(Kwargs::from_iter(vec![("value", Value::from("my---app"))]))
            .unwrap();

    assert_eq!(result.to_string(), "my-app");
}

#[test]
fn test_k8s_label_safe_long_string() {
    let long_str =
        "this-is-a-very-long-label-name-that-exceeds-the-kubernetes-maximum-label-length-limit";
    let result =
        kubernetes::k8s_label_safe_fn(Kwargs::from_iter(vec![("value", Value::from(long_str))]))
            .unwrap();

    let output = result.to_string();
    assert!(output.len() <= 63);
    assert!(output.chars().last().unwrap().is_ascii_alphanumeric());
}

#[test]
fn test_k8s_label_safe_empty_result() {
    let result =
        kubernetes::k8s_label_safe_fn(Kwargs::from_iter(vec![("value", Value::from("!!!"))]))
            .unwrap();

    assert_eq!(result.to_string(), "default");
}

#[test]
fn test_k8s_label_safe_missing_param() {
    let result = kubernetes::k8s_label_safe_fn(Kwargs::from_iter(vec![("dummy", Value::from(0))]));

    assert!(result.is_err());
}

// ============================================================================
// k8s_dns_label_safe Tests
// ============================================================================

#[test]
fn test_k8s_dns_label_safe_simple() {
    let result = kubernetes::k8s_dns_label_safe_fn(Kwargs::from_iter(vec![(
        "value",
        Value::from("my-service"),
    )]))
    .unwrap();

    assert_eq!(result.to_string(), "my-service");
}

#[test]
fn test_k8s_dns_label_safe_uppercase() {
    let result = kubernetes::k8s_dns_label_safe_fn(Kwargs::from_iter(vec![(
        "value",
        Value::from("MyService"),
    )]))
    .unwrap();

    assert_eq!(result.to_string(), "myservice");
}

#[test]
fn test_k8s_dns_label_safe_spaces() {
    let result = kubernetes::k8s_dns_label_safe_fn(Kwargs::from_iter(vec![(
        "value",
        Value::from("My Service Name"),
    )]))
    .unwrap();

    assert_eq!(result.to_string(), "my-service-name");
}

#[test]
fn test_k8s_dns_label_safe_no_underscores() {
    let result = kubernetes::k8s_dns_label_safe_fn(Kwargs::from_iter(vec![(
        "value",
        Value::from("my_service"),
    )]))
    .unwrap();

    assert_eq!(result.to_string(), "my-service");
}

#[test]
fn test_k8s_dns_label_safe_no_dots() {
    let result = kubernetes::k8s_dns_label_safe_fn(Kwargs::from_iter(vec![(
        "value",
        Value::from("my.service.v1"),
    )]))
    .unwrap();

    assert_eq!(result.to_string(), "my-service-v1");
}

#[test]
fn test_k8s_dns_label_safe_multiple_dashes() {
    let result = kubernetes::k8s_dns_label_safe_fn(Kwargs::from_iter(vec![(
        "value",
        Value::from("my---service"),
    )]))
    .unwrap();

    assert_eq!(result.to_string(), "my-service");
}

#[test]
fn test_k8s_dns_label_safe_leading_trailing_dashes() {
    let result = kubernetes::k8s_dns_label_safe_fn(Kwargs::from_iter(vec![(
        "value",
        Value::from("--my-service--"),
    )]))
    .unwrap();

    assert_eq!(result.to_string(), "my-service");
}

#[test]
fn test_k8s_dns_label_safe_long_string() {
    let long_str =
        "this-is-a-very-long-dns-label-that-exceeds-the-kubernetes-maximum-dns-label-length-limit";
    let result = kubernetes::k8s_dns_label_safe_fn(Kwargs::from_iter(vec![(
        "value",
        Value::from(long_str),
    )]))
    .unwrap();

    let output = result.to_string();
    assert!(output.len() <= 63);
    assert!(!output.ends_with('-'));
}

#[test]
fn test_k8s_dns_label_safe_empty_result() {
    let result =
        kubernetes::k8s_dns_label_safe_fn(Kwargs::from_iter(vec![("value", Value::from("!!!"))]))
            .unwrap();

    assert_eq!(result.to_string(), "default");
}

#[test]
fn test_k8s_dns_label_safe_missing_param() {
    let result =
        kubernetes::k8s_dns_label_safe_fn(Kwargs::from_iter(vec![("dummy", Value::from(0))]));

    assert!(result.is_err());
}
