use crate::{Model, NodeId, Node, Link};

use super::utils::{self, cstr};

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
pub unsafe extern "C" fn odeir_model_take_node(
    model: *mut Model, node_id: NodeId
) -> *mut Node
{
    let model = unsafe { &mut *model };

    let node = model.nodes.remove(&node_id);

    match node {
        Some(node) => Box::leak(Box::new(node)),
        None => std::ptr::null_mut(),
    }
}

/// # Safety
/// This function is unsafe because it derefences the node raw pointer.
#[no_mangle]
pub unsafe extern "C" fn odeir_population_take_next_link(node: *mut Node) -> utils::Option<Link> {
    let node = unsafe { &mut *node };

    if let Node::Population { ref mut links, .. } = node {
        match links.pop() {
            Some(link) => utils::Option::Some(link),
            None => utils::Option::None,
        }
    } else {
        panic!("Node is not a population");
    }
}
