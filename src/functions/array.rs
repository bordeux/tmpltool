//! Array manipulation functions for MiniJinja templates
//!
//! This module provides utility functions for working with arrays:
//! - Counting elements
//! - Chunking arrays into groups
//! - Zipping arrays together
//! - Taking/dropping elements
//! - Finding elements
//! - Filtering by conditions
//! - Set operations (intersection, difference, union)
//!
//! Note: array_unique, array_flatten are now in filter_functions/array.rs
//! with dual function+filter syntax support.

use super::metadata::{ArgumentMetadata, FunctionMetadata, SyntaxVariants};
use super::traits::Function;
use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};
use std::collections::HashSet;

/// Count array items (alias for length)
pub struct ArrayCount;

impl Function for ArrayCount {
    const NAME: &'static str = "array_count";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "array_count",
        category: "array",
        description: "Count the number of items in an array",
        arguments: &[ArgumentMetadata {
            name: "array",
            arg_type: "array",
            required: true,
            default: None,
            description: "Array to count",
        }],
        return_type: "integer",
        examples: &[
            "{{ array_count(array=[1, 2, 3]) }}",
            "{{ array_count(array=items) }}",
        ],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let array: Value = kwargs.get("array")?;

        if !matches!(array.kind(), minijinja::value::ValueKind::Seq) {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                "array_count requires an array",
            ));
        }

        let mut count = 0;
        if let Ok(seq) = array.try_iter() {
            count = seq.count();
        }

        Ok(Value::from(count))
    }
}

/// Split array into chunks of specified size
pub struct ArrayChunk;

impl Function for ArrayChunk {
    const NAME: &'static str = "array_chunk";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "array_chunk",
        category: "array",
        description: "Split array into chunks of specified size",
        arguments: &[
            ArgumentMetadata {
                name: "array",
                arg_type: "array",
                required: true,
                default: None,
                description: "Array to chunk",
            },
            ArgumentMetadata {
                name: "size",
                arg_type: "integer",
                required: true,
                default: None,
                description: "Size of each chunk",
            },
        ],
        return_type: "array",
        examples: &["{{ array_chunk(array=[1, 2, 3, 4, 5], size=2) }}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let array: Value = kwargs.get("array")?;
        let size: usize = kwargs.get("size")?;

        if !matches!(array.kind(), minijinja::value::ValueKind::Seq) {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                "array_chunk requires an array",
            ));
        }

        if size == 0 {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                "array_chunk size must be greater than 0",
            ));
        }

        let mut chunks: Vec<Vec<Value>> = Vec::new();
        let mut current_chunk: Vec<Value> = Vec::new();

        if let Ok(seq) = array.try_iter() {
            for item in seq {
                current_chunk.push(item);
                if current_chunk.len() == size {
                    chunks.push(current_chunk.clone());
                    current_chunk.clear();
                }
            }
        }

        // Add remaining items as last chunk
        if !current_chunk.is_empty() {
            chunks.push(current_chunk);
        }

        Ok(Value::from_serialize(&chunks))
    }
}

/// Combine two arrays into pairs (zip)
pub struct ArrayZip;

impl Function for ArrayZip {
    const NAME: &'static str = "array_zip";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "array_zip",
        category: "array",
        description: "Combine two arrays into pairs",
        arguments: &[
            ArgumentMetadata {
                name: "array1",
                arg_type: "array",
                required: true,
                default: None,
                description: "First array",
            },
            ArgumentMetadata {
                name: "array2",
                arg_type: "array",
                required: true,
                default: None,
                description: "Second array",
            },
        ],
        return_type: "array",
        examples: &["{{ array_zip(array1=[\"a\", \"b\"], array2=[1, 2]) }}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let array1: Value = kwargs.get("array1")?;
        let array2: Value = kwargs.get("array2")?;

        if !matches!(array1.kind(), minijinja::value::ValueKind::Seq) {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                "array_zip requires array1 to be an array",
            ));
        }

        if !matches!(array2.kind(), minijinja::value::ValueKind::Seq) {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                "array_zip requires array2 to be an array",
            ));
        }

        let mut pairs: Vec<Vec<Value>> = Vec::new();

        if let (Ok(seq1), Ok(seq2)) = (array1.try_iter(), array2.try_iter()) {
            let vec1: Vec<Value> = seq1.collect();
            let vec2: Vec<Value> = seq2.collect();

            let min_len = vec1.len().min(vec2.len());

            for i in 0..min_len {
                pairs.push(vec![vec1[i].clone(), vec2[i].clone()]);
            }
        }

        Ok(Value::from_serialize(&pairs))
    }
}

