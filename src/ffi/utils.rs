use std::ffi::{c_char, CStr, CString};

#[allow(non_camel_case_types)]
pub(crate) type cchar = c_char;

#[allow(non_camel_case_types)]
pub(crate) type cstr = *const cchar;

#[repr(C)]
pub enum Option<T: Sized> {
    None,
    Some(T),
}

/// # Safety
/// This function is unsafe because it dereferences a raw pointer.
/// The caller must ensure that the pointer is valid.
pub(crate) unsafe fn cstr_cloned_into_string(cstr: cstr) -> String {
    let cstr = unsafe { CStr::from_ptr(cstr) };
    cstr.to_str().unwrap().to_string()
}


pub(crate) fn string_to_cstr(str: String) -> cstr {
    CString::new(str).unwrap().into_raw()
}

pub(crate) unsafe fn free_cstr(cstr: cstr) {
    let _ = CString::from_raw(cstr as *mut c_char);
}
