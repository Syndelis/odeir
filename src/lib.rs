
pub type Map<K, V> = std::collections::BTreeMap<K, V>;

pub mod json;
pub mod models;
pub mod transformations;

pub use json::{Json, Metadata, ModelMetadata};
pub use models::Argument;

#[cfg(test)]
mod tests {}