/// Sort array by object key
pub struct ArraySortBy;

impl Function for ArraySortBy {
    const NAME: &'static str = "array_sort_by";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "array_sort_by",
        category: "array",
        description: "Sort array of objects by a key",
        arguments: &[
            ArgumentMetadata {
                name: "array",
                arg_type: "array",
                required: true,
                default: None,
                description: "Array of objects to sort",
            },
            ArgumentMetadata {
                name: "key",
                arg_type: "string",
                required: true,
                default: None,
                description: "Key name to sort by",
            },
        ],
        return_type: "array",
        examples: &["{{ array_sort_by(array=users, key=\"age\") }}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let array: Value = kwargs.get("array")?;
        let key: String = kwargs.get("key")?;

        if !matches!(array.kind(), minijinja::value::ValueKind::Seq) {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                "array_sort_by requires an array",
            ));
        }

        // Convert to serde_json::Value for easier manipulation
        let mut json_array: Vec<serde_json::Value> = Vec::new();

        if let Ok(seq) = array.try_iter() {
            for item in seq {
                let json_value: serde_json::Value = serde_json::to_value(&item).map_err(|e| {
                    Error::new(
                        ErrorKind::InvalidOperation,
                        format!("Failed to convert item: {}", e),
                    )
                })?;
                json_array.push(json_value);
            }
        }

        // Sort by key
        json_array.sort_by(|a, b| {
            let a_val = a.get(&key);
            let b_val = b.get(&key);

            match (a_val, b_val) {
                (Some(av), Some(bv)) => {
                    // Compare based on type
                    if let (Some(a_num), Some(b_num)) = (av.as_f64(), bv.as_f64()) {
                        a_num
                            .partial_cmp(&b_num)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    } else if let (Some(a_str), Some(b_str)) = (av.as_str(), bv.as_str()) {
                        a_str.cmp(b_str)
                    } else {
                        std::cmp::Ordering::Equal
                    }
                }
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => std::cmp::Ordering::Equal,
            }
        });

        Ok(Value::from_serialize(&json_array))
    }
}

/// Group array items by key
pub struct ArrayGroupBy;

impl Function for ArrayGroupBy {
    const NAME: &'static str = "array_group_by";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "array_group_by",
        category: "array",
        description: "Group array items by a key",
        arguments: &[
            ArgumentMetadata {
                name: "array",
                arg_type: "array",
                required: true,
                default: None,
                description: "Array of objects to group",
            },
            ArgumentMetadata {
                name: "key",
                arg_type: "string",
                required: true,
                default: None,
                description: "Key name to group by",
            },
        ],
        return_type: "object",
        examples: &["{{ array_group_by(array=users, key=\"dept\") }}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let array: Value = kwargs.get("array")?;
        let key: String = kwargs.get("key")?;

        if !matches!(array.kind(), minijinja::value::ValueKind::Seq) {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                "array_group_by requires an array",
            ));
        }

        use std::collections::HashMap;
        let mut groups: HashMap<String, Vec<serde_json::Value>> = HashMap::new();

        if let Ok(seq) = array.try_iter() {
            for item in seq {
                let json_value: serde_json::Value = serde_json::to_value(&item).map_err(|e| {
                    Error::new(
                        ErrorKind::InvalidOperation,
                        format!("Failed to convert item: {}", e),
                    )
                })?;

                // Get the key value as string
                if let Some(obj) = json_value.as_object()
                    && let Some(key_val) = obj.get(&key)
                {
                    let group_key = match key_val {
                        serde_json::Value::String(s) => s.clone(),
                        serde_json::Value::Number(n) => n.to_string(),
                        serde_json::Value::Bool(b) => b.to_string(),
                        _ => "null".to_string(),
                    };

                    groups.entry(group_key).or_default().push(json_value);
                }
            }
        }

        Ok(Value::from_serialize(&groups))
    }
}

