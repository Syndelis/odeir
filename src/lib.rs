pub mod ffi;
pub mod transformations;

use std::{ffi::{c_char, CStr}, str::FromStr, collections::HashMap};

use serde::{Deserialize, Serialize};

pub type NodeId = u32;

#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
    pub meta_data: MetaData,
    pub nodes: HashMap<NodeId, Node>,
    pub constants: Vec<Constant>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MetaData {
    start_time: f64,
    end_time: f64,
    delta_time: f64,
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
pub struct Link {
    pub sign: char,
    pub node_id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Constant {
    pub name: String,
    pub value: f64,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn simple_is_ser() {

        // Given - A JSON representing a model with:
        // - 3 Nodes: 2 Populations and 1 Combinator
        // - 4 Constants

        const SIMPLE_JSON: &str = include_str!("../tests/fixtures/simple.json");

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
            related_constant_name,
            links,
        } = pop_1
        {
            assert_eq!(*id, 1);
            assert_eq!(name.as_str(), "Population 1");
            assert_eq!(related_constant_name.as_str(), "Population 1_0");
            assert_eq!(links.len(), 1);
            
            let link = &links[0];

            assert_eq!(link.sign, '+');
            assert_eq!(link.node_id, 30);
        } else {
            panic!("Expected Node::Population for id 1");
        }

        let pop_2 = model.nodes.get(&2).unwrap();

        if let Node::Population {
            id,
            name,
            related_constant_name,
            links,
        } = pop_2
        {
            assert_eq!(*id, 2);
            assert_eq!(name.as_str(), "Population 2");
            assert_eq!(related_constant_name.as_str(), "Population 2_0");
            assert_eq!(links.len(), 1);

            let link = &links[0];

            assert_eq!(link.sign, '-');
            assert_eq!(link.node_id, 30);
        } else {
            panic!("Expected Node::Population for id 2");
        }

        let comb_30 = model.nodes.get(&30).unwrap();

        if let Node::Combinator {
            id,
            name,
            operation,
            inputs,
        } = comb_30
        {
            assert_eq!(*id, 30);
            assert_eq!(name.as_str(), "Pop1 + Pop2");
            assert_eq!(*operation, '+');
            assert_eq!(inputs.len(), 2);
            assert_eq!(inputs[0], 1);
            assert_eq!(inputs[1], 2);
        } else {
            panic!("Expected Node::Combinator for id 30");
        }

    }

}
