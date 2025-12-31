//! Network-related functions for MiniJinja templates
//!
//! This module provides functions for network operations like:
//! - Getting IP addresses
//! - DNS resolution
//! - Port availability checking

use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};
use std::net::{TcpListener, ToSocketAddrs};

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
