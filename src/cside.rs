use serde::Deserialize;

use crate::rustside::{Constant, MetaData, Model, Node, NodeId};

use std::{ffi::{c_char, c_int, CStr, CString}, panic::{RefUnwindSafe, UnwindSafe}, fmt::Display};

impl<T: Deserialize> serde::Deserialize for BoxedSlice<T> {
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        let vec = Vec::deserialize(deserializer)?;
        Ok(BoxedSlice::from(vec))
    }
}

impl<T: Serialize> serde::Serialize for BoxedSlice<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        self.as_ref().serialize(serializer)
    }
}

#[repr(C)]
pub struct BoxedSlice<T> {
    ptr: *const T,
    len: usize,
    destructor: extern "C" fn(*const Self) -> c_int,
}

impl<T> AsRef<[T]> for  BoxedSlice<T> {
    fn as_ref(&self) -> &[T] {
        unsafe {
            std::slice::from_raw_parts(self.ptr, self.len)
        }
     } 
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
