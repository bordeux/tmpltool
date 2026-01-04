//! Kubernetes helper functions for MiniJinja templates
//!
//! This module provides Kubernetes-specific formatting and validation functions:
//! - Resource request/limit formatting
//! - ConfigMap and Secret references
//! - Quantity conversions
//! - Pod affinity and toleration generation
//! - Liveness/readiness probe generation
//!
//! Note: k8s_label_safe, k8s_dns_label_safe, k8s_annotation_safe are now in
//! filter_functions/kubernetes.rs with dual function+filter syntax support.

use super::metadata::{ArgumentMetadata, FunctionMetadata, SyntaxVariants};
use super::traits::Function;
use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};
use std::collections::HashMap;

/// Format Kubernetes resource requests
pub struct K8sResourceRequest;

impl Function for K8sResourceRequest {
    const NAME: &'static str = "k8s_resource_request";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "k8s_resource_request",
        category: "kubernetes",
        description: "Format Kubernetes resource requests",
        arguments: &[
            ArgumentMetadata {
                name: "cpu",
                arg_type: "string|number",
                required: true,
                default: None,
                description: "CPU request (e.g., \"500m\" or 0.5)",
            },
            ArgumentMetadata {
                name: "memory",
                arg_type: "string|number",
                required: true,
                default: None,
                description: "Memory request (e.g., \"512Mi\" or 512)",
            },
        ],
        return_type: "string",
        examples: &[
            "{{ k8s_resource_request(cpu=\"500m\", memory=\"512Mi\") }}",
            "{{ k8s_resource_request(cpu=0.5, memory=512) }}",
        ],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let cpu: Value = kwargs.get("cpu")?;
        let memory: Value = kwargs.get("memory")?;

        // Format CPU value
        let cpu_str = if let Some(cpu_str) = cpu.as_str() {
            cpu_str.to_string()
        } else {
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

            let millicores = (cpu_num * 1000.0).round() as i64;
            format!("{}m", millicores)
        };

        // Format memory value
        let memory_str = if let Some(memory_str) = memory.as_str() {
            memory_str.to_string()
        } else {
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

            if memory_num >= 1024.0 {
                let gib = memory_num / 1024.0;
                if gib.fract() == 0.0 {
                    format!("{}Gi", gib as i64)
                } else {
                    format!("{:.2}Gi", gib)
                }
            } else if memory_num.fract() == 0.0 {
                format!("{}Mi", memory_num as i64)
            } else {
                format!("{:.2}Mi", memory_num)
            }
        };

        let output = format!(
            "requests:\n  cpu: \"{}\"\n  memory: \"{}\"",
            cpu_str, memory_str
        );

        Ok(Value::from(output))
    }
}

/// Generate Kubernetes environment variable reference
pub struct K8sEnvVarRef;

impl Function for K8sEnvVarRef {
    const NAME: &'static str = "k8s_env_var_ref";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "k8s_env_var_ref",
        category: "kubernetes",
        description: "Generate Kubernetes environment variable reference",
        arguments: &[
            ArgumentMetadata {
                name: "var_name",
                arg_type: "string",
                required: true,
                default: None,
                description: "The environment variable name/key",
            },
            ArgumentMetadata {
                name: "source",
                arg_type: "string",
                required: false,
                default: Some("\"configmap\""),
                description: "Source type: \"configmap\" or \"secret\"",
            },
            ArgumentMetadata {
                name: "name",
                arg_type: "string",
                required: false,
                default: None,
                description: "Name of the ConfigMap/Secret",
            },
        ],
        return_type: "string",
        examples: &[
            "{{ k8s_env_var_ref(var_name=\"DATABASE_HOST\", source=\"configmap\", name=\"app-config\") }}",
        ],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
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
}

/// Generate Kubernetes Secret reference
pub struct K8sSecretRef;

