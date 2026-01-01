//! Kubernetes helper functions for MiniJinja templates
//!
//! This module provides Kubernetes-specific formatting and validation functions:
//! - Resource request/limit formatting
//! - Label sanitization
//! - ConfigMap and Secret references
//! - Quantity conversions
//! - Pod affinity and toleration generation
//! - Liveness/readiness probe generation

use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};
use std::collections::HashMap;

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

/// Helm-style template function
///
/// Renders a template string with provided values, similar to Helm's `tpl` function.
///
/// # Arguments
///
/// * `template` (required) - Template string with {{ .key }} placeholders
/// * `values` (required) - Object with values to substitute
///
/// # Returns
///
/// Returns the rendered template string
///
/// # Example
///
/// ```jinja
/// {% set tpl = "Hello {{ .name }}, you have {{ .count }} messages" %}
/// {% set vals = {"name": "Alice", "count": 5} %}
/// {{ helm_tpl(template=tpl, values=vals) }}
/// {# Output: Hello Alice, you have 5 messages #}
/// ```
pub fn helm_tpl_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let template: String = kwargs.get("template")?;
    let values: Value = kwargs.get("values")?;

    // Convert values to a JSON value for nested lookup
    let json_values: serde_json::Value = serde_json::to_value(&values).map_err(|e| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("Failed to convert values: {}", e),
        )
    })?;

    let mut result = template.clone();

    // Find all {{ .path }} patterns and replace them
    let re = regex::Regex::new(r"\{\{\s*\.([a-zA-Z0-9_.]+)\s*\}\}").unwrap();

    // Collect all matches first to avoid borrow issues
    let matches: Vec<(String, String)> = re
        .captures_iter(&template)
        .map(|cap| {
            let full_match = cap.get(0).unwrap().as_str().to_string();
            let path = cap.get(1).unwrap().as_str().to_string();
            (full_match, path)
        })
        .collect();

    for (placeholder, path) in matches {
        // Navigate through nested values
        let mut current = &json_values;
        for part in path.split('.') {
            current = match current.get(part) {
                Some(v) => v,
                None => &serde_json::Value::Null,
            };
        }

        let replacement = match current {
            serde_json::Value::String(s) => s.clone(),
            serde_json::Value::Number(n) => n.to_string(),
            serde_json::Value::Bool(b) => b.to_string(),
            serde_json::Value::Null => "".to_string(),
            _ => serde_json::to_string(current).unwrap_or_default(),
        };
        result = result.replace(&placeholder, &replacement);
    }

    Ok(Value::from(result))
}

/// Sanitize string to be Kubernetes annotation-safe
///
/// # Arguments
///
/// * `value` (required) - String to sanitize
///
/// # Returns
///
/// Returns a sanitized string suitable for Kubernetes annotations.
/// Annotations have fewer restrictions than labels but values should be valid UTF-8.
///
/// # Example
///
/// ```jinja
/// {{ k8s_annotation_safe(value="Some description with special chars: <>&\"'") }}
/// {# Output: Some description with special chars: <>&"' #}
///
/// metadata:
///   annotations:
///     description: {{ k8s_annotation_safe(value=description) | tojson }}
/// ```
pub fn k8s_annotation_safe_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let value: String = kwargs.get("value")?;

    // Kubernetes annotations can contain any valid UTF-8 string
    // Main concern is ensuring the string is valid and not too long
    // Max total size for annotations is 256KB, but individual values should be reasonable

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
        Ok(Value::from(result[..65536].to_string()))
    } else {
        Ok(Value::from(result))
    }
}

