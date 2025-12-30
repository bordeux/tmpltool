/// Validation functions
///
/// Provides functions for validating various string formats:
/// - is_email: Validate email address format
/// - is_url: Validate URL format
/// - is_ip: Validate IP address (IPv4 or IPv6)
/// - is_uuid: Validate UUID format
/// - matches_regex: Check if string matches regex pattern
use regex::Regex;
use std::collections::HashMap;
use tera::{Function, Result, Value, to_value};

/// Validate email address format
pub struct IsEmail;

impl Function for IsEmail {
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value> {
        let string = args.get("string").and_then(|v| v.as_str()).ok_or_else(|| {
            tera::Error::msg(
                "is_email requires a 'string' argument (e.g., string=\"user@example.com\")",
            )
        })?;

        // Simple email validation regex
        // This is not exhaustive but covers most common cases
        let email_re = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        let is_valid = email_re.is_match(string);

        to_value(is_valid).map_err(|e| tera::Error::msg(format!("Failed to convert result: {}", e)))
    }
}

/// Validate URL format
pub struct IsUrl;

impl Function for IsUrl {
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value> {
        let string = args.get("string").and_then(|v| v.as_str()).ok_or_else(|| {
            tera::Error::msg(
                "is_url requires a 'string' argument (e.g., string=\"https://example.com\")",
            )
        })?;

        // URL validation regex - supports http(s), ftp, file schemes
        let url_re = Regex::new(
            r"^(https?|ftp|file)://[-A-Za-z0-9+&@#/%?=~_|!:,.;]*[-A-Za-z0-9+&@#/%=~_|]$",
        )
        .unwrap();
        let is_valid = url_re.is_match(string);

        to_value(is_valid).map_err(|e| tera::Error::msg(format!("Failed to convert result: {}", e)))
    }
}

/// Validate IP address (IPv4 or IPv6)
pub struct IsIp;

impl Function for IsIp {
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value> {
        let string = args.get("string").and_then(|v| v.as_str()).ok_or_else(|| {
            tera::Error::msg("is_ip requires a 'string' argument (e.g., string=\"192.168.1.1\")")
        })?;

        // Try parsing as standard library's IP address types
        let is_valid = string.parse::<std::net::IpAddr>().is_ok();

        to_value(is_valid).map_err(|e| tera::Error::msg(format!("Failed to convert result: {}", e)))
    }
}

/// Validate UUID format
pub struct IsUuid;

impl Function for IsUuid {
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value> {
        let string = args
            .get("string")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                tera::Error::msg(
                    "is_uuid requires a 'string' argument (e.g., string=\"550e8400-e29b-41d4-a716-446655440000\")",
                )
            })?;

        // UUID validation regex (supports all UUID versions)
        let uuid_re = Regex::new(
            r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$",
        )
        .unwrap();
        let is_valid = uuid_re.is_match(string);

        to_value(is_valid).map_err(|e| tera::Error::msg(format!("Failed to convert result: {}", e)))
    }
}

/// Check if string matches regex pattern
pub struct MatchesRegex;

impl Function for MatchesRegex {
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value> {
        let pattern = args
            .get("pattern")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                tera::Error::msg(
                    "matches_regex requires a 'pattern' argument (e.g., pattern=\"^[A-Z]+$\")",
                )
            })?;

        let string = args.get("string").and_then(|v| v.as_str()).ok_or_else(|| {
            tera::Error::msg("matches_regex requires a 'string' argument (e.g., string=\"HELLO\")")
        })?;

        // Compile and match regex
        let re = Regex::new(pattern)
            .map_err(|e| tera::Error::msg(format!("Invalid regex pattern '{}': {}", pattern, e)))?;

        let is_match = re.is_match(string);

        to_value(is_match).map_err(|e| tera::Error::msg(format!("Failed to convert result: {}", e)))
    }
}
