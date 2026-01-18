//! Nested generic types.

use std::collections::{HashMap, BTreeMap};

/// Process data with deeply nested generic types.
///
/// # Type Parameters
///
/// * `T` - The inner value type
///
/// # Arguments
///
/// * `data` - A vector of hashmaps containing optional values
///
/// # Returns
///
/// A result containing the extracted values or an error.
pub fn process_nested<T>(
    data: Vec<HashMap<String, Option<T>>>,
) -> Result<Vec<T>, ProcessError>
where
    T: Clone,
{
    let mut results = Vec::new();
    for map in data {
        for (_, value) in map {
            if let Some(v) = value {
                results.push(v);
            }
        }
    }
    Ok(results)
}

/// Even more deeply nested types.
pub fn deep_nesting<K, V>(
    input: HashMap<K, Vec<Option<BTreeMap<String, (V, Vec<V>)>>>>,
) -> Vec<V>
where
    K: std::hash::Hash + Eq,
    V: Clone,
{
    let mut results = Vec::new();
    for (_, outer_vec) in input {
        for opt_map in outer_vec.into_iter().flatten() {
            for (_, (v, vec)) in opt_map {
                results.push(v);
                results.extend(vec);
            }
        }
    }
    results
}

/// Triple-nested Result types.
pub fn nested_results<T, E1, E2, E3>(
    input: Result<Result<Result<T, E1>, E2>, E3>,
) -> Option<T> {
    input.ok()?.ok()?.ok()
}

/// Nested function pointer types.
pub fn higher_order<T, U, V>(
    f: fn(fn(T) -> U) -> fn(U) -> V,
    g: fn(T) -> U,
    x: U,
) -> V {
    f(g)(x)
}

/// Error type for process operations.
#[derive(Debug, thiserror::Error)]
pub enum ProcessError {
    #[error("Empty input")]
    EmptyInput,
    #[error("Invalid data: {0}")]
    InvalidData(String),
}