/// Convert Kubernetes quantity string to bytes
///
/// # Arguments
///
/// * `quantity` (required) - Kubernetes quantity string (e.g., "1Gi", "500Mi", "100Ki")
///
/// # Returns
///
/// Returns the value in bytes as an integer
///
/// # Example
///
/// ```jinja
/// {{ k8s_quantity_to_bytes(quantity="1Gi") }}
/// {# Output: 1073741824 #}
///
/// {{ k8s_quantity_to_bytes(quantity="500Mi") }}
/// {# Output: 524288000 #}
///
/// {{ k8s_quantity_to_bytes(quantity="100Ki") }}
/// {# Output: 102400 #}
/// ```
pub fn k8s_quantity_to_bytes_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let quantity: String = kwargs.get("quantity")?;

    // Parse quantity string
    let (num_str, suffix) = parse_quantity_string(&quantity)?;

    let num: f64 = num_str.parse().map_err(|_| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("Invalid number in quantity: '{}'", quantity),
        )
    })?;

    // Handle millicores (m suffix) for CPU - just return the number as-is
    if suffix == "m" {
        let millicores = (num * 1.0) as i64;
        return Ok(Value::from(millicores));
    }

    // Convert based on suffix (binary prefixes for Ki, Mi, Gi, Ti, Pi, Ei)
    let multiplier: u64 = match suffix.as_str() {
        "" => 1,
        "Ki" => 1024,
        "Mi" => 1024 * 1024,
        "Gi" => 1024 * 1024 * 1024,
        "Ti" => 1024_u64 * 1024 * 1024 * 1024,
        "Pi" => 1024_u64 * 1024 * 1024 * 1024 * 1024,
        "Ei" => 1024_u64 * 1024 * 1024 * 1024 * 1024 * 1024,
        // Decimal prefixes
        "k" | "K" => 1000,
        "M" => 1000 * 1000,
        "G" => 1000 * 1000 * 1000,
        "T" => 1000_u64 * 1000 * 1000 * 1000,
        "P" => 1000_u64 * 1000 * 1000 * 1000 * 1000,
        "E" => 1000_u64 * 1000 * 1000 * 1000 * 1000 * 1000,
        _ => {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                format!("Unknown quantity suffix: '{}'", suffix),
            ));
        }
    };

    let bytes = (num * multiplier as f64) as i64;
    Ok(Value::from(bytes))
}

/// Convert bytes to Kubernetes quantity string
///
/// # Arguments
///
/// * `bytes` (required) - Number of bytes
/// * `unit` (optional) - Force a specific unit (e.g., "Mi", "Gi", "Ki")
/// * `binary` (optional) - Use binary prefixes (Ki, Mi, Gi) if true, decimal (K, M, G) if false (default: true)
///
/// # Returns
///
/// Returns a human-readable Kubernetes quantity string
///
/// # Example
///
/// ```jinja
/// {{ k8s_bytes_to_quantity(bytes=1073741824) }}
/// {# Output: 1Gi #}
///
/// {{ k8s_bytes_to_quantity(bytes=524288000) }}
/// {# Output: 500Mi #}
///
/// {{ k8s_bytes_to_quantity(bytes=1073741824, unit="Mi") }}
/// {# Output: 1024Mi #}
///
/// {{ k8s_bytes_to_quantity(bytes=1000000000, binary=false) }}
/// {# Output: 1G #}
/// ```
pub fn k8s_bytes_to_quantity_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let bytes: i64 = kwargs.get("bytes")?;
    let unit: Option<String> = kwargs.get("unit").ok();
    let binary: bool = kwargs.get("binary").unwrap_or(true);

    if bytes < 0 {
        return Err(Error::new(
            ErrorKind::InvalidOperation,
            "Bytes cannot be negative",
        ));
    }

    let bytes = bytes as u64;

    // If a specific unit is requested, convert to that unit
    if let Some(ref unit_str) = unit {
        let divisor: u64 = match unit_str.as_str() {
            "Ki" => 1024,
            "Mi" => 1024 * 1024,
            "Gi" => 1024 * 1024 * 1024,
            "Ti" => 1024_u64 * 1024 * 1024 * 1024,
            "Pi" => 1024_u64 * 1024 * 1024 * 1024 * 1024,
            "Ei" => 1024_u64 * 1024 * 1024 * 1024 * 1024 * 1024,
            "K" => 1000,
            "M" => 1000 * 1000,
            "G" => 1000 * 1000 * 1000,
            "T" => 1000_u64 * 1000 * 1000 * 1000,
            "P" => 1000_u64 * 1000 * 1000 * 1000 * 1000,
            "E" => 1000_u64 * 1000 * 1000 * 1000 * 1000 * 1000,
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidOperation,
                    format!("Unknown unit: '{}'", unit_str),
                ));
            }
        };
        let value = bytes / divisor;
        return Ok(Value::from(format!("{}{}", value, unit_str)));
    }

    let result = if binary {
        // Binary prefixes (Ki, Mi, Gi, Ti)
        if bytes >= 1024_u64 * 1024 * 1024 * 1024 {
            let ti = bytes as f64 / (1024_u64 * 1024 * 1024 * 1024) as f64;
            if ti.fract() == 0.0 {
                format!("{}Ti", ti as u64)
            } else {
                format!("{:.2}Ti", ti)
            }
        } else if bytes >= 1024 * 1024 * 1024 {
            let gi = bytes as f64 / (1024 * 1024 * 1024) as f64;
            if gi.fract() == 0.0 {
                format!("{}Gi", gi as u64)
            } else {
                format!("{:.2}Gi", gi)
            }
        } else if bytes >= 1024 * 1024 {
            let mi = bytes as f64 / (1024 * 1024) as f64;
            if mi.fract() == 0.0 {
                format!("{}Mi", mi as u64)
            } else {
                format!("{:.2}Mi", mi)
            }
        } else if bytes >= 1024 {
            let ki = bytes as f64 / 1024.0;
            if ki.fract() == 0.0 {
                format!("{}Ki", ki as u64)
            } else {
                format!("{:.2}Ki", ki)
            }
        } else {
            format!("{}", bytes)
        }
    } else {
        // Decimal prefixes (K, M, G, T)
        if bytes >= 1000_u64 * 1000 * 1000 * 1000 {
            let t = bytes as f64 / (1000_u64 * 1000 * 1000 * 1000) as f64;
            if t.fract() == 0.0 {
                format!("{}T", t as u64)
            } else {
                format!("{:.2}T", t)
            }
        } else if bytes >= 1000 * 1000 * 1000 {
            let g = bytes as f64 / (1000 * 1000 * 1000) as f64;
            if g.fract() == 0.0 {
                format!("{}G", g as u64)
            } else {
                format!("{:.2}G", g)
            }
        } else if bytes >= 1000 * 1000 {
            let m = bytes as f64 / (1000 * 1000) as f64;
            if m.fract() == 0.0 {
                format!("{}M", m as u64)
            } else {
                format!("{:.2}M", m)
            }
        } else if bytes >= 1000 {
            let k = bytes as f64 / 1000.0;
            if k.fract() == 0.0 {
                format!("{}K", k as u64)
            } else {
                format!("{:.2}K", k)
            }
        } else {
            format!("{}", bytes)
        }
    };

    Ok(Value::from(result))
}

