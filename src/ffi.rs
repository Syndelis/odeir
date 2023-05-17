use crate::Model;

pub fn model_into_json(model: Box<Model>) -> String {
    serde_json::to_string(&*model).unwrap()
}
