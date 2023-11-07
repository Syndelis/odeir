pub type Map<K, V> = std::collections::BTreeMap<K, V>;

pub mod json;
pub mod models;
pub mod transformations;

pub use json::{Json, Metadata, Model, ModelMetadata};
pub use models::{Argument, Component, Equation};

#[cfg(test)]
mod tests {}
