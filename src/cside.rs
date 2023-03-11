use crate::rustside::{Constant, MetaData, Model, Node, NodeId};

use std::{ffi::{c_char, c_int, CStr, CString}, panic::{RefUnwindSafe, UnwindSafe}, fmt::Display};

#[repr(C)]
pub struct BoxedSlice<T> {
    ptr: *const T,
    len: usize,
    destructor: extern "C" fn(*const Self) -> c_int,
}

impl<T> std::fmt::Debug for BoxedSlice<T> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RawVec<{}> {{ len: {}, ptr: {:p}}}", std::any::type_name::<T>(), self.len, self.ptr)
    } 
}

impl<T> BoxedSlice<T> where T: RefUnwindSafe {
    pub extern "C" fn destructor(this: *const Self) -> c_int {
        catch_panic(move || {
            let this = unsafe { &*this };
            // If this is zero, there was no memory allocated.
            if this.len == 0 {
                return;
            }
            unsafe {
                let slice = std::ptr::slice_from_raw_parts_mut(this.ptr as *mut T, this.len);
                std::mem::drop(Box::from_raw(slice));
            }
        })
    }
}

fn catch_panic(f: impl FnOnce() + UnwindSafe) -> c_int {
    result_to_int(std::panic::catch_unwind(f))
}

impl<T, U> From<Vec<T>> for BoxedSlice<U>
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
    pub nodes: BoxedSlice<CNode>,
    pub constants: BoxedSlice<CConstant>,
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
        inputs: BoxedSlice<NodeId>,
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
    (data.as_mut_ptr(), data.len())
}

fn result_to_int<T, E>(result: Result<T, E>) -> c_int {
    match result {
        Ok(_) => 1,
        Err(_) => 0,
    }
}

fn print_unwrap<T, E: Display>(result: Result<T, E>) -> T {
    match result {
        Ok(v) => v, 
        Err(e) => unsafe {
            let err = e.to_string().replace("\0", "\\0");
            let err = CString::new(err).unwrap();
            libc::printf(b"Rust Error: %s\n\0".as_ptr() as *const c_char, err.as_ptr());
            panic!("{}", e);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn model_from_cstring(json_str: *const c_char, cmodel: *mut CModel) -> c_int {
    catch_panic(|| {
        let json_str = print_unwrap(unsafe { CStr::from_ptr(json_str) }.to_str());
        let model: Model = print_unwrap(serde_json::from_str(json_str));
        cmodel.write(model.into())
    })
}
