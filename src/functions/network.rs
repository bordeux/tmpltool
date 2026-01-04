//! Network-related functions for MiniJinja templates
//!
//! This module provides functions for network operations:
//! - `get_ip_address`: Get IP address of interface or primary local IP
//! - `get_interfaces`: List all network interfaces
//! - `resolve_dns`: DNS hostname resolution
//! - `cidr_contains`: Check if IP is in CIDR range
//! - `cidr_network`: Get network address from CIDR
//! - `cidr_broadcast`: Get broadcast address from CIDR
//! - `cidr_netmask`: Get netmask from CIDR
//! - `ip_to_int`: Convert IP to integer
//! - `int_to_ip`: Convert integer to IP

use super::metadata::{ArgumentMetadata, FunctionMetadata, SyntaxVariants};
use super::traits::Function;
use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};
use std::net::{Ipv4Addr, ToSocketAddrs};

/// Get IP address of a network interface or the primary local IP
pub struct GetIpAddress;

impl Function for GetIpAddress {
    const NAME: &'static str = "get_ip_address";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "get_ip_address",
        category: "network",
        description: "Get IP address of a network interface or the primary local IP",
        arguments: &[ArgumentMetadata {
            name: "interface",
            arg_type: "string",
            required: false,
            default: None,
            description: "Network interface name (e.g., eth0, en0). If not provided, returns primary local IP",
        }],
        return_type: "string",
        examples: &[
            "{{ get_ip_address() }}",
            "{{ get_ip_address(interface=\"eth0\") }}",
        ],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let interface: Option<String> = kwargs.get("interface").ok();

        if let Some(iface) = interface {
            get_interface_ip(&iface)
        } else {
            get_local_ip()
        }
    }
}

/// Get the primary local IP address
fn get_local_ip() -> Result<Value, Error> {
    let socket = std::net::UdpSocket::bind("0.0.0.0:0").map_err(|e| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("Failed to create socket: {}", e),
        )
    })?;

    socket.connect("8.8.8.8:80").map_err(|e| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("Failed to determine local IP: {}", e),
        )
    })?;

    let local_addr = socket.local_addr().map_err(|e| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("Failed to get local address: {}", e),
        )
    })?;

    Ok(Value::from(local_addr.ip().to_string()))
}

/// Get IP address for a specific network interface
fn get_interface_ip(interface: &str) -> Result<Value, Error> {
    let ifaces = if_addrs::get_if_addrs().map_err(|e| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("Failed to get network interfaces: {}", e),
        )
    })?;

    for iface in ifaces {
        if iface.name == interface {
            return Ok(Value::from(iface.ip().to_string()));
        }
    }

    Err(Error::new(
        ErrorKind::InvalidOperation,
        format!("Network interface '{}' not found", interface),
    ))
}

/// Get a list of all network interfaces with their IP addresses
pub struct GetInterfaces;

impl Function for GetInterfaces {
    const NAME: &'static str = "get_interfaces";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "get_interfaces",
        category: "network",
        description: "Get list of all network interfaces with their IP addresses",
        arguments: &[],
        return_type: "array",
        examples: &[
            "{% for iface in get_interfaces() %}{{ iface.name }}: {{ iface.ip }}{% endfor %}",
        ],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(_kwargs: Kwargs) -> Result<Value, Error> {
        let ifaces = if_addrs::get_if_addrs().map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Failed to get network interfaces: {}", e),
            )
        })?;

        let interfaces: Vec<Value> = ifaces
            .into_iter()
            .map(|iface| {
                let mut map = std::collections::BTreeMap::new();
                map.insert("name".to_string(), Value::from(iface.name.clone()));
                map.insert("ip".to_string(), Value::from(iface.ip().to_string()));
                map.insert("is_loopback".to_string(), Value::from(iface.is_loopback()));
                Value::from_object(map)
            })
            .collect();

        Ok(Value::from(interfaces))
    }
}

/// Resolve a hostname to an IP address using DNS
pub struct ResolveDns;

