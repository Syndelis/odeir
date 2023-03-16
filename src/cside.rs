use crate::rustside::{Constant, MetaData, Model, Node, NodeId};

use std::ffi::{c_char, CStr, CString};

#[repr(C)]
#[derive(Debug)]
pub struct CModel {
    pub meta_data: MetaData,
    pub nodes: *const CNode,
    pub constants: *const CConstant,
    pub node_count: usize,
    pub constant_count: usize,
}

#[repr(C)]
#[derive(Debug)]
pub enum CNode {
    Population {
        id: NodeId,
        name: *const c_char,
        related_constant_name: *const c_char,
    },
    Combinator {
        id: NodeId,
        name: *const c_char,
        operation: char,
        inputs: *const NodeId,
        input_count: usize,
    },
}

#[repr(C)]
#[derive(Debug)]
pub struct CConstant {
    pub name: *const c_char,
    pub value: f64,
}

impl From<Model> for CModel {
    fn from(value: Model) -> Self {
        let (nodes_ptr, nodes_len, _nodes_cap) = vec_to_ptr(value.nodes.into_values().collect::<Vec<Node>>());
        let (constants_ptr, constants_len, _constants_cap) = vec_to_ptr(value.constants);

        CModel {
            meta_data: value.meta_data,
            node_count: nodes_len,
            constant_count: constants_len,
            nodes: nodes_ptr,
            constants: constants_ptr,
        }
    }
}

impl From<Node> for CNode {
    fn from(value: Node) -> Self {
        match value {
            Node::Population { id, name, related_constant_name, links, .. } => CNode::Population {
                id,
                name: string_to_cstring(name),
                related_constant_name: string_to_cstring(related_constant_name),
            },
            Node::Combinator {
                id,
                name,
                operation,
                inputs,
            } => {
                let (inputs_ptr, inputs_len, _cap) = inputs.into_raw_parts();

                CNode::Combinator {
                    id,
                    name: string_to_cstring(name),
                    operation,
                    input_count: inputs_len,
                    inputs: inputs_ptr,
                }
            }
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

fn string_to_cstring(str: String) -> *const c_char {
    CString::new(str).unwrap().into_raw()
}

fn vec_to_ptr<T, U>(vec: Vec<T>) -> (*mut U, usize, usize)
where
    T: Into<U>,
{
    vec.into_iter()
        .map(|el: T| el.into())
        .collect::<Vec<U>>()
        .into_raw_parts()
}

/// # Safety
/// `json_str` must be a valid pointer to a null-terminated C string. The
/// caller is responsible for freeing the returned `CModel` fields.
#[no_mangle]
pub unsafe extern "C" fn model_from_cstring(json_str: *const c_char) -> CModel {
    let json_str = unsafe { CStr::from_ptr(json_str) };
    let model: Model = serde_json::from_str(json_str.to_str().unwrap()).unwrap();
    model.into()
}