/// Take first N elements from array
pub struct ArrayTake;

impl Function for ArrayTake {
    const NAME: &'static str = "array_take";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "array_take",
        category: "array",
        description: "Take first N elements from array",
        arguments: &[
            ArgumentMetadata {
                name: "array",
                arg_type: "array",
                required: true,
                default: None,
                description: "Source array",
            },
            ArgumentMetadata {
                name: "n",
                arg_type: "integer",
                required: true,
                default: None,
                description: "Number of elements to take",
            },
        ],
        return_type: "array",
        examples: &["{{ array_take(array=[1, 2, 3, 4, 5], n=3) }}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let array: Value = kwargs.get("array")?;
        let n: usize = kwargs.get("n")?;

        if !matches!(array.kind(), minijinja::value::ValueKind::Seq) {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                "array_take requires an array",
            ));
        }

        let mut result: Vec<serde_json::Value> = Vec::new();

        if let Ok(seq) = array.try_iter() {
            for (i, item) in seq.enumerate() {
                if i >= n {
                    break;
                }
                let json_value: serde_json::Value = serde_json::to_value(&item).map_err(|e| {
                    Error::new(
                        ErrorKind::InvalidOperation,
                        format!("Failed to convert item: {}", e),
                    )
                })?;
                result.push(json_value);
            }
        }

        Ok(Value::from_serialize(&result))
    }
}

/// Skip first N elements from array
pub struct ArrayDrop;

impl Function for ArrayDrop {
    const NAME: &'static str = "array_drop";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "array_drop",
        category: "array",
        description: "Skip first N elements from array",
        arguments: &[
            ArgumentMetadata {
                name: "array",
                arg_type: "array",
                required: true,
                default: None,
                description: "Source array",
            },
            ArgumentMetadata {
                name: "n",
                arg_type: "integer",
                required: true,
                default: None,
                description: "Number of elements to skip",
            },
        ],
        return_type: "array",
        examples: &["{{ array_drop(array=[1, 2, 3, 4, 5], n=2) }}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let array: Value = kwargs.get("array")?;
        let n: usize = kwargs.get("n")?;

        if !matches!(array.kind(), minijinja::value::ValueKind::Seq) {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                "array_drop requires an array",
            ));
        }

        let mut result: Vec<serde_json::Value> = Vec::new();

        if let Ok(seq) = array.try_iter() {
            for (i, item) in seq.enumerate() {
                if i < n {
                    continue;
                }
                let json_value: serde_json::Value = serde_json::to_value(&item).map_err(|e| {
                    Error::new(
                        ErrorKind::InvalidOperation,
                        format!("Failed to convert item: {}", e),
                    )
                })?;
                result.push(json_value);
            }
        }

        Ok(Value::from_serialize(&result))
    }
}

/// Find index of element in array
pub struct ArrayIndexOf;

impl Function for ArrayIndexOf {
    const NAME: &'static str = "array_index_of";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "array_index_of",
        category: "array",
        description: "Find index of element in array (0-based, -1 if not found)",
        arguments: &[
            ArgumentMetadata {
                name: "array",
                arg_type: "array",
                required: true,
                default: None,
                description: "Array to search",
            },
            ArgumentMetadata {
                name: "value",
                arg_type: "any",
                required: true,
                default: None,
                description: "Value to find",
            },
        ],
        return_type: "integer",
        examples: &["{{ array_index_of(array=[\"a\", \"b\", \"c\"], value=\"b\") }}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let array: Value = kwargs.get("array")?;
        let value: Value = kwargs.get("value")?;

        if !matches!(array.kind(), minijinja::value::ValueKind::Seq) {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                "array_index_of requires an array",
            ));
        }

        let search_value: serde_json::Value = serde_json::to_value(&value).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Failed to convert value: {}", e),
            )
        })?;

        if let Ok(seq) = array.try_iter() {
            for (i, item) in seq.enumerate() {
                let item_value: serde_json::Value = serde_json::to_value(&item).map_err(|e| {
                    Error::new(
                        ErrorKind::InvalidOperation,
                        format!("Failed to convert item: {}", e),
                    )
                })?;

                if item_value == search_value {
                    return Ok(Value::from(i as i64));
                }
            }
        }

        Ok(Value::from(-1_i64))
    }
}

