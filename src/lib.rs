#![feature(vec_into_raw_parts)]

pub mod ffi;
pub mod transformations;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub type NodeId = u32;

#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum LinkType {
    #[default]
    Normal,
    Negative,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Link {
    incoming: NodeId,
    ongoing: NodeId,
    link_type: LinkType,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Model {
    pub meta_data: MetaData,
    pub nodes: HashMap<NodeId, Node>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MetaData {
    start_time: f64,
    end_time: f64,
    delta_time: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Operation {
    #[default]
    Add,
    Sub,
    Div,
    Mul,
}

impl Operation {
    pub fn to_char(&self) -> char {
        match self {
            Self::Add => '+',
            Self::Sub => '-',
            Self::Div => '/',
            Self::Mul => '*',
        }
    }
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '+' => Some(Self::Add),
            '-' => Some(Self::Sub),
            '/' => Some(Self::Div),
            '*' => Some(Self::Mul),
            _ => None,
        }
    }
}

use serde::de::Error;
impl<'de> serde::Deserialize<'de> for Operation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        Self::from_char(char::deserialize(deserializer)?).ok_or_else(|| <D::Error>::custom("Expected either +, -, / or *"))
    }
}

impl serde::Serialize for Operation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        Self::to_char(self).serialize(serializer)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Constant {
    pub name: String,
    pub value: f64,
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
        // - 3 Nodes: 2 Populations and 1 Combinator
        // - 4 Constants

        // SIMPLE_JSON is included above

        // When - We deserialize the JSON into a Model

        let model = serde_json::from_str::<Model>(SIMPLE_JSON);

        assert!(model.is_ok());

        let model = model.unwrap();

        // Then - The correct data is loaded up

        assert_eq!(model.nodes.len(), 3);

        assert_eq!(model.constants.len(), 4);

        let pop_1 = model.nodes.get(&1).unwrap();

        if let Node::Population {
            id,
            name,
            links,
        // !TODO: match this
            initial_population
        } = pop_1
        {
            assert_eq!(*id, 1);
            assert_eq!(name.as_str(), "Population 1");
            assert_eq!(links.len(), 1);

            let link = &links[0];

            assert_eq!(link.link_type,  LinkType::Normal);
            assert_eq!(link.node_id, 30);
        } else {
            panic!("Expected Node::Population for id 1");
        }

        let pop_2 = model.nodes.get(&2).unwrap();

        if let Node::Population {
            id,
            name,
            links,
        // !TODO: match this
            initial_population
        } = pop_2
        {
            assert_eq!(*id, 2);
            assert_eq!(name.as_str(), "Population 2");
            assert_eq!(links.len(), 1);

            let link = &links[0];

            assert_eq!(link.link_type,  LinkType::Negative);
            assert_eq!(link.node_id, 30);
        } else {
            panic!("Expected Node::Population for id 2");
        }

        let comb_30 = model.nodes.get(&30).unwrap();

        if let Node::Combinator {
            id,
            name,
            operation,
            links
        } = comb_30
        {
            assert_eq!(*id, 30);
            assert_eq!(name.as_str(), "Pop1 + Pop2");
            assert_eq!(*operation, Operation::Add);
            assert_eq!(links.len(), 2);
            assert_eq!(links[0], 1);
            assert_eq!(links[1], 2);
        } else {
            panic!("Expected Node::Combinator for id 30");
        }
    }

    #[test]
    fn simple_is_ser() {
        // Given - We've recreated Simple's model in Rust

        let node1 = Node::Population {
            id: 1,
            name: "Population 1".into(),
            links: vec![Link {
                operation: Operation::Add,
                node_id: 30,
            }],
        };

        let node2 = Node::Population {
            id: 2,
            name: "Population 2".into(),
            links: vec![Link {
                operation: Operation::Sub,
                node_id: 30,
            }],
        };

        let node30 = Node::Combinator {
            id: 30,
            name: "Pop1 + Pop2".into(),
            operation: Operation::Add,
            links: vec![1, 2],
        };

        let mut nodes = HashMap::new();

        nodes.insert(1, node1);
        nodes.insert(2, node2);
        nodes.insert(30, node30);

        let model = Model {
            nodes,
            meta_data: MetaData {
                start_time: 0.0,
                end_time: 10.5,
                delta_time: 0.125,
            },
            constants: vec![
                Constant {
                    name: "gravity".into(),
                    value: 9.81,
                },
                Constant {
                    name: "Population 1_0".into(),
                    value: 100.0,
                },
                Constant {
                    name: "Population 2_0".into(),
                    value: 200.0,
                },
                Constant {
                    name: "a".into(),
                    value: 1.6,
                },
            ],
        };

        // When - We serialize the model into JSON

        let model_json = model_into_json(&model);

        // Then - The JSON is identical to simple.json

        let model_json: Value = serde_json::from_str(&model_json).unwrap();
        let simple_json: Value = serde_json::from_str(SIMPLE_JSON).unwrap();

        assert_json_eq!(model_json, simple_json);
    }
}
