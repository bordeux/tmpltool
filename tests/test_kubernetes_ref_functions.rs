use minijinja::Value;
use minijinja::value::Kwargs;
use tmpltool::functions::kubernetes;

// ============================================================================
// k8s_env_var_ref Tests
// ============================================================================

#[test]
fn test_k8s_env_var_ref_configmap_default() {
    let result = kubernetes::k8s_env_var_ref_fn(Kwargs::from_iter(vec![(
        "var_name",
        Value::from("DATABASE_HOST"),
    )]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("valueFrom:"));
    assert!(output.contains("configMapKeyRef:"));
    assert!(output.contains("name: database-host"));
    assert!(output.contains("key: DATABASE_HOST"));
}

#[test]
fn test_k8s_env_var_ref_configmap_explicit() {
    let result = kubernetes::k8s_env_var_ref_fn(Kwargs::from_iter(vec![
        ("var_name", Value::from("DB_HOST")),
        ("source", Value::from("configmap")),
        ("name", Value::from("app-config")),
    ]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("configMapKeyRef:"));
    assert!(output.contains("name: app-config"));
    assert!(output.contains("key: DB_HOST"));
}

#[test]
fn test_k8s_env_var_ref_secret() {
    let result = kubernetes::k8s_env_var_ref_fn(Kwargs::from_iter(vec![
        ("var_name", Value::from("API_KEY")),
        ("source", Value::from("secret")),
        ("name", Value::from("api-secrets")),
    ]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("secretKeyRef:"));
    assert!(output.contains("name: api-secrets"));
    assert!(output.contains("key: API_KEY"));
}

#[test]
fn test_k8s_env_var_ref_secret_auto_name() {
    let result = kubernetes::k8s_env_var_ref_fn(Kwargs::from_iter(vec![
        ("var_name", Value::from("DB_PASSWORD")),
        ("source", Value::from("secret")),
    ]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("secretKeyRef:"));
    assert!(output.contains("name: db-password"));
    assert!(output.contains("key: DB_PASSWORD"));
}

#[test]
fn test_k8s_env_var_ref_invalid_source() {
    let result = kubernetes::k8s_env_var_ref_fn(Kwargs::from_iter(vec![
        ("var_name", Value::from("TEST")),
        ("source", Value::from("invalid")),
    ]));

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("must be 'configmap' or 'secret'")
    );
}

#[test]
fn test_k8s_env_var_ref_missing_var_name() {
    let result =
        kubernetes::k8s_env_var_ref_fn(Kwargs::from_iter(vec![("source", Value::from("secret"))]));

    assert!(result.is_err());
}

// ============================================================================
// k8s_secret_ref Tests
// ============================================================================

#[test]
fn test_k8s_secret_ref_basic() {
    let result = kubernetes::k8s_secret_ref_fn(Kwargs::from_iter(vec![
        ("secret_name", Value::from("db-credentials")),
        ("key", Value::from("password")),
    ]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("valueFrom:"));
    assert!(output.contains("secretKeyRef:"));
    assert!(output.contains("name: db-credentials"));
    assert!(output.contains("key: password"));
    assert!(!output.contains("optional"));
}

#[test]
fn test_k8s_secret_ref_optional() {
    let result = kubernetes::k8s_secret_ref_fn(Kwargs::from_iter(vec![
        ("secret_name", Value::from("tokens")),
        ("key", Value::from("api_token")),
        ("optional", Value::from(true)),
    ]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("secretKeyRef:"));
    assert!(output.contains("name: tokens"));
    assert!(output.contains("key: api_token"));
    assert!(output.contains("optional: true"));
}

#[test]
fn test_k8s_secret_ref_optional_false() {
    let result = kubernetes::k8s_secret_ref_fn(Kwargs::from_iter(vec![
        ("secret_name", Value::from("creds")),
        ("key", Value::from("key1")),
        ("optional", Value::from(false)),
    ]))
    .unwrap();

    let output = result.to_string();
    assert!(!output.contains("optional"));
}

#[test]
fn test_k8s_secret_ref_missing_secret_name() {
    let result =
        kubernetes::k8s_secret_ref_fn(Kwargs::from_iter(vec![("key", Value::from("password"))]));

    assert!(result.is_err());
}

#[test]
fn test_k8s_secret_ref_missing_key() {
    let result = kubernetes::k8s_secret_ref_fn(Kwargs::from_iter(vec![(
        "secret_name",
        Value::from("db-creds"),
    )]));

    assert!(result.is_err());
}

#[test]
fn test_k8s_secret_ref_complex_names() {
    let result = kubernetes::k8s_secret_ref_fn(Kwargs::from_iter(vec![
        ("secret_name", Value::from("my-app-tls-cert")),
        ("key", Value::from("tls.crt")),
    ]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("name: my-app-tls-cert"));
    assert!(output.contains("key: tls.crt"));
}

// ============================================================================
// k8s_configmap_ref Tests
// ============================================================================

#[test]
fn test_k8s_configmap_ref_basic() {
    let result = kubernetes::k8s_configmap_ref_fn(Kwargs::from_iter(vec![
        ("configmap_name", Value::from("app-config")),
        ("key", Value::from("database_host")),
    ]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("valueFrom:"));
    assert!(output.contains("configMapKeyRef:"));
    assert!(output.contains("name: app-config"));
    assert!(output.contains("key: database_host"));
    assert!(!output.contains("optional"));
}

#[test]
fn test_k8s_configmap_ref_optional() {
    let result = kubernetes::k8s_configmap_ref_fn(Kwargs::from_iter(vec![
        ("configmap_name", Value::from("features")),
        ("key", Value::from("new_ui")),
        ("optional", Value::from(true)),
    ]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("configMapKeyRef:"));
    assert!(output.contains("name: features"));
    assert!(output.contains("key: new_ui"));
    assert!(output.contains("optional: true"));
}

#[test]
fn test_k8s_configmap_ref_optional_false() {
    let result = kubernetes::k8s_configmap_ref_fn(Kwargs::from_iter(vec![
        ("configmap_name", Value::from("config")),
        ("key", Value::from("key1")),
        ("optional", Value::from(false)),
    ]))
    .unwrap();

    let output = result.to_string();
    assert!(!output.contains("optional"));
}

#[test]
fn test_k8s_configmap_ref_missing_configmap_name() {
    let result =
        kubernetes::k8s_configmap_ref_fn(Kwargs::from_iter(vec![("key", Value::from("db_host"))]));

    assert!(result.is_err());
}

#[test]
fn test_k8s_configmap_ref_missing_key() {
    let result = kubernetes::k8s_configmap_ref_fn(Kwargs::from_iter(vec![(
        "configmap_name",
        Value::from("app-config"),
    )]));

    assert!(result.is_err());
}

#[test]
fn test_k8s_configmap_ref_complex_names() {
    let result = kubernetes::k8s_configmap_ref_fn(Kwargs::from_iter(vec![
        ("configmap_name", Value::from("my-app-config-v2")),
        ("key", Value::from("redis.url")),
    ]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("name: my-app-config-v2"));
    assert!(output.contains("key: redis.url"));
}

// ============================================================================
// Integration Tests (combined usage)
// ============================================================================

#[test]
fn test_all_ref_types_together() {
    // Test that all three reference types produce valid YAML
    let env_var = kubernetes::k8s_env_var_ref_fn(Kwargs::from_iter(vec![
        ("var_name", Value::from("HOST")),
        ("source", Value::from("configmap")),
        ("name", Value::from("config")),
    ]))
    .unwrap();

    let secret = kubernetes::k8s_secret_ref_fn(Kwargs::from_iter(vec![
        ("secret_name", Value::from("secrets")),
        ("key", Value::from("pass")),
    ]))
    .unwrap();

    let configmap = kubernetes::k8s_configmap_ref_fn(Kwargs::from_iter(vec![
        ("configmap_name", Value::from("config")),
        ("key", Value::from("url")),
    ]))
    .unwrap();

    // All should contain valueFrom
    assert!(env_var.to_string().contains("valueFrom:"));
    assert!(secret.to_string().contains("valueFrom:"));
    assert!(configmap.to_string().contains("valueFrom:"));
}
