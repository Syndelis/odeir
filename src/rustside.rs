use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::error::Result;

pub type NodeId = u32;

#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
    pub meta_data: MetaData,
    pub nodes: HashMap<NodeId, Node>,
    pub constants: Vec<Constant>,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug)]
pub struct MetaData {
    start_time: f64,
    end_time: f64,
    delta_time: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Link {
    pub sign: char,
    pub node_id: NodeId,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Node {
    Population {
        id: NodeId,
        name: String,
        related_constant_name: String,
        links: Vec<Link>,
    },
    Combinator {
        id: NodeId,
        name: String,
        operation: char,
        inputs: Vec<NodeId>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Constant {
    pub name: String,
    pub value: f64,
}

pub fn model_from_string(json_str: &str) -> Result<Model> {
    serde_json::from_str(json_str)
}