/// Find first matching object in array
pub struct ArrayFind;

impl Function for ArrayFind {
    const NAME: &'static str = "array_find";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "array_find",
        category: "array",
        description: "Find first matching object in array",
        arguments: &[
            ArgumentMetadata {
                name: "array",
                arg_type: "array",
                required: true,
                default: None,
                description: "Array of objects to search",
            },
            ArgumentMetadata {
                name: "key",
                arg_type: "string",
                required: true,
                default: None,
                description: "Key to match",
            },
            ArgumentMetadata {
                name: "value",
                arg_type: "any",
                required: true,
                default: None,
                description: "Value to match",
            },
        ],
        return_type: "object",
        examples: &["{{ array_find(array=users, key=\"id\", value=2) }}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let array: Value = kwargs.get("array")?;
        let key: String = kwargs.get("key")?;
        let value: Value = kwargs.get("value")?;

        if !matches!(array.kind(), minijinja::value::ValueKind::Seq) {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                "array_find requires an array",
            ));
        }

        let search_value: serde_json::Value = serde_json::to_value(&value).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Failed to convert value: {}", e),
            )
        })?;

        if let Ok(seq) = array.try_iter() {
            for item in seq {
                let json_value: serde_json::Value = serde_json::to_value(&item).map_err(|e| {
                    Error::new(
                        ErrorKind::InvalidOperation,
                        format!("Failed to convert item: {}", e),
                    )
                })?;

                if let Some(obj) = json_value.as_object()
                    && let Some(item_val) = obj.get(&key)
                    && *item_val == search_value
                {
                    return Ok(Value::from_serialize(&json_value));
                }
            }
        }

        Ok(Value::from(()))
    }
}

/// Filter array by key with comparison operator
pub struct ArrayFilterBy;

impl Function for ArrayFilterBy {
    const NAME: &'static str = "array_filter_by";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "array_filter_by",
        category: "array",
        description: "Filter array by key with comparison operator",
        arguments: &[
            ArgumentMetadata {
                name: "array",
                arg_type: "array",
                required: true,
                default: None,
                description: "Array of objects to filter",
            },
            ArgumentMetadata {
                name: "key",
                arg_type: "string",
                required: true,
                default: None,
                description: "Key to compare",
            },
            ArgumentMetadata {
                name: "op",
                arg_type: "string",
                required: true,
                default: None,
                description: "Operator: eq, ne, gt, lt, gte, lte, contains",
            },
            ArgumentMetadata {
                name: "value",
                arg_type: "any",
                required: true,
                default: None,
                description: "Value to compare against",
            },
        ],
        return_type: "array",
        examples: &["{{ array_filter_by(array=items, key=\"price\", op=\"gt\", value=15) }}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let array: Value = kwargs.get("array")?;
        let key: String = kwargs.get("key")?;
        let op: String = kwargs.get("op")?;
        let value: Value = kwargs.get("value")?;

        if !matches!(array.kind(), minijinja::value::ValueKind::Seq) {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                "array_filter_by requires an array",
            ));
        }

        let compare_value: serde_json::Value = serde_json::to_value(&value).map_err(|e| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("Failed to convert value: {}", e),
            )
        })?;

        let mut result: Vec<serde_json::Value> = Vec::new();

        if let Ok(seq) = array.try_iter() {
            for item in seq {
                let json_value: serde_json::Value = serde_json::to_value(&item).map_err(|e| {
                    Error::new(
                        ErrorKind::InvalidOperation,
                        format!("Failed to convert item: {}", e),
                    )
                })?;

                if let Some(obj) = json_value.as_object()
                    && let Some(item_val) = obj.get(&key)
                {
                    let matches = match op.as_str() {
                        "eq" => *item_val == compare_value,
                        "ne" => *item_val != compare_value,
                        "gt" => compare_numeric(item_val, &compare_value, |a, b| a > b),
                        "lt" => compare_numeric(item_val, &compare_value, |a, b| a < b),
                        "gte" => compare_numeric(item_val, &compare_value, |a, b| a >= b),
                        "lte" => compare_numeric(item_val, &compare_value, |a, b| a <= b),
                        "contains" => {
                            if let (Some(s1), Some(s2)) =
                                (item_val.as_str(), compare_value.as_str())
                            {
                                s1.contains(s2)
                            } else {
                                false
                            }
                        }
                        _ => {
                            return Err(Error::new(
                                ErrorKind::InvalidOperation,
                                format!(
                                    "Invalid operator '{}'. Use: eq, ne, gt, lt, gte, lte, contains",
                                    op
                                ),
                            ));
                        }
                    };

                    if matches {
                        result.push(json_value);
                    }
                }
            }
        }

        Ok(Value::from_serialize(&result))
    }
}

