//! Tests for Kubernetes filter-functions.
//!
//! Tests both function and filter syntax for:
//! - k8s_label_safe, k8s_dns_label_safe, k8s_annotation_safe

use minijinja::Value;
use minijinja::value::Kwargs;
use tmpltool::filter_functions::FilterFunction;
use tmpltool::filter_functions::kubernetes::{K8sAnnotationSafe, K8sDnsLabelSafe, K8sLabelSafe};

/// Helper to create empty kwargs
fn empty_kwargs() -> Kwargs {
    Kwargs::from_iter(Vec::<(&str, Value)>::new())
}

// ============================================
// K8sLabelSafe tests
// ============================================

#[test]
fn test_k8s_label_safe_filter_syntax() {
    let result =
        K8sLabelSafe::call_as_filter(&Value::from("My App (v2.0)"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "my-app-v2.0");
}

#[test]
fn test_k8s_label_safe_function_syntax() {
    let kwargs = Kwargs::from_iter(vec![("value", Value::from("My App (v2.0)"))]);
    let result = K8sLabelSafe::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "my-app-v2.0");
}

#[test]
fn test_k8s_label_safe_lowercase() {
    let result = K8sLabelSafe::call_as_filter(&Value::from("UPPERCASE"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "uppercase");
}

#[test]
fn test_k8s_label_safe_special_chars() {
    let result =
        K8sLabelSafe::call_as_filter(&Value::from("hello@world#test"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "hello-world-test");
}

#[test]
fn test_k8s_label_safe_consecutive_dashes() {
    let result = K8sLabelSafe::call_as_filter(&Value::from("a---b"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "a-b");
}

#[test]
fn test_k8s_label_safe_leading_trailing() {
    let result = K8sLabelSafe::call_as_filter(&Value::from("---test---"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "test");
}

#[test]
fn test_k8s_label_safe_empty_result() {
    let result = K8sLabelSafe::call_as_filter(&Value::from("@#$%"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "default");
}

#[test]
fn test_k8s_label_safe_max_length() {
    let long_string = "a".repeat(100);
    let result = K8sLabelSafe::call_as_filter(&Value::from(long_string), empty_kwargs()).unwrap();
    assert!(result.as_str().unwrap().len() <= 63);
}

#[test]
fn test_k8s_label_safe_error_not_string() {
    let result = K8sLabelSafe::call_as_filter(&Value::from(123), empty_kwargs());
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires a string")
    );
}

// ============================================
// K8sDnsLabelSafe tests
// ============================================

#[test]
fn test_k8s_dns_label_safe_filter_syntax() {
    let result =
        K8sDnsLabelSafe::call_as_filter(&Value::from("My Service Name"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "my-service-name");
}

#[test]
fn test_k8s_dns_label_safe_function_syntax() {
    let kwargs = Kwargs::from_iter(vec![("value", Value::from("My Service Name"))]);
    let result = K8sDnsLabelSafe::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "my-service-name");
}

#[test]
fn test_k8s_dns_label_safe_no_underscores() {
    let result =
        K8sDnsLabelSafe::call_as_filter(&Value::from("my_service"), empty_kwargs()).unwrap();
    // DNS labels don't allow underscores, they become dashes
    assert_eq!(result.as_str().unwrap(), "my-service");
}

#[test]
fn test_k8s_dns_label_safe_no_dots() {
    let result =
        K8sDnsLabelSafe::call_as_filter(&Value::from("my.service"), empty_kwargs()).unwrap();
    // DNS labels don't allow dots, they become dashes
    assert_eq!(result.as_str().unwrap(), "my-service");
}

#[test]
fn test_k8s_dns_label_safe_consecutive_dashes() {
    let result = K8sDnsLabelSafe::call_as_filter(&Value::from("a---b"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "a-b");
}

#[test]
fn test_k8s_dns_label_safe_leading_trailing_dashes() {
    let result =
        K8sDnsLabelSafe::call_as_filter(&Value::from("---test---"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "test");
}

#[test]
fn test_k8s_dns_label_safe_empty_result() {
    let result = K8sDnsLabelSafe::call_as_filter(&Value::from("!!!"), empty_kwargs()).unwrap();
    assert_eq!(result.as_str().unwrap(), "default");
}

#[test]
fn test_k8s_dns_label_safe_max_length() {
    let long_string = "a".repeat(100);
    let result =
        K8sDnsLabelSafe::call_as_filter(&Value::from(long_string), empty_kwargs()).unwrap();
    assert!(result.as_str().unwrap().len() <= 63);
}

#[test]
fn test_k8s_dns_label_safe_error_not_string() {
    let result = K8sDnsLabelSafe::call_as_filter(&Value::from(123), empty_kwargs());
    assert!(result.is_err());
}

// ============================================
// K8sAnnotationSafe tests
// ============================================

#[test]
fn test_k8s_annotation_safe_filter_syntax() {
    let result = K8sAnnotationSafe::call_as_filter(
        &Value::from("Some description with\nnewlines"),
        empty_kwargs(),
    )
    .unwrap();
    assert_eq!(result.as_str().unwrap(), "Some description with newlines");
}

#[test]
fn test_k8s_annotation_safe_function_syntax() {
    let kwargs = Kwargs::from_iter(vec![("value", Value::from("Description\nwith\nnewlines"))]);
    let result = K8sAnnotationSafe::call_as_function(kwargs).unwrap();
    assert_eq!(result.as_str().unwrap(), "Description with newlines");
}

#[test]
fn test_k8s_annotation_safe_tabs() {
    let result =
        K8sAnnotationSafe::call_as_filter(&Value::from("text\twith\ttabs"), empty_kwargs())
            .unwrap();
    assert_eq!(result.as_str().unwrap(), "text with tabs");
}

#[test]
fn test_k8s_annotation_safe_carriage_return() {
    let result =
        K8sAnnotationSafe::call_as_filter(&Value::from("text\r\nwith\r\ncrlf"), empty_kwargs())
            .unwrap();
    assert_eq!(result.as_str().unwrap(), "text  with  crlf");
}

#[test]
fn test_k8s_annotation_safe_preserves_unicode() {
    let result =
        K8sAnnotationSafe::call_as_filter(&Value::from("Hello 世界 مرحبا"), empty_kwargs())
            .unwrap();
    assert_eq!(result.as_str().unwrap(), "Hello 世界 مرحبا");
}

#[test]
fn test_k8s_annotation_safe_preserves_special_chars() {
    let result =
        K8sAnnotationSafe::call_as_filter(&Value::from("test@example.com"), empty_kwargs())
            .unwrap();
    assert_eq!(result.as_str().unwrap(), "test@example.com");
}

#[test]
fn test_k8s_annotation_safe_error_not_string() {
    let result = K8sAnnotationSafe::call_as_filter(&Value::from(123), empty_kwargs());
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("requires a string")
    );
}
