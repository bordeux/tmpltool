//! Kubernetes helper functions for MiniJinja templates
//!
//! This module provides Kubernetes-specific formatting and validation functions:
//! - Resource request/limit formatting
//! - Label sanitization
//! - ConfigMap and Secret references

use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};

/// Format Kubernetes resource requests
///
/// # Arguments
///
/// * `cpu` (required) - CPU request (string like "500m" or number like 0.5)
/// * `memory` (required) - Memory request (string like "512Mi" or number for MiB)
///
/// # Returns
///
/// Returns a YAML-formatted string with resource requests
///
/// # Example
///
/// ```jinja
/// {# Basic usage with strings #}
/// {{ k8s_resource_request(cpu="500m", memory="512Mi") }}
/// {# Output:
/// requests:
///   cpu: "500m"
///   memory: "512Mi"
/// #}
///
/// {# With numeric values (auto-formatted) #}
/// {{ k8s_resource_request(cpu=0.5, memory=512) }}
/// {# Output:
/// requests:
///   cpu: "500m"
///   memory: "512Mi"
/// #}
///
/// {# In a Kubernetes deployment #}
/// apiVersion: apps/v1
/// kind: Deployment
/// metadata:
///   name: my-app
/// spec:
///   template:
///     spec:
///       containers:
///       - name: app
///         image: myapp:latest
///         resources:
///           {{ k8s_resource_request(cpu="1000m", memory="1Gi") | indent(10) }}
///
/// {# With variables from config #}
/// {% set app_config = {"cpu": "250m", "memory": "256Mi"} %}
/// resources:
///   {{ k8s_resource_request(cpu=app_config.cpu, memory=app_config.memory) | indent(2) }}
/// ```
pub fn k8s_resource_request_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let cpu: Value = kwargs.get("cpu")?;
    let memory: Value = kwargs.get("memory")?;

    // Format CPU value
    let cpu_str = if let Some(cpu_str) = cpu.as_str() {
        // Already a string, use as-is
        cpu_str.to_string()
    } else {
        // Try to convert to number
        let json_cpu: serde_json::Value = serde_json::to_value(&cpu).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Failed to convert cpu: {}", e),
            )
        })?;

        let cpu_num = json_cpu.as_f64().ok_or_else(|| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("cpu must be a string or number, found: {}", cpu),
            )
        })?;

        // Convert to millicores (1 CPU = 1000m)
        let millicores = (cpu_num * 1000.0).round() as i64;
        format!("{}m", millicores)
    };

    // Format memory value
    let memory_str = if let Some(memory_str) = memory.as_str() {
        // Already a string, use as-is
        memory_str.to_string()
    } else {
        // Try to convert to number
        let json_memory: serde_json::Value = serde_json::to_value(&memory).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Failed to convert memory: {}", e),
            )
        })?;

        let memory_num = json_memory.as_f64().ok_or_else(|| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("memory must be a string or number, found: {}", memory),
            )
        })?;

        // Convert to appropriate unit
        if memory_num >= 1024.0 {
            // Use Gi for values >= 1024 MiB
            let gib = memory_num / 1024.0;
            if gib.fract() == 0.0 {
                format!("{}Gi", gib as i64)
            } else {
                format!("{:.2}Gi", gib)
            }
        } else {
            // Use Mi for smaller values
            if memory_num.fract() == 0.0 {
                format!("{}Mi", memory_num as i64)
            } else {
                format!("{:.2}Mi", memory_num)
            }
        }
    };

    // Build YAML output
    let output = format!(
        "requests:\n  cpu: \"{}\"\n  memory: \"{}\"",
        cpu_str, memory_str
    );

    Ok(Value::from(output))
}

/// Sanitize string to be Kubernetes label-safe
///
/// # Arguments
///
/// * `value` (required) - String to sanitize
///
/// # Returns
///
/// Returns a sanitized string that follows Kubernetes label requirements:
/// - Max 63 characters
/// - Only alphanumeric, dashes, underscores, dots
/// - Must start and end with alphanumeric
///
/// # Example
///
/// ```jinja
/// {# Sanitize label value #}
/// {{ k8s_label_safe(value="My App (v2.0)") }}
/// {# Output: my-app-v2.0 #}
///
/// {# Long string gets truncated #}
/// {{ k8s_label_safe(value="this-is-a-very-long-label-name-that-exceeds-the-kubernetes-maximum-label-length-limit") }}
/// {# Output: this-is-a-very-long-label-name-that-exceeds-the-kubernetes-ma #}
///
/// {# Use in labels #}
/// metadata:
///   labels:
///     app: {{ k8s_label_safe(value=app_name) }}
///     version: {{ k8s_label_safe(value=version) }}
/// ```
pub fn k8s_label_safe_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let value: String = kwargs.get("value")?;

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

    Ok(Value::from(result))
}