impl Function for ResolveDns {
    const NAME: &'static str = "resolve_dns";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "resolve_dns",
        category: "network",
        description: "Resolve a hostname to an IP address using DNS",
        arguments: &[ArgumentMetadata {
            name: "hostname",
            arg_type: "string",
            required: true,
            default: None,
            description: "Hostname to resolve (e.g., google.com, localhost)",
        }],
        return_type: "string",
        examples: &[
            "{{ resolve_dns(hostname=\"google.com\") }}",
            "{{ resolve_dns(hostname=\"localhost\") }}",
        ],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let hostname: String = kwargs.get("hostname")?;

        let address = format!("{}:0", hostname);

        let addrs: Vec<_> = address
            .to_socket_addrs()
            .map_err(|e| {
                Error::new(
                    ErrorKind::InvalidOperation,
                    format!("Failed to resolve hostname '{}': {}", hostname, e),
                )
            })?
            .collect();

        if addrs.is_empty() {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                format!("No IP addresses found for hostname '{}'", hostname),
            ));
        }

        Ok(Value::from(addrs[0].ip().to_string()))
    }
}

/// Parse a CIDR string into network address and prefix length
fn parse_cidr(cidr: &str) -> Result<(Ipv4Addr, u8), Error> {
    let parts: Vec<&str> = cidr.split('/').collect();
    if parts.len() != 2 {
        return Err(Error::new(
            ErrorKind::InvalidOperation,
            format!(
                "Invalid CIDR notation '{}': expected format 'IP/prefix'",
                cidr
            ),
        ));
    }

    let ip: Ipv4Addr = parts[0].parse().map_err(|_| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("Invalid IP address in CIDR '{}': '{}'", cidr, parts[0]),
        )
    })?;

    let prefix: u8 = parts[1].parse().map_err(|_| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("Invalid prefix length in CIDR '{}': '{}'", cidr, parts[1]),
        )
    })?;

    if prefix > 32 {
        return Err(Error::new(
            ErrorKind::InvalidOperation,
            format!("Prefix length must be 0-32, got {}", prefix),
        ));
    }

    Ok((ip, prefix))
}

/// Calculate the network mask from prefix length
fn prefix_to_mask(prefix: u8) -> u32 {
    if prefix == 0 {
        0
    } else {
        !0u32 << (32 - prefix)
    }
}

/// Check if an IP address is within a CIDR range
pub struct CidrContains;

impl Function for CidrContains {
    const NAME: &'static str = "cidr_contains";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "cidr_contains",
        category: "network",
        description: "Check if an IP address is within a CIDR range",
        arguments: &[
            ArgumentMetadata {
                name: "cidr",
                arg_type: "string",
                required: true,
                default: None,
                description: "CIDR notation (e.g., 192.168.1.0/24)",
            },
            ArgumentMetadata {
                name: "ip",
                arg_type: "string",
                required: true,
                default: None,
                description: "IP address to check",
            },
        ],
        return_type: "boolean",
        examples: &[
            "{% if cidr_contains(cidr=\"192.168.1.0/24\", ip=\"192.168.1.100\") %}in subnet{% endif %}",
        ],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let cidr: String = kwargs.get("cidr")?;
        let ip_str: String = kwargs.get("ip")?;

        let (network_ip, prefix) = parse_cidr(&cidr)?;
        let check_ip: Ipv4Addr = ip_str.parse().map_err(|_| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Invalid IP address: '{}'", ip_str),
            )
        })?;

        let mask = prefix_to_mask(prefix);
        let network_int = u32::from(network_ip);
        let check_int = u32::from(check_ip);

        let is_contained = (network_int & mask) == (check_int & mask);

        Ok(Value::from(is_contained))
    }
}

/// Get the network address from a CIDR notation
pub struct CidrNetwork;

