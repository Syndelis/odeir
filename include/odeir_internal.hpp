#ifndef ODEIR_INTERNAL_HPP
#define ODEIR_INTERNAL_HPP

#include <cstdarg>
#include <cstddef>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>


namespace internal_api {

enum class NodeType : uint8_t {
    Population,
    Combinator,
};

struct Model;

struct Node;

using NodeId = uint32_t;

using cchar = char;

using cstr = const cchar*;

struct Link {
    uint32_t sign;
    uint32_t node_id;
    bool operator==(const Link &other) const {
        return sign == other.sign && node_id == other.node_id;
    }
};


extern "C" {

/// # Safety
/// This function is unsafe because it derefences the node raw pointer.
NodeId *odeir_combinator_take_inputs(Node *node, size_t *out_len, size_t *out_cap);

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
/// This function is unsafe because it derefences and frees a pointer.
void odeir_free_link_vec(Link *vec, size_t len, size_t cap);

/// # Safety
/// This function is unsafe because it derefences the model raw pointer and
/// frees it. Using the pointer after calling this function will result in
/// undefined behavior.
void odeir_free_model(Model *model);

/// # Safety
/// This function is unsafe because it derefences the node raw pointer and
/// frees it. Using the pointer after calling this function will result in
/// undefined behavior.
void odeir_free_node(Node *node);

/// # Safety
/// This function is unsafe because it derefences and frees a pointer.
void odeir_free_node_id_vec(NodeId *vec, size_t len, size_t cap);

/// # Safety
/// This function is unsafe because it derefences both raw pointers for the
/// model and the name. The caller must ensure that the pointers are valid.
Node *odeir_insert_combinator(Model *model, NodeId node_id, cstr name, cchar operation);

/// # Safety
/// This function is unsafe because it derefences the model raw pointer.
void odeir_insert_combinator_input(Node *node, NodeId input_node_id);

/// # Safety
/// This function is unsafe because it derefences the model raw pointer.
void odeir_insert_const(Model *model, cstr name, double value);

/// # Safety
/// This function is unsafe because it derefences both raw pointers for the
/// model and the name. The caller must ensure that the pointers are valid.
Node *odeir_insert_population(Model *model, NodeId id, cstr name, cstr related_constant_name);

/// # Safety
/// This function is unsafe because it derefences the model raw pointer.
void odeir_insert_population_link(Node *node, cchar sign, NodeId target_node_id);

/// # Safety
/// This function is unsafe because it dereferences the string pointer.
/// This function may return a null pointer if the file doesn't exist or if the
/// file content is invalid JSON.
Model *odeir_json_file_path_to_model(cstr json_file_path);

/// # Safety
/// This function is unsafe because it derefences the string pointer.
/// This function may return a null pointer if the string is invalid JSON.
Model *odeir_json_to_model(cstr json);

/// # Safety
/// This function is unsafe because it derefences the model raw pointer.
Node *odeir_model_get_node(Model *model, NodeId node_id);

/// # Safety
/// This function is unsafe because it derefences the model raw pointer and also
/// writes to both `out_len` and `out_cap` pointers.
NodeId *odeir_model_get_node_ids(Model *model, size_t *out_len, size_t *out_cap);

/// # Safety
/// This function is unsafe because it derefences the model raw pointer.
cstr odeir_model_to_json(Model *model);

Model *odeir_new_model();

/// # Safety
/// This function is unsafe because it derefences the node raw pointer and the
/// out_type pointer.
cstr odeir_node_get_info(Node *node, NodeType *out_type);

/// # Safety
/// This function is unsafe because it derefences the node raw pointer.
Link *odeir_population_take_links(Node *node, size_t *out_len, size_t *out_cap);

/// # Safety
/// This function is unsafe because it derefences the model raw pointer.
void odeir_set_metadata(Model *model, double start_time, double end_time, double delta_time);

} // extern "C"

} // namespace internal_api

#endif // ODEIR_INTERNAL_HPP
