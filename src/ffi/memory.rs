use crate::{Model, Node};

use super::utils::{cstr, self};

/// # Safety
/// This function is unsafe because it derefences the model raw pointer and
/// frees it. Using the pointer after calling this function will result in
/// undefined behavior.
#[no_mangle]
pub unsafe extern "C" fn odeir_free_model(model: *mut Model) {
    if model.is_null() { return; }
    let _ = unsafe { Box::from_raw(model) };
}

/// # Safety
/// This function is unsafe because it derefences and frees a pointer.
#[no_mangle]
pub unsafe extern "C" fn odeir_free_cstr(cstr: cstr) {
    utils::free_cstr(cstr);
}

/// # Safety
/// This function is unsafe because it derefences the node raw pointer and
/// frees it. Using the pointer after calling this function will result in
/// undefined behavior.
#[no_mangle]
pub unsafe extern "C" fn odeir_free_node(node: *mut Node)  {
    if node.is_null() { return; }
    let _ = unsafe { Box::from_raw(node) };
}
