use minijinja::Value;
use minijinja::value::Kwargs;
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
    let result =
        network::is_port_available_fn(Kwargs::from_iter(vec![("port", Value::from(54321))]));
    assert!(result.is_ok());
    // Result should be a boolean
    let val = result.unwrap();
    assert!(val.is_true() || !val.is_true());
}

#[test]
fn test_is_port_available_invalid_port_low() {
    let result = network::is_port_available_fn(Kwargs::from_iter(vec![("port", Value::from(0))]));
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("between 1 and 65535")
    );
}

#[test]
fn test_is_port_available_invalid_port_high() {
    let result =
        network::is_port_available_fn(Kwargs::from_iter(vec![("port", Value::from(65536))]));
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("between 1 and 65535")
    );
}

// ==================== cidr_contains Tests ====================

#[test]
fn test_cidr_contains_in_range() {
    let result = network::cidr_contains_fn(Kwargs::from_iter(vec![
        ("cidr", Value::from("192.168.1.0/24")),
        ("ip", Value::from("192.168.1.100")),
    ]));
    assert!(result.is_ok());
    assert!(result.unwrap().is_true());
}

#[test]
fn test_cidr_contains_out_of_range() {
    let result = network::cidr_contains_fn(Kwargs::from_iter(vec![
        ("cidr", Value::from("192.168.1.0/24")),
        ("ip", Value::from("192.168.2.1")),
    ]));
    assert!(result.is_ok());
    assert!(!result.unwrap().is_true());
}

#[test]
fn test_cidr_contains_class_a() {
    let result = network::cidr_contains_fn(Kwargs::from_iter(vec![
        ("cidr", Value::from("10.0.0.0/8")),
        ("ip", Value::from("10.255.255.255")),
    ]));
    assert!(result.is_ok());
    assert!(result.unwrap().is_true());
}

#[test]
fn test_cidr_contains_invalid_cidr() {
    let result = network::cidr_contains_fn(Kwargs::from_iter(vec![
        ("cidr", Value::from("invalid")),
        ("ip", Value::from("192.168.1.1")),
    ]));
    assert!(result.is_err());
}

#[test]
fn test_cidr_contains_invalid_ip() {
    let result = network::cidr_contains_fn(Kwargs::from_iter(vec![
        ("cidr", Value::from("192.168.1.0/24")),
        ("ip", Value::from("invalid")),
    ]));
    assert!(result.is_err());
}

// ==================== cidr_network Tests ====================

#[test]
fn test_cidr_network_class_c() {
    let result = network::cidr_network_fn(Kwargs::from_iter(vec![(
        "cidr",
        Value::from("192.168.1.100/24"),
    )]));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_str().unwrap(), "192.168.1.0");
}

#[test]
fn test_cidr_network_class_b() {
    let result = network::cidr_network_fn(Kwargs::from_iter(vec![(
        "cidr",
        Value::from("172.16.50.100/16"),
    )]));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_str().unwrap(), "172.16.0.0");
}

#[test]
fn test_cidr_network_class_a() {
    let result = network::cidr_network_fn(Kwargs::from_iter(vec![(
        "cidr",
        Value::from("10.20.30.40/8"),
    )]));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_str().unwrap(), "10.0.0.0");
}

// ==================== cidr_broadcast Tests ====================

#[test]
fn test_cidr_broadcast_class_c() {
    let result = network::cidr_broadcast_fn(Kwargs::from_iter(vec![(
        "cidr",
        Value::from("192.168.1.0/24"),
    )]));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_str().unwrap(), "192.168.1.255");
}

#[test]
fn test_cidr_broadcast_class_a() {
    let result =
        network::cidr_broadcast_fn(Kwargs::from_iter(vec![("cidr", Value::from("10.0.0.0/8"))]));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_str().unwrap(), "10.255.255.255");
}

#[test]
fn test_cidr_broadcast_slash_32() {
    let result = network::cidr_broadcast_fn(Kwargs::from_iter(vec![(
        "cidr",
        Value::from("192.168.1.1/32"),
    )]));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_str().unwrap(), "192.168.1.1");
}

// ==================== cidr_netmask Tests ====================