/// Format DNS-safe label (max 63 chars)
///
/// # Arguments
///
/// * `value` (required) - String to format
///
/// # Returns
///
/// Returns a DNS-safe string (lowercase, alphanumeric and dashes only, max 63 chars)
///
/// # Example
///
/// ```jinja
/// {# Format DNS label #}
/// {{ k8s_dns_label_safe(value="My Service Name") }}
/// {# Output: my-service-name #}
///
/// {# Use in service names #}
/// apiVersion: v1
/// kind: Service
/// metadata:
///   name: {{ k8s_dns_label_safe(value=service_name) }}
/// ```
pub fn k8s_dns_label_safe_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let value: String = kwargs.get("value")?;

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

    Ok(Value::from(result))
}

/// Generate Kubernetes environment variable reference
///
/// # Arguments
///
/// * `var_name` (required) - The environment variable name/key
/// * `source` (optional) - Source type: "configmap" or "secret" (default: "configmap")
/// * `name` (optional) - Name of the ConfigMap/Secret (default: uses var_name in lowercase)
///
/// # Returns
///
/// Returns a YAML-formatted valueFrom reference
///
/// # Example
///
/// ```jinja
/// {# ConfigMap reference #}
/// - name: DATABASE_HOST
///   {{ k8s_env_var_ref(var_name="DATABASE_HOST", source="configmap", name="app-config") | indent(2) }}
///
/// {# Secret reference #}
/// - name: API_KEY
///   {{ k8s_env_var_ref(var_name="API_KEY", source="secret", name="api-secrets") | indent(2) }}
/// ```
pub fn k8s_env_var_ref_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let var_name: String = kwargs.get("var_name")?;
    let source: String = kwargs
        .get("source")
        .unwrap_or_else(|_| "configmap".to_string());
    let name: String = kwargs
        .get("name")
        .unwrap_or_else(|_| var_name.to_lowercase().replace('_', "-"));

    let output = match source.as_str() {
        "secret" => format!(
            "valueFrom:\n  secretKeyRef:\n    name: {}\n    key: {}",
            name, var_name
        ),
        "configmap" => format!(
            "valueFrom:\n  configMapKeyRef:\n    name: {}\n    key: {}",
            name, var_name
        ),
        _ => {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                format!(
                    "Invalid source '{}', must be 'configmap' or 'secret'",
                    source
                ),
            ));
        }
    };

    Ok(Value::from(output))
}

/// Generate Kubernetes Secret reference
///
/// # Arguments
///
/// * `secret_name` (required) - Name of the Secret
/// * `key` (required) - Key within the Secret
/// * `optional` (optional) - Whether the Secret is optional (default: false)
///
/// # Returns
///
/// Returns a YAML-formatted valueFrom secretKeyRef
///
/// # Example
///
/// ```jinja
/// {# Basic secret reference #}
/// - name: DB_PASSWORD
///   {{ k8s_secret_ref(secret_name="db-credentials", key="password") | indent(2) }}
///
/// {# Optional secret #}
/// - name: OPTIONAL_TOKEN
///   {{ k8s_secret_ref(secret_name="tokens", key="api_token", optional=true) | indent(2) }}
/// ```
pub fn k8s_secret_ref_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let secret_name: String = kwargs.get("secret_name")?;
    let key: String = kwargs.get("key")?;
    let optional: bool = kwargs.get("optional").unwrap_or(false);

    let mut output = format!(
        "valueFrom:\n  secretKeyRef:\n    name: {}\n    key: {}",
        secret_name, key
    );

    if optional {
        output.push_str("\n    optional: true");
    }

    Ok(Value::from(output))
}

/// Generate Kubernetes ConfigMap reference
///
/// # Arguments
///
/// * `configmap_name` (required) - Name of the ConfigMap
/// * `key` (required) - Key within the ConfigMap
/// * `optional` (optional) - Whether the ConfigMap is optional (default: false)
///
/// # Returns
///
/// Returns a YAML-formatted valueFrom configMapKeyRef
///
/// # Example
///
/// ```jinja
/// {# Basic ConfigMap reference #}
/// - name: DATABASE_HOST
///   {{ k8s_configmap_ref(configmap_name="app-config", key="database_host") | indent(2) }}
///
/// {# Optional ConfigMap #}
/// - name: FEATURE_FLAG
///   {{ k8s_configmap_ref(configmap_name="features", key="new_ui", optional=true) | indent(2) }}
/// ```
pub fn k8s_configmap_ref_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let configmap_name: String = kwargs.get("configmap_name")?;
    let key: String = kwargs.get("key")?;
    let optional: bool = kwargs.get("optional").unwrap_or(false);

    let mut output = format!(
        "valueFrom:\n  configMapKeyRef:\n    name: {}\n    key: {}",
        configmap_name, key
    );

    if optional {
        output.push_str("\n    optional: true");
    }

    Ok(Value::from(output))
}
