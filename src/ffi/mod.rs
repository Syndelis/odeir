mod utils;

use serde_json::Value;

use crate::{Model, Node, NodeId, Link, Constant};

use self::utils::{cstr, cchar};

#[no_mangle]
pub extern "C" fn odeir_new_model() -> *mut Model {
    let model = Box::default();
    Box::leak(model)
}

/// # Safety
/// This function is unsafe because it derefences both raw pointers for the
/// model and the name. The caller must ensure that the pointers are valid.
#[no_mangle]
pub unsafe extern "C" fn odeir_insert_population(
    model: *mut Model, id: NodeId, name: cstr, related_constant_name: cstr
) {
    let name = utils::cstr_cloned_into_string(name);
    let related_constant_name = utils::cstr_cloned_into_string(related_constant_name);

    let node = Node::Population {
        id,
        name,
        related_constant_name,
        links: vec![],
    };

    let model = unsafe { &mut *model };

    model.nodes.insert(id, node);
}

/// # Safety
/// This function is unsafe because it derefences the model raw pointer.
#[no_mangle]
pub unsafe extern "C" fn odeir_insert_population_link(
    model: *mut Model, node_id: NodeId, sign: cchar, target_node_id: NodeId
) {
    let link = Link {
        sign: sign as u8 as char,
        node_id: target_node_id,
    };

    let model = unsafe { &mut *model };

    match model.nodes.get_mut(&node_id) {
        Some(Node::Population { links, .. }) => {
            links.push(link);
        },
        _ => panic!("Population with id {} not found", node_id),
    }
}

/// # Safety
/// This function is unsafe because it derefences both raw pointers for the
/// model and the name. The caller must ensure that the pointers are valid.
#[no_mangle]
pub unsafe extern "C" fn odeir_insert_combinator(
    model: *mut Model, node_id: NodeId, name: cstr, operation: cchar
) {

    let name = utils::cstr_cloned_into_string(name);

    let node = Node::Combinator {
        id: node_id,
        name,
        operation: operation as u8 as char,
        inputs: vec![],
    };

    let model = unsafe { &mut *model };

    model.nodes.insert(node_id, node);

}

/// # Safety
/// This function is unsafe because it derefences the model raw pointer.
#[no_mangle]
pub unsafe extern "C" fn odeir_insert_combinator_input(
    model: *mut Model, node_id: NodeId, input_node_id: NodeId
) {
    let model = unsafe { &mut *model };

    match model.nodes.get_mut(&node_id) {
        Some(Node::Combinator { inputs, .. }) => {
            inputs.push(input_node_id);
        },
        _ => panic!("Combinator with id {} not found", node_id),
    }
}

/// # Safety
/// This function is unsafe because it derefences the model raw pointer.
#[no_mangle]
pub unsafe extern "C" fn odeir_insert_const(model: *mut Model, name: cstr, value: f64) {
    let name = utils::cstr_cloned_into_string(name);

    let model = unsafe { &mut *model };

    model.constants.push(Constant { name, value });
}

/// # Safety
/// This function is unsafe because it derefences the model raw pointer and
/// frees it. Using the pointer after calling this function will result in
/// undefined behavior.
#[no_mangle]
pub unsafe extern "C" fn odeir_free_model(model: *mut Model) {
    let _ = unsafe { Box::from_raw(model) };
}

/// # Safety
/// This function is unsafe because it derefences the model raw pointer.
/// This function also allocates a new string. The caller must save it so they
/// can free it later.
#[no_mangle]
pub unsafe extern "C" fn odeir_debug_string_model(model: *mut Model) -> cstr {
    let model = unsafe { &mut *model };

    let model_str = format!("{:#?}", model);

    utils::string_to_cstr(model_str)
}

/// # Safety
/// This function is unsafe because it derefences and frees a pointer.
#[no_mangle]
pub unsafe extern "C" fn odeir_free_cstr(cstr: cstr) {
    utils::free_cstr(cstr);
}

/// # Safety
/// This function is unsafe because it derefences the model raw pointer.
#[no_mangle]
pub unsafe extern "C" fn odeir_set_metadata(
    model: *mut Model, start_time: f64, end_time: f64, delta_time: f64
) {
    let model = unsafe { &mut *model };

    model.meta_data.start_time = start_time;
    model.meta_data.end_time = end_time;
    model.meta_data.delta_time = delta_time;
}

/// # Safety
/// This function is unsafe because it derefences the two strings.
#[no_mangle]
pub unsafe extern "C" fn odeir_debug_compare_jsons(json1: cstr, json2: cstr) -> bool {
    let json1 = unsafe { utils::cstr_cloned_into_string(json1) };
    let json2 = unsafe { utils::cstr_cloned_into_string(json2) };

    let json1: Value = serde_json::from_str(&json1).unwrap();
    let json2: Value = serde_json::from_str(&json2).unwrap();

    let comparison_result = std::panic::catch_unwind(||
        assert_json_diff::assert_json_eq!(json1, json2)
    );

    if let Err(comparion_difference) = comparison_result {
        dbg!(comparion_difference);
        false
    }

    else {
        true
    }
}

/// # Safety
/// This function is unsafe because it derefences the model raw pointer.
#[no_mangle]
pub unsafe extern "C" fn odeir_model_to_json(model: *mut Model) -> cstr {
    let model = unsafe { &mut *model };

    let json = serde_json::to_string(model).unwrap();

    utils::string_to_cstr(json)
}
