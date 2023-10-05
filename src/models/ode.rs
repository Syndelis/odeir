use serde::{Deserialize, Serialize};

use super::Equations;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Metadata {
    start_time: f64,
    delta_time: f64,
    end_time: f64,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub metadata: Metadata,
    pub equations: Equations,
}
