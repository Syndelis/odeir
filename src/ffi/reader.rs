use crate::{Model, NodeId, Node, Link};

use super::utils::{self, cstr};

#[repr(u8)]
pub enum NodeType {
    Population,
    Combinator,
    Constant
}

/// # Safety
/// This function is unsafe because it derefences the string pointer.
/// This function may return a null pointer if the JSON is invalid.
#[no_mangle]
pub unsafe extern "C" fn odeir_json_to_model(json: cstr) -> *mut Model {
    let json = utils::cstr_cloned_into_string(json);

    let model = serde_json::from_str::<Model>(&json);

    match model {
        Ok(model) => Box::leak(Box::new(model)),
        Err(_) => std::ptr::null_mut(),
    }
}

/// # Safety
/// This function is unsafe because it derefences the model raw pointer and also
/// writes to both `out_len` and `out_cap` pointers.
#[no_mangle]
pub unsafe extern "C" fn odeir_model_get_node_ids(
    model: *mut Model, out_len: *mut usize, out_cap: *mut usize
) -> *mut NodeId
{
    let model = unsafe { &mut *model };

    let ids: Vec<NodeId> = model.nodes.keys().copied().collect();

    let (ptr, len, cap) = ids.into_raw_parts();

    unsafe {
        *out_len = len;
        *out_cap = cap;
    }

    ptr
}


/// # Safety
/// This function is unsafe because it derefences the model raw pointer.
#[no_mangle]
pub unsafe extern "C" fn odeir_model_get_node(
    model: *mut Model, node_id: NodeId
) -> *mut Node
{
    let model = unsafe { &mut *model };

    let node = model.nodes.get_mut(&node_id);

    match node {
        Some(node_ref) => node_ref,
        None => std::ptr::null_mut(),
    }
}

/// # Safety
/// This function is unsafe because it derefences the node raw pointer.
#[no_mangle]
pub unsafe extern "C" fn odeir_node_outputs(
    node: *mut Node
) -> *mut Link {
    let node = unsafe { &mut *node };

    node.outputs().as_mut_ptr()
}
///
/// # Safety
/// This function is unsafe because it derefences the node raw pointer.
#[no_mangle]
pub unsafe extern "C" fn odeir_node_outputs_len(
    node: *mut Node
) -> usize {
    let node = unsafe { &mut *node };

    node.outputs().len()
}

/// # Safety
/// This function is unsafe because it derefences the node raw pointer.
#[no_mangle]
pub unsafe extern "C" fn odeir_combinator_inputs(
    node: *mut Node
) -> *mut Link
{
    let node = unsafe { &mut *node };

    if let Node::Combinator { ref mut inputs, .. } = node {
        inputs.as_mut_ptr()
    } else {
        panic!("Node is not a combinator");
    }
}
///
/// # Safety
/// This function is unsafe because it derefences the node raw pointer.
#[no_mangle]
pub unsafe extern "C" fn odeir_combinator_inputs_len(
    node: *mut Node
) -> usize
{
    let node = unsafe { &mut *node };

    if let Node::Combinator { ref mut inputs, .. } = node {
        inputs.len()
    } else {
        panic!("Node is not a combinator");
    }
}

/// # Safety
/// This function is unsafe because it derefences the node raw pointer and the
/// out_type pointer.
#[no_mangle]
pub unsafe extern "C" fn odeir_node_get_info(node: *mut Node, out_type: *mut NodeType) -> cstr {
    let node = unsafe { &mut *node };

    let node_type = match node {
        Node::Constant {..} => NodeType::Constant,
        Node::Population {..} => NodeType::Population,
        Node::Combinator {..} => NodeType::Combinator,
    };

    unsafe { *out_type = node_type };

    utils::string_to_cstr(node.name())
}