impl Function for CidrNetwork {
    const NAME: &'static str = "cidr_network";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "cidr_network",
        category: "network",
        description: "Get the network address from a CIDR notation",
        arguments: &[ArgumentMetadata {
            name: "cidr",
            arg_type: "string",
            required: true,
            default: None,
            description: "CIDR notation (e.g., 192.168.1.100/24)",
        }],
        return_type: "string",
        examples: &["{{ cidr_network(cidr=\"192.168.1.100/24\") }}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let cidr: String = kwargs.get("cidr")?;

        let (ip, prefix) = parse_cidr(&cidr)?;
        let mask = prefix_to_mask(prefix);
        let network_int = u32::from(ip) & mask;
        let network_ip = Ipv4Addr::from(network_int);

        Ok(Value::from(network_ip.to_string()))
    }
}

/// Get the broadcast address from a CIDR notation
pub struct CidrBroadcast;

impl Function for CidrBroadcast {
    const NAME: &'static str = "cidr_broadcast";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "cidr_broadcast",
        category: "network",
        description: "Get the broadcast address from a CIDR notation",
        arguments: &[ArgumentMetadata {
            name: "cidr",
            arg_type: "string",
            required: true,
            default: None,
            description: "CIDR notation (e.g., 192.168.1.0/24)",
        }],
        return_type: "string",
        examples: &["{{ cidr_broadcast(cidr=\"192.168.1.0/24\") }}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let cidr: String = kwargs.get("cidr")?;

        let (ip, prefix) = parse_cidr(&cidr)?;
        let mask = prefix_to_mask(prefix);
        let network_int = u32::from(ip) & mask;
        let broadcast_int = network_int | !mask;
        let broadcast_ip = Ipv4Addr::from(broadcast_int);

        Ok(Value::from(broadcast_ip.to_string()))
    }
}

/// Get the netmask from a CIDR notation
pub struct CidrNetmask;

impl Function for CidrNetmask {
    const NAME: &'static str = "cidr_netmask";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "cidr_netmask",
        category: "network",
        description: "Get the netmask from a CIDR notation",
        arguments: &[ArgumentMetadata {
            name: "cidr",
            arg_type: "string",
            required: true,
            default: None,
            description: "CIDR notation (e.g., 192.168.1.0/24)",
        }],
        return_type: "string",
        examples: &["{{ cidr_netmask(cidr=\"192.168.1.0/24\") }}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let cidr: String = kwargs.get("cidr")?;

        let (_, prefix) = parse_cidr(&cidr)?;
        let mask = prefix_to_mask(prefix);
        let mask_ip = Ipv4Addr::from(mask);

        Ok(Value::from(mask_ip.to_string()))
    }
}

/// Convert an IPv4 address to its integer representation
pub struct IpToInt;

impl Function for IpToInt {
    const NAME: &'static str = "ip_to_int";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "ip_to_int",
        category: "network",
        description: "Convert an IPv4 address to its integer representation",
        arguments: &[ArgumentMetadata {
            name: "ip",
            arg_type: "string",
            required: true,
            default: None,
            description: "IPv4 address (e.g., 192.168.1.1)",
        }],
        return_type: "integer",
        examples: &["{{ ip_to_int(ip=\"192.168.1.1\") }}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let ip_str: String = kwargs.get("ip")?;

        let ip: Ipv4Addr = ip_str.parse().map_err(|_| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Invalid IPv4 address: '{}'", ip_str),
            )
        })?;

        let int_value = u32::from(ip);

        Ok(Value::from(int_value as i64))
    }
}

/// Convert an integer to its IPv4 address representation
pub struct IntToIp;

impl Function for IntToIp {
    const NAME: &'static str = "int_to_ip";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "int_to_ip",
        category: "network",
        description: "Convert an integer to its IPv4 address representation",
        arguments: &[ArgumentMetadata {
            name: "int",
            arg_type: "integer",
            required: true,
            default: None,
            description: "Integer value (0 to 4294967295)",
        }],
        return_type: "string",
        examples: &["{{ int_to_ip(int=3232235777) }}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let int_value: i64 = kwargs.get("int")?;

        if int_value < 0 || int_value > u32::MAX as i64 {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                format!(
                    "Integer must be between 0 and {}, got {}",
                    u32::MAX,
                    int_value
                ),
            ));
        }

        let ip = Ipv4Addr::from(int_value as u32);

        Ok(Value::from(ip.to_string()))
    }
}
