pub mod ffi;

use std::ffi::{c_char, CStr};

use ffi::{BoxedSlice, HashWrapper};

use serde::{Deserialize, Serialize};

pub type NodeId = u32;

#[repr(C)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    pub meta_data: MetaData,
    pub nodes: BoxedSlice<Node>,
    pub constants: BoxedSlice<Constant>,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MetaData {
    start_time: f64,
    end_time: f64,
    delta_time: f64,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum Node {
    Population {
        id: NodeId,
        name: String,
        related_constant_name: String,
    },
    Combinator {
        id: NodeId,
        name: String,
        operation: char,
        inputs: BoxedSlice<NodeId>,
    },
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Constant {
    pub name: String,
    pub value: f64,
}

pub fn model_from_string(json_str: &str) -> Model {
    serde_json::from_str(json_str).unwrap()
}

#[no_mangle]
pub extern "C" fn model_from_cstr(json_cstr: *const c_char) -> Model {
    let json_str = unsafe { std::ffi::CStr::from_ptr(json_cstr) }
        .to_str()
        .unwrap();

    model_from_string(json_str)
}
