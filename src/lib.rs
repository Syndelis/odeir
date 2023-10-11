
type Map<K, V> = std::collections::BTreeMap<K, V>;

pub mod json;
pub mod models;
pub mod transformations;

#[cfg(test)]
mod tests {}