fn compare_numeric<F>(a: &serde_json::Value, b: &serde_json::Value, cmp: F) -> bool
where
    F: Fn(f64, f64) -> bool,
{
    match (a.as_f64(), b.as_f64()) {
        (Some(a_num), Some(b_num)) => cmp(a_num, b_num),
        _ => false,
    }
}

/// Extract nested key from array of objects (pluck)
pub struct ArrayPluck;

impl Function for ArrayPluck {
    const NAME: &'static str = "array_pluck";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "array_pluck",
        category: "array",
        description: "Extract nested key from array of objects",
        arguments: &[
            ArgumentMetadata {
                name: "array",
                arg_type: "array",
                required: true,
                default: None,
                description: "Array of objects",
            },
            ArgumentMetadata {
                name: "key",
                arg_type: "string",
                required: true,
                default: None,
                description: "Key path to extract (supports dot notation)",
            },
        ],
        return_type: "array",
        examples: &[
            "{{ array_pluck(array=users, key=\"name\") }}",
            "{{ array_pluck(array=data, key=\"user.name\") }}",
        ],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let array: Value = kwargs.get("array")?;
        let key: String = kwargs.get("key")?;

        if !matches!(array.kind(), minijinja::value::ValueKind::Seq) {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                "array_pluck requires an array",
            ));
        }

        let key_parts: Vec<&str> = key.split('.').collect();
        let mut result: Vec<serde_json::Value> = Vec::new();

        if let Ok(seq) = array.try_iter() {
            for item in seq {
                let json_value: serde_json::Value = serde_json::to_value(&item).map_err(|e| {
                    Error::new(
                        ErrorKind::InvalidOperation,
                        format!("Failed to convert item: {}", e),
                    )
                })?;

                // Navigate through the key path
                let mut current = &json_value;
                let mut found = true;

                for part in &key_parts {
                    if let Some(obj) = current.as_object() {
                        if let Some(val) = obj.get(*part) {
                            current = val;
                        } else {
                            found = false;
                            break;
                        }
                    } else {
                        found = false;
                        break;
                    }
                }

                if found {
                    result.push(current.clone());
                } else {
                    result.push(serde_json::Value::Null);
                }
            }
        }

        Ok(Value::from_serialize(&result))
    }
}

// ==================== Set Operations ====================

/// Get intersection of two arrays (common elements)
pub struct ArrayIntersection;

