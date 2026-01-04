use minijinja::Value;
use minijinja::value::Kwargs;
use std::collections::BTreeMap;
use tmpltool::functions::Function;
use tmpltool::functions::url::{BasicAuth, BuildUrl, QueryString};

// ============================================================================
// basic_auth Tests
// ============================================================================

#[test]
fn test_basic_auth_simple() {
    let result = BasicAuth::call(Kwargs::from_iter(vec![
        ("username", Value::from("admin")),
        ("password", Value::from("secret")),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "Basic YWRtaW46c2VjcmV0");
}

#[test]
fn test_basic_auth_special_chars() {
    let result = BasicAuth::call(Kwargs::from_iter(vec![
        ("username", Value::from("user@example.com")),
        ("password", Value::from("p@ss:w0rd!")),
    ]))
    .unwrap();

    // Decode to verify
    let output = result.to_string();
    assert!(output.starts_with("Basic "));
}

#[test]
fn test_basic_auth_empty_password() {
    let result = BasicAuth::call(Kwargs::from_iter(vec![
        ("username", Value::from("admin")),
        ("password", Value::from("")),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "Basic YWRtaW46");
}

#[test]
fn test_basic_auth_empty_username() {
    let result = BasicAuth::call(Kwargs::from_iter(vec![
        ("username", Value::from("")),
        ("password", Value::from("secret")),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "Basic OnNlY3JldA==");
}

#[test]
fn test_basic_auth_missing_username() {
    let result = BasicAuth::call(Kwargs::from_iter(vec![("password", Value::from("secret"))]));

    assert!(result.is_err());
}

#[test]
fn test_basic_auth_missing_password() {
    let result = BasicAuth::call(Kwargs::from_iter(vec![("username", Value::from("admin"))]));

    assert!(result.is_err());
}

// Note: parse_url_fn tests removed - function now in filter_functions/url.rs
// with dual function+filter syntax support. See tests/test_filters_integration.rs.

// ============================================================================
// build_url Tests
// ============================================================================

#[test]
fn test_build_url_simple() {
    let result = BuildUrl::call(Kwargs::from_iter(vec![
        ("scheme", Value::from("https")),
        ("host", Value::from("example.com")),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "https://example.com/");
}

#[test]
fn test_build_url_with_port() {
    let result = BuildUrl::call(Kwargs::from_iter(vec![
        ("scheme", Value::from("https")),
        ("host", Value::from("example.com")),
        ("port", Value::from(8080)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "https://example.com:8080/");
}

#[test]
fn test_build_url_with_path() {
    let result = BuildUrl::call(Kwargs::from_iter(vec![
        ("scheme", Value::from("https")),
        ("host", Value::from("example.com")),
        ("path", Value::from("/api/v1/users")),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "https://example.com/api/v1/users");
}

#[test]
fn test_build_url_path_without_leading_slash() {
    let result = BuildUrl::call(Kwargs::from_iter(vec![
        ("scheme", Value::from("https")),
        ("host", Value::from("example.com")),
        ("path", Value::from("api/users")),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "https://example.com/api/users");
}

#[test]
fn test_build_url_with_query() {
    let result = BuildUrl::call(Kwargs::from_iter(vec![
        ("scheme", Value::from("https")),
        ("host", Value::from("example.com")),
        ("path", Value::from("/search")),
        ("query", Value::from("q=test&limit=10")),
    ]))
    .unwrap();

    assert_eq!(
        result.to_string(),
        "https://example.com/search?q=test&limit=10"
    );
}

#[test]
fn test_build_url_complete() {
    let result = BuildUrl::call(Kwargs::from_iter(vec![
        ("scheme", Value::from("https")),
        ("host", Value::from("api.example.com")),
        ("port", Value::from(8080)),
        ("path", Value::from("/v1/users")),
        ("query", Value::from("active=true&limit=50")),
    ]))
    .unwrap();

    assert_eq!(
        result.to_string(),
        "https://api.example.com:8080/v1/users?active=true&limit=50"
    );
}

#[test]
fn test_build_url_empty_query() {
    let result = BuildUrl::call(Kwargs::from_iter(vec![
        ("scheme", Value::from("https")),
        ("host", Value::from("example.com")),
        ("query", Value::from("")),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "https://example.com/");
}

#[test]
fn test_build_url_http_scheme() {
    let result = BuildUrl::call(Kwargs::from_iter(vec![
        ("scheme", Value::from("http")),
        ("host", Value::from("localhost")),
        ("port", Value::from(3000)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "http://localhost:3000/");
}

#[test]
fn test_build_url_default_scheme() {
    let result = BuildUrl::call(Kwargs::from_iter(vec![(
        "host",
        Value::from("example.com"),
    )]))
    .unwrap();

    assert_eq!(result.to_string(), "https://example.com/");
}

#[test]
fn test_build_url_missing_host() {
    let result = BuildUrl::call(Kwargs::from_iter(vec![("scheme", Value::from("https"))]));

    assert!(result.is_err());
}

#[test]
fn test_build_url_with_query_object() {
    let mut params = BTreeMap::new();
    params.insert("page".to_string(), Value::from(1));
    params.insert("limit".to_string(), Value::from(20));

    let result = BuildUrl::call(Kwargs::from_iter(vec![
        ("host", Value::from("api.example.com")),
        ("path", Value::from("/users")),
        ("query", Value::from_object(params)),
    ]))
    .unwrap();

    let output = result.to_string();
    assert!(output.starts_with("https://api.example.com/users?"));
    assert!(output.contains("page=1"));
    assert!(output.contains("limit=20"));
}

#[test]
fn test_build_url_with_query_object_complex() {
    let mut params = BTreeMap::new();
    params.insert("search".to_string(), Value::from("hello world"));
    params.insert("active".to_string(), Value::from(true));
    params.insert("count".to_string(), Value::from(42));

    let result = BuildUrl::call(Kwargs::from_iter(vec![
        ("host", Value::from("example.com")),
        ("query", Value::from_object(params)),
    ]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("active=true"));
    assert!(output.contains("count=42"));
    assert!(output.contains("search=hello") || output.contains("search=hello%20world"));
}

// ============================================================================
// query_string Tests
// ============================================================================

#[test]
fn test_query_string_simple() {
    let mut params = BTreeMap::new();
    params.insert("name".to_string(), Value::from("test"));
    params.insert("value".to_string(), Value::from(42));

    let result = QueryString::call(Kwargs::from_iter(vec![(
        "params",
        Value::from_object(params),
    )]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("name=test"));
    assert!(output.contains("value=42"));
}

#[test]
fn test_query_string_with_special_chars() {
    let mut params = BTreeMap::new();
    params.insert("query".to_string(), Value::from("hello world"));
    params.insert("email".to_string(), Value::from("user@example.com"));

    let result = QueryString::call(Kwargs::from_iter(vec![(
        "params",
        Value::from_object(params),
    )]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("query=hello+world") || output.contains("query=hello%20world"));
    assert!(output.contains("email=user%40example.com"));
}

#[test]
fn test_query_string_boolean_values() {
    let mut params = BTreeMap::new();
    params.insert("active".to_string(), Value::from(true));
    params.insert("verified".to_string(), Value::from(false));

    let result = QueryString::call(Kwargs::from_iter(vec![(
        "params",
        Value::from_object(params),
    )]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("active=true"));
    assert!(output.contains("verified=false"));
}

#[test]
fn test_query_string_empty_object() {
    let params: BTreeMap<String, Value> = BTreeMap::new();

    let result = QueryString::call(Kwargs::from_iter(vec![(
        "params",
        Value::from_object(params),
    )]))
    .unwrap();

    assert_eq!(result.to_string(), "");
}

#[test]
fn test_query_string_multiple_params() {
    let mut params = BTreeMap::new();
    params.insert("page".to_string(), Value::from(1));
    params.insert("limit".to_string(), Value::from(20));
    params.insert("sort".to_string(), Value::from("name"));
    params.insert("order".to_string(), Value::from("asc"));

    let result = QueryString::call(Kwargs::from_iter(vec![(
        "params",
        Value::from_object(params),
    )]))
    .unwrap();

    let output = result.to_string();
    assert!(output.contains("page=1"));
    assert!(output.contains("limit=20"));
    assert!(output.contains("sort=name"));
    assert!(output.contains("order=asc"));
}

#[test]
fn test_query_string_error_not_object() {
    let result = QueryString::call(Kwargs::from_iter(vec![(
        "params",
        Value::from("not an object"),
    )]));

    assert!(result.is_err());
}

#[test]
fn test_query_string_missing_param() {
    let result = QueryString::call(Kwargs::from_iter(vec![("dummy", Value::from(0))]));

    assert!(result.is_err());
}