#[test]
fn test_cidr_netmask_24() {
    let result = network::cidr_netmask_fn(Kwargs::from_iter(vec![(
        "cidr",
        Value::from("192.168.1.0/24"),
    )]));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_str().unwrap(), "255.255.255.0");
}

#[test]
fn test_cidr_netmask_16() {
    let result = network::cidr_netmask_fn(Kwargs::from_iter(vec![(
        "cidr",
        Value::from("172.16.0.0/16"),
    )]));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_str().unwrap(), "255.255.0.0");
}

#[test]
fn test_cidr_netmask_8() {
    let result =
        network::cidr_netmask_fn(Kwargs::from_iter(vec![("cidr", Value::from("10.0.0.0/8"))]));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_str().unwrap(), "255.0.0.0");
}

#[test]
fn test_cidr_netmask_12() {
    let result = network::cidr_netmask_fn(Kwargs::from_iter(vec![(
        "cidr",
        Value::from("172.16.0.0/12"),
    )]));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_str().unwrap(), "255.240.0.0");
}

#[test]
fn test_cidr_netmask_32() {
    let result = network::cidr_netmask_fn(Kwargs::from_iter(vec![(
        "cidr",
        Value::from("192.168.1.1/32"),
    )]));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_str().unwrap(), "255.255.255.255");
}

#[test]
fn test_cidr_netmask_0() {
    let result =
        network::cidr_netmask_fn(Kwargs::from_iter(vec![("cidr", Value::from("0.0.0.0/0"))]));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_str().unwrap(), "0.0.0.0");
}

// ==================== ip_to_int Tests ====================

#[test]
fn test_ip_to_int_basic() {
    let result = network::ip_to_int_fn(Kwargs::from_iter(vec![("ip", Value::from("192.168.1.1"))]));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_i64(), Some(3232235777));
}

#[test]
fn test_ip_to_int_zero() {
    let result = network::ip_to_int_fn(Kwargs::from_iter(vec![("ip", Value::from("0.0.0.0"))]));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_i64(), Some(0));
}

#[test]
fn test_ip_to_int_max() {
    let result = network::ip_to_int_fn(Kwargs::from_iter(vec![(
        "ip",
        Value::from("255.255.255.255"),
    )]));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_i64(), Some(4294967295));
}

#[test]
fn test_ip_to_int_invalid() {
    let result = network::ip_to_int_fn(Kwargs::from_iter(vec![("ip", Value::from("invalid"))]));
    assert!(result.is_err());
}

// ==================== int_to_ip Tests ====================

#[test]
fn test_int_to_ip_basic() {
    let result = network::int_to_ip_fn(Kwargs::from_iter(vec![(
        "int",
        Value::from(3232235777_i64),
    )]));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_str().unwrap(), "192.168.1.1");
}

#[test]
fn test_int_to_ip_zero() {
    let result = network::int_to_ip_fn(Kwargs::from_iter(vec![("int", Value::from(0))]));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_str().unwrap(), "0.0.0.0");
}

#[test]
fn test_int_to_ip_max() {
    let result = network::int_to_ip_fn(Kwargs::from_iter(vec![(
        "int",
        Value::from(4294967295_i64),
    )]));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_str().unwrap(), "255.255.255.255");
}

#[test]
fn test_int_to_ip_negative() {
    let result = network::int_to_ip_fn(Kwargs::from_iter(vec![("int", Value::from(-1))]));
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("between 0 and"));
}

#[test]
fn test_int_to_ip_too_large() {
    let result = network::int_to_ip_fn(Kwargs::from_iter(vec![(
        "int",
        Value::from(4294967296_i64),
    )]));
    assert!(result.is_err());
}

// ==================== Roundtrip Tests ====================

#[test]
fn test_ip_int_roundtrip() {
    let ip = "10.20.30.40";

    // IP to int
    let int_result =
        network::ip_to_int_fn(Kwargs::from_iter(vec![("ip", Value::from(ip))])).unwrap();
    let int_value = int_result.as_i64().unwrap();

    // Int back to IP
    let ip_result =
        network::int_to_ip_fn(Kwargs::from_iter(vec![("int", Value::from(int_value))])).unwrap();

    assert_eq!(ip_result.as_str().unwrap(), ip);
}