impl Function for ArrayIntersection {
    const NAME: &'static str = "array_intersection";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "array_intersection",
        category: "array",
        description: "Get intersection of two arrays (common elements)",
        arguments: &[
            ArgumentMetadata {
                name: "array1",
                arg_type: "array",
                required: true,
                default: None,
                description: "First array",
            },
            ArgumentMetadata {
                name: "array2",
                arg_type: "array",
                required: true,
                default: None,
                description: "Second array",
            },
        ],
        return_type: "array",
        examples: &["{{ array_intersection(array1=[1, 2, 3, 4], array2=[3, 4, 5, 6]) }}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let array1: Value = kwargs.get("array1")?;
        let array2: Value = kwargs.get("array2")?;

        if !matches!(array1.kind(), minijinja::value::ValueKind::Seq) {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                "array_intersection requires array1 to be an array",
            ));
        }
        if !matches!(array2.kind(), minijinja::value::ValueKind::Seq) {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                "array_intersection requires array2 to be an array",
            ));
        }

        // Convert array2 to a set for O(1) lookup
        let mut set2: HashSet<String> = HashSet::new();
        if let Ok(seq) = array2.try_iter() {
            for item in seq {
                let json_value: serde_json::Value = serde_json::to_value(&item).unwrap_or_default();
                set2.insert(serde_json::to_string(&json_value).unwrap_or_default());
            }
        }

        let mut result: Vec<serde_json::Value> = Vec::new();
        let mut seen: HashSet<String> = HashSet::new();

        if let Ok(seq) = array1.try_iter() {
            for item in seq {
                let json_value: serde_json::Value = serde_json::to_value(&item).unwrap_or_default();
                let item_str = serde_json::to_string(&json_value).unwrap_or_default();

                if set2.contains(&item_str) && seen.insert(item_str) {
                    result.push(json_value);
                }
            }
        }

        Ok(Value::from_serialize(&result))
    }
}

/// Get difference of two arrays (elements in first but not in second)
pub struct ArrayDifference;

impl Function for ArrayDifference {
    const NAME: &'static str = "array_difference";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "array_difference",
        category: "array",
        description: "Get difference of two arrays (elements in first but not in second)",
        arguments: &[
            ArgumentMetadata {
                name: "array1",
                arg_type: "array",
                required: true,
                default: None,
                description: "First array",
            },
            ArgumentMetadata {
                name: "array2",
                arg_type: "array",
                required: true,
                default: None,
                description: "Second array",
            },
        ],
        return_type: "array",
        examples: &["{{ array_difference(array1=[1, 2, 3, 4], array2=[3, 4, 5, 6]) }}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let array1: Value = kwargs.get("array1")?;
        let array2: Value = kwargs.get("array2")?;

        if !matches!(array1.kind(), minijinja::value::ValueKind::Seq) {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                "array_difference requires array1 to be an array",
            ));
        }
        if !matches!(array2.kind(), minijinja::value::ValueKind::Seq) {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                "array_difference requires array2 to be an array",
            ));
        }

        // Convert array2 to a set for O(1) lookup
        let mut set2: HashSet<String> = HashSet::new();
        if let Ok(seq) = array2.try_iter() {
            for item in seq {
                let json_value: serde_json::Value = serde_json::to_value(&item).unwrap_or_default();
                set2.insert(serde_json::to_string(&json_value).unwrap_or_default());
            }
        }

        let mut result: Vec<serde_json::Value> = Vec::new();
        let mut seen: HashSet<String> = HashSet::new();

        if let Ok(seq) = array1.try_iter() {
            for item in seq {
                let json_value: serde_json::Value = serde_json::to_value(&item).unwrap_or_default();
                let item_str = serde_json::to_string(&json_value).unwrap_or_default();

                if !set2.contains(&item_str) && seen.insert(item_str) {
                    result.push(json_value);
                }
            }
        }

        Ok(Value::from_serialize(&result))
    }
}

/// Get union of two arrays (all unique elements from both)
pub struct ArrayUnion;