impl Function for K8sSecretRef {
    const NAME: &'static str = "k8s_secret_ref";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "k8s_secret_ref",
        category: "kubernetes",
        description: "Generate Kubernetes Secret reference",
        arguments: &[
            ArgumentMetadata {
                name: "secret_name",
                arg_type: "string",
                required: true,
                default: None,
                description: "Name of the Secret",
            },
            ArgumentMetadata {
                name: "key",
                arg_type: "string",
                required: true,
                default: None,
                description: "Key within the Secret",
            },
            ArgumentMetadata {
                name: "optional",
                arg_type: "boolean",
                required: false,
                default: Some("false"),
                description: "Whether the Secret is optional",
            },
        ],
        return_type: "string",
        examples: &["{{ k8s_secret_ref(secret_name=\"db-credentials\", key=\"password\") }}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
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
}

/// Generate Kubernetes ConfigMap reference
pub struct K8sConfigmapRef;

impl Function for K8sConfigmapRef {
    const NAME: &'static str = "k8s_configmap_ref";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "k8s_configmap_ref",
        category: "kubernetes",
        description: "Generate Kubernetes ConfigMap reference",
        arguments: &[
            ArgumentMetadata {
                name: "configmap_name",
                arg_type: "string",
                required: true,
                default: None,
                description: "Name of the ConfigMap",
            },
            ArgumentMetadata {
                name: "key",
                arg_type: "string",
                required: true,
                default: None,
                description: "Key within the ConfigMap",
            },
            ArgumentMetadata {
                name: "optional",
                arg_type: "boolean",
                required: false,
                default: Some("false"),
                description: "Whether the ConfigMap is optional",
            },
        ],
        return_type: "string",
        examples: &[
            "{{ k8s_configmap_ref(configmap_name=\"app-config\", key=\"database_host\") }}",
        ],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
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
}

/// Helm-style template function
pub struct HelmTpl;

impl Function for HelmTpl {
    const NAME: &'static str = "helm_tpl";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "helm_tpl",
        category: "kubernetes",
        description: "Helm-style template function with {{ .key }} placeholders",
        arguments: &[
            ArgumentMetadata {
                name: "template",
                arg_type: "string",
                required: true,
                default: None,
                description: "Template string with {{ .key }} placeholders",
            },
            ArgumentMetadata {
                name: "values",
                arg_type: "object",
                required: true,
                default: None,
                description: "Object with values to substitute",
            },
        ],
        return_type: "string",
        examples: &["{{ helm_tpl(template=\"Hello {{ .name }}\", values={\"name\": \"World\"}) }}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let template: String = kwargs.get("template")?;
        let values: Value = kwargs.get("values")?;

        let json_values: serde_json::Value = serde_json::to_value(&values).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Failed to convert values: {}", e),
            )
        })?;

        let mut result = template.clone();

        let re = regex::Regex::new(r"\{\{\s*\.([a-zA-Z0-9_.]+)\s*\}\}").unwrap();

        let matches: Vec<(String, String)> = re
            .captures_iter(&template)
            .map(|cap| {
                let full_match = cap.get(0).unwrap().as_str().to_string();
                let path = cap.get(1).unwrap().as_str().to_string();
                (full_match, path)
            })
            .collect();

        for (placeholder, path) in matches {
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
}

/// Convert Kubernetes quantity string to bytes
pub struct K8sQuantityToBytes;

impl Function for K8sQuantityToBytes {
    const NAME: &'static str = "k8s_quantity_to_bytes";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "k8s_quantity_to_bytes",
        category: "kubernetes",
        description: "Convert Kubernetes quantity string to bytes",
        arguments: &[ArgumentMetadata {
            name: "quantity",
            arg_type: "string",
            required: true,
            default: None,
            description: "Kubernetes quantity string (e.g., \"1Gi\", \"500Mi\")",
        }],
        return_type: "integer",
        examples: &[
            "{{ k8s_quantity_to_bytes(quantity=\"1Gi\") }}",
            "{{ k8s_quantity_to_bytes(quantity=\"500Mi\") }}",
        ],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let quantity: String = kwargs.get("quantity")?;

        let (num_str, suffix) = parse_quantity_string(&quantity)?;

        let num: f64 = num_str.parse().map_err(|_| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Invalid number in quantity: '{}'", quantity),
            )
        })?;

        if suffix == "m" {
            let millicores = (num * 1.0) as i64;
            return Ok(Value::from(millicores));
        }

        let multiplier: u64 = match suffix.as_str() {
            "" => 1,
            "Ki" => 1024,
            "Mi" => 1024 * 1024,
            "Gi" => 1024 * 1024 * 1024,
            "Ti" => 1024_u64 * 1024 * 1024 * 1024,
            "Pi" => 1024_u64 * 1024 * 1024 * 1024 * 1024,
            "Ei" => 1024_u64 * 1024 * 1024 * 1024 * 1024 * 1024,
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
}

