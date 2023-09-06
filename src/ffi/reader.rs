use std::{fs::File, io::BufReader};

use crate::{Link, Model, Node, NodeId};

use super::utils::{self, cstr};

#[repr(u8)]
pub enum NodeType {
    Population,
    Combinator,
}

/// # Safety
/// This function is unsafe because it dereferences the string pointer.
/// This function may return a null pointer if the file doesn't exist or if the
/// file content is invalid JSON.
#[no_mangle]
pub unsafe extern "C" fn odeir_json_file_path_to_model(json_file_path: cstr) -> *mut Model {
    let json_file_path = utils::cstr_cloned_into_string(json_file_path);

    let Ok(json_file) = File::open(json_file_path) else {
        eprintln!("DBG: File not found!!");
        return std::ptr::null_mut();
    };

    let json_reader = BufReader::new(json_file);

    let model = serde_json::from_reader(json_reader);

    match model {
        Ok(model) => Box::leak(Box::new(model)),
        Err(_) => {
            eprintln!("DBG: couldn't make it into a model!!");
            std::ptr::null_mut()
        },
    }
}

/// # Safety
/// This function is unsafe because it derefences the string pointer.
/// This function may return a null pointer if the string is invalid JSON.
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
    model: *mut Model,
    out_len: *mut usize,
    out_cap: *mut usize,
) -> *mut NodeId {
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
pub unsafe extern "C" fn odeir_model_get_node(model: *mut Model, node_id: NodeId) -> *mut Node {
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
pub unsafe extern "C" fn odeir_population_take_links(
    node: *mut Node,
    out_len: *mut usize,
    out_cap: *mut usize,
) -> *mut Link {
    let node = unsafe { &mut *node };

    if let Node::Population { ref mut links, .. } = node {
        let (ptr, len, cap) = links.clone().into_raw_parts();

        unsafe {
            *out_len = len;
            *out_cap = cap;
        }

        ptr
    } else {
        panic!("Node is not a population");
    }
}

/// # Safety
/// This function is unsafe because it derefences the node raw pointer.
#[no_mangle]
pub unsafe extern "C" fn odeir_combinator_take_inputs(
    node: *mut Node,
    out_len: *mut usize,
    out_cap: *mut usize,
) -> *mut NodeId {
    let node = unsafe { &mut *node };

    if let Node::Combinator { ref mut inputs, .. } = node {
        let (ptr, len, cap) = inputs.clone().into_raw_parts();

        unsafe {
            *out_len = len;
            *out_cap = cap;
        }

        ptr
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

    match node {
        Node::Population { name, .. } => {
            *out_type = NodeType::Population;
            utils::string_to_cstr(name.clone())
        }
        Node::Combinator { name, .. } => {
            *out_type = NodeType::Combinator;
            utils::string_to_cstr(name.clone())
        }
    }
}
