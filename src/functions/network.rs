//! Network-related functions for MiniJinja templates
//!
//! This module provides functions for network operations like:
//! - Getting IP addresses
//! - DNS resolution
//! - Port availability checking
//! - CIDR operations
//! - IP address conversion

use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};
use std::net::{Ipv4Addr, TcpListener, ToSocketAddrs};

/// Get IP address of a network interface or the primary local IP
///
/// # Arguments
///
/// * `interface` (optional) - Network interface name (e.g., "eth0", "en0")
///   If not provided, attempts to get the primary local IP address
///
/// # Returns
///
/// Returns the IP address as a string
///
/// # Example
///
/// ```jinja
/// {# Get primary local IP #}
/// IP: {{ get_ip_address() }}
///
/// {# Get specific interface IP (platform-specific) #}
/// IP: {{ get_ip_address(interface="eth0") }}
/// ```
pub fn get_ip_address_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let interface: Option<String> = kwargs.get("interface").ok();

    if let Some(iface) = interface {
        // Try to get IP for specific interface
        get_interface_ip(&iface)
    } else {
        // Get primary local IP by connecting to an external address
        get_local_ip()
    }
}

/// Get the primary local IP address
///
/// This works by creating a connection to an external address (doesn't actually send data)
/// and checking what local IP the system would use
fn get_local_ip() -> Result<Value, Error> {
    // Connect to a well-known DNS server to determine our local IP
    // This doesn't actually send any data, just determines routing
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
    // Use if-addrs crate to get interface information
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

/// Resolve a hostname to an IP address using DNS
///
/// # Arguments
///
/// * `hostname` (required) - Hostname to resolve (e.g., "google.com", "localhost")
///
/// # Returns
///
/// Returns the first resolved IP address as a string
///
/// # Example
///
/// ```jinja
/// IP for google.com: {{ resolve_dns(hostname="google.com") }}
/// Localhost IP: {{ resolve_dns(hostname="localhost") }}
/// ```
pub fn resolve_dns_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let hostname: String = kwargs.get("hostname")?;

    // Add default port for DNS resolution (doesn't matter which port)
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

    // Return the first IP address
    Ok(Value::from(addrs[0].ip().to_string()))
}

/// Check if a port is available (not in use)
///
/// # Arguments
///
/// * `port` (required) - Port number to check (1-65535)
///
/// # Returns
///
/// Returns true if the port is available, false if it's in use
///
/// # Example
///
/// ```jinja
/// {% if is_port_available(port=8080) %}
///   Port 8080 is available
/// {% else %}
///   Port 8080 is in use
/// {% endif %}
/// ```
pub fn is_port_available_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let port: u16 = kwargs.get::<i64>("port").and_then(|p| {
        if (1..=65535).contains(&p) {
            Ok(p as u16)
        } else {
            Err(Error::new(
                ErrorKind::InvalidOperation,
                format!("Port must be between 1 and 65535, got {}", p),
            ))
        }
    })?;

    // Try to bind to the port on all interfaces
    // If successful, the port is available
    let is_available = TcpListener::bind(("0.0.0.0", port)).is_ok();

    Ok(Value::from(is_available))
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
///
/// # Arguments
///
/// * `cidr` (required) - CIDR notation (e.g., "192.168.1.0/24")
/// * `ip` (required) - IP address to check (e.g., "192.168.1.100")
///
/// # Returns
///
/// Returns true if the IP is within the CIDR range, false otherwise
///
/// # Example
///
/// ```jinja
/// {% if cidr_contains(cidr="192.168.1.0/24", ip="192.168.1.100") %}
///   IP is in the subnet
/// {% endif %}
/// ```
pub fn cidr_contains_fn(kwargs: Kwargs) -> Result<Value, Error> {
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

    // Check if both IPs have the same network portion
    let is_contained = (network_int & mask) == (check_int & mask);

    Ok(Value::from(is_contained))
}

/// Get the network address from a CIDR notation
///
/// # Arguments
///
/// * `cidr` (required) - CIDR notation (e.g., "192.168.1.100/24")
///
/// # Returns
///
/// Returns the network address (e.g., "192.168.1.0")
///
/// # Example
///
/// ```jinja
/// Network: {{ cidr_network(cidr="192.168.1.100/24") }}
/// {# Output: 192.168.1.0 #}
/// ```
pub fn cidr_network_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let cidr: String = kwargs.get("cidr")?;

    let (ip, prefix) = parse_cidr(&cidr)?;
    let mask = prefix_to_mask(prefix);
    let network_int = u32::from(ip) & mask;
    let network_ip = Ipv4Addr::from(network_int);

    Ok(Value::from(network_ip.to_string()))
}

/// Get the broadcast address from a CIDR notation
///
/// # Arguments
///
/// * `cidr` (required) - CIDR notation (e.g., "192.168.1.0/24")
///
/// # Returns
///
/// Returns the broadcast address (e.g., "192.168.1.255")
///
/// # Example
///
/// ```jinja
/// Broadcast: {{ cidr_broadcast(cidr="192.168.1.0/24") }}
/// {# Output: 192.168.1.255 #}
/// ```
pub fn cidr_broadcast_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let cidr: String = kwargs.get("cidr")?;

    let (ip, prefix) = parse_cidr(&cidr)?;
    let mask = prefix_to_mask(prefix);
    let network_int = u32::from(ip) & mask;
    let broadcast_int = network_int | !mask;
    let broadcast_ip = Ipv4Addr::from(broadcast_int);

    Ok(Value::from(broadcast_ip.to_string()))
}

/// Get the netmask from a CIDR notation
///
/// # Arguments
///
/// * `cidr` (required) - CIDR notation (e.g., "192.168.1.0/24")
///
/// # Returns
///
/// Returns the netmask (e.g., "255.255.255.0")
///
/// # Example
///
/// ```jinja
/// Netmask: {{ cidr_netmask(cidr="192.168.1.0/24") }}
/// {# Output: 255.255.255.0 #}
/// ```
pub fn cidr_netmask_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let cidr: String = kwargs.get("cidr")?;

    let (_, prefix) = parse_cidr(&cidr)?;
    let mask = prefix_to_mask(prefix);
    let mask_ip = Ipv4Addr::from(mask);

    Ok(Value::from(mask_ip.to_string()))
}

/// Convert an IPv4 address to its integer representation
///
/// # Arguments
///
/// * `ip` (required) - IPv4 address (e.g., "192.168.1.1")
///
/// # Returns
///
/// Returns the integer representation of the IP address
///
/// # Example
///
/// ```jinja
/// Integer: {{ ip_to_int(ip="192.168.1.1") }}
/// {# Output: 3232235777 #}
/// ```
pub fn ip_to_int_fn(kwargs: Kwargs) -> Result<Value, Error> {
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

/// Convert an integer to its IPv4 address representation
///
/// # Arguments
///
/// * `int` (required) - Integer value (0 to 4294967295)
///
/// # Returns
///
/// Returns the IPv4 address as a string
///
/// # Example
///
/// ```jinja
/// IP: {{ int_to_ip(int=3232235777) }}
/// {# Output: 192.168.1.1 #}
/// ```
pub fn int_to_ip_fn(kwargs: Kwargs) -> Result<Value, Error> {
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
