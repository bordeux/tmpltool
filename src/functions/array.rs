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
