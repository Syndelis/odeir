use crate::rustside::{Link, MetaData, NodeId};

use std::ffi::c_char;

use crate::boxed_slice::BoxedSlice;

#[repr(C)]
#[derive(Debug)]
pub struct CModel {
    pub meta_data: MetaData,
    pub nodes: BoxedSlice<CNode>,
    pub constants: BoxedSlice<CConstant>,
}

#[repr(C)]
#[derive(Debug)]
pub struct CLink {
    pub sign: c_char,
    pub node_id: NodeId,
}

#[repr(C)]
#[derive(Debug)]
pub enum CNode {
    Population {
        id: NodeId,
        name: *const c_char,
        related_constant_name: *const c_char,
        links: BoxedSlice<Link>,
    },
    Combinator {
        id: NodeId,
        name: *const c_char,
        operation: char,
        inputs: BoxedSlice<NodeId>,
    },
}

#[repr(C)]
#[derive(Debug)]
pub struct CConstant {
    pub name: *const c_char,
    pub value: f64,
}