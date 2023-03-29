use std::ffi::{CString, c_char};

use crate::rustside::{Model, Node, Constant};

use super::structs::{CModel, CNode, CConstant};

fn string_to_cstring(str: String) -> *const c_char {
    CString::new(str).unwrap().into_raw()
}

impl From<Model> for CModel {
    fn from(value: Model) -> Self {
        let mut nodes = value.nodes.into_values().collect::<Vec<Node>>();

        // Sorted mostly for tests, but isn't strictly necessary.
        // Either way, it shouldn't be that expensive.
        nodes.sort_by_key(
            |node| match node {
                Node::Population { id, .. } => *id,
                Node::Combinator { id, .. } => *id,
            }
        );

        CModel {
            meta_data: value.meta_data,
            nodes: nodes.into(),
            constants: value.constants.into(),
        }
    }
}

impl From<Node> for CNode {
    fn from(value: Node) -> Self {
        match value {
            Node::Population { id, name, related_constant_name, links } => CNode::Population {
                id,
                name: string_to_cstring(name),
                related_constant_name: string_to_cstring(related_constant_name),
                links: links.into(),
            },
            Node::Combinator {
                id,
                name,
                operation,
                inputs,
            } => CNode::Combinator {
                id,
                name: string_to_cstring(name),
                operation,
                inputs: inputs.into(),
            },
        }
    }
}

impl From<Constant> for CConstant {
    fn from(value: Constant) -> Self {
        CConstant {
            name: string_to_cstring(value.name),
            value: value.value,
        }
    }
}
