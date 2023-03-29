use std::{ffi::{CStr, c_char, c_int, CString}, fmt::Display};

use crate::{boxed_slice::catch_panic, rustside::Model};

use self::structs::CModel;

pub mod structs;
pub mod impls;

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

/// # Safety
/// `json_str` must be a valid pointer to a null-terminated C string. The
/// caller is responsible for freeing the returned `CModel` fields.
#[no_mangle]
pub unsafe extern "C" fn model_from_cstring(json_str: *const c_char, cmodel: *mut CModel) -> c_int {
    catch_panic(|| {
        let json_str = print_unwrap(unsafe { CStr::from_ptr(json_str) }.to_str());
        let model: Model = print_unwrap(serde_json::from_str(json_str));
        cmodel.write(model.into())
    })
}