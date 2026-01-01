#!/usr/bin/env bash
# Test: CIDR and System functions


echo "Test: CIDR and System functions"

# ==================== CIDR Functions ====================

# Test: cidr_contains() - IP in range
create_template "cidr1.tmpl" '{{ cidr_contains(cidr="192.168.1.0/24", ip="192.168.1.100") }}'
OUTPUT=$(run_binary "cidr1.tmpl")
assert_equals "true" "$OUTPUT" "cidr_contains() IP in range"

# Test: cidr_contains() - IP out of range
create_template "cidr2.tmpl" '{{ cidr_contains(cidr="192.168.1.0/24", ip="192.168.2.1") }}'
OUTPUT=$(run_binary "cidr2.tmpl")
assert_equals "false" "$OUTPUT" "cidr_contains() IP out of range"

# Test: cidr_contains() - Class A network
create_template "cidr3.tmpl" '{{ cidr_contains(cidr="10.0.0.0/8", ip="10.255.255.255") }}'
OUTPUT=$(run_binary "cidr3.tmpl")
assert_equals "true" "$OUTPUT" "cidr_contains() class A network"

# Test: cidr_network() - Class C
create_template "net1.tmpl" '{{ cidr_network(cidr="192.168.1.100/24") }}'
OUTPUT=$(run_binary "net1.tmpl")
assert_equals "192.168.1.0" "$OUTPUT" "cidr_network() class C"

# Test: cidr_network() - Class B
create_template "net2.tmpl" '{{ cidr_network(cidr="172.16.50.100/16") }}'
OUTPUT=$(run_binary "net2.tmpl")
assert_equals "172.16.0.0" "$OUTPUT" "cidr_network() class B"

# Test: cidr_broadcast() - Class C
create_template "bcast1.tmpl" '{{ cidr_broadcast(cidr="192.168.1.0/24") }}'
OUTPUT=$(run_binary "bcast1.tmpl")
assert_equals "192.168.1.255" "$OUTPUT" "cidr_broadcast() class C"

# Test: cidr_broadcast() - Class A
create_template "bcast2.tmpl" '{{ cidr_broadcast(cidr="10.0.0.0/8") }}'
OUTPUT=$(run_binary "bcast2.tmpl")
assert_equals "10.255.255.255" "$OUTPUT" "cidr_broadcast() class A"

# Test: cidr_netmask() - /24
create_template "mask1.tmpl" '{{ cidr_netmask(cidr="192.168.1.0/24") }}'
OUTPUT=$(run_binary "mask1.tmpl")
assert_equals "255.255.255.0" "$OUTPUT" "cidr_netmask() /24"

# Test: cidr_netmask() - /16
create_template "mask2.tmpl" '{{ cidr_netmask(cidr="172.16.0.0/16") }}'
OUTPUT=$(run_binary "mask2.tmpl")
assert_equals "255.255.0.0" "$OUTPUT" "cidr_netmask() /16"

# Test: cidr_netmask() - /8
create_template "mask3.tmpl" '{{ cidr_netmask(cidr="10.0.0.0/8") }}'
OUTPUT=$(run_binary "mask3.tmpl")
assert_equals "255.0.0.0" "$OUTPUT" "cidr_netmask() /8"

# Test: cidr_netmask() - /12
create_template "mask4.tmpl" '{{ cidr_netmask(cidr="172.16.0.0/12") }}'
OUTPUT=$(run_binary "mask4.tmpl")
assert_equals "255.240.0.0" "$OUTPUT" "cidr_netmask() /12"

# ==================== IP Conversion Functions ====================

# Test: ip_to_int() - basic
create_template "ip2int1.tmpl" '{{ ip_to_int(ip="192.168.1.1") }}'
OUTPUT=$(run_binary "ip2int1.tmpl")
assert_equals "3232235777" "$OUTPUT" "ip_to_int() basic"

# Test: ip_to_int() - zero
create_template "ip2int2.tmpl" '{{ ip_to_int(ip="0.0.0.0") }}'
OUTPUT=$(run_binary "ip2int2.tmpl")
assert_equals "0" "$OUTPUT" "ip_to_int() zero"

