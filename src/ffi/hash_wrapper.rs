use std::{hash::Hash, collections::HashMap};

use serde::{Serialize, Deserialize};

#[repr(C)]
#[derive(Serialize, Deserialize, Debug)]
pub struct HashWrapper<K: Hash + Eq, V>(pub Box<HashMap<K, V>>);
