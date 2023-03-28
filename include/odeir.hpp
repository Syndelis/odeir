#include <cstdarg>
#include <cstddef>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>


struct MetaData {
    double start_time;
    double end_time;
    double delta_time;
};

using NodeId = uint32_t;

struct CNode {
    enum class Tag {
        Population,
        Combinator,
    };

    struct Population_Body {
        NodeId id;
        const char *name;
        const char *related_constant_name;
    };

    struct Combinator_Body {
        NodeId id;
        const char *name;
        uint32_t operation;
        const NodeId *inputs;
        size_t input_count;
    };

    Tag tag;
    union {
        Population_Body population;
        Combinator_Body combinator;
    };
};

struct CConstant {
    const char *name;
    double value;
};

struct CModel {
    MetaData meta_data;
    const CNode *nodes;
    const CConstant *constants;
    size_t node_count;
    size_t constant_count;
};


extern "C" {

/// # Safety
/// `json_str` must be a valid pointer to a null-terminated C string. The
/// caller is responsible for freeing the returned `CModel` fields.
CModel model_from_cstring(const char *json_str);

} // extern "C"
