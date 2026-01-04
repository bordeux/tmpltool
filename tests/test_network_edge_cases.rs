//! Edge case tests for network functions
//!
//! Tests covering error paths and boundary conditions not covered in main tests.

use minijinja::Value;
use minijinja::value::Kwargs;
use tmpltool::functions::Function;
use tmpltool::functions::network::{
    CidrBroadcast, CidrContains, CidrNetmask, CidrNetwork, GetIpAddress, IntToIp, IpToInt,
    ResolveDns,
};

// ==================== parse_cidr error cases ====================

#[test]
fn test_cidr_invalid_prefix_too_large() {
    let result = CidrNetwork::call(Kwargs::from_iter(vec![(
        "cidr",
        Value::from("192.168.1.0/33"),
    )]));
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("0-32") || err.contains("Prefix"),
        "Error should mention prefix range: {}",
        err
    );
}

#[test]
fn test_cidr_invalid_prefix_negative() {
    // This will fail at parsing since -1 is not a valid u8
    let result = CidrNetwork::call(Kwargs::from_iter(vec![(
        "cidr",
        Value::from("192.168.1.0/-1"),
    )]));
    assert!(result.is_err());
}

#[test]
fn test_cidr_missing_prefix() {
    let result = CidrNetwork::call(Kwargs::from_iter(vec![(
        "cidr",
        Value::from("192.168.1.0"),
    )]));
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("IP/prefix"),
        "Error should mention expected format: {}",
        err
    );
}

#[test]
fn test_cidr_invalid_ip_in_cidr() {
    let result = CidrNetwork::call(Kwargs::from_iter(vec![(
        "cidr",
        Value::from("999.999.999.999/24"),
    )]));
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("Invalid IP"),
        "Error should mention invalid IP: {}",
        err
    );
}

#[test]
fn test_cidr_non_numeric_prefix() {
    let result = CidrNetwork::call(Kwargs::from_iter(vec![(
        "cidr",
        Value::from("192.168.1.0/abc"),
    )]));
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("prefix"),
        "Error should mention prefix: {}",
        err
    );
}

#[test]
fn test_cidr_too_many_slashes() {
    let result = CidrNetwork::call(Kwargs::from_iter(vec![(
        "cidr",
        Value::from("192.168.1.0/24/16"),
    )]));
    assert!(result.is_err());
}

// ==================== prefix_to_mask edge cases ====================

#[test]
fn test_cidr_netmask_prefix_1() {
    let result = CidrNetmask::call(Kwargs::from_iter(vec![("cidr", Value::from("0.0.0.0/1"))]));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_str().unwrap(), "128.0.0.0");
}

#[test]
fn test_cidr_netmask_prefix_31() {
    let result = CidrNetmask::call(Kwargs::from_iter(vec![(
        "cidr",
        Value::from("192.168.1.0/31"),
    )]));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_str().unwrap(), "255.255.255.254");
}

#[test]
fn test_cidr_network_slash_0() {
    let result = CidrNetwork::call(Kwargs::from_iter(vec![(
        "cidr",
        Value::from("192.168.1.100/0"),
    )]));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_str().unwrap(), "0.0.0.0");
}

#[test]
fn test_cidr_broadcast_slash_0() {
    let result = CidrBroadcast::call(Kwargs::from_iter(vec![("cidr", Value::from("0.0.0.0/0"))]));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_str().unwrap(), "255.255.255.255");
}

// ==================== cidr_contains edge cases ====================

#[test]
fn test_cidr_contains_exact_network_address() {
    let result = CidrContains::call(Kwargs::from_iter(vec![
        ("cidr", Value::from("192.168.1.0/24")),
        ("ip", Value::from("192.168.1.0")),
    ]));
    assert!(result.is_ok());
    assert!(result.unwrap().is_true());
}

#[test]
fn test_cidr_contains_exact_broadcast_address() {
    let result = CidrContains::call(Kwargs::from_iter(vec![
        ("cidr", Value::from("192.168.1.0/24")),
        ("ip", Value::from("192.168.1.255")),
    ]));
    assert!(result.is_ok());
    assert!(result.unwrap().is_true());
}

#[test]
fn test_cidr_contains_single_host() {
    let result = CidrContains::call(Kwargs::from_iter(vec![
        ("cidr", Value::from("10.0.0.1/32")),
        ("ip", Value::from("10.0.0.1")),
    ]));
    assert!(result.is_ok());
    assert!(result.unwrap().is_true());
}

#[test]
fn test_cidr_contains_single_host_not_match() {
    let result = CidrContains::call(Kwargs::from_iter(vec![
        ("cidr", Value::from("10.0.0.1/32")),
        ("ip", Value::from("10.0.0.2")),
    ]));
    assert!(result.is_ok());
    assert!(!result.unwrap().is_true());
}

#[test]
fn test_cidr_contains_slash_0_contains_everything() {
    let result = CidrContains::call(Kwargs::from_iter(vec![
        ("cidr", Value::from("0.0.0.0/0")),
        ("ip", Value::from("8.8.8.8")),
    ]));
    assert!(result.is_ok());
    assert!(result.unwrap().is_true());
}

#[test]
fn test_cidr_contains_missing_cidr_param() {
    let result = CidrContains::call(Kwargs::from_iter(vec![("ip", Value::from("192.168.1.1"))]));
    assert!(result.is_err());
}

#[test]
fn test_cidr_contains_missing_ip_param() {
    let result = CidrContains::call(Kwargs::from_iter(vec![(
        "cidr",
        Value::from("192.168.1.0/24"),
    )]));
    assert!(result.is_err());
}

