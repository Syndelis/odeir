use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

#[cfg(not(test))]
type Map<K, V> = HashMap<K, V>;
// Trocamos para `BTreeMap` para testar pois ele garante uma ordem dos elementos
#[cfg(test)]
type Map<K, V> = BTreeMap<K, V>;

mod json;
mod models;
pub mod transformations;

#[cfg(test)]
mod tests {}
