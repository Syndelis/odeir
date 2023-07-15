#![feature(vec_into_raw_parts)]
pub mod ffi;
pub mod transformations;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub type NodeId = u32;

mod json;

#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy, PartialEq, Eq)]
#[serde(try_from = "char")]
#[serde(into = "char")]
#[repr(u8)]
pub enum LinkType {
    #[default]
    Normal,
    Negative,
}

impl Into<char> for LinkType {
    fn into(self) -> char {
        match self {
            Self::Normal => '+',
            Self::Negative => '-',
        }
    }
}
impl TryFrom<char> for LinkType {
    type Error = String;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(match c {
            '+' => Self::Normal,
            '-' => Self::Negative,
            _ => return Err(format!("Expected either + or -"))
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Link {
    pub receiver: NodeId,
    pub sender: NodeId,
    pub link_type: LinkType,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(from = "json::JsonModel")]
#[serde(into = "json::JsonModel")]
pub struct Model {
    pub meta_data: MetaData,
    pub links: Vec<Link>,
    pub nodes: HashMap<NodeId, Node>,
}


#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct MetaData {
    start_time: f64,
    end_time: f64,
    delta_time: f64,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Constant {
        id: NodeId,
        name: String,

        outputs: Vec<Link>,
        value: f64,
    },
    Population {
        id: NodeId,
        name: String,

        outputs: Vec<Link>,
        initial_population: f64,
    },
    Combinator {
        id: NodeId,
        name: String,

        outputs: Vec<Link>,
        inputs: Vec<Link>,
        operation: Operation,
    },
}

impl Node {
    fn name(&self) -> &str {
        match self {
            Self::Constant { name, .. } => name,
            Self::Population { name, .. } => name,
            Self::Combinator { name, .. } => name,
        }
    }
    fn id(&self) -> NodeId {
        match self {
            Self::Constant { id, .. } => *id,
            Self::Population { id, .. } => *id,
            Self::Combinator { id, .. } => *id,
        }
    }
    fn outputs(&mut self) -> &mut Vec<Link> {
        match self {
            Self::Constant { outputs, .. } => outputs,
            Self::Population { outputs, .. } => outputs,
            Self::Combinator { outputs, .. } => outputs,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default, PartialEq, Eq)]
#[serde(try_from = "char")]
#[serde(into = "char")]
#[repr(u8)]
pub enum Operation {
    #[default]
    Add,
    Sub,
    Div,
    Mul,
}

impl Into<char> for Operation {
    fn into(self) -> char {
        match self {
            Self::Add => '+',
            Self::Sub => '-',
            Self::Div => '/',
            Self::Mul => '*',
        }
    }
}
impl TryFrom<char> for Operation {
    type Error = String;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '+' => Ok(Self::Add),
            '-' => Ok(Self::Sub),
            '/' => Ok(Self::Div),
            '*' => Ok(Self::Mul),
            _ => Err(format!("Expected either '+', '-', '/', or '*'")),
        }
    }
}

pub fn model_into_json(model: &Model) -> String {
    serde_json::to_string(model).unwrap()
}

#[cfg(test)]
mod tests {

    use assert_json_diff::assert_json_eq;
    use serde_json::Value;

    use super::*;

    const SIMPLE_JSON: &str = include_str!("../tests/fixtures/simple.json");

