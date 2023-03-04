use crate::rustside::{Constant, MetaData, Model, Node, NodeId};

use std::{ffi::{c_char, c_int, CStr, CString}, panic::RefUnwindSafe};

#[repr(C)]
#[derive(Debug)]
pub struct RawVec<T> {
    pub ptr: *const T,
    pub len: usize,
    pub destructor: extern "C" fn(Self) -> c_int,
}

impl<T> RawVec<T> where T: RefUnwindSafe {
    pub extern "C" fn destructor(self) -> c_int {
        let result = std::panic::catch_unwind(move || {
            unsafe {
                let slice = std::ptr::slice_from_raw_parts_mut(self.ptr as *mut T, self.len);
                std::mem::drop(Box::from_raw(slice));
            }
        });
        result_to_int(result)
    }
}

impl<T, U> From<Vec<T>> for RawVec<U>
where
    T: Into<U>,
    U: RefUnwindSafe
{
    fn from(value: Vec<T>) -> Self {
        let (ptr, len) = vec_to_ptr(value);
        Self {
            len,
            ptr,
            destructor: Self::destructor,
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CModel {
    pub meta_data: MetaData,
    pub nodes: RawVec<CNode>,
    pub constants: RawVec<CConstant>,
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
        inputs: RawVec<NodeId>,
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
        CModel {
            meta_data: value.meta_data,
            nodes: value.nodes.into(),
            constants: value.constants.into(),
        }
    }
}

impl From<Node> for CNode {
    fn from(value: Node) -> Self {
        match value {
            Node::Population {
                id,
                name,
                related_constant_name,
            } => CNode::Population {
                id,
                name: string_to_cstring(name),
                related_constant_name: string_to_cstring(related_constant_name),
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

fn string_to_cstring(str: String) -> *const c_char {
    CString::new(str).unwrap().into_raw()
}

fn vec_to_ptr<T, U>(vec: Vec<T>) -> (*mut U, usize)
where
    T: Into<U>,
{
    let data = vec
        .into_iter()
        .map(|el: T| el.into())
        .collect::<Vec<U>>()
        .into_boxed_slice();
    let data = Box::leak(data);
    (data.as_ptr() as *mut _, data.len())
}

fn result_to_int<T, E>(result: Result<T, E>) -> c_int {
    match result {
        Ok(_) => 1,
        Err(_) => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn model_from_cstring(json_str: *const c_char, cmodel: *mut CModel) -> c_int {
    let result = std::panic::catch_unwind(|| {
        let json_str = unsafe { CStr::from_ptr(json_str) };
        let model: Model = serde_json::from_str(json_str.to_str().unwrap()).unwrap();
        *cmodel = model.into()
    });
    result_to_int(result)
}
