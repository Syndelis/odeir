pub mod ffi;
pub mod transformations;

use std::{ffi::{c_char, CStr}, str::FromStr};

use ffi::{BoxedSlice, HashWrapper};

use serde::{Deserialize, Serialize};
use ustr::Ustr;

pub type NodeId = u32;

#[repr(C)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
    pub meta_data: MetaData,
    pub nodes: HashWrapper<NodeId, Node>,
    pub constants: BoxedSlice<Constant>,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug)]
pub struct MetaData {
    start_time: f64,
    end_time: f64,
    delta_time: f64,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Node {
    Population {
        id: NodeId,
        name: Ustr,
        related_constant_name: Ustr,
        links: BoxedSlice<Link>,
    },
    Combinator {
        id: NodeId,
        name: Ustr,
        operation: char,
        inputs: BoxedSlice<NodeId>,
    },
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Link {
    pub sign: char,
    pub node_id: u32,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Constant {
    pub name: Ustr,
    pub value: f64,
}

impl FromStr for Model {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

#[no_mangle]
pub extern "C" fn model_from_cstr(json_cstr: *const c_char) -> Model {
    let json_str = unsafe { std::ffi::CStr::from_ptr(json_cstr) }
        .to_str()
        .unwrap();

    Model::from_str(json_str).unwrap()
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

        let model = Model::from_str(SIMPLE_JSON);

        assert!(model.is_ok());

        let model = model.unwrap();

        // Then - The correct data is loaded up

        assert_eq!(model.nodes.0.len(), 3);

        assert_eq!(model.constants.as_ref().len(), 4);

        let pop_1 = model.nodes.0.get(&1).unwrap();

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
            assert_eq!(links.as_ref().len(), 1);
            
            let link = &links.as_ref()[0];

            assert_eq!(link.sign, '+');
            assert_eq!(link.node_id, 30);
        } else {
            panic!("Expected Node::Population for id 1");
        }

        let pop_2 = model.nodes.0.get(&2).unwrap();

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
            assert_eq!(links.as_ref().len(), 1);

            let link = &links.as_ref()[0];

            assert_eq!(link.sign, '-');
            assert_eq!(link.node_id, 30);
        } else {
            panic!("Expected Node::Population for id 2");
        }

        let comb_30 = model.nodes.0.get(&30).unwrap();

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
            assert_eq!(inputs.as_ref().len(), 2);
            assert_eq!(inputs.as_ref()[0], 1);
            assert_eq!(inputs.as_ref()[1], 2);
        } else {
            panic!("Expected Node::Combinator for id 30");
        }

    }

}