    #[test]
    fn simple_is_de() {
        // Given - A JSON representing a model with:
        // - 7 Nodes: 2 Populations, 1 Combinator and 4 Constants

        // SIMPLE_JSON is included above

        // When - We deserialize the JSON into a Model

        let model = serde_json::from_str::<Model>(SIMPLE_JSON);

        let model = model.unwrap();

        // Then - The correct data is loaded up

        assert_eq!(model.nodes.len(), 5);

        let pop_1 = model.nodes.get(&1).unwrap();

        if let Node::Population {
            id,
            name,
            outputs,
            initial_population
        } = pop_1
        {
            assert_eq!(*id, 1);
            assert_eq!(name.as_str(), "Population 1");
            assert_eq!(outputs.len(), 1);
            assert_eq!(*initial_population, 100.0);

            let link = &outputs[0];

            assert_eq!(link.link_type,  LinkType::Normal);
            assert_eq!(link.receiver, 30);
        } else {
            panic!("Expected Node::Population for id 1");
        }

        let pop_2 = model.nodes.get(&2).unwrap();

        if let Node::Population {
            id,
            name,
            outputs,
            initial_population
        } = pop_2
        {
            assert_eq!(*id, 2);
            assert_eq!(name.as_str(), "Population 2");
            assert_eq!(outputs.len(), 1);
            assert_eq!(*initial_population, 200.0);

            let link = &outputs[0];

            assert_eq!(link.link_type,  LinkType::Negative);
            assert_eq!(link.receiver, 30);
        } else {
            panic!("Expected Node::Population for id 2");
        }

        let comb_30 = model.nodes.get(&30).unwrap();

        if let Node::Combinator {
            id,
            name,
            operation,
            inputs,
            outputs,
        } = comb_30
        {
            assert_eq!(*id, 30);
            assert_eq!(name.as_str(), "Pop1 + Pop2");
            assert_eq!(*operation, Operation::Add);
            assert_eq!(inputs.len(), 2);

            // Order matters here. Be careful.
            assert_eq!(inputs[0].sender, 1);
            assert_eq!(inputs[0].receiver, 30);
            assert_eq!(inputs[1].sender, 2);
            assert_eq!(inputs[1].receiver, 30);
        } else {
            panic!("Expected Node::Combinator for id 30");
        }

        let gravity = model.nodes.get(&4).unwrap();
        if let Node::Constant {
            id,
            name,
            value,
            outputs
        } = gravity
        {
            assert_eq!(*id, 4);
            assert_eq!(name.as_str(), "gravity");
            assert_eq!(*value, 9.81);
            assert_eq!(outputs.len(), 0);
        }

        let a = model.nodes.get(&5).unwrap();
        if let Node::Constant {
            id,
            name,
            value,
            outputs
        } = a
        {
            assert_eq!(*id, 5);
            assert_eq!(name.as_str(), "a");
            assert_eq!(*value, 1.6);
            assert_eq!(outputs.len(), 0);
        }
    }

    #[test]
    fn simple_is_ser() {
        // Given - We've recreated Simple's model in Rust

        let link1_30 = Link {
                link_type: LinkType::Normal,
                receiver: 30,
                sender: 1,
            };
        let node1 = Node::Population {
            id: 1,
            name: "Population 1".into(),
            initial_population: 100.0,
            outputs: vec![link1_30.clone()],
        };

        let link2_30 = Link {
                link_type: LinkType::Negative,
                receiver: 30,
                sender: 2,
            };
        let node2 = Node::Population {
            id: 2,
            name: "Population 2".into(),
            initial_population: 200.0,
            outputs: vec![link2_30.clone()],
        };

        let node4 = Node::Constant {
            id: 4,
            name: "gravity".into(),
            value: 9.81,
            outputs: vec![],
        };

        let node5 = Node::Constant {
            id: 5,
            name: "a".into(),
            value: 1.6,
            outputs: vec![],
        };

        let node30 = Node::Combinator {
            id: 30,
            name: "Pop1 + Pop2".into(),
            operation: Operation::Add,
            outputs: vec![],
            // Order here matters again. Be careful
            inputs: vec![link1_30.clone(), link2_30.clone()],
        };
        

        let mut nodes = HashMap::new();

        nodes.insert(1, node1);
        nodes.insert(2, node2);
        nodes.insert(4, node4);
        nodes.insert(5, node5);
        nodes.insert(30, node30);

        let model = Model {
            nodes,
            links: vec![link1_30, link2_30],
            meta_data: MetaData {
                start_time: 0.0,
                end_time: 10.5,
                delta_time: 0.125,
            },
        };

        // When - We serialize the model into JSON

        let model_json = model_into_json(&model);

        // Then - The JSON is identical to simple.json

        let model_json: Value = serde_json::from_str(&model_json).unwrap();
        let simple_json: Value = serde_json::from_str(SIMPLE_JSON).unwrap();

        assert_json_eq!(model_json, simple_json);
    }
}