// ==================== get_ip_address edge cases ====================

#[test]
fn test_get_ip_address_nonexistent_interface() {
    let result = GetIpAddress::call(Kwargs::from_iter(vec![(
        "interface",
        Value::from("nonexistent_interface_xyz123"),
    )]));
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("not found"),
        "Error should indicate interface not found: {}",
        err
    );
}

#[test]
fn test_get_ip_address_loopback_interface() {
    // Try common loopback interface names
    let loopback_names = ["lo", "lo0"];

    for name in loopback_names {
        let result = GetIpAddress::call(Kwargs::from_iter(vec![("interface", Value::from(name))]));

        if let Ok(ip) = result {
            let ip_str = ip.as_str().unwrap();
            // Loopback should be 127.x.x.x or ::1
            assert!(
                ip_str.starts_with("127.") || ip_str == "::1",
                "Loopback should return localhost IP, got {}",
                ip_str
            );
            return; // Found a working loopback, test passes
        }
    }
    // If no loopback found by these names, that's okay for this test
}

// ==================== resolve_dns edge cases ====================

#[test]
fn test_resolve_dns_localhost() {
    let result = ResolveDns::call(Kwargs::from_iter(vec![(
        "hostname",
        Value::from("localhost"),
    )]));
    assert!(result.is_ok());
    let ip = result.unwrap();
    let ip_str = ip.as_str().unwrap();
    assert!(
        ip_str.starts_with("127.") || ip_str == "::1",
        "localhost should resolve to loopback, got {}",
        ip_str
    );
}

#[test]
fn test_resolve_dns_invalid_hostname() {
    let result = ResolveDns::call(Kwargs::from_iter(vec![(
        "hostname",
        Value::from("this.hostname.definitely.does.not.exist.invalid"),
    )]));
    assert!(result.is_err());
}

#[test]
fn test_resolve_dns_empty_hostname() {
    let result = ResolveDns::call(Kwargs::from_iter(vec![("hostname", Value::from(""))]));
    assert!(result.is_err());
}

// ==================== ip_to_int edge cases ====================

#[test]
fn test_ip_to_int_missing_param() {
    let result = IpToInt::call(Kwargs::from_iter(Vec::<(&str, Value)>::new()));
    assert!(result.is_err());
}

#[test]
fn test_ip_to_int_localhost() {
    let result = IpToInt::call(Kwargs::from_iter(vec![("ip", Value::from("127.0.0.1"))]));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_i64(), Some(2130706433));
}

#[test]
fn test_ip_to_int_class_a_private() {
    let result = IpToInt::call(Kwargs::from_iter(vec![("ip", Value::from("10.0.0.0"))]));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_i64(), Some(167772160));
}

#[test]
fn test_ip_to_int_octet_too_large() {
    let result = IpToInt::call(Kwargs::from_iter(vec![("ip", Value::from("256.1.1.1"))]));
    assert!(result.is_err());
}

#[test]
fn test_ip_to_int_not_enough_octets() {
    let result = IpToInt::call(Kwargs::from_iter(vec![("ip", Value::from("192.168.1"))]));
    assert!(result.is_err());
}

#[test]
fn test_ip_to_int_too_many_octets() {
    let result = IpToInt::call(Kwargs::from_iter(vec![(
        "ip",
        Value::from("192.168.1.1.1"),
    )]));
    assert!(result.is_err());
}

// ==================== int_to_ip edge cases ====================

#[test]
fn test_int_to_ip_missing_param() {
    let result = IntToIp::call(Kwargs::from_iter(Vec::<(&str, Value)>::new()));
    assert!(result.is_err());
}

#[test]
fn test_int_to_ip_localhost() {
    let result = IntToIp::call(Kwargs::from_iter(vec![(
        "int",
        Value::from(2130706433_i64),
    )]));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_str().unwrap(), "127.0.0.1");
}

#[test]
fn test_int_to_ip_class_a_private() {
    let result = IntToIp::call(Kwargs::from_iter(vec![("int", Value::from(167772160_i64))]));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_str().unwrap(), "10.0.0.0");
}

#[test]
fn test_int_to_ip_way_too_large() {
    let result = IntToIp::call(Kwargs::from_iter(vec![(
        "int",
        Value::from(9999999999999_i64),
    )]));
    assert!(result.is_err());
}

// ==================== CIDR functions missing param tests ====================

#[test]
fn test_cidr_network_missing_param() {
    let result = CidrNetwork::call(Kwargs::from_iter(Vec::<(&str, Value)>::new()));
    assert!(result.is_err());
}

#[test]
fn test_cidr_broadcast_missing_param() {
    let result = CidrBroadcast::call(Kwargs::from_iter(Vec::<(&str, Value)>::new()));
    assert!(result.is_err());
}

#[test]
fn test_cidr_netmask_missing_param() {
    let result = CidrNetmask::call(Kwargs::from_iter(Vec::<(&str, Value)>::new()));
    assert!(result.is_err());
}

// ==================== Error paths for invalid CIDR on other functions ====================

#[test]
fn test_cidr_broadcast_invalid_cidr() {
    let result = CidrBroadcast::call(Kwargs::from_iter(vec![("cidr", Value::from("not-a-cidr"))]));
    assert!(result.is_err());
}

#[test]
fn test_cidr_netmask_invalid_cidr() {
    let result = CidrNetmask::call(Kwargs::from_iter(vec![("cidr", Value::from("not-a-cidr"))]));
    assert!(result.is_err());
}
