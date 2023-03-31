#![feature(vec_into_raw_parts)]

pub mod transformations;
pub mod node;

use std::collections::HashMap;

use cxx::{ExternType, type_id};
use serde::{Deserialize, Serialize};
use serde_json::error::Result;

unsafe impl ExternType for node::Node {
    type Id = type_id!("Node");
    type Kind = cxx::kind::Trivial;
}


#[derive(Serialize, Deserialize, Debug)]
pub struct OpaqueNodeHashMap(HashMap<node::NodeId, node::Node>);

#[cxx::bridge]
mod ffi {

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Model {
        pub meta_data: MetaData,
        pub nodes: Box<OpaqueNodeHashMap>,
        pub constants: Vec<Constant>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct MetaData {
        start_time: f64,
        end_time: f64,
        delta_time: f64,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Constant {
        pub name: String,
        pub value: f64,
    }

    extern "C++" {
        include!("odeir.hpp");
        type Node = crate::node::Node;
    }

    extern "Rust" {
        type OpaqueNodeHashMap;

        fn model_from_string(json_str: &str) -> Result<Model>;
        fn new_model() -> Model;
        unsafe fn get_next_node(model: &mut Model, node_ptr: *mut Node);
        // fn return_next_node(model: &mut Model) -> Node;
    }
}

use ffi::*;

fn new_model() -> Model {

    let mut nodes = HashMap::new();
    nodes.insert(1, Node::Population {
        id: 1,
        name: "Meu nodo".to_string(),
        related_constant_name: "Meu nodo_0".to_string(),
        links: vec![node::Link {
            sign: '+',
            node_id: 2,
        }],
    });

    Model {
        meta_data: MetaData {
            start_time: 0.0,
            end_time: 1.0,
            delta_time: 0.1,
        },
        nodes: Box::new(OpaqueNodeHashMap(nodes)),
        constants: vec![Constant {
            name: "gravity".to_string(),
            value: 9.81,
        }],
    }
}

pub fn model_from_string(json_str: &str) -> Result<Model> {
    serde_json::from_str(json_str)
}

unsafe fn get_next_node(model: &mut Model, node_ptr: *mut Node) {
    // Take random element from Model.nodes.0
    let node_id = *model.nodes.0.keys().next().unwrap();
    let node = model.nodes.0.remove(&node_id).unwrap();

    unsafe {
        std::ptr::copy_nonoverlapping(&node, node_ptr, 1);
        std::mem::forget(node);
    }
}

// fn return_next_node(model: &mut Model) -> Node {
//     model.nodes.0.remove(&0).unwrap()
// }


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simple_json_is_valid() {
        use model_from_string;
        const SIMPLE_JSON_STR: &str = include_str!("../tests/fixtures/simple.json");
        
        let model = model_from_string(SIMPLE_JSON_STR);

        assert!(model.is_ok(), "{}", model.unwrap_err());

        let model = model.unwrap();

    }

    #[test]
    fn test_render_edo_simple_json() {
        use transformations::edo::render_edo;

        const SIMPLE_JSON_STR: &str = include_str!("../tests/fixtures/simple.json");

        let model = model_from_string(SIMPLE_JSON_STR).unwrap();

        let edo = render_edo(model);

        println!("{}", edo);
    }

    #[test]
    fn test_render_edo_abc_json() {
        use transformations::edo::render_edo;

        const ABC_JSON_STR: &str = include_str!("../tests/fixtures/abc.json");

        let model = model_from_string(ABC_JSON_STR).unwrap();

        let edo = render_edo(model);

        println!("{}", edo);
    }

}