/// Convert bytes to Kubernetes quantity string
pub struct K8sBytesToQuantity;

impl Function for K8sBytesToQuantity {
    const NAME: &'static str = "k8s_bytes_to_quantity";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "k8s_bytes_to_quantity",
        category: "kubernetes",
        description: "Convert bytes to Kubernetes quantity string",
        arguments: &[
            ArgumentMetadata {
                name: "bytes",
                arg_type: "integer",
                required: true,
                default: None,
                description: "Number of bytes",
            },
            ArgumentMetadata {
                name: "unit",
                arg_type: "string",
                required: false,
                default: None,
                description: "Force a specific unit (e.g., \"Mi\", \"Gi\")",
            },
            ArgumentMetadata {
                name: "binary",
                arg_type: "boolean",
                required: false,
                default: Some("true"),
                description: "Use binary prefixes (Ki, Mi, Gi) if true",
            },
        ],
        return_type: "string",
        examples: &[
            "{{ k8s_bytes_to_quantity(bytes=1073741824) }}",
            "{{ k8s_bytes_to_quantity(bytes=1073741824, unit=\"Mi\") }}",
        ],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
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
        } else if bytes >= 1000_u64 * 1000 * 1000 * 1000 {
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
        };

        Ok(Value::from(result))
    }
}

/// Helper function to parse quantity string into number and suffix
fn parse_quantity_string(quantity: &str) -> Result<(String, String), Error> {
    let quantity = quantity.trim();

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
pub struct K8sSelector;

impl Function for K8sSelector {
    const NAME: &'static str = "k8s_selector";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "k8s_selector",
        category: "kubernetes",
        description: "Generate Kubernetes label selector string",
        arguments: &[ArgumentMetadata {
            name: "labels",
            arg_type: "object",
            required: true,
            default: None,
            description: "Object with label key-value pairs",
        }],
        return_type: "string",
        examples: &["{{ k8s_selector(labels={\"app\": \"nginx\", \"env\": \"prod\"}) }}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
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

        selectors.sort();

        Ok(Value::from(selectors.join(",")))
    }
}

/// Generate Kubernetes pod affinity YAML
pub struct K8sPodAffinity;

impl Function for K8sPodAffinity {
    const NAME: &'static str = "k8s_pod_affinity";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "k8s_pod_affinity",
        category: "kubernetes",
        description: "Generate Kubernetes pod affinity YAML",
        arguments: &[
            ArgumentMetadata {
                name: "key",
                arg_type: "string",
                required: true,
                default: None,
                description: "Label key for the affinity rule",
            },
            ArgumentMetadata {
                name: "operator",
                arg_type: "string",
                required: true,
                default: None,
                description: "Operator: In, NotIn, Exists, DoesNotExist",
            },
            ArgumentMetadata {
                name: "values",
                arg_type: "array",
                required: false,
                default: None,
                description: "Array of values (required for In/NotIn)",
            },
            ArgumentMetadata {
                name: "topology_key",
                arg_type: "string",
                required: false,
                default: Some("\"kubernetes.io/hostname\""),
                description: "Topology key",
            },
            ArgumentMetadata {
                name: "type",
                arg_type: "string",
                required: false,
                default: Some("\"preferred\""),
                description: "Affinity type: required or preferred",
            },
            ArgumentMetadata {
                name: "weight",
                arg_type: "integer",
                required: false,
                default: Some("100"),
                description: "Weight for preferred affinity",
            },
        ],
        return_type: "string",
        examples: &[
            "{{ k8s_pod_affinity(key=\"app\", operator=\"In\", values=[\"web\", \"api\"]) }}",
        ],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
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

        if (operator == "In" || operator == "NotIn") && values.is_none() {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                format!("values are required for operator '{}'", operator),
            ));
        }

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
}

/// Generate Kubernetes toleration YAML
pub struct K8sToleration;