# Test: ip_to_int() - max
create_template "ip2int3.tmpl" '{{ ip_to_int(ip="255.255.255.255") }}'
OUTPUT=$(run_binary "ip2int3.tmpl")
assert_equals "4294967295" "$OUTPUT" "ip_to_int() max"

# Test: int_to_ip() - basic
create_template "int2ip1.tmpl" '{{ int_to_ip(int=3232235777) }}'
OUTPUT=$(run_binary "int2ip1.tmpl")
assert_equals "192.168.1.1" "$OUTPUT" "int_to_ip() basic"

# Test: int_to_ip() - zero
create_template "int2ip2.tmpl" '{{ int_to_ip(int=0) }}'
OUTPUT=$(run_binary "int2ip2.tmpl")
assert_equals "0.0.0.0" "$OUTPUT" "int_to_ip() zero"

# Test: int_to_ip() - max
create_template "int2ip3.tmpl" '{{ int_to_ip(int=4294967295) }}'
OUTPUT=$(run_binary "int2ip3.tmpl")
assert_equals "255.255.255.255" "$OUTPUT" "int_to_ip() max"

# Test: roundtrip conversion
create_template "roundtrip.tmpl" '{% set ip = "10.20.30.40" %}{% set num = ip_to_int(ip=ip) %}{{ int_to_ip(int=num) }}'
OUTPUT=$(run_binary "roundtrip.tmpl")
assert_equals "10.20.30.40" "$OUTPUT" "ip_to_int/int_to_ip roundtrip"

# ==================== System Functions ====================

# Test: get_os() - returns valid OS
create_template "os1.tmpl" '{{ get_os() }}'
OUTPUT=$(run_binary "os1.tmpl")
assert_matches "$OUTPUT" "^(linux|macos|windows|freebsd)$" "get_os() returns valid OS"

# Test: get_arch() - returns valid architecture
create_template "arch1.tmpl" '{{ get_arch() }}'
OUTPUT=$(run_binary "arch1.tmpl")
assert_matches "$OUTPUT" "^(x86_64|x86|aarch64|arm)$" "get_arch() returns valid architecture"

# Test: get_cwd() - returns non-empty path
create_template "cwd1.tmpl" '{{ get_cwd() }}'
OUTPUT=$(run_binary "cwd1.tmpl")
assert_matches "$OUTPUT" "^/" "get_cwd() returns absolute path"

# Test: Combined system info
create_template "sysinfo.tmpl" 'OS={{ get_os() }}, Arch={{ get_arch() }}'
OUTPUT=$(run_binary "sysinfo.tmpl")
assert_contains "$OUTPUT" "OS=" "combined system info has OS"
assert_contains "$OUTPUT" "Arch=" "combined system info has Arch"

# ==================== Practical Examples ====================

# Test: Network configuration template
create_template "netconfig.tmpl" 'network: {{ cidr_network(cidr="192.168.1.0/24") }}
broadcast: {{ cidr_broadcast(cidr="192.168.1.0/24") }}
netmask: {{ cidr_netmask(cidr="192.168.1.0/24") }}'
OUTPUT=$(run_binary "netconfig.tmpl")
assert_contains "$OUTPUT" "network: 192.168.1.0" "network config has network address"
assert_contains "$OUTPUT" "broadcast: 192.168.1.255" "network config has broadcast"
assert_contains "$OUTPUT" "netmask: 255.255.255.0" "network config has netmask"

# Test: Conditional based on OS
create_template "oscheck.tmpl" '{% if get_os() == "linux" %}Linux detected{% elif get_os() == "macos" %}macOS detected{% else %}Other OS{% endif %}'
OUTPUT=$(run_binary "oscheck.tmpl")
assert_contains "$OUTPUT" "detected\|Other" "OS conditional works"

# Test: IP range check in config
create_template "ipcheck.tmpl" '{% set client = "192.168.1.50" %}{% if cidr_contains(cidr="192.168.1.0/24", ip=client) %}allowed{% else %}denied{% endif %}'
OUTPUT=$(run_binary "ipcheck.tmpl")
assert_equals "allowed" "$OUTPUT" "IP range check allows valid IP"