/// Helper function to parse quantity string into number and suffix
fn parse_quantity_string(quantity: &str) -> Result<(String, String), Error> {
    let quantity = quantity.trim();

    // Find where the number ends and suffix begins
    let mut num_end = 0;
    for (i, c) in quantity.char_indices() {
        if c.is_ascii_digit() || c == '.' || c == '-' {
            num_end = i + c.len_utf8();
        } else {
            break;
        }
    }

    if num_end == 0 {
        return Err(Error::new(
            ErrorKind::InvalidOperation,
            format!("Invalid quantity format: '{}'", quantity),
        ));
    }

    let num_str = quantity[..num_end].to_string();
    let suffix = quantity[num_end..].to_string();

    Ok((num_str, suffix))
}

/// Generate Kubernetes label selector string
///
/// # Arguments
///
/// * `labels` (required) - Object with label key-value pairs
///
/// # Returns
///
/// Returns a comma-separated label selector string
///
/// # Example
///
/// ```jinja
/// {% set labels = {"app": "nginx", "env": "production", "version": "v1"} %}
/// {{ k8s_selector(labels=labels) }}
/// {# Output: app=nginx,env=production,version=v1 #}
///
/// {# Use in kubectl commands #}
/// kubectl get pods -l {{ k8s_selector(labels={"app": "myapp"}) }}
/// ```
pub fn k8s_selector_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let labels: Value = kwargs.get("labels")?;

    let labels_map: HashMap<String, serde_json::Value> =
        serde_json::from_value(serde_json::to_value(&labels).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Failed to convert labels: {}", e),
            )
        })?)
        .map_err(|_| Error::new(ErrorKind::InvalidOperation, "labels must be an object"))?;

    let mut selectors: Vec<String> = labels_map
        .iter()
        .map(|(k, v)| {
            let value_str = match v {
                serde_json::Value::String(s) => s.clone(),
                _ => v.to_string().trim_matches('"').to_string(),
            };
            format!("{}={}", k, value_str)
        })
        .collect();

    // Sort for consistent output
    selectors.sort();

    Ok(Value::from(selectors.join(",")))
}

