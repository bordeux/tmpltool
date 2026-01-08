## System & Network Functions

System information (hostname, username, OS) and network functions (IP, DNS, ports, CIDR).

**See also:** [Function Reference](../FUNCTIONS.md) | [Main Documentation](../README.md)

Access system information and perform network operations.

#### `get_hostname()`

Get the system hostname.

**Arguments:** None

**Returns:** String containing the system hostname

**Example:**
```
Server: {{ get_hostname() }}
{# Output: Server: myserver.local #}
```

#### `get_username()`

Get the current system username.

**Arguments:** None

**Returns:** String containing the current username

**Example:**
```
User: {{ get_username() }}
{# Output: User: john #}
```

#### `get_home_dir()`

Get the user's home directory.

**Arguments:** None

**Returns:** String containing the home directory path

**Example:**
```
Home: {{ get_home_dir() }}
{# Output: Home: /Users/john #}
```

#### `get_temp_dir()`

Get the system temporary directory.

**Arguments:** None

**Returns:** String containing the temp directory path

**Example:**
```
Temp: {{ get_temp_dir() }}
{# Output: Temp: /tmp #}
```

#### `get_ip_address(interface)`

Get IP address of a network interface or the primary local IP.

**Arguments:**
- `interface` (optional) - Network interface name (e.g., "eth0", "en0")

**Returns:** String containing the IP address

**Example:**
```
{# Get primary local IP #}
Local IP: {{ get_ip_address() }}
{# Output: Local IP: 192.168.1.100 #}

{# Get specific interface IP #}
Eth0 IP: {{ get_ip_address(interface="eth0") }}
```

#### `get_interfaces()`

Get a list of all network interfaces with their IP addresses.

**Arguments:** None

**Returns:** List of objects with interface information:
- `name` - Interface name (e.g., "eth0", "en0", "lo")
- `ip` - IP address assigned to the interface
- `is_loopback` - Boolean indicating if this is a loopback interface

**Example:**
```
{# List all interfaces #}
{% for iface in get_interfaces() %}
  {{ iface.name }}: {{ iface.ip }}{% if iface.is_loopback %} (loopback){% endif %}
{% endfor %}

{# Filter non-loopback interfaces #}
{% for iface in get_interfaces() | selectattr("is_loopback", "equalto", false) %}
  {{ iface.name }}: {{ iface.ip }}
{% endfor %}

{# Find first non-loopback IP #}
{% set external_iface = get_interfaces() | selectattr("is_loopback", "equalto", false) | first %}
Bind IP: {{ external_iface.ip }}
```

#### `resolve_dns(hostname)`

Resolve a hostname to an IP address using DNS.

**Arguments:**
- `hostname` (required) - Hostname to resolve

**Returns:** String containing the resolved IP address

**Example:**
```
Google IP: {{ resolve_dns(hostname="google.com") }}
{# Output: Google IP: 142.250.190.46 #}

Local: {{ resolve_dns(hostname="localhost") }}
{# Output: Local: 127.0.0.1 or ::1 #}
```

#### `is_port_available(port)` / `{% if port is port_available %}`

Check if a port is available (not in use). Supports both function syntax and "is" test syntax.

**Function Syntax Arguments:**
- `port` (required) - Port number to check (1-65535)

**Is-Test Syntax:**
- The value must be an integer between 1 and 65535, or a string that can be parsed as such

**Returns:** Boolean (`true` if available, `false` if in use)

**Examples:**
```jinja
{# Function syntax #}
{% if is_port_available(port=8080) %}
  Port 8080 is available
{% else %}
  Port 8080 is already in use
{% endif %}

{# Is-test syntax (preferred for readability) #}
{% if 8080 is port_available %}
  Port 8080 is available
{% endif %}

{# With variables #}
{% set my_port = 3000 %}
{% if my_port is port_available %}
APP_PORT={{ my_port }}
{% elif 3001 is port_available %}
APP_PORT=3001
{% else %}
APP_PORT=8080
{% endif %}
```

#### `get_os()`

Get the operating system name.

**Arguments:** None

**Returns:** String containing the OS name (e.g., "linux", "macos", "windows")

**Example:**
```
OS: {{ get_os() }}
{# Output: OS: macos #}

{% if get_os() == "linux" %}
  Running on Linux
{% elif get_os() == "macos" %}
  Running on macOS
{% endif %}
```

#### `get_arch()`

Get the CPU architecture.

**Arguments:** None

**Returns:** String containing the architecture (e.g., "x86_64", "aarch64", "arm")

**Example:**
```
Arch: {{ get_arch() }}
{# Output: Arch: aarch64 #}

{% if get_arch() == "aarch64" %}
  Running on ARM64
{% elif get_arch() == "x86_64" %}
  Running on x86-64
{% endif %}
```

#### `get_cwd()`

Get the current working directory.

**Arguments:** None

**Returns:** String containing the current working directory path

**Example:**
```
CWD: {{ get_cwd() }}
{# Output: CWD: /home/user/projects/myapp #}
```

