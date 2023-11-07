use serde::{Deserialize, Serialize};

use super::CoreModel;

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
    pub core: CoreModel,
}

impl Model {
    pub fn new(metadata: Metadata) -> Self {
        Self {
            metadata,
            core: CoreModel::new(),
        }
    }
}

impl std::ops::Deref for Model {
    type Target = CoreModel;

    fn deref(&self) -> &Self::Target {
        &self.core
    }
}

impl std::ops::DerefMut for Model {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.core
    }
}
