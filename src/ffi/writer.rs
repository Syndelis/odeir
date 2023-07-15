use serde_json::Value;

use crate::{Model, Node, NodeId, Link, LinkType, Operation};

use super::utils::{cstr, cchar, self};

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
    model: *mut Model, id: NodeId, name: cstr, initial_population: f64
) -> *mut Node
{
    let name = utils::cstr_cloned_into_string(name);

    let node = Node::Population {
        id,
        initial_population,
        name,
        outputs: vec![],
    };

    let model = unsafe { &mut *model };

    model.nodes.insert(id, node);

    model.nodes.get_mut(&id).unwrap()
}

/// # Safety
/// This function is unsafe because it derefences the model raw pointer.
#[no_mangle]
pub unsafe extern "C" fn odeir_combinator_insert_input(
    node: *mut Node, link_type: LinkType, source_node_id: NodeId
) {
    let node = unsafe { &mut *node };
    let link = Link {
        link_type,
        receiver: node.id(),
        sender: source_node_id,
    };
    if let Node::Combinator { inputs, .. } = node {
        inputs.push(link);
    } else {
        panic!("Expected Combinator node");
    }
}

/// # Safety
/// This function is unsafe because it derefences the model raw pointer.
#[no_mangle]
pub unsafe extern "C" fn odeir_node_insert_output(
    node: *mut Node, link_type: LinkType, target_node_id: NodeId
) {
    let node = unsafe { &mut *node };

    let link = Link {
        link_type,
        receiver: target_node_id,
        sender: node.id(),
    };

    node.outputs().push(link);
}

/// # Safety
/// This function is unsafe because it derefences both raw pointers for the
/// model and the name. The caller must ensure that the pointers are valid.
#[no_mangle]
pub unsafe extern "C" fn odeir_insert_combinator(
    model: *mut Model, node_id: NodeId, name: cstr, operation: Operation
) -> *mut Node
{

    let name = utils::cstr_cloned_into_string(name);

    let node = Node::Combinator {
        id: node_id,
        name,
        operation,
        outputs: vec![],
        inputs: vec![],
    };

    let model = unsafe { &mut *model };

    model.nodes.insert(node_id, node);

    model.nodes.get_mut(&node_id).unwrap()

}

/// # Safety
/// This function is unsafe because it derefences the model raw pointer.
#[no_mangle]
pub unsafe extern "C" fn odeir_insert_const(model: *mut Model, node_id: NodeId, name: cstr, value: f64) -> *mut Node {
    let name = utils::cstr_cloned_into_string(name);

    let node = Node::Constant { id: node_id, name, outputs: vec![], value };

    let model = unsafe { &mut *model };

    model.nodes.insert(node_id, node);

    model.nodes.get_mut(&node_id).unwrap()
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
