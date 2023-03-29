mod internals;

pub use internals::catch_panic;
use internals::vec_to_ptr;
use std::{ffi::c_int, panic::RefUnwindSafe};

#[repr(C)]
pub struct BoxedSlice<T> {
    pub ptr: *const T,
    pub len: usize,
    pub destructor: extern "C" fn(*const Self) -> c_int,
}

impl<T> std::fmt::Debug for BoxedSlice<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RawVec<{}> {{ len: {}, ptr: {:p}}}",
            std::any::type_name::<T>(),
            self.len,
            self.ptr
        )
    }
}

impl<T> BoxedSlice<T>
where
    T: RefUnwindSafe,
{
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

impl<T, U> From<Vec<T>> for BoxedSlice<U>
where
    T: Into<U>,
    U: RefUnwindSafe,
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