#### `cidr_contains(cidr, ip)`

Check if an IP address is within a CIDR range.

**Arguments:**
- `cidr` (required) - CIDR notation (e.g., "192.168.1.0/24")
- `ip` (required) - IP address to check

**Returns:** Boolean (`true` if IP is in range, `false` otherwise)

**Example:**
```
{% if cidr_contains(cidr="192.168.1.0/24", ip="192.168.1.100") %}
  IP is in the subnet
{% else %}
  IP is outside the subnet
{% endif %}
{# Output: IP is in the subnet #}

{% if cidr_contains(cidr="10.0.0.0/8", ip="192.168.1.1") %}
  In private range
{% else %}
  Not in 10.x.x.x range
{% endif %}
{# Output: Not in 10.x.x.x range #}
```

#### `cidr_network(cidr)`

Get the network address from a CIDR notation.

**Arguments:**
- `cidr` (required) - CIDR notation (e.g., "192.168.1.100/24")

**Returns:** String containing the network address

**Example:**
```
Network: {{ cidr_network(cidr="192.168.1.100/24") }}
{# Output: Network: 192.168.1.0 #}

Network: {{ cidr_network(cidr="10.20.30.40/16") }}
{# Output: Network: 10.20.0.0 #}
```

#### `cidr_broadcast(cidr)`

Get the broadcast address from a CIDR notation.

**Arguments:**
- `cidr` (required) - CIDR notation (e.g., "192.168.1.0/24")

**Returns:** String containing the broadcast address

**Example:**
```
Broadcast: {{ cidr_broadcast(cidr="192.168.1.0/24") }}
{# Output: Broadcast: 192.168.1.255 #}

Broadcast: {{ cidr_broadcast(cidr="10.0.0.0/8") }}
{# Output: Broadcast: 10.255.255.255 #}
```

#### `cidr_netmask(cidr)`

Get the netmask from a CIDR notation.

**Arguments:**
- `cidr` (required) - CIDR notation (e.g., "192.168.1.0/24")

**Returns:** String containing the netmask

**Example:**
```
Netmask: {{ cidr_netmask(cidr="192.168.1.0/24") }}
{# Output: Netmask: 255.255.255.0 #}

Netmask: {{ cidr_netmask(cidr="10.0.0.0/8") }}
{# Output: Netmask: 255.0.0.0 #}

Netmask: {{ cidr_netmask(cidr="172.16.0.0/12") }}
{# Output: Netmask: 255.240.0.0 #}
```

#### `ip_to_int(ip)`

Convert an IPv4 address to its integer representation.

**Arguments:**
- `ip` (required) - IPv4 address (e.g., "192.168.1.1")

**Returns:** Integer representation of the IP address

**Example:**
```
Integer: {{ ip_to_int(ip="192.168.1.1") }}
{# Output: Integer: 3232235777 #}

Integer: {{ ip_to_int(ip="0.0.0.0") }}
{# Output: Integer: 0 #}

Integer: {{ ip_to_int(ip="255.255.255.255") }}
{# Output: Integer: 4294967295 #}
```

#### `int_to_ip(int)`

Convert an integer to its IPv4 address representation.

**Arguments:**
- `int` (required) - Integer value (0 to 4294967295)

**Returns:** String containing the IPv4 address

**Example:**
```
IP: {{ int_to_ip(int=3232235777) }}
{# Output: IP: 192.168.1.1 #}

IP: {{ int_to_ip(int=0) }}
{# Output: IP: 0.0.0.0 #}

IP: {{ int_to_ip(int=4294967295) }}
{# Output: IP: 255.255.255.255 #}
```

**Practical Example - Network Configuration:**
```yaml
network:
  # CIDR operations
  cidr: 192.168.1.0/24
  network_addr: {{ cidr_network(cidr="192.168.1.0/24") }}
  broadcast: {{ cidr_broadcast(cidr="192.168.1.0/24") }}
  netmask: {{ cidr_netmask(cidr="192.168.1.0/24") }}

  # IP validation
  {% set client_ip = "192.168.1.50" %}
  {% if cidr_contains(cidr="192.168.1.0/24", ip=client_ip) %}
  client_allowed: true
  {% else %}
  client_allowed: false
  {% endif %}

  # System info
  os: {{ get_os() }}
  arch: {{ get_arch() }}
  cwd: {{ get_cwd() }}
```

**Practical Example - Dynamic Application Config:**
```yaml
application:
  hostname: {{ get_hostname() }}
  user: {{ get_username() }}

network:
  bind_ip: {{ get_ip_address() }}
  {% if is_port_available(port=8080) %}
  port: 8080
  {% else %}
  port: 8081  # Fallback port
  {% endif %}

paths:
  home: {{ get_home_dir() }}
  temp: {{ get_temp_dir() }}
  logs: {{ get_home_dir() }}/logs/app.log

services:
  database: {{ resolve_dns(hostname="db.local") }}
  cache: {{ resolve_dns(hostname="redis.local") }}
```

