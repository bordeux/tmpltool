use minijinja::Value;
use minijinja::value::Kwargs;
use std::collections::BTreeMap;
use tmpltool::functions::url;

// ============================================================================
// basic_auth Tests
// ============================================================================

#[test]
fn test_basic_auth_simple() {
    let result = url::basic_auth_fn(Kwargs::from_iter(vec![
        ("username", Value::from("admin")),
        ("password", Value::from("secret")),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "Basic YWRtaW46c2VjcmV0");
}

#[test]
fn test_basic_auth_special_chars() {
    let result = url::basic_auth_fn(Kwargs::from_iter(vec![
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
    let result = url::basic_auth_fn(Kwargs::from_iter(vec![
        ("username", Value::from("admin")),
        ("password", Value::from("")),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "Basic YWRtaW46");
}

#[test]
fn test_basic_auth_empty_username() {
    let result = url::basic_auth_fn(Kwargs::from_iter(vec![
        ("username", Value::from("")),
        ("password", Value::from("secret")),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "Basic OnNlY3JldA==");
}

#[test]
fn test_basic_auth_missing_username() {
    let result = url::basic_auth_fn(Kwargs::from_iter(vec![("password", Value::from("secret"))]));

    assert!(result.is_err());
}

#[test]
fn test_basic_auth_missing_password() {
    let result = url::basic_auth_fn(Kwargs::from_iter(vec![("username", Value::from("admin"))]));

    assert!(result.is_err());
}

// ============================================================================
// parse_url Tests
// ============================================================================

#[test]
fn test_parse_url_simple() {
    let result = url::parse_url_fn(Kwargs::from_iter(vec![(
        "url",
        Value::from("https://example.com/path"),
    )]))
    .unwrap();

    let obj = result.as_object().unwrap();
    assert_eq!(
        obj.get_value(&Value::from("scheme")).unwrap().as_str(),
        Some("https")
    );
    assert_eq!(
        obj.get_value(&Value::from("host")).unwrap().as_str(),
        Some("example.com")
    );
    assert_eq!(
        obj.get_value(&Value::from("path")).unwrap().as_str(),
        Some("/path")
    );
    assert_eq!(
        obj.get_value(&Value::from("port")).unwrap().as_i64(),
        Some(443)
    );
}

#[test]
fn test_parse_url_with_port() {
    let result = url::parse_url_fn(Kwargs::from_iter(vec![(
        "url",
        Value::from("https://example.com:8080/api"),
    )]))
    .unwrap();

    let obj = result.as_object().unwrap();
    assert_eq!(
        obj.get_value(&Value::from("port")).unwrap().as_i64(),
        Some(8080)
    );
}

#[test]
fn test_parse_url_with_query() {
    let result = url::parse_url_fn(Kwargs::from_iter(vec![(
        "url",
        Value::from("https://example.com/search?q=test&limit=10"),
    )]))
    .unwrap();

    let obj = result.as_object().unwrap();
    assert_eq!(
        obj.get_value(&Value::from("query")).unwrap().as_str(),
        Some("q=test&limit=10")
    );
}

#[test]
fn test_parse_url_with_fragment() {
    let result = url::parse_url_fn(Kwargs::from_iter(vec![(
        "url",
        Value::from("https://example.com/page#section"),
    )]))
    .unwrap();

    let obj = result.as_object().unwrap();
    assert_eq!(
        obj.get_value(&Value::from("fragment")).unwrap().as_str(),
        Some("section")
    );
}

#[test]
fn test_parse_url_with_credentials() {
    let result = url::parse_url_fn(Kwargs::from_iter(vec![(
        "url",
        Value::from("https://user:pass@example.com/path"),
    )]))
    .unwrap();

    let obj = result.as_object().unwrap();
    assert_eq!(
        obj.get_value(&Value::from("username")).unwrap().as_str(),
        Some("user")
    );
    assert_eq!(
        obj.get_value(&Value::from("password")).unwrap().as_str(),
        Some("pass")
    );
}

#[test]
fn test_parse_url_complete() {
    let result = url::parse_url_fn(Kwargs::from_iter(vec![(
        "url",
        Value::from("https://user:pass@example.com:8080/path?foo=bar#section"),
    )]))
    .unwrap();

    let obj = result.as_object().unwrap();
    assert_eq!(
        obj.get_value(&Value::from("scheme")).unwrap().as_str(),
        Some("https")
    );
    assert_eq!(
        obj.get_value(&Value::from("host")).unwrap().as_str(),
        Some("example.com")
    );
    assert_eq!(
        obj.get_value(&Value::from("port")).unwrap().as_i64(),
        Some(8080)
    );
    assert_eq!(
        obj.get_value(&Value::from("path")).unwrap().as_str(),
        Some("/path")
    );
    assert_eq!(
        obj.get_value(&Value::from("query")).unwrap().as_str(),
        Some("foo=bar")
    );
    assert_eq!(
        obj.get_value(&Value::from("fragment")).unwrap().as_str(),
        Some("section")
    );
    assert_eq!(
        obj.get_value(&Value::from("username")).unwrap().as_str(),
        Some("user")
    );
    assert_eq!(
        obj.get_value(&Value::from("password")).unwrap().as_str(),
        Some("pass")
    );
}

#[test]
fn test_parse_url_http_default_port() {
    let result = url::parse_url_fn(Kwargs::from_iter(vec![(
        "url",
        Value::from("http://example.com/path"),
    )]))
    .unwrap();

    let obj = result.as_object().unwrap();
    assert_eq!(
        obj.get_value(&Value::from("port")).unwrap().as_i64(),
        Some(80)
    );
}

#[test]
fn test_parse_url_invalid() {
    let result = url::parse_url_fn(Kwargs::from_iter(vec![(
        "url",
        Value::from("not a valid url"),
    )]));

    assert!(result.is_err());
}

#[test]
fn test_parse_url_missing_param() {
    let result = url::parse_url_fn(Kwargs::from_iter(vec![("dummy", Value::from(0))]));

    assert!(result.is_err());
}

// ============================================================================
// build_url Tests
// ============================================================================

#[test]
fn test_build_url_simple() {
    let result = url::build_url_fn(Kwargs::from_iter(vec![
        ("scheme", Value::from("https")),
        ("host", Value::from("example.com")),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "https://example.com/");
}

#[test]
fn test_build_url_with_port() {
    let result = url::build_url_fn(Kwargs::from_iter(vec![
        ("scheme", Value::from("https")),
        ("host", Value::from("example.com")),
        ("port", Value::from(8080)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "https://example.com:8080/");
}

#[test]
fn test_build_url_with_path() {
    let result = url::build_url_fn(Kwargs::from_iter(vec![
        ("scheme", Value::from("https")),
        ("host", Value::from("example.com")),
        ("path", Value::from("/api/v1/users")),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "https://example.com/api/v1/users");
}

#[test]
fn test_build_url_path_without_leading_slash() {
    let result = url::build_url_fn(Kwargs::from_iter(vec![
        ("scheme", Value::from("https")),
        ("host", Value::from("example.com")),
        ("path", Value::from("api/users")),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "https://example.com/api/users");
}

#[test]
fn test_build_url_with_query() {
    let result = url::build_url_fn(Kwargs::from_iter(vec![
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
    let result = url::build_url_fn(Kwargs::from_iter(vec![
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
    let result = url::build_url_fn(Kwargs::from_iter(vec![
        ("scheme", Value::from("https")),
        ("host", Value::from("example.com")),
        ("query", Value::from("")),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "https://example.com/");
}

#[test]
fn test_build_url_http_scheme() {
    let result = url::build_url_fn(Kwargs::from_iter(vec![
        ("scheme", Value::from("http")),
        ("host", Value::from("localhost")),
        ("port", Value::from(3000)),
    ]))
    .unwrap();

    assert_eq!(result.to_string(), "http://localhost:3000/");
}

#[test]
fn test_build_url_missing_scheme() {
    let result = url::build_url_fn(Kwargs::from_iter(vec![(
        "host",
        Value::from("example.com"),
    )]));

    assert!(result.is_err());
}

#[test]
fn test_build_url_missing_host() {
    let result = url::build_url_fn(Kwargs::from_iter(vec![("scheme", Value::from("https"))]));

    assert!(result.is_err());
}

// ============================================================================
// query_string Tests
// ============================================================================

#[test]
fn test_query_string_simple() {
    let mut params = BTreeMap::new();
    params.insert("name".to_string(), Value::from("test"));
    params.insert("value".to_string(), Value::from(42));

    let result = url::query_string_fn(Kwargs::from_iter(vec![(
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

    let result = url::query_string_fn(Kwargs::from_iter(vec![(
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

    let result = url::query_string_fn(Kwargs::from_iter(vec![(
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

    let result = url::query_string_fn(Kwargs::from_iter(vec![(
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

    let result = url::query_string_fn(Kwargs::from_iter(vec![(
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
    let result = url::query_string_fn(Kwargs::from_iter(vec![(
        "params",
        Value::from("not an object"),
    )]));

    assert!(result.is_err());
}

#[test]
fn test_query_string_missing_param() {
    let result = url::query_string_fn(Kwargs::from_iter(vec![("dummy", Value::from(0))]));

    assert!(result.is_err());
}
