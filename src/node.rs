use serde::{Serialize, Deserialize};

pub type NodeId = u32;

#[repr(C)]
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
    pub node_id: NodeId,
}

#[no_mangle]
pub unsafe extern "C" fn _this_function_guarantees_node_is_exported_by_cbindgen(node: *mut Node) {}