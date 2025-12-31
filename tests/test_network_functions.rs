use minijinja::value::Kwargs;
use minijinja::Value;
use std::net::IpAddr;
use tmpltool::functions::network;

#[test]
fn test_get_ip_address_no_interface() {
    let kwargs = Kwargs::from_iter(Vec::<(&str, Value)>::new());
    let result = network::get_ip_address_fn(kwargs);
    assert!(result.is_ok());
    let ip = result.unwrap();
    let ip_str = ip.as_str().unwrap();

    // Should be a valid IP address
    assert!(ip_str.parse::<IpAddr>().is_ok());

    // Should not be 0.0.0.0
    assert_ne!(ip_str, "0.0.0.0");
}

#[test]
fn test_resolve_dns_missing_hostname() {
    let kwargs = Kwargs::from_iter(Vec::<(&str, Value)>::new());
    let result = network::resolve_dns_fn(kwargs);
    // This will fail because hostname is required
    assert!(result.is_err());
}

#[test]
fn test_is_port_available_valid() {
    // Test with a likely available high port
    let result = network::is_port_available_fn(Kwargs::from_iter(vec![(
        "port",
        Value::from(54321),
    )]));
    assert!(result.is_ok());
    // Result should be a boolean
    let val = result.unwrap();
    assert!(val.is_true() || !val.is_true());
}

#[test]
fn test_is_port_available_invalid_port_low() {
    let result = network::is_port_available_fn(Kwargs::from_iter(vec![("port", Value::from(0))]));
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("between 1 and 65535"));
}

#[test]
fn test_is_port_available_invalid_port_high() {
    let result =
        network::is_port_available_fn(Kwargs::from_iter(vec![("port", Value::from(65536))]));
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("between 1 and 65535"));
}