/// Generate Kubernetes pod affinity YAML
///
/// # Arguments
///
/// * `key` (required) - Label key for the affinity rule
/// * `operator` (required) - Operator: "In", "NotIn", "Exists", "DoesNotExist"
/// * `values` (optional) - Array of values (required for In/NotIn operators)
/// * `topology_key` (optional) - Topology key (default: "kubernetes.io/hostname")
/// * `type` (optional) - Affinity type: "required" or "preferred" (default: "preferred")
/// * `weight` (optional) - Weight for preferred affinity (default: 100)
///
/// # Returns
///
/// Returns YAML-formatted pod affinity configuration
///
/// # Example
///
/// ```jinja
/// {{ k8s_pod_affinity(key="app", operator="In", values=["web", "api"]) }}
/// {# Output:
/// podAffinity:
///   preferredDuringSchedulingIgnoredDuringExecution:
///   - weight: 100
///     podAffinityTerm:
///       labelSelector:
///         matchExpressions:
///         - key: app
///           operator: In
///           values:
///           - web
///           - api
///       topologyKey: kubernetes.io/hostname
/// #}
/// ```
pub fn k8s_pod_affinity_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let key: String = kwargs.get("key")?;
    let operator: String = kwargs.get("operator")?;
    let values: Option<Vec<String>> = kwargs.get("values").ok();
    let topology_key: String = kwargs
        .get("topology_key")
        .unwrap_or_else(|_| "kubernetes.io/hostname".to_string());
    let affinity_type: String = kwargs
        .get("type")
        .unwrap_or_else(|_| "preferred".to_string());
    let weight: i64 = kwargs.get("weight").unwrap_or(100);

    // Validate operator
    let valid_operators = ["In", "NotIn", "Exists", "DoesNotExist"];
    if !valid_operators.contains(&operator.as_str()) {
        return Err(Error::new(
            ErrorKind::InvalidOperation,
            format!(
                "Invalid operator '{}', must be one of: {:?}",
                operator, valid_operators
            ),
        ));
    }

    // Validate values are provided for In/NotIn
    if (operator == "In" || operator == "NotIn") && values.is_none() {
        return Err(Error::new(
            ErrorKind::InvalidOperation,
            format!("values are required for operator '{}'", operator),
        ));
    }

    // Build the match expression
    let mut match_expr = format!("        - key: {}\n          operator: {}", key, operator);

    if let Some(vals) = values {
        match_expr.push_str("\n          values:");
        for v in vals {
            match_expr.push_str(&format!("\n          - {}", v));
        }
    }

    let output = if affinity_type == "required" {
        format!(
            "podAffinity:\n  requiredDuringSchedulingIgnoredDuringExecution:\n  - labelSelector:\n      matchExpressions:\n{}\n    topologyKey: {}",
            match_expr, topology_key
        )
    } else {
        format!(
            "podAffinity:\n  preferredDuringSchedulingIgnoredDuringExecution:\n  - weight: {}\n    podAffinityTerm:\n      labelSelector:\n        matchExpressions:\n{}\n      topologyKey: {}",
            weight, match_expr, topology_key
        )
    };

    Ok(Value::from(output))
}

/// Generate Kubernetes toleration YAML
///
/// # Arguments
///
/// * `key` (optional) - Taint key to match (omit for wildcard)
/// * `operator` (optional) - Operator: "Equal" or "Exists" (default: "Equal")
/// * `value` (optional) - Taint value to match (required if operator is "Equal")
/// * `effect` (optional) - Effect: "NoSchedule", "PreferNoSchedule", or "NoExecute"
/// * `toleration_seconds` (optional) - Seconds to tolerate (only for NoExecute)
///
/// # Returns
///
/// Returns YAML-formatted toleration configuration
///
/// # Example
///
/// ```jinja
/// {{ k8s_toleration(key="dedicated", operator="Equal", value="gpu", effect="NoSchedule") }}
/// {# Output:
/// - key: dedicated
///   operator: Equal
///   value: gpu
///   effect: NoSchedule
/// #}
///
/// {{ k8s_toleration(key="node.kubernetes.io/not-ready", operator="Exists", effect="NoExecute", toleration_seconds=300) }}
/// {# Output:
/// - key: node.kubernetes.io/not-ready
///   operator: Exists
///   effect: NoExecute
///   tolerationSeconds: 300
/// #}
/// ```
pub fn k8s_toleration_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let key: Option<String> = kwargs.get("key").ok();
    let operator: String = kwargs
        .get("operator")
        .unwrap_or_else(|_| "Equal".to_string());
    let value: Option<String> = kwargs.get("value").ok();
    let effect: Option<String> = kwargs.get("effect").ok();
    let toleration_seconds: Option<i64> = kwargs.get("toleration_seconds").ok();

    // Validate operator
    if operator != "Equal" && operator != "Exists" {
        return Err(Error::new(
            ErrorKind::InvalidOperation,
            format!(
                "Invalid operator '{}', must be 'Equal' or 'Exists'",
                operator
            ),
        ));
    }

    // Build toleration YAML
    let mut output = String::from("-");

    if let Some(k) = &key {
        output.push_str(&format!(" key: {}\n ", k));
    }

    output.push_str(&format!(" operator: {}", operator));

    if let (true, Some(v)) = (operator == "Equal", &value) {
        output.push_str(&format!("\n  value: {}", v));
    }

    if let Some(e) = &effect {
        // Validate effect
        let valid_effects = ["NoSchedule", "PreferNoSchedule", "NoExecute"];
        if !valid_effects.contains(&e.as_str()) {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                format!(
                    "Invalid effect '{}', must be one of: {:?}",
                    e, valid_effects
                ),
            ));
        }
        output.push_str(&format!("\n  effect: {}", e));
    }

    if let Some(secs) = toleration_seconds {
        output.push_str(&format!("\n  tolerationSeconds: {}", secs));
    }

    Ok(Value::from(output))
}

