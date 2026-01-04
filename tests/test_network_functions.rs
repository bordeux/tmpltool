use minijinja::Value;
use minijinja::value::Kwargs;
use std::net::IpAddr;
use tmpltool::functions::Function;
use tmpltool::functions::network::{
    CidrBroadcast, CidrContains, CidrNetmask, CidrNetwork, GetInterfaces, GetIpAddress, IntToIp,
    IpToInt, ResolveDns,
};

// ==================== get_interfaces Tests ====================

#[test]
fn test_get_interfaces_returns_list() {
    let kwargs = Kwargs::from_iter(Vec::<(&str, Value)>::new());
    let result = GetInterfaces::call(kwargs);
    assert!(result.is_ok());

    let interfaces = result.unwrap();
    // Should be iterable (a list)
    assert!(interfaces.try_iter().is_ok());
}

#[test]
fn test_get_interfaces_has_loopback() {
    let kwargs = Kwargs::from_iter(Vec::<(&str, Value)>::new());
    let result = GetInterfaces::call(kwargs).unwrap();

    let mut found_loopback = false;
    for iface in result.try_iter().unwrap() {
        let is_loopback = iface.get_attr("is_loopback").unwrap();
        if is_loopback.is_true() {
            found_loopback = true;
            break;
        }
    }

    assert!(
        found_loopback,
        "Should have at least one loopback interface"
    );
}

#[test]
fn test_get_interfaces_has_required_fields() {
    let kwargs = Kwargs::from_iter(Vec::<(&str, Value)>::new());
    let result = GetInterfaces::call(kwargs).unwrap();

    for iface in result.try_iter().unwrap() {
        // Each interface should have name, ip, and is_loopback fields
        let name = iface.get_attr("name");
        let ip = iface.get_attr("ip");
        let is_loopback = iface.get_attr("is_loopback");

        assert!(name.is_ok(), "Interface should have 'name' field");
        assert!(ip.is_ok(), "Interface should have 'ip' field");
        assert!(
            is_loopback.is_ok(),
            "Interface should have 'is_loopback' field"
        );

        // Name should be a non-empty string
        let name_str = name.unwrap();
        assert!(name_str.as_str().is_some(), "Name should be a string");
        assert!(
            !name_str.as_str().unwrap().is_empty(),
            "Name should not be empty"
        );

        // IP should be a valid IP address
        let ip_str = ip.unwrap();
        assert!(ip_str.as_str().is_some(), "IP should be a string");
        assert!(
            ip_str.as_str().unwrap().parse::<IpAddr>().is_ok(),
            "IP should be a valid IP address"
        );
    }
}

#[test]
fn test_get_interfaces_loopback_has_localhost_ip() {
    let kwargs = Kwargs::from_iter(Vec::<(&str, Value)>::new());
    let result = GetInterfaces::call(kwargs).unwrap();

    for iface in result.try_iter().unwrap() {
        let is_loopback = iface.get_attr("is_loopback").unwrap();
        if is_loopback.is_true() {
            let ip = iface.get_attr("ip").unwrap();
            let ip_str = ip.as_str().unwrap();
            // Loopback should be 127.x.x.x or ::1
            assert!(
                ip_str.starts_with("127.") || ip_str == "::1",
                "Loopback interface should have localhost IP, got: {}",
                ip_str
            );
        }
    }
}

#[test]
fn test_get_ip_address_no_interface() {
    let kwargs = Kwargs::from_iter(Vec::<(&str, Value)>::new());
    let result = GetIpAddress::call(kwargs);
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
    let result = ResolveDns::call(kwargs);
    // This will fail because hostname is required
    assert!(result.is_err());
}

// Note: is_port_available tests have been moved to tests/test_is_network.rs
// as part of the is-functions refactoring.

// ==================== cidr_contains Tests ====================

#[test]
fn test_cidr_contains_in_range() {
    let result = CidrContains::call(Kwargs::from_iter(vec![
        ("cidr", Value::from("192.168.1.0/24")),
        ("ip", Value::from("192.168.1.100")),
    ]));
    assert!(result.is_ok());
    assert!(result.unwrap().is_true());
}

#[test]
fn test_cidr_contains_out_of_range() {
    let result = CidrContains::call(Kwargs::from_iter(vec![
        ("cidr", Value::from("192.168.1.0/24")),
        ("ip", Value::from("192.168.2.1")),
    ]));
    assert!(result.is_ok());
    assert!(!result.unwrap().is_true());
}

#[test]
fn test_cidr_contains_class_a() {
    let result = CidrContains::call(Kwargs::from_iter(vec![
        ("cidr", Value::from("10.0.0.0/8")),
        ("ip", Value::from("10.255.255.255")),
    ]));
    assert!(result.is_ok());
    assert!(result.unwrap().is_true());
}

#[test]
fn test_cidr_contains_invalid_cidr() {
    let result = CidrContains::call(Kwargs::from_iter(vec![
        ("cidr", Value::from("invalid")),
        ("ip", Value::from("192.168.1.1")),
    ]));
    assert!(result.is_err());
}

#[test]
fn test_cidr_contains_invalid_ip() {
    let result = CidrContains::call(Kwargs::from_iter(vec![
        ("cidr", Value::from("192.168.1.0/24")),
        ("ip", Value::from("invalid")),
    ]));
    assert!(result.is_err());
}

// ==================== cidr_network Tests ====================