impl Function for ArrayUnion {
    const NAME: &'static str = "array_union";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "array_union",
        category: "array",
        description: "Get union of two arrays (all unique elements from both)",
        arguments: &[
            ArgumentMetadata {
                name: "array1",
                arg_type: "array",
                required: true,
                default: None,
                description: "First array",
            },
            ArgumentMetadata {
                name: "array2",
                arg_type: "array",
                required: true,
                default: None,
                description: "Second array",
            },
        ],
        return_type: "array",
        examples: &["{{ array_union(array1=[1, 2, 3], array2=[3, 4, 5]) }}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let array1: Value = kwargs.get("array1")?;
        let array2: Value = kwargs.get("array2")?;

        if !matches!(array1.kind(), minijinja::value::ValueKind::Seq) {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                "array_union requires array1 to be an array",
            ));
        }
        if !matches!(array2.kind(), minijinja::value::ValueKind::Seq) {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                "array_union requires array2 to be an array",
            ));
        }

        let mut result: Vec<serde_json::Value> = Vec::new();
        let mut seen: HashSet<String> = HashSet::new();

        // Add all unique items from array1
        if let Ok(seq) = array1.try_iter() {
            for item in seq {
                let json_value: serde_json::Value = serde_json::to_value(&item).unwrap_or_default();
                let item_str = serde_json::to_string(&json_value).unwrap_or_default();

                if seen.insert(item_str) {
                    result.push(json_value);
                }
            }
        }

        // Add unique items from array2 that aren't in array1
        if let Ok(seq) = array2.try_iter() {
            for item in seq {
                let json_value: serde_json::Value = serde_json::to_value(&item).unwrap_or_default();
                let item_str = serde_json::to_string(&json_value).unwrap_or_default();

                if seen.insert(item_str) {
                    result.push(json_value);
                }
            }
        }

        Ok(Value::from_serialize(&result))
    }
}

/// Get symmetric difference of two arrays (elements in either but not both)
pub struct ArraySymmetricDifference;

impl Function for ArraySymmetricDifference {
    const NAME: &'static str = "array_symmetric_difference";
    const METADATA: FunctionMetadata = FunctionMetadata {
        name: "array_symmetric_difference",
        category: "array",
        description: "Get symmetric difference of two arrays (elements in either but not both)",
        arguments: &[
            ArgumentMetadata {
                name: "array1",
                arg_type: "array",
                required: true,
                default: None,
                description: "First array",
            },
            ArgumentMetadata {
                name: "array2",
                arg_type: "array",
                required: true,
                default: None,
                description: "Second array",
            },
        ],
        return_type: "array",
        examples: &["{{ array_symmetric_difference(array1=[1, 2, 3, 4], array2=[3, 4, 5, 6]) }}"],
        syntax: SyntaxVariants::FUNCTION_ONLY,
    };

    fn call(kwargs: Kwargs) -> Result<Value, Error> {
        let array1: Value = kwargs.get("array1")?;
        let array2: Value = kwargs.get("array2")?;

        if !matches!(array1.kind(), minijinja::value::ValueKind::Seq) {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                "array_symmetric_difference requires array1 to be an array",
            ));
        }
        if !matches!(array2.kind(), minijinja::value::ValueKind::Seq) {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                "array_symmetric_difference requires array2 to be an array",
            ));
        }

        // Convert both arrays to sets
        let mut set1: HashSet<String> = HashSet::new();
        let mut vec1: Vec<serde_json::Value> = Vec::new();
        if let Ok(seq) = array1.try_iter() {
            for item in seq {
                let json_value: serde_json::Value = serde_json::to_value(&item).unwrap_or_default();
                let item_str = serde_json::to_string(&json_value).unwrap_or_default();
                if set1.insert(item_str) {
                    vec1.push(json_value);
                }
            }
        }

        let mut set2: HashSet<String> = HashSet::new();
        let mut vec2: Vec<serde_json::Value> = Vec::new();
        if let Ok(seq) = array2.try_iter() {
            for item in seq {
                let json_value: serde_json::Value = serde_json::to_value(&item).unwrap_or_default();
                let item_str = serde_json::to_string(&json_value).unwrap_or_default();
                if set2.insert(item_str) {
                    vec2.push(json_value);
                }
            }
        }

        let mut result: Vec<serde_json::Value> = Vec::new();

        // Add items from array1 that are not in array2
        for item in vec1 {
            let item_str = serde_json::to_string(&item).unwrap_or_default();
            if !set2.contains(&item_str) {
                result.push(item);
            }
        }

        // Add items from array2 that are not in array1
        for item in vec2 {
            let item_str = serde_json::to_string(&item).unwrap_or_default();
            if !set1.contains(&item_str) {
                result.push(item);
            }
        }

        Ok(Value::from_serialize(&result))
    }
}
