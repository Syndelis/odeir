#include <cstdarg>
#include <cstddef>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>


struct Model;

using cchar = char;

using cstr = const cchar*;

using NodeId = uint32_t;


extern "C" {

/// # Safety
/// This function is unsafe because it derefences the two strings.
bool odeir_debug_compare_jsons(cstr json1, cstr json2);

/// # Safety
/// This function is unsafe because it derefences the model raw pointer.
/// This function also allocates a new string. The caller must save it so they
/// can free it later.
cstr odeir_debug_string_model(Model *model);

/// # Safety
/// This function is unsafe because it derefences and frees a pointer.
void odeir_free_cstr(cstr cstr);

/// # Safety
/// This function is unsafe because it derefences the model raw pointer and
/// frees it. Using the pointer after calling this function will result in
/// undefined behavior.
void odeir_free_model(Model *model);

/// # Safety
/// This function is unsafe because it derefences both raw pointers for the
/// model and the name. The caller must ensure that the pointers are valid.
void odeir_insert_combinator(Model *model, NodeId node_id, cstr name, cchar operation);

/// # Safety
/// This function is unsafe because it derefences the model raw pointer.
void odeir_insert_combinator_input(Model *model, NodeId node_id, NodeId input_node_id);

/// # Safety
/// This function is unsafe because it derefences the model raw pointer.
void odeir_insert_const(Model *model, cstr name, double value);

/// # Safety
/// This function is unsafe because it derefences both raw pointers for the
/// model and the name. The caller must ensure that the pointers are valid.
void odeir_insert_population(Model *model, NodeId id, cstr name, cstr related_constant_name);

/// # Safety
/// This function is unsafe because it derefences the model raw pointer.
void odeir_insert_population_link(Model *model, NodeId node_id, cchar sign, NodeId target_node_id);

/// # Safety
/// This function is unsafe because it derefences the model raw pointer.
cstr odeir_model_to_json(Model *model);

Model *odeir_new_model();

/// # Safety
/// This function is unsafe because it derefences the model raw pointer.
void odeir_set_metadata(Model *model, double start_time, double end_time, double delta_time);

} // extern "C"