#[test]
fn test_cidr_network_class_c() {
    let result = CidrNetwork::call(Kwargs::from_iter(vec![(
        "cidr",
        Value::from("192.168.1.100/24"),
    )]));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_str().unwrap(), "192.168.1.0");
}

#[test]
fn test_cidr_network_class_b() {
    let result = CidrNetwork::call(Kwargs::from_iter(vec![(
        "cidr",
        Value::from("172.16.50.100/16"),
    )]));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_str().unwrap(), "172.16.0.0");
}

#[test]
fn test_cidr_network_class_a() {
    let result = CidrNetwork::call(Kwargs::from_iter(vec![(
        "cidr",
        Value::from("10.20.30.40/8"),
    )]));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_str().unwrap(), "10.0.0.0");
}

// ==================== cidr_broadcast Tests ====================

#[test]
fn test_cidr_broadcast_class_c() {
    let result = CidrBroadcast::call(Kwargs::from_iter(vec![(
        "cidr",
        Value::from("192.168.1.0/24"),
    )]));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_str().unwrap(), "192.168.1.255");
}

#[test]
fn test_cidr_broadcast_class_a() {
    let result = CidrBroadcast::call(Kwargs::from_iter(vec![("cidr", Value::from("10.0.0.0/8"))]));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_str().unwrap(), "10.255.255.255");
}

#[test]
fn test_cidr_broadcast_slash_32() {
    let result = CidrBroadcast::call(Kwargs::from_iter(vec![(
        "cidr",
        Value::from("192.168.1.1/32"),
    )]));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_str().unwrap(), "192.168.1.1");
}

// ==================== cidr_netmask Tests ====================

#[test]
fn test_cidr_netmask_24() {
    let result = CidrNetmask::call(Kwargs::from_iter(vec![(
        "cidr",
        Value::from("192.168.1.0/24"),
    )]));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_str().unwrap(), "255.255.255.0");
}

#[test]
fn test_cidr_netmask_16() {
    let result = CidrNetmask::call(Kwargs::from_iter(vec![(
        "cidr",
        Value::from("172.16.0.0/16"),
    )]));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_str().unwrap(), "255.255.0.0");
}

#[test]
fn test_cidr_netmask_8() {
    let result = CidrNetmask::call(Kwargs::from_iter(vec![("cidr", Value::from("10.0.0.0/8"))]));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_str().unwrap(), "255.0.0.0");
}

#[test]
fn test_cidr_netmask_12() {
    let result = CidrNetmask::call(Kwargs::from_iter(vec![(
        "cidr",
        Value::from("172.16.0.0/12"),
    )]));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_str().unwrap(), "255.240.0.0");
}

#[test]
fn test_cidr_netmask_32() {
    let result = CidrNetmask::call(Kwargs::from_iter(vec![(
        "cidr",
        Value::from("192.168.1.1/32"),
    )]));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_str().unwrap(), "255.255.255.255");
}

#[test]
fn test_cidr_netmask_0() {
    let result = CidrNetmask::call(Kwargs::from_iter(vec![("cidr", Value::from("0.0.0.0/0"))]));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_str().unwrap(), "0.0.0.0");
}

// ==================== ip_to_int Tests ====================

#[test]
fn test_ip_to_int_basic() {
    let result = IpToInt::call(Kwargs::from_iter(vec![("ip", Value::from("192.168.1.1"))]));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_i64(), Some(3232235777));
}

#[test]
fn test_ip_to_int_zero() {
    let result = IpToInt::call(Kwargs::from_iter(vec![("ip", Value::from("0.0.0.0"))]));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_i64(), Some(0));
}

#[test]
fn test_ip_to_int_max() {
    let result = IpToInt::call(Kwargs::from_iter(vec![(
        "ip",
        Value::from("255.255.255.255"),
    )]));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_i64(), Some(4294967295));
}

#[test]
fn test_ip_to_int_invalid() {
    let result = IpToInt::call(Kwargs::from_iter(vec![("ip", Value::from("invalid"))]));
    assert!(result.is_err());
}

// ==================== int_to_ip Tests ====================

#[test]
fn test_int_to_ip_basic() {
    let result = IntToIp::call(Kwargs::from_iter(vec![(
        "int",
        Value::from(3232235777_i64),
    )]));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_str().unwrap(), "192.168.1.1");
}

#[test]
fn test_int_to_ip_zero() {
    let result = IntToIp::call(Kwargs::from_iter(vec![("int", Value::from(0))]));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_str().unwrap(), "0.0.0.0");
}

#[test]
fn test_int_to_ip_max() {
    let result = IntToIp::call(Kwargs::from_iter(vec![(
        "int",
        Value::from(4294967295_i64),
    )]));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_str().unwrap(), "255.255.255.255");
}

#[test]
fn test_int_to_ip_negative() {
    let result = IntToIp::call(Kwargs::from_iter(vec![("int", Value::from(-1))]));
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("between 0 and"));
}

#[test]
fn test_int_to_ip_too_large() {
    let result = IntToIp::call(Kwargs::from_iter(vec![(
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
    let int_result = IpToInt::call(Kwargs::from_iter(vec![("ip", Value::from(ip))])).unwrap();
    let int_value = int_result.as_i64().unwrap();

    // Int back to IP
    let ip_result =
        IntToIp::call(Kwargs::from_iter(vec![("int", Value::from(int_value))])).unwrap();

    assert_eq!(ip_result.as_str().unwrap(), ip);
}
