#ifndef ODEIR_HPP
#define ODEIR_HPP

#include <cstdarg>
#include <cstddef>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>
#include "cxx.h"


struct Link;

using NodeId = uint32_t;

struct Node {
    enum class Tag {
        Population,
        Combinator,
    };

    struct Population_Body {
        NodeId id;
        rust::String name;
        rust::String related_constant_name;
        rust::Vec<Link> links;
    };

    struct Combinator_Body {
        NodeId id;
        rust::String name;
        uint32_t operation;
        rust::Vec<NodeId> inputs;
    };

    Tag tag;
    union {
        Population_Body population;
        Combinator_Body combinator;
    };
};


extern "C" {

void _this_function_guarantees_node_is_exported_by_cbindgen(Node *node);

} // extern "C"

#endif // ODEIR_HPP
