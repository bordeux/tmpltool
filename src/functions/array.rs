//! Array manipulation functions for MiniJinja templates
//!
//! This module provides utility functions for working with arrays:
//! - Counting elements
//! - Chunking arrays into groups
//! - Zipping arrays together

use minijinja::value::Kwargs;
use minijinja::{Error, ErrorKind, Value};

/// Count array items (alias for length)
///
/// # Arguments
///
/// * `array` (required) - Array to count
///
/// # Returns
///
/// Returns the number of items in the array
///
/// # Example
///
/// ```jinja
/// {# Count array items #}
/// {% set items = ["apple", "banana", "cherry"] %}
/// {{ array_count(array=items) }}
/// {# Output: 3 #}
///
/// {# Empty array #}
/// {% set empty = [] %}
/// {{ array_count(array=empty) }}
/// {# Output: 0 #}
/// ```
pub fn array_count_fn(kwargs: Kwargs) -> Result<Value, Error> {
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

/// Split array into chunks of specified size
///
/// # Arguments
///
/// * `array` (required) - Array to chunk
/// * `size` (required) - Size of each chunk
///
/// # Returns
///
/// Returns an array of arrays, where each sub-array has at most `size` elements.
/// The last chunk may have fewer elements if the array length is not evenly divisible.
///
/// # Example
///
/// ```jinja
/// {# Split into chunks of 2 #}
/// {% set nums = [1, 2, 3, 4, 5] %}
/// {% for chunk in array_chunk(array=nums, size=2) %}
///   Chunk: {{ chunk }}
/// {% endfor %}
/// {# Output:
///    Chunk: [1, 2]
///    Chunk: [3, 4]
///    Chunk: [5]
/// #}
///
/// {# Pagination example #}
/// {% set items = ["a", "b", "c", "d", "e", "f"] %}
/// {% for page in array_chunk(array=items, size=3) %}
///   Page: {{ page }}
/// {% endfor %}
/// {# Output:
///    Page: ["a", "b", "c"]
///    Page: ["d", "e", "f"]
/// #}
/// ```
pub fn array_chunk_fn(kwargs: Kwargs) -> Result<Value, Error> {
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

/// Combine two arrays into pairs (zip)
///
/// # Arguments
///
/// * `array1` (required) - First array
/// * `array2` (required) - Second array
///
/// # Returns
///
/// Returns an array of two-element arrays, where each sub-array contains one element
/// from array1 and one from array2. The result length is the minimum of the two input lengths.
///
/// # Example
///
/// ```jinja
/// {# Zip two arrays #}
/// {% set keys = ["name", "age", "city"] %}
/// {% set values = ["Alice", 30, "NYC"] %}
/// {% for pair in array_zip(array1=keys, array2=values) %}
///   {{ pair[0] }}: {{ pair[1] }}
/// {% endfor %}
/// {# Output:
///    name: Alice
///    age: 30
///    city: NYC
/// #}
///
/// {# Different lengths - stops at shorter array #}
/// {% set a = [1, 2, 3, 4] %}
/// {% set b = ["a", "b"] %}
/// {{ array_zip(array1=a, array2=b) }}
/// {# Output: [[1, "a"], [2, "b"]] #}
/// ```
pub fn array_zip_fn(kwargs: Kwargs) -> Result<Value, Error> {
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

/// Sort array by object key
///
/// # Arguments
///
/// * `array` (required) - Array of objects to sort
/// * `key` (required) - Key name to sort by
///
/// # Returns
///
/// Returns a new array sorted by the specified key value
///
/// # Example
///
/// ```jinja
/// {# Sort users by age #}
/// {% set users = [
///   {"name": "Alice", "age": 30},
///   {"name": "Bob", "age": 25},
///   {"name": "Charlie", "age": 35}
/// ] %}
/// {% for user in array_sort_by(array=users, key="age") %}
///   {{ user.name }}: {{ user.age }}
/// {% endfor %}
/// {# Output:
///    Bob: 25
///    Alice: 30
///    Charlie: 35
/// #}
/// ```
pub fn array_sort_by_fn(kwargs: Kwargs) -> Result<Value, Error> {
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

/// Group array items by key
///
/// # Arguments
///
/// * `array` (required) - Array of objects to group
/// * `key` (required) - Key name to group by
///
/// # Returns
///
/// Returns an object where keys are the unique values from the specified key,
/// and values are arrays of items with that key value
///
/// # Example
///
/// ```jinja
/// {# Group users by department #}
/// {% set users = [
///   {"name": "Alice", "dept": "Engineering"},
///   {"name": "Bob", "dept": "Sales"},
///   {"name": "Charlie", "dept": "Engineering"}
/// ] %}
/// {% set grouped = array_group_by(array=users, key="dept") %}
/// {% for dept, members in grouped %}
///   {{ dept }}: {{ members | length }} members
/// {% endfor %}
/// {# Output:
///    Engineering: 2 members
///    Sales: 1 members
/// #}
/// ```
pub fn array_group_by_fn(kwargs: Kwargs) -> Result<Value, Error> {
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

/// Remove duplicates from array
///
/// # Arguments
///
/// * `array` (required) - Array to deduplicate
///
/// # Returns
///
/// Returns a new array with duplicate values removed (first occurrence kept)
///
/// # Example
///
/// ```jinja
/// {# Remove duplicates #}
/// {% set nums = [1, 2, 2, 3, 1, 4, 3, 5] %}
/// {{ array_unique(array=nums) }}
/// {# Output: [1, 2, 3, 4, 5] #}
///
/// {# Unique strings #}
/// {% set tags = ["docker", "kubernetes", "docker", "helm"] %}
/// {{ array_unique(array=tags) }}
/// {# Output: ["docker", "kubernetes", "helm"] #}
/// ```
pub fn array_unique_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let array: Value = kwargs.get("array")?;

    if !matches!(array.kind(), minijinja::value::ValueKind::Seq) {
        return Err(Error::new(
            ErrorKind::InvalidOperation,
            "array_unique requires an array",
        ));
    }

    use std::collections::HashSet;
    let mut seen: HashSet<String> = HashSet::new();
    let mut unique: Vec<serde_json::Value> = Vec::new();

    if let Ok(seq) = array.try_iter() {
        for item in seq {
            let json_value: serde_json::Value = serde_json::to_value(&item).map_err(|e| {
                Error::new(
                    ErrorKind::InvalidOperation,
                    format!("Failed to convert item: {}", e),
                )
            })?;

            // Create a string representation for comparison
            let item_str = serde_json::to_string(&json_value).unwrap_or_default();

            if seen.insert(item_str) {
                unique.push(json_value);
            }
        }
    }

    Ok(Value::from_serialize(&unique))
}

/// Flatten nested arrays
///
/// # Arguments
///
/// * `array` (required) - Array with nested arrays to flatten
///
/// # Returns
///
/// Returns a new array with all nested arrays flattened one level
///
/// # Example
///
/// ```jinja
/// {# Flatten nested arrays #}
/// {% set nested = [[1, 2], [3, 4], [5]] %}
/// {{ array_flatten(array=nested) }}
/// {# Output: [1, 2, 3, 4, 5] #}
///
/// {# Mixed types #}
/// {% set mixed = [["a", "b"], ["c"], ["d", "e"]] %}
/// {{ array_flatten(array=mixed) }}
/// {# Output: ["a", "b", "c", "d", "e"] #}
///
/// {# Multiple levels (only flattens one level) #}
/// {% set deep = [[1, [2, 3]], [4]] %}
/// {{ array_flatten(array=deep) }}
/// {# Output: [1, [2, 3], 4] #}
/// ```
pub fn array_flatten_fn(kwargs: Kwargs) -> Result<Value, Error> {
    let array: Value = kwargs.get("array")?;

    if !matches!(array.kind(), minijinja::value::ValueKind::Seq) {
        return Err(Error::new(
            ErrorKind::InvalidOperation,
            "array_flatten requires an array",
        ));
    }

    let mut flattened: Vec<serde_json::Value> = Vec::new();

    if let Ok(seq) = array.try_iter() {
        for item in seq {
            let json_value: serde_json::Value = serde_json::to_value(&item).map_err(|e| {
                Error::new(
                    ErrorKind::InvalidOperation,
                    format!("Failed to convert item: {}", e),
                )
            })?;

            // If item is an array, flatten it one level
            if let Some(nested_array) = json_value.as_array() {
                for nested_item in nested_array {
                    flattened.push(nested_item.clone());
                }
            } else {
                // Not an array, just add the item
                flattened.push(json_value);
            }
        }
    }

    Ok(Value::from_serialize(&flattened))
}
