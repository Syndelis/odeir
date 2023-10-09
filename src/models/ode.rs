use serde::{Deserialize, Serialize};

use super::Equations;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Metadata {
    pub start_time: f64,
    pub delta_time: f64,
    pub end_time: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct Model {
    pub metadata: Metadata,
    #[serde(flatten)]
    pub equations: Equations,
}

impl Model {
    pub fn new(metadata: Metadata) -> Self {
        Self {
            metadata,
            equations: Equations::new(),
        }
    }
}