impl Function for K8sToleration {
    const NAME: &'static str = "k8s_toleration";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "k8s_toleration",
        category: "kubernetes",
        description: "Generate Kubernetes toleration YAML",
        arguments: &[
            ArgumentMetadata {
                name: "key",
                arg_type: "string",
                required: false,
                default: None,
                description: "Taint key to match (omit for wildcard)",
            },
            ArgumentMetadata {
                name: "operator",
                arg_type: "string",
                required: false,
                default: Some("\"Equal\""),
                description: "Operator: Equal or Exists",
            },
            ArgumentMetadata {
                name: "value",
                arg_type: "string",
                required: false,
                default: None,
                description: "Taint value to match",
            },
            ArgumentMetadata {
                name: "effect",
                arg_type: "string",
                required: false,
                default: None,
                description: "Effect: NoSchedule, PreferNoSchedule, or NoExecute",
            },
            ArgumentMetadata {
                name: "toleration_seconds",
                arg_type: "integer",
                required: false,
                default: None,
                description: "Seconds to tolerate (only for NoExecute)",
            },
        ],
        return_type: "string",
        examples: &[
            "{{ k8s_toleration(key=\"dedicated\", operator=\"Equal\", value=\"gpu\", effect=\"NoSchedule\") }}",
        ],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let key: Option<String> = kwargs.get("key").ok();
        let operator: String = kwargs
            .get("operator")
            .unwrap_or_else(|_| "Equal".to_string());
        let value: Option<String> = kwargs.get("value").ok();
        let effect: Option<String> = kwargs.get("effect").ok();
        let toleration_seconds: Option<i64> = kwargs.get("toleration_seconds").ok();

        if operator != "Equal" && operator != "Exists" {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                format!(
                    "Invalid operator '{}', must be 'Equal' or 'Exists'",
                    operator
                ),
            ));
        }

        let mut output = String::from("-");

        if let Some(k) = &key {
            output.push_str(&format!(" key: {}\n ", k));
        }

        output.push_str(&format!(" operator: {}", operator));

        if let (true, Some(v)) = (operator == "Equal", &value) {
            output.push_str(&format!("\n  value: {}", v));
        }

        if let Some(e) = &effect {
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
}

/// Generate Kubernetes liveness/readiness probe YAML
pub struct K8sProbe;

impl Function for K8sProbe {
    const NAME: &'static str = "k8s_probe";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "k8s_probe",
        category: "kubernetes",
        description: "Generate Kubernetes liveness/readiness probe YAML",
        arguments: &[
            ArgumentMetadata {
                name: "type",
                arg_type: "string",
                required: true,
                default: None,
                description: "Probe type: http, tcp, or exec",
            },
            ArgumentMetadata {
                name: "path",
                arg_type: "string",
                required: false,
                default: None,
                description: "HTTP path (required for http type)",
            },
            ArgumentMetadata {
                name: "port",
                arg_type: "integer",
                required: false,
                default: None,
                description: "Port number (required for http/tcp)",
            },
            ArgumentMetadata {
                name: "command",
                arg_type: "array",
                required: false,
                default: None,
                description: "Command array (required for exec type)",
            },
            ArgumentMetadata {
                name: "initial_delay",
                arg_type: "integer",
                required: false,
                default: Some("0"),
                description: "Initial delay in seconds",
            },
            ArgumentMetadata {
                name: "period",
                arg_type: "integer",
                required: false,
                default: Some("10"),
                description: "Period in seconds",
            },
            ArgumentMetadata {
                name: "timeout",
                arg_type: "integer",
                required: false,
                default: Some("1"),
                description: "Timeout in seconds",
            },
            ArgumentMetadata {
                name: "success_threshold",
                arg_type: "integer",
                required: false,
                default: Some("1"),
                description: "Success threshold",
            },
            ArgumentMetadata {
                name: "failure_threshold",
                arg_type: "integer",
                required: false,
                default: Some("3"),
                description: "Failure threshold",
            },
        ],
        return_type: "string",
        examples: &[
            "{{ k8s_probe(type=\"http\", path=\"/healthz\", port=8080) }}",
            "{{ k8s_probe(type=\"tcp\", port=3306) }}",
            "{{ k8s_probe(type=\"exec\", command=[\"cat\", \"/tmp/healthy\"]) }}",
        ],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
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
}
