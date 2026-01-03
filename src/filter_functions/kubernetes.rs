//! Kubernetes functions that support both function and filter syntax.
//!
//! # Function Syntax
//! ```jinja
//! {{ k8s_label_safe(value="My App (v2.0)") }}
//! {{ k8s_dns_label_safe(value="My Service Name") }}
//! {{ k8s_annotation_safe(value="Some description") }}
//! ```
//!
//! # Filter Syntax
//! ```jinja
//! {{ "My App (v2.0)" | k8s_label_safe }}
//! {{ "My Service Name" | k8s_dns_label_safe }}
//! {{ description | k8s_annotation_safe }}
//! ```
//!
//! # Chaining
//! ```jinja
//! {{ app_name | k8s_label_safe }}
//! {{ service_name | k8s_dns_label_safe | lower }}
//! ```

use super::FilterFunction;
use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};

/// Helper to extract string from Value
fn extract_string(value: &Value, fn_name: &str) -> Result<String, Error> {
    value.as_str().map(|s| s.to_string()).ok_or_else(|| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("{} requires a string, found: {}", fn_name, value),
        )
    })
}

// ============================================
// K8sLabelSafe
// ============================================

/// Sanitize string to be Kubernetes label-safe.
///
/// Returns a sanitized string that follows Kubernetes label requirements:
/// - Max 63 characters
/// - Only alphanumeric, dashes, underscores, dots
/// - Must start and end with alphanumeric
///
/// # Function Syntax
/// ```jinja
/// {{ k8s_label_safe(value="My App (v2.0)") }}
/// {# Output: my-app-v2.0 #}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ "My App (v2.0)" | k8s_label_safe }}
/// {# Output: my-app-v2.0 #}
///
/// metadata:
///   labels:
///     app: {{ app_name | k8s_label_safe }}
/// ```
pub struct K8sLabelSafe;

impl K8sLabelSafe {
    fn compute(value: &str) -> String {
        // Convert to lowercase
        let mut result = value.to_lowercase();

        // Replace invalid characters with dashes
        result = result
            .chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.' {
                    c
                } else {
                    '-'
                }
            })
            .collect();

        // Replace multiple consecutive dashes with single dash
        while result.contains("--") {
            result = result.replace("--", "-");
        }

        // Remove leading/trailing non-alphanumeric characters
        result = result
            .trim_matches(|c: char| !c.is_ascii_alphanumeric())
            .to_string();

        // Truncate to 63 characters
        if result.len() > 63 {
            result.truncate(63);
            // Ensure it still ends with alphanumeric after truncation
            result = result
                .trim_end_matches(|c: char| !c.is_ascii_alphanumeric())
                .to_string();
        }

        // If empty after sanitization, use a default
        if result.is_empty() {
            result = "default".to_string();
        }

        result
    }
}

impl FilterFunction for K8sLabelSafe {
    const NAME: &'static str = "k8s_label_safe";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let value: String = kwargs.get("value")?;
        Ok(Value::from(Self::compute(&value)))
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        let input = extract_string(value, "k8s_label_safe")?;
        Ok(Value::from(Self::compute(&input)))
    }
}

// ============================================
// K8sDnsLabelSafe
// ============================================

/// Format DNS-safe label (max 63 chars).
///
/// Returns a DNS-safe string (lowercase, alphanumeric and dashes only, max 63 chars)
///
/// # Function Syntax
/// ```jinja
/// {{ k8s_dns_label_safe(value="My Service Name") }}
/// {# Output: my-service-name #}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ "My Service Name" | k8s_dns_label_safe }}
/// {# Output: my-service-name #}
///
/// apiVersion: v1
/// kind: Service
/// metadata:
///   name: {{ service_name | k8s_dns_label_safe }}
/// ```
pub struct K8sDnsLabelSafe;

impl K8sDnsLabelSafe {
    fn compute(value: &str) -> String {
        // Convert to lowercase
        let mut result = value.to_lowercase();

        // Replace invalid characters with dashes
        result = result
            .chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() || c == '-' {
                    c
                } else {
                    '-'
                }
            })
            .collect();

        // Remove leading/trailing dashes
        result = result.trim_matches('-').to_string();

        // Replace multiple consecutive dashes with single dash
        while result.contains("--") {
            result = result.replace("--", "-");
        }

        // Truncate to 63 characters
        if result.len() > 63 {
            result.truncate(63);
            // Ensure it still ends with alphanumeric after truncation
            result = result.trim_end_matches('-').to_string();
        }

        // If empty after sanitization, use a default
        if result.is_empty() {
            result = "default".to_string();
        }

        result
    }
}

impl FilterFunction for K8sDnsLabelSafe {
    const NAME: &'static str = "k8s_dns_label_safe";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let value: String = kwargs.get("value")?;
        Ok(Value::from(Self::compute(&value)))
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        let input = extract_string(value, "k8s_dns_label_safe")?;
        Ok(Value::from(Self::compute(&input)))
    }
}

// ============================================
// K8sAnnotationSafe
// ============================================

/// Sanitize string to be Kubernetes annotation-safe.
///
/// Annotations have fewer restrictions than labels but values should be valid UTF-8.
/// Replaces newlines, tabs, and control characters with spaces.
///
/// # Function Syntax
/// ```jinja
/// {{ k8s_annotation_safe(value="Some description with\nnewlines") }}
/// {# Output: Some description with newlines #}
/// ```
///
/// # Filter Syntax
/// ```jinja
/// {{ description | k8s_annotation_safe }}
///
/// metadata:
///   annotations:
///     description: {{ description | k8s_annotation_safe | tojson }}
/// ```
pub struct K8sAnnotationSafe;

impl K8sAnnotationSafe {
    fn compute(value: &str) -> String {
        // Replace newlines, tabs, and control characters with spaces for single-line values
        let result: String = value
            .chars()
            .map(|c| {
                if c == '\n' || c == '\t' || c == '\r' || c.is_control() {
                    ' '
                } else {
                    c
                }
            })
            .collect();

        // Truncate if extremely long (annotations can be up to 256KB total)
        // We'll limit individual values to 64KB to be safe
        if result.len() > 65536 {
            result[..65536].to_string()
        } else {
            result
        }
    }
}

impl FilterFunction for K8sAnnotationSafe {
    const NAME: &'static str = "k8s_annotation_safe";

    fn call_as_function(kwargs: Kwargs) -> Result<Value, Error> {
        let value: String = kwargs.get("value")?;
        Ok(Value::from(Self::compute(&value)))
    }

    fn call_as_filter(value: &Value, _kwargs: Kwargs) -> Result<Value, Error> {
        let input = extract_string(value, "k8s_annotation_safe")?;
        Ok(Value::from(Self::compute(&input)))
    }
}
