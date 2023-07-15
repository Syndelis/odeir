#ifndef ODEIR_INTERNAL_HPP
#define ODEIR_INTERNAL_HPP

#include <cstdarg>
#include <cstddef>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>


namespace internal_api {

enum class LinkType : uint8_t {
    Normal,
    Negative,
};

enum class NodeType : uint8_t {
    Population,
    Combinator,
    Constant,
};

enum class Operation : uint8_t {
    Add,
    Sub,
    Div,
    Mul,
};

struct Model;

struct Node;

using NodeId = uint32_t;

struct Link {
    NodeId receiver;
    NodeId sender;
    LinkType link_type;
    bool operator==(const Link &other) const {
        return link_type == other.link_type &&
               receiver == other.receiver &&
               sender == other.sender;
    }
};

using cchar = char;

using cstr = const cchar*;


extern "C" {

/// # Safety
/// This function is unsafe because it derefences the node raw pointer.
Link *odeir_combinator_inputs(Node *node);

///
/// # Safety
/// This function is unsafe because it derefences the node raw pointer.
size_t odeir_combinator_inputs_len(Node *node);

/// # Safety
/// This function is unsafe because it derefences the model raw pointer.
void odeir_combinator_insert_input(Node *node, LinkType link_type, NodeId source_node_id);

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
Node *odeir_insert_combinator(Model *model, NodeId node_id, cstr name, Operation operation);

/// # Safety
/// This function is unsafe because it derefences the model raw pointer.
Node *odeir_insert_const(Model *model, NodeId node_id, cstr name, double value);

/// # Safety
/// This function is unsafe because it derefences both raw pointers for the
/// model and the name. The caller must ensure that the pointers are valid.
Node *odeir_insert_population(Model *model, NodeId id, cstr name, double initial_population);

/// # Safety
/// This function is unsafe because it derefences the string pointer.
/// This function may return a null pointer if the JSON is invalid.
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
/// This function is unsafe because it derefences the model raw pointer.
void odeir_node_insert_output(Node *node, LinkType link_type, NodeId target_node_id);

/// # Safety
/// This function is unsafe because it derefences the node raw pointer.
Link *odeir_node_outputs(Node *node);

///
/// # Safety
/// This function is unsafe because it derefences the node raw pointer.
size_t odeir_node_outputs_len(Node *node);

/// # Safety
/// This function is unsafe because it derefences the model raw pointer.
void odeir_set_metadata(Model *model, double start_time, double end_time, double delta_time);

} // extern "C"

} // namespace internal_api

#endif // ODEIR_INTERNAL_HPP