/// Generate Kubernetes liveness/readiness probe YAML
///
/// # Arguments
///
/// * `type` (required) - Probe type: "http", "tcp", or "exec"
/// * `path` (optional) - HTTP path (required for http type)
/// * `port` (required for http/tcp) - Port number
/// * `command` (optional) - Command array (required for exec type)
/// * `initial_delay` (optional) - Initial delay in seconds (default: 0)
/// * `period` (optional) - Period in seconds (default: 10)
/// * `timeout` (optional) - Timeout in seconds (default: 1)
/// * `success_threshold` (optional) - Success threshold (default: 1)
/// * `failure_threshold` (optional) - Failure threshold (default: 3)
///
/// # Returns
///
/// Returns YAML-formatted probe configuration
///
/// # Example
///
/// ```jinja
/// {{ k8s_probe(type="http", path="/healthz", port=8080, initial_delay=10, period=30) }}
/// {# Output:
/// httpGet:
///   path: /healthz
///   port: 8080
/// initialDelaySeconds: 10
/// periodSeconds: 30
/// timeoutSeconds: 1
/// successThreshold: 1
/// failureThreshold: 3
/// #}
///
/// {{ k8s_probe(type="tcp", port=3306) }}
/// {# Output:
/// tcpSocket:
///   port: 3306
/// initialDelaySeconds: 0
/// periodSeconds: 10
/// ...
/// #}
///
/// {{ k8s_probe(type="exec", command=["cat", "/tmp/healthy"]) }}
/// {# Output:
/// exec:
///   command:
///   - cat
///   - /tmp/healthy
/// ...
/// #}
/// ```
pub fn k8s_probe_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let probe_type: String = kwargs.get("type")?;
    let path: Option<String> = kwargs.get("path").ok();
    let port: Option<i64> = kwargs.get("port").ok();
    let command: Option<Vec<String>> = kwargs.get("command").ok();
    let initial_delay: i64 = kwargs.get("initial_delay").unwrap_or(0);
    let period: i64 = kwargs.get("period").unwrap_or(10);
    let timeout: i64 = kwargs.get("timeout").unwrap_or(1);
    let success_threshold: i64 = kwargs.get("success_threshold").unwrap_or(1);
    let failure_threshold: i64 = kwargs.get("failure_threshold").unwrap_or(3);

    let probe_config = match probe_type.as_str() {
        "http" => {
            let path = path.ok_or_else(|| {
                Error::new(
                    ErrorKind::InvalidOperation,
                    "path is required for http probe",
                )
            })?;
            let port = port.ok_or_else(|| {
                Error::new(
                    ErrorKind::InvalidOperation,
                    "port is required for http probe",
                )
            })?;
            format!("httpGet:\n  path: {}\n  port: {}", path, port)
        }
        "tcp" => {
            let port = port.ok_or_else(|| {
                Error::new(
                    ErrorKind::InvalidOperation,
                    "port is required for tcp probe",
                )
            })?;
            format!("tcpSocket:\n  port: {}", port)
        }
        "exec" => {
            let cmd = command.ok_or_else(|| {
                Error::new(
                    ErrorKind::InvalidOperation,
                    "command is required for exec probe",
                )
            })?;
            let mut cmd_yaml = String::from("exec:\n  command:");
            for c in cmd {
                cmd_yaml.push_str(&format!("\n  - {}", c));
            }
            cmd_yaml
        }
        _ => {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                format!(
                    "Invalid probe type '{}', must be 'http', 'tcp', or 'exec'",
                    probe_type
                ),
            ));
        }
    };

    let output = format!(
        "{}\ninitialDelaySeconds: {}\nperiodSeconds: {}\ntimeoutSeconds: {}\nsuccessThreshold: {}\nfailureThreshold: {}",
        probe_config, initial_delay, period, timeout, success_threshold, failure_threshold
    );

    Ok(Value::from(output))
}
